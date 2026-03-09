import { computed, ref } from 'vue'
import type { Ref } from 'vue'
import type { UserResponse } from '@/api/generated/schema'
import { user as userApi } from '@/api/user'
import { ApiError, toApiError } from '@/api/client'
import { session } from './session'

type CurrentUserStatus = 'idle' | 'loading' | 'ready' | 'error'

const user: Ref<UserResponse | null> = ref(null)
const status: Ref<CurrentUserStatus> = ref('idle')
const error: Ref<ApiError | null> = ref(null)

// Дедупликация параллельных запросов current user
let fetchMeInFlight: Promise<UserResponse | null> | null = null

function setUser(nextUser: UserResponse): void {
  user.value = nextUser
  status.value = 'ready'
  error.value = null
}

function clear(): void {
  user.value = null
  status.value = 'idle'
  error.value = null
}

async function fetchMe(options: { force?: boolean } = {}): Promise<UserResponse | null> {
  if (!session.isAuthenticated.value) {
    clear()
    return null
  }

  // Переиспользуем уже загруженного пользователя пока не запросили force refresh
  if (!options.force && status.value === 'ready' && user.value) {
    return user.value
  }

  // Возвращаем текущий in-flight запрос чтобы не дублировать fetch /me
  if (fetchMeInFlight) {
    return fetchMeInFlight
  }

  status.value = 'loading'
  error.value = null

  fetchMeInFlight = (async () => {
    try {
      const currentUser = await userApi.getMe()
      setUser(currentUser)
      return currentUser
    } catch (cause) {
      const apiError = toApiError(cause)

      // Logout и redirect уже синхронизируются выше через session и initializeAuthFlow
      if (apiError.status === 401) {
        throw apiError
      }

      status.value = 'error'
      error.value = apiError
      throw apiError
    } finally {
      fetchMeInFlight = null
    }
  })()

  return fetchMeInFlight
}

export const currentUserStore = {
  user,
  status,
  error,
  isReady: computed(() => status.value === 'ready'),
  hasUser: computed(() => user.value !== null),
  fetchMe,
  setUser,
  clear,
}
