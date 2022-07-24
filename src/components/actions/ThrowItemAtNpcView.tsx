import { FunctionComponent, useEffect, useState } from "react";
import { performThrowItemAtNpc } from "../../api/actions";
import {
  Item,
  ItemType,
  Throwable,
  ThrowableEffect,
  ThrowItemAtNpc,
} from "../../generated-api";

import styles from "./styles.module.css";

export interface ThrowItemAtNpcViewProps {
  items: Array<Item>;
  npcId: string;
}

const itemTypeText = (itemType: ItemType): string =>
  itemType.replaceAll("_", " ");
const throwableText = ({ name }: ThrowableEffect): string => {
  switch (name) {
    case "oil_splash":
      return "oil splash";
  }
};

const itemText = (itemType: ItemType, throwable: Throwable): string =>
  `${throwableText(throwable.effect)} ${itemTypeText(itemType)}`;

export const ThrowItemAtNpcView: FunctionComponent<ThrowItemAtNpcViewProps> = ({
  items,
  npcId,
}) => {
  const [itemId, setItemId] = useState<string>(items[0].id);

  useEffect(() => {
    setItemId(items[0].id);
  }, [items]);

  const options = items.map((item) => (
    <option key={item.id} value={item.id}>
      {itemText(item.item_type, item.throwable!)}
    </option>
  ));

  options.splice(0, 0, <option key="empty" value=""></option>);

  const onClick = () => {
    const throwItemAtNpc: ThrowItemAtNpc = {
      item_id: itemId,
      npc_id: npcId,
    };

    performThrowItemAtNpc(throwItemAtNpc);
  };

  const value = items.find((item) => item.id === itemId)?.id || "";

  return (
    <div className={styles["items-on-npc"]}>
      <select
        value={value}
        onChange={(event) => setItemId(event.currentTarget.value)}
      >
        {options}
      </select>
      <button onClick={onClick} className={styles["action-button"]}>
        Throw
      </button>
    </div>
  );
};
