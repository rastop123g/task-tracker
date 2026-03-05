import type { RegisterRequest } from '@protocol/RegisterRequest'
import type { LoginRequest } from '@protocol/LoginRequest'
import type { LoginResponse } from '@protocol/LoginResponse'
import { useAuth } from '@/composables/useAuth'
import { request } from './client'

export const auth = {
  async register(payload: RegisterRequest): Promise<void> {
    return request<void>({
      method: 'POST',
      path: '/api/v1/auth/register',
      body: payload,
      authenticated: false,
    })
  },

  async login(payload: LoginRequest): Promise<LoginResponse> {
    const { setSession } = useAuth()
    const data = await request<LoginResponse>({
      method: 'POST',
      path: '/api/v1/auth/login',
      body: payload,
      authenticated: false,
    })
    setSession(data)
    return data
  },

  async verify(): Promise<void> {
    return request<void>({
      method: 'GET',
      path: '/api/v1/auth/verify',
    })
  },
}
