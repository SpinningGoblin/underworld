import { FunctionComponent, useState } from "react";
import { performCastSpellOnNpc } from "../../api/actions";
import { CastSpellOnNpc, LearnedSpell, SpellName } from "../../generated-api";

import styles from "./styles.module.css";

export interface CastSpellOnNpcViewProps {
  learnedSpells: Array<LearnedSpell>;
  npcId: string;
}

const spellNameText = (spellName: SpellName): string => {
  switch (spellName) {
    case "electric_blast":
      return "electric blast";
    case "poison_cloud":
      return "poison cloud";
    case "poison_dart":
      return "poison dart";
    case "raging_fireball":
      return "raging fireball";
    default:
      return "";
  }
};

export const CastSpellOnNpcView: FunctionComponent<CastSpellOnNpcViewProps> = ({
  learnedSpells,
  npcId,
}) => {
  const [spellId, setSpellId] = useState<string>(learnedSpells[0].id);

  const options = learnedSpells.map((learnedSpell) => (
    <option key={learnedSpell.id} value={learnedSpell.id}>
      {spellNameText(learnedSpell.spell.name)}
    </option>
  ));

  const onClick = () => {
    const castSpellOnNpc: CastSpellOnNpc = {
      spell_id: spellId,
      npc_id: npcId,
    };

    performCastSpellOnNpc(castSpellOnNpc);
  };

  return (
    <div className={styles["spells-on-npc"]}>
      <select
        value={spellId}
        onChange={(event) => setSpellId(event.currentTarget.value)}
      >
        {options}
      </select>
      <button onClick={onClick} className={styles["action-button"]}>
        Cast
      </button>
    </div>
  );
};
