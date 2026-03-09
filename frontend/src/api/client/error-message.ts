import { toApiError } from './api-error'

const DEFAULT_FALLBACK_MESSAGE = 'Произошла ошибка. Попробуйте позже.'
const DEFAULT_NETWORK_MESSAGE = 'Не удалось выполнить запрос. Проверьте соединение'
const DEFAULT_SERVER_MESSAGE = 'Ошибка сервера. Попробуйте позже'
const DEFAULT_VALIDATION_MESSAGE = 'Проверьте корректность введенных данных'

export type ApiErrorMessageOptions = {
  fallback?: string
  network?: string
  server?: string
  validation?: string
  status?: Partial<Record<number, string>>
  backendError?: Record<string, string>
}

// Подбирает сообщение для пользователя по типу ошибки
export function getApiErrorMessage(cause: unknown, options: ApiErrorMessageOptions = {}): string {
  const error = toApiError(cause)
  const backendErrorCode =
    error.backend && 'error' in error.backend && typeof error.backend.error === 'string'
      ? error.backend.error
      : null
  const backendErrorMessage = backendErrorCode
    ? options.backendError?.[backendErrorCode]
    : undefined
  const statusMessage = error.status !== null ? options.status?.[error.status] : undefined

  if (backendErrorMessage) {
    return backendErrorMessage
  }

  if (error.code === 'network') {
    return options.network ?? DEFAULT_NETWORK_MESSAGE
  }

  if (statusMessage) {
    return statusMessage
  }

  if (error.status === 400) {
    return options.validation ?? DEFAULT_VALIDATION_MESSAGE
  }

  if (error.status !== null && error.status >= 500) {
    return options.server ?? DEFAULT_SERVER_MESSAGE
  }

  return options.fallback ?? DEFAULT_FALLBACK_MESSAGE
}
