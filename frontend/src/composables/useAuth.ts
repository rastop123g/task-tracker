import { session } from '@/app/session/session'

export function useAuth() {
  return {
    ...session,
    clearTokens: session.clear,
  }
}
