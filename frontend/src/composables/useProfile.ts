import { ref, computed } from 'vue'
import type { Ref } from 'vue'
import { api } from '@/api'
import type { UserProfile } from '@/api/user'
import { useAuth } from './useAuth'

const { userId, accessToken } = useAuth()

const profile: Ref<UserProfile | null> = ref(null)
const isLoading: Ref<boolean> = ref(false)
const error: Ref<string | null> = ref(null)

const avatarUrl = computed(() => (userId.value ? `/api/v1/avatar/${userId.value}` : null))

async function fetchProfile(): Promise<void> {
  isLoading.value = true
  error.value = null
  try {
    profile.value = await api.user.getMe()
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to load profile'
  } finally {
    isLoading.value = false
  }
}

async function updateName(name: string): Promise<void> {
  profile.value = await api.user.patchMe(name)
}

async function uploadAvatar(file: File): Promise<void> {
  if (!userId.value) {
    throw new Error('Cannot upload avatar: user ID is not available')
  }
  const formData = new FormData()
  formData.append('avatar', file)
  const response = await fetch(`/api/v1/avatar/${userId.value}`, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${accessToken.value}`,
    },
    body: formData,
  })
  if (!response.ok) {
    throw new Error(`Avatar upload failed: ${response.status} ${response.statusText}`)
  }
}

export function useProfile() {
  return {
    profile,
    isLoading,
    error,
    avatarUrl,
    fetchProfile,
    updateName,
    uploadAvatar,
  }
}
