import { FunctionComponent } from "react";
import { PlayerCharacter } from "../../generated-api";
import { themes } from "../../themes";
import { useTheme } from "../../themes/context";

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
  const { theme, setTheme } = useTheme();
  const options = [<option key="empty" value=""></option>];

  const themeOptions = themes.map((theme) => (
    <option key={theme.name} value={theme.name}>
      {theme.name}
    </option>
  ));

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
    <div className={styles.body} style={{ color: theme.colors.secondary }}>
      <div className={styles["generate-pc"]}>
        {player && (
          <span>You are a {player.character.species.replaceAll("_", " ")}</span>
        )}
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
      </div>
      <div className={openingPage ? "" : styles["id-section"]}>
        <div className={styles.section}>
          <h2>Current Game</h2>
          <div className={styles["id-and-generate"]}>
            {gameIds.length > 0 && (
              <select
                className={styles.select}
                value={selectedGameId || ""}
                onChange={(event) => onGameIdChange(event.currentTarget.value)}
                style={{
                  backgroundColor: theme.colors.secondary,
                  color: theme.colors.primary,
                }}
              >
                {options}
              </select>
            )}
            {player && (
              <button
                className={styles["generate-button"]}
                onClick={onClickGenerateGame}
                style={{
                  backgroundColor: theme.colors.secondary,
                  color: theme.colors.primary,
                }}
              >
                New Game
              </button>
            )}
            {!player && <span>Generate a player to generate a game</span>}
          </div>
        </div>
      </div>
      <div className={openingPage ? "" : styles["id-section"]}>
        <div className={styles.section}>
          <h2>Site Theme</h2>
          <select
            className={styles.select}
            value={theme.name}
            onChange={(event) => {
              const theme = themes.find(
                (t) => t.name === event.currentTarget.value,
              );
              theme && setTheme(theme);
            }}
            style={{
              backgroundColor: theme.colors.secondary,
              color: theme.colors.primary,
            }}
          >
            {themeOptions}
          </select>
        </div>
      </div>
    </div>
  );
};
