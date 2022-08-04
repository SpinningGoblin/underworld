import Cookies from "js-cookie";

export const getApiToken = (): string | undefined => {
  const hashToken = window.location.hash.replaceAll("#", "");
  if (hashToken) {
    Cookies.set("underworldApiToken", hashToken, {
      expires: 2,
    });
    window.location.assign("/");
    return hashToken;
  }

  return Cookies.get("underworldApiToken");
};
