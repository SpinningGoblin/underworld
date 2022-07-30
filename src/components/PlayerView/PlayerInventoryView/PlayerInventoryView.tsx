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
import { useTheme } from "../../../themes/context";
import { MovePlayerItemView } from "../../actions";
import { SellPlayerItemView } from "../../actions/SellPlayerItemView";
import { UseItemOnPlayerView } from "../../actions/UseItemOnPlayerView";
import { Card } from "../../Card";

import styles from "./PlayerInventoryView.module.css";

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
    <div>
      <span>{`${attack.num_rolls}d6 ${
        attack.modifier !== 0 ? `+${attack.modifier}` : ""
      }`}</span>
      {attack.effects.length > 0 && <span>{attack.effects.join(", ")}</span>}
    </div>
  );

  const renderDefense = (defense: Defense) => (
    <span>{`${defense.damage_resistance} damage resistance`}</span>
  );

  return (
    <Card className={styles.item}>
      <div className={styles["item-name"]}>
        {[
          ...characterItem.item.descriptors.map(descriptorText),
          characterItem.item.material ? characterItem.item.material : "",
          itemTypeText(characterItem.item.item_type),
        ].join(" ")}
      </div>
      {characterItem.item.attack && renderAttack(characterItem.item.attack)}
      {characterItem.item.defense && renderDefense(characterItem.item.defense)}
      <div className={styles.actions}>
        {equipActions.length > 0 && (
          <>
            <span>Equip to </span>
            <div className={styles.actions}>
              {equipActions.map((action, index) => (
                <MovePlayerItemView
                  key={index}
                  args={action.args! as MovePlayerItem}
                />
              ))}
            </div>
          </>
        )}
        {unequipActions.length > 0 && (
          <>
            <span>Unequip to </span>
            <div className={styles.actions}>
              {unequipActions.map((action, index) => (
                <MovePlayerItemView
                  key={index}
                  args={action.args! as MovePlayerItem}
                />
              ))}
            </div>
          </>
        )}
        {useActions.length > 0 && characterItem.item.consumable && (
          <>
            <span>{effectText(characterItem.item.consumable)}</span>
            <div className={styles.actions}>
              {useActions.map((action, index) => (
                <UseItemOnPlayerView key={index} args={action.args} />
              ))}
            </div>
          </>
        )}
        <SellPlayerItemView itemId={characterItem.item.id} />
      </div>
    </Card>
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
  const { theme } = useTheme();

  return (
    <div className={styles.inventory} style={{ color: theme.colors.secondary }}>
      <h2>Inventory</h2>
      <div className={styles.equipment}>
        <div className={styles["item-group"]}>
          <h3>Equipped Items</h3>
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
          <h3>Unequipped Items</h3>
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
