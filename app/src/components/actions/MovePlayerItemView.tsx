import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { EquipLocationTag, MovePlayerItem } from "../../generated-api";
import { performMovePlayerItem } from "../../api/actions";

export interface MovePlayerItemViewProps {
  args: MovePlayerItem;
}

const tagText = (locationTag: EquipLocationTag): string => {
  const text = locationTag.replace("_", " ");

  const upper = text[0].toLocaleUpperCase();
  return `${upper}${text.substring(1)}`;
};

export const MovePlayerItemView: FunctionComponent<MovePlayerItemViewProps> = ({
  args,
}) => {
  const onClick = () => {
    performMovePlayerItem(args);
  };

  return (
    <button onClick={onClick} className={styles["action-button"]}>
      {tagText(args.location_tag)}
    </button>
  );
};
