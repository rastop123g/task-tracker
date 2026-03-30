<script setup lang="ts">
import { onBeforeUnmount, ref } from 'vue'
import { useRouter } from 'vue-router'
import { CircleAlert, ImagePlus, X } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Field, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Alert, AlertDescription } from '@/components/ui/alert'
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
import { areasStore } from '@/app/areas/areas-store'
import type { ColorSchema, StatusCategorySchema } from '@/api/generated/schema'
import { COLOR_MAP, COLOR_LIST, DEFAULT_COLOR } from '@/lib/colors'
import { STATUS_CATEGORY_LIST } from '@/lib/status-categories'
import { areaCreateSchema, getFieldErrors, type AreaCreateFormValues } from '@/lib/validation/areas'
import type { FieldErrors } from '@/lib/validation/auth'
import { getApiErrorMessage } from '@/api'

type StatusDraft = {
  name: string
  color: ColorSchema
  category: StatusCategorySchema | ''
}

type TagDraft = {
  name: string
  color: ColorSchema
}

const router = useRouter()

const name = ref('')
const avatarObjectUrl = ref<string | null>(null)
const statuses = ref<{ name: string; color: ColorSchema; category: StatusCategorySchema }[]>([])
const tags = ref<{ name: string; color: ColorSchema }[]>([])
const statusDialogOpen = ref(false)
const tagDialogOpen = ref(false)
const editingStatusId = ref<number | null>(null)
const editingTagId = ref<number | null>(null)
const statusDraft = ref<StatusDraft>(createStatusDraft())
const tagDraft = ref<TagDraft>(createTagDraft())
const fieldErrors = ref<FieldErrors<AreaCreateFormValues>>({})
const formError = ref<string | null>(null)
const isLoading = ref(false)

const fileInputRef = ref<HTMLInputElement | null>(null)

function createStatusDraft(): StatusDraft {
  return { name: '', color: DEFAULT_COLOR, category: '' }
}

function createTagDraft(): TagDraft {
  return { name: '', color: DEFAULT_COLOR }
}

function hexToRgba(hex: string, alpha: number): string {
  const normalized = hex.replace('#', '')
  const value = Number.parseInt(normalized, 16)
  const r = (value >> 16) & 255
  const g = (value >> 8) & 255
  const b = value & 255

  return `rgba(${r}, ${g}, ${b}, ${alpha})`
}

