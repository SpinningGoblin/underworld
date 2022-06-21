import { FunctionComponent } from "react";
import { PlayerCharacter, Room } from "../../generated-api";
import { NpcPositionView } from "../NpcPositionView";

import styles from "./styles.module.css";

export interface NpcPositionsViewProps {
  room: Room;
  player: PlayerCharacter;
}

const npcText = (room: Room): string => {
  if (!room.npc_positions.length) {
    return "There are no other creatures in the room.";
  }

  const creatureText =
    room.npc_positions.length === 1
      ? "is 1 creature"
      : `are ${room.npc_positions.length} creatures`;

  return `There ${creatureText} in the room with you.`;
};

export const NpcPositionsView: FunctionComponent<NpcPositionsViewProps> = ({
  player,
  room,
}) => (
  <div className={styles.npcs}>
    <div className={styles["npc-text"]}>{npcText(room)}</div>
    <div className={styles["npc-list"]}>
      {room.npc_positions.map((npcPosition) => (
        <NpcPositionView
          key={npcPosition.npc.id}
          npcPosition={npcPosition}
          player={player}
        />
      ))}
    </div>
  </div>
);
