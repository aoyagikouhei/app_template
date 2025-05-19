import { type Client, type ClientOptions, getClient } from "@typespec/ts-http-runtime";

export interface TemplateServiceClientContext extends Client {

}export interface TemplateServiceClientOptions extends ClientOptions {
  endpoint?: string;
}export function createTemplateServiceClientContext(
  endpoint: string,
  options?: TemplateServiceClientOptions,
): TemplateServiceClientContext {
  const params: Record<string, any> = {
    endpoint: endpoint
  };
  const resolvedEndpoint = "{endpoint}".replace(/{([^}]+)}/g, (_, key) =>
    key in params ? String(params[key]) : (() => { throw new Error(`Missing parameter: ${key}`); })()
  );;return getClient(resolvedEndpoint,{
    ...options,
  })
}