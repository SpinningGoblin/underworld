import { ReactNode, useEffect, useState } from "react";
import "./App.css";
import { generateGame, getGameIds } from "./api/game";
import { getCurrentGameId, setCurrentGameId } from "./api/current-game";
import { GameEvent, ResponseError } from "./generated-api";
import {
  ActionPerformed,
  getCurrentRoom,
  listenActionPerformed,
  listenError,
  removeActionPerformedListener,
  removeErrorListener,
} from "./api/actions";
import { generatePlayer, getCurrentPlayer } from "./api/player";
import { GameScreen, LoadingScreen, OptionsScreen } from "./screens";
import { Header } from "./components/Header";

import OptionsIcon from "./images/options.svg";
import CloseIcon from "./images/close.svg";
import { useTheme } from "./themes/context";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const App = () => {
  const { theme } = useTheme();
  const queryClient = useQueryClient();
  const [gameId, setGameId] = useState<string | undefined>(getCurrentGameId());
  const { data: room, isLoading: loadingRoom } = useQuery(
    ["room"],
    getCurrentRoom,
    {
      onError: (err) => {
        if (err instanceof ResponseError && err.response.status === 404) {
          setGameId(undefined);
        }
      },
    },
  );

  const { data: player, isLoading: loadingPlayer } = useQuery(
    ["player"],
    getCurrentPlayer,
  );
  const { data: gameIds, isLoading: loadingGameIds } = useQuery(
    ["game-ids"],
    getGameIds,
  );
  const generateGameMutation = useMutation(generateGame, {
    onSuccess: (game) => {
      queryClient.invalidateQueries(["game-ids"]);
      setGameId(game.game_id);
      setEvents([]);
      setLastEvents([]);
    },
  });
  const generatePlayerMutation = useMutation(generatePlayer, {
    onSuccess: (player) => {
      queryClient.setQueryData(["player"], player);
    },
  });
  const [events, setEvents] = useState<Array<GameEvent>>([]);
  const [lastEvents, setLastEvents] = useState<Array<GameEvent>>([]);
  const [showOptions, setShowOptions] = useState<boolean>(false);
  const [error, setError] = useState<string>();

  const onClickGenerateGame = () => generateGameMutation.mutate();
  const onClickGeneratePlayer = () => generatePlayerMutation.mutate();

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
      queryClient.setQueryData(["player"], actionPerformed.currentPlayer);
      queryClient.setQueryData(["room"], actionPerformed.currentRoom);

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
      setCurrentGameId(gameId);
      queryClient.invalidateQueries(["player"]);
      queryClient.invalidateQueries(["room"]);
      console.log(`getting current room ${gameId}`);
    } else {
      setCurrentGameId("");
    }
  }, [gameId]);

  const allowGeneratePlayer =
    !player || player.character.stats.health!.current === 0;

  let body: ReactNode;
  if (loadingGameIds && loadingPlayer && loadingRoom) {
    body = <LoadingScreen />;
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
        gameIds={gameIds ?? []}
        selectedGameId={gameId}
        onGameIdChange={setGameId}
      />
    );
  }

  const headerButton = () => {
    if (!player || !room) {
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
