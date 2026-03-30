import { computed, ref, shallowRef } from 'vue'
import type {
  CreateWorkspaceRequest,
  WorkspaceWithStatusesAndTagsResponse,
} from '@/api/generated/schema'
import { api } from '@/api'

const workspaces = ref<WorkspaceWithStatusesAndTagsResponse[]>([])
const current = shallowRef<WorkspaceWithStatusesAndTagsResponse | null>(null)
const isEmpty = computed(() => workspaces.value.length === 0)

async function createWorkspace(
  payload: CreateWorkspaceRequest,
): Promise<WorkspaceWithStatusesAndTagsResponse> {
  const ws = await api.workspace.create(payload)
  workspaces.value.push(ws)
  current.value = ws
  return ws
}

function setCurrent(ws: WorkspaceWithStatusesAndTagsResponse): void {
  current.value = ws
}

function clear(): void {
  workspaces.value = []
  current.value = null
}

export const areasStore = {
  workspaces,
  current,
  isEmpty,
  createWorkspace,
  setCurrent,
  clear,
}
