import { FunctionComponent } from "react";
import {
  Exit,
  ExitRoom,
  FixturePosition,
  FlavourText,
  NpcPosition,
  PerformAction,
  PlayerCharacter,
  Room,
} from "../../generated-api";
import { ExitView } from "../ExitView/ExitView";
import { FixturePositionsView } from "../FixturePositionsView";
import { NpcPositionsView } from "../NpcPositionsView";

import styles from "./styles.module.css";

export interface RoomViewProps {
  room: Room;
  actions: Array<PerformAction>;
  player: PlayerCharacter;
}

const flavourText = (flavour: FlavourText): string => {
  switch (flavour) {
    case "a_strange_breeze_blows":
      return "A strange breeze blows through the room.";
    case "mold_moss_covers_walls":
      return "A strange mold and moss cover the walls.";
    case "unseen_lights_flicker_walls":
      return "Unseen lights flicker across the walls.";
    case "is_something_watching_you":
      return "It feels like there is something you can't see watching you.";
    case "smells_like_old_goblin_socks":
      return "It smells like old goblin socks. Where is that smell coming from?";
    case "something_squishy_all_over_floor":
      return "There's something squishy all over the floor...";
    case "you_hear_scratching_all_around_you":
      return "You hear scratching all around you... But from what?";
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

const exitText = (exits: Exit[]): string => {
  const seenBefore = exits.filter((exit) => exit.has_visited_connected_room);

  return `You see ${exits.length} exits. ${seenBefore.length} you have been through before.`;
};

const npcText = (npcPositions: NpcPosition[]): string => {
  if (!npcPositions.length) {
    return "There are no other creatures in the room.";
  }

  const creatureText =
    npcPositions.length === 1
      ? "is 1 creature"
      : `are ${npcPositions.length} creatures`;

  return `There ${creatureText} in the room with you.`;
};

const fixtureText = (fixturePositions: FixturePosition[]): string => {
  if (!fixturePositions.length) {
    return "There is nothing else interesting in the room.";
  }

  const itemText =
    fixturePositions.length === 1
      ? "is 1 item"
      : `are ${fixturePositions.length} items`;

  return `There ${itemText} in the room with you.`;
};

export const RoomView: FunctionComponent<RoomViewProps> = ({
  room,
  player,
}) => (
  <div className={styles.room}>
    <span className={styles.description}>
      {`You are in a ${description(room)} `}
      {`${exitText(room.exits)} `}
      {`${npcText(room.npc_positions)} `}
      {`${fixtureText(room.fixture_positions)} `}
    </span>
    {room.exits.length > 0 && (
      <div className={styles.exits}>
        {room.exits.map((exit) => {
          const exitArgs: ExitRoom = {
            exit_id: exit.id,
          };

          return <ExitView key={exit.id} exit={exit} exitArgs={exitArgs} />;
        })}
      </div>
    )}
    <NpcPositionsView room={room} player={player} />
    <hr className={styles.divider} />
    <FixturePositionsView room={room} />
  </div>
);
