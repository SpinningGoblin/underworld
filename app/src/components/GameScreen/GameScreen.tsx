import { FunctionComponent, useState } from "react";
import {
  GameEvent,
  PerformAction,
  PlayerCharacter,
  Room,
} from "../../generated-api";
import { GameEventView } from "../GameEventView";
import { PlayerView } from "../PlayerView";
import { RoomView } from "../RoomView";

import styles from "./GameScreen.module.css";

export interface GameScreenProps {
  actions: Array<PerformAction>;
  allowGeneratePlayer: boolean;
  events: Array<GameEvent>;
  onClickGeneratePlayer: () => void;
  player: PlayerCharacter;
  room: Room;
}

export const GameScreen: FunctionComponent<GameScreenProps> = ({
  actions,
  allowGeneratePlayer,
  events,
  onClickGeneratePlayer,
  player,
  room,
}) => {
  const [showFullPlayer, setShowFullPlayer] = useState<boolean>(false);

  return (
    <div className={styles.screen}>
      <div className={styles.game}>
        <PlayerView
          player={player}
          actions={actions}
          toggleShowFullPlayer={() => setShowFullPlayer((current) => !current)}
          showFullPlayer={showFullPlayer}
        />
        {!showFullPlayer && (
          <RoomView room={room} actions={actions} player={player} />
        )}
      </div>
      <div className={styles.side}>
        {allowGeneratePlayer && (
          <button
            className={styles["generate-button"]}
            onClick={onClickGeneratePlayer}
          >
            Generate new PC
          </button>
        )}
        <div className={styles["events-container"]}>
          <span className={["title", styles["events-title"]].join(" ")}>
            Game Events
          </span>
          <div className={styles["events-list"]}>
            {events.map((event, index) => (
              <GameEventView key={index} event={event} />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};
