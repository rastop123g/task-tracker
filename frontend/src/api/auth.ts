import type { RegisterRequest, LoginRequest, LoginResponse } from '@/api/generated/schema'
import { session } from '@/app/session/session'
import { get, post } from './client'

export const auth = {
  async register(payload: RegisterRequest): Promise<void> {
    await post('/api/v1/auth/register', {
      json: payload,
      auth: false,
    })
  },

  async login(payload: LoginRequest): Promise<LoginResponse> {
    const data = await post('/api/v1/auth/login', {
      json: payload,
      auth: false,
    })
    session.setSession(data)
    return data
  },

  async verify(token: string) {
    return get('/api/v1/auth/verify', {
      query: { token },
    })
  },
}
