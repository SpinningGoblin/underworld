import { FunctionComponent } from "react";
import { performOpenFixture } from "../../api/actions";
import { OpenFixture } from "../../generated-api";
import { useTheme } from "../../themes";

import styles from "./styles.module.css";

export interface OpenFixtureViewProps {
  args: OpenFixture;
}

export const OpenFixtureView: FunctionComponent<OpenFixtureViewProps> = ({
  args,
}) => {
  const { theme } = useTheme();
  const onClick = () => {
    performOpenFixture(args);
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
