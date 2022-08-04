const backendUrl = import.meta.env.VITE_UNDERWORLD_URL;
const frontendBasePath = import.meta.env.VITE_FRONTEND_BASE_URL;

export const getBasePath = (): string => `${backendUrl}/api`;
export const getSignInUrl = () => `${frontendBasePath}#/sign-in`;
export const getApiLoginUrl = () => `${backendUrl}/auth/login`;
export const getMailCallbackUrl = (mailToken: string) =>
  `${backendUrl}/auth/enter_the_underworld?token=${mailToken}`;
