<script setup lang="ts">
import { ref } from 'vue'
import type { HTMLAttributes } from 'vue'
import { RouterLink, useRouter } from 'vue-router'
import { GalleryVerticalEnd } from 'lucide-vue-next'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { api } from '@/api'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()

const email = ref('')
const password = ref('')
const isLoading = ref(false)
const errorMsg = ref<string | null>(null)
const router = useRouter()
const EMAIL_RE = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

async function handleSubmit(): Promise<void> {
  errorMsg.value = null

  if (!EMAIL_RE.test(email.value)) {
    errorMsg.value = 'Введите корректный email'
    return
  }

  if (password.value.length < 8 || password.value.length > 128) {
    errorMsg.value = 'Пароль должен быть от 8 до 128 символов'
    return
  }

  isLoading.value = true
  try {
    await api.auth.login({ email: email.value, password: password.value })
    await router.push({ name: 'home' })
  } catch (err: unknown) {
    const fallback = 'Произошла ошибка. Попробуйте позже.'
    if (err instanceof Response) {
      if (err.status === 401) {
        errorMsg.value = 'Неверный email или пароль'
      } else if (err.status >= 500) {
        errorMsg.value = 'Ошибка сервера. Попробуйте позже'
      } else {
        errorMsg.value = fallback
      }
    } else if (err instanceof Error && err.message) {
      errorMsg.value = 'Не удалось выполнить запрос. Проверьте соединение'
    } else {
      errorMsg.value = fallback
    }
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div
    :class="
      cn('flex w-full max-w-sm flex-col gap-6 rounded-xl bg-card p-6 sm:p-8 shadow-sm', props.class)
    "
  >
    <form @submit.prevent="handleSubmit">
      <FieldGroup>
        <div class="flex flex-col items-center gap-2 text-center">
          <div class="flex flex-col items-center gap-2 font-medium">
            <div class="flex size-8 items-center justify-center rounded-md">
              <GalleryVerticalEnd class="size-6" />
            </div>
            <span class="sr-only">Task Tracker</span>
          </div>
          <h1 class="text-xl font-bold">Добро пожаловать</h1>
          <FieldDescription>
            У вас нет аккаунта?
            <RouterLink to="/auth/register" class="text-primary underline-offset-4 hover:underline">
              Создать
            </RouterLink>
          </FieldDescription>
        </div>
        <Field>
          <FieldLabel for="email">Email</FieldLabel>
          <Input id="email" type="email" placeholder="m@example.com" required v-model="email" />
        </Field>
        <Field>
          <FieldLabel for="password">Пароль</FieldLabel>
          <Input id="password" type="password" placeholder="••••••••" required v-model="password" />
        </Field>
        <FieldError :errors="errorMsg ? [errorMsg] : []" />
        <Field>
          <Button :disabled="isLoading" type="submit" class="w-full">
            {{ isLoading ? 'Загрузка...' : 'Войти' }}
          </Button>
        </Field>
      </FieldGroup>
    </form>
  </div>
</template>
