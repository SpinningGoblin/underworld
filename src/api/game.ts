import { GamesApi } from "../generated-api";
import { getConfiguration } from "./configuration";
import { getUsername } from "./username";

let gameApi: GamesApi;

const getGamesApi = (): GamesApi => {
  if (!gameApi) {
    gameApi = new GamesApi(getConfiguration());
  }

  return gameApi;
};

export const getGameIds = async (): Promise<Array<string>> => {
  const api = getGamesApi();
  return api.getGameIds({ underworldUsername: getUsername()! });
};
