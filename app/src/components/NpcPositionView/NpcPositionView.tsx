import { FunctionComponent } from "react";
import {
  AttackNpc,
  InspectNpc,
  LootNpc,
  NpcPosition,
  PerformAction,
} from "../../generated-api";
import { AttackNpcView } from "../actions";
import { InspectNpcView } from "../actions/InspectNpcView";
import { NpcInventoryView } from "./NpcInventoryView";

import styles from "./styles.module.css";

export interface NpcPositionViewProps {
  npcPosition: NpcPosition;
  actions: Array<PerformAction>;
}

const nameText = (npcPosition: NpcPosition): string =>
  npcPosition.npc.name ? npcPosition.npc.name : "No name";

const speciesText = (npcPosition: NpcPosition): string => {
  const parts: Array<string> = [];

  if (npcPosition.npc.character.life_modifier) {
    parts.push(npcPosition.npc.character.life_modifier.replaceAll("_", " "));
  }

  parts.push(npcPosition.npc.character.species.replaceAll("_", " "));

  return parts.join(" ");
};

const healthText = (npcPosition: NpcPosition): string => {
  if (!npcPosition.npc.character.stats.health_known) {
    return "Unknown";
  }

  return `${npcPosition.npc.character.stats.health?.current} HP`;
};

const positionText = (npcPosition: NpcPosition): string => {
  if (npcPosition.position_descriptor) {
    switch (npcPosition.position_descriptor) {
      case "in_corner_stands":
        return "They are standing in the corner";
      case "is_crouched_in_the_center_of_room":
        return "They are crouched in the center of the room";
      case "is_crouched_over_chest":
        return "They are crouched over a chest";
      case "is_glaring_at_you":
        return "They are glaring at you";
      case "is_glaring_at_you_from_nearby":
        return "They are glaring at you from nearby";
      case "is_leaning_against_the_table":
        return "They are leaning against a table";
      case "is_leaning_on_a_crate":
        return "They are leaning against a crate";
      case "is_looking_at_the_weapon_rack":
        return "They are gazing at the weapon rack";
      case "is_lying_in_pool_blood":
        return "They are lying a pool of blood";
      case "is_rummaging_through_a_chest":
        return "They are rummaging through a chest";
      case "is_sitting_and_dozing_in_center_of_room":
        return "They are sitting and dozing in the center of the room";
      case "is_sitting_in_a_chair":
        return "They are sitting in a chair";
      case "is_sleeping_in_a_cot":
        return "They are sleeping in a cot";
      case "is_sleeping_in_sleeping_roll":
        return "They are sleeping in a sleeping roll";
      case "is_sleeping_in_the_bed":
        return "They are sleeping in a bed";
      case "is_standing_around":
        return "They are standing around";
      case "is_standing_in_a_barrel":
        return "They are standing inside of a barrel";
      case "is_standing_on_the_table":
        return "They are standing on a table";
      default:
        return "";
    }
  }
  return "";
};

export const NpcPositionView: FunctionComponent<NpcPositionViewProps> = ({
  npcPosition,
  actions,
}) => {
  const inspectArgs: InspectNpc = {
    npc_id: npcPosition.npc.id,
    discover_health: true,
    discover_hidden_items: true,
    discover_packed_items: true,
  };

  const attackArgs: AttackNpc = {
    npc_id: npcPosition.npc.id,
  };

  const hasLootActions = actions.some(
    (action) =>
      action.name === "loot_npc" &&
      (action.args as LootNpc).npc_id === npcPosition.npc.id,
  );

  return (
    <div className={styles["npc-position"]}>
      <div>
        <span className={styles.label}>Name: </span>
        {nameText(npcPosition)}
      </div>
      <div>
        <span className={styles.label}>Species: </span>
        {speciesText(npcPosition)}
      </div>
      <div>{positionText(npcPosition)}</div>
      <div>
        <span className={styles.label}>Health: </span>
        {healthText(npcPosition)}
      </div>
      <div className={styles["basic-actions"]}>
        <InspectNpcView args={inspectArgs} />
        <AttackNpcView args={attackArgs} />
      </div>
      <div className={styles.inventory}>
        {!npcPosition.npc.character.inventory_known && "You do not know their inventory."}
        {npcPosition.npc.character.inventory_known && npcPosition.npc.character.inventory && (
          <NpcInventoryView
            inventory={npcPosition.npc.character.inventory}
            canLoot={hasLootActions}
            npcId={npcPosition.npc.id}
          />
        )}
      </div>
    </div>
  );
};
