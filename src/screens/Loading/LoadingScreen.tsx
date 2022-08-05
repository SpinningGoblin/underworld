import { FunctionComponent } from "react";
import { useTheme } from "../../themes";

import styles from "./LoadingScreen.module.css";

export const LoadingScreen: FunctionComponent = () => {
  const { theme } = useTheme();

  return (
    <div className={styles.body} style={{ color: theme.colors.secondary }}>
      <div className={styles.container}>
        <h1 className={styles.loading}>Loading...</h1>
      </div>
    </div>
  );
};
