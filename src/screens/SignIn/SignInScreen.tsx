import { FunctionComponent, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { getApiLoginUrl } from "../../api/path";
import { Header } from "../../components/Header";
import { useTheme } from "../../themes";

import styles from "./SignInScreen.module.css";

export const SignInScreen: FunctionComponent = () => {
  const { theme } = useTheme();
  const [valid, setValid] = useState(true);
  const [searchParams, _] = useSearchParams();

  const error = searchParams.get("error");

  return (
    <div
      className={styles.screen}
      style={{ backgroundColor: theme.colors.primary }}
    >
      <Header />
      <div
        className={styles["login-form-wrap"]}
        style={{ backgroundColor: theme.colors.secondary }}
      >
        <h2>Login</h2>
        <form id="login-form" action={getApiLoginUrl()} method="post">
          <p className={styles["token-container"]}>
            <label htmlFor="token_type">Play game or get token?</label>
            <select
              className={styles["token-type"]}
              id="token_type"
              name="token_type"
            >
              <option value="play_the_game">Play the game</option>
              <option value="api_token">Get API Token</option>
            </select>
          </p>
          <p className={styles["email-container"]}>
            <input
              className={styles["email-input"]}
              type="email"
              id="email"
              name="email"
              placeholder="Email"
              required
              onChange={(event) => {
                if (!event.currentTarget.value) {
                  setValid(true);
                } else {
                  setValid(event.currentTarget.checkValidity());
                }
              }}
              style={{
                backgroundColor: theme.colors.secondary,
                borderColor: valid ? theme.colors.primary : theme.colors.error,
                color: theme.colors.primary,
              }}
            />
            <i className={styles.validation}>
              <span></span>
              <span></span>
            </i>
            {error === "email_required" && (
              <span style={{ color: theme.colors.error }}>
                Email is required
              </span>
            )}
          </p>
          <p className={styles["submit-container"]}>
            <input
              className={styles.submit}
              type="submit"
              id="login"
              value="Login"
              style={{
                backgroundColor: theme.colors.primary,
                borderColor: theme.colors.tertiary,
                color: theme.colors.secondary,
              }}
            />
          </p>
        </form>
      </div>
    </div>
  );
};
