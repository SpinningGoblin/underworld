import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { UseItemOnPlayer } from "../../generated-api";
import { performUseItemOnPlayer } from "../../api/actions";

export interface UseItemOnPlayerViewProps {
  args: UseItemOnPlayer;
}

export const UseItemOnPlayerView: FunctionComponent<
  UseItemOnPlayerViewProps
> = ({ args }) => {
  const onClick = () => {
    performUseItemOnPlayer(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      Use
    </button>
  );
};
