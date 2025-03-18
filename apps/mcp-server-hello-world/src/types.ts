export type ToolResultSchema<T> = {
  content: Array<{
    type: string;
    text: string;
  }>;
  isError: boolean;
  toolResult?: T;
};

export type ToolResultSchemaLegacy = {
  toolResult: {
    content: Array<{
      type: string;
      text: string;
    }>;
  };
  isError: boolean;
};