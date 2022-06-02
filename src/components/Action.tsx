import styles from "./Action.module.css";

import { FunctionComponent, ReactElement } from "react";
import {
  InspectFixture,
  LookAtFixture,
  LookAtNpc,
  PerformAction,
  PlayerCharacter,
  Room,
} from "../generated-api";
import {
  AttackNpcView,
  CastSpellOnPlayerView,
  ExitRoomView,
  MovePlayerItemView,
} from "./actions";

export interface ActionProps {
  room: Room;
  action: PerformAction;
  player: PlayerCharacter;
}

const renderInspectFixture = (
  action: PerformAction,
  room: Room,
): ReactElement => {
  const args = action.args as InspectFixture;
  const fixture = room.fixture_positions
    .flatMap((fp) => fp.fixtures)
    .find((f) => f.identifier.id === args.fixture_id);

  if (!fixture) {
    return <></>;
  }

  return <div>Inspect {fixture.fixture_type}</div>;
};

const renderLookAtFixture = (
  action: PerformAction,
  room: Room,
): ReactElement => {
  const args = action.args as LookAtFixture;
  const fixture = room.fixture_positions
    .flatMap((fp) => fp.fixtures)
    .find((f) => f.identifier.id === args.fixture_id);

  if (!fixture) {
    return <></>;
  }

  return <div>Look at {fixture.fixture_type}</div>;
};

const renderLookAtNpc = (action: PerformAction, room: Room): ReactElement => {
  const args = action.args as LookAtNpc;

  const npc = room.npc_positions
    .map((npcPosition) => npcPosition.npc)
    .find((npc) => npc.identifier.id === args.npc_id);

  if (!npc) {
    return <></>;
  }

  return <div>Look at {npc.character.species}</div>;
};

export const Action: FunctionComponent<ActionProps> = ({
  room,
  action,
  player,
}) => {
  const name = action.name!;

  let child: ReactElement;

  switch (name) {
    case "attack_npc":
      return <AttackNpcView args={action.args!} room={room} />;
    case "move_player_item":
      return <MovePlayerItemView args={action.args!} player={player} />;
    case "exit_room":
      return <ExitRoomView args={action.args!} room={room} />;
    case "inspect_fixture":
      child = renderInspectFixture(action, room);
      break;
    case "look_at_fixture":
      child = renderLookAtFixture(action, room);
      break;
    case "look_at_npc":
      child = renderLookAtNpc(action, room);
      break;
    case "cast_spell_on_player":
      return <CastSpellOnPlayerView args={action.args!} player={player} />;
    default:
      child = <div>{name}</div>;
      break;
  }

  return <div className={styles.action}>{child}</div>;
};
