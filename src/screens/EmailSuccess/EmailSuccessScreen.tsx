import { FunctionComponent } from "react";
import { useTheme } from "../../themes";

import styles from "./EmailSuccessScreen.module.css";

export const EmailSuccessScreen: FunctionComponent = () => {
  const { theme } = useTheme();

  const text =
    "An email has been sent. Click the provided link to log into the game. If you don't see it, be sure to check your spam folder.";

  return (
    <div
      className={styles.screen}
      style={{ backgroundColor: theme.colors.primary }}
    >
      <div
        className={styles["success-wrap"]}
        style={{
          backgroundColor: theme.colors.secondary,
          color: theme.colors.primary,
        }}
      >
        <h2>Success!</h2>
        <p>{text}</p>
      </div>
    </div>
  );
};
