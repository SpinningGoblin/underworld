import { FunctionComponent } from "react";
import { Room } from "../../generated-api";
import { FixturePositionView } from "../FixturePositionView";

import styles from "./styles.module.css";

export interface FixturePositionsViewProps {
  room: Room;
}

export const FixturePositionsView: FunctionComponent<
  FixturePositionsViewProps
> = ({ room }) => (
  <div className={styles.fixtures}>
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
