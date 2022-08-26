import Cookies from "js-cookie";

export const getApiToken = (): string | undefined => {
  const params = new URL(window.location.href).searchParams;
  const paramsToken = params.get("token");
  if (paramsToken) {
    Cookies.set("underworldApiToken", paramsToken, {
      expires: 2,
    });
    window.history.replaceState(null, "", window.location.pathname);
    return paramsToken;
  }

  return Cookies.get("underworldApiToken");
};

export const removeApiToken = () => {
  Cookies.remove("underworldApiToken");
};
