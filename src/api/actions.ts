import {
  AttackNpc,
  CastSpellOnPlayer,
  ExitRoom,
  GameEvent,
  InspectFixture,
  InspectNpc,
  LootFixture,
  LootNpc,
  MovePlayerItem,
  PerformAction,
  Room,
} from "../generated-api";
import { getCurrentGameId } from "./current-game";
import { getGameActionsApi } from "./factory";
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

  notifyListeners({
    actions,
    events,
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

  notifyListeners({
    actions,
    events,
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

  const events: Array<GameEvent> = [];

  if (response.can_be_opened_discovered) {
    events.push({
      name: "fixture_can_be_opened_discovered",
    });
  }
  if (response.contained_items_discovered) {
    events.push({
      name: "fixture_contained_discovered",
    });
  }
  if (response.has_hidden_discovered) {
    events.push({
      name: "fixture_has_hidden_discovered",
    });
  }
  if (response.hidden_items_discovered) {
    events.push({
      name: "fixture_hidden_items_discovered",
    });
  }

  notifyListeners({
    actions: response.actions,
    events,
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

  const events: Array<GameEvent> = [];
  if (response.health_discovered) {
    events.push({
      name: "npc_health_discovered",
    });
  }
  if (response.hidden_items_discovered) {
    events.push({
      name: "npc_hidden_discovered",
    });
  }
  if (response.packed_items_discovered) {
    events.push({
      name: "npc_packed_discovered",
    });
  }
  if (response.name_discovered) {
    events.push({
      name: "npc_name_discovered",
    });
  }

  notifyListeners({
    actions: response.actions,
    events,
  });
};

export const performLootNpc = async (
  args: LootNpc,
): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const { actions, events } = await api.lootNpc({
    underworldUsername: username,
    gameId,
    lootNpc: args,
  });

  notifyListeners({
    actions,
    events,
  });
};

export const performLootFixture = async (
  args: LootFixture,
): Promise<void> => {
  const { username, gameId } = getBasicParams();
  const api = getGameActionsApi();
  const { actions, events } = await api.lootFixture({
    underworldUsername: username,
    gameId,
    lootFixture: args,
  });

  notifyListeners({
    actions,
    events,
  });
};
