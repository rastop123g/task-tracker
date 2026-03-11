import { z } from 'zod'
export { getFieldErrors } from './auth'

export const areaCreateSchema = z.object({
  name: z.string().trim().min(1, 'Введите название').max(128, 'Максимум 128 символов'),
  statuses: z
    .array(
      z.object({
        id: z.string(),
        label: z.string(),
        color: z.string(),
        category: z.enum(['Новая', 'Отменена', 'В работе', 'На тестировании']),
      }),
    )
    .min(1, 'Добавьте хотя бы один статус'),
  tags: z.array(
    z.object({
      id: z.string(),
      label: z.string(),
      color: z.string(),
    }),
  ),
})

export type AreaCreateFormValues = z.infer<typeof areaCreateSchema>
