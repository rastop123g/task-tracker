<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { computed } from 'vue'
import { cn } from '@/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
  errors?: Array<string | { message: string | undefined } | undefined>
}>()

const content = computed(() => {
  if (!props.errors || props.errors.length === 0) return null

  const uniqueErrors = [
    ...new Map(
      props.errors.filter(Boolean).map((error) => {
        const message = typeof error === 'string' ? error : error?.message
        return [message, error]
      }),
    ).values(),
  ]

  if (uniqueErrors.length === 1 && uniqueErrors[0]) {
    return typeof uniqueErrors[0] === 'string' ? uniqueErrors[0] : uniqueErrors[0].message
  }

  return uniqueErrors.map((error) => (typeof error === 'string' ? error : error?.message))
})

const contentKey = computed(() => {
  if (!content.value) return 'empty'

  return Array.isArray(content.value) ? content.value.join('|') : content.value
})
</script>

<template>
  <Transition name="field-error" mode="out-in" appear>
    <div
      v-if="$slots.default || content"
      :key="contentKey"
      role="alert"
      data-slot="field-error"
      :class="cn('text-destructive text-sm font-normal', props.class)"
    >
      <slot v-if="$slots.default" />

      <template v-else-if="typeof content === 'string'">
        {{ content }}
      </template>

      <ul v-else-if="Array.isArray(content)" class="ml-4 flex list-disc flex-col gap-1">
        <li v-for="(error, index) in content" :key="index">
          {{ error }}
        </li>
      </ul>
    </div>
  </Transition>
</template>

<style scoped>
.field-error-enter-active,
.field-error-leave-active {
  overflow: hidden;
  transition:
    opacity 180ms ease,
    transform 180ms ease,
    max-height 220ms ease;
}

.field-error-enter-from,
.field-error-leave-to {
  opacity: 0;
  transform: translateY(-6px);
  max-height: 0;
}

.field-error-enter-to,
.field-error-leave-from {
  opacity: 1;
  transform: translateY(0);
  max-height: 120px;
}
</style>
