import { useEffect, useState } from "react";
import goblin from "./images/goblin_big_hat.svg";
import "./App.css";
import { getUsername, setUsername } from "./api/username";
import { generateGame, getGameIds } from "./api/game";
import { setCurrentGameId } from "./api/current-game";
import { PerformAction, PlayerCharacter, Room } from "./generated-api";
import {
  ActionPerformed,
  getCurrentActions,
  getCurrentRoom,
  listenActionPerformed,
  removeActionPerformedListener,
} from "./api/actions";
import { Action } from "./components/Action";
import { generatePlayer, getCurrentPlayer } from "./api/player";

export const App = () => {
  const [user, setUser] = useState<string | undefined>(getUsername());
  const [gameIds, setGameIds] = useState<Array<string>>([]);
  const [gameId, setGameId] = useState<string | undefined>();
  const [room, setRoom] = useState<Room | undefined>();
  const [actions, setActions] = useState<Array<PerformAction>>([]);
  const [player, setPlayer] = useState<PlayerCharacter | undefined>();

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
    const callback = (actionPerformed: ActionPerformed) => {
      if (actionPerformed.room) {
        setRoom(actionPerformed.room);
      }
      setActions(actionPerformed.actions);

      for (const event of actionPerformed.events) {
        if (event.name === "player_killed") {
          alert("You Died!");
        }
        console.log(event);
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

  gameIds.forEach((id) =>
    options.push(
      <option key={id} value={id}>
        {id}
      </option>,
    ),
  );

  return (
    <div className="App">
      <header className="App-header">
        <img src={goblin} className="App-logo" alt="logo" />
        <p>Underworld Server</p>
      </header>
      <div>
        <div className="basics">
          <input
            value={user || ""}
            onChange={(event) => setUser(event.target.value)}
          />
          <button className="generate-button" onClick={onClickGenerateGame}>
            Generate Game
          </button>

          <button className="generate-button" onClick={onClickGeneratePlayer}>
            Generate Player
          </button>
        </div>
        <button onClick={onClickGetGameIds}>Get Game IDs</button>
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
        {player && room && (
          <div className="room">
            <span className="room-id">{room?.identifier?.id}</span>
            <div className="actions-list">
              {actions.length > 0 &&
                actions.map((action, index) => (
                  <Action
                    key={`action_${index}`}
                    room={room}
                    action={action}
                    player={player}
                  />
                ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
