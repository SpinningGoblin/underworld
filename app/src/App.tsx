import { useEffect, useRef, useState } from "react";
import goblin from "./images/goblin_big_hat.svg";
import "./App.css";
import { getUsername, setUsername } from "./api/username";
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
  removeActionPerformedListener,
} from "./api/actions";
import { generatePlayer, getCurrentPlayer } from "./api/player";
import { GameEventView } from "./components/GameEventView";
import { RoomView } from "./components/RoomView";
import { PlayerView } from "./components/PlayerView";
import { NpcPositionView } from "./components/NpcPositionView";
import { FixturePositionView } from "./components/FixturePositionView";

export const App = () => {
  const [user, setUser] = useState<string | undefined>(getUsername());
  const [gameIds, setGameIds] = useState<Array<string>>([]);
  const [gameId, setGameId] = useState<string | undefined>();
  const [room, setRoom] = useState<Room | undefined>();
  const [actions, setActions] = useState<Array<PerformAction>>([]);
  const [player, setPlayer] = useState<PlayerCharacter | undefined>();
  const [events, setEvents] = useState<Array<GameEvent>>([]);
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
    const username = getUsername();
    if (username && !firstPlayerLoadDone.current) {
      getCurrentPlayer()
        .then(setPlayer)
        .finally(() => (firstPlayerLoadDone.current = true));
    }
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

      setEvents((existing) => [
        ...actionPerformed.events,
        ...existing,
      ]);

      for (const event of actionPerformed.events) {
        if (event.name === "player_killed") {
          alert("You Died!");
        }
      }
    };
    listenActionPerformed(callback);

    return () => removeActionPerformedListener(callback);
  });

  useEffect(() => {
    if (user) {
      setUsername(user);
    }
  }, [user]);

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

  const username = getUsername();

  const npcText = (room: Room): string => {
    if (!room.npc_positions.length) {
      return "There are no other creatures in the room.";
    }

    const creatureText =
      room.npc_positions.length === 1
        ? "is 1 creature"
        : `are ${room.npc_positions.length} creatures`;

    return `There ${creatureText} in the room with you.`;
  };

  const fixtureText = (room: Room): string => {
    if (!room.fixture_positions.length) {
      return "There is nothing else interesting in the room.";
    }

    const itemText =
      room.fixture_positions.length === 1
        ? "is 1 item"
        : `are ${room.fixture_positions.length} items`;

    return `There ${itemText} in the room with you.`;
  };

  return (
    <div className="App">
      <header className="App-header">
        <img src={goblin} className="App-logo" alt="logo" />
        <p>Underworld Server</p>
      </header>
      <div className="body">
        <div className="basics">
          <input
            className="name-input"
            value={user || ""}
            onChange={(event) => setUser(event.target.value)}
          />
          {!!username && (
            <button className="generate-button" onClick={onClickGeneratePlayer}>
              Generate Player
            </button>
          )}
          {player && (
            <button className="generate-button" onClick={onClickGenerateGame}>
              Generate Game
            </button>
          )}
          {player && (
            <button className="generate-button" onClick={onClickGetGameIds}>
              Get Game IDs
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
        </div>
        {events.length > 0 && (
          <div className="events-list">
            {events.map((event, index) => (
              <GameEventView key={index} event={event} />
            ))}
          </div>
        )}
        <div className="room-and-player">
          {room && <RoomView room={room} actions={actions} />}
          {player && <PlayerView player={player} actions={actions} />}
        </div>
        {player && room && (
          <div className="npcs">
            <div className="npc-text">{npcText(room)}</div>
            <div className="npc-list">
              {room.npc_positions.map((npcPosition) => (
                <NpcPositionView
                  key={npcPosition.npc.id}
                  npcPosition={npcPosition}
                  actions={actions}
                />
              ))}
            </div>
          </div>
        )}
        {player && room && (
          <div className="fixtures">
            <div className="fixture-text">{fixtureText(room)}</div>
            <div className="fixture-list">
              {room.fixture_positions.map((fixturePosition) => (
                <FixturePositionView
                  key={fixturePosition.fixture.id}
                  fixturePosition={fixturePosition}
                />
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
