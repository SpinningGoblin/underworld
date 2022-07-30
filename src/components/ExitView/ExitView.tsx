import { FunctionComponent, useState } from "react";
import { Exit, ExitRoom, ExitType } from "../../generated-api";
import { useTheme } from "../../themes";
import { ExitRoomView } from "../actions";

import styles from "./ExitView.module.css";

export interface ExitViewProps {
  exit: Exit;
  exitArgs?: ExitRoom;
}

const typeText = (exitType: ExitType): string => exitType.replaceAll("_", " ");

export const ExitView: FunctionComponent<ExitViewProps> = ({
  exit,
  exitArgs,
}) => {
  const { theme } = useTheme();
  const [hovering, setHovering] = useState(false);

  return (
    <div
      className={styles.card}
      onMouseEnter={() => setHovering(true)}
      onMouseLeave={() => setHovering(false)}
      style={{
        borderColor: hovering ? theme.colors.secondary : theme.colors.primary,
        color: theme.colors.secondary,
      }}
    >
      <span className={styles.description}>
        {`${[...exit.descriptors, typeText(exit.exit_type)].join(" ")} ${
          exit.has_visited_connected_room ? "(Traveled before)" : "(Never seen)"
        }`}
        {exitArgs && <ExitRoomView args={exitArgs} />}
      </span>
    </div>
  );
};
