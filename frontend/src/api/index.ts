import { avatar } from './avatar'
import { auth } from './auth'
import { user } from './user'

export const api = { auth, user, avatar }
export type { ApiError } from './types'
export { getApiErrorMessage, isApiError } from './client'
