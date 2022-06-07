import { FunctionComponent } from "react";
import { Room } from "../../generated-api";
import { FixturePositionView } from "../FixturePositionView";

import styles from "./styles.module.css";

export interface FixturePositionsViewProps {
  room: Room;
}

const fixtureText = (room: Room): string => {
  if (!room.fixture_positions.length) {
    return "There is nothing else interesting in the room.";
  }

  const itemText =
    room.fixture_positions.length === 1
      ? "is 1 item"
      : `are ${room.fixture_positions.length} items`;

  return `There ${itemText} in the room with you.`;
};

export const FixturePositionsView: FunctionComponent<
  FixturePositionsViewProps
> = ({ room }) => (
  <div className={styles.fixtures}>
    <div className={styles["fixture-text"]}>{fixtureText(room)}</div>
    <div className={styles["fixture-list"]}>
      {room.fixture_positions.map((fixturePosition) => (
        <FixturePositionView
          key={fixturePosition.fixture.id}
          fixturePosition={fixturePosition}
        />
      ))}
    </div>
  </div>
);
