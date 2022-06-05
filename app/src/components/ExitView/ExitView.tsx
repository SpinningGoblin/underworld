import { FunctionComponent } from "react";
import { Exit, ExitType, PerformAction } from "../../generated-api";
import { ExitRoomView } from "../actions";

import styles from "./styles.module.css";

export interface ExitViewProps {
  exit: Exit;
  exitAction?: PerformAction;
}

const typeText = (exitType: ExitType): string => exitType.replaceAll("_", " ");

export const ExitView: FunctionComponent<ExitViewProps> = ({
  exit,
  exitAction,
}) => (
  <div className={styles.exit}>
    <span className={styles.description}>
      {`${[...exit.descriptors, typeText(exit.exit_type)].join(" ")} ${
        exit.has_visited_connected_room ? "(Traveled before)" : "(Never seen)"
      }`}
      {exitAction && <ExitRoomView args={exitAction.args!} />}
    </span>
  </div>
);
