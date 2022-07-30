import { createContext, useContext } from "react";
import { Theme } from "./Theme";

export interface ThemeContext {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

const Context = createContext<ThemeContext>({
  theme: {} as Theme,
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  setTheme: () => {},
});

export const ThemeProvider = Context.Provider;

export const useTheme = () => {
  const theme = useContext(Context);

  return theme;
};
