import { FunctionComponent } from "react";
import { ExitRoom, PerformAction, Room, RoomType } from "../../generated-api";
import { ExitView } from "../ExitView/ExitView";

import styles from "./styles.module.css";

export interface RoomViewProps {
  room: Room;
  actions: Array<PerformAction>;
}

const typeText = (roomType: RoomType): string => {
  switch (roomType) {
    case "cave":
      return "Cave";
    case "cavern":
      return "Cavern";
    case "cemetery":
      return "Cemetery";
    case "crypt":
      return "Crypt";
    case "entry_way":
      return "Entry way";
    case "mausoleum":
      return "Mausoleum";
    case "prison_cell":
      return "Prison cell";
    case "room":
      return "Room";
    case "tavern_hall":
      return "Tavern hall";
    case "temple_hall":
      return "Temple hall";
  }
};

export const RoomView: FunctionComponent<RoomViewProps> = ({
  room,
  actions,
}) => {
  const description = [
    ...room.descriptors.sort((a, b) => a.localeCompare(b)),
    typeText(room.room_type),
  ].join(" ");

  return (
    <div className={styles.room}>
      <span
        className={styles.description}
      >{`You are in a ${description}`}</span>
      {room.exits.length > 0 && (
        <div className={styles.exits}>
          <span
            className={styles["exit-title"]}
          >{`You see ${room.exits.length} exits you can jump through.`}</span>
          {room.exits.map((exit) => {
            const exitAction = actions.find(
              (action) =>
                action.name === "exit_room" &&
                (action.args! as ExitRoom).exit_id === exit.id,
            );

            return (
              <ExitView key={exit.id} exit={exit} exitAction={exitAction} />
            );
          })}
        </div>
      )}
    </div>
  );
};
