const ACCESS_KEY = 'tt_access_token'
const REFRESH_KEY = 'tt_refresh_token'
const TOKEN_EXP_KEY = 'tt_token_exp'
const REFRESH_EXP_KEY = 'tt_refresh_exp'
const SERVER_OFFSET_KEY = 'tt_server_offset_ms'

export type StoredTokens = {
  accessToken: string | null
  refreshToken: string | null
  tokenExp: string | null
  refreshExp: string | null
  serverOffsetMs: number
}

function readString(key: string): string | null {
  return localStorage.getItem(key)
}

function writeString(key: string, value: string): void {
  localStorage.setItem(key, value)
}

function remove(key: string): void {
  localStorage.removeItem(key)
}

export function readStoredTokens(): StoredTokens {
  // Невалидный offset считаем отсутствующим
  const storedServerOffset = Number(readString(SERVER_OFFSET_KEY) ?? '0')
  return {
    accessToken: readString(ACCESS_KEY),
    refreshToken: readString(REFRESH_KEY),
    tokenExp: readString(TOKEN_EXP_KEY),
    refreshExp: readString(REFRESH_EXP_KEY),
    serverOffsetMs: Number.isFinite(storedServerOffset) ? storedServerOffset : 0,
  }
}

export function writeStoredTokens(tokens: StoredTokens): void {
  // Отсутствующие значения удаляем чтобы storage не расходился с reactive state
  if (tokens.accessToken) {
    writeString(ACCESS_KEY, tokens.accessToken)
  } else {
    remove(ACCESS_KEY)
  }

  if (tokens.refreshToken) {
    writeString(REFRESH_KEY, tokens.refreshToken)
  } else {
    remove(REFRESH_KEY)
  }

  if (tokens.tokenExp) {
    writeString(TOKEN_EXP_KEY, tokens.tokenExp)
  } else {
    remove(TOKEN_EXP_KEY)
  }

  if (tokens.refreshExp) {
    writeString(REFRESH_EXP_KEY, tokens.refreshExp)
  } else {
    remove(REFRESH_EXP_KEY)
  }

  writeString(SERVER_OFFSET_KEY, String(tokens.serverOffsetMs))
}

export function clearStoredTokens(): void {
  // Полный logout должен очищать все session ключи
  remove(ACCESS_KEY)
  remove(REFRESH_KEY)
  remove(TOKEN_EXP_KEY)
  remove(REFRESH_EXP_KEY)
  remove(SERVER_OFFSET_KEY)
}
