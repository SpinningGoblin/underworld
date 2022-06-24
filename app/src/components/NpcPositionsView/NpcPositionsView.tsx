import { FunctionComponent } from "react";
import { PlayerCharacter, Room } from "../../generated-api";
import { NpcPositionView } from "../NpcPositionView";

import styles from "./styles.module.css";

export interface NpcPositionsViewProps {
  room: Room;
  player: PlayerCharacter;
}

export const NpcPositionsView: FunctionComponent<NpcPositionsViewProps> = ({
  player,
  room,
}) => (
  <div className={styles.npcs}>
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
