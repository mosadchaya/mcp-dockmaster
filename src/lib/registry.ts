const availableTools: RegistryTool[] = [
  {
    id: "hello-world",
    name: "Hello World",
    description: "A simple tool that says hello world",
    publisher: {
      id: "shinkai-labs",
      name: "Shinkai Labs",
      url: "https://shinkai.com/",
    },
    isOfficial: false,
    sourceUrl: "https://github.com/modelcontextprotocol/servers/tree/main/src/hello-world",
    distribution: {
      type: "npm",
      package: "@shinkai-labs/hello-world",
    },
    license: "MIT",
    runtime: "uv",
    config: {
      command: "npx",
      args: ["-y", "@shinkai-labs/hello-world"],
      env: {}
    }
  },
  {
    id: "brave-search-ref",
    name: "Brave Search",
    description: "Web and local search using Brave's Search API. A Model Context Protocol reference server.",
    publisher: {
      id: "modelcontextprotocol",
      name: "Anthropic, PBC",
      url: "https://modelcontextprotocol.io/",
    },
    isOfficial: false,
    sourceUrl: "https://github.com/modelcontextprotocol/servers/tree/main/src/brave-search",
    distribution: {
      type: "npm",
      package: "@modelcontextprotocol/server-brave-search",
    },
    license: "MIT",
    runtime: "node",
    config: {
      command: "npx",
      args: ["-y", "@modelcontextprotocol/server-brave-search"],
      env: {
        "BRAVE_API_KEY": {
          description: "Your Brave Search API key. See: https://brave.com/search/api",
        }
      }
    }
  },
  {
    id: "github-ref",
    name: "GitHub",
    description: "GitHub repository access and management. A Model Context Protocol reference server.",
    publisher: {
      id: "modelcontextprotocol",
      name: "Anthropic, PBC",
      url: "https://modelcontextprotocol.io/",
    },
    isOfficial: false,
    sourceUrl: "https://github.com/modelcontextprotocol/servers/tree/main/src/github",
    distribution: {
      type: "npm",
      package: "@modelcontextprotocol/server-github",
    },
    license: "MIT",
    runtime: "node",
    config: {
      command: "npx",
      args: ["-y", "@modelcontextprotocol/server-github"],
      env: {
        "GITHUB_PERSONAL_ACCESS_TOKEN": {
          description: "Your GitHub Personal Access Token. Find it at: https://github.com/settings/tokens",
        }
      }
    }
  },
  {
    id: "memory-ref",
    name: "Memory",
    description: "Knowledge graph-based persistent memory system. A Model Context Protocol reference server.",
    publisher: {
      id: "modelcontextprotocol",
      name: "Anthropic, PBC",
      url: "https://modelcontextprotocol.io/",
    },
    isOfficial: false,
    sourceUrl: "https://github.com/modelcontextprotocol/servers/tree/main/src/memory",
    distribution: {
      type: "npm",
      package: "@modelcontextprotocol/server-memory",
    },
    license: "MIT",
    runtime: "node",
    config: {
      command: "npx",
      args: ["-y", "@modelcontextprotocol/server-memory"],
      env: {}
    }
  }
]

interface RegistryTool {
  id: string;
  name: string;
  description: string;
  publisher: {
    id: string;
    name: string;
    url: string;
  };
  isOfficial: boolean;
  sourceUrl: string;
  distribution: {
    type: string;
    package: string;
  };
  license: string;
  runtime: string;
  config: {
    command: string;
    args: string[];
    env: Record<string, any>;
  };
}

/**
 * Get all available tools from the registry
 */
export const getAvailableTools = async (): Promise<RegistryTool[]> => {
  return availableTools;
};

/**
 * Get a specific tool by ID
 */
export const getToolById = async (id: string): Promise<RegistryTool | null> => {
  return availableTools.find(tool => tool.id === id) || null;
};

export default {
  getAvailableTools,
  getToolById
};