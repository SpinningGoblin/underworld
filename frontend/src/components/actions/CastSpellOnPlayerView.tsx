import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { CastSpellOnPlayer } from "../../generated-api";
import { performCastSpellOnPlayer } from "../../api/actions";
import { useTheme } from "../../themes";

export interface CastSpellOnPlayerViewProps {
  args: CastSpellOnPlayer;
}

export const CastSpellOnPlayerView: FunctionComponent<
  CastSpellOnPlayerViewProps
> = ({ args }) => {
  const { theme } = useTheme();

  const onClick = () => {
    performCastSpellOnPlayer(args);
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
      Cast spell
    </button>
  );
};
