/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_UNDERWORLD_URL: string;
  readonly VITE_FRONTEND_BASE_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
