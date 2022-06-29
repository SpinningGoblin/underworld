import { FunctionComponent, useState } from "react";
import { PerformAction, PlayerCharacter } from "../../generated-api";
import { EffectsView } from "../EffectsView";
import { PlayerInventoryView } from "./PlayerInventoryView";
import { PlayerSpellMemoryView } from "./PlayerSpellMemoryView";
import chevron from "../../images/chevron.svg";

import styles from "./styles.module.css";

export interface PlayerViewProps {
  player: PlayerCharacter;
  actions: Array<PerformAction>;
}

export const PlayerView: FunctionComponent<PlayerViewProps> = ({
  player,
  actions,
}) => {
  const [collapsed, setCollapsed] = useState<boolean>(true);
  const description = `You are a ${player.character.stats.height} ${player.character.species}`;

  const healthClasses = [
    styles.health,
    player.character.stats.health!.current < 5 ? styles["low-health"] : "",
  ].join(" ");

  const collapsedClass = collapsed ? "" : styles.showing;

  return (
    <div className={styles.player}>
      <div className={styles.details}>
        <div className={styles.description}>{description}</div>
        <div className={healthClasses}>{`Health ${
          player.character.stats.health!.current
        } / ${player.character.stats.health!.max}`}</div>
        <div className={styles.gold}>{`${player.gold} gold`}</div>
        <EffectsView effects={player.character.current_effects!} />
        {!collapsed && (
          <>
            {actions.length > 0 && (
              <PlayerSpellMemoryView
                spellMemory={player.character.spell_memory!}
              />
            )}
            {actions.length > 0 && player.character.inventory && (
              <PlayerInventoryView
                inventory={player.character.inventory}
                actions={actions}
              />
            )}
          </>
        )}
        <button
          className={styles.collapse}
          onClick={() => setCollapsed((current) => !current)}
        >
          <img
            className={[collapsedClass, styles["collapse-icon"]].join(" ")}
            src={chevron}
            alt="chevron"
          />
        </button>
      </div>
    </div>
  );
};
