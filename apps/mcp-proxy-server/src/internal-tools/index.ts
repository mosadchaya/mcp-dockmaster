import { Tools } from "../types.ts";
import { MCPConfig } from "./MCPConfig.ts";
import { MCPInstall } from "./MCPInstall.ts";
import { MCPSearch } from "./MCPSearch.ts";


export async function runInternalTool(params: { name: string, arguments: Record<string, any> }): Promise<{ isInternalTool: boolean, result: any }> {
  if (params.name === MCPInstall.name) {
    const response = { 
        isInternalTool: true, 
        result: await MCPInstall.install(params.arguments.tool_id),
     };

    //  setTimeout(() => {
    //   console.log(JSON.stringify({"jsonrpc":"2.0","method":"notifications/tools/list_changed"}));
    //  }, 1000);

     return response;
  }
  
  if (params.name === MCPSearch.name) {
    return { 
        isInternalTool: true, 
        result: await MCPSearch.search(params.arguments.query, params.arguments.exact || false),
     };
  }

  if (params.name === MCPConfig.name) {
    return {
      isInternalTool: true,
      result: await MCPConfig.configure(params.arguments.tool_id, params.arguments.config),
    };
  }
  
  return { isInternalTool: false, result: null };
}

export async function initInternalTools() {
  await MCPInstall.init();
  await MCPSearch.init();
  await MCPConfig.init();
}

export async function injectInternalTools(tools: Tools) {
  tools.tools.push(MCPInstall.tool);
  tools.tools.push(MCPSearch.tool);
  tools.tools.push(MCPConfig.tool);
}