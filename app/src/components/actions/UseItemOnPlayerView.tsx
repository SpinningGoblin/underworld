import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { PlayerCharacter, UseItemOnPlayer } from "../../generated-api";
import { performUseItemOnPlayer } from "../../api/actions";

export interface UseItemOnPlayerViewProps {
  args: UseItemOnPlayer;
  player: PlayerCharacter;
}

export const UseItemOnPlayerView: FunctionComponent<
  UseItemOnPlayerViewProps
> = ({ args, player }) => {
  const item = (player.character.inventory?.equipment || []).map(c => c.item).find(
    (item) => item.identifier.id === args.item_id,
  );

  if (!item) {
    return <div>Item not found in inventory</div>;
  }

  const onClick = () => {
    performUseItemOnPlayer(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Use</b> <b>{item.item_type}</b> on player
    </button>
  );
};
