import { ToolResultSchema, ToolResultSchemaLegacy } from "../types";

/**
 * Utility function to create a response
 * @param message The response message
 * @param isError Whether the response is an error
 * @returns A ToolResultSchema with the response message
 */
export const createResponse = <T>(message: T, isError: boolean = false): ToolResultSchema<T> => {
  return {
    content: [{
      type: "text",
      text: typeof message === "string" ? message : JSON.stringify(message, null, 2)
    }],
    isError
  };
};

/**
 * Utility function to create a legacy response
 * @param message The response message
 * @param isError Whether the response is an error
 * @returns A ToolResultSchemaLegacy with the response message
 */
export const createResponseLegacy = <T>(message: T, isError: boolean = false): ToolResultSchemaLegacy => {
  return {
    toolResult: { content: [{
      type: "text",
      text: typeof message === "string" ? message : JSON.stringify(message, null, 2)
    }]},
    isError
  };
};