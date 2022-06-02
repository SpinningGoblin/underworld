import { FunctionComponent } from "react";
import { GameEvent } from "../generated-api";

import styles from "./GameEventView.module.css";

export interface GameEventViewProps {
  event: GameEvent;
}

export const GameEventView: FunctionComponent<GameEventViewProps> = ({
  event,
}) => {
  const onClick = () => {
    if (event.data) {
      alert(JSON.stringify(event.data, null, 2));
    }
  };

  return (
    <button onClick={onClick} className={styles.event}>
      {event.name}
    </button>
  );
};