function getBadgeStyles(color: ColorSchema): Record<string, string> {
  const hex = COLOR_MAP[color].hex
  return {
    backgroundColor: hexToRgba(hex, 0.14),
    borderColor: hexToRgba(hex, 0.28),
    color: hex,
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

function removeStatus(index: number): void {
  statuses.value.splice(index, 1)
}

function removeTag(index: number): void {
  tags.value.splice(index, 1)
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

function openStatusEditDialog(index: number): void {
  const status = statuses.value[index]
  if (!status) return
  editingStatusId.value = index
  statusDraft.value = { name: status.name, color: status.color, category: status.category }
  statusDialogOpen.value = true
}

function openTagEditDialog(index: number): void {
  const tag = tags.value[index]
  if (!tag) return
  editingTagId.value = index
  tagDraft.value = { name: tag.name, color: tag.color }
  tagDialogOpen.value = true
}

function saveStatus(): void {
  const statusName = statusDraft.value.name.trim()
  const category = statusDraft.value.category

  if (!statusName || !category) return

  if (editingStatusId.value !== null) {
    statuses.value[editingStatusId.value] = {
      name: statusName,
      color: statusDraft.value.color,
      category,
    }
  } else {
    statuses.value.push({
      name: statusName,
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
  const tagName = tagDraft.value.name.trim()
  if (!tagName) return

  if (editingTagId.value !== null) {
    tags.value[editingTagId.value] = {
      name: tagName,
      color: tagDraft.value.color,
    }
  } else {
    tags.value.push({
      name: tagName,
      color: tagDraft.value.color,
    })
  }
  editingTagId.value = null
  tagDraft.value = createTagDraft()
  tagDialogOpen.value = false
}

async function handleSubmit(): Promise<void> {
  fieldErrors.value = {}
  formError.value = null

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
    await areasStore.createWorkspace({
      name: result.data.name,
      statuses: result.data.statuses.map((s) => ({
        name: s.name,
        color: s.color,
        category: s.category,
      })),
      tags: result.data.tags.map((t) => ({
        name: t.name,
        color: t.color,
      })),
    })
    await router.push({ name: 'home' })
  } catch (err: unknown) {
    formError.value = getApiErrorMessage(err, {
      validation: 'Проверьте корректность введённых данных',
    })
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
      <Alert v-if="formError" variant="destructive">
        <CircleAlert />
        <AlertDescription>{{ formError }}</AlertDescription>
      </Alert>

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
                v-for="(status, index) in statuses"
                :key="index"
                class="inline-flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1 text-sm font-medium transition-opacity hover:opacity-85"
                :style="getBadgeStyles(status.color)"
                role="button"
                tabindex="0"
                :aria-label="`Редактировать статус ${status.name}`"
                @click="openStatusEditDialog(index)"
                @keydown.enter.prevent="openStatusEditDialog(index)"
                @keydown.space.prevent="openStatusEditDialog(index)"
              >
                {{ status.name }}
                <button
                  type="button"
                  class="flex size-4 items-center justify-center rounded-full transition-colors hover:bg-black/10"
                  :aria-label="`Удалить статус ${status.name}`"
                  @click.stop="removeStatus(index)"
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
                v-for="(tag, index) in tags"
                :key="index"
                class="inline-flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1 text-sm font-medium transition-opacity hover:opacity-85"
                :style="getBadgeStyles(tag.color)"
                role="button"
                tabindex="0"
                :aria-label="`Редактировать тэг ${tag.name}`"
                @click="openTagEditDialog(index)"
                @keydown.enter.prevent="openTagEditDialog(index)"
                @keydown.space.prevent="openTagEditDialog(index)"
              >
                {{ tag.name }}
                <button
                  type="button"
                  class="flex size-4 items-center justify-center rounded-full transition-colors hover:bg-black/10"
                  :aria-label="`Удалить тэг ${tag.name}`"
                  @click.stop="removeTag(index)"
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
        <DialogTitle>{{ editingStatusId !== null ? 'Редактировать статус' : 'Добавить статус' }}</DialogTitle>
      </DialogHeader>
      <div class="grid gap-4 py-2">
        <div class="grid gap-2">
          <FieldLabel>Цвет</FieldLabel>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="[colorKey, colorInfo] in COLOR_LIST"
              :key="colorKey"
              type="button"
              class="size-8 rounded-full border-2 border-white shadow-sm ring-offset-2 transition-transform hover:scale-105 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/50"
              :class="statusDraft.color === colorKey ? 'ring-2 ring-foreground' : 'ring-1 ring-border'"
              :style="{ backgroundColor: colorInfo.hex }"
              :aria-label="`Выбрать цвет ${colorInfo.label}`"
              @click="statusDraft.color = colorKey"
            />
          </div>
        </div>

        <Field class="gap-2">
          <FieldLabel for="status-name">Название статуса</FieldLabel>
          <Input
            id="status-name"
            v-model="statusDraft.name"
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
                v-for="[categoryKey, categoryLabel] in STATUS_CATEGORY_LIST"
                :key="categoryKey"
                :value="categoryKey"
              >
                {{ categoryLabel }}
              </SelectItem>
            </SelectContent>
          </Select>
        </Field>
      </div>
      <DialogFooter>
        <Button
          type="button"
          :disabled="!statusDraft.name.trim() || !statusDraft.category"
          @click="saveStatus"
        >
          {{ editingStatusId !== null ? 'Сохранить' : 'Добавить' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <Dialog v-model:open="tagDialogOpen">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>{{ editingTagId !== null ? 'Редактировать тэг' : 'Добавить тэг' }}</DialogTitle>
      </DialogHeader>
      <div class="grid gap-4 py-2">
        <div class="grid gap-2">
          <FieldLabel>Цвет</FieldLabel>
          <div class="flex flex-wrap gap-2">
            <button
              v-for="[colorKey, colorInfo] in COLOR_LIST"
              :key="colorKey"
              type="button"
              class="size-8 rounded-full border-2 border-white shadow-sm ring-offset-2 transition-transform hover:scale-105 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/50"
              :class="tagDraft.color === colorKey ? 'ring-2 ring-foreground' : 'ring-1 ring-border'"
              :style="{ backgroundColor: colorInfo.hex }"
              :aria-label="`Выбрать цвет ${colorInfo.label}`"
              @click="tagDraft.color = colorKey"
            />
          </div>
        </div>

        <Field class="gap-2">
          <FieldLabel for="tag-name">Название тэга</FieldLabel>
          <Input
            id="tag-name"
            v-model="tagDraft.name"
            placeholder="Например, backend"
            @keydown.enter.prevent="saveTag"
          />
        </Field>
      </div>
      <DialogFooter>
        <Button type="button" :disabled="!tagDraft.name.trim()" @click="saveTag">
          {{ editingTagId !== null ? 'Сохранить' : 'Добавить' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
