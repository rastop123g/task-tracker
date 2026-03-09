import { post } from './client'

export const avatar = {
  async uploadMyAvatar(file: File): Promise<void> {
    const formData = new FormData()
    formData.append('avatar', file)

    await post('/api/v1/avatar', {
      formData,
    })
  },
}
