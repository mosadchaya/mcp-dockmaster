const availableTools: RegistryTool[] = [
  {
    id: "helius-proxy",
    name: "Helius Proxy",
    description: "A simple tool that proxies requests to Helius",
    publisher: {
      id: "dcspark",
      name: "dcspark",
      url: "https://www.dcspark.com/",
    },
    isOfficial: true,
    sourceUrl: "https://github.com/dcspark/mcp-server-helius",
    distribution: {
      type: "npm",
      package: "@mcp-dockmaster/mcp-server-helius",
    },
    license: "MIT",
    runtime: "node",
    config: {
      command: "npx",
      args: ["-y", "@mcp-dockmaster/mcp-server-helius"],
      env: {
        "HELIUS_API_KEY": {
          description: "Your Helius API key. See: https://www.helius.xyz/api",
        }
      }
    }
  },
  // {
  //   id: "hello-world",
  //   name: "Hello World",
  //   description: "A simple tool that says hello world",
  //   publisher: {
  //     id: "dcspark",
  //     name: "dcspark",
  //     url: "https://www.dcspark.com/",
  //   },
  //   isOfficial: true,
  //   sourceUrl: "https://github.com/dcspark/mcp-server-hello-world",
  //   distribution: {
  //     type: "npm",
  //     package: "@mcp-dockmaster/mcp-server-hello-world",
  //   },
  //   license: "MIT",
  //   runtime: "node",
  //   config: {
  //     command: "node",
  //     "args": ["--experimental-modules", "--no-warnings", "../../../libs/mcp-core/tests/it/integration/mcp-server-hello-world/build/index.js"],
  //     "env": {}
  //   }
  // },
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
  },
  {
    id: "fetch",
    name: "Fetch manager",
    description: "Web content fetching and processing tool for efficient LLM usage.",
    publisher: {
      id: "modelcontextprotocol",
      name: "Anthropic, PBC",
      url: "https://modelcontextprotocol.io/",
    },
    isOfficial: false,
    sourceUrl: "https://github.com/modelcontextprotocol/servers/tree/main/src/fetch",
    distribution: {
      type: "python",
      package: "mcp-server-fetch",
    },
    license: "MIT",
    runtime: "python",
    config: {
      command: "uvx",
      args: ["mcp-server-fetch"],
      env: {}
    }
  },
  {
    id: "perplexity",
    name: "Perplexity",
    description: "Web search using Perplexity's API.",
    publisher: {
      id: "daniel-lxs",
      name: "Daniel Lxs",
      url: "https://github.com/daniel-lxs",
    },
    isOfficial: false,
    sourceUrl: "https://github.com/daniel-lxs/mcp-perplexity",
    distribution: {
      type: "python",
      package: "mcp-perplexity",
    },
    license: "MIT",
    runtime: "python",
    config: {
      command: "uvx",
      args: ["mcp-perplexity"],
      "env": {
        "PERPLEXITY_API_KEY": {
          description: "Your Perplexity API key. See: https://www.perplexity.ai/settings/api",
        },
        "PERPLEXITY_MODEL": {
          description: "The Perplexity model to use. See: https://www.perplexity.ai/settings/api",
          default: "sonar-pro"
        }
      }
    }
  },  
  {
    id: "fetch-docker",
    name: "Fetch manager (Docker)",
    description: "Web content fetching and processing tool for efficient LLM usage.",
    publisher: {
      id: "modelcontextprotocol",
      name: "Anthropic, PBC",
      url: "https://modelcontextprotocol.io/",
    },
    isOfficial: false,
    sourceUrl: "https://github.com/modelcontextprotocol/servers/tree/main/src/fetch",
    distribution: {
      type: "dockerhub",
      package: "mcp/fetch",
    },
    license: "MIT",
    runtime: "docker",
    config: {
      command: "docker",
      args: ["mcp/fetch", "--user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36", "--ignore-robots-txt"],
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