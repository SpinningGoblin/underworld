import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { ThemeWrapper } from "./themes/ThemeWrapper";

import "./index.css";
import { setAuthToken } from "./api/configuration";

const apiToken = window.location.hash.replaceAll("#", "");

if (apiToken) {
  setAuthToken(apiToken);
}

console.log(apiToken);
window.location.hash = "";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ThemeWrapper>
      <App />
    </ThemeWrapper>
  </React.StrictMode>,
);
