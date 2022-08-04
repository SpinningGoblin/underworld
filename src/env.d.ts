/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_UNDERWORLD_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
