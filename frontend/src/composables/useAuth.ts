import { ref, computed } from 'vue'
import type { Ref } from 'vue'
import type { RefreshTokenResponse } from '@protocol/RefreshTokenResponse'

const ACCESS_KEY = 'tt_access_token'
const REFRESH_KEY = 'tt_refresh_token'
const TOKEN_EXP_KEY = 'tt_token_exp'
const REFRESH_EXP_KEY = 'tt_refresh_exp'
const SERVER_OFFSET_KEY = 'tt_server_offset_ms'
const TOKEN_KEYS = [ACCESS_KEY, REFRESH_KEY, TOKEN_EXP_KEY, REFRESH_EXP_KEY, SERVER_OFFSET_KEY] as const

const accessToken: Ref<string | null> = ref(localStorage.getItem(ACCESS_KEY))
const refreshToken: Ref<string | null> = ref(localStorage.getItem(REFRESH_KEY))
const tokenExp: Ref<string | null> = ref(localStorage.getItem(TOKEN_EXP_KEY))
const refreshExp: Ref<string | null> = ref(localStorage.getItem(REFRESH_EXP_KEY))
const storedServerOffset = Number(localStorage.getItem(SERVER_OFFSET_KEY) ?? '0')
// Смещение нужно, чтобы сравнивать срок токена по времени сервера, а не только по локальным часам.
const serverOffsetMs: Ref<number> = ref(Number.isFinite(storedServerOffset) ? storedServerOffset : 0)

const isAuthenticated = computed(() => {
  if (!accessToken.value || !tokenExp.value) {
    return false
  }

  const expiresAt = Date.parse(tokenExp.value)
  if (!Number.isFinite(expiresAt)) {
    return false
  }

  // Токен считается валидным, пока serverNow() не достиг времени истечения.
  return serverNow() < expiresAt
})

function serverNow(): number {
  return Date.now() + serverOffsetMs.value
}

function setSession(payload: RefreshTokenResponse): void {
  // Выравниваем локальное время с временем сервера для точной проверки срока токена.
  const parsedServerTime = Date.parse(payload.server_time)
  const offset = Number.isFinite(parsedServerTime) ? parsedServerTime - Date.now() : 0
  accessToken.value = payload.token
  refreshToken.value = payload.refresh_token
  tokenExp.value = payload.token_exp
  refreshExp.value = payload.refresh_exp
  serverOffsetMs.value = offset
  const storageData: Record<(typeof TOKEN_KEYS)[number], string> = {
    [ACCESS_KEY]: payload.token,
    [REFRESH_KEY]: payload.refresh_token,
    [TOKEN_EXP_KEY]: payload.token_exp,
    [REFRESH_EXP_KEY]: payload.refresh_exp,
    [SERVER_OFFSET_KEY]: String(offset),
  }
  for (const key of TOKEN_KEYS) {
    localStorage.setItem(key, storageData[key])
  }
}

function clearTokens(): void {
  // Полная очистка нужна, чтобы UI не оставался в "полу-авторизованном" состоянии.
  accessToken.value = null
  refreshToken.value = null
  tokenExp.value = null
  refreshExp.value = null
  serverOffsetMs.value = 0
  for (const key of TOKEN_KEYS) {
    localStorage.removeItem(key)
  }
}

export function useAuth() {
  return {
    accessToken,
    refreshToken,
    tokenExp,
    refreshExp,
    isAuthenticated,
    serverNow,
    setSession,
    clearTokens,
  }
}
