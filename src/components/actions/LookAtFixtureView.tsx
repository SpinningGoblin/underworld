import { FunctionComponent } from "react";
import { performLookAtFixture } from "../../api/actions";
import { LookAtFixture, Room } from "../../generated-api";

import styles from "./styles.module.css";

export interface LookAtFixtureViewProps {
  args: LookAtFixture;
  room: Room;
}

export const LookAtFixtureView: FunctionComponent<LookAtFixtureViewProps> = ({
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
    performLookAtFixture(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Look</b> at <b>{fixture.fixture_type}</b>
    </button>
  );
};
