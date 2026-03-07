<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useProfile } from '@/composables/useProfile'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Skeleton } from '@/components/ui/skeleton'
import { Separator } from '@/components/ui/separator'
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
    saveError.value = e instanceof Error ? e.message : 'Save failed'
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
    uploadError.value = e instanceof Error ? e.message : 'Upload failed'
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
      <!-- Page header -->
      <div class="mb-6">
        <h1 class="text-2xl font-semibold tracking-tight">Profile</h1>
        <p class="text-sm text-muted-foreground mt-1">
          Manage your account settings and preferences.
        </p>
      </div>

      <!-- Card -->
      <div class="rounded-xl border bg-card text-card-foreground shadow-sm">
        <!-- Loading skeleton -->
        <template v-if="isLoading && !profile">
          <div class="p-6 space-y-6">
            <div class="flex items-center gap-4">
              <Skeleton class="size-16 rounded-full shrink-0" />
              <div class="space-y-2 flex-1">
                <Skeleton class="h-4 w-32" />
                <Skeleton class="h-3 w-48" />
              </div>
            </div>
            <Separator />
            <div class="space-y-4">
              <div class="space-y-2">
                <Skeleton class="h-4 w-24" />
                <Skeleton class="h-9 w-full" />
              </div>
              <div class="space-y-2">
                <Skeleton class="h-4 w-16" />
                <Skeleton class="h-9 w-full" />
              </div>
            </div>
          </div>
        </template>

        <!-- Error state -->
        <template v-else-if="error && !profile">
          <div class="p-6">
            <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4">
              <p class="text-sm text-destructive">{{ error }}</p>
              <Button
                variant="link"
                size="sm"
                class="mt-2 h-auto p-0 text-destructive"
                @click="fetchProfile"
              >
                Try again
              </Button>
            </div>
          </div>
        </template>

        <!-- Profile content -->
        <template v-else-if="profile">
          <!-- Avatar section -->
          <div class="p-6 flex items-center gap-4">
            <button
              type="button"
              class="relative size-16 rounded-full overflow-hidden ring-2 ring-border shrink-0 focus-visible:outline-none focus-visible:ring-ring transition-opacity"
              :class="{
                'opacity-60 cursor-not-allowed': isUploading || !avatarUrl,
                'cursor-pointer hover:opacity-80': !isUploading && avatarUrl,
              }"
              :disabled="isUploading || !avatarUrl"
              :title="
                !avatarUrl ? 'Avatar upload unavailable (no user ID)' : 'Click to upload avatar'
              "
              @click="onAvatarClick"
            >
              <img
                v-if="avatarUrlWithBust && !showInitials"
                :src="avatarUrlWithBust"
                alt="Avatar"
                class="size-full object-cover"
                @error="showInitials = true"
              />
              <div
                v-else
                class="size-full flex items-center justify-center bg-sidebar-primary text-sidebar-primary-foreground text-xl font-semibold"
              >
                {{ initials }}
              </div>
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

            <input
              ref="fileInputRef"
              type="file"
              accept="image/png,image/jpeg,image/gif"
              class="hidden"
              @change="onFileChange"
            />

            <div class="min-w-0">
              <p class="font-medium truncate">{{ profile.name || profile.email }}</p>
              <p class="text-sm text-muted-foreground truncate">{{ profile.email }}</p>
              <p v-if="!avatarUrl" class="text-xs text-muted-foreground mt-1">
                Re-login to enable avatar upload.
              </p>
              <p v-if="uploadError" class="text-xs text-destructive mt-1">{{ uploadError }}</p>
            </div>
          </div>

          <Separator />

          <!-- Fields section -->
          <div class="p-6 space-y-4">
            <!-- Display name -->
            <div class="grid gap-2">
              <Label for="profile-name">Display name</Label>
              <Input
                id="profile-name"
                v-model="localName"
                type="text"
                placeholder="Your name"
                :disabled="isSaving"
              />
              <p v-if="saveError" class="text-xs text-destructive">{{ saveError }}</p>
            </div>

            <!-- Email (read-only) -->
            <div class="grid gap-2">
              <Label for="profile-email">Email</Label>
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
  </div>
</template>
