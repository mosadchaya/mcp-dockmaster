import { ToolResultSchema, ToolResultSchemaLegacy } from "../types";
import { createResponseLegacy, createResponse } from "./utils";
import { HelloWorldInput, HelloWorldConfigInput, OutputFormat } from "./hello-world.types";

export const helloWorldHandler = async (input: HelloWorldInput): Promise<ToolResultSchema<string> | ToolResultSchemaLegacy> => {
  const format = input.format || OutputFormat.CURRENT;

  try {
    if (format === OutputFormat.LEGACY) {
      return createResponseLegacy<string>("hello world");
    }
    else {
      return createResponse<string>("hello world");
    }
  } catch (error) {
    if (format === OutputFormat.LEGACY) {
      return createResponseLegacy<string>(`Error in hello_world: ${error instanceof Error ? error.message : String(error)}`, true);
    }
    else {
      return createResponse<string>(`Error in hello_world: ${error instanceof Error ? error.message : String(error)}`, true);
    }
  }
};

export const helloWorldWithInputHandler = async (input: HelloWorldInput): Promise<ToolResultSchema<string> | ToolResultSchemaLegacy> => {
  const format = input.format || OutputFormat.CURRENT;

  try {
    const message = input.message || "";
    if (format === OutputFormat.LEGACY) {
      return createResponseLegacy<string>(`hello world ${message}`);
    }
    else {
      return createResponse<string>(`hello world ${message}`);
    }
  } catch (error) {
    if (format === OutputFormat.LEGACY) {
      return createResponseLegacy<string>(`Error in hello_world_with_input: ${error instanceof Error ? error.message : String(error)}`, true);
    }
    else {
      return createResponse<string>(`Error in hello_world_with_input: ${error instanceof Error ? error.message : String(error)}`, true);
    }
  }
};

export const helloWorldWithConfigHandler = async (input: HelloWorldConfigInput): Promise<ToolResultSchema<string> | ToolResultSchemaLegacy> => {
  const format = input.format || OutputFormat.CURRENT;

  try {
    if (format === OutputFormat.LEGACY) {
      if (!input.config) {
        return createResponseLegacy<string>("Config is required", true);
      }
      return createResponseLegacy<string>(`hello configuration ${input.config}`);
    }
    else {
      if (!input.config) {
        return createResponse<string>("Config is required", true);
      }
      return createResponse<string>(`hello configuration ${input.config}`);
    }
  } catch (error) {
    if (format === OutputFormat.LEGACY) {
      return createResponseLegacy<string>(`Error in hello_world_with_config: ${error instanceof Error ? error.message : String(error)}`, true);
    }
    else {
      return createResponse<string>(`Error in hello_world_with_config: ${error instanceof Error ? error.message : String(error)}`, true);
    }
  }
};
