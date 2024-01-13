import { getSignInUrl } from "./api/path";
import { removeApiToken } from "./api/tokens";
import { ResponseError } from "./generated-api";

export const wrapForAuth = async <T>(call: () => Promise<T>): Promise<T> => {
  try {
    const result = await call();
    return result;
  } catch (err) {
    if (err instanceof ResponseError && err.response.status === 401) {
      removeApiToken();
      window.location.assign(getSignInUrl());
      throw new Error("Redirecting");
    } else {
      throw err;
    }
  }
};
