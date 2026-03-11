<script setup lang="ts">
import { onBeforeUnmount, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ImagePlus, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Field, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  areasStore,
  MOCK_TEMPLATES,
  type Status,
  type StatusCategory,
  type Tag,
  type Template,
} from '@/app/areas/areas-store'
import { areaCreateSchema, getFieldErrors, type AreaCreateFormValues } from '@/lib/validation/areas'
import type { FieldErrors } from '@/lib/validation/auth'

type ColorPreset = {
  name: string
  value: string
}

type StatusDraft = {
  label: string
  color: string
  category: StatusCategory | ''
}

type TagDraft = {
  label: string
  color: string
}

const STATUS_CATEGORIES: StatusCategory[] = ['Новая', 'Отменена', 'В работе', 'На тестировании']

const COLOR_PRESETS: ColorPreset[] = [
  { name: 'Красный', value: '#EF4444' },
  { name: 'Оранжевый', value: '#F97316' },
  { name: 'Желтый', value: '#EAB308' },
  { name: 'Зеленый', value: '#22C55E' },
  { name: 'Бирюзовый', value: '#06B6D4' },
  { name: 'Синий', value: '#3B82F6' },
  { name: 'Фиолетовый', value: '#8B5CF6' },
  { name: 'Розовый', value: '#EC4899' },
]

const DEFAULT_COLOR = '#EF4444'

const router = useRouter()

const name = ref('')
const avatarObjectUrl = ref<string | null>(null)
const statuses = ref<Status[]>([])
const tags = ref<Tag[]>([])
const availableTemplates = ref<Template[]>([...MOCK_TEMPLATES])
const selectedTemplateId = ref<string | undefined>(undefined)
const templateDialogOpen = ref(false)
const newTemplateName = ref('')
const statusDialogOpen = ref(false)
const tagDialogOpen = ref(false)
const editingStatusId = ref<string | null>(null)
const editingTagId = ref<string | null>(null)
const statusDraft = ref<StatusDraft>(createStatusDraft())
const tagDraft = ref<TagDraft>(createTagDraft())
const fieldErrors = ref<FieldErrors<AreaCreateFormValues>>({})
const isLoading = ref(false)

const fileInputRef = ref<HTMLInputElement | null>(null)

function createStatusDraft(): StatusDraft {
  return {
    label: '',
    color: DEFAULT_COLOR,
    category: '',
  }
}

function createTagDraft(): TagDraft {
  return {
    label: '',
    color: DEFAULT_COLOR,
  }
}

function hexToRgba(hex: string, alpha: number): string {
  const normalized = hex.replace('#', '')
  const value = Number.parseInt(normalized, 16)
  const r = (value >> 16) & 255
  const g = (value >> 8) & 255
  const b = value & 255

  return `rgba(${r}, ${g}, ${b}, ${alpha})`
}

function getBadgeStyles(color: string): Record<string, string> {
  return {
    backgroundColor: hexToRgba(color, 0.14),
    borderColor: hexToRgba(color, 0.28),
    color,
  }
}

function handleAvatarChange(e: Event): void {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file) return
  if (avatarObjectUrl.value) {
    URL.revokeObjectURL(avatarObjectUrl.value)
  }
  avatarObjectUrl.value = URL.createObjectURL(file)
}

function removeStatus(id: string): void {
  statuses.value = statuses.value.filter((status) => status.id !== id)
}

function removeTag(id: string): void {
  tags.value = tags.value.filter((tag) => tag.id !== id)
}

function openStatusDialog(): void {
  editingStatusId.value = null
  statusDraft.value = createStatusDraft()
  statusDialogOpen.value = true
}

function openTagDialog(): void {
  editingTagId.value = null
  tagDraft.value = createTagDraft()
  tagDialogOpen.value = true
}

function openStatusEditDialog(status: Status): void {
  editingStatusId.value = status.id
  statusDraft.value = {
    label: status.label,
    color: status.color,
    category: status.category,
  }
  statusDialogOpen.value = true
}

