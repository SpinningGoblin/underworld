import { FunctionComponent } from "react";
import { performOpenFixtureHiddenCompartment } from "../../api/actions";
import { OpenFixtureHiddenCompartment } from "../../generated-api";
import { useTheme } from "../../themes";

import styles from "./styles.module.css";

export interface OpenFixtureHiddenCompartmentViewProps {
  args: OpenFixtureHiddenCompartment;
}

export const OpenFixtureHiddenCompartmentView: FunctionComponent<
  OpenFixtureHiddenCompartmentViewProps
> = ({ args }) => {
  const { theme } = useTheme();
  const onClick = () => {
    performOpenFixtureHiddenCompartment(args);
  };

  return (
    <button
      onClick={onClick}
      className={styles["action-button"]}
      style={{
        backgroundColor: theme.colors.secondary,
        color: theme.colors.primary,
      }}
    >
      Open
    </button>
  );
};
