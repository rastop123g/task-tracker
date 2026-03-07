import { ref, computed, onUnmounted } from 'vue'

const _handler = ref<(() => Promise<void>) | null>(null)
const _saving = ref(false)

export function useSettingsActions() {
  function register(handler: () => Promise<void>) {
    _handler.value = handler
    onUnmounted(() => {
      _handler.value = null
    })
  }

  async function trigger() {
    if (!_handler.value || _saving.value) return
    _saving.value = true
    try {
      await _handler.value()
    } finally {
      _saving.value = false
    }
  }

  return {
    register,
    trigger,
    isSaving: _saving,
    hasAction: computed(() => _handler.value !== null),
  }
}
