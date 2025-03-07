import { Tools } from "../types.js";
import { MCPInstall } from "./MCPInstall.js";
import { MCPSearch } from "./MCPSearch.js";


export async function runInternalTool(params: { name: string, arguments: Record<string, any> }): Promise<{ isInternalTool: boolean, result: any }> {
  if (params.name === MCPInstall.name) {
    return { 
        isInternalTool: true, 
        result: await MCPInstall.install(params.arguments.name),
     };
  }
  
  if (params.name === MCPSearch.name) {
    return { 
        isInternalTool: true, 
        result: await MCPSearch.search(params.arguments.query, params.arguments.exact || false),
     };
  }
  
  return { isInternalTool: false, result: null };
}

export async function initInternalTools() {
  await MCPInstall.init();
  await MCPSearch.init();
}

export async function injectInternalTools(tools: Tools) {
  tools.tools.push(MCPInstall.tool);
  tools.tools.push(MCPSearch.tool);
}