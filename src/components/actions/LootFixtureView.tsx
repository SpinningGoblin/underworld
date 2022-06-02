import { FunctionComponent } from "react";
import { performLootFixture } from "../../api/actions";
import { LootFixture, Room } from "../../generated-api";

import styles from "./styles.module.css";

export interface LootFixtureViewProps {
  args: LootFixture;
  room: Room;
}

export const LootFixtureView: FunctionComponent<LootFixtureViewProps> = ({
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
    performLootFixture(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles.actionButton}>
      <b>Loot {args.item_ids.length}</b> items from <b>{fixture.fixture_type}</b>
    </button>
  );
};
