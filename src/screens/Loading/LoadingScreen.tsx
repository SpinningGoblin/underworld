import { FunctionComponent } from "react";

import styles from "./LoadingScreen.module.css";

export const LoadingScreen: FunctionComponent = () => (
  <div className={styles.body}>
    <div className={styles.container}>
      <h1 className={styles.loading}>Loading...</h1>
    </div>
  </div>
);
