import { FunctionComponent } from "react";
import { CastSpellOnPlayer, Spell, SpellMemory } from "../../../generated-api";
import { useTheme } from "../../../themes/context";
import { CastSpellOnPlayerView } from "../../actions";
import { Card } from "../../Card";

import styles from "./PlayerSpellMemoryView.module.css";

export interface PlayerSpellMemoryViewProps {
  spellMemory: SpellMemory;
}

interface PlayerSpellViewProps {
  spell: Spell;
  castArgs?: CastSpellOnPlayer;
}

const PlayerSpellView: FunctionComponent<PlayerSpellViewProps> = ({
  spell,
  castArgs,
}) => (
  <Card className={styles.spell}>
    <div className={styles["spell-name"]}>{spell.name}</div>
    <div className={styles.uses}>
      <span>{spell.uses} uses remain</span>
      {castArgs && <CastSpellOnPlayerView args={castArgs} />}
    </div>
  </Card>
);

export const PlayerSpellMemoryView: FunctionComponent<
  PlayerSpellMemoryViewProps
> = ({ spellMemory }) => {
  const { theme } = useTheme();
  return (
    <div
      className={styles["spell-memory"]}
      style={{ color: theme.colors.secondary }}
    >
      <h3>Spell Memory</h3>
      {(spellMemory.spells || []).length === 0 && (
        <span>No spells in memory</span>
      )}
      <div className={styles["spell-list"]}>
        {(spellMemory.spells || []).map((learnedSpell) => {
          const args: CastSpellOnPlayer | undefined =
            learnedSpell.spell.spell_type !== "attack"
              ? { spell_id: learnedSpell.id }
              : undefined;

          return (
            <PlayerSpellView
              key={learnedSpell.id}
              spell={learnedSpell.spell}
              castArgs={args}
            />
          );
        })}
      </div>
    </div>
  );
};
