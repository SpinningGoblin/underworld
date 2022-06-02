import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { LootNpc, Room } from "../../generated-api";
import { performLootNpc } from "../../api/actions";

export interface LootNpcViewProps {
  args: LootNpc;
  room: Room;
}

export const LootNpcView: FunctionComponent<LootNpcViewProps> = ({
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
    performLootNpc(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Loot</b> <b>{args.item_ids.length}</b> items from a{" "}
      <b>{npc.character.species}</b>
    </button>
  );
};
