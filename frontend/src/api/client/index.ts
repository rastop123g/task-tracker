import type { RefreshTokenRequest, RefreshTokenResponse } from '@/api/generated/schema'
import { session } from '@/app/session/session'
import { ApiError, apiErrorFromCause, apiErrorFromResponse } from './api-error'
import { createUrl } from './create-url'
import { parseBody } from './parse-body'
import type {
  HttpMethod,
  OperationFor,
  PathForMethod,
  RequestOptions,
  ResponseFor,
} from './request-options'

const JSON_CONTENT_TYPE = 'application/json'

let refreshInFlight: Promise<void> | null = null

// Проверяет, что ответ обновления сессии имеет ожидаемую форму
function isRefreshTokenResponse(data: unknown): data is RefreshTokenResponse {
  if (!data || typeof data !== 'object') {
    return false
  }

  const payload = data as Partial<RefreshTokenResponse>
  return (
    typeof payload.token === 'string' &&
    typeof payload.refresh_token === 'string' &&
    typeof payload.token_exp === 'string' &&
    typeof payload.refresh_exp === 'string' &&
    typeof payload.server_time === 'string'
  )
}

// Собирает заголовки запроса с учётом тела и авторизации
function buildHeaders(options: {
  hasJsonBody: boolean
  needsAuth: boolean
}): Record<string, string> {
  const headers: Record<string, string> = {}

  if (options.hasJsonBody) {
    headers['Content-Type'] = JSON_CONTENT_TYPE
  }

  if (options.needsAuth) {
    const token = session.accessToken.value
    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    }
  }

  return headers
}

type SerializablePathParams = Record<string, string | number | boolean>
type SerializableQuery = Record<
  string,
  string | number | boolean | null | undefined | Array<string | number | boolean | null | undefined>
>

type RequestShape<M extends HttpMethod, P extends PathForMethod<M>> = {
  options: RequestOptions<M, P>
  needsAuth: boolean
}

type RequestArgs<M extends HttpMethod, P extends PathForMethod<M>> = Omit<
  RequestOptions<M, P>,
  'method' | 'path'
>

// Выделяет из опций параметры URL и тело запроса
function extractRequestParts<M extends HttpMethod, P extends PathForMethod<M>>(
  options: RequestOptions<M, P>,
): {
  hasJsonBody: boolean
  params?: SerializablePathParams
  query?: SerializableQuery
  body?: BodyInit
} {
  const hasJsonBody = 'json' in options

  return {
    hasJsonBody,
    params: ('params' in options ? options.params : undefined) as
      | SerializablePathParams
      | undefined,
    query: ('query' in options ? options.query : undefined) as SerializableQuery | undefined,
    body: hasJsonBody
      ? JSON.stringify(options.json)
      : 'formData' in options
        ? options.formData
        : undefined,
  }
}

// Готовит аргументы для вызова fetch
function buildRequestInput<M extends HttpMethod, P extends PathForMethod<M>>({
  options,
  needsAuth,
}: RequestShape<M, P>): [input: string, init: RequestInit] {
  const request = extractRequestParts(options)

  return [
    createUrl(options.path, {
      params: request.params,
      query: request.query,
    }),
    {
      method: options.method,
      headers: buildHeaders({ hasJsonBody: request.hasJsonBody, needsAuth }),
      body: request.body,
      signal: options.signal,
    },
  ]
}

// Выполняет HTTP-запрос и нормализует сетевые ошибки
async function executeRequest<M extends HttpMethod, P extends PathForMethod<M>>(
  options: RequestOptions<M, P>,
  needsAuth: boolean,
): Promise<Response> {
  try {
    return await fetch(...buildRequestInput({ options, needsAuth }))
  } catch (error) {
    throw apiErrorFromCause(error)
  }
}

// Разбирает успешный ответ в тип операции
async function parseSuccessResponse<Operation>(
  response: Response,
): Promise<ResponseFor<Operation>> {
  return (await parseBody(response)) as ResponseFor<Operation>
}

// Обновляет access token по refresh token
async function refreshSession(): Promise<void> {
  const refreshToken = session.refreshToken.value

  if (!refreshToken) {
    session.clear()
    throw new ApiError({
      code: 'http',
      status: 401,
      message: 'Missing refresh token',
    })
  }

  let response: Response
  try {
    const payload: RefreshTokenRequest = { refresh_token: refreshToken }
    response = await fetch('/api/v1/auth/refresh', {
      method: 'POST',
      headers: { 'Content-Type': JSON_CONTENT_TYPE },
      body: JSON.stringify(payload),
    })
  } catch (error) {
    session.clear()
    throw apiErrorFromCause(error)
  }

  const body = await parseBody(response)
  if (!response.ok) {
    session.clear()
    throw apiErrorFromResponse(response, body)
  }

  if (!isRefreshTokenResponse(body)) {
    session.clear()
    throw new ApiError({
      code: 'http',
      status: response.status,
      message: 'Invalid refresh response',
      cause: response,
    })
  }

  session.setSession(body)
}

