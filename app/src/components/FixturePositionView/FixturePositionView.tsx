import { FunctionComponent, ReactElement } from "react";
import {
  FixtureItem,
  FixturePosition,
  FixtureType,
  InspectFixture,
  Item,
  ItemDescriptor,
  ItemType,
} from "../../generated-api";
import { InspectFixtureView } from "../actions/InspectFixtureView";
import { LootFixtureView } from "../actions/LootFixtureView";

import styles from "./styles.module.css";

export interface FixturePositionViewProps {
  fixturePosition: FixturePosition;
}

const itemTypeText = (itemType: ItemType): string => itemType.replace("_", " ");
const descriptorText = (descriptor: ItemDescriptor): string =>
  descriptor.replace("_", " ");

const fixtureTypeText = (fixtureType: FixtureType): string => {
  switch (fixtureType) {
    case "statue_tentacled_monstrosity":
      return "statue of a tentacled monstrosity";
    case "statue_warrior":
      return "statue of an unknown warrior";
    default:
      return fixtureType.replaceAll("_", " ");
  }
};

const descriptionText = (fixturePosition: FixturePosition): string =>
  [
    ...fixturePosition.fixture.descriptors.map((text) =>
      text.replaceAll("_", " "),
    ),
    fixturePosition.fixture.material ? fixturePosition.fixture.material : "",
    fixtureTypeText(fixturePosition.fixture.fixture_type),
  ].join(" ");

const positionText = (fixturePosition: FixturePosition): string => {
  if (fixturePosition.position_descriptor) {
    switch (fixturePosition.position_descriptor) {
      case "cracked_and_broken_on_the_ground":
        return "It is cracked and broken on the ground";
      case "is_in_the_corner":
        return "It is in the corner";
      case "sits_along_one_side":
        return "It sits on one side of the room";
      case "stands_in_the_corner":
        return "It stands in the corner";
      default:
        return "";
    }
  }
  return "";
};

interface ItemViewProps {
  item: Item;
  fixtureId: string;
}

const ItemView: FunctionComponent<ItemViewProps> = ({ item, fixtureId }) => (
  <div className={[styles.item, "action-card"].join(" ")}>
    <div className={styles["item-name"]}>
      {[
        ...item.descriptors.map(descriptorText),
        itemTypeText(item.item_type),
      ].join(" ")}
    </div>

    <LootFixtureView args={{ fixture_id: fixtureId, item_ids: [item.id] }} />
  </div>
);

export const FixturePositionView: FunctionComponent<
  FixturePositionViewProps
> = ({ fixturePosition }) => {
  const inspectArgs: InspectFixture = {
    fixture_id: fixturePosition.fixture.id,
    discover_can_be_opened: true,
    discover_contained: true,
    discover_hidden: true,
    discover_hidden_items: true,
  };

  const items = fixturePosition.fixture.items.filter((i) => !i.is_hidden);

  const renderHiddenCompartment = (
    hiddenItems: Array<FixtureItem>,
  ): ReactElement => {
    if (!fixturePosition.fixture.knows_hidden_compartment_items) {
      return <span>You do not know their inventory</span>;
    }

    if (hiddenItems.length === 0) {
      return <span>There are no hidden items</span>;
    }

    return (
      <>
        {hiddenItems.map((item, index) => (
          <ItemView
            key={`${fixturePosition.fixture.id}_${index}`}
            fixtureId={fixturePosition.fixture.id}
            item={item.item}
          />
        ))}
      </>
    );
  };

  return (
    <div className={styles["fixture-position"]}>
      <div>
        <span className={styles.label}>Description: </span>
        {descriptionText(fixturePosition)}
      </div>
      <div>{positionText(fixturePosition)}</div>
      <div className={styles["basic-actions"]}>
        <InspectFixtureView args={inspectArgs} />
      </div>
      <div className={styles.items}>
        {!fixturePosition.fixture.knows_contained_items &&
          "You do not know what items it holds."}
        {fixturePosition.fixture.knows_contained_items &&
          items.length === 0 &&
          "It holds no items"}
        {fixturePosition.fixture.knows_contained_items &&
          items.length > 0 &&
          items.map((item, index) => (
            <ItemView
              key={`${fixturePosition.fixture.id}_${index}`}
              fixtureId={fixturePosition.fixture.id}
              item={item.item}
            />
          ))}
      </div>
      <div className={styles.items}>
        {!fixturePosition.fixture.knows_if_hidden_compartment &&
          "You do not know if there is a hidden compartment."}
        {fixturePosition.fixture.knows_if_hidden_compartment && (
          <>
            <span className="title">Hidden Compartment</span>
            {renderHiddenCompartment(
              fixturePosition.fixture.items.filter((i) => i.is_hidden),
            )}
          </>
        )}
      </div>
    </div>
  );
};
