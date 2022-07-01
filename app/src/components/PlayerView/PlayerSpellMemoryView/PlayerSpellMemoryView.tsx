import { FunctionComponent } from "react";
import { CastSpellOnPlayer, Spell, SpellMemory } from "../../../generated-api";
import { CastSpellOnPlayerView } from "../../actions";

import styles from "./PlayerSpellView.module.css";

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
  <div className={[styles.spell, styles.card].join(" ")}>
    <div className={styles["spell-name"]}>{spell.name}</div>
    <div className={styles.uses}>
      <span>{spell.uses} uses remain</span>
      {castArgs && <CastSpellOnPlayerView args={castArgs} />}
    </div>
  </div>
);

export const PlayerSpellMemoryView: FunctionComponent<
  PlayerSpellMemoryViewProps
> = ({ spellMemory }) => (
  <div className={styles["spell-memory"]}>
    <div className="title">Spell Memory</div>
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
