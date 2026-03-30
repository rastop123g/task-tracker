import type { StatusCategorySchema } from '@/api/generated/schema'

export const STATUS_CATEGORY_MAP: Record<StatusCategorySchema, string> = {
  task_difinition: 'Определение задачи',
  work_waiting: 'Ожидает работы',
  work_in_progress: 'В работе',
  blocked: 'Заблокировано',
  test_waiting: 'Ожидает тестирования',
  test_in_progress: 'На тестировании',
  done: 'Готово',
  canceled: 'Отменено',
}

export const STATUS_CATEGORY_LIST = Object.entries(STATUS_CATEGORY_MAP) as [
  StatusCategorySchema,
  string,
][]
