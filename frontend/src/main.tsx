import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Routes, Route } from "react-router-dom";

import { setAuthToken } from "./api/configuration";
import { getApiToken } from "./api/tokens";
import { App } from "./App";
import { ThemeWrapper } from "./themes/ThemeWrapper";

import "./index.css";
import { SignInScreen } from "./screens/SignIn";
import { EmailSuccessScreen } from "./screens/EmailSuccess";
import { getMailCallbackUrl } from "./api/path";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { TokenProvider } from "./hooks/tokens";

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
  }

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        refetchOnWindowFocus: false,
        retry: false,
        gcTime: Infinity,
      },
    },
  });

  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <QueryClientProvider client={queryClient}>
        <HashRouter>
          <ThemeWrapper>
            <TokenProvider value={apiToken}>
              <Routes>
                <Route
                  path="/"
                  element={apiToken ? <App /> : <SignInScreen />}
                ></Route>
                <Route path="/sign-in" element={<SignInScreen />}></Route>
                <Route path="/success" element={<EmailSuccessScreen />}></Route>
              </Routes>
            </TokenProvider>
          </ThemeWrapper>
        </HashRouter>
      </QueryClientProvider>
    </React.StrictMode>,
  );
})();
