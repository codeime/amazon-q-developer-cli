import React from "react";
import logger from "loglevel";
import ReactDOM from "react-dom/client";
import { State } from "@amzn/fig-io-api-bindings-wrappers";
import { preloadSpecs } from "@amzn/fig-io-autocomplete-parser";
import App from "./App";
import { captureError } from "./sentry";
import ErrorBoundary from "./components/ErrorBoundary";

State.watch();

// Reload autocomplete every 24 hours
setTimeout(
  () => {
    window.location.reload();
  },
  1000 * 60 * 60 * 24,
);

window.onerror = (message, source, lineno, colno, error) => {
  captureError(error ?? new Error(`${source}:${lineno}:${colno}: ${message}`));
};

window.globalCWD = "";
window.globalSSHString = "";
window.globalTerminalSessionId = "";
window.logger = logger;

logger.setDefaultLevel("warn");

setTimeout(() => {
  preloadSpecs();
}, 0);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ErrorBoundary>
      <App />
    </ErrorBoundary>
  </React.StrictMode>,
);
