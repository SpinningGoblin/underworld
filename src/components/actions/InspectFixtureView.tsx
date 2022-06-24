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
    performInspectFixture(args);
  };

  return (
    <button onClick={onClick} className={styles["action-button"]}>
      Search
    </button>
  );
};
