import {
  AttackNpc,
  CastSpellOnNpc,
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
  OpenFixture,
  OpenFixtureHiddenCompartment,
  PerformAction,
  ResponseError,
  Room,
  SellPlayerItem,
  ThrowItemAtNpc,
  UseItemOnPlayer,
} from "../generated-api";
import { getCurrentGameId } from "./current-game";
import { getGameActionsApi } from "./factory";

interface BasicParams {
  gameId: string;
}

export interface ActionPerformed {
  events: Array<GameEvent>;
}

const getBasicParams = (): BasicParams => {
  const gameId = getCurrentGameId();

  if (!gameId) {
    throw new Error("Missing required parameters");
  }

  return {
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

export type ErrorCallback = (error: string) => void;

const errorListeners: Array<ErrorCallback> = [];

export const listenError = (callback: ErrorCallback) => {
  if (!errorListeners.includes(callback)) {
    errorListeners.push(callback);
  }
};

export const removeErrorListener = (callback: ErrorCallback) => {
  const index = errorListeners.indexOf(callback);

  if (index >= 0) {
    errorListeners.splice(index, 1);
  }
};

const notifyListeners = (actionPerformed: ActionPerformed) => {
  for (const listener of listeners) {
    listener(actionPerformed);
  }
};

const notifyError = (error: string) => {
  for (const listener of errorListeners) {
    listener(error);
  }
};

export const getCurrentRoom = async (): Promise<Room> => {
  const { gameId } = getBasicParams();

  const api = getGameActionsApi();

  return api.lookAroundRoom({ gameId });
};

export const getCurrentActions = async (): Promise<Array<PerformAction>> => {
  const { gameId } = getBasicParams();

  const api = getGameActionsApi();
  return api.currentActions({ gameId });
};

export const performExitRoom = async (args: ExitRoom): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();

    const { events } = await api.exitRoom({
      gameId,
      exitRoom: args,
    });

    const actionPerformed: ActionPerformed = {
      events,
    };

    notifyListeners(actionPerformed);
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performThrowItemAtNpc = async (
  args: ThrowItemAtNpc,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const { events } = await api.throwItemAtNpc({
      gameId,
      throwItemAtNpc: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performAttackNpc = async (args: AttackNpc): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const { events } = await api.attackNpc({
      gameId,
      attackNpc: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performCastSpellOnNpc = async (
  args: CastSpellOnNpc,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const { events } = await api.castSpellOnNpc({
      gameId,
      castSpellOnNpc: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performCastSpellOnPlayer = async (
  args: CastSpellOnPlayer,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const { events } = await api.castSpellOnPlayer({
      gameId,
      castSpellOnPlayer: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performUseItemOnPlayer = async (
  args: UseItemOnPlayer,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const { events } = await api.useItemOnPlayer({
      gameId,
      useItemOnPlayer: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performOpenFixture = async (args: OpenFixture): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const response = await api.openFixture({
      gameId,
      openFixture: args,
    });

    notifyListeners({
      events: response.events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performOpenFixtureHiddenCompartment = async (
  args: OpenFixtureHiddenCompartment,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const response = await api.openFixtureHiddenCompartment({
      gameId,
      openFixtureHiddenCompartment: args,
    });

    notifyListeners({
      events: response.events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performInspectFixture = async (
  args: InspectFixture,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const response = await api.inspectFixture({
      gameId,
      inspectFixture: args,
    });

    notifyListeners({
      events: response.events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performMovePlayerItem = async (
  args: MovePlayerItem,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();

    const { events } = await api.movePlayerItem({
      gameId,
      movePlayerItem: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      console.log("getting response");
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performSellPlayerItem = async (
  args: SellPlayerItem,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();

    const { events } = await api.sellPlayerItem({
      gameId,
      sellPlayerItem: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      console.log("getting response");
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performInspectNpc = async (args: InspectNpc): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();

    const response = await api.inspectNpc({
      gameId,
      inspectNpc: args,
    });

    notifyListeners({
      events: response.events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performLootNpc = async (args: LootNpc): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const { events } = await api.lootNpc({
      gameId,
      lootNpc: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performLootFixture = async (args: LootFixture): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();
    const { events } = await api.lootFixture({
      gameId,
      lootFixture: args,
    });

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performLookAtNpc = async (args: LookAtNpc): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();

    const response = await api.lookAtNpc({
      gameId,
      lookAtNpc: args,
    });

    const events: Array<GameEvent> = [
      {
        name: "npc_viewed",
        data: response,
      },
    ];

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};

export const performLookAtFixture = async (
  args: LookAtFixture,
): Promise<void> => {
  try {
    const { gameId } = getBasicParams();
    const api = getGameActionsApi();

    const response = await api.lookAtFixture({
      gameId,
      lookAtFixture: args,
    });

    const events: Array<GameEvent> = [
      {
        name: "fixture_viewed",
        data: response,
      },
    ];

    notifyListeners({
      events,
    });
  } catch (e) {
    if (typeof e === "string") {
      notifyError(e);
    } else if (e instanceof ResponseError) {
      const message = await e.response.text();
      notifyError(message);
    }
    throw e;
  }
};
