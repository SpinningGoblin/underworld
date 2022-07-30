import {
  FunctionComponent,
  ReactNode,
  useEffect,
  useRef,
  useState,
} from "react";
import {
  ExitRoom,
  FixturePosition,
  FixtureType,
  FlavourText,
  NpcPosition,
  PerformAction,
  PlayerCharacter,
  Room,
} from "../../generated-api";
import { ExitView } from "../ExitView";
import { FixturePositionView } from "../FixturePositionView";
import { NpcPositionView } from "../NpcPositionView";
import CloseIcon from "../../images/close.svg";

import styles from "./RoomView.module.css";
import { useTheme } from "../../themes/context";

export interface RoomViewProps {
  room: Room;
  actions: Array<PerformAction>;
  player: PlayerCharacter;
}

const flavourText = (flavour: FlavourText): string => {
  switch (flavour) {
    case "a_strange_breeze_blows":
      return "A strange breeze blows through the room.";
    case "mold_moss_covers_walls":
      return "A strange mold and moss cover the walls.";
    case "unseen_lights_flicker_walls":
      return "Unseen lights flicker across the walls.";
    case "is_something_watching_you":
      return "It feels like there is something you can't see watching you.";
    case "smells_like_old_goblin_socks":
      return "It smells like old goblin socks. Where is that smell coming from?";
    case "something_squishy_all_over_floor":
      return "There's something squishy all over the floor...";
    case "you_hear_scratching_all_around_you":
      return "You hear scratching all around you... But from what?";
  }
};

const description = (room: Room): string => {
  const sizes: Array<string> = [];

  if (room.dimensions.height !== "average") {
    sizes.push(room.dimensions.height.replaceAll("_", ""));
  }

  if (room.dimensions.length !== "average") {
    sizes.push(room.dimensions.length.replaceAll("_", " "));
  }

  if (room.dimensions.width !== "average") {
    sizes.push(room.dimensions.width.replaceAll("_", " "));
  }

  const descriptors = room.descriptors
    .slice()
    .sort((a, b) => a.localeCompare(b))
    .map((t) => t.replaceAll("_", " "));

  const flavour: Array<string> = [];
  if (room.flavour) {
    flavour.push(flavourText(room.flavour));
  }

  return [
    ...sizes,
    ...descriptors,
    `${room.room_type.replaceAll("_", " ")}.`,
    ...flavour,
  ].join(" ");
};

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

const fixtureDescriptionText = (fixturePosition: FixturePosition): string =>
  [
    ...fixturePosition.fixture.descriptors.map((text) =>
      text.replaceAll("_", " "),
    ),
    fixturePosition.fixture.material ? fixturePosition.fixture.material : "",
    fixtureTypeText(fixturePosition.fixture.fixture_type),
  ].join(" ");

const starterText = (index: number, max: number): string => {
  if (index === 0) {
    return "There is a ";
  }

  if (index === max) {
    return " and a ";
  }

  return ", a ";
};

