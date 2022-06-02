import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { CastSpellOnPlayer, PlayerCharacter } from "../../generated-api";
import { performCastSpellOnPlayer } from "../../api/actions";

export interface CastSpellOnPlayerViewProps {
  args: CastSpellOnPlayer;
  player: PlayerCharacter;
}

export const CastSpellOnPlayerView: FunctionComponent<
  CastSpellOnPlayerViewProps
> = ({ args, player }) => {
  const learnedSpell = (player.character.spell_memory?.spells || []).find(
    (learnedSpell) => learnedSpell.identifier.id === args.spell_id,
  );

  if (!learnedSpell) {
    return <div>Spell not found in memory</div>;
  }

  const onClick = () => {
    performCastSpellOnPlayer(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Cast</b> <b>{learnedSpell.spell.name}</b> on player
    </button>
  );
};
