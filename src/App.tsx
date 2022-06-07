import { useEffect, useRef, useState } from "react";
import goblin from "./images/goblin_big_hat.svg";
import "./App.css";
import { getUsername } from "./api/username";
import { generateGame, getGameIds } from "./api/game";
import { setCurrentGameId } from "./api/current-game";
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
import { GameEventView } from "./components/GameEventView";
import { RoomView } from "./components/RoomView";
import { PlayerView } from "./components/PlayerView";
import { GetReadyScreen } from "./components/GetReadyScreen";

export const App = () => {
  const [gameIds, setGameIds] = useState<Array<string>>([]);
  const [gameId, setGameId] = useState<string | undefined>();
  const [room, setRoom] = useState<Room | undefined>();
  const [actions, setActions] = useState<Array<PerformAction>>([]);
  const [player, setPlayer] = useState<PlayerCharacter | undefined>();
  const [events, setEvents] = useState<Array<GameEvent>>([]);
  const [ready, setReady] = useState<boolean>(false);
  const firstPlayerLoadDone = useRef<boolean>(false);

  const onClickGetGameIds = async () => {
    setGameIds(await getGameIds());
  };

  const onClickGenerateGame = () => {
    generateGame()
      .then((generatedGame) => {
        setGameIds((existing) => [...existing, generatedGame.game_id]);
        setGameId(generatedGame.game_id);
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
      getCurrentPlayer()
        .then(setPlayer)
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

      setEvents((existing) => [...actionPerformed.events, ...existing]);

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

  const renderBody = () => {
    if (!ready) {
      return <GetReadyScreen onReadyClicked={() => setReady(true)} />;
    }

    const allowGeneratePlayer =
      !player || player.character.stats.health!.current === 0;

    return (
      <>
        <div className="basics">
          <span className="username">User: {getUsername()}</span>
          {allowGeneratePlayer && (
            <button className="generate-button" onClick={onClickGeneratePlayer}>
              Generate new player character
            </button>
          )}
          {player && (
            <button className="generate-button" onClick={onClickGenerateGame}>
              Generate a new game
            </button>
          )}
          {player && gameIds.length === 0 && (
            <button className="generate-button" onClick={onClickGetGameIds}>
              Get game IDs
            </button>
          )}
          <div className="game-ids">
            {gameIds.length > 0 && (
              <select
                className="game-id-select"
                value={gameId || ""}
                onChange={(event) => {
                  if (event.currentTarget.value) {
                    setGameId(event.currentTarget.value);
                  } else {
                    setGameId(undefined);
                  }
                }}
              >
                {options}
              </select>
            )}
          </div>
          {events.length > 0 && (
            <div className="events-list">
              <span className="title events-title">Game Events</span>
              {events.map((event, index) => (
                <GameEventView key={index} event={event} />
              ))}
            </div>
          )}
        </div>
        {player && room && <PlayerView player={player} actions={actions} />}
        {room && player && (
          <RoomView room={room} actions={actions} player={player} />
        )}
      </>
    );
  };

  return (
    <div className="App">
      <header className="App-header">
        <img src={goblin} className="App-logo" alt="logo" />
        <p>Underworld Server</p>
      </header>
      <div className="body">{renderBody()}</div>
    </div>
  );
};
