import { watch } from 'vue'
import router from '@/router'
import { currentUserStore } from './current-user-store'
import { session } from './session'

async function bootstrapCurrentUser(): Promise<void> {
  // После перезагрузки страницы восстанавливаем current user если session еще валидна
  if (!session.isAuthenticated.value) {
    return
  }

  try {
    await currentUserStore.fetchMe({ force: true })
  } catch {}
}

function installAuthStateSync(): void {
  // Реагируем на logout сразу после очистки session а не только при следующей навигации
  watch(session.isAuthenticated, async (isAuthenticated) => {
    if (isAuthenticated) {
      return
    }

    currentUserStore.clear()

    if (router.currentRoute.value.meta.requiresAuth) {
      await router.replace({ name: 'login' })
    }
  })
}

export async function initializeAuthFlow(): Promise<void> {
  installAuthStateSync()
  await bootstrapCurrentUser()
}
