export const getBasePath = (): string => {
  if (window.location.href === "http://localhost:3000/") {
    return "http://localhost:8080/api";
  }

  return `${window.location.href}api`;
};
