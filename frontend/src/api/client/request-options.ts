import type { paths } from '@/api/generated/schema'

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE'

type MethodKey<M extends HttpMethod> = Lowercase<M>
type EmptyObject = Record<string, never>

type PropertyValue<T, K extends PropertyKey> = T extends {
  [P in K]?: infer Value
}
  ? NonNullable<Value>
  : never

export type PathForMethod<M extends HttpMethod> = {
  [P in keyof paths & string]: paths[P][MethodKey<M>] extends never | undefined ? never : P
}[keyof paths & string]

export type OperationFor<M extends HttpMethod, P extends PathForMethod<M>> = NonNullable<
  paths[P][MethodKey<M>]
>

type ParametersFor<Operation> =
  PropertyValue<Operation, 'parameters'> extends never
    ? EmptyObject
    : PropertyValue<Operation, 'parameters'>

type QueryFor<Operation> = PropertyValue<ParametersFor<Operation>, 'query'>
type PathParamsFor<Operation> = PropertyValue<ParametersFor<Operation>, 'path'>
type BodyContentFor<Operation> = PropertyValue<PropertyValue<Operation, 'requestBody'>, 'content'>
type JsonBodyFor<Operation> = PropertyValue<BodyContentFor<Operation>, 'application/json'>
type MultipartBodyFor<Operation> = PropertyValue<BodyContentFor<Operation>, 'multipart/form-data'>

type WithQuery<Operation> = [QueryFor<Operation>] extends [never]
  ? {}
  : { query: QueryFor<Operation> }

type WithPathParams<Operation> = [PathParamsFor<Operation>] extends [never]
  ? {}
  : { params: PathParamsFor<Operation> }

type WithJsonBody<Operation> = [JsonBodyFor<Operation>] extends [never]
  ? {}
  : { json: JsonBodyFor<Operation> }

type WithFormData<Operation> = [MultipartBodyFor<Operation>] extends [never]
  ? {}
  : { formData: FormData }

type SuccessBody<Response> = Response extends { content: infer Content }
  ? Content extends { 'application/json': infer JsonBody }
    ? JsonBody
    : Content extends { 'text/plain': infer TextBody }
      ? TextBody
      : void
  : void

type SuccessResponseFor<Responses> = 200 extends keyof Responses
  ? SuccessBody<Responses[200]>
  : 201 extends keyof Responses
    ? SuccessBody<Responses[201]>
    : 202 extends keyof Responses
      ? SuccessBody<Responses[202]>
      : 204 extends keyof Responses
        ? SuccessBody<Responses[204]>
        : void

export type ResponseFor<Operation> = Operation extends { responses: infer Responses }
  ? SuccessResponseFor<Responses>
  : void

type RequestBase<M extends HttpMethod, P extends PathForMethod<M>> = {
  method: M
  path: P
  auth?: boolean
  signal?: AbortSignal
}

export type RequestOptions<M extends HttpMethod, P extends PathForMethod<M>> = {} & RequestBase<
  M,
  P
> &
  WithPathParams<OperationFor<M, P>> &
  WithQuery<OperationFor<M, P>> &
  WithJsonBody<OperationFor<M, P>> &
  WithFormData<OperationFor<M, P>>
