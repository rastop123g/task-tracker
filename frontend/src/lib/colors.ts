import type { ColorSchema } from '@/api/generated/schema'

type ColorInfo = { hex: string; label: string; tw: string }

export const COLOR_MAP: Record<ColorSchema, ColorInfo> = {
  red: { hex: '#ef4444', label: 'Красный', tw: 'bg-red-500' },
  green: { hex: '#22c55e', label: 'Зелёный', tw: 'bg-green-500' },
  blue: { hex: '#3b82f6', label: 'Синий', tw: 'bg-blue-500' },
  yellow: { hex: '#eab308', label: 'Жёлтый', tw: 'bg-yellow-500' },
  pink: { hex: '#ec4899', label: 'Розовый', tw: 'bg-pink-500' },
  purple: { hex: '#a855f7', label: 'Фиолетовый', tw: 'bg-purple-500' },
  orange: { hex: '#f97316', label: 'Оранжевый', tw: 'bg-orange-500' },
  brown: { hex: '#92400e', label: 'Коричневый', tw: 'bg-amber-800' },
  gray: { hex: '#6b7280', label: 'Серый', tw: 'bg-gray-500' },
}

export const COLOR_LIST = Object.entries(COLOR_MAP) as [ColorSchema, ColorInfo][]

export const DEFAULT_COLOR: ColorSchema = 'red'
