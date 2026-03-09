<script setup lang="ts">
import { ref, watch } from 'vue'
import type { HTMLAttributes } from 'vue'
import { RouterLink } from 'vue-router'
import { CircleAlert, GalleryVerticalEnd, LoaderCircle } from 'lucide-vue-next'
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
        'flex w-full max-w-sm flex-col gap-3 rounded-xl bg-card px-10 pb-10 pt-5 shadow-sm',
        props.class,
      )
    "
  >
    <div class="min-h-12">
      <Transition name="form-alert" appear>
        <Alert v-if="formError" variant="destructive">
          <CircleAlert />
          <AlertDescription>{{ formError }}</AlertDescription>
        </Alert>
      </Transition>
    </div>

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
      <FieldGroup class="gap-5">
        <div class="flex flex-col gap-2">
          <div class="flex flex-col gap-2 font-medium">
            <div class="flex size-8 rounded-md">
              <GalleryVerticalEnd class="size-6" />
            </div>
            <span class="sr-only">Трекер задач</span>
          </div>
          <h1 class="text-xl font-bold">Создание аккаунта</h1>
          <FieldDescription>
            Уже есть аккаунт?
            <RouterLink to="/auth/login" class="text-primary underline-offset-4 hover:underline">
              Войти
            </RouterLink>
          </FieldDescription>
        </div>
        <Field class="gap-2">
          <FieldLabel for="name">Имя</FieldLabel>
          <Input
            id="name"
            v-model="name"
            type="text"
            placeholder="Иван Иванов"
            required
            :aria-invalid="Boolean(fieldErrors.name)"
          />
          <div class="min-h-5">
            <FieldError :errors="fieldErrors.name ? [fieldErrors.name] : []" />
          </div>
        </Field>
        <Field class="gap-2">
          <FieldLabel for="email">Электронная почта</FieldLabel>
          <Input
            id="email"
            v-model="email"
            type="email"
            placeholder="m@example.com"
            required
            :aria-invalid="Boolean(fieldErrors.email)"
          />
          <div class="min-h-5">
            <FieldError :errors="fieldErrors.email ? [fieldErrors.email] : []" />
          </div>
        </Field>
        <Field class="gap-2">
          <FieldLabel for="password">Пароль</FieldLabel>
          <Input
            id="password"
            v-model="password"
            type="password"
            placeholder="••••••••"
            required
            :aria-invalid="Boolean(fieldErrors.password)"
          />
          <div class="min-h-5">
            <FieldError :errors="fieldErrors.password ? [fieldErrors.password] : []" />
          </div>
        </Field>
        <Field class="gap-2">
          <FieldLabel for="confirm-password">Повторите пароль</FieldLabel>
          <Input
            id="confirm-password"
            v-model="confirmPassword"
            type="password"
            placeholder="••••••••"
            required
            :aria-invalid="Boolean(fieldErrors.confirmPassword)"
          />
          <div class="min-h-5">
            <FieldError
              :errors="fieldErrors.confirmPassword ? [fieldErrors.confirmPassword] : []"
            />
          </div>
        </Field>
        <Field>
          <div class="flex justify-center">
            <Button :disabled="isLoading" type="submit" class="w-full">
              <LoaderCircle v-if="isLoading" class="size-4 animate-spin" />
              <span> Зарегистрироваться </span>
            </Button>
          </div>
        </Field>
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
