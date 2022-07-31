import { FunctionComponent } from "react";
import {
  Attack,
  CharacterItem,
  Consumable,
  Defense,
  Inventory,
  ItemDescriptor,
  ItemType,
} from "../../../generated-api";
import { useTheme } from "../../../themes/context";
import { MovePlayerItemView } from "../../actions";
import { SellPlayerItemView } from "../../actions/SellPlayerItemView";
import { UseItemOnPlayerView } from "../../actions/UseItemOnPlayerView";
import { Card } from "../../Card";

import styles from "./PlayerInventoryView.module.css";

export interface PlayerInventoryViewProps {
  inventory: Inventory;
}

interface CharacterItemViewProps {
  characterItem: CharacterItem;
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
      break;
    case "healing_grog":
      parts.push("healing grog");
      break;
  }

  parts.push(`(${consumable.uses} uses remaining)`);

  return parts.join(" ");
};

const CharacterItemView: FunctionComponent<CharacterItemViewProps> = ({
  characterItem,
}) => {
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
        {characterItem.at_the_ready && (
          <MovePlayerItemView itemId={characterItem.item.id} equip={false} />
        )}
        {!characterItem.at_the_ready && characterItem.item.is_equippable && (
          <MovePlayerItemView itemId={characterItem.item.id} equip />
        )}
        {characterItem.item.consumable && (
          <>
            <span>{effectText(characterItem.item.consumable)}</span>
            <div className={styles.actions}>
              <UseItemOnPlayerView itemId={characterItem.item.id} />
            </div>
          </>
        )}
        <SellPlayerItemView itemId={characterItem.item.id} />
      </div>
    </Card>
  );
};

export const PlayerInventoryView: FunctionComponent<
  PlayerInventoryViewProps
> = ({ inventory }) => {
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
              equippedItems.map((characterItem) => (
                <CharacterItemView
                  key={characterItem.item.id}
                  characterItem={characterItem}
                />
              ))}
          </div>
        </div>
        <div className={styles["item-group"]}>
          <h3>Unequipped Items</h3>
          <div className={styles["item-list"]}>
            {unequippedItems.length > 0 &&
              unequippedItems.map((characterItem) => (
                <CharacterItemView
                  key={characterItem.item.id}
                  characterItem={characterItem}
                />
              ))}
          </div>
        </div>
      </div>
    </div>
  );
};
