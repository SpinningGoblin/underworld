import { FunctionComponent } from "react";
import {
  CharacterItem,
  Consumable,
  Inventory,
  ItemDescriptor,
  ItemType,
  MovePlayerItem,
  PerformAction,
  UseItemOnPlayer,
} from "../../../generated-api";
import { MovePlayerItemView } from "../../actions";
import { UseItemOnPlayerView } from "../../actions/UseItemOnPlayerView";

import styles from "./styles.module.css";

export interface PlayerInventoryViewProps {
  inventory: Inventory;
  actions: Array<PerformAction>;
}

interface CharacterItemViewProps {
  characterItem: CharacterItem;
  itemActions: Array<PerformAction>;
}

const itemTypeText = (itemType: ItemType): string => itemType.replace("_", " ");

const descriptorText = (descriptor: ItemDescriptor): string =>
  descriptor.replace("_", " ");

const effectText = (consumable: Consumable): string => {
  const parts: Array<string> = [];

  switch (consumable.effect.name) {
    case "learn_spell":
      parts.push(
        `learn spell ${consumable.effect.learn_spell_effect!.spell_name.replaceAll(
          "_",
          " ",
        )}`,
      );
  }

  parts.push(`(${consumable.uses} uses remaining)`);

  return parts.join(" ");
};

const CharacterItemView: FunctionComponent<CharacterItemViewProps> = ({
  characterItem,
  itemActions,
}) => {
  const equipActions = itemActions.filter(
    (action) =>
      action.name === "move_player_item" &&
      (action.args! as MovePlayerItem).put_at_the_ready,
  );

  const unequipActions = itemActions.filter(
    (action) =>
      action.name === "move_player_item" &&
      !(action.args! as MovePlayerItem).put_at_the_ready,
  );

  const useActions = itemActions.filter(
    (action) => action.name === "use_item_on_player",
  );

  return (
    <div className={styles.item}>
      <div className={styles["item-name"]}>
        {[
          ...characterItem.item.descriptors.map(descriptorText),
          itemTypeText(characterItem.item.item_type),
        ].join(" ")}
      </div>
      {equipActions.length > 0 && (
        <div className={styles.equip}>
          <span>Equip to </span>
          {equipActions.map((action, index) => (
            <MovePlayerItemView
              key={index}
              args={action.args! as MovePlayerItem}
            />
          ))}
        </div>
      )}
      {unequipActions.length > 0 && (
        <div className={styles.equip}>
          <span>Unequip to </span>
          {unequipActions.map((action, index) => (
            <MovePlayerItemView
              key={index}
              args={action.args! as MovePlayerItem}
            />
          ))}
        </div>
      )}
      {useActions.length > 0 && characterItem.item.consumable && (
        <div className={styles.equip}>
          <span>
            {effectText(characterItem.item.consumable)}
          </span>
          {useActions.map((action, index) => (
            <UseItemOnPlayerView key={index} args={action.args} />
          ))}
        </div>
      )}
    </div>
  );
};

const actionForItem = (
  action: PerformAction,
  characterItem: CharacterItem,
): boolean => {
  if (
    action.name === "move_player_item" &&
    (action.args! as MovePlayerItem).item_id === characterItem.item.id
  ) {
    return true;
  }

  if (
    action.name === "use_item_on_player" &&
    (action.args! as UseItemOnPlayer).item_id === characterItem.item.id
  ) {
    return true;
  }

  return false;
};

export const PlayerInventoryView: FunctionComponent<
  PlayerInventoryViewProps
> = ({ inventory, actions }) => (
  <div className={styles.inventory}>
    <span className={styles.title}>Inventory</span>
    <div className={styles.equipment}>
      {inventory.equipment.length > 0 &&
        inventory.equipment.map((characterItem) => {
          const itemActions = actions.filter((action) =>
            actionForItem(action, characterItem),
          );

          return (
            <CharacterItemView
              key={characterItem.item.id}
              itemActions={itemActions}
              characterItem={characterItem}
            />
          );
        })}
    </div>
  </div>
);
