import { FunctionComponent } from "react";
import { performInspectFixture } from "../../api/actions";
import { InspectFixture } from "../../generated-api";
import { useTheme } from "../../themes";

import styles from "./styles.module.css";

export interface InspectFixtureViewProps {
  args: InspectFixture;
}

export const InspectFixtureView: FunctionComponent<InspectFixtureViewProps> = ({
  args,
}) => {
  const { theme } = useTheme();

  const onClick = () => {
    performInspectFixture(args);
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
      Search
    </button>
  );
};
