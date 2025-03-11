/// <reference types="vite/client" />

declare module "../../../package.json" {
  export const version: string;
  export default { version: string };
}

interface ImportMetaEnv {
  readonly VITE_POSTHOG_KEY: string;
  readonly VITE_POSTHOG_HOST: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
