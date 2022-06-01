export const getBasePath = (): string => {
  console.log(window.location.href);
  if (window.location.href === "http://localhost:3000/") {
    return "http://localhost:8080/api";
  }

  return `${window.location.href}api`;
};
