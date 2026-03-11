<script setup lang="ts">
import { ref, watch } from 'vue'
import type { HTMLAttributes } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { CircleAlert, LoaderCircle } from 'lucide-vue-next'
import { cn } from '@/lib/utils'
import {
  getFieldErrors,
  loginSchema,
  type FieldErrors,
  type LoginFormValues,
} from '@/lib/validation/auth'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { api, getApiErrorMessage } from '@/api'
import { currentUserStore } from '@/app/session/current-user-store'
import { areasStore } from '@/app/areas/areas-store'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()

const email = ref('')
const password = ref('')
const isLoading = ref(false)
const fieldErrors = ref<FieldErrors<LoginFormValues>>({})
const formError = ref<string | null>(null)
const route = useRoute()
const router = useRouter()

function clearFieldError(field: keyof LoginFormValues): void {
  if (!fieldErrors.value[field]) {
    return
  }

  fieldErrors.value = {
    ...fieldErrors.value,
    [field]: undefined,
  }
}

watch(
  () => route.query.email,
  (queryEmail) => {
    if (typeof queryEmail === 'string' && queryEmail.length > 0) {
      email.value = queryEmail
    }
  },
  { immediate: true },
)

watch(email, () => {
  clearFieldError('email')
  formError.value = null
})

watch(password, () => {
  clearFieldError('password')
  formError.value = null
})

async function handleSubmit(): Promise<void> {
  fieldErrors.value = {}
  formError.value = null

  const validationResult = loginSchema.safeParse({
    email: email.value,
    password: password.value,
  })

  if (!validationResult.success) {
    fieldErrors.value = getFieldErrors(validationResult.error)
    return
  }

  isLoading.value = true
  try {
    await api.auth.login(validationResult.data)
    await currentUserStore.fetchMe({ force: true })
    if (areasStore.isEmpty.value) {
      await router.push({ name: 'welcome' })
    } else {
      await router.push({ name: 'home' })
    }
  } catch (err: unknown) {
    formError.value = getApiErrorMessage(err, {
      status: { 401: 'Неверный email или пароль' },
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

    <form novalidate @submit.prevent="handleSubmit">
      <FieldGroup class="gap-4">
        <div class="flex flex-col gap-2.5 pb-1">
          <h1 class="text-xl font-bold sm:text-2xl">Вход в аккаунт</h1>
          <FieldDescription class="text-sm">
            Введите email и пароль, чтобы продолжить работу
          </FieldDescription>
        </div>
        <div class="flex flex-col gap-3">
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
            <div class="flex items-center justify-between gap-3">
              <FieldLabel for="password" class="text-sm font-medium">Пароль</FieldLabel>
              <button
                type="button"
                class="text-muted-foreground text-sm underline-offset-4 transition-colors hover:text-foreground hover:underline"
              >
                Забыли пароль?
              </button>
            </div>
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
        </div>
        <Field>
          <div class="flex justify-center">
            <Button :disabled="isLoading" type="submit" class="h-11 w-full rounded-lg text-base">
              <LoaderCircle v-if="isLoading" class="size-4 animate-spin" />
              <span>Войти</span>
            </Button>
          </div>
        </Field>
        <div class="text-muted-foreground pt-1 text-center text-sm">
          Ещё нет аккаунта?
          <RouterLink
            to="/auth/register"
            class="text-foreground font-medium underline underline-offset-4 hover:text-primary"
          >
            Создать аккаунт
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
