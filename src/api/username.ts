const USERNAME_KEY = "username";

export const setUsername = (username: string) => {
  window.localStorage.setItem(USERNAME_KEY, username);
};

export const getUsername = (): string | undefined =>
  window.localStorage.getItem(USERNAME_KEY) || undefined;
