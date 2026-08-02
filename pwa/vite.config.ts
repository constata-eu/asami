import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: {
    host: "0.0.0.0",
    proxy: {
      "/members": "http://127.0.0.1:8000",
      "/assets": "http://127.0.0.1:8000",
      "/config": "http://127.0.0.1:8000",
      "/logs": "http://127.0.0.1:8000"
    }
  }
});
