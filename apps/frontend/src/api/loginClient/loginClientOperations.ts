import { parse } from "uri-template";
import type { LoginClientContext } from "./loginClientContext.js";
import { createRestError } from "../../helpers/error.js";
import type { OperationOptions } from "../../helpers/interfaces.js";
import { jsonOAuthLoginToApplicationTransform, jsonOAuthResultToTransportTransform, jsonPlainResultToApplicationTransform } from "../../models/internal/serializers.js";
import { OAuthLogin, type OAuthResult, PlainResult } from "../../models/models.js";

export interface LoginOptions extends OperationOptions {

}
export async function login(
  client: LoginClientContext,
  options?: LoginOptions,
): Promise<OAuthLogin> {
  const path = parse("/logins").expand({

  });
  const httpRequestOptions = {
    headers: {

    },
  };
  const response = await client.pathUnchecked(path).get(httpRequestOptions);
  console.log("response", response);
  if (typeof options?.operationOptions?.onResponse === "function") {
    options?.operationOptions?.onResponse(response);
  }
  if (+response.status === 200 && response.headers["content-type"]?.includes("application/json")) {
    return jsonOAuthLoginToApplicationTransform(response.body)!;
  }
  throw createRestError(response);
}
;
export interface OauthOptions extends OperationOptions {

}
export async function oauth(
  client: LoginClientContext,
  body: OAuthResult,
  options?: OauthOptions,
): Promise<PlainResult> {
  const path = parse("/logins").expand({

  });
  const httpRequestOptions = {
    headers: {

    },body: jsonOAuthResultToTransportTransform(body),
  };
  const response = await client.pathUnchecked(path).post(httpRequestOptions);

  ;
  if (typeof options?.operationOptions?.onResponse === "function") {
    options?.operationOptions?.onResponse(response);
  }
  if (+response.status === 200 && response.headers["content-type"]?.includes("application/json")) {
    return jsonPlainResultToApplicationTransform(response.body)!;
  }
  throw createRestError(response);
}
;
export interface LogoutOptions extends OperationOptions {

}
export async function logout(
  client: LoginClientContext,
  options?: LogoutOptions,
): Promise<PlainResult> {
  const path = parse("/logins").expand({

  });
  const httpRequestOptions = {
    headers: {

    },
  };
  const response = await client.pathUnchecked(path).delete(httpRequestOptions);

  ;
  if (typeof options?.operationOptions?.onResponse === "function") {
    options?.operationOptions?.onResponse(response);
  }
  if (+response.status === 200 && response.headers["content-type"]?.includes("application/json")) {
    return jsonPlainResultToApplicationTransform(response.body)!;
  }
  throw createRestError(response);
}
;