import { FunctionComponent } from "react";
import { PerformAction, PlayerCharacter } from "../../generated-api";
import { EffectsView } from "../EffectsView";
import { PlayerInventoryView } from "./PlayerInventoryView";
import { PlayerSpellMemoryView } from "./PlayerSpellMemoryView";

import styles from "./styles.module.css";

export interface PlayerViewProps {
  player: PlayerCharacter;
  actions: Array<PerformAction>;
}

export const PlayerView: FunctionComponent<PlayerViewProps> = ({
  player,
  actions,
}) => {
  const description = `You are a ${player.character.stats.height} ${player.character.species}`;

  const healthClasses = [
    styles.health,
    player.character.stats.health!.current < 5 ? styles["low-health"] : "",
  ].join(" ");

  return (
    <div className={styles.player}>
      <div className={styles.description}>{description}</div>
      <div className={healthClasses}>{`Health - Current: ${
        player.character.stats.health!.current
      } Max: ${player.character.stats.health!.max}`}</div>
      <EffectsView effects={player.character.current_effects!} />
      {actions.length > 0 && (
        <PlayerSpellMemoryView spellMemory={player.character.spell_memory!} />
      )}
      {actions.length > 0 && player.character.inventory && (
        <PlayerInventoryView
          inventory={player.character.inventory}
          actions={actions}
        />
      )}
    </div>
  );
};
