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
import { OpenFixtureHiddenCompartmentView } from "../actions/OpenFixtureHiddenCompartment";
import { OpenFixtureView } from "../actions/OpenFixtureView";

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
        item.material ? item.material : "",
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
    discover_hidden_compartment: true,
  };

  const items = fixturePosition.fixture.items.slice();

  const renderHiddenCompartment = (
    hiddenItems: Array<FixtureItem>,
  ): ReactElement => {
    if (!fixturePosition.fixture.knows_if_hidden_compartment) {
      return (
        <div className={styles["hidden-search"]}>
          <span>Hidden compartment?</span>
          <div className={styles["basic-actions"]}>
            <InspectFixtureView args={inspectArgs} />
          </div>
        </div>
      );
    }

    if (!fixturePosition.fixture.has_hidden_compartment) {
      return <span>There is no hidden compartment.</span>;
    }

    if (
      fixturePosition.fixture.has_hidden_compartment &&
      !fixturePosition.fixture.hidden_compartment_open
    ) {
      return (
        <div className={styles["hidden-search"]}>
          <span>The hidden compartment is closed</span>
          <div className={styles["basic-actions"]}>
            <OpenFixtureHiddenCompartmentView
              args={{ fixture_id: fixturePosition.fixture.id }}
            />
          </div>
        </div>
      );
    }

    if (hiddenItems.length === 0) {
      return <span>There are no hidden items.</span>;
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
      {fixturePosition.fixture.can_be_opened && !fixturePosition.fixture.open && (
        <div className={styles["basic-actions"]}>
          <OpenFixtureView args={{ fixture_id: fixturePosition.fixture.id }} />
        </div>
      )}
      <div className={styles.items}>
        {items.length === 0 && "There are no items"}
        {items.length > 0 &&
          items.filter(item => !item.is_in_hidden_compartment).map((item, index) => (
            <ItemView
              key={`${fixturePosition.fixture.id}_${index}`}
              fixtureId={fixturePosition.fixture.id}
              item={item.item}
            />
          ))}
      </div>
      <div className={styles.items}>
        <span className="title">Hidden Compartment</span>
        {renderHiddenCompartment(
          fixturePosition.fixture.items.filter(
            (i) => i.is_in_hidden_compartment,
          ),
        )}
      </div>
    </div>
  );
};
