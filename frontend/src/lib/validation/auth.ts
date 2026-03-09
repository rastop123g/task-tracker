import { z } from 'zod'

export type FieldErrors<TValues extends Record<string, unknown>> = Partial<
  Record<keyof TValues, string>
>

export function getFieldErrors<TValues extends Record<string, unknown>>(
  error: z.ZodError<TValues>,
): FieldErrors<TValues> {
  const fieldErrors: FieldErrors<TValues> = {}

  for (const issue of error.issues) {
    const [firstPathSegment] = issue.path
    if (typeof firstPathSegment !== 'string') {
      continue
    }

    const key = firstPathSegment as keyof TValues
    if (!fieldErrors[key]) {
      fieldErrors[key] = issue.message
    }
  }

  return fieldErrors
}

export const loginSchema = z.object({
  email: z.string().trim().min(1, 'Введите email').email('Введите корректный email'),
  password: z
    .string()
    .min(1, 'Введите пароль')
    .min(8, 'Пароль должен быть от 8 до 128 символов')
    .max(128, 'Пароль должен быть от 8 до 128 символов'),
})

export type LoginFormValues = z.infer<typeof loginSchema>

export const registerSchema = z
  .object({
    name: z
      .string()
      .trim()
      .min(1, 'Введите имя')
      .min(3, 'Имя должно быть от 3 до 128 символов')
      .max(128, 'Имя должно быть от 3 до 128 символов'),
    email: z.string().trim().min(1, 'Введите email').email('Введите корректный email'),
    password: z
      .string()
      .min(1, 'Введите пароль')
      .min(8, 'Пароль должен быть от 8 до 128 символов')
      .max(128, 'Пароль должен быть от 8 до 128 символов'),
    confirmPassword: z.string().min(1, 'Повторите пароль'),
  })
  .refine((data) => data.password === data.confirmPassword, {
    path: ['confirmPassword'],
    message: 'Пароли не совпадают',
  })

export type RegisterFormValues = z.infer<typeof registerSchema>
