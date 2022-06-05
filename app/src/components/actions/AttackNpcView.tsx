import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { AttackNpc } from "../../generated-api";
import { performAttackNpc } from "../../api/actions";

export interface AttackNpcViewProps {
  args: AttackNpc;
}

export const AttackNpcView: FunctionComponent<AttackNpcViewProps> = ({
  args,
}) => {
  const onClick = () => {
    performAttackNpc(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles["attack-button"]}>
      Attack
    </button>
  );
};
