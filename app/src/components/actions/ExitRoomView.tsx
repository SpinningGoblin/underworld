import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { ExitRoom, Room } from "../../generated-api";
import { performExitRoom } from "../../api/actions";

export interface ExitRoomViewProps {
  args: ExitRoom;
  room: Room;
}

export const ExitRoomView: FunctionComponent<ExitRoomViewProps> = ({
  args,
  room,
}) => {
  const exit = room.exits.find((exit) => exit.identifier.id === args.exit_id);

  if (!exit) {
    return <div>Exit not found in room</div>;
  }

  const onClick = () => {
    performExitRoom(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Exit room</b> by <b>{exit.exit_type}</b>
    </button>
  );
};
