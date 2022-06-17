import { FunctionComponent, useEffect, useState } from "react";
import { getUsername, setUsername } from "../../api/username";

import goblin from "../../images/goblin_big_hat.svg";

import styles from "./styles.module.css";

export interface GetReadyScreenProps {
  onReadyClicked: () => void;
}

export const GetReadyScreen: FunctionComponent<GetReadyScreenProps> = ({
  onReadyClicked,
}) => {
  const [user, setUser] = useState<string | undefined>(getUsername());

  useEffect(() => {
    if (user) {
      setUsername(user);
    }
  }, [user]);

  return (
    <div className="App">
      <header className="App-header">
        <img src={goblin} className="App-logo" alt="logo" />
        <p>Underworld Server</p>
      </header>
      <div className="body">
        <div className={styles.container}>
          <input
            className={styles.input}
            value={user || ""}
            onChange={(event) => setUser(event.target.value)}
          />
          <button
            className={styles.ready}
            onClick={onReadyClicked}
            disabled={!user}
          >
            Ready
          </button>
        </div>
      </div>
    </div>
  );
};