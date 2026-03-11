<script setup lang="ts">
import { ref, watch } from 'vue'
import type { HTMLAttributes } from 'vue'
import { RouterLink } from 'vue-router'
import { CircleAlert, LoaderCircle } from 'lucide-vue-next'
import { cn } from '@/lib/utils'
import {
  getFieldErrors,
  registerSchema,
  type FieldErrors,
  type RegisterFormValues,
} from '@/lib/validation/auth'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { api, getApiErrorMessage } from '@/api'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()

const name = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')

const isLoading = ref(false)
const fieldErrors = ref<FieldErrors<RegisterFormValues>>({})
const formError = ref<string | null>(null)
const registrationSent = ref(false)

function clearFieldError(field: keyof RegisterFormValues): void {
  if (!fieldErrors.value[field]) {
    return
  }

  fieldErrors.value = {
    ...fieldErrors.value,
    [field]: undefined,
  }
}

watch(name, () => {
  clearFieldError('name')
  formError.value = null
})

watch(email, () => {
  clearFieldError('email')
  formError.value = null
})

watch(password, () => {
  clearFieldError('password')
  clearFieldError('confirmPassword')
  formError.value = null
})

watch(confirmPassword, () => {
  clearFieldError('confirmPassword')
  formError.value = null
})

async function handleSubmit(): Promise<void> {
  fieldErrors.value = {}
  formError.value = null

  const validationResult = registerSchema.safeParse({
    name: name.value,
    email: email.value,
    password: password.value,
    confirmPassword: confirmPassword.value,
  })

  if (!validationResult.success) {
    fieldErrors.value = getFieldErrors(validationResult.error)
    return
  }

  isLoading.value = true
  try {
    await api.auth.register(validationResult.data)
    registrationSent.value = true
  } catch (err: unknown) {
    formError.value = getApiErrorMessage(err, {
      backendError: { 'email already used': 'Этот email уже используется' },
    })
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div
    :class="
      cn(
        'flex w-full max-w-sm flex-col rounded-xl bg-card px-6 py-6 shadow-sm sm:px-8 sm:py-7',
        props.class,
      )
    "
  >
    <Transition name="form-alert" appear>
      <Alert v-if="formError" variant="destructive" class="mb-4">
        <CircleAlert />
        <AlertDescription>{{ formError }}</AlertDescription>
      </Alert>
    </Transition>

    <div v-if="registrationSent" class="flex flex-col items-center gap-4">
      <p class="text-lg font-semibold">Аккаунт создан</p>
      <p class="text-sm text-muted-foreground">Проверьте почту для подтверждения.</p>
      <RouterLink
        :to="{ name: 'login', query: { email } }"
        class="text-primary underline-offset-4 hover:underline text-sm"
      >
        Войти в аккаунт
      </RouterLink>
    </div>

    <form v-else novalidate @submit.prevent="handleSubmit">
      <FieldGroup class="gap-4">
        <div class="flex flex-col gap-2.5 pb-1">
          <h1 class="text-xl font-bold sm:text-2xl">Создание аккаунта</h1>
          <FieldDescription class="text-sm">
            Введите данные, чтобы создать аккаунт
          </FieldDescription>
        </div>
        <div class="flex flex-col gap-3">
          <Field class="gap-2">
            <FieldLabel for="name" class="text-sm font-medium">Имя</FieldLabel>
            <Input
              id="name"
              v-model="name"
              type="text"
              placeholder="Иван Иванов"
              required
              :aria-invalid="Boolean(fieldErrors.name)"
              class="h-11 rounded-lg px-4"
            />
            <div class="min-h-4">
              <FieldError :errors="fieldErrors.name ? [fieldErrors.name] : []" />
            </div>
          </Field>
          <Field class="gap-2">
            <FieldLabel for="email" class="text-sm font-medium">Электронная почта</FieldLabel>
            <Input
              id="email"
              v-model="email"
              type="email"
              placeholder="m@example.com"
              required
              :aria-invalid="Boolean(fieldErrors.email)"
              class="h-11 rounded-lg px-4"
            />
            <div class="min-h-4">
              <FieldError :errors="fieldErrors.email ? [fieldErrors.email] : []" />
            </div>
          </Field>
          <Field class="gap-2">
            <FieldLabel for="password" class="text-sm font-medium">Пароль</FieldLabel>
            <Input
              id="password"
              v-model="password"
              type="password"
              placeholder="••••••••"
              required
              :aria-invalid="Boolean(fieldErrors.password)"
              class="h-11 rounded-lg px-4"
            />
            <div class="min-h-4">
              <FieldError :errors="fieldErrors.password ? [fieldErrors.password] : []" />
            </div>
          </Field>
          <Field class="gap-2">
            <FieldLabel for="confirm-password" class="text-sm font-medium">
              Повторите пароль
            </FieldLabel>
            <Input
              id="confirm-password"
              v-model="confirmPassword"
              type="password"
              placeholder="••••••••"
              required
              :aria-invalid="Boolean(fieldErrors.confirmPassword)"
              class="h-11 rounded-lg px-4"
            />
            <div class="min-h-4">
              <FieldError
                :errors="fieldErrors.confirmPassword ? [fieldErrors.confirmPassword] : []"
              />
            </div>
          </Field>
        </div>
        <Field>
          <div class="flex justify-center">
            <Button :disabled="isLoading" type="submit" class="h-11 w-full rounded-lg text-base">
              <LoaderCircle v-if="isLoading" class="size-4 animate-spin" />
              <span>Зарегистрироваться</span>
            </Button>
          </div>
        </Field>
        <div class="text-muted-foreground pt-1 text-center text-sm">
          Уже есть аккаунт?
          <RouterLink
            to="/auth/login"
            class="text-foreground font-medium underline underline-offset-4 hover:text-primary"
          >
            Войти
          </RouterLink>
        </div>
      </FieldGroup>
    </form>
  </div>
</template>

<style scoped>
.form-alert-enter-active,
.form-alert-leave-active {
  transition:
    opacity 180ms ease,
    transform 180ms ease;
}

.form-alert-enter-from,
.form-alert-leave-to {
  opacity: 0;
  transform: translateY(-6px);
}

.form-alert-enter-to,
.form-alert-leave-from {
  opacity: 1;
  transform: translateY(0);
}
</style>
