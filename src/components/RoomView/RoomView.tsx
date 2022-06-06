import { FunctionComponent } from "react";
import {
  ExitRoom,
  FlavourText,
  PerformAction,
  Room,
} from "../../generated-api";
import { ExitView } from "../ExitView/ExitView";

import styles from "./styles.module.css";

export interface RoomViewProps {
  room: Room;
  actions: Array<PerformAction>;
}

const flavourText = (flavour: FlavourText): string => {
  switch (flavour) {
    case "a_strange_breeze_blows":
      return "A strange breeze blows through the room.";
    case "mold_moss_covers_walls":
      return "A strange mold and moss cover the walls.";
    case "unseen_lights_flicker_walls":
      return "Unseen lights flicker across the walls.";
  }
};

const description = (room: Room): string => {
  const sizes: Array<string> = [];

  if (room.dimensions.height !== "average") {
    sizes.push(room.dimensions.height.replaceAll("_", ""));
  }

  if (room.dimensions.length !== "average") {
    sizes.push(room.dimensions.length.replaceAll("_", " "));
  }

  if (room.dimensions.width !== "average") {
    sizes.push(room.dimensions.width.replaceAll("_", " "));
  }

  const descriptors = room.descriptors
    .slice()
    .sort((a, b) => a.localeCompare(b))
    .map((t) => t.replaceAll("_", " "));

  const flavour: Array<string> = [];
  if (room.flavour) {
    flavour.push(flavourText(room.flavour));
  }

  return [
    ...sizes,
    ...descriptors,
    `${room.room_type.replaceAll("_", " ")}.`,
    ...flavour,
  ].join(" ");
};

export const RoomView: FunctionComponent<RoomViewProps> = ({
  room,
  actions,
}) => (
  <div className={styles.room}>
    <span className={styles.description}>{`You are in a ${description(
      room,
    )}`}</span>
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

          return <ExitView key={exit.id} exit={exit} exitAction={exitAction} />;
        })}
      </div>
    )}
  </div>
);