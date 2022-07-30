import { FunctionComponent, ReactNode, useState } from "react";
import { ThemeProvider } from "./context";
import { themes } from "./list";
import { Theme } from "./Theme";

interface ThemeWrapperProps {
  children?: ReactNode;
}

export const ThemeWrapper: FunctionComponent<ThemeWrapperProps> = ({
  children,
}) => {
  const [theme, setTheme] = useState<Theme>(themes[0]);

  return <ThemeProvider value={{ theme, setTheme }}>{children}</ThemeProvider>;
};
