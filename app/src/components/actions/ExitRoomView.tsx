import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { ExitRoom } from "../../generated-api";
import { performExitRoom } from "../../api/actions";

export interface ExitRoomViewProps {
  args: ExitRoom;
}

export const ExitRoomView: FunctionComponent<ExitRoomViewProps> = ({
  args,
}) => {
  const onClick = () => {
    performExitRoom(args);
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      Use exit
    </button>
  );
};
