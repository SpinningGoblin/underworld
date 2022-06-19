import { FunctionComponent } from "react";
import { EventName, GameEvent } from "../../generated-api";

import styles from "./GameEventView.module.css";

export interface GameEventViewProps {
  event: GameEvent;
}

const nameText = (eventName: EventName): string => {
  switch (eventName) {
    case "dead_npc_beaten":
      return "You beat a dead NPC. Why?";
    case "fixture_can_be_opened_discovered":
      return "You found out if the fixture can be opened";
    case "fixture_contained_discovered":
      return "You found out what the fixture contains";
    case "fixture_has_hidden_discovered":
      return "You found out if the fixture has hidden items";
    case "fixture_hidden_items_discovered":
      return "You found out the hidden items in fixture";
    case "item_taken_from_fixture":
      return "You took an item from the fixture";
    case "item_taken_from_npc":
      return "You took an item from an NPC";
    case "npc_health_discovered":
      return "You discovered the NPCs health";
    case "npc_hidden_discovered":
      return "You found out if the NPC has hidden anything";
    case "npc_missed":
      return "You missed the NPC. Too bad.";
    case "npc_packed_discovered":
      return "You discovered what the NPC is carrying";
    case "npc_weapon_readied":
      return "The NPC readied its weapon";
    case "player_gains_resurrection_aura":
      return "You gained a resurrection aura";
    case "player_gains_retribution_aura":
      return "You gained a retribution aura";
    case "player_gains_shield_aura":
      return "You gained a shield aura";
    case "player_healed":
      return "You were healed";
    case "player_hit":
      return "An NPC hit you";
    case "player_hit_npc":
      return "You hit an NPC";
    case "player_item_moved":
      return "You moved an item around";
    case "player_item_removed":
      return "You removed an item";
    case "player_item_used":
      return "You used an item";
    case "player_killed":
      return "You were killed!";
    case "player_killed_npc":
      return "You killed an NPC. You monster!";
    case "player_missed":
      return "The NPC missed you. Lucky";
    case "player_resurrected":
      return "You were resurrected";
    case "player_retribution_aura_dissipated":
      return "You lost your retribution aura";
    case "player_spell_forgotten":
      return "You forgot a spell";
    case "player_spell_learned":
      return "You learned a new spell";
    case "player_spell_used":
      return "You used a spell";
    case "room_exited":
      return "You exited the room";
    case "room_first_seen":
      return "You saw a room for the first time";
    case "room_generated":
      return "A new room was spawned from the void";
    case "game_danger_level_increased":
      return "The world gets more dangerous...";
    default:
      return eventName;
  }
};

export const GameEventView: FunctionComponent<GameEventViewProps> = ({
  event,
}) => {
  const onClick = () => {
    if (event.data) {
      alert(JSON.stringify(event.data, null, 2));
    }
  };

  return (
    <button onClick={onClick} className={styles.event}>
      {nameText(event.name)}
    </button>
  );
};
