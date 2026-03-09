<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useProfile } from '@/composables/useProfile'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Skeleton } from '@/components/ui/skeleton'
import { useSettingsActions } from '@/composables/useSettingsActions'

const { profile, isLoading, error, avatarUrl, fetchProfile, updateName, uploadAvatar } =
  useProfile()

const localName = ref('')
const saveError = ref<string | null>(null)

const { register, isSaving } = useSettingsActions()
register(async () => {
  saveError.value = null
  try {
    await updateName(localName.value)
  } catch (e) {
    saveError.value = e instanceof Error ? e.message : 'Не удалось сохранить изменения'
  }
})

const cacheBust = ref(0)
const avatarUrlWithBust = computed(() =>
  avatarUrl.value ? `${avatarUrl.value}?t=${cacheBust.value}` : null,
)

const isUploading = ref(false)
const uploadError = ref<string | null>(null)
const showInitials = ref(false)

const initials = computed(() => {
  const src = profile.value?.name || profile.value?.email || ''
  return src.charAt(0).toUpperCase() || '?'
})

const fileInputRef = ref<HTMLInputElement | null>(null)

function onAvatarClick() {
  if (isUploading.value || !avatarUrl.value) return
  fileInputRef.value?.click()
}

async function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  isUploading.value = true
  uploadError.value = null
  try {
    await uploadAvatar(file)
    cacheBust.value++
    showInitials.value = false
  } catch (e) {
    uploadError.value = e instanceof Error ? e.message : 'Не удалось загрузить аватар'
  } finally {
    isUploading.value = false
    input.value = ''
  }
}

onMounted(async () => {
  await fetchProfile()
  if (profile.value) {
    localName.value = profile.value.name
  }
})
</script>

<template>
  <div class="p-6">
    <div class="max-w-lg">
      <!-- Заголовок страницы -->
      <div class="mb-8">
        <h1 class="text-2xl font-semibold tracking-tight">Профиль</h1>
        <p class="text-sm text-muted-foreground mt-1">
          Управляйте настройками и параметрами своей учетной записи.
        </p>
      </div>

      <!-- Скелетон загрузки -->
      <template v-if="isLoading && !profile">
        <div class="space-y-8">
          <div class="flex items-end gap-5">
            <Skeleton class="size-20 rounded-full shrink-0" />
            <div class="space-y-2 pb-1">
              <Skeleton class="h-4 w-32" />
              <Skeleton class="h-3 w-48" />
            </div>
          </div>
          <div class="space-y-5">
            <div class="space-y-2">
              <Skeleton class="h-3.5 w-28" />
              <Skeleton class="h-9 w-full" />
            </div>
            <div class="space-y-2">
              <Skeleton class="h-3.5 w-20" />
              <Skeleton class="h-9 w-full" />
            </div>
          </div>
        </div>
      </template>

      <!-- Состояние ошибки -->
      <template v-else-if="error && !profile">
        <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4">
          <p class="text-sm text-destructive">{{ error }}</p>
          <Button
            variant="link"
            size="sm"
            class="mt-2 h-auto p-0 text-destructive"
            @click="fetchProfile"
          >
            Попробовать снова
          </Button>
        </div>
      </template>

      <!-- Содержимое профиля -->
      <template v-else-if="profile">
        <!-- Аватар -->
        <div class="mb-8">
          <button
            type="button"
            class="group relative size-20 rounded-full overflow-hidden ring-1 ring-border shrink-0 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring transition-opacity"
            :class="{
              'opacity-60 cursor-not-allowed': isUploading || !avatarUrl,
              'cursor-pointer': !isUploading && avatarUrl,
            }"
            :disabled="isUploading || !avatarUrl"
            :title="
              !avatarUrl
                ? 'Загрузка аватара недоступна: отсутствует идентификатор пользователя'
                : 'Нажмите, чтобы загрузить аватар'
            "
            @click="onAvatarClick"
          >
            <img
              v-if="avatarUrlWithBust && !showInitials"
              :src="avatarUrlWithBust"
              alt="Аватар"
              class="size-full object-cover"
              @error="showInitials = true"
            />
            <div
              v-else
              class="size-full flex items-center justify-center bg-sidebar-primary text-sidebar-primary-foreground text-2xl font-semibold"
            >
              {{ initials }}
            </div>

            <!-- Hover overlay с иконкой камеры -->
            <div
              v-if="!isUploading && avatarUrl"
              class="absolute inset-0 flex items-center justify-center bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity"
            >
              <svg
                class="size-5 text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="1.8"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M6.827 6.175A2.31 2.31 0 0 1 5.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 0 0-1.134-.175 2.31 2.31 0 0 1-1.64-1.055l-.822-1.316a2.192 2.192 0 0 0-1.736-1.039 48.774 48.774 0 0 0-5.232 0 2.192 2.192 0 0 0-1.736 1.039l-.821 1.316Z"
                />
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M16.5 12.75a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0ZM18.75 10.5h.008v.008h-.008V10.5Z"
                />
              </svg>
            </div>

            <!-- Спиннер загрузки -->
            <div
              v-if="isUploading"
              class="absolute inset-0 flex items-center justify-center bg-black/50"
            >
              <svg
                class="size-5 animate-spin text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                />
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
                />
              </svg>
            </div>
          </button>

          <p v-if="uploadError" class="text-xs text-destructive mt-2">{{ uploadError }}</p>

          <input
            ref="fileInputRef"
            type="file"
            accept="image/png,image/jpeg,image/gif"
            class="hidden"
            @change="onFileChange"
          />
        </div>

        <!-- Поля формы -->
        <div class="space-y-5">
          <!-- Отображаемое имя -->
          <div class="grid gap-2">
            <Label for="profile-name">Отображаемое имя</Label>
            <Input
              id="profile-name"
              v-model="localName"
              type="text"
              placeholder="Ваше имя"
              :disabled="isSaving"
            />
            <p v-if="saveError" class="text-xs text-destructive">{{ saveError }}</p>
          </div>

          <!-- Электронная почта (только чтение) -->
          <div class="grid gap-2">
            <Label for="profile-email">Электронная почта</Label>
            <Input
              id="profile-email"
              :model-value="profile.email"
              type="email"
              readonly
              class="bg-muted text-muted-foreground cursor-not-allowed"
            />
          </div>
        </div>
      </template>
    </div>
  </div>
</template>
