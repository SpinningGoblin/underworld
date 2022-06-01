import { PlayerCharacter, PlayerCharactersApi } from "../generated-api";
import { getUsername } from "./username";

export const getCurrentPlayer = async (): Promise<PlayerCharacter> => {
  const username = getUsername();

  if (!username) {
    throw new Error("No username");
  }

  const api = new PlayerCharactersApi();

  return api.getCurrentPc({ underworldUsername: username });
};
