import { ReactNode, useEffect, useRef, useState } from "react";
import "./App.css";
import { generateGame, getGameIds } from "./api/game";
import { getCurrentGameId, setCurrentGameId } from "./api/current-game";
import {
  GameEvent,
  PerformAction,
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
import { GetReadyScreen } from "./components/GetReadyScreen";
import { GameScreen } from "./components/GameScreen";
import { Header } from "./components/Header";
import { OptionsScreen } from "./components/OptionsScreen";

import OptionsIcon from "./images/options.svg";
import CloseIcon from "./images/close.svg";

type OpeningPromises = [
  Promise<PlayerCharacter>,
  Promise<string[]>,
  Promise<PerformAction[]>,
  Promise<Room | undefined>,
];

export const App = () => {
  const [gameIds, setGameIds] = useState<Array<string>>([]);
  const [gameId, setGameId] = useState<string | undefined>(getCurrentGameId());
  const [room, setRoom] = useState<Room | undefined>();
  const [actions, setActions] = useState<Array<PerformAction>>([]);
  const [player, setPlayer] = useState<PlayerCharacter | undefined>();
  const [events, setEvents] = useState<Array<GameEvent>>([]);
  const [lastEvents, setLastEvents] = useState<Array<GameEvent>>([]);
  const [ready, setReady] = useState<boolean>(false);
  const [showOptions, setShowOptions] = useState<boolean>(false);
  const firstPlayerLoadDone = useRef<boolean>(false);

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
      .then((currentActions) => setActions(currentActions))
      .catch((e) => console.error(e));
  };

  useEffect(() => {
    if (ready && !firstPlayerLoadDone.current) {
      console.log("getting initial");
      console.log(gameId);
      const roomPromise: Promise<Room | undefined> = gameId
        ? getCurrentRoom()
        : Promise.resolve(undefined);
      const actionsPromise: Promise<PerformAction[]> = gameId
        ? getCurrentActions()
        : Promise.resolve([]);

      const promises: OpeningPromises = [
        getCurrentPlayer(),
        getGameIds(),
        actionsPromise,
        roomPromise,
      ];

      Promise.all(promises)
        .then(([pl, ids, actions, room]) => {
          console.log(ids);
          setPlayer(pl);
          setGameIds(ids);
          setActions(actions);
          setRoom(room);
        })
        .catch((e) => console.error(e))
        .finally(() => (firstPlayerLoadDone.current = true));
    }
  }, [ready]);

  useEffect(() => {
    const callback = (error: string) => {
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
      if (actionPerformed.room) {
        setRoom(actionPerformed.room);
      }
      if (actionPerformed.player) {
        setPlayer(actionPerformed.player);
      }
      setActions(actionPerformed.actions);

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
        getCurrentActions(),
        getCurrentPlayer(),
      ]).then(([room, actions, player]) => {
        setRoom(room);
        setActions(actions);
        setPlayer(player);
      });
    } else {
      setCurrentGameId("");
      setRoom(undefined);
      setActions([]);
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
        room={room}
        player={player}
        lastEvents={lastEvents}
        events={events}
        actions={actions}
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
        >
          <img className="options-icon" src={CloseIcon} alt="close" />
        </button>
      );
    } else {
      return (
        <button onClick={() => setShowOptions(true)} className="options-button">
          <img className="options-icon" src={OptionsIcon} alt="options" />
        </button>
      );
    }
  };

  return (
    <div className="App">
      <Header>{headerButton()}</Header>
      {body}
    </div>
  );
};
