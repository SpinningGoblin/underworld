import { FunctionComponent, useState } from "react";
import { GameEvent, PlayerCharacter, Room } from "../../generated-api";
import { useTheme } from "../../themes/context";
import { GameEventView } from "../../components/GameEventView";
import { PlayerView } from "../../components/PlayerView";
import { RoomView } from "../../components/RoomView";

import styles from "./GameScreen.module.css";

export interface GameScreenProps {
  allowGeneratePlayer: boolean;
  lastEvents: Array<GameEvent>;
  events: Array<GameEvent>;
  onClickGeneratePlayer: () => void;
  player: PlayerCharacter;
  room: Room;
  error?: string;
}

export const GameScreen: FunctionComponent<GameScreenProps> = ({
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
          toggleShowFullPlayer={() => setShowFullPlayer((current) => !current)}
          showFullPlayer={showFullPlayer}
        />
        {!showFullPlayer && <RoomView room={room} player={player} />}
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
          {events.length > 0 && (
            <div
              className={[styles["events-list"], styles["all-events"]].join(
                " ",
              )}
              style={{ borderColor: theme.colors.secondary }}
            >
              <button
                className={styles["generate-button"]}
                onClick={() => setShowAllEvents((current) => !current)}
                style={{
                  backgroundColor: theme.colors.secondary,
                  color: theme.colors.primary,
                }}
              >
                {showText}
              </button>
              {showAllEvents &&
                events.map((event, index) => (
                  <GameEventView key={index} event={event} />
                ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
