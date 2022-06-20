import { FunctionComponent } from "react";
import { performOpenFixture } from "../../api/actions";
import { OpenFixture } from "../../generated-api";

import styles from "./styles.module.css";

export interface OpenFixtureViewProps {
  args: OpenFixture;
}

export const OpenFixtureView: FunctionComponent<OpenFixtureViewProps> = ({
  args,
}) => {
  const onClick = () => {
    performOpenFixture(args);
  };

  return (
    <button onClick={onClick} className={styles["inspect-button"]}>
      Open
    </button>
  );
};
