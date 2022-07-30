import styles from "./styles.module.css";

import { FunctionComponent, useState } from "react";
import { AttackNpc } from "../../generated-api";
import { performAttackNpc } from "../../api/actions";
import { useTheme } from "../../themes";

export interface AttackNpcViewProps {
  args: AttackNpc;
}

export const AttackNpcView: FunctionComponent<AttackNpcViewProps> = ({
  args,
}) => {
  const { theme } = useTheme();
  const onClick = () => {
    performAttackNpc(args);
  };

  const [hovering, setHovering] = useState(false);

  return (
    <button
      onClick={onClick}
      className={styles["action-button"]}
      style={{
        backgroundColor: hovering ? theme.colors.red : theme.colors.secondary,
        color: hovering ? theme.colors.secondary : theme.colors.primary,
      }}
      onMouseEnter={() => setHovering(true)}
      onMouseLeave={() => setHovering(false)}
    >
      Attack
    </button>
  );
};
