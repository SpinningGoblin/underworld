import { FunctionComponent, useState } from "react";
import {
  GameEvent,
  PerformAction,
  PlayerCharacter,
  Room,
} from "../../generated-api";
import { useTheme } from "../../themes/context";
import { GameEventView } from "../GameEventView";
import { PlayerView } from "../PlayerView";
import { RoomView } from "../RoomView";

import styles from "./GameScreen.module.css";

export interface GameScreenProps {
  actions: Array<PerformAction>;
  allowGeneratePlayer: boolean;
  lastEvents: Array<GameEvent>;
  events: Array<GameEvent>;
  onClickGeneratePlayer: () => void;
  player: PlayerCharacter;
  room: Room;
  error?: string;
}

export const GameScreen: FunctionComponent<GameScreenProps> = ({
  actions,
  allowGeneratePlayer,
  lastEvents,
  events,
  onClickGeneratePlayer,
  player,
  room,
  error,
}) => {
  const { theme } = useTheme();
  const [showFullPlayer, setShowFullPlayer] = useState(false);
  const [showAllEvents, setShowAllEvents] = useState(false);

  const showText = showAllEvents ? "Hide" : "Show all events";

  return (
    <div className={styles.screen} style={{ color: theme.colors.secondary }}>
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
            style={{
              backgroundColor: theme.colors.secondary,
              color: theme.colors.primary,
            }}
          >
            Generate new PC
          </button>
        )}
        <div className={styles["events-container"]}>
          {error && (
            <>
              <h2>Error</h2>
              <div
                className={[styles["events-list"]].join(" ")}
                style={{ color: theme.colors.error }}
              >
                {error}
              </div>
            </>
          )}
          <h2>Last Game Events</h2>
          <div
            className={styles["events-list"]}
            style={{ borderColor: theme.colors.secondary }}
          >
            {lastEvents.map((event, index) => (
              <GameEventView key={index} event={event} />
            ))}
          </div>
          <div
            className={[styles["events-list"], styles["all-events"]].join(" ")}
            style={{ borderColor: theme.colors.secondary }}
          >
            <button
              className={styles["generate-button"]}
              onClick={() => setShowAllEvents((current) => !current)}
            >
              {showText}
            </button>
            {showAllEvents &&
              events.map((event, index) => (
                <GameEventView key={index} event={event} />
              ))}
          </div>
        </div>
      </div>
    </div>
  );
};
