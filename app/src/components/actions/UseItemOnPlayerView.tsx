import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { UseItemOnPlayer } from "../../generated-api";
import { performUseItemOnPlayer } from "../../api/actions";
import { useTheme } from "../../themes";

export interface UseItemOnPlayerViewProps {
  args: UseItemOnPlayer;
}

export const UseItemOnPlayerView: FunctionComponent<
  UseItemOnPlayerViewProps
> = ({ args }) => {
  const { theme } = useTheme();
  const onClick = () => {
    performUseItemOnPlayer(args);
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
      Use
    </button>
  );
};
