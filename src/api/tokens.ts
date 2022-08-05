import Cookies from "js-cookie";
import { getFrontendBaseUrl } from "./path";

export const getApiToken = (): string | undefined => {
  const params = new URL(window.location.href).searchParams;
  const paramsToken = params.get("token");
  if (paramsToken) {
    Cookies.set("underworldApiToken", paramsToken, {
      expires: 2,
    });
    window.location.assign(getFrontendBaseUrl());
    return paramsToken;
  }

  return Cookies.get("underworldApiToken");
};

export const removeApiToken = () => {
  Cookies.remove("underworldApiToken");
};
