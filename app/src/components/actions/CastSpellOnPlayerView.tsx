import styles from "./styles.module.css";

import { FunctionComponent } from "react";
import { CastSpellOnPlayer } from "../../generated-api";
import { performCastSpellOnPlayer } from "../../api/actions";

export interface CastSpellOnPlayerViewProps {
  args: CastSpellOnPlayer;
}

export const CastSpellOnPlayerView: FunctionComponent<
  CastSpellOnPlayerViewProps
> = ({ args }) => {
  const onClick = () => {
    performCastSpellOnPlayer(args);
  };

  return (
    <button onClick={onClick} className={styles["cast-spell"]}>
      Cast spell
    </button>
  );
};
