import { FunctionComponent } from "react";
import {
  CastSpellOnPlayer,
  PerformAction,
  Spell,
  SpellMemory,
} from "../../../generated-api";
import { CastSpellOnPlayerView } from "../../actions";

import styles from "./styles.module.css";

export interface PlayerSpellMemoryViewProps {
  spellMemory: SpellMemory;
  actions: Array<PerformAction>;
}

interface PlayerSpellViewProps {
  spell: Spell;
  castAction?: PerformAction;
}

const PlayerSpellView: FunctionComponent<PlayerSpellViewProps> = ({
  spell,
  castAction,
}) => (
  <div className={styles.spell}>
    <div className={styles["spell-name"]}>{spell.name}</div>
    <div className={styles.uses}>
      <span>{spell.uses} uses remain</span>
      {castAction && <CastSpellOnPlayerView args={castAction.args!} />}
    </div>
  </div>
);

export const PlayerSpellMemoryView: FunctionComponent<
  PlayerSpellMemoryViewProps
> = ({ spellMemory, actions }) => (
  <div className={styles["spell-memory"]}>
    <div className={styles.title}>Spell Memory</div>
    {(spellMemory.spells || []).length === 0 && <span>No spells in memory</span>}
    <div className={styles["spell-list"]}>
      {(spellMemory.spells || []).map((learnedSpell) => {
        const action = actions.find(
          (action) =>
            action.name === "cast_spell_on_player" &&
            (action.args! as CastSpellOnPlayer).spell_id === learnedSpell.id,
        );

        return (
          <PlayerSpellView
            key={learnedSpell.id}
            spell={learnedSpell.spell}
            castAction={action}
          />
        );
      })}
    </div>
  </div>
);
