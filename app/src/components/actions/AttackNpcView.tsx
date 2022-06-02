import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { AttackNpc, Room } from "../../generated-api";
import { performAttackNpc } from "../../api/actions";

export interface AttackNpcViewProps {
  args: AttackNpc;
  room: Room;
}

export const AttackNpcView: FunctionComponent<AttackNpcViewProps> = ({
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
    performAttackNpc(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Attack</b> a <b>{npc.character.species}</b>
    </button>
  );
};
