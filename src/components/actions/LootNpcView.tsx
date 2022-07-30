import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { LootNpc } from "../../generated-api";
import { performLootNpc } from "../../api/actions";
import { useTheme } from "../../themes";

export interface LootNpcViewProps {
  args: LootNpc;
}

export const LootNpcView: FunctionComponent<LootNpcViewProps> = ({ args }) => {
  const { theme } = useTheme();
  const onClick = () => {
    performLootNpc(args);
  };

  return (
    <button
      onClick={onClick}
      className={styles["action-button"]}
      style={{
        backgroundColor: theme.colors.secondary,
        color: theme.colors.primary,
      }}
    >
      Loot
    </button>
  );
};
