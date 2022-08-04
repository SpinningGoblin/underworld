const backendUrl = import.meta.env.VITE_UNDERWORLD_URL;

export const getBasePath = (): string => `${backendUrl}/api`;
export const getSignInUrl = () => `${backendUrl}/sign-in`;
export const getApiLoginUrl = () => `${backendUrl}/auth/login`;
export const getMailCallbackUrl = (mailToken: string) =>
  `${backendUrl}/auth/enter_the_underworld?token=${mailToken}`;
