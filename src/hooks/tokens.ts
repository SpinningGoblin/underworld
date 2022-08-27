import { createContext, useContext } from "react";

const TokenContext = createContext<string | undefined>(undefined);
export const TokenProvider = TokenContext.Provider;

export const useApiToken = () => {
  const value = useContext(TokenContext);

  return value;
};
