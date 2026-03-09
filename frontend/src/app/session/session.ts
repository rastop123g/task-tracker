import { computed, ref } from 'vue'
import type { Ref } from 'vue'
import type { RefreshTokenResponse } from '@/api/generated/schema'
import { clearStoredTokens, readStoredTokens, writeStoredTokens } from './token-storage'

// Поднимаем session из persisted токенов при старте приложения
const storedTokens = readStoredTokens()

const accessToken: Ref<string | null> = ref(storedTokens.accessToken)
const refreshToken: Ref<string | null> = ref(storedTokens.refreshToken)
const tokenExp: Ref<string | null> = ref(storedTokens.tokenExp)
const refreshExp: Ref<string | null> = ref(storedTokens.refreshExp)
const serverOffsetMs: Ref<number> = ref(storedTokens.serverOffsetMs)

function serverNow(): number {
  return Date.now() + serverOffsetMs.value
}

// Сравниваем с server time чтобы меньше зависеть от локального времени браузера
const isAuthenticated = computed(() => {
  if (!accessToken.value || !tokenExp.value) {
    return false
  }

  const expiresAt = Date.parse(tokenExp.value)
  if (!Number.isFinite(expiresAt)) {
    return false
  }

  return serverNow() < expiresAt
})

function setSession(payload: RefreshTokenResponse): void {
  // Храним offset между сервером и клиентом для проверки срока жизни токена
  const parsedServerTime = Date.parse(payload.server_time)
  const offset = Number.isFinite(parsedServerTime) ? parsedServerTime - Date.now() : 0

  accessToken.value = payload.token
  refreshToken.value = payload.refresh_token
  tokenExp.value = payload.token_exp
  refreshExp.value = payload.refresh_exp
  serverOffsetMs.value = offset

  writeStoredTokens({
    accessToken: payload.token,
    refreshToken: payload.refresh_token,
    tokenExp: payload.token_exp,
    refreshExp: payload.refresh_exp,
    serverOffsetMs: offset,
  })
}

function clear(): void {
  // Полностью сбрасываем и reactive state и persisted токены
  accessToken.value = null
  refreshToken.value = null
  tokenExp.value = null
  refreshExp.value = null
  serverOffsetMs.value = 0

  clearStoredTokens()
}

export const session = {
  accessToken,
  refreshToken,
  tokenExp,
  refreshExp,
  serverOffsetMs,
  isAuthenticated,
  serverNow,
  setSession,
  clear,
}
