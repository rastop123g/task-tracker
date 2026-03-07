import { useAuth } from '@/composables/useAuth'

const NAME_KEY = 'tt_user_name'

export interface UserProfile {
  id: string
  name: string
  email: string
}

export const user = {
  async getMe(): Promise<UserProfile> {
    const { userId, email } = useAuth()
    const name = localStorage.getItem(NAME_KEY) ?? ''
    return {
      id: userId.value ?? '',
      name,
      email: email.value ?? '',
    }
  },

  async patchMe(name: string): Promise<UserProfile> {
    const { userId, email } = useAuth()
    localStorage.setItem(NAME_KEY, name)
    return {
      id: userId.value ?? '',
      name,
      email: email.value ?? '',
    }
  },
}
