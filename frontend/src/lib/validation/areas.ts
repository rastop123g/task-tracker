import { z } from 'zod'
export { getFieldErrors } from './auth'

const colorValues = [
  'red',
  'green',
  'blue',
  'yellow',
  'pink',
  'purple',
  'orange',
  'brown',
  'gray',
] as const

const statusCategoryValues = [
  'task_difinition',
  'work_waiting',
  'work_in_progress',
  'blocked',
  'test_waiting',
  'test_in_progress',
  'done',
  'canceled',
] as const

export const areaCreateSchema = z.object({
  name: z
    .string()
    .trim()
    .min(1, 'Введите название')
    .min(3, 'Название должно быть от 3 до 128 символов')
    .max(128, 'Название должно быть от 3 до 128 символов'),
  statuses: z
    .array(
      z.object({
        name: z.string().min(3, 'Минимум 3 символа').max(64, 'Максимум 64 символа'),
        color: z.enum(colorValues),
        category: z.enum(statusCategoryValues),
      }),
    )
    .min(1, 'Добавьте хотя бы один статус'),
  tags: z.array(
    z.object({
      name: z.string().min(3, 'Минимум 3 символа').max(64, 'Максимум 64 символа'),
      color: z.enum(colorValues),
    }),
  ),
})

export type AreaCreateFormValues = z.infer<typeof areaCreateSchema>
