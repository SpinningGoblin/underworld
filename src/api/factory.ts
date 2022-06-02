import {
  GameActionsApi,
  GamesApi,
  PlayerCharactersApi,
} from "../generated-api";
import { getConfiguration } from "./configuration";

let gameApi: GamesApi;
export const getGamesApi = (): GamesApi => {
  if (!gameApi) {
    gameApi = new GamesApi(getConfiguration());
  }

  return gameApi;
};

let actionApi: GameActionsApi;
export const getGameActionsApi = () => {
  if (!actionApi) {
    actionApi = new GameActionsApi(getConfiguration());
  }

  return actionApi;
};

let playerApi: PlayerCharactersApi;
export const getPlayerApi = () => {
  if (!playerApi) {
    playerApi = new PlayerCharactersApi(getConfiguration());
  }

  return playerApi;
};
