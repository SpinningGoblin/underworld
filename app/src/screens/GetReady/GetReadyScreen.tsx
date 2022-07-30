import { FunctionComponent, useEffect, useState } from "react";
import { getUsername, setUsername } from "../../api/username";
import { useTheme } from "../../themes";

import styles from "./GetReadyScreen.module.css";

export interface GetReadyScreenProps {
  onReadyClicked: () => void;
}

export const GetReadyScreen: FunctionComponent<GetReadyScreenProps> = ({
  onReadyClicked,
}) => {
  const { theme } = useTheme();
  const [user, setUser] = useState<string | undefined>(getUsername());

  useEffect(() => {
    if (user) {
      setUsername(user);
    }
  }, [user]);

  return (
    <div className={styles.body}>
      <div className={styles.container}>
        <input
          className={styles.input}
          value={user || ""}
          onChange={(event) => setUser(event.target.value)}
          style={{
            backgroundColor: theme.colors.secondary,
            color: theme.colors.primary,
          }}
        />
        <button
          className={styles.ready}
          onClick={onReadyClicked}
          style={{
            backgroundColor: !user ? "#aaaaaa" : theme.colors.secondary,
            color: theme.colors.primary,
          }}
          disabled={!user}
        >
          Ready
        </button>
      </div>
    </div>
  );
};
