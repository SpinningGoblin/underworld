import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import { setAuthToken } from "./api/configuration";
import { getApiToken } from "./api/tokens";
import { App } from "./App";
import { ThemeWrapper } from "./themes/ThemeWrapper";

import "./index.css";
import { SignInScreen } from "./screens/SignIn";
import { EmailSuccessScreen } from "./screens/EmailSuccess";
import { getMailCallbackUrl, getSignInUrl } from "./api/path";

(() => {
  const params = new URL(window.location.href).searchParams;
  const mailToken = params.get("mail_token");
  if (mailToken) {
    const mailUrl = getMailCallbackUrl(mailToken);
    window.location.assign(mailUrl);
    return;
  }

  const apiToken = getApiToken();
  if (apiToken) {
    setAuthToken(apiToken);
  } else if (
    window.location.pathname !== "/sign-in" &&
    window.location.pathname !== "/success"
  ) {
    window.location.assign(getSignInUrl());
    return;
  }

  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <BrowserRouter>
        <ThemeWrapper>
          <Routes>
            <Route path="/" element={<App />}></Route>
            <Route path="/sign-in" element={<SignInScreen />}></Route>
            <Route path="/success" element={<EmailSuccessScreen />}></Route>
          </Routes>
        </ThemeWrapper>
      </BrowserRouter>
    </React.StrictMode>,
  );
})();
