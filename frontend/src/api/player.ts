import { PlayerCharacter } from "../generated-api";
import { getPlayerApi } from "./factory";

export const getCurrentPlayer = async (): Promise<PlayerCharacter> => {
  const api = getPlayerApi();

  return api.getCurrentPc();
};

export const generatePlayer = async (): Promise<PlayerCharacter> => {
  const api = getPlayerApi();
  const generatedPlayer = await api.generatePc({
    generatePlayerCharacter: {},
  });

  if (!generatedPlayer.set_as_current) {
    await api.setPcAsCurrent({
      id: generatedPlayer.player_character_id,
    });
  }

  generatedPlayer.set_as_current = true;

  return api.getCurrentPc();
};
