let currentGameId: string;

export const setCurrentGameId = (gameId: string) => {
  currentGameId = gameId;
};

export const getCurrentGameId = (): string | undefined => currentGameId;