export const RoomView: FunctionComponent<RoomViewProps> = ({
  room,
  player,
}) => {
  const { theme } = useTheme();
  const [showExits, setShowExits] = useState<boolean>(false);
  const [shownNpcId, setShownNpcId] = useState<string | undefined>();
  const [shownFixtureId, setShownFixtureId] = useState<string | undefined>();
  const roomId = useRef(room.id);

  useEffect(() => {
    if (room.id !== roomId.current) {
      setShowExits(false);
      setShownNpcId(undefined);
      setShownFixtureId(undefined);
      roomId.current = room.id;
    }
  }, [room]);

  const exitText = (): ReactNode => (
    <span>
      <span>You see </span>
      <button
        className={styles["description-button"]}
        onClick={() => setShowExits((current) => !current)}
      >
        {room.exits.length} exits
      </button>{" "}
    </span>
  );

  const singleNpcText = (
    npcPosition: NpcPosition,
    starterText: string,
    finisherText: string,
  ): ReactNode => (
    <span key={npcPosition.npc.id}>
      <span>{starterText}</span>
      <button
        className={styles["description-button"]}
        onClick={() => setShownNpcId(npcPosition.npc.id)}
      >
        {`${npcPosition.npc.character.species}`}
      </button>
      <span>{finisherText}</span>
    </span>
  );

  const npcText = (): ReactNode => {
    if (!room.npc_positions.length) {
      return "There are no creatures in the room with you.";
    }

    if (room.npc_positions.length === 1) {
      return singleNpcText(room.npc_positions[0], "There is a ", ". ");
    }

    return (
      <>
        {room.npc_positions.map((npcPosition, index) =>
          singleNpcText(
            npcPosition,
            starterText(index, room.npc_positions.length - 1),
            index === room.npc_positions.length - 1
              ? " in the room with you. "
              : "",
          ),
        )}
      </>
    );
  };

  const singleFixtureText = (
    fixturePosition: FixturePosition,
    starterText: string,
    finisherText: string,
  ): ReactNode => (
    <span key={fixturePosition.fixture.id}>
      <span>{starterText}</span>
      <button
        className={styles["description-button"]}
        onClick={() => setShownFixtureId(fixturePosition.fixture.id)}
      >
        {`${fixtureDescriptionText(fixturePosition)}`}
      </button>
      <span>{finisherText}</span>
    </span>
  );

  const fixtureText = (): ReactNode => {
    if (!room.fixture_positions.length) {
      return " There is nothing else interesting in the room.";
    }

    if (room.fixture_positions.length === 1) {
      return singleFixtureText(room.fixture_positions[0], " There is a ", ". ");
    }

    return (
      <>
        {room.fixture_positions.map((fixturePosition, index) =>
          singleFixtureText(
            fixturePosition,
            starterText(index, room.fixture_positions.length - 1),
            index === room.fixture_positions.length - 1 ? " around. " : "",
          ),
        )}
      </>
    );
  };

  const exitView = (): ReactNode => (
    <>
      <div className={styles.exits}>
        {room.exits.map((exit) => {
          const exitArgs: ExitRoom = {
            exit_id: exit.id,
          };

          return <ExitView key={exit.id} exit={exit} exitArgs={exitArgs} />;
        })}
      </div>
    </>
  );

  const npcView = (): ReactNode => {
    const npcPosition = room.npc_positions.find(
      (npcPosition) => npcPosition.npc.id === shownNpcId,
    );
    return (
      <>
        {npcPosition && (
          <NpcPositionView npcPosition={npcPosition} player={player} />
        )}
      </>
    );
  };

  const fixtureView = (): ReactNode => {
    const fixturePosition = room.fixture_positions.find(
      (fixturePosition) => fixturePosition.fixture.id === shownFixtureId,
    );
    return (
      <>
        {fixturePosition && (
          <FixturePositionView fixturePosition={fixturePosition} />
        )}
      </>
    );
  };

  const descriptionView = () => (
    <span
      className={styles.description}
      style={{ color: theme.colors.secondary }}
    >
      {`You are in a ${description(room)} `}
      <br />
      {exitText()}
      <br />
      {npcText()}
      <br />
      {fixtureText()}
    </span>
  );

  let body: ReactNode;
  if (showExits) {
    body = exitView();
  } else if (shownNpcId) {
    body = npcView();
  } else if (shownFixtureId) {
    body = fixtureView();
  } else {
    body = descriptionView();
  }

  const showClose = showExits || !!shownNpcId || !!shownFixtureId;

  return (
    <div style={{ color: theme.colors.secondary }}>
      {showClose && (
        <div className={styles.close}>
          <button
            onClick={() => {
              setShowExits(false);
              setShownFixtureId(undefined);
              setShownNpcId(undefined);
            }}
            className={styles["close-button"]}
            style={{ backgroundColor: theme.colors.secondary }}
          >
            <img className={styles["close-icon"]} src={CloseIcon} alt="close" />
          </button>
        </div>
      )}
      {body}
    </div>
  );
};
