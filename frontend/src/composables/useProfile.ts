import { computed } from 'vue'
import { api } from '@/api'
import { currentUserStore } from '@/app/session/current-user-store'

async function fetchProfile(): Promise<void> {
  try {
    await currentUserStore.fetchMe()
  } catch {}
}

async function updateName(name: string): Promise<void> {
  const updated = await api.user.updateMe({ name })
  currentUserStore.setUser(updated)
}

async function uploadAvatar(file: File): Promise<void> {
  await api.avatar.uploadMyAvatar(file)
  await currentUserStore.fetchMe({ force: true })
}

export function useProfile() {
  return {
    profile: currentUserStore.user,
    isLoading: computed(() => currentUserStore.status.value === 'loading'),
    error: computed(() => currentUserStore.error.value?.message ?? null),
    avatarUrl: computed(() =>
      currentUserStore.user.value ? `/api/v1/avatar/${currentUserStore.user.value.id}` : null,
    ),
    fetchProfile,
    updateName,
    uploadAvatar,
  }
}
