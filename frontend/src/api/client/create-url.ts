type QueryValue = string | number | boolean | null | undefined
type QueryInput = Record<string, QueryValue | QueryValue[]>
type PathParamsInput = Record<string, string | number | boolean>

// Преобразует значение запроса в строку
function serializeQueryValue(value: string | number | boolean): string {
  return String(value)
}

// Собирает URL с path-параметрами и query-строкой
export function createUrl(
  pathname: string,
  options?: { params?: PathParamsInput; query?: QueryInput },
) {
  let path = pathname

  if (options?.params) {
    for (const [key, value] of Object.entries(options.params)) {
      path = path.replace(`{${key}}`, encodeURIComponent(String(value)))
    }
  }

  if (!options?.query) {
    return path
  }

  const search = new URLSearchParams()

  for (const [key, rawValue] of Object.entries(options.query)) {
    if (rawValue === null || rawValue === undefined) {
      continue
    }

    if (Array.isArray(rawValue)) {
      for (const item of rawValue) {
        if (item === null || item === undefined) {
          continue
        }
        search.append(key, serializeQueryValue(item))
      }
      continue
    }

    search.set(key, serializeQueryValue(rawValue))
  }

  const query = search.toString()
  return query ? `${path}?${query}` : path
}
