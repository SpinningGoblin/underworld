import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { InspectNpc } from "../../generated-api";
import { performInspectNpc } from "../../api/actions";

export interface InspectNpcViewProps {
  args: InspectNpc;
}

export const InspectNpcView: FunctionComponent<InspectNpcViewProps> = ({
  args,
}) => {
  const onClick = () => {
    performInspectNpc(args).catch((e) => console.error(e));
  };

  return (
    <button onClick={onClick} className={styles["inspect-button"]}>
      Inspect
    </button>
  );
};
