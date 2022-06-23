import { FunctionComponent } from "react";
import {
  Attack,
  CharacterItem,
  Consumable,
  Defense,
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

  const renderAttack = (attack: Attack) => (
    <div className={styles.attack}>
      <span>{`${attack.num_rolls}d6 ${
        attack.modifier !== 0 ? `+${attack.modifier}` : ""
      }`}</span>
    </div>
  );

  const renderDefense = (defense: Defense) => (
    <div className={styles.defense}>
      <span>{`${defense.damage_resistance} damage resistance`}</span>
    </div>
  );

  return (
    <div className={[styles.item, "action-card"].join(" ")}>
      <div className={styles["item-name"]}>
        {[
          ...characterItem.item.descriptors.map(descriptorText),
          characterItem.item.material ? characterItem.item.material : "",
          itemTypeText(characterItem.item.item_type),
        ].join(" ")}
      </div>
      {characterItem.item.attack && renderAttack(characterItem.item.attack)}
      {characterItem.item.defense && renderDefense(characterItem.item.defense)}
      {equipActions.length > 0 && (
        <div className={styles.equip}>
          <span>Equip to </span>
          <div className={styles.actions}>
            {equipActions.map((action, index) => (
              <MovePlayerItemView
                key={index}
                args={action.args! as MovePlayerItem}
              />
            ))}
          </div>
        </div>
      )}
      {unequipActions.length > 0 && (
        <div className={styles.equip}>
          <span>Unequip to </span>
          <div className={styles.actions}>
            {unequipActions.map((action, index) => (
              <MovePlayerItemView
                key={index}
                args={action.args! as MovePlayerItem}
              />
            ))}
          </div>
        </div>
      )}
      {useActions.length > 0 && characterItem.item.consumable && (
        <div className={styles.equip}>
          <span>{effectText(characterItem.item.consumable)}</span>
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
> = ({ inventory, actions }) => {
  const equippedItems = inventory.equipment.filter((c) => c.at_the_ready);
  const unequippedItems = inventory.equipment.filter((c) => !c.at_the_ready);

  return (
    <div className={styles.inventory}>
      <span className="title">Inventory</span>
      <div className={styles.equipment}>
        <div className={styles["item-group"]}>
          <span className="title">Equipped Items</span>
          <div className={styles["item-list"]}>
            {equippedItems.length > 0 &&
              equippedItems.map((characterItem) => {
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
        <div className={styles["item-group"]}>
          <span className="title">Unequipped Items</span>
          <div className={styles["item-list"]}>
            {unequippedItems.length > 0 &&
              unequippedItems.map((characterItem) => {
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
      </div>
    </div>
  );
};
