import {
  AttackNpc,
  CastSpellOnPlayer,
  ExitRoom,
  GameEvent,
  InspectFixture,
  InspectNpc,
  LookAtFixture,
  LookAtNpc,
  LootFixture,
  LootNpc,
  MovePlayerItem,
  PerformAction,
  PlayerCharacter,
  Room,
  UseItemOnPlayer,
} from "../generated-api";
import { getCurrentGameId } from "./current-game";
import { getGameActionsApi, getPlayerApi } from "./factory";
import { getUsername } from "./username";

interface BasicParams {
  username: string;
  gameId: string;
}

export interface ActionPerformed {
  actions: Array<PerformAction>;
  events: Array<GameEvent>;
  room?: Room;
  player?: PlayerCharacter;
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

  const api = getGameActionsApi();

  return api.lookAroundRoom({ underworldUsername: username, gameId });
};

export const getCurrentActions = async (): Promise<Array<PerformAction>> => {
  const { username, gameId } = getBasicParams();

  const api = getGameActionsApi();
  return api.currentActions({ underworldUsername: username, gameId });
};

export const performExitRoom = async (args: ExitRoom): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
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

export const performAttackNpc = async (args: AttackNpc): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const { actions, events } = await api.attackNpc({
    underworldUsername: username,
    gameId,
    attackNpc: args,
  });

  const playerApi = getPlayerApi();
  const [room, player] = await Promise.all([
    getCurrentRoom(),
    playerApi.getCurrentPc({ underworldUsername: username }),
  ]);

  notifyListeners({
    actions,
    events,
    player,
    room,
  });
};

export const performCastSpellOnPlayer = async (
  args: CastSpellOnPlayer,
): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const { actions, events } = await api.castSpellOnPlayer({
    underworldUsername: username,
    gameId,
    castSpellOnPlayer: args,
  });

  const playerApi = getPlayerApi();
  const player = await playerApi.getCurrentPc({ underworldUsername: username });

  notifyListeners({
    actions,
    events,
    player,
  });
};

export const performUseItemOnPlayer = async (
  args: UseItemOnPlayer,
): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const { actions, events } = await api.useItemOnPlayer({
    underworldUsername: username,
    gameId,
    useItemOnPlayer: args,
  });

  const playerApi = getPlayerApi();
  const player = await playerApi.getCurrentPc({ underworldUsername: username });

  notifyListeners({
    actions,
    events,
    player,
  });
};

export const performInspectFixture = async (
  args: InspectFixture,
): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const response = await api.inspectFixture({
    underworldUsername: username,
    gameId,
    inspectFixture: args,
  });

  const room = await getCurrentRoom();

  notifyListeners({
    actions: response.actions,
    events: response.events,
    room,
  });
};

export const performMovePlayerItem = async (
  args: MovePlayerItem,
): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();

  const { actions, events } = await api.movePlayerItem({
    underworldUsername: username,
    gameId,
    movePlayerItem: args,
  });

  notifyListeners({
    actions,
    events,
  });
};

export const performInspectNpc = async (args: InspectNpc): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();

  const response = await api.inspectNpc({
    underworldUsername: username,
    gameId,
    inspectNpc: args,
  });

  const playerApi = getPlayerApi();
  const [room, player] = await Promise.all([
    getCurrentRoom(),
    playerApi.getCurrentPc({ underworldUsername: username }),
  ]);

  notifyListeners({
    actions: response.actions,
    events: response.events,
    room,
    player,
  });
};

export const performLootNpc = async (args: LootNpc): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const { actions, events } = await api.lootNpc({
    underworldUsername: username,
    gameId,
    lootNpc: args,
  });

  const playerApi = getPlayerApi();

  const [room, player] = await Promise.all([
    getCurrentRoom(),
    playerApi.getCurrentPc({ underworldUsername: username }),
  ]);

  notifyListeners({
    actions,
    events,
    player,
    room,
  });
};

export const performLootFixture = async (args: LootFixture): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const { actions, events } = await api.lootFixture({
    underworldUsername: username,
    gameId,
    lootFixture: args,
  });

  const playerApi = getPlayerApi();
  const [room, player] = await Promise.all([
    getCurrentRoom(),
    playerApi.getCurrentPc({ underworldUsername: username }),
  ]);

  notifyListeners({
    actions,
    events,
    player,
    room,
  });
};

export const performLookAtNpc = async (args: LookAtNpc): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();

  const response = await api.lookAtNpc({
    underworldUsername: username,
    gameId,
    lookAtNpc: args,
  });

  const actions = await getCurrentActions();
  const events: Array<GameEvent> = [
    {
      name: "npc_viewed",
      data: response,
    },
  ];

  notifyListeners({
    actions,
    events,
  });
};

export const performLookAtFixture = async (
  args: LookAtFixture,
): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();

  const response = await api.lookAtFixture({
    underworldUsername: username,
    gameId,
    lookAtFixture: args,
  });

  const actions = await getCurrentActions();
  const events: Array<GameEvent> = [
    {
      name: "fixture_viewed",
      data: response,
    },
  ];

  notifyListeners({
    actions,
    events,
  });
};
