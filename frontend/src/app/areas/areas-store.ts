import { computed, ref } from 'vue'

export type StatusCategory = 'Новая' | 'Отменена' | 'В работе' | 'На тестировании'
export type Status = { id: string; label: string; color: string; category: StatusCategory }
export type Tag = { id: string; label: string; color: string }
export type Template = { id: string; name: string }
export type Area = {
  id: string
  name: string
  avatarUrl: string | null
  statuses: Status[]
  tags: Tag[]
  templateId: string | null
}

const areas = ref<Area[]>([])
const isEmpty = computed(() => areas.value.length === 0)

function addArea(area: Area): void {
  areas.value.push(area)
}

function clear(): void {
  areas.value = []
}

export const areasStore = { areas, isEmpty, addArea, clear }

export const MOCK_TEMPLATES: Template[] = [
  { id: 'kanban', name: 'Kanban' },
  { id: 'simple', name: 'Simple list' },
  { id: 'bugs', name: 'Bug tracker' },
]
