import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { QueryClient, QueryClientProvider } from "react-query";
import { AnimeWrapper } from "./components/AnimeWrapper";

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <AnimeWrapper>
        <App />
      </AnimeWrapper>
    </QueryClientProvider>
  </React.StrictMode>,
);
