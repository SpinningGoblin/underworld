import { GeneratedGame } from "../generated-api";
import { getGamesApi } from "./factory";
import { getUsername } from "./username";

export const getGameIds = async (): Promise<Array<string>> => {
  const api = getGamesApi();
  return api.getGameIds({ underworldUsername: getUsername()! });
};

export const generateGame = async (): Promise<GeneratedGame> => {
  const api = getGamesApi();
  const username = getUsername()!;

  return api.generateGame({ underworldUsername: username });
};
