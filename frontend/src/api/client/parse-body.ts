// Разбирает тело ответа по content-type
export async function parseBody(response: Response): Promise<unknown> {
  if (response.status === 204 || response.status === 205) {
    return undefined
  }

  const contentType = response.headers.get('content-type') ?? ''
  if (!contentType) {
    return undefined
  }

  if (contentType.includes('application/json')) {
    try {
      return await response.json()
    } catch {
      return undefined
    }
  }

  if (contentType.startsWith('text/')) {
    return await response.text()
  }

  return undefined
}
