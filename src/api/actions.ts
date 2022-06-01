import {
  ExitRoom,
  GameActionsApi,
  GameEvent,
  PerformAction,
  Room,
} from "../generated-api";
import { getCurrentGameId } from "./current-game";
import { getUsername } from "./username";

interface BasicParams {
  username: string;
  gameId: string;
}

export interface ActionPerformed {
  actions: Array<PerformAction>;
  events: Array<GameEvent>;
  room?: Room;
}

const getBasicParams = (): BasicParams => {
  const gameId = getCurrentGameId();
  const username = getUsername();

  if (!gameId || !username) {
    throw new Error("Missing required parameters");
  }

  return {
    username,
    gameId,
  };
};

export type ActionPerformedCallback = (event: ActionPerformed) => void;

const listeners: Array<ActionPerformedCallback> = [];

export const listenActionPerformed = (callback: ActionPerformedCallback) => {
  if (!listeners.includes(callback)) {
    listeners.push(callback);
  }
};

export const removeActionPerformedListener = (
  callback: ActionPerformedCallback,
) => {
  const index = listeners.indexOf(callback);

  if (index >= 0) {
    listeners.splice(index, 1);
  }
};

const notifyListeners = (actionPerformed: ActionPerformed) => {
  for (const listener of listeners) {
    listener(actionPerformed);
  }
};

export const getCurrentRoom = async (): Promise<Room> => {
  const { gameId, username } = getBasicParams();

  const api = new GameActionsApi();

  return api.lookAroundRoom({ underworldUsername: username, gameId });
};

export const getCurrentActions = async (): Promise<Array<PerformAction>> => {
  const { username, gameId } = getBasicParams();

  const api = new GameActionsApi();
  return api.currentActions({ underworldUsername: username, gameId });
};

export const performExitRoom = async (args: ExitRoom): Promise<void> => {
  const { username, gameId } = getBasicParams();

  const api = new GameActionsApi();

  const { actions, events } = await api.exitRoom({
    underworldUsername: username,
    gameId,
    exitRoom: args,
  });

  const room = await getCurrentRoom();

  const actionPerformed: ActionPerformed = {
    actions,
    events,
    room,
  };

  notifyListeners(actionPerformed);
};