function openTagEditDialog(tag: Tag): void {
  editingTagId.value = tag.id
  tagDraft.value = {
    label: tag.label,
    color: tag.color,
  }
  tagDialogOpen.value = true
}

function saveStatus(): void {
  const label = statusDraft.value.label.trim()
  const category = statusDraft.value.category

  if (!label || !category) return

  if (editingStatusId.value) {
    statuses.value = statuses.value.map((status) =>
      status.id === editingStatusId.value
        ? {
            ...status,
            label,
            color: statusDraft.value.color,
            category,
          }
        : status,
    )
  } else {
    statuses.value.push({
      id: crypto.randomUUID(),
      label,
      color: statusDraft.value.color,
      category,
    })
  }
  fieldErrors.value.statuses = undefined
  editingStatusId.value = null
  statusDraft.value = createStatusDraft()
  statusDialogOpen.value = false
}

function saveTag(): void {
  const label = tagDraft.value.label.trim()
  if (!label) return

  if (editingTagId.value) {
    tags.value = tags.value.map((tag) =>
      tag.id === editingTagId.value
        ? {
            ...tag,
            label,
            color: tagDraft.value.color,
          }
        : tag,
    )
  } else {
    tags.value.push({
      id: crypto.randomUUID(),
      label,
      color: tagDraft.value.color,
    })
  }
  editingTagId.value = null
  tagDraft.value = createTagDraft()
  tagDialogOpen.value = false
}

function handleCreateTemplate(): void {
  const tplName = newTemplateName.value.trim()
  if (!tplName) return
  const id = crypto.randomUUID()
  availableTemplates.value.push({ id, name: tplName })
  selectedTemplateId.value = id
  newTemplateName.value = ''
  templateDialogOpen.value = false
}

async function handleSubmit(): Promise<void> {
  fieldErrors.value = {}

  const result = areaCreateSchema.safeParse({
    name: name.value,
    statuses: statuses.value,
    tags: tags.value,
  })

  if (!result.success) {
    fieldErrors.value = getFieldErrors(result.error)
    return
  }

  isLoading.value = true
  try {
    areasStore.addArea({
      id: crypto.randomUUID(),
      name: result.data.name,
      avatarUrl: avatarObjectUrl.value,
      statuses: result.data.statuses,
      tags: result.data.tags,
      templateId: selectedTemplateId.value ?? null,
    })
    await router.push({ name: 'home' })
  } finally {
    isLoading.value = false
  }
}

onBeforeUnmount(() => {
  if (avatarObjectUrl.value) {
    URL.revokeObjectURL(avatarObjectUrl.value)
  }
})
</script>

