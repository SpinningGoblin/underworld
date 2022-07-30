import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { ThemeWrapper } from "./themes/ThemeWrapper";

import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ThemeWrapper>
      <App />
    </ThemeWrapper>
  </React.StrictMode>,
);
