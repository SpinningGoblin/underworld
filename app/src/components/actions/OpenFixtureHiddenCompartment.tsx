import { FunctionComponent } from "react";
import { performOpenFixtureHiddenCompartment } from "../../api/actions";
import { OpenFixtureHiddenCompartment } from "../../generated-api";

import styles from "./styles.module.css";

export interface OpenFixtureHiddenCompartmentViewProps {
  args: OpenFixtureHiddenCompartment;
}

export const OpenFixtureHiddenCompartmentView: FunctionComponent<
  OpenFixtureHiddenCompartmentViewProps
> = ({ args }) => {
  const onClick = () => {
    performOpenFixtureHiddenCompartment(args);
  };

  return (
    <button onClick={onClick} className={styles["action-button"]}>
      Open
    </button>
  );
};
