import { FunctionComponent, ReactNode, useState } from "react";
import { ThemeProvider } from "./context";
import { themes } from "./list";
import { Theme } from "./Theme";

interface ThemeWrapperProps {
  children?: ReactNode;
}

const THEME_KEY = "underworld_theme";

const startingThemeName = localStorage.getItem(THEME_KEY);

export const ThemeWrapper: FunctionComponent<ThemeWrapperProps> = ({
  children,
}) => {
  const startingTheme =
    themes.find((t) => t.name === startingThemeName) ?? themes[0];
  const [theme, setTheme] = useState<Theme>(startingTheme);

  const onThemeChange = (newTheme: Theme) => {
    localStorage.setItem(THEME_KEY, newTheme.name);
    setTheme(newTheme);
  };

  return (
    <ThemeProvider value={{ theme, setTheme: onThemeChange }}>
      {children}
    </ThemeProvider>
  );
};
