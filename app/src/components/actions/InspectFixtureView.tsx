import { FunctionComponent } from "react";
import { performInspectFixture } from "../../api/actions";
import { InspectFixture } from "../../generated-api";

import styles from "./styles.module.css";

export interface InspectFixtureViewProps {
  args: InspectFixture;
}

export const InspectFixtureView: FunctionComponent<InspectFixtureViewProps> = ({
  args,
}) => {
  const onClick = () => {
    performInspectFixture(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles["inspect-button"]}>
      Inspect
    </button>
  );
};
