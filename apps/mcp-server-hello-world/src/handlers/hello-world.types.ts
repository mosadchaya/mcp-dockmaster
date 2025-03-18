export type HelloWorldInput = {
  message?: string;
  format: OutputFormat;
};

export type HelloWorldConfigInput = {
  config: string;
  format: OutputFormat;
};

export enum OutputFormat {
  CURRENT = "current",
  LEGACY = "legacy"
}