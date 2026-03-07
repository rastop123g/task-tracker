import type { RefreshTokenResponse } from '@protocol/RefreshTokenResponse'
import { useAuth } from '@/composables/useAuth'

type RequestOptions = {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
  path: string
  body?: unknown
  authenticated?: boolean
}

const JSON_CONTENT_TYPE = 'application/json'

let refreshInFlight: Promise<void> | null = null

function buildHeaders(opts: RequestOptions, needsAuth: boolean): Record<string, string> {
  const headers: Record<string, string> = {}

  if (opts.body !== undefined) {
    headers['Content-Type'] = JSON_CONTENT_TYPE
  }

  if (needsAuth) {
    const token = useAuth().accessToken.value
    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    }
  }

  return headers
}

function sendRequest(opts: RequestOptions, needsAuth: boolean): Promise<Response> {
  return fetch(opts.path, {
    method: opts.method,
    headers: buildHeaders(opts, needsAuth),
    body: opts.body !== undefined ? JSON.stringify(opts.body) : undefined,
  })
}

async function parseSuccessResponse<T>(resp: Response): Promise<T> {
  // На бэке есть и JSON, и текстовые успешные ответы.
  if (resp.status === 204) {
    return undefined as T
  }
  const contentType = resp.headers.get('content-type') ?? ''
  if (contentType.includes(JSON_CONTENT_TYPE)) {
    return (await resp.json()) as T
  }
  return (await resp.text()) as T
}

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

async function refreshSession(): Promise<void> {
  const { refreshToken, setSession, clearTokens } = useAuth()
  const refresh_token = refreshToken.value

  // Если refresh токена нет, текущую сессию больше нельзя восстановить.
  if (!refresh_token) {
    clearTokens()
    throw new Error('no refresh token')
  }

  const resp = await fetch('/api/v1/auth/refresh', {
    method: 'POST',
    headers: { 'Content-Type': JSON_CONTENT_TYPE },
    body: JSON.stringify({ refresh_token }),
  })

  if (!resp.ok) {
    clearTokens()
    throw resp
  }

  let data: unknown
  try {
    data = await resp.json()
  } catch (error) {
    clearTokens()
    throw error
  }

  if (!isRefreshTokenResponse(data)) {
    clearTokens()
    throw new Error('invalid refresh response')
  }

  setSession(data)
}

async function refreshSessionLocked(): Promise<void> {
  // Один refresh на все параллельные 401, чтобы не плодить дублирующие запросы.
  if (!refreshInFlight) {
    refreshInFlight = refreshSession()
  }

  try {
    await refreshInFlight
  } finally {
    refreshInFlight = null
  }
}

export async function request<T>(opts: RequestOptions): Promise<T> {
  const needsAuth = opts.authenticated !== false
  const resp = await sendRequest(opts, needsAuth)

  if (resp.ok) {
    return parseSuccessResponse<T>(resp)
  }

  if (resp.status !== 401 || !needsAuth) {
    throw resp
  }

  // После refresh повторяем исходный запрос только один раз.
  await refreshSessionLocked()

  const retryResp = await sendRequest(opts, needsAuth)

  if (retryResp.ok) {
    return parseSuccessResponse<T>(retryResp)
  }

  throw retryResp
}
