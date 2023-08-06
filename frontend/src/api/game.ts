import { GeneratedGame } from "../generated-api";
import { getGamesApi } from "./factory";

export const getGameIds = async (): Promise<Array<string>> => {
  const api = getGamesApi();
  return api.getGameIds();
};

export const generateGame = async (): Promise<GeneratedGame> => {
  const api = getGamesApi();

  return api.generateGame();
};
