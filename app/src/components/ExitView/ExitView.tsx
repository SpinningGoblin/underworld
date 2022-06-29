import { FunctionComponent } from "react";
import { Exit, ExitRoom, ExitType } from "../../generated-api";
import { ExitRoomView } from "../actions";

import styles from "./styles.module.css";

export interface ExitViewProps {
  exit: Exit;
  exitArgs?: ExitRoom;
}

const typeText = (exitType: ExitType): string => exitType.replaceAll("_", " ");

export const ExitView: FunctionComponent<ExitViewProps> = ({
  exit,
  exitArgs,
}) => (
  <div className={styles.card}>
    <span className={styles.description}>
      {`${[...exit.descriptors, typeText(exit.exit_type)].join(" ")} ${
        exit.has_visited_connected_room ? "(Traveled before)" : "(Never seen)"
      }`}
      {exitArgs && <ExitRoomView args={exitArgs} />}
    </span>
  </div>
);
