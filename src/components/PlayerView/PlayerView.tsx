import { FunctionComponent } from "react";
import { PerformAction, PlayerCharacter } from "../../generated-api";
import { EffectsView } from "../EffectsView";
import { PlayerInventoryView } from "./PlayerInventoryView";
import { PlayerSpellMemoryView } from "./PlayerSpellMemoryView";
import chevron from "../../images/chevron.svg";
import GoldCoin from "../../images/gold_coin.svg";
import Heart from "../../images/heart.svg";

import styles from "./PlayerView.module.css";
import { useTheme } from "../../themes";

export interface PlayerViewProps {
  player: PlayerCharacter;
  actions: Array<PerformAction>;
  toggleShowFullPlayer: () => void;
  showFullPlayer: boolean;
}

export const PlayerView: FunctionComponent<PlayerViewProps> = ({
  player,
  actions,
  toggleShowFullPlayer,
  showFullPlayer,
}) => {
  const { theme } = useTheme();
  const description = `You are a ${player.character.stats.height} ${player.character.species}`;

  const collapsedClass = showFullPlayer ? "" : styles.showing;
  const health = player.character.stats.health!;

  return (
    <div
      className={styles.player}
      style={{ borderBottomColor: theme.colors.secondary }}
    >
      <div className={styles.details}>
        <div className={styles.basics}>
          <div className={styles.description}>{description}</div>
          <div className={styles["health-and-gold"]}>
            <div className={styles.health} style={{ color: theme.colors.red }}>
              <img src={Heart} alt="heart" height={16} width={16} />
              {`${health.current} / ${health.max}`}
            </div>
            <div className={styles.gold} style={{ color: theme.colors.yellow }}>
              <img src={GoldCoin} alt="gold coin" height={16} width={16} />
              {player.gold}
            </div>
            <button
              className={styles.collapse}
              onClick={toggleShowFullPlayer}
              style={{ background: theme.colors.secondary }}
            >
              <img
                className={[collapsedClass, styles["collapse-icon"]].join(" ")}
                src={chevron}
                alt="chevron"
              />
            </button>
          </div>
        </div>
        {showFullPlayer && (
          <>
            <EffectsView effects={player.character.current_effects!} />
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
      </div>
    </div>
  );
};
