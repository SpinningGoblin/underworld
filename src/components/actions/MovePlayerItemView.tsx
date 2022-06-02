import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { MovePlayerItem, PlayerCharacter } from "../../generated-api";
import { performMovePlayerItem } from "../../api/actions";

export interface MovePlayerItemViewProps {
  args: MovePlayerItem;
  player: PlayerCharacter;
}

export const MovePlayerItemView: FunctionComponent<MovePlayerItemViewProps> = ({
  args,
  player,
}) => {
  const item = player.character.inventory!.equipment.find(
    (characterItem) => characterItem.item.identifier.id === args.item_id,
  );

  if (!item) {
    return <></>;
  }

  const equipText = args.put_at_the_ready ? "Equip" : "Unequip";

  const onClick = () => {
    performMovePlayerItem(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>{equipText}</b> <b>{item.item.item_type}</b> to{" "}
      <b>{args.location_tag}</b>
    </button>
  );
};
