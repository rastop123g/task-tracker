<script setup lang="ts">
import { ref, watch } from 'vue'
import type { HTMLAttributes } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { CircleAlert, GalleryVerticalEnd, LoaderCircle } from 'lucide-vue-next'
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
    await router.push({ name: 'home' })
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

    <form novalidate @submit.prevent="handleSubmit">
      <FieldGroup class="gap-5">
        <div class="flex flex-col gap-2">
          <div class="flex flex-col gap-2 font-medium">
            <div class="flex size-8 rounded-md">
              <GalleryVerticalEnd class="size-6" />
            </div>
            <span class="sr-only">Трекер задач</span>
          </div>
          <h1 class="text-xl font-bold">Добро пожаловать</h1>
          <FieldDescription>
            У вас нет аккаунта?
            <RouterLink to="/auth/register" class="text-primary underline-offset-4 hover:underline">
              Создать
            </RouterLink>
          </FieldDescription>
        </div>
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
        <Field>
          <div class="flex justify-center">
            <Button :disabled="isLoading" type="submit" class="w-full">
              <LoaderCircle v-if="isLoading" class="size-4 animate-spin" />
              <span> Войти </span>
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
