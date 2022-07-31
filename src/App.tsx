import { ReactNode, useEffect, useRef, useState } from "react";
import "./App.css";
import { generateGame, getGameIds } from "./api/game";
import { getCurrentGameId, setCurrentGameId } from "./api/current-game";
import {
  GameEvent,
  PlayerCharacter,
  Room,
} from "./generated-api";
import {
  ActionPerformed,
  getCurrentActions,
  getCurrentRoom,
  listenActionPerformed,
  listenError,
  removeActionPerformedListener,
  removeErrorListener,
} from "./api/actions";
import { generatePlayer, getCurrentPlayer } from "./api/player";
import { GameScreen, GetReadyScreen, OptionsScreen } from "./screens";
import { Header } from "./components/Header";

import OptionsIcon from "./images/options.svg";
import CloseIcon from "./images/close.svg";
import { useTheme } from "./themes/context";

type OpeningPromises = [
  Promise<PlayerCharacter>,
  Promise<string[]>,
  Promise<Room | undefined>,
];

export const App = () => {
  const { theme } = useTheme();
  const [gameIds, setGameIds] = useState<Array<string>>([]);
  const [gameId, setGameId] = useState<string | undefined>(getCurrentGameId());
  const [room, setRoom] = useState<Room | undefined>();
  const [player, setPlayer] = useState<PlayerCharacter | undefined>();
  const [events, setEvents] = useState<Array<GameEvent>>([]);
  const [lastEvents, setLastEvents] = useState<Array<GameEvent>>([]);
  const [ready, setReady] = useState<boolean>(false);
  const [showOptions, setShowOptions] = useState<boolean>(false);
  const firstPlayerLoadDone = useRef<boolean>(false);
  const [error, setError] = useState<string>();

  const onClickGenerateGame = () => {
    generateGame()
      .then((generatedGame) => {
        setGameIds((existing) => [...existing, generatedGame.game_id]);
        setGameId(generatedGame.game_id);
        setEvents([]);
        setLastEvents([]);
      })
      .catch((e) => console.error(e));
  };

  const onClickGeneratePlayer = () => {
    generatePlayer()
      .then((generatedPlayer) => {
        setPlayer(generatedPlayer);
        return getCurrentActions();
      })
      .catch((e) => console.error(e));
  };

  useEffect(() => {
    if (ready && !firstPlayerLoadDone.current) {
      console.log("getting initial");
      console.log(gameId);
      const roomPromise: Promise<Room | undefined> = gameId
        ? getCurrentRoom()
        : Promise.resolve(undefined);

      const promises: OpeningPromises = [
        getCurrentPlayer(),
        getGameIds(),
        roomPromise,
      ];

      Promise.all(promises)
        .then(([pl, ids, room]) => {
          console.log(ids);
          setPlayer(pl);
          setGameIds(ids);
          setRoom(room);
        })
        .catch((e) => console.error(e))
        .finally(() => (firstPlayerLoadDone.current = true));
    }
  }, [ready]);

  useEffect(() => {
    const callback = (error: string) => {
      setError(error);
      console.error(error);
      if (error === "PlayerIsDeadError") {
        alert("You can't do that with a dead character.");
      }
    };
    listenError(callback);

    return () => removeErrorListener(callback);
  }, []);

  useEffect(() => {
    const callback = (actionPerformed: ActionPerformed) => {
      setError(undefined);
      if (actionPerformed.room) {
        setRoom(actionPerformed.room);
      }
      if (actionPerformed.player) {
        setPlayer(actionPerformed.player);
      }

      const events = actionPerformed.events.slice();
      events.reverse();

      setEvents((existing) => [...events, ...existing]);
      setLastEvents(events);

      for (const event of actionPerformed.events) {
        if (event.name === "player_killed") {
          alert("You Died!");
        }
      }
    };
    listenActionPerformed(callback);

    return () => removeActionPerformedListener(callback);
  }, []);

  useEffect(() => {
    if (gameId) {
      console.log(`here ${gameId}`);
      setCurrentGameId(gameId);
      Promise.all([
        getCurrentRoom(),
        getCurrentPlayer(),
      ]).then(([room, player]) => {
        setRoom(room);
        setPlayer(player);
      });
    } else {
      setCurrentGameId("");
      setRoom(undefined);
    }
  }, [gameId]);

  const allowGeneratePlayer =
    !player || player.character.stats.health!.current === 0;

  let body: ReactNode;
  if (!ready) {
    body = <GetReadyScreen onReadyClicked={() => setReady(true)} />;
  } else if (room && player && !showOptions) {
    body = (
      <GameScreen
        error={error}
        room={room}
        player={player}
        lastEvents={lastEvents}
        events={events}
        allowGeneratePlayer={allowGeneratePlayer}
        onClickGeneratePlayer={onClickGeneratePlayer}
      />
    );
  } else {
    body = (
      <OptionsScreen
        player={player}
        onClickGeneratePlayer={onClickGeneratePlayer}
        onClickGenerateGame={onClickGenerateGame}
        gameIds={gameIds}
        selectedGameId={gameId}
        onGameIdChange={setGameId}
      />
    );
  }

  const headerButton = () => {
    if (!ready || !player || !room) {
      return <></>;
    }

    if (showOptions) {
      return (
        <button
          onClick={() => setShowOptions(false)}
          className="options-button"
          style={{ backgroundColor: theme.colors.secondary }}
        >
          <img className="options-icon" src={CloseIcon} alt="close" />
        </button>
      );
    } else {
      return (
        <button
          onClick={() => setShowOptions(true)}
          className="options-button"
          style={{ backgroundColor: theme.colors.secondary }}
        >
          <img className="options-icon" src={OptionsIcon} alt="options" />
        </button>
      );
    }
  };

  return (
    <div className="App" style={{ backgroundColor: theme.colors.primary }}>
      <Header>{headerButton()}</Header>
      {body}
    </div>
  );
};
