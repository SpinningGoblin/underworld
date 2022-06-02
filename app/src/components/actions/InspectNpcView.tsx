import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { InspectNpc, Room } from "../../generated-api";
import { performInspectNpc } from "../../api/actions";

export interface InspectNpcViewProps {
  args: InspectNpc;
  room: Room;
}

export const InspectNpcView: FunctionComponent<InspectNpcViewProps> = ({
  args,
  room,
}) => {
  const npc = room.npc_positions
    .map((npcPosition) => npcPosition.npc)
    .find((npc) => npc.identifier.id === args.npc_id);

  if (!npc) {
    return <div>NPC not found in room</div>;
  }

  const onClick = () => {
    performInspectNpc(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Inspect</b> a <b>{npc.character.species}</b>
    </button>
  );
};
