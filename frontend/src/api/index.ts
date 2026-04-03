import { avatar } from './avatar'
import { auth } from './auth'
import { user } from './user'
import { workspace } from './workspace'

export const api = { auth, user, avatar, workspace }
export type { ApiError } from './types'
export { getApiErrorMessage, isApiError } from './client'
