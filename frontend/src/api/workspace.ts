import { del, get, patch, post, request } from './client'

// Some workspace operations are missing path-param declarations in the generated
// OpenAPI schema (utoipa issue). For those we build the URL manually and cast.
function wp(workspaceId: string, suffix = '') {
  return `/api/v1/workspace/${encodeURIComponent(workspaceId)}${suffix}`
}

export const workspace = {
  async create(payload: Parameters<typeof post<'/api/v1/workspace/create'>>[1] extends infer O ? O extends { json: infer J } ? J : never : never) {
    return post('/api/v1/workspace/create', { json: payload })
  },

  async get(workspaceId: string) {
    return request({ method: 'GET', path: wp(workspaceId) } as any)
  },

  async update(workspaceId: string, payload: any) {
    return request({ method: 'PATCH', path: wp(workspaceId), json: payload } as any)
  },

  async changeAdmin(workspaceId: string, payload: any) {
    return request({ method: 'POST', path: wp(workspaceId, '/change-admin'), json: payload } as any)
  },

  // --- Avatar ---
  async uploadAvatar(workspaceId: string, file: File) {
    const formData = new FormData()
    formData.append('avatar', file)
    return post('/api/v1/workspace/{workspace_id}/avatar', {
      params: { workspace_id: workspaceId },
      formData,
    })
  },

  async deleteAvatar(workspaceId: string) {
    return del('/api/v1/workspace/{workspace_id}/avatar', {
      params: { workspace_id: workspaceId },
    })
  },

  // --- Statuses ---
  async getStatuses(workspaceId: string) {
    return get('/api/v1/workspace/{workspace_id}/status/list', {
      params: { workspace_id: workspaceId },
    })
  },

  async createStatus(workspaceId: string, payload: any) {
    return post('/api/v1/workspace/{workspace_id}/status', {
      params: { workspace_id: workspaceId },
      json: payload,
    })
  },

  async updateStatus(workspaceId: string, statusId: string, payload: any) {
    return patch('/api/v1/workspace/{workspace_id}/status/{status_id}', {
      params: { workspace_id: workspaceId, status_id: statusId },
      json: payload,
    })
  },

  async deleteStatus(workspaceId: string, statusId: string) {
    return del('/api/v1/workspace/{workspace_id}/status/{status_id}', {
      params: { workspace_id: workspaceId, status_id: statusId },
    })
  },

  // --- Tags ---
  async getTags(workspaceId: string) {
    return get('/api/v1/workspace/{workspace_id}/tag/list', {
      params: { workspace_id: workspaceId },
    })
  },

  async createTag(workspaceId: string, payload: any) {
    return post('/api/v1/workspace/{workspace_id}/tag', {
      params: { workspace_id: workspaceId },
      json: payload,
    })
  },

  async updateTag(workspaceId: string, tagId: string, payload: any) {
    return patch('/api/v1/workspace/{workspace_id}/tag/{tag_id}', {
      params: { workspace_id: workspaceId, tag_id: tagId },
      json: payload,
    })
  },

  async deleteTag(workspaceId: string, tagId: string) {
    return del('/api/v1/workspace/{workspace_id}/tag/{tag_id}', {
      params: { workspace_id: workspaceId, tag_id: tagId },
    })
  },

  // --- Invites ---
  async getInvites(workspaceId: string) {
    return get('/api/v1/workspace/{workspace_id}/invite/list', {
      params: { workspace_id: workspaceId },
    })
  },

  async searchUsersForInvite(
    workspaceId: string,
    search: string,
    limit?: number,
    offset?: number,
  ) {
    return get('/api/v1/workspace/{workspace_id}/invite/list-for-invite', {
      params: { workspace_id: workspaceId },
      query: { search, limit, offset },
    })
  },

  async createInvite(workspaceId: string, userId: string) {
    return post('/api/v1/workspace/{workspace_id}/invite', {
      params: { workspace_id: workspaceId },
      json: { user_id: userId },
    })
  },

  async deleteInvite(workspaceId: string, userId: string) {
    return del('/api/v1/workspace/{workspace_id}/invite', {
      params: { workspace_id: workspaceId },
      json: { user_id: userId },
    })
  },

  async acceptInvite(workspaceId: string) {
    return post('/api/v1/workspace/{workspace_id}/invite/accept', {
      params: { workspace_id: workspaceId },
    })
  },
}
