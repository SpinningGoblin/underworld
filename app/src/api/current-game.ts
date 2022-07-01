let currentGameId: string | undefined;

export const setCurrentGameId = (gameId?: string) => {
  currentGameId = gameId;
  saveGameIdLocalStorage(gameId);
};

export const getCurrentGameId = (): string | undefined => {
  if (currentGameId) {
    return currentGameId;
  }

  currentGameId = getGameIdLocalStorage();
  return currentGameId;
};

const GAME_ID_KEY = "current_game";

export const saveGameIdLocalStorage = (gameId?: string) => {
  if (gameId) {
    window.localStorage.setItem(GAME_ID_KEY, gameId);
  } else if (getGameIdLocalStorage()) {
    window.localStorage.removeItem(GAME_ID_KEY);
  }
};

export const getGameIdLocalStorage = (): string | undefined =>
  window.localStorage.getItem(GAME_ID_KEY) || undefined;
