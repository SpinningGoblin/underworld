import { FunctionComponent } from "react";

import goblin from "../../images/goblin_big_hat.svg";

import styles from "./Header.module.css";

export const Header: FunctionComponent = () => (
  <header className={styles.header}>
    <img src={goblin} className={styles.logo} alt="logo" />
    <p>Underworld Server</p>
  </header>
);