<template>
  <div class="min-h-screen bg-muted px-4 pt-6 pb-4 sm:px-6 sm:pt-10">
    <section class="mx-auto w-full max-w-2xl rounded-2xl border bg-card px-4 py-6 shadow-sm sm:px-6 sm:py-7">
      <div class="flex w-full flex-col gap-6">
      <div class="text-left">
        <h1 class="text-3xl font-semibold tracking-tight">Создать область</h1>
        <p class="mt-2 text-sm text-muted-foreground">Настройте рабочую область под свои задачи</p>
      </div>

      <form novalidate @submit.prevent="handleSubmit">
        <FieldGroup class="gap-6">
          <div class="flex justify-start">
            <button
              type="button"
              class="group relative flex size-24 items-center justify-center overflow-hidden rounded-full border-2 border-dashed border-border bg-muted transition-colors hover:border-primary hover:bg-muted/70 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/50"
              @click="fileInputRef?.click()"
            >
              <img
                v-if="avatarObjectUrl"
                :src="avatarObjectUrl"
                alt="Аватар"
                class="size-full object-cover"
              />
              <ImagePlus
                v-else
                class="size-8 text-muted-foreground transition-colors group-hover:text-primary"
              />
              <input
                ref="fileInputRef"
                type="file"
                accept="image/*"
                class="hidden"
                @change="handleAvatarChange"
              />
            </button>
          </div>

          <Field class="gap-2">
            <FieldLabel for="area-name">Название области</FieldLabel>
            <Input
              id="area-name"
              v-model="name"
              placeholder="Мой проект"
              :aria-invalid="Boolean(fieldErrors.name)"
              @input="fieldErrors.name = undefined"
            />
            <div class="min-h-5">
              <FieldError :errors="fieldErrors.name ? [fieldErrors.name] : []" />
            </div>
          </Field>

          <div class="flex flex-col gap-3 rounded-2xl border border-sky-200 bg-sky-50 p-4 sm:p-5">
            <div class="flex items-center justify-between gap-3">
              <FieldLabel class="text-sky-900">Статусы</FieldLabel>
              <Button type="button" variant="outline" @click="openStatusDialog">Добавить</Button>
            </div>
            <div v-if="statuses.length" class="flex flex-wrap gap-2">
              <span
                v-for="status in statuses"
                :key="status.id"
                class="inline-flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1 text-sm font-medium transition-opacity hover:opacity-85"
                :style="getBadgeStyles(status.color)"
                role="button"
                tabindex="0"
                :aria-label="`Редактировать статус ${status.label}`"
                @click="openStatusEditDialog(status)"
                @keydown.enter.prevent="openStatusEditDialog(status)"
                @keydown.space.prevent="openStatusEditDialog(status)"
              >
                {{ status.label }}
                <button
                  type="button"
                  class="flex size-4 items-center justify-center rounded-full transition-colors hover:bg-black/10"
                  :aria-label="`Удалить статус ${status.label}`"
                  @click.stop="removeStatus(status.id)"
                >
                  <X class="size-3" />
                </button>
              </span>
            </div>
            <p v-else class="text-sm text-sky-700">Пока нет ни одного статуса.</p>
            <div class="min-h-5">
              <FieldError :errors="fieldErrors.statuses ? [fieldErrors.statuses] : []" />
            </div>
          </div>

          <div class="flex flex-col gap-3 rounded-2xl border border-sky-200 bg-sky-50 p-4 sm:p-5">
            <div class="flex items-center justify-between gap-3">
              <FieldLabel class="text-sky-900">Теги</FieldLabel>
              <Button type="button" variant="outline" @click="openTagDialog">Добавить</Button>
            </div>
            <div v-if="tags.length" class="flex flex-wrap gap-2">
              <span
                v-for="tag in tags"
                :key="tag.id"
                class="inline-flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1 text-sm font-medium transition-opacity hover:opacity-85"
                :style="getBadgeStyles(tag.color)"
                role="button"
                tabindex="0"
                :aria-label="`Редактировать тэг ${tag.label}`"
                @click="openTagEditDialog(tag)"
                @keydown.enter.prevent="openTagEditDialog(tag)"
                @keydown.space.prevent="openTagEditDialog(tag)"
              >
                {{ tag.label }}
                <button
                  type="button"
                  class="flex size-4 items-center justify-center rounded-full transition-colors hover:bg-black/10"
                  :aria-label="`Удалить тэг ${tag.label}`"
                  @click.stop="removeTag(tag.id)"
                >
                  <X class="size-3" />
                </button>
              </span>
            </div>
            <p v-else class="text-sm text-sky-700">Пока нет ни одного тэга.</p>
            <div class="min-h-5">
              <FieldError :errors="fieldErrors.tags ? [fieldErrors.tags] : []" />
            </div>
          </div>

          <!-- <div class="flex flex-col gap-3">
            <FieldLabel>Шаблон</FieldLabel>
            <div class="flex items-center gap-3">
              <Select v-model="selectedTemplateId">
                <SelectTrigger class="flex-1">
                  <SelectValue placeholder="Выберите шаблон" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem v-for="tpl in availableTemplates" :key="tpl.id" :value="tpl.id">
                    {{ tpl.name }}
                  </SelectItem>
                </SelectContent>
              </Select>
              <Button
                type="button"
                variant="ghost"
                class="shrink-0"
                @click="templateDialogOpen = true"
              >
                Создать шаблон
              </Button>
            </div>
          </div> -->

          <div class="flex justify-end pt-1">
            <Button
              type="submit"
              :disabled="isLoading"
              class="min-w-36 bg-black text-white hover:bg-black/90 focus-visible:ring-black/30"
            >
              Создать
            </Button>
          </div>
        </FieldGroup>
      </form>
      </div>
    </section>
  </div>

  <Dialog v-model:open="statusDialogOpen">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>{{ editingStatusId ? 'Редактировать статус' : 'Добавить статус' }}</DialogTitle>
      </DialogHeader>
      <div class="grid gap-4 py-2">
        <div class="grid gap-2">
          <FieldLabel>Цвет</FieldLabel>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="preset in COLOR_PRESETS"
              :key="preset.value"
              type="button"
              class="size-8 rounded-full border-2 border-white shadow-sm ring-offset-2 transition-transform hover:scale-105 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/50"
              :class="statusDraft.color === preset.value ? 'ring-2 ring-foreground' : 'ring-1 ring-border'"
              :style="{ backgroundColor: preset.value }"
              :aria-label="`Выбрать цвет ${preset.name}`"
              @click="statusDraft.color = preset.value"
            />
          </div>
        </div>

        <Field class="gap-2">
          <FieldLabel for="status-name">Название статуса</FieldLabel>
          <Input
            id="status-name"
            v-model="statusDraft.label"
            placeholder="Например, В очереди"
            @keydown.enter.prevent="saveStatus"
          />
        </Field>

        <Field class="gap-2">
          <FieldLabel>Категория</FieldLabel>
          <Select v-model="statusDraft.category">
            <SelectTrigger class="w-full">
              <SelectValue placeholder="Выберите категорию" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem
                v-for="category in STATUS_CATEGORIES"
                :key="category"
                :value="category"
              >
                {{ category }}
              </SelectItem>
            </SelectContent>
          </Select>
        </Field>
      </div>
      <DialogFooter>
        <Button
          type="button"
          :disabled="!statusDraft.label.trim() || !statusDraft.category"
          @click="saveStatus"
        >
          {{ editingStatusId ? 'Сохранить' : 'Добавить' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <Dialog v-model:open="tagDialogOpen">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>{{ editingTagId ? 'Редактировать тэг' : 'Добавить тэг' }}</DialogTitle>
      </DialogHeader>
      <div class="grid gap-4 py-2">
        <div class="grid gap-2">
          <FieldLabel>Цвет</FieldLabel>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="preset in COLOR_PRESETS"
              :key="preset.value"
              type="button"
              class="size-8 rounded-full border-2 border-white shadow-sm ring-offset-2 transition-transform hover:scale-105 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/50"
              :class="tagDraft.color === preset.value ? 'ring-2 ring-foreground' : 'ring-1 ring-border'"
              :style="{ backgroundColor: preset.value }"
              :aria-label="`Выбрать цвет ${preset.name}`"
              @click="tagDraft.color = preset.value"
            />
          </div>
        </div>

        <Field class="gap-2">
          <FieldLabel for="tag-name">Название тэга</FieldLabel>
          <Input
            id="tag-name"
            v-model="tagDraft.label"
            placeholder="Например, backend"
            @keydown.enter.prevent="saveTag"
          />
        </Field>
      </div>
      <DialogFooter>
        <Button type="button" :disabled="!tagDraft.label.trim()" @click="saveTag">
          {{ editingTagId ? 'Сохранить' : 'Добавить' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <Dialog v-model:open="templateDialogOpen">
    <DialogContent class="sm:max-w-sm">
      <DialogHeader>
        <DialogTitle>Новый шаблон</DialogTitle>
      </DialogHeader>
      <div class="py-2">
        <Input
          v-model="newTemplateName"
          placeholder="Название шаблона"
          @keydown.enter.prevent="handleCreateTemplate"
        />
      </div>
      <DialogFooter>
        <Button type="button" :disabled="!newTemplateName.trim()" @click="handleCreateTemplate">
          Создать
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
