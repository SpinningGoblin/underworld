import styles from "./styles.module.css";

import { FunctionComponent, useState } from "react";
import { ExitRoom } from "../../generated-api";
import { performExitRoom } from "../../api/actions";
import openDoor from "../../images/open_door.svg";
import closedDoor from "../../images/closed_door.svg";
import { useTheme } from "../../themes";

export interface ExitRoomViewProps {
  args: ExitRoom;
}

export const ExitRoomView: FunctionComponent<ExitRoomViewProps> = ({
  args,
}) => {
  const { theme } = useTheme();
  const [image, setImage] = useState<string>(closedDoor);

  const onClick = () => {
    performExitRoom(args);
  };

  return (
    <button
      onClick={onClick}
      className={styles["exit-button"]}
      onMouseEnter={() => setImage(openDoor)}
      onMouseLeave={() => setImage(closedDoor)}
      style={{
        backgroundColor: theme.colors.secondary,
        color: theme.colors.primary,
      }}
    >
      <img className={styles["exit-door"]} src={image} alt="open door" />
    </button>
  );
};
