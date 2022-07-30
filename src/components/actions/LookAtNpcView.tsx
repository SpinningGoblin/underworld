import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { LookAtNpc, Room } from "../../generated-api";
import { performLookAtNpc } from "../../api/actions";
import { useTheme } from "../../themes";

export interface LookAtNpcViewProps {
  args: LookAtNpc;
  room: Room;
}

export const LookAtNpcView: FunctionComponent<LookAtNpcViewProps> = ({
  args,
  room,
}) => {
  const { theme } = useTheme();
  const npc = room.npc_positions
    .map((npcPosition) => npcPosition.npc)
    .find((npc) => npc.id === args.npc_id);

  if (!npc) {
    return <div>NPC not found in room</div>;
  }

  const onClick = () => {
    performLookAtNpc(args).catch((e) => console.error(e));
  };

  return (
    <button
      onClick={onClick}
      className={styles["action-button"]}
      style={{
        backgroundColor: theme.colors.secondary,
        color: theme.colors.primary,
      }}
    >
      <b>Look</b> at <b>{npc.character.species}</b>
    </button>
  );
};
