import type { UpdateUserRequest, UserResponse } from '@/api/generated/schema'
import { get, put } from './client'

export const user = {
  async getMe(): Promise<UserResponse> {
    return get('/api/v1/user/me')
  },

  async updateMe(payload: UpdateUserRequest): Promise<UserResponse> {
    return put('/api/v1/user/me', {
      json: payload,
    })
  },
}
