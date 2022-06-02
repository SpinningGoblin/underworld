import styles from "./Action.module.css";

import { FunctionComponent, ReactElement } from "react";
import {
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
import { LootNpcView } from "./actions/LootNpcView";
import { InspectFixtureView } from "./actions/InspectFixtureView";
import { InspectNpcView } from "./actions/InspectNpcView";
import { UseItemOnPlayerView } from "./actions/UseItemOnPlayerView";
import { LootFixtureView } from "./actions/LootFixtureView";
import { LookAtFixtureView } from "./actions/LookAtFixtureView";
import { LookAtNpcView } from "./actions/LookAtNpcView";

export interface ActionProps {
  room: Room;
  action: PerformAction;
  player: PlayerCharacter;
}

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
      return <InspectFixtureView args={action.args!} room={room} />;
    case "look_at_fixture":
      return <LookAtFixtureView args={action.args!} room={room} />;
    case "look_at_npc":
      return <LookAtNpcView args={action.args!} room={room} />;
    case "cast_spell_on_player":
      return <CastSpellOnPlayerView args={action.args!} player={player} />;
    case "loot_npc":
      return <LootNpcView args={action.args!} room={room} />;
    case "inspect_npc":
      return <InspectNpcView args={action.args!} room={room} />;
    case "use_item_on_player":
      return <UseItemOnPlayerView args={action.args!} player={player} />;
    case "loot_fixture":
      return <LootFixtureView args={action.args!} room={room} />;
    default:
      child = <div>{name}</div>;
      break;
  }

  return <div className={styles.action}>{child}</div>;
};
