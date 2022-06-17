import { FunctionComponent, ReactElement } from "react";
import {
  GameEvent,
  PerformAction,
  PlayerCharacter,
  Room,
} from "../../generated-api";
import { GameEventView } from "../GameEventView";
import { PlayerView } from "../PlayerView";
import { RoomView } from "../RoomView";
import goblin from "../../images/goblin_big_hat.svg";

import styles from "./styles.module.css";

export interface GameScreenProps {
  actions: Array<PerformAction>;
  allowGeneratePlayer: boolean;
  events: Array<GameEvent>;
  gameIdSelector: ReactElement;
  onClickGeneratePlayer: () => void;
  player: PlayerCharacter;
  room: Room;
}

export const GameScreen: FunctionComponent<GameScreenProps> = ({
  actions,
  allowGeneratePlayer,
  events,
  gameIdSelector,
  onClickGeneratePlayer,
  player,
  room,
}) => (
  <div className={styles.screen}>
    <div className={styles.game}>
      <PlayerView player={player} actions={actions} />
      <RoomView room={room} actions={actions} player={player} />
    </div>
    <div className={styles.side}>
      <header className={styles.header}>
        <img src={goblin} className={styles.logo} alt="logo" />
        <p>Underworld Server</p>
      </header>
      {allowGeneratePlayer && (
        <button className="generate-button" onClick={onClickGeneratePlayer}>
          Generate new player character
        </button>
      )}
      {gameIdSelector}
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
