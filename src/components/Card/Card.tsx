import { FunctionComponent, HTMLProps, ReactNode, useState } from "react";
import { useTheme } from "../../themes";

import styles from "./Card.module.css";

export interface CardProps {
  children?: ReactNode;
}

export const Card: FunctionComponent<
  CardProps & Pick<HTMLProps<HTMLDivElement>, "className">
> = ({ children, className }) => {
  const { theme } = useTheme();

  const [hovering, setHovering] = useState(false);

  return (
    <div
      className={[styles.card, className ?? ""].join(" ")}
      onMouseEnter={() => setHovering(true)}
      onMouseLeave={() => setHovering(false)}
      style={{
        borderColor: hovering ? theme.colors.tertiary : theme.colors.primary,
      }}
    >
      {children}
    </div>
  );
};
