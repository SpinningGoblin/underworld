import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Routes, Route } from "react-router-dom";

import { setAuthToken } from "./api/configuration";
import { getApiToken, removeApiToken } from "./api/tokens";
import { App } from "./App";
import { ThemeWrapper } from "./themes/ThemeWrapper";

import "./index.css";
import { SignInScreen } from "./screens/SignIn";
import { EmailSuccessScreen } from "./screens/EmailSuccess";
import { getMailCallbackUrl, getSignInUrl } from "./api/path";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ResponseError } from "./generated-api";

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
    window.location.hash !== "#/sign-in" &&
    window.location.hash !== "#/success"
  ) {
    const signInUrl = getSignInUrl();
    console.log(signInUrl);
    window.location.assign(getSignInUrl());
    window.location.reload();
    return;
  }

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        refetchOnWindowFocus: false,
        retry: false,
        cacheTime: Infinity,
        onError: (err) => {
          if (err instanceof ResponseError && err.response.status === 401) {
            removeApiToken();
            window.location.assign(getSignInUrl());
          }
        },
      },
    },
  });

  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <QueryClientProvider client={queryClient}>
        <HashRouter>
          <ThemeWrapper>
            <Routes>
              <Route path="/" element={<App />}></Route>
              <Route path="/sign-in" element={<SignInScreen />}></Route>
              <Route path="/success" element={<EmailSuccessScreen />}></Route>
            </Routes>
          </ThemeWrapper>
        </HashRouter>
      </QueryClientProvider>
    </React.StrictMode>,
  );
})();
