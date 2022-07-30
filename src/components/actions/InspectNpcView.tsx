import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { InspectNpc } from "../../generated-api";
import { performInspectNpc } from "../../api/actions";
import { useTheme } from "../../themes";

export interface InspectNpcViewProps {
  args: InspectNpc;
}

export const InspectNpcView: FunctionComponent<InspectNpcViewProps> = ({
  args,
}) => {
  const { theme } = useTheme();
  const onClick = () => {
    performInspectNpc(args);
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
      Inspect
    </button>
  );
};