// Не даёт запускать обновление сессии параллельно
async function refreshSessionLocked(): Promise<void> {
  if (!refreshInFlight) {
    refreshInFlight = refreshSession()
  }

  try {
    await refreshInFlight
  } finally {
    refreshInFlight = null
  }
}

// Сбрасывает сессию и выбрасывает ошибку авторизации
function failUnauthorized(response: Response, body: unknown): never {
  session.clear()
  throw apiErrorFromResponse(response, body)
}

// Разбирает ошибочный ответ и выбрасывает ошибку API
async function parseErrorResponse(response: Response): Promise<never> {
  const body = await parseBody(response)
  throw apiErrorFromResponse(response, body)
}

// Повторяет запрос после успешного обновления сессии
async function retryWithFreshSession<M extends HttpMethod, P extends PathForMethod<M>>(
  options: RequestOptions<M, P>,
  needsAuth: boolean,
): Promise<ResponseFor<OperationFor<M, P>>> {
  const retryResponse = await executeRequest(options, needsAuth)
  if (retryResponse.ok) {
    return parseSuccessResponse<OperationFor<M, P>>(retryResponse)
  }

  const retryBody = await parseBody(retryResponse)
  if (retryResponse.status === 401) {
    return failUnauthorized(retryResponse, retryBody)
  }

  throw apiErrorFromResponse(retryResponse, retryBody)
}

// Обрабатывает неуспешный ответ и при необходимости делает retry
async function handleFailure<M extends HttpMethod, P extends PathForMethod<M>>(
  response: Response,
  options: RequestOptions<M, P>,
  needsAuth: boolean,
): Promise<never> {
  if (response.status === 401 && needsAuth) {
    await refreshSessionLocked()
    return retryWithFreshSession(options, needsAuth) as never
  }

  return parseErrorResponse(response)
}

// Отправляет типизированный запрос и возвращает результат
export async function request<M extends HttpMethod, P extends PathForMethod<M>>(
  options: RequestOptions<M, P>,
): Promise<ResponseFor<OperationFor<M, P>>> {
  const needsAuth = options.auth !== false
  const response = await executeRequest(options, needsAuth)

  if (response.ok) {
    return parseSuccessResponse<OperationFor<M, P>>(response)
  }

  return handleFailure(response, options, needsAuth)
}

// Создаёт запрос с фиксированным HTTP-методом
function requestWithMethod<M extends HttpMethod, P extends PathForMethod<M>>(
  method: M,
  path: P,
  options?: RequestArgs<M, P>,
): Promise<ResponseFor<OperationFor<M, P>>> {
  return request({
    method,
    path,
    ...(options ?? ({} as RequestArgs<M, P>)),
  } as RequestOptions<M, P>)
}

// Выполняет GET-запрос
export function get<P extends PathForMethod<'GET'>>(
  path: P,
  options?: RequestArgs<'GET', P>,
): Promise<ResponseFor<OperationFor<'GET', P>>> {
  return requestWithMethod('GET', path, options)
}

// Выполняет POST-запрос
export function post<P extends PathForMethod<'POST'>>(
  path: P,
  options?: RequestArgs<'POST', P>,
): Promise<ResponseFor<OperationFor<'POST', P>>> {
  return requestWithMethod('POST', path, options)
}

// Выполняет PUT-запрос
export function put<P extends PathForMethod<'PUT'>>(
  path: P,
  options?: RequestArgs<'PUT', P>,
): Promise<ResponseFor<OperationFor<'PUT', P>>> {
  return requestWithMethod('PUT', path, options)
}

// Выполняет PATCH-запрос
export function patch<P extends PathForMethod<'PATCH'>>(
  path: P,
  options?: RequestArgs<'PATCH', P>,
): Promise<ResponseFor<OperationFor<'PATCH', P>>> {
  return requestWithMethod('PATCH', path, options)
}

// Выполняет DELETE-запрос
export function del<P extends PathForMethod<'DELETE'>>(
  path: P,
  options?: RequestArgs<'DELETE', P>,
): Promise<ResponseFor<OperationFor<'DELETE', P>>> {
  return requestWithMethod('DELETE', path, options)
}

export type { HttpMethod, PathForMethod, RequestOptions, ResponseFor }
export { getApiErrorMessage } from './error-message'
export { ApiError, isApiError, toApiError } from './api-error'
