<script setup lang="ts">
import { ref } from 'vue'
import type { HTMLAttributes } from 'vue'
import { RouterLink } from 'vue-router'
import { GalleryVerticalEnd } from 'lucide-vue-next'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { api } from '@/api'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()

const name = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')

const isLoading = ref(false)
const errorMsg = ref<string | null>(null)
const registrationSent = ref(false)
const EMAIL_RE = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

async function handleSubmit(): Promise<void> {
  errorMsg.value = null

  if (name.value.length < 3 || name.value.length > 128) {
    errorMsg.value = 'Имя должно быть от 3 до 128 символов'
    return
  }

  if (!EMAIL_RE.test(email.value)) {
    errorMsg.value = 'Введите корректный email'
    return
  }

  if (password.value.length < 8 || password.value.length > 128) {
    errorMsg.value = 'Пароль должен быть от 8 до 128 символов'
    return
  }

  if (password.value !== confirmPassword.value) {
    errorMsg.value = 'Пароли не совпадают'
    return
  }

  isLoading.value = true
  try {
    await api.auth.register({ name: name.value, email: email.value, password: password.value })
    registrationSent.value = true
  } catch (err: unknown) {
    const fallback = 'Произошла ошибка. Попробуйте позже.'
    if (err instanceof Response) {
      if (err.status === 400) {
        try {
          const data = (await err.json()) as { error?: string }
          if (data.error === 'email already used') {
            errorMsg.value = 'Этот email уже используется'
          } else {
            errorMsg.value = 'Проверьте корректность введенных данных'
          }
        } catch {
          errorMsg.value = 'Проверьте корректность введенных данных'
        }
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
    <div v-if="registrationSent" class="flex flex-col items-center gap-4 text-center">
      <p class="text-lg font-semibold">Аккаунт создан</p>
      <p class="text-sm text-muted-foreground">Проверьте почту для подтверждения.</p>
      <RouterLink to="/auth/login" class="text-primary underline-offset-4 hover:underline text-sm">
        Войти в аккаунт
      </RouterLink>
    </div>

    <form v-else @submit.prevent="handleSubmit">
      <FieldGroup>
        <div class="flex flex-col items-center gap-2 text-center">
          <div class="flex flex-col items-center gap-2 font-medium">
            <div class="flex size-8 items-center justify-center rounded-md">
              <GalleryVerticalEnd class="size-6" />
            </div>
            <span class="sr-only">Task Tracker</span>
          </div>
          <h1 class="text-xl font-bold">Создание аккаунта</h1>
          <FieldDescription>
            Уже есть аккаунт?
            <RouterLink to="/auth/login" class="text-primary underline-offset-4 hover:underline">
              Войти
            </RouterLink>
          </FieldDescription>
        </div>
        <Field>
          <FieldLabel for="name">Имя</FieldLabel>
          <Input id="name" type="text" placeholder="Иван Иванов" required v-model="name" />
        </Field>
        <Field>
          <FieldLabel for="email">Email</FieldLabel>
          <Input id="email" type="email" placeholder="m@example.com" required v-model="email" />
        </Field>
        <Field>
          <FieldLabel for="password">Пароль</FieldLabel>
          <Input id="password" type="password" placeholder="••••••••" required v-model="password" />
        </Field>
        <Field>
          <FieldLabel for="confirm-password">Повторите пароль</FieldLabel>
          <Input
            id="confirm-password"
            type="password"
            placeholder="••••••••"
            required
            v-model="confirmPassword"
          />
        </Field>
        <FieldError :errors="errorMsg ? [errorMsg] : []" />
        <Field>
          <Button :disabled="isLoading" type="submit" class="w-full">
            {{ isLoading ? 'Загрузка...' : 'Зарегистрироваться' }}
          </Button>
        </Field>
      </FieldGroup>
    </form>
  </div>
</template>
