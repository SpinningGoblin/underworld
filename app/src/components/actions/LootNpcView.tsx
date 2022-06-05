import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { LootNpc } from "../../generated-api";
import { performLootNpc } from "../../api/actions";

export interface LootNpcViewProps {
  args: LootNpc;
}

export const LootNpcView: FunctionComponent<LootNpcViewProps> = ({
  args,
}) => {
  const onClick = () => {
    performLootNpc(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      Loot
    </button>
  );
};
