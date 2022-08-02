export const getBasePath = (): string => {
  if (window.location.origin === "http://localhost:5173") {
    return "http://localhost:8080/api";
  }

  return `${window.location.href}api`;
};
