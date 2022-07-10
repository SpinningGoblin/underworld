import { FunctionComponent } from "react";
import { PlayerCharacter } from "../../generated-api";

import styles from "./OptionsScreen.module.css";

export interface OptionsScreenProps {
  openingPage?: boolean;
  gameIds: string[];
  selectedGameId?: string;
  onGameIdChange: (gameId?: string) => void;
  onClickGenerateGame: () => void;
  onClickGeneratePlayer: () => void;
  player?: PlayerCharacter;
}

export const OptionsScreen: FunctionComponent<OptionsScreenProps> = ({
  onClickGenerateGame,
  onClickGeneratePlayer,
  onGameIdChange,
  openingPage,
  gameIds,
  player,
  selectedGameId,
}) => {
  const options = [<option key="empty" value=""></option>];

  gameIds
    .sort((a, b) => a.localeCompare(b))
    .forEach((id) =>
      options.push(
        <option key={id} value={id}>
          {id}
        </option>,
      ),
    );

  return (
    <div className={styles.body}>
      <div className={styles["generate-pc"]}>
        {player && <span>You are a {player.character.species.replaceAll("_", " ")}</span>}
        <button
          className={styles["generate-button"]}
          onClick={onClickGeneratePlayer}
        >
          Generate new PC
        </button>
      </div>
      <div className={openingPage ? "" : styles["game-id-section"]}>
        <div className={styles["game-ids"]}>
          <span className="title">Current Game</span>
          <div className={styles["id-and-generate"]}>
            {gameIds.length > 0 && (
              <select
                className={styles["game-id-select"]}
                value={selectedGameId || ""}
                onChange={(event) => onGameIdChange(event.currentTarget.value)}
              >
                {options}
              </select>
            )}
            <button
              className={styles["generate-button"]}
              onClick={onClickGenerateGame}
            >
              New Game
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
