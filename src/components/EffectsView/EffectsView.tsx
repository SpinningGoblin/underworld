import { FunctionComponent } from "react";
import { Effects } from "../../generated-api";

import styles from "./styles.module.css";

export interface EffectsViewProps {
  effects: Effects;
}

export const EffectsView: FunctionComponent<EffectsViewProps> = ({
  effects,
}) => (
  <div className={styles.effects}>
    <div className="title">Current Effects</div>
    {!effects.resurrection_aura &&
      !effects.retribution_aura &&
      !effects.shield_aura &&
      !effects.poison && <span>No current effects</span>}

    <span>{effects.resurrection_aura && "Has resurrection aura"}</span>
    <span>
      {effects.retribution_aura &&
        `Retribution aura dealing ${effects.retribution_aura.num_rolls} rolls when hit`}
    </span>
    <span>
      {effects.shield_aura &&
        `Shield aura with ${effects.shield_aura.damage_resistance} damage resistance`}
    </span>
    <span>
      {effects.poison &&
        `Poisoned taking ${effects.poison.damage} every ${effects.poison.duration} actions`}
    </span>
  </div>
);
