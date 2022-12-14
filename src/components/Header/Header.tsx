import { FunctionComponent, PropsWithChildren } from "react";

import goblin from "../../images/goblin_big_hat.svg";
import { useTheme } from "../../themes";

import styles from "./Header.module.css";

export const Header: FunctionComponent<PropsWithChildren> = ({ children }) => {
  const { theme } = useTheme();

  return (
    <header
      className={styles.header}
      style={{
        color: theme.colors.secondary,
        borderColor: theme.colors.secondary,
      }}
    >
      <div className={styles.title}>
        <img src={goblin} className={styles.logo} alt="logo" />
        <h1>Underworld Server {`(${import.meta.env.VITE_APP_VERSION})`}</h1>
      </div>
      {children}
    </header>
  );
};
