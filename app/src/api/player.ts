import { PlayerCharacter } from "../generated-api";
import { getPlayerApi } from "./factory";
import { getUsername } from "./username";

export const getCurrentPlayer = async (): Promise<PlayerCharacter> => {
  const username = getUsername();

  if (!username) {
    throw new Error("No username");
  }

  const api = getPlayerApi();

  return api.getCurrentPc({ underworldUsername: username });
};

export const generatePlayer = async (): Promise<PlayerCharacter> => {
  const username = getUsername();

  if (!username) {
    throw new Error("No username");
  }

  const api = getPlayerApi();
  const generatedPlayer = await api.generatePc({
    underworldUsername: username,
    generatePlayerCharacter: {},
  });

  if (!generatedPlayer.set_as_current) {
    await api.setPcAsCurrent({
      underworldUsername: username,
      id: generatedPlayer.player_character_id,
    });
  }

  generatedPlayer.set_as_current = true;

  return api.getCurrentPc({ underworldUsername: username });
};
