import { FunctionComponent } from "react";
import { performInspectFixture } from "../../api/actions";
import { InspectFixture, Room } from "../../generated-api";

import styles from "./styles.module.css";

export interface InspectFixtureViewProps {
  args: InspectFixture;
  room: Room;
}

export const InspectFixtureView: FunctionComponent<InspectFixtureViewProps> = ({
  args,
  room,
}) => {
  const fixture = room.fixture_positions
    .flatMap((f) => f.fixtures)
    .find((fixture) => fixture.identifier.id === args.fixture_id);

  if (!fixture) {
    return <div>Fixture not found in room...</div>;
  }

  const onClick = () => {
    performInspectFixture(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Inspect</b> a <b>{fixture.fixture_type}</b>
    </button>
  );
};
