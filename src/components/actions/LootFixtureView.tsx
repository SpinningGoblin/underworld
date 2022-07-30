import { FunctionComponent } from "react";
import { performLootFixture } from "../../api/actions";
import { LootFixture } from "../../generated-api";
import { useTheme } from "../../themes";

import styles from "./styles.module.css";

export interface LootFixtureViewProps {
  args: LootFixture;
}

export const LootFixtureView: FunctionComponent<LootFixtureViewProps> = ({
  args,
}) => {
  const { theme } = useTheme();
  const onClick = () => {
    performLootFixture(args);
  };

  return (
    <button
      onClick={onClick}
      className={styles["action-button"]}
      style={{
        backgroundColor: theme.colors.secondary,
        color: theme.colors.primary,
      }}
    >
      Loot
    </button>
  );
};
