import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { MovePlayerItem } from "../../generated-api";
import { performMovePlayerItem } from "../../api/actions";
import { useTheme } from "../../themes";

export interface MovePlayerItemViewProps {
  itemId: string,
  equip: boolean;
}

export const MovePlayerItemView: FunctionComponent<MovePlayerItemViewProps> = ({
  itemId,
  equip,
}) => {
  const { theme } = useTheme();
  const onClick = () => {
    const args: MovePlayerItem = {
      item_id: itemId,
      put_at_the_ready: equip,
    };
    performMovePlayerItem(args);
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
      {equip ? "Equip" : "Unequip"}
    </button>
  );
};
