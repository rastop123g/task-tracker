import type {
  ApiErrorResponse,
  ForbiddenErrorResponse,
  UnauthotizedErrorResponse,
  ValidationErrorResponse,
} from '@/api/generated/schema'

export type BackendErrorPayload =
  | ApiErrorResponse
  | ForbiddenErrorResponse
  | UnauthotizedErrorResponse
  | ValidationErrorResponse

export type ApiErrorCode = 'network' | 'abort' | 'http'

// Достаёт понятное сообщение из ответа бэкенда
function extractBackendMessage(backend: BackendErrorPayload | undefined): string | undefined {
  if (!backend) {
    return undefined
  }

  if ('error' in backend && typeof backend.error === 'string') {
    return backend.error
  }

  if ('reason' in backend && typeof backend.reason === 'string') {
    return backend.reason
  }

  if ('errors' in backend && Array.isArray(backend.errors) && backend.errors.length > 0) {
    return 'Ошибка валидации'
  }

  return undefined
}

export class ApiError extends Error {
  readonly code: ApiErrorCode
  readonly status: number | null
  readonly backend?: BackendErrorPayload
  readonly cause?: unknown

  constructor(options: {
    code: ApiErrorCode
    status?: number | null
    message?: string
    backend?: BackendErrorPayload
    cause?: unknown
  }) {
    super(options.message ?? extractBackendMessage(options.backend) ?? 'Ошибка запроса к API')
    this.name = 'ApiError'
    this.code = options.code
    this.status = options.status ?? null
    this.backend = options.backend
    this.cause = options.cause
  }
}

// Проверяет, что ошибка уже приведена к типу API
export function isApiError(error: unknown): error is ApiError {
  return error instanceof ApiError
}

// Определяет, похож ли ответ на структуру ошибки бэкенда
function isBackendErrorPayload(value: unknown): value is BackendErrorPayload {
  if (!value || typeof value !== 'object') {
    return false
  }

  return 'error' in value || 'reason' in value || 'errors' in value
}

// Создаёт ошибку API из неуспешного HTTP-ответа
export function apiErrorFromResponse(response: Response, body: unknown): ApiError {
  const backend = isBackendErrorPayload(body) ? body : undefined

  return new ApiError({
    code: 'http',
    status: response.status,
    backend,
    message: extractBackendMessage(backend) ?? `HTTP ${response.status}`,
    cause: response,
  })
}

// Нормализует любую причину сбоя в ошибку API
export function apiErrorFromCause(cause: unknown): ApiError {
  if (cause instanceof ApiError) {
    return cause
  }

  if (cause instanceof DOMException && cause.name === 'AbortError') {
    return new ApiError({
      code: 'abort',
      message: 'Запрос был прерван',
      cause,
    })
  }

  return new ApiError({
    code: 'network',
    message: cause instanceof Error ? cause.message : 'Сетевой запрос завершился ошибкой',
    cause,
  })
}

// Приводит внешнюю ошибку к единому формату API
export function toApiError(cause: unknown): ApiError {
  return apiErrorFromCause(cause)
}
