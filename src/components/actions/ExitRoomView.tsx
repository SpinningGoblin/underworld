import styles from "./styles.module.css";

import { FunctionComponent, useState } from "react";
import { ExitRoom } from "../../generated-api";
import { performExitRoom } from "../../api/actions";
import openDoor from "../../images/open_door.svg";
import closedDoor from "../../images/closed_door.svg";

export interface ExitRoomViewProps {
  args: ExitRoom;
}

export const ExitRoomView: FunctionComponent<ExitRoomViewProps> = ({
  args,
}) => {
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
    >
      <img src={image} alt="open door" height={30} />
    </button>
  );
};
