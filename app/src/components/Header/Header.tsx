import { FunctionComponent, PropsWithChildren } from "react";

import goblin from "../../images/goblin_big_hat.svg";

import styles from "./Header.module.css";

export const Header: FunctionComponent<PropsWithChildren> = ({ children }) => (
  <header className={styles.header}>
    <div className={styles.title}>
      <img src={goblin} className={styles.logo} alt="logo" />
      <p>Underworld Server</p>
    </div>
    {children}
  </header>
);
