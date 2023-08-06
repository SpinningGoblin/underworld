import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { SellPlayerItem } from "../../generated-api";
import { performSellPlayerItem } from "../../api/actions";
import { useTheme } from "../../themes";

export interface SellPlayerItemViewProps {
  itemId: string;
}

export const SellPlayerItemView: FunctionComponent<SellPlayerItemViewProps> = ({
  itemId,
}) => {
  const { theme } = useTheme();

  const onClick = () => {
    const args: SellPlayerItem = {
      item_id: itemId,
    };

    performSellPlayerItem(args);
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
      Sell
    </button>
  );
};
