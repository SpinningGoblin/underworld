import styles from "./Action.module.css";

import { FunctionComponent, ReactElement } from "react";
import {
  AttackNpc,
  InspectFixture,
  LookAtFixture,
  LookAtNpc,
  MovePlayerItem,
  PerformAction,
  PlayerCharacter,
  Room,
} from "../generated-api";
import { ExitRoomView } from "./actions/ExitRoomView";

export interface ActionProps {
  room: Room;
  action: PerformAction;
  player: PlayerCharacter;
}

const renderAttackNpc = (room: Room, action: PerformAction): ReactElement => {
  const args = action.args! as AttackNpc;

  const npc = room.npc_positions.find(
    (npcPosition) => npcPosition.npc.identifier.id === args.npc_id,
  )?.npc;

  return (
    <div>
      <p>Attack {npc?.character?.species}</p>
    </div>
  );
};

const renderMoveItem = (
  action: PerformAction,
  player: PlayerCharacter,
): ReactElement => {
  const args = action.args as MovePlayerItem;
  const item = player.character.inventory!.equipment.find(
    (characterItem) => characterItem.item.identifier.id === args.item_id,
  );

  if (!item) {
    return <></>;
  }

  const equipText = args.put_at_the_ready ? "Equip" : "Unequip";

  return (
    <div>
      {equipText} {item.item.item_type} to {args.location_tag}
    </div>
  );
};

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
      child = renderAttackNpc(room, action);
      break;
    case "move_player_item":
      child = renderMoveItem(action, player);
      break;
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
    default:
      child = <div>{name}</div>;
      break;
  }

  return <div className={styles.action}>{child}</div>;
};
