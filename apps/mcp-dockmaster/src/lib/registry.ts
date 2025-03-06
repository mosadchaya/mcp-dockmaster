const availableTools: RegistryTool[] = [
  {
    "id": "helius-proxy",
    "name": "Helius Proxy",
    "description": "A simple tool that proxies requests to Helius",
    "publisher": {
      "id": "dcspark",
      "name": "dcspark",
      "url": "https://www.dcspark.com/"
    },
    "isOfficial": true,
    "sourceUrl": "https://github.com/dcspark/mcp-server-helius",
    "distribution": {
      "type": "npm",
      "package": "@mcp-dockmaster/mcp-server-helius"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@mcp-dockmaster/mcp-server-helius"
      ],
      "env": {
        "HELIUS_API_KEY": {
          "required": true,
          "description": "Your Helius API key. See: https://www.helius.xyz/api"
        }
      }
    }
  },
  {
    "id": "e7851c24-a8d2-4396-8cec-482c8c9f7838",
    "name": "executeautomation/mcp-playwright",
    "description": "Playwright MCP Server",
    "publisher": {
      "id": "executeautomation",
      "name": "executeautomation",
      "url": "https://github.com/executeautomation/mcp-playwright"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/executeautomation/mcp-playwright",
    "distribution": {
      "type": "npm",
      "package": "@executeautomation/playwright-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@executeautomation/playwright-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "216924e1-afcc-4f77-99a8-4e1148865103",
    "name": "felores/placid-mcp-server",
    "description": "Placid.app MCP Server",
    "publisher": {
      "id": "felores",
      "name": "felores",
      "url": "https://github.com/felores/placid-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/felores/placid-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@felores/placid-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@felores/placid-mcp-server"
      ],
      "env": {
        "PLACID_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "678f1b69-8b7a-46d2-8f69-a494a4938f0d",
    "name": "johnnyoshika/mcp-server-sqlite-npx",
    "description": "MCP SQLite Server",
    "publisher": {
      "id": "johnnyoshika",
      "name": "johnnyoshika",
      "url": "https://github.com/johnnyoshika/mcp-server-sqlite-npx"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/johnnyoshika/mcp-server-sqlite-npx",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-sqlite-npx"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-sqlite-npx",
        "$ENVARG_DATABASE_PATH"
      ],
      "env": {
        "ENVARG_DATABASE_PATH": {
          "description": "Path to the SQLite database file",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "28318514-0043-47b1-9fc1-665e84dc58d4",
    "name": "Rickyyy1116/mcp-youtube-sheets",
    "description": "YouTube to Google Sheets MCP Server",
    "publisher": {
      "id": "rikukawa",
      "name": "rikukawa",
      "url": "https://github.com/Rickyyy1116/mcp-youtube-sheets"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Rickyyy1116/mcp-youtube-sheets",
    "distribution": {
      "type": "npm",
      "package": "@rikukawa/youtube-sheets-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@rikukawa/youtube-sheets-server"
      ],
      "env": {
        "YOUTUBE_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "SPREADSHEET_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "fb32b683-ef21-4eda-984a-0036765b4f2e",
    "name": "crazyrabbitLTC/mcp-etherscan-server",
    "description": "MCP Etherscan Server",
    "publisher": {
      "id": "crazyrabbitLTC",
      "name": "crazyrabbitLTC",
      "url": "https://github.com/crazyrabbitLTC/mcp-etherscan-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/crazyrabbitLTC/mcp-etherscan-server",
    "distribution": {
      "type": "npm",
      "package": "mcp-etherscan-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-etherscan-server"
      ],
      "env": {
        "ETHERSCAN_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "b9e1c302-e105-4b8d-ba10-178df1ac276d",
    "name": "sammcj/mcp-package-version",
    "description": "Package Version MCP Server",
    "publisher": {
      "id": "sammcj",
      "name": "sammcj",
      "url": "https://github.com/sammcj/mcp-package-version"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/sammcj/mcp-package-version",
    "distribution": {
      "type": "npm",
      "package": "mcp-package-version"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-package-version"
      ],
      "env": {
        "NODE_EXTRA_CA_CERTS": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "fd8463d5-ec76-43ef-9bf0-f91435f5c646",
    "name": "tatn/mcp-server-diff-python",
    "description": "mcp-server-diff-python",
    "publisher": {
      "id": "tatn",
      "name": "tatn",
      "url": "https://github.com/tatn/mcp-server-diff-python"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tatn/mcp-server-diff-python",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-diff-python"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-diff-python"
      ],
      "env": {}
    }
  },
  {
    "id": "4dcd214b-8987-41ab-a47d-443fb568dc10",
    "name": "cr7258/elasticsearch-mcp-server",
    "description": "Elasticsearch MCP Server",
    "publisher": {
      "id": "cr7258",
      "name": "cr7258",
      "url": "https://github.com/cr7258/elasticsearch-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/cr7258/elasticsearch-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "elasticsearch-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "elasticsearch-mcp-server"
      ],
      "env": {
        "ELASTIC_HOST": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ELASTIC_USERNAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ELASTIC_PASSWORD": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "4dec19cd-da8b-4156-bd3d-a8dc5a63a8b7",
    "name": "kennethreitz/mcp-applemusic",
    "description": "MCP-AppleMusic",
    "publisher": {
      "id": "kennethreitz",
      "name": "kennethreitz",
      "url": "https://github.com/kennethreitz/mcp-applemusic"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kennethreitz/mcp-applemusic",
    "distribution": {
      "type": "pip",
      "package": "mcp-applemusic"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-applemusic"
      ],
      "env": {}
    }
  },
  {
    "id": "ca64b556-b14b-4e50-9a3c-b124970ba378",
    "name": "liuyoshio/mcp-compass",
    "description": "MCP Compass 🧭",
    "publisher": {
      "id": "liuyoshio",
      "name": "liuyoshio",
      "url": "https://github.com/liuyoshio/mcp-compass"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/liuyoshio/mcp-compass",
    "distribution": {
      "type": "npm",
      "package": "@liuyoshio/mcp-compass"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@liuyoshio/mcp-compass"
      ],
      "env": {}
    }
  },
  {
    "id": "311f036a-6a5a-42aa-95fa-475329113bbf",
    "name": "kazuph/mcp-taskmanager",
    "description": "MCP TaskManager",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-taskmanager"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-taskmanager",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-taskmanager"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-taskmanager"
      ],
      "env": {}
    }
  },
  {
    "id": "675119e1-7d7e-4aa7-832a-43dc105fff58",
    "name": "server-slack",
    "description": "Slack MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/slack"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/slack",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-slack"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-slack"
      ],
      "env": {
        "SLACK_BOT_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "SLACK_TEAM_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "c708034e-75e5-4767-b2d4-822964b19498",
    "name": "QuantGeekDev/coincap-mcp",
    "description": "Coincap MCP",
    "publisher": {
      "id": "QuantGeekDev",
      "name": "QuantGeekDev",
      "url": "https://github.com/QuantGeekDev/coincap-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/QuantGeekDev/coincap-mcp",
    "distribution": {
      "type": "npm",
      "package": "coincap-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "coincap-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "3d7ee3fb-7886-4e74-8463-ad37f288a3f9",
    "name": "tadasant/mcp-server-ssh-rails-runner",
    "description": "MCP Server: SSH Rails Runner",
    "publisher": {
      "id": "tadasant",
      "name": "tadasant",
      "url": "https://github.com/tadasant/mcp-server-ssh-rails-runner"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tadasant/mcp-server-ssh-rails-runner",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-ssh-rails-runner"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-ssh-rails-runner"
      ],
      "env": {
        "SSH_HOST": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "SSH_USER": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "SSH_PRIVATE_KEY_PATH": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "RAILS_WORKING_DIR": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "98d89cab-a857-4813-ab93-6535294d26f2",
    "name": "mcp-server-sqlite",
    "description": "SQLite MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "Model Context Protocol",
      "url": "https://pypi.org/project/mcp-server-sqlite/"
    },
    "isOfficial": false,
    "sourceUrl": "https://pypi.org/project/mcp-server-sqlite/",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-sqlite"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-sqlite",
        "--db-path",
        "$ENVARG_DATABASE_PATH"
      ],
      "env": {
        "ENVARG_DATABASE_PATH": {
          "description": "Path to the SQLite database file.",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "3992b704-c0a8-4567-b852-f010aba64527",
    "name": "sammcj/mcp-github-issue",
    "description": "MCP GitHub Issue Server",
    "publisher": {
      "id": "sammcj",
      "name": "sammcj",
      "url": "https://github.com/sammcj/mcp-github-issue"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/sammcj/mcp-github-issue",
    "distribution": {
      "type": "npm",
      "package": "mcp-github-issue"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-github-issue"
      ],
      "env": {}
    }
  },
  {
    "id": "f8f72cde-448f-4254-aff7-15a693191f22",
    "name": "bigcodegen/mcp-neovim-server",
    "description": "Neovim MCP Server",
    "publisher": {
      "id": "bigcodegen",
      "name": "bigcodegen",
      "url": "https://github.com/bigcodegen/mcp-neovim-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/bigcodegen/mcp-neovim-server",
    "distribution": {
      "type": "npm",
      "package": "mcp-neovim-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-neovim-server"
      ],
      "env": {}
    }
  },
  {
    "id": "dcf44fdf-28ed-40ba-938b-cf211f17dabc",
    "name": "esignaturescom/mcp-server-esignatures",
    "description": "mcp-server-esignatures MCP server",
    "publisher": {
      "id": "esignaturescom",
      "name": "esignaturescom",
      "url": "https://github.com/esignaturescom/mcp-server-esignatures"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/esignaturescom/mcp-server-esignatures",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-esignatures"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-esignatures"
      ],
      "env": {
        "ESIGNATURES_SECRET_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "5a8da77c-7bfe-458a-ac75-e7f7992e58fd",
    "name": "jsonallen/perplexity-mcp",
    "description": "perplexity-mcp MCP server",
    "publisher": {
      "id": "jsonallen",
      "name": "jsonallen",
      "url": "https://github.com/jsonallen/perplexity-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/jsonallen/perplexity-mcp",
    "distribution": {
      "type": "pip",
      "package": "perplexity-mcp"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "perplexity-mcp"
      ],
      "env": {
        "PERPLEXITY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "9068f336-041e-403f-885f-0eb52ef4d586",
    "name": "snaggle-ai/openapi-mcp-server",
    "description": "OpenAPI MCP Server",
    "publisher": {
      "id": "snaggle-ai",
      "name": "snaggle-ai",
      "url": "https://github.com/snaggle-ai/openapi-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/snaggle-ai/openapi-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "openapi-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "openapi-mcp-server",
        "$ENVARG_OPENAPI_SPECIFICATION_PATH"
      ],
      "env": {
        "ENVARG_OPENAPI_SPECIFICATION_PATH": {
          "description": "Path to the OpenAPI v3.1 specification file (JSON or YAML)",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "a8655661-db98-44d7-a8da-02b0271142bf",
    "name": "spences10/mcp-sequentialthinking-tools",
    "description": "mcp-sequentialthinking-tools",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-sequentialthinking-tools"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-sequentialthinking-tools",
    "distribution": {
      "type": "npm",
      "package": "mcp-sequentialthinking-tools"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-sequentialthinking-tools"
      ],
      "env": {}
    }
  },
  {
    "id": "bfb46c45-aa27-4d17-b7ae-0ad3e88e1669",
    "name": "Automata-Labs-team/MCP-Server-Playwright",
    "description": "A Model Context Protocol server that provides browser automation capabilities using Playwright.",
    "publisher": {
      "id": "automatalabs",
      "name": "automatalabs",
      "url": "https://github.com/Automata-Labs-team/MCP-Server-Playwright"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Automata-Labs-team/MCP-Server-Playwright",
    "distribution": {
      "type": "npm",
      "package": "@automatalabs/mcp-server-playwright"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@automatalabs/mcp-server-playwright"
      ],
      "env": {}
    }
  },
  {
    "id": "3af06c10-9d8d-4451-bea4-36a30667be0f",
    "name": "crazyrabbitLTC/mcp-expert-server",
    "description": "MCP Expert Server",
    "publisher": {
      "id": "crazyrabbitLTC",
      "name": "crazyrabbitLTC",
      "url": "https://github.com/crazyrabbitLTC/mcp-expert-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/crazyrabbitLTC/mcp-expert-server",
    "distribution": {
      "type": "npm",
      "package": "mcp-expert-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-expert-server"
      ],
      "env": {
        "ANTHROPIC_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "aadd6c3b-5109-41f9-86f8-0d68c9fd5fc8",
    "name": "ktanaka101/mcp-server-duckdb",
    "description": "mcp-server-duckdb",
    "publisher": {
      "id": "ktanaka101",
      "name": "ktanaka101",
      "url": "https://github.com/ktanaka101/mcp-server-duckdb"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ktanaka101/mcp-server-duckdb",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-duckdb"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-duckdb",
        "--db-path",
        "$ENVARG_DATABASE_PATH"
      ],
      "env": {
        "ENVARG_DATABASE_PATH": {
          "description": "Path to the DuckDB database file. The server will automatically create the database file and parent directories if they don't exist.",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "f987f99a-0be1-40bc-9f8f-ff2a48dc270b",
    "name": "SecretiveShell/MCP-timeserver",
    "description": "MCP-timeserver",
    "publisher": {
      "id": "SecretiveShell",
      "name": "SecretiveShell",
      "url": "https://github.com/SecretiveShell/MCP-timeserver"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/SecretiveShell/MCP-timeserver",
    "distribution": {
      "type": "pip",
      "package": "mcp-timeserver"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-timeserver"
      ],
      "env": {}
    }
  },
  {
    "id": "437c3fc1-828b-4e32-bb59-ab9ef7dcf394",
    "name": "kaliaboi/mcp-zotero",
    "description": "MCP Zotero",
    "publisher": {
      "id": "kaliaboi",
      "name": "kaliaboi",
      "url": "https://github.com/kaliaboi/mcp-zotero"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kaliaboi/mcp-zotero",
    "distribution": {
      "type": "npm",
      "package": "mcp-zotero"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-zotero"
      ],
      "env": {
        "ZOTERO_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ZOTERO_USER_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "ecb90629-0100-44f0-849b-4dea77f41b73",
    "name": "skydeckai/mcp-server-aidd",
    "description": "AiDD MCP Server",
    "publisher": {
      "id": "skydeckai",
      "name": "skydeckai",
      "url": "https://github.com/skydeckai/mcp-server-aidd"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/skydeckai/mcp-server-aidd",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-aidd"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-aidd"
      ],
      "env": {}
    }
  },
  {
    "id": "69c7bf35-95e2-48ff-85c4-cad855a3e698",
    "name": "sooperset/mcp-atlassian",
    "description": "MCP Atlassian",
    "publisher": {
      "id": "sooperset",
      "name": "sooperset",
      "url": "https://github.com/sooperset/mcp-atlassian"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/sooperset/mcp-atlassian",
    "distribution": {
      "type": "pip",
      "package": "mcp-atlassian"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-atlassian"
      ],
      "env": {
        "CONFLUENCE_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "CONFLUENCE_USERNAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "CONFLUENCE_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "JIRA_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "JIRA_USERNAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "JIRA_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "88dfb5f6-1e64-4394-a27c-32c378ab899a",
    "name": "GongRzhe/Calendar-Autoauth-MCP-Server",
    "description": "Calendar AutoAuth MCP Server",
    "publisher": {
      "id": "gongrzhe",
      "name": "gongrzhe",
      "url": "https://github.com/GongRzhe/Calendar-Autoauth-MCP-Server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/GongRzhe/Calendar-Autoauth-MCP-Server",
    "distribution": {
      "type": "npm",
      "package": "@gongrzhe/server-calendar-autoauth-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@gongrzhe/server-calendar-autoauth-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "2e058a7f-5a1b-4903-8984-226335dbc143",
    "name": "BurtTheCoder/mcp-shodan",
    "description": "Shodan MCP Server",
    "publisher": {
      "id": "burtthecoder",
      "name": "burtthecoder",
      "url": "https://github.com/BurtTheCoder/mcp-shodan"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/BurtTheCoder/mcp-shodan",
    "distribution": {
      "type": "npm",
      "package": "@burtthecoder/mcp-shodan"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@burtthecoder/mcp-shodan"
      ],
      "env": {
        "SHODAN_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "3850c614-daa6-4c49-a463-29adf7392ecd",
    "name": "kazuph/mcp-obsidian",
    "description": "MCP Obsidian",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-obsidian"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-obsidian",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-obsidian"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-obsidian"
      ],
      "env": {
        "OBSIDIAN_VAULT_PATH": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "15de3fb3-2058-4237-8d01-76a24e22075c",
    "name": "server-postgres",
    "description": "PostgreSQL",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/postgres"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/postgres",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-postgres"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-postgres",
        "$ENVARG_CONNECTION_URL"
      ],
      "env": {
        "ENVARG_CONNECTION_URL": {
          "description": "PostgreSQL connection string",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "c957ba24-5853-41b3-bc3c-4931da0ff81a",
    "name": "zxkane/mcp-server-amazon-bedrock",
    "description": "Amazon Bedrock MCP Server",
    "publisher": {
      "id": "zxkane",
      "name": "zxkane",
      "url": "https://github.com/zxkane/mcp-server-amazon-bedrock"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/zxkane/mcp-server-amazon-bedrock",
    "distribution": {
      "type": "npm",
      "package": "@zxkane/mcp-server-amazon-bedrock"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@zxkane/mcp-server-amazon-bedrock"
      ],
      "env": {
        "AWS_ACCESS_KEY_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "AWS_SECRET_ACCESS_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "AWS_REGION": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "AWS_PROFILE": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "9f2b0e2b-f13d-4cad-8549-973ad1e75e0f",
    "name": "sirmews/apple-notes-mcp",
    "description": "Apple Notes Model Context Protocol Server for Claude Desktop.",
    "publisher": {
      "id": "sirmews",
      "name": "sirmews",
      "url": "https://github.com/sirmews/apple-notes-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/sirmews/apple-notes-mcp",
    "distribution": {
      "type": "pip",
      "package": "apple-notes-mcp"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "apple-notes-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "aeb1802e-f072-40c6-9def-9f52ea14d429",
    "name": "server-everart",
    "description": "EverArt MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/everart"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/everart",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-everart"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-everart"
      ],
      "env": {
        "EVERART_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "3e8aa07c-36f1-4759-8282-30a170267ce5",
    "name": "dkmaker/mcp-azure-tablestorage",
    "description": "Azure TableStore MCP Server",
    "publisher": {
      "id": "dkmaker",
      "name": "dkmaker",
      "url": "https://github.com/dkmaker/mcp-azure-tablestorage"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/dkmaker/mcp-azure-tablestorage",
    "distribution": {
      "type": "npm",
      "package": "dkmaker-mcp-server-tablestore"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "dkmaker-mcp-server-tablestore"
      ],
      "env": {
        "AZURE_STORAGE_CONNECTION_STRING": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "8e8848e0-68f8-4cb3-9340-a11e7e920c52",
    "name": "gerred/mcpmc",
    "description": "MCPMC (Minecraft Model Context Protocol)",
    "publisher": {
      "id": "gerred",
      "name": "gerred",
      "url": "https://github.com/gerred/mcpmc"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/gerred/mcpmc",
    "distribution": {
      "type": "npm",
      "package": "@gerred/mcpmc"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@gerred/mcpmc"
      ],
      "env": {}
    }
  },
  {
    "id": "5616ffdf-2b6f-4e40-af87-616c3f66f4ab",
    "name": "emzimmer/server-moz-readability",
    "description": "Mozilla Readability Parser MCP Server",
    "publisher": {
      "id": "emzimmer",
      "name": "emzimmer",
      "url": "https://github.com/emzimmer/server-moz-readability"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/emzimmer/server-moz-readability",
    "distribution": {
      "type": "npm",
      "package": "server-moz-readability"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "server-moz-readability"
      ],
      "env": {}
    }
  },
  {
    "id": "593de79f-1f8b-4a92-93e6-b6af59b705bc",
    "name": "qdrant/mcp-server-qdrant",
    "description": "mcp-server-qdrant: A Qdrant MCP server",
    "publisher": {
      "id": "qdrant",
      "name": "qdrant",
      "url": "https://github.com/qdrant/mcp-server-qdrant"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/qdrant/mcp-server-qdrant",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-qdrant"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-qdrant"
      ],
      "env": {
        "QDRANT_URL": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "QDRANT_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "COLLECTION_NAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "FASTEMBED_MODEL_NAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "QDRANT_LOCAL_PATH": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "60fd99bd-2a69-46bc-99c5-52ca47ff3366",
    "name": "andybrandt/mcp-simple-timeserver",
    "description": "MCP Simple Timeserver",
    "publisher": {
      "id": "andybrandt",
      "name": "andybrandt",
      "url": "https://github.com/andybrandt/mcp-simple-timeserver"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/andybrandt/mcp-simple-timeserver",
    "distribution": {
      "type": "pip",
      "package": "mcp-simple-timeserver"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-simple-timeserver"
      ],
      "env": {}
    }
  },
  {
    "id": "eb4dec2d-ba85-45fa-9df7-db5fb902b55f",
    "name": "crazyrabbitLTC/mcp-twitter-server",
    "description": "Twitter MCP Server",
    "publisher": {
      "id": "crazyrabbitltc",
      "name": "crazyrabbitltc",
      "url": "https://github.com/crazyrabbitLTC/mcp-twitter-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/crazyrabbitLTC/mcp-twitter-server",
    "distribution": {
      "type": "npm",
      "package": "@crazyrabbitltc/mcp-twitter-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@crazyrabbitltc/mcp-twitter-server"
      ],
      "env": {
        "X_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "X_API_SECRET": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "X_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "X_ACCESS_TOKEN_SECRET": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "3053491c-049e-4489-a032-e723dc8882cc",
    "name": "BurtTheCoder/mcp-virustotal",
    "description": "VirusTotal MCP Server",
    "publisher": {
      "id": "burtthecoder",
      "name": "burtthecoder",
      "url": "https://github.com/BurtTheCoder/mcp-virustotal"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/BurtTheCoder/mcp-virustotal",
    "distribution": {
      "type": "npm",
      "package": "@burtthecoder/mcp-virustotal"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@burtthecoder/mcp-virustotal"
      ],
      "env": {
        "VIRUSTOTAL_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "1e78692f-8f11-49a2-9923-5dc0ac20c2d8",
    "name": "Bigsy/Clojars-MCP-Server",
    "description": "Clojars MCP Server",
    "publisher": {
      "id": "Bigsy",
      "name": "Bigsy",
      "url": "https://github.com/Bigsy/Clojars-MCP-Server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Bigsy/Clojars-MCP-Server",
    "distribution": {
      "type": "npm",
      "package": "clojars-deps-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "clojars-deps-server"
      ],
      "env": {}
    }
  },
  {
    "id": "d12d1582-6d60-4b16-b8fe-ec85a34a9d54",
    "name": "turlockmike/apple-notifier-mcp",
    "description": "Apple Notifier MCP Server",
    "publisher": {
      "id": "turlockmike",
      "name": "turlockmike",
      "url": "https://github.com/turlockmike/apple-notifier-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/turlockmike/apple-notifier-mcp",
    "distribution": {
      "type": "npm",
      "package": "apple-notifier-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "apple-notifier-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "b826f448-8642-4f2b-a85e-c5f505238d34",
    "name": "r-huijts/ns-mcp-server",
    "description": "NS Travel Information MCP Server",
    "publisher": {
      "id": "r-huijts",
      "name": "r-huijts",
      "url": "https://github.com/r-huijts/ns-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/r-huijts/ns-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "ns-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "ns-mcp-server"
      ],
      "env": {
        "NS_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "92cf4e9d-fb8e-4019-bd13-d37919ecac16",
    "name": "tinybirdco/mcp-tinybird",
    "description": "Tinybird MCP server",
    "publisher": {
      "id": "tinybirdco",
      "name": "tinybirdco",
      "url": "https://github.com/tinybirdco/mcp-tinybird"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tinybirdco/mcp-tinybird",
    "distribution": {
      "type": "pip",
      "package": "mcp-tinybird"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-tinybird",
        "stdio"
      ],
      "env": {
        "TB_API_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "TB_ADMIN_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "9fdc7032-70f3-430d-990f-2870b1d1a49d",
    "name": "tatn/mcp-server-diff-typescript",
    "description": "mcp-server-diff-typescript MCP Server",
    "publisher": {
      "id": "tatn",
      "name": "tatn",
      "url": "https://github.com/tatn/mcp-server-diff-typescript"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tatn/mcp-server-diff-typescript",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-diff-typescript"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-diff-typescript"
      ],
      "env": {}
    }
  },
  {
    "id": "597791f4-c1ac-414a-9a92-dcdbe0767179",
    "name": "server-github",
    "description": "GitHub MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/github"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/github",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-github"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-github"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "173c77d2-1ad6-4b14-b92e-b77edb848bd8",
    "name": "DMontgomery40/deepseek-mcp-server",
    "description": "DeepSeek MCP Server",
    "publisher": {
      "id": "DMontgomery40",
      "name": "DMontgomery40",
      "url": "https://github.com/DMontgomery40/deepseek-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/DMontgomery40/deepseek-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "deepseek-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "deepseek-mcp-server"
      ],
      "env": {
        "DEEPSEEK_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "4064cdca-d8e6-4860-a4ad-3546f70bdc0e",
    "name": "yosider/cosense-mcp-server",
    "description": "Cosense MCP Server",
    "publisher": {
      "id": "yosider",
      "name": "yosider",
      "url": "https://github.com/yosider/cosense-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/yosider/cosense-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@yosider/cosense-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@yosider/cosense-mcp-server"
      ],
      "env": {
        "COSENSE_PROJECT_NAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "COSENSE_SID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "b751536c-5b7f-4941-859b-b839553f37e2",
    "name": "filesystem",
    "description": "Filesystem MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/filesystem"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/filesystem",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-filesystem"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-filesystem",
        "$ENVARG_ALLOWED_DIRECTORIES"
      ],
      "env": {
        "ENVARG_ALLOWED_DIRECTORIES": {
          "description": "List of directories allowed to be accessed by this mcp server, one path per line.",
          "type": "array",
          "required": true
        }
      }
    }
  },
  {
    "id": "985017cd-94db-42b5-a842-af109bbb1a7c",
    "name": "spences10/mcp-wsl-exec",
    "description": "mcp-wsl-exec",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-wsl-exec"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-wsl-exec",
    "distribution": {
      "type": "npm",
      "package": "mcp-wsl-exec"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-wsl-exec"
      ],
      "env": {}
    }
  },
  {
    "id": "fe11ebc9-4fa4-4fea-b3ae-06512de88df6",
    "name": "qpd-v/mcp-guide",
    "description": "MCP Guide Server (v0.1.5)",
    "publisher": {
      "id": "qpd-v",
      "name": "qpd-v",
      "url": "https://github.com/qpd-v/mcp-guide"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/qpd-v/mcp-guide",
    "distribution": {
      "type": "npm",
      "package": "mcp-guide"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-guide"
      ],
      "env": {}
    }
  },
  {
    "id": "2e7239ad-1db6-409f-83a6-953575647027",
    "name": "aws-kb-retrieval",
    "description": "AWS Knowledge Base Retrieval MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/aws-kb-retrieval-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/aws-kb-retrieval-server",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-aws-kb-retrieval"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-aws-kb-retrieval"
      ],
      "env": {
        "AWS_ACCESS_KEY_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "AWS_SECRET_ACCESS_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "AWS_REGION": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "7a2930e1-5c10-4df9-ac92-af77df5b94c2",
    "name": "sequential-thinking",
    "description": "Sequential Thinking MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/sequentialthinking"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/sequentialthinking",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-sequential-thinking"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-sequential-thinking"
      ],
      "env": {}
    }
  },
  {
    "id": "fdcb20eb-67b5-49a4-a10b-23126455b55a",
    "name": "cyanheads/obsidian-mcp-server",
    "description": "Obsidian MCP Server",
    "publisher": {
      "id": "cyanheads",
      "name": "cyanheads",
      "url": "https://github.com/cyanheads/obsidian-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/cyanheads/obsidian-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "obsidian-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "obsidian-mcp-server"
      ],
      "env": {
        "OBSIDIAN_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "VERIFY_SSL": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "OBSIDIAN_PROTOCOL": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "OBSIDIAN_HOST": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "OBSIDIAN_PORT": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "REQUEST_TIMEOUT": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "MAX_CONTENT_LENGTH": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "MAX_BODY_LENGTH": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "RATE_LIMIT_WINDOW_MS": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "RATE_LIMIT_MAX_REQUESTS": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "TOOL_TIMEOUT_MS": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "13a5740d-2aa1-4635-b6c4-566e0ab4c2d7",
    "name": "shannonlal/mcp-postman",
    "description": "Postman MCP Server",
    "publisher": {
      "id": "shannonlal",
      "name": "shannonlal",
      "url": "https://github.com/shannonlal/mcp-postman"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/shannonlal/mcp-postman",
    "distribution": {
      "type": "npm",
      "package": "mcp-postman"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-postman"
      ],
      "env": {}
    }
  },
  {
    "id": "88259707-0eaa-4e63-ba60-1d81ce937df0",
    "name": "brave-search",
    "description": "Brave Search MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://www.npmjs.com/package/@modelcontextprotocol/server-brave-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://www.npmjs.com/package/@modelcontextprotocol/server-brave-search",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-brave-search"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-brave-search"
      ],
      "env": {
        "BRAVE_API_KEY": {
          "description": "Your Brave Search API key. See: https://brave.com/search/api/",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "6b4f2082-98c2-4731-9fec-52f9b18b1610",
    "name": "ravenwits/mcp-server-arangodb",
    "description": "MCP Server for ArangoDB",
    "publisher": {
      "id": "ravenwits",
      "name": "ravenwits",
      "url": "https://github.com/ravenwits/mcp-server-arangodb"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ravenwits/mcp-server-arangodb",
    "distribution": {
      "type": "npm",
      "package": "arango-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "arango-server"
      ],
      "env": {
        "ARANGO_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ARANGO_DATABASE": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ARANGO_USERNAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ARANGO_PASSWORD": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "17ce9dbf-2a26-4935-b098-f9e31d04ba2e",
    "name": "Flux159/mcp-server-kubernetes",
    "description": "mcp-server-kubernetes",
    "publisher": {
      "id": "Flux159",
      "name": "Flux159",
      "url": "https://github.com/Flux159/mcp-server-kubernetes"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Flux159/mcp-server-kubernetes",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-kubernetes"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-kubernetes"
      ],
      "env": {}
    }
  },
  {
    "id": "cf108a72-aafa-48e4-b085-63d5d8b8e416",
    "name": "kimtaeyoon83/mcp-server-youtube-transcript",
    "description": "YouTube Transcript Server",
    "publisher": {
      "id": "kimtaeyoon83",
      "name": "kimtaeyoon83",
      "url": "https://github.com/kimtaeyoon83/mcp-server-youtube-transcript"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kimtaeyoon83/mcp-server-youtube-transcript",
    "distribution": {
      "type": "npm",
      "package": "@kimtaeyoon83/mcp-server-youtube-transcript"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kimtaeyoon83/mcp-server-youtube-transcript"
      ],
      "env": {}
    }
  },
  {
    "id": "3c8840e6-c883-4cca-b531-e544ed2e550c",
    "name": "suekou/mcp-notion-server",
    "description": "Notion MCP Server",
    "publisher": {
      "id": "suekou",
      "name": "suekou",
      "url": "https://github.com/suekou/mcp-notion-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/suekou/mcp-notion-server",
    "distribution": {
      "type": "npm",
      "package": "@suekou/mcp-notion-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@suekou/mcp-notion-server"
      ],
      "env": {
        "NOTION_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "9bfb2d78-ac1c-4780-8bc7-1aabb2757ff4",
    "name": "d-kimuson/esa-mcp-server",
    "description": "esa-mcp-server",
    "publisher": {
      "id": "d-kimuson",
      "name": "d-kimuson",
      "url": "https://github.com/d-kimuson/esa-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/d-kimuson/esa-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "esa-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "esa-mcp-server"
      ],
      "env": {
        "ESA_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "DEFAULT_ESA_TEAM": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "af0000f7-391e-4d85-88e6-e8064bda31c4",
    "name": "kazuph/mcp-screenshot",
    "description": "MCP Screenshot",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-screenshot"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-screenshot",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-screenshot"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-screenshot"
      ],
      "env": {
        "OCR_API_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "66f5ac2a-7225-448d-bd67-e7f6e69311a6",
    "name": "StevenStavrakis/obsidian-mcp",
    "description": "Obsidian MCP Server",
    "publisher": {
      "id": "StevenStavrakis",
      "name": "StevenStavrakis",
      "url": "https://github.com/StevenStavrakis/obsidian-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/StevenStavrakis/obsidian-mcp",
    "distribution": {
      "type": "npm",
      "package": "obsidian-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "obsidian-mcp",
        "$ENVARG_VAULT_PATHS"
      ],
      "env": {
        "ENVARG_VAULT_PATHS": {
          "description": "List of absolute paths to your Obsidian vaults, one path per line",
          "type": "array",
          "required": true
        }
      }
    }
  },
  {
    "id": "c18591e6-d3fe-497d-b753-b92e2e5939fd",
    "name": "amidabuddha/unichat-mcp-server",
    "description": "Unichat MCP Server in Python",
    "publisher": {
      "id": "amidabuddha",
      "name": "amidabuddha",
      "url": "https://github.com/amidabuddha/unichat-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/amidabuddha/unichat-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "unichat-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "unichat-mcp-server"
      ],
      "env": {
        "UNICHAT_MODEL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "UNICHAT_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "a5255739-d117-496d-be80-ae804cdc2ecd",
    "name": "delorenj/mcp-qdrant-memory",
    "description": "MCP Memory Server with Qdrant Persistence",
    "publisher": {
      "id": "delorenj",
      "name": "delorenj",
      "url": "https://github.com/delorenj/mcp-qdrant-memory"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/delorenj/mcp-qdrant-memory",
    "distribution": {
      "type": "npm",
      "package": "mcp-qdrant-memory"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-qdrant-memory"
      ],
      "env": {
        "OPENAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "QDRANT_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "QDRANT_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "QDRANT_COLLECTION_NAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "5f4f8328-32da-4cd0-9f25-a5030b2653e8",
    "name": "qpd-v/mcp-delete",
    "description": "@qpd-v/mcp-delete",
    "publisher": {
      "id": "qpd-v",
      "name": "qpd-v",
      "url": "https://github.com/qpd-v/mcp-delete"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/qpd-v/mcp-delete",
    "distribution": {
      "type": "npm",
      "package": "@qpd-v/mcp-delete"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@qpd-v/mcp-delete"
      ],
      "env": {}
    }
  },
  {
    "id": "b50e8180-e014-47e1-a302-6a9785b6e148",
    "name": "exa-labs/exa-mcp-server",
    "description": "Exa MCP Server 🔍",
    "publisher": {
      "id": "exa-labs",
      "name": "exa-labs",
      "url": "https://github.com/exa-labs/exa-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/exa-labs/exa-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "exa-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "exa-mcp-server"
      ],
      "env": {
        "EXA_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "f72f502c-26c5-4295-abfe-6cfd6dcb82d8",
    "name": "MladenSU/cli-mcp-server",
    "description": "CLI MCP Server",
    "publisher": {
      "id": "MladenSU",
      "name": "MladenSU",
      "url": "https://github.com/MladenSU/cli-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/MladenSU/cli-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "cli-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "cli-mcp-server"
      ],
      "env": {
        "ALLOWED_DIR": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ALLOWED_COMMANDS": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ALLOWED_FLAGS": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "MAX_COMMAND_LENGTH": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "COMMAND_TIMEOUT": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "80f7569f-b807-49bc-ac46-7f69627be31c",
    "name": "meilisearch/meilisearch-mcp",
    "description": "Meilisearch MCP Server",
    "publisher": {
      "id": "meilisearch",
      "name": "meilisearch",
      "url": "https://github.com/meilisearch/meilisearch-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/meilisearch/meilisearch-mcp",
    "distribution": {
      "type": "pip",
      "package": "meilisearch-mcp"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "meilisearch-mcp"
      ],
      "env": {
        "MEILI_HTTP_ADDR": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MEILI_MASTER_KEY": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "dc2764b3-d999-4142-9a59-fa53f4cbfab4",
    "name": "delorenj/mcp-server-ticketmaster",
    "description": "MCP Server for Ticketmaster",
    "publisher": {
      "id": "delorenj",
      "name": "delorenj",
      "url": "https://github.com/delorenj/mcp-server-ticketmaster"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/delorenj/mcp-server-ticketmaster",
    "distribution": {
      "type": "npm",
      "package": "@delorenj/mcp-server-ticketmaster"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@delorenj/mcp-server-ticketmaster"
      ],
      "env": {
        "TICKETMASTER_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "fa0f56a3-d93c-4a5f-a82a-78a8799418dc",
    "name": "google-maps",
    "description": "Google Maps MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/google-maps"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/google-maps",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-google-maps"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-google-maps"
      ],
      "env": {
        "GOOGLE_MAPS_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "9a4fca25-e9ca-4c12-b2ab-57e623750dab",
    "name": "mcp-server-git",
    "description": "mcp-server-git: A git MCP server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "Model Context Protocol",
      "url": "https://pypi.org/project/mcp-server-git/"
    },
    "isOfficial": false,
    "sourceUrl": "https://pypi.org/project/mcp-server-git/",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-git"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-git"
      ],
      "env": {}
    }
  },
  {
    "id": "25249251-856f-4030-890c-3d6f1b1f8e9e",
    "name": "Text2Go/ai-humanizer-mcp-server",
    "description": "AI Humanize MCP Server",
    "publisher": {
      "id": "Text2Go",
      "name": "Text2Go",
      "url": "https://github.com/Text2Go/ai-humanizer-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Text2Go/ai-humanizer-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "ai-humanizer-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "ai-humanizer-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "0c92c442-45bc-47d3-b7d8-547acf22880d",
    "name": "qpd-v/mcp-DEEPwebresearch",
    "description": "MCP Deep Web Research Server (v0.3.0)",
    "publisher": {
      "id": "qpd-v",
      "name": "qpd-v",
      "url": "https://github.com/qpd-v/mcp-DEEPwebresearch"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/qpd-v/mcp-DEEPwebresearch",
    "distribution": {
      "type": "npm",
      "package": "mcp-deepwebresearch"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-deepwebresearch"
      ],
      "env": {}
    }
  },
  {
    "id": "acfd7d78-2f8b-400f-80e0-bcebbefa5e01",
    "name": "qpd-v/mcp-communicator-telegram",
    "description": "MCP Communicator (Telegram)",
    "publisher": {
      "id": "qpd-v",
      "name": "qpd-v",
      "url": "https://github.com/qpd-v/mcp-communicator-telegram"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/qpd-v/mcp-communicator-telegram",
    "distribution": {
      "type": "npm",
      "package": "mcp-communicator-telegram"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-communicator-telegram"
      ],
      "env": {
        "TELEGRAM_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "CHAT_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "09ef6485-dd73-4844-bbe1-c0c5381d1e03",
    "name": "daniel-lxs/mcp-perplexity",
    "description": "Perplexity MCP Server",
    "publisher": {
      "id": "daniel-lxs",
      "name": "daniel-lxs",
      "url": "https://github.com/daniel-lxs/mcp-perplexity"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/daniel-lxs/mcp-perplexity",
    "distribution": {
      "type": "pip",
      "package": "mcp-perplexity"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-perplexity"
      ],
      "env": {
        "PERPLEXITY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "PERPLEXITY_MODEL": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "PERPLEXITY_MODEL_ASK": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "PERPLEXITY_MODEL_CHAT": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "DB_PATH": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "e2c87a4e-e185-4193-93a0-f6e971dadb60",
    "name": "integration-app/mcp-server",
    "description": "Integration App MCP Server ",
    "publisher": {
      "id": "integration-app",
      "name": "integration-app",
      "url": "https://github.com/integration-app/mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/integration-app/mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@integration-app/mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@integration-app/mcp-server"
      ],
      "env": {
        "INTEGRATION_APP_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "INTEGRATION_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "b15c0dea-2a5f-4ae0-a8b5-56ef7b634eec",
    "name": "todoist-mcp-server",
    "description": "Todoist MCP Server",
    "publisher": {
      "id": "abhiz123",
      "name": "abhiz123",
      "url": "https://github.com/abhiz123/todoist-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/abhiz123/todoist-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@abhiz123/todoist-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@abhiz123/todoist-mcp-server"
      ],
      "env": {
        "TODOIST_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "bee97805-4920-4647-a313-d2a32747863f",
    "name": "QuantGeekDev/docker-mcp",
    "description": "🐳 docker-mcp",
    "publisher": {
      "id": "QuantGeekDev",
      "name": "QuantGeekDev",
      "url": "https://github.com/QuantGeekDev/docker-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/QuantGeekDev/docker-mcp",
    "distribution": {
      "type": "pip",
      "package": "docker-mcp"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "docker-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "ba242eab-6f55-4f89-b56f-1a60bd146e29",
    "name": "aindreyway/mcp-codex-keeper",
    "description": "Aindreyway MCP Codex Keeper",
    "publisher": {
      "id": "aindreyway",
      "name": "aindreyway",
      "url": "https://github.com/aindreyway/mcp-codex-keeper"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/aindreyway/mcp-codex-keeper",
    "distribution": {
      "type": "npm",
      "package": "@aindreyway/mcp-codex-keeper"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@aindreyway/mcp-codex-keeper"
      ],
      "env": {
        "npm_config_cache_max": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "NODE_OPTIONS": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "a64e32a0-044b-4725-8ee8-f182ea9de151",
    "name": "pskill9/website-downloader",
    "description": "Website Downloader MCP Server",
    "publisher": {
      "id": "pskill9",
      "name": "pskill9",
      "url": "https://github.com/pskill9/website-downloader"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/pskill9/website-downloader",
    "distribution": {
      "type": "npm",
      "package": "website-downloader"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "website-downloader"
      ],
      "env": {}
    }
  },
  {
    "id": "c05ecb16-104d-4fa6-81d0-26a3701ccbfd",
    "name": "reeeeemo/ancestry-mcp",
    "description": "Ancestry MCP Server",
    "publisher": {
      "id": "reeeeemo",
      "name": "reeeeemo",
      "url": "https://github.com/reeeeemo/ancestry-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/reeeeemo/ancestry-mcp",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-ancestry"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-ancestry",
        "--gedcom-path",
        "$ENVARG_GEDCOM_FILES_DIRECTORY"
      ],
      "env": {
        "ENVARG_GEDCOM_FILES_DIRECTORY": {
          "description": "Directory path containing .ged (GEDCOM) files",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "54048321-62d8-4e9f-8887-ba6bb99d683e",
    "name": "heltonteixeira/openrouterai",
    "description": "OpenRouter MCP Server",
    "publisher": {
      "id": "mcpservers",
      "name": "mcpservers",
      "url": "https://github.com/heltonteixeira/openrouterai"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/heltonteixeira/openrouterai",
    "distribution": {
      "type": "npm",
      "package": "@mcpservers/openrouterai"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@mcpservers/openrouterai"
      ],
      "env": {
        "OPENROUTER_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "OPENROUTER_DEFAULT_MODEL": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "3b816e6e-dfbe-4eaf-a570-f31a0b0c1448",
    "name": "mtane0412/ghost-mcp-server",
    "description": "Ghost MCP Server",
    "publisher": {
      "id": "mtane0412",
      "name": "mtane0412",
      "url": "https://github.com/mtane0412/ghost-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/mtane0412/ghost-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@mtane0412/ghost-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@mtane0412/ghost-mcp-server"
      ],
      "env": {
        "GHOST_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "GHOST_ADMIN_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "54c8ef12-d4d2-4764-a99b-1158b4d01b46",
    "name": "superseoworld/mcp-spotify",
    "description": "MCP Spotify Server",
    "publisher": {
      "id": "thomaswawra",
      "name": "thomaswawra",
      "url": "https://github.com/superseoworld/mcp-spotify"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/superseoworld/mcp-spotify",
    "distribution": {
      "type": "npm",
      "package": "@thomaswawra/server-spotify"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@thomaswawra/server-spotify"
      ],
      "env": {
        "SPOTIFY_CLIENT_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "SPOTIFY_CLIENT_SECRET": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "253b7d66-b954-487c-9301-29479ce4bc90",
    "name": "turlockmike/mcp-rand",
    "description": "MCP Rand",
    "publisher": {
      "id": "turlockmike",
      "name": "turlockmike",
      "url": "https://github.com/turlockmike/mcp-rand"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/turlockmike/mcp-rand",
    "distribution": {
      "type": "npm",
      "package": "mcp-rand"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-rand"
      ],
      "env": {}
    }
  },
  {
    "id": "96e9e109-1214-4f2d-891e-d019307e16a2",
    "name": "spences10/mcp-svelte-docs",
    "description": "mcp-svelte-docs",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-svelte-docs"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-svelte-docs",
    "distribution": {
      "type": "npm",
      "package": "mcp-svelte-docs"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-svelte-docs"
      ],
      "env": {
        "LIBSQL_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "LIBSQL_AUTH_TOKEN": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "667610d6-0db4-406e-b1e4-2cddff9cc645",
    "name": "hmk/box-mcp-server",
    "description": "box-mcp-server",
    "publisher": {
      "id": "hmk",
      "name": "hmk",
      "url": "https://github.com/hmk/box-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/hmk/box-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "box-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "box-mcp-server"
      ],
      "env": {
        "BOX_USER_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "BOX_JWT_BASE64": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "d83cd60a-d551-4509-9cb1-d1c3647247c7",
    "name": "apify/mcp-server-rag-web-browser",
    "description": "Model Context Protocol (MCP) Server for the RAG Web Browser Actor 🌐",
    "publisher": {
      "id": "apify",
      "name": "apify",
      "url": "https://github.com/apify/mcp-server-rag-web-browser"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/apify/mcp-server-rag-web-browser",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-rag-web-browser"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-rag-web-browser"
      ],
      "env": {
        "APIFY-API-TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "50ed2152-4143-41d4-b80d-09eb46372af4",
    "name": "emgeee/mcp-ollama",
    "description": "MCP Ollama",
    "publisher": {
      "id": "emgeee",
      "name": "emgeee",
      "url": "https://github.com/emgeee/mcp-ollama"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/emgeee/mcp-ollama",
    "distribution": {
      "type": "pip",
      "package": "mcp-ollama"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-ollama"
      ],
      "env": {}
    }
  },
  {
    "id": "8d6a17e4-5d29-46c6-8e7d-67dc59df315f",
    "name": "spences10/mcp-memory-libsql",
    "description": "mcp-memory-libsql",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-memory-libsql"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-memory-libsql",
    "distribution": {
      "type": "npm",
      "package": "mcp-memory-libsql"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-memory-libsql"
      ],
      "env": {
        "LIBSQL_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "cb3353d9-4e6e-4a59-a376-3d922424586d",
    "name": "QuantGeekDev/mongo-mcp",
    "description": "🗄️ MongoDB MCP Server for LLMS",
    "publisher": {
      "id": "QuantGeekDev",
      "name": "QuantGeekDev",
      "url": "https://github.com/QuantGeekDev/mongo-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/QuantGeekDev/mongo-mcp",
    "distribution": {
      "type": "npm",
      "package": "mongo-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mongo-mcp",
        "$ENVARG_MONGODB_CONNECTION_URL"
      ],
      "env": {
        "ENVARG_MONGODB_CONNECTION_URL": {
          "description": "MongoDB connection string including authentication credentials and database name",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "cb33f6c2-08b5-4260-9e2b-2f2d1076434a",
    "name": "motherduckdb/mcp-server-motherduck",
    "description": "mcp-server-motherduck MCP server",
    "publisher": {
      "id": "motherduckdb",
      "name": "motherduckdb",
      "url": "https://github.com/motherduckdb/mcp-server-motherduck"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/motherduckdb/mcp-server-motherduck",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-motherduck"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-motherduck"
      ],
      "env": {
        "motherduck_token": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "HOME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "39b7f5a8-6e8d-4624-b745-035815cf4d50",
    "name": "dillip285/mcp-terminal",
    "description": "MCP Terminal Server",
    "publisher": {
      "id": "dillip285",
      "name": "dillip285",
      "url": "https://github.com/dillip285/mcp-terminal"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/dillip285/mcp-terminal",
    "distribution": {
      "type": "npm",
      "package": "@dillip285/mcp-terminal"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@dillip285/mcp-terminal",
        "--allowed-paths",
        "$ENVARG_ALLOWED_PATHS"
      ],
      "env": {
        "ENVARG_ALLOWED_PATHS": {
          "description": "List of directories allowed to be accessed by this mcp server, one path per line",
          "type": "array",
          "required": true
        }
      }
    }
  },
  {
    "id": "9009aa03-fc7a-4eb9-bae6-747720095de9",
    "name": "skrapeai/skrape-mcp",
    "description": "Skrape MCP Server",
    "publisher": {
      "id": "skrapeai",
      "name": "skrapeai",
      "url": "https://github.com/skrapeai/skrape-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/skrapeai/skrape-mcp",
    "distribution": {
      "type": "npm",
      "package": "skrape"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "skrape"
      ],
      "env": {
        "SKRAPE_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "59f1bd04-49de-4dd6-bd44-73db6fdccd9e",
    "name": "cmann50/mcp-chrome-google-search",
    "description": "MCP Chrome Google Search Tool",
    "publisher": {
      "id": "cmann50",
      "name": "cmann50",
      "url": "https://github.com/cmann50/mcp-chrome-google-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/cmann50/mcp-chrome-google-search",
    "distribution": {
      "type": "npm",
      "package": "@cmann50/mcp-chrome-google-search"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@cmann50/mcp-chrome-google-search"
      ],
      "env": {}
    }
  },
  {
    "id": "11824ed7-62be-412f-86e7-6dfc888c9637",
    "name": "GongRzhe/REDIS-MCP-Server",
    "description": "Redis MCP Server",
    "publisher": {
      "id": "gongrzhe",
      "name": "gongrzhe",
      "url": "https://github.com/GongRzhe/REDIS-MCP-Server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/GongRzhe/REDIS-MCP-Server",
    "distribution": {
      "type": "npm",
      "package": "@gongrzhe/server-redis-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@gongrzhe/server-redis-mcp",
        "$ENVARG_REDIS_URL"
      ],
      "env": {
        "ENVARG_REDIS_URL": {
          "description": "Redis connection URL string",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "1b8d4304-df48-4d82-b609-fc222ca6408e",
    "name": "lumile/lumbretravel-mcp",
    "description": "LumbreTravel MCP Server",
    "publisher": {
      "id": "lumile",
      "name": "lumile",
      "url": "https://github.com/lumile/lumbretravel-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/lumile/lumbretravel-mcp",
    "distribution": {
      "type": "npm",
      "package": "lumbretravel-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "lumbretravel-mcp"
      ],
      "env": {
        "CLIENT_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "CLIENT_SECRET": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "EMAIL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "PASSWORD": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "1a84671b-e2b1-4ae5-8209-30a4d107c798",
    "name": "aindreyway/mcp-neurolora",
    "description": "MCP Neurolora",
    "publisher": {
      "id": "aindreyway",
      "name": "aindreyway",
      "url": "https://github.com/aindreyway/mcp-neurolora"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/aindreyway/mcp-neurolora",
    "distribution": {
      "type": "npm",
      "package": "@aindreyway/mcp-neurolora"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@aindreyway/mcp-neurolora"
      ],
      "env": {
        "OPENAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "NODE_OPTIONS": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "2db586c3-99a7-41fd-8a27-97b1ff50ea0e",
    "name": "kazuph/mcp-pocket",
    "description": "MCP Pocket",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-pocket"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-pocket",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-pocket"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-pocket"
      ],
      "env": {
        "POCKET_CONSUMER_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "POCKET_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "fed3b4e1-5b0e-4478-a7f3-14b636b87195",
    "name": "jonathanfischer97/juliadoc-mcp",
    "description": "Julia Documentation MCP Server ",
    "publisher": {
      "id": "jonathanfischer97",
      "name": "jonathanfischer97",
      "url": "https://github.com/jonathanfischer97/juliadoc-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/jonathanfischer97/juliadoc-mcp",
    "distribution": {
      "type": "npm",
      "package": "@jonathanfischer97/server-juliadoc"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@jonathanfischer97/server-juliadoc"
      ],
      "env": {
        "JULIA_PROJECT": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "e4207769-7f2a-4950-a362-bfbfbd2f7ab6",
    "name": "tatn/mcp-server-fetch-python",
    "description": "mcp-server-fetch-python",
    "publisher": {
      "id": "tatn",
      "name": "tatn",
      "url": "https://github.com/tatn/mcp-server-fetch-python"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tatn/mcp-server-fetch-python",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-fetch-python"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-fetch-python"
      ],
      "env": {
        "OPENAI_API_KEY": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "PYTHONIOENCODING": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "MODEL_NAME": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "1011130f-8eaf-4d21-b985-687ff46b6f1b",
    "name": "designcomputer/mysql_mcp_server",
    "description": "MySQL MCP Server",
    "publisher": {
      "id": "designcomputer",
      "name": "designcomputer",
      "url": "https://github.com/designcomputer/mysql_mcp_server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/designcomputer/mysql_mcp_server",
    "distribution": {
      "type": "pip",
      "package": "mysql-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mysql-mcp-server"
      ],
      "env": {
        "MYSQL_HOST": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MYSQL_USER": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MYSQL_PASSWORD": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MYSQL_DATABASE": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "a363d68a-7e0d-4e56-bf8e-e6080c4ec004",
    "name": "adenot/mcp-google-search",
    "description": "mcp-google-server A MCP Server for Google Custom Search and Webpage Reading",
    "publisher": {
      "id": "adenot",
      "name": "adenot",
      "url": "https://github.com/adenot/mcp-google-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/adenot/mcp-google-search",
    "distribution": {
      "type": "npm",
      "package": "@adenot/mcp-google-search"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@adenot/mcp-google-search"
      ],
      "env": {
        "GOOGLE_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "GOOGLE_SEARCH_ENGINE_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "87278d10-4b74-4743-aaa0-1bbba4ba174c",
    "name": "mamertofabian/mcp-everything-search",
    "description": "Everything Search MCP Server",
    "publisher": {
      "id": "mamertofabian",
      "name": "mamertofabian",
      "url": "https://github.com/mamertofabian/mcp-everything-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/mamertofabian/mcp-everything-search",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-everything-search"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-everything-search"
      ],
      "env": {
        "EVERYTHING_SDK_PATH": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "7480455d-75bc-480a-8dec-e86c0ca6146c",
    "name": "cosmix/jira-mcp",
    "description": "JIRA MCP Server",
    "publisher": {
      "id": "cosmix",
      "name": "cosmix",
      "url": "https://github.com/cosmix/jira-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/cosmix/jira-mcp",
    "distribution": {
      "type": "npm",
      "package": "jira-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "jira-mcp"
      ],
      "env": {
        "JIRA_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "JIRA_BASE_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "JIRA_USER_EMAIL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "fa37a5ca-e21a-4161-939f-baf224e296bb",
    "name": "JoshuaRileyDev/simulator-mcp-server",
    "description": "iOS Simulator MCP Server",
    "publisher": {
      "id": "joshuarileydev",
      "name": "joshuarileydev",
      "url": "https://github.com/JoshuaRileyDev/simulator-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/JoshuaRileyDev/simulator-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@joshuarileydev/simulator-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@joshuarileydev/simulator-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "316e845c-7576-44ce-907b-319cdbe53cbc",
    "name": "mamertofabian/elevenlabs-mcp-server",
    "description": "ElevenLabs MCP Server",
    "publisher": {
      "id": "mamertofabian",
      "name": "mamertofabian",
      "url": "https://github.com/mamertofabian/elevenlabs-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/mamertofabian/elevenlabs-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "elevenlabs-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "elevenlabs-mcp-server"
      ],
      "env": {
        "ELEVENLABS_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ELEVENLABS_VOICE_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ELEVENLABS_MODEL_ID": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ELEVENLABS_STABILITY": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ELEVENLABS_SIMILARITY_BOOST": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ELEVENLABS_STYLE": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ELEVENLABS_OUTPUT_DIR": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "2154e5e7-af72-49cc-a4e9-fd89de108363",
    "name": "sirmews/mcp-pinecone",
    "description": "Pinecone Model Context Protocol Server for Claude Desktop.",
    "publisher": {
      "id": "sirmews",
      "name": "sirmews",
      "url": "https://github.com/sirmews/mcp-pinecone"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/sirmews/mcp-pinecone",
    "distribution": {
      "type": "pip",
      "package": "mcp-pinecone"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-pinecone",
        "--index-name",
        "$ENVARG_INDEX_NAME",
        "--api-key",
        "$ENVARG_API_KEY"
      ],
      "env": {
        "ENVARG_INDEX_NAME": {
          "description": "The name of your Pinecone index",
          "type": "input",
          "required": true
        },
        "ENVARG_API_KEY": {
          "description": "Your Pinecone API key",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "21e264fe-84f9-4434-badc-c1b6a201a9a6",
    "name": "umshere/uiflowchartcreator",
    "description": "UIFlowchartCreator",
    "publisher": {
      "id": "umshere",
      "name": "umshere",
      "url": "https://github.com/umshere/uiflowchartcreator"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/umshere/uiflowchartcreator",
    "distribution": {
      "type": "npm",
      "package": "uiflowchartcreator"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "uiflowchartcreator"
      ],
      "env": {}
    }
  },
  {
    "id": "139cd284-2fca-41ac-b784-6a4682d1957a",
    "name": "kujenga/zotero-mcp",
    "description": "Model Context Protocol server for Zotero",
    "publisher": {
      "id": "kujenga",
      "name": "kujenga",
      "url": "https://github.com/kujenga/zotero-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kujenga/zotero-mcp",
    "distribution": {
      "type": "pip",
      "package": "zotero-mcp"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "zotero-mcp"
      ],
      "env": {
        "ZOTERO_LOCAL": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ZOTERO_LIBRARY_ID": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ZOTERO_LIBRARY_TYPE": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ZOTERO_API_KEY": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "f21726ce-1b14-4665-8c72-ade8ebc46b68",
    "name": "felores/cloudinary-mcp-server",
    "description": "Cloudinary MCP Server",
    "publisher": {
      "id": "felores",
      "name": "felores",
      "url": "https://github.com/felores/cloudinary-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/felores/cloudinary-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@felores/cloudinary-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@felores/cloudinary-mcp-server"
      ],
      "env": {
        "CLOUDINARY_CLOUD_NAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "CLOUDINARY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "CLOUDINARY_API_SECRET": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "68f29b1a-1050-498e-9d83-f0c0fd6c2172",
    "name": "gerred/mcp-server-replicate",
    "description": "MCP Server Replicate",
    "publisher": {
      "id": "gerred",
      "name": "gerred",
      "url": "https://github.com/gerred/mcp-server-replicate"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/gerred/mcp-server-replicate",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-replicate"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-replicate"
      ],
      "env": {
        "REPLICATE_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "10c55074-3a95-4954-a08b-e636cfda1e1e",
    "name": "spences10/mcp-tavily-search",
    "description": "mcp-tavily-search",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-tavily-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-tavily-search",
    "distribution": {
      "type": "npm",
      "package": "mcp-tavily-search"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-tavily-search"
      ],
      "env": {
        "TAVILY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "54cfbc41-daec-44f5-9fc7-f029230dcba0",
    "name": "vivekVells/mcp-pandoc",
    "description": "mcp-pandoc: A Document Conversion MCP Server",
    "publisher": {
      "id": "vivekVells",
      "name": "vivekVells",
      "url": "https://github.com/vivekVells/mcp-pandoc"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/vivekVells/mcp-pandoc",
    "distribution": {
      "type": "pip",
      "package": "mcp-pandoc"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-pandoc"
      ],
      "env": {}
    }
  },
  {
    "id": "1eb41024-3a61-4040-86b6-2c1f791544ef",
    "name": "SecretiveShell/MCP-wolfram-alpha",
    "description": "MCP-wolfram-alpha",
    "publisher": {
      "id": "SecretiveShell",
      "name": "SecretiveShell",
      "url": "https://github.com/SecretiveShell/MCP-wolfram-alpha"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/SecretiveShell/MCP-wolfram-alpha",
    "distribution": {
      "type": "pip",
      "package": "mcp-wolfram-alpha"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-wolfram-alpha"
      ],
      "env": {
        "WOLFRAM_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "c77a7998-f8e0-43e2-b581-3a816ac8f2fa",
    "name": "yamaton/mcp-dice",
    "description": "mcp-dice: A MCP Server for Rolling Dice",
    "publisher": {
      "id": "yamaton",
      "name": "yamaton",
      "url": "https://github.com/yamaton/mcp-dice"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/yamaton/mcp-dice",
    "distribution": {
      "type": "pip",
      "package": "mcp-dice"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-dice"
      ],
      "env": {}
    }
  },
  {
    "id": "ecacc636-ca5a-49ba-8fed-8bdf15bf37a8",
    "name": "2b3pro/roam-research-mcp",
    "description": "Roam Research MCP Server",
    "publisher": {
      "id": "2b3pro",
      "name": "2b3pro",
      "url": "https://github.com/2b3pro/roam-research-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/2b3pro/roam-research-mcp",
    "distribution": {
      "type": "npm",
      "package": "roam-research-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "roam-research-mcp"
      ],
      "env": {
        "ROAM_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "ROAM_GRAPH_NAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MEMORIES_TAG": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "45e86f53-4028-4b53-861f-e0669f37ed99",
    "name": "evalstate/mcp-hfspace",
    "description": "mcp-hfspace MCP Server 🤗",
    "publisher": {
      "id": "llmindset",
      "name": "llmindset",
      "url": "https://github.com/evalstate/mcp-hfspace"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/evalstate/mcp-hfspace",
    "distribution": {
      "type": "npm",
      "package": "@llmindset/mcp-hfspace"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@llmindset/mcp-hfspace"
      ],
      "env": {
        "MCP_HF_WORK_DIR": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "HF_TOKEN": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "b042caa4-543d-4b06-b652-5218a4909134",
    "name": "JoshuaRileyDev/mac-apps-launcher",
    "description": "Mac Apps Launcher MCP Server",
    "publisher": {
      "id": "joshuarileydev",
      "name": "joshuarileydev",
      "url": "https://github.com/JoshuaRileyDev/mac-apps-launcher"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/JoshuaRileyDev/mac-apps-launcher",
    "distribution": {
      "type": "npm",
      "package": "@joshuarileydev/mac-apps-launcher-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@joshuarileydev/mac-apps-launcher-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "b822003b-1523-4ee8-abe4-4e2179c305c4",
    "name": "r-huijts/rijksmuseum-mcp",
    "description": "Rijksmuseum MCP Server",
    "publisher": {
      "id": "r-huijts",
      "name": "r-huijts",
      "url": "https://github.com/r-huijts/rijksmuseum-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/r-huijts/rijksmuseum-mcp",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-rijksmuseum"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-rijksmuseum"
      ],
      "env": {
        "RIJKSMUSEUM_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "ebba1ed3-eb66-41a6-8e22-a7cacadd8144",
    "name": "Siddhant-K-code/memory-journal-mcp-server",
    "description": "📸 Smart Photo Journal MCP Server",
    "publisher": {
      "id": "Siddhant-K-code",
      "name": "Siddhant-K-code",
      "url": "https://github.com/Siddhant-K-code/memory-journal-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Siddhant-K-code/memory-journal-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "memory-journal-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "memory-journal-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "3c843f9c-59b9-4bd6-88c3-0a09e8319be2",
    "name": "EyevinnOSC/mcp-server",
    "description": "Eyevinn Open Source Cloud MCP Server",
    "publisher": {
      "id": "osaas",
      "name": "osaas",
      "url": "https://github.com/EyevinnOSC/mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/EyevinnOSC/mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@osaas/mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@osaas/mcp-server"
      ],
      "env": {
        "OSC_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "f526b9b6-ddca-494e-a4e4-8a2161697ba3",
    "name": "Braffolk/mcp-summarization-functions",
    "description": "Summarization Functions",
    "publisher": {
      "id": "Braffolk",
      "name": "Braffolk",
      "url": "https://github.com/Braffolk/mcp-summarization-functions"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Braffolk/mcp-summarization-functions",
    "distribution": {
      "type": "npm",
      "package": "mcp-summarization-functions"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-summarization-functions"
      ],
      "env": {
        "PROVIDER": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MODEL_ID": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "PROVIDER_BASE_URL": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "MAX_TOKENS": {
          "description": "Optional environment variable",
          "type": "number"
        },
        "SUMMARIZATION_CHAR_THRESHOLD": {
          "description": "Optional environment variable",
          "type": "number"
        },
        "SUMMARIZATION_CACHE_MAX_AGE": {
          "description": "Optional environment variable",
          "type": "number"
        },
        "MCP_WORKING_DIR": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "b2db1437-ff83-43dd-8bb1-7dbb6f033e2f",
    "name": "ChanMeng666/server-google-news",
    "description": "Google News MCP Server",
    "publisher": {
      "id": "chanmeng666",
      "name": "chanmeng666",
      "url": "https://github.com/ChanMeng666/server-google-news"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ChanMeng666/server-google-news",
    "distribution": {
      "type": "npm",
      "package": "@chanmeng666/google-news-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@chanmeng666/google-news-server"
      ],
      "env": {
        "SERP_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "61734caf-df4a-4c77-8b3d-f38a5e53700c",
    "name": "fatwang2/search1api-mcp",
    "description": "Search1API MCP Server",
    "publisher": {
      "id": "fatwang2",
      "name": "fatwang2",
      "url": "https://github.com/fatwang2/search1api-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/fatwang2/search1api-mcp",
    "distribution": {
      "type": "npm",
      "package": "search1api-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "search1api-mcp"
      ],
      "env": {
        "SEARCH1API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "ad9728b8-ce3d-4ed4-ae9f-d0063f015090",
    "name": "rogerheykoop/mcp-safari-screenshot",
    "description": "Safari Screenshot",
    "publisher": {
      "id": "rogerheykoop",
      "name": "rogerheykoop",
      "url": "https://github.com/rogerheykoop/mcp-safari-screenshot"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/rogerheykoop/mcp-safari-screenshot",
    "distribution": {
      "type": "npm",
      "package": "@rogerheykoop/mcp-safari-screenshot"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@rogerheykoop/mcp-safari-screenshot"
      ],
      "env": {}
    }
  },
  {
    "id": "0a2515b0-a252-42ce-a174-aac20cb1da01",
    "name": "MindscapeHQ/mcp-server-raygun",
    "description": "Raygun MCP Server",
    "publisher": {
      "id": "raygun.io",
      "name": "raygun.io",
      "url": "https://github.com/MindscapeHQ/mcp-server-raygun"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/MindscapeHQ/mcp-server-raygun",
    "distribution": {
      "type": "npm",
      "package": "@raygun.io/mcp-server-raygun"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@raygun.io/mcp-server-raygun"
      ],
      "env": {
        "RAYGUN_PAT_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "3b0ff804-871c-408b-8c95-60299b37fbbf",
    "name": "CamdenClark/jira-mcp",
    "description": "JIRA MCP Server",
    "publisher": {
      "id": "CamdenClark",
      "name": "CamdenClark",
      "url": "https://github.com/CamdenClark/jira-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/CamdenClark/jira-mcp",
    "distribution": {
      "type": "npm",
      "package": "jira-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "jira-mcp"
      ],
      "env": {
        "JIRA_INSTANCE_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "JIRA_USER_EMAIL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "JIRA_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "b4091e76-7f19-43df-9ac0-8621c3f20f34",
    "name": "evalstate/mcp-miro",
    "description": "mcp-miro MCP Server",
    "publisher": {
      "id": "llmindset",
      "name": "llmindset",
      "url": "https://github.com/evalstate/mcp-miro"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/evalstate/mcp-miro",
    "distribution": {
      "type": "npm",
      "package": "@llmindset/mcp-miro"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@llmindset/mcp-miro",
        "--token",
        "$ENVARG_MIRO_OAUTH_TOKEN"
      ],
      "env": {
        "ENVARG_MIRO_OAUTH_TOKEN": {
          "description": "MIRO OAuth authentication token for API access",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "2d912578-392d-4bf1-a854-9803b04a0694",
    "name": "blurrah/mcp-graphql",
    "description": "mcp-graphql",
    "publisher": {
      "id": "blurrah",
      "name": "blurrah",
      "url": "https://github.com/blurrah/mcp-graphql"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/blurrah/mcp-graphql",
    "distribution": {
      "type": "npm",
      "package": "mcp-graphql"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-graphql",
        "--endpoint",
        "$ENVARG_GRAPHQL_ENDPOINT"
      ],
      "env": {
        "ENVARG_GRAPHQL_ENDPOINT": {
          "description": "URL of the GraphQL server endpoint",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "aa525b5f-2eed-4ee0-bf48-def74eed58de",
    "name": "@stripe/mcp",
    "description": "Stripe Model Context Protocol",
    "publisher": {
      "id": "stripe",
      "name": "stripe",
      "url": "https://github.com/stripe/agent-toolkit/tree/main/modelcontextprotocol"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/stripe/agent-toolkit/tree/main/modelcontextprotocol",
    "distribution": {
      "type": "npm",
      "package": "@stripe/mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@stripe/mcp",
        "--tools=all",
        "--api-key=${STRIPE_SECRET_KEY}"
      ],
      "env": {
        "STRIPE_SECRET_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "7a8ceb01-c088-4a71-b44e-5eb7f05b8d93",
    "name": "Kush36Agrawal/Video_Editor_MCP",
    "description": "Video Editor MCP Server",
    "publisher": {
      "id": "Kush36Agrawal",
      "name": "Kush36Agrawal",
      "url": "https://github.com/Kush36Agrawal/Video_Editor_MCP"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Kush36Agrawal/Video_Editor_MCP",
    "distribution": {
      "type": "pip",
      "package": "video-editor"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "video-editor"
      ],
      "env": {}
    }
  },
  {
    "id": "fc5f3f4c-1fb7-4115-a8b1-2a74997de37a",
    "name": "crazyrabbitLTC/mpc-tally-api-server",
    "description": "MPC Tally API Server",
    "publisher": {
      "id": "crazyrabbitLTC",
      "name": "crazyrabbitLTC",
      "url": "https://github.com/crazyrabbitLTC/mpc-tally-api-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/crazyrabbitLTC/mpc-tally-api-server",
    "distribution": {
      "type": "npm",
      "package": "mpc-tally-api-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mpc-tally-api-server"
      ],
      "env": {
        "TALLY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "54767014-c3aa-4efb-a6c8-96a2070e03b8",
    "name": "kazuph/mcp-fetch",
    "description": "MCP Fetch",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-fetch"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-fetch",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-fetch"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-fetch"
      ],
      "env": {}
    }
  },
  {
    "id": "8cf6aa1d-a128-4a7e-bf85-c1970c51350f",
    "name": "folderr-tech/folderr-mcp-server",
    "description": "Folderr MCP Server",
    "publisher": {
      "id": "folderr",
      "name": "folderr",
      "url": "https://github.com/folderr-tech/folderr-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/folderr-tech/folderr-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@folderr/folderr-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@folderr/folderr-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "f47e3cd9-5c52-4b61-adac-d8959bf7d170",
    "name": "linear-mcp",
    "description": "Linear MCP Server",
    "publisher": {
      "id": "ibraheem4",
      "name": "ibraheem4",
      "url": "https://github.com/ibraheem4/linear-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ibraheem4/linear-mcp",
    "distribution": {
      "type": "npm",
      "package": "@ibraheem4/linear-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@ibraheem4/linear-mcp"
      ],
      "env": {
        "LINEAR_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "f628cd5d-87cf-4e6b-b74e-76ca33566323",
    "name": "mouhamadalmounayar/mcp-confluence",
    "description": "Mcp-Confluence",
    "publisher": {
      "id": "mouhamadalmounayar",
      "name": "mouhamadalmounayar",
      "url": "https://github.com/mouhamadalmounayar/mcp-confluence"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/mouhamadalmounayar/mcp-confluence",
    "distribution": {
      "type": "npm",
      "package": "mcp-confluence"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-confluence"
      ],
      "env": {
        "API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "DOMAIN_NAME": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "EMAIL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "03717bc9-a354-413e-a79e-e8b36bee7aea",
    "name": "kazuph/mcp-github-pera1",
    "description": "GitHub MCP Server for Pera1",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-github-pera1"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-github-pera1",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-github-pera1"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-github-pera1"
      ],
      "env": {}
    }
  },
  {
    "id": "44ff4775-dce6-4422-a635-262aac4dc24d",
    "name": "domdomegg/airtable-mcp-server",
    "description": "airtable-mcp-server",
    "publisher": {
      "id": "domdomegg",
      "name": "domdomegg",
      "url": "https://github.com/domdomegg/airtable-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/domdomegg/airtable-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "airtable-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "airtable-mcp-server"
      ],
      "env": {
        "AIRTABLE_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "c2920ba1-fa2c-42c6-9453-b12bb0301d18",
    "name": "JoshuaRileyDev/supabase-mcp-server",
    "description": "Supabase MCP Server",
    "publisher": {
      "id": "joshuarileydev",
      "name": "joshuarileydev",
      "url": "https://github.com/JoshuaRileyDev/supabase-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/JoshuaRileyDev/supabase-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@joshuarileydev/supabase-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@joshuarileydev/supabase-mcp-server"
      ],
      "env": {
        "SUPABASE_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "e9f640d0-3fcd-430e-8483-0f0d917ea5f3",
    "name": "deus-h/claudeus-wp-mcp",
    "description": "🤘 Claudeus WordPress MCP",
    "publisher": {
      "id": "deus-h",
      "name": "deus-h",
      "url": "https://github.com/deus-h/claudeus-wp-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/deus-h/claudeus-wp-mcp",
    "distribution": {
      "type": "npm",
      "package": "claudeus-wp-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "claudeus-wp-mcp"
      ],
      "env": {
        "WP_SITES_PATH": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "c84c2223-5cfd-4dbb-b1cc-ae838d6f2981",
    "name": "andybrandt/mcp-simple-pubmed",
    "description": "MCP Simple PubMed",
    "publisher": {
      "id": "andybrandt",
      "name": "andybrandt",
      "url": "https://github.com/andybrandt/mcp-simple-pubmed"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/andybrandt/mcp-simple-pubmed",
    "distribution": {
      "type": "pip",
      "package": "mcp-simple-pubmed"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-simple-pubmed"
      ],
      "env": {
        "PUBMED_EMAIL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "PUBMED_API_KEY": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "6bae3460-b8d0-42a3-a0c7-8e110ad770f1",
    "name": "kazuph/mcp-youtube",
    "description": "YouTube MCP Server",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-youtube"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-youtube",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-youtube"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-youtube"
      ],
      "env": {}
    }
  },
  {
    "id": "f4d3af3f-ece7-4ede-bb7d-fc900c1b8eca",
    "name": "shanejonas/openrpc-mpc-server",
    "description": "OpenRPC MCP Server",
    "publisher": {
      "id": "shanejonas",
      "name": "shanejonas",
      "url": "https://github.com/shanejonas/openrpc-mpc-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/shanejonas/openrpc-mpc-server",
    "distribution": {
      "type": "npm",
      "package": "openrpc-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "openrpc-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "18965e6c-5c6b-434d-b8f9-5c2dc90afc7c",
    "name": "arjunkmrm/mcp-minecraft",
    "description": "Minecraft MCP Integration",
    "publisher": {
      "id": "arjunkmrm",
      "name": "arjunkmrm",
      "url": "https://github.com/arjunkmrm/mcp-minecraft"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/arjunkmrm/mcp-minecraft",
    "distribution": {
      "type": "npm",
      "package": "mcp-minecraft"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-minecraft",
        "--server-jar",
        "$ENVARG_SERVER_JAR_PATH"
      ],
      "env": {
        "ENVARG_SERVER_JAR_PATH": {
          "description": "Absolute path to your Minecraft server.jar file",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "ea4e81a2-bf16-411f-b8e4-30d528303f31",
    "name": "amidabuddha/unichat-ts-mcp-server",
    "description": "Unichat MCP Server in TypeScript",
    "publisher": {
      "id": "amidabuddha",
      "name": "amidabuddha",
      "url": "https://github.com/amidabuddha/unichat-ts-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/amidabuddha/unichat-ts-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "unichat-ts-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "unichat-ts-mcp-server"
      ],
      "env": {
        "UNICHAT_MODEL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "UNICHAT_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "baeb7674-3462-4542-bae1-119e6b2dc802",
    "name": "cyanheads/toolkit-mcp-server",
    "description": "toolkit-mcp-server",
    "publisher": {
      "id": "cyanheads",
      "name": "cyanheads",
      "url": "https://github.com/cyanheads/toolkit-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/cyanheads/toolkit-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@cyanheads/toolkit-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@cyanheads/toolkit-mcp-server"
      ],
      "env": {
        "NODE_ENV": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "d314af2c-cc32-4879-bf93-6db1f94434cc",
    "name": "amir-bengherbi/shopify-mcp-server",
    "description": "Shopify MCP Server",
    "publisher": {
      "id": "amir-bengherbi",
      "name": "amir-bengherbi",
      "url": "https://github.com/amir-bengherbi/shopify-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/amir-bengherbi/shopify-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "shopify-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "shopify-mcp-server"
      ],
      "env": {
        "SHOPIFY_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MYSHOPIFY_DOMAIN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "687cdb50-eef8-427e-9c83-d586ad789d7c",
    "name": "upstash/mcp-server",
    "description": "Upstash MCP Server",
    "publisher": {
      "id": "upstash",
      "name": "upstash",
      "url": "https://github.com/upstash/mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/upstash/mcp-server",
    "distribution": {
      "type": "npm",
      "package": "@upstash/mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@upstash/mcp-server",
        "run",
        "$ENVARG_UPSTASH_EMAIL",
        "$ENVARG_UPSTASH_API_KEY"
      ],
      "env": {
        "ENVARG_UPSTASH_EMAIL": {
          "description": "Your Upstash account email address",
          "type": "input",
          "required": true
        },
        "ENVARG_UPSTASH_API_KEY": {
          "description": "Your Upstash API key from the developer console",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "9d53b4a9-9661-4c96-b0aa-33b3a584a4cc",
    "name": "sparfenyuk/mcp-proxy",
    "description": "mcp-proxy",
    "publisher": {
      "id": "sparfenyuk",
      "name": "sparfenyuk",
      "url": "https://github.com/sparfenyuk/mcp-proxy"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/sparfenyuk/mcp-proxy",
    "distribution": {
      "type": "pip",
      "package": "mcp-proxy"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-proxy",
        "$ENVARG_COMMAND_OR_URL"
      ],
      "env": {
        "API_ACCESS_TOKEN": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ENVARG_COMMAND_OR_URL": {
          "description": "The MCP server SSE endpoint to connect to.",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "06081012-c989-4ca8-93ee-11679264c9f8",
    "name": "kazuph/mcp-gmail-gas",
    "description": "MCP Gmail",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-gmail-gas"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-gmail-gas",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-gmail-gas"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-gmail-gas"
      ],
      "env": {
        "GAS_ENDPOINT": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "VALID_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "4e3838ac-6ddc-4771-868b-77c9ce2da2c9",
    "name": "Sunwood-ai-labs/obsidian-mcp",
    "description": "obsidian-mcp MCP Server",
    "publisher": {
      "id": "Sunwood-ai-labs",
      "name": "Sunwood-ai-labs",
      "url": "https://github.com/Sunwood-ai-labs/obsidian-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Sunwood-ai-labs/obsidian-mcp",
    "distribution": {
      "type": "npm",
      "package": "obsidian-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "obsidian-mcp"
      ],
      "env": {
        "OBSIDIAN_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "8b6df165-2094-46d1-98b8-58f58e29ef3c",
    "name": "claudemind/mcp-webresearch",
    "description": "MCP Web Research Server",
    "publisher": {
      "id": "claudemind",
      "name": "claudemind",
      "url": "https://github.com/Hawstein/mcp-webresearch"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Hawstein/mcp-webresearch",
    "distribution": {
      "type": "npm",
      "package": "@claudemind/mcp-webresearch"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@claudemind/mcp-webresearch"
      ],
      "env": {}
    }
  },
  {
    "id": "c70cf05c-bfdb-4d73-8678-f0d2e65ed5d1",
    "name": "lumile/mercadolibre-mcp",
    "description": "MercadoLibre MCP Server",
    "publisher": {
      "id": "lumile",
      "name": "lumile",
      "url": "https://github.com/lumile/mercadolibre-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/lumile/mercadolibre-mcp",
    "distribution": {
      "type": "npm",
      "package": "mercadolibre-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mercadolibre-mcp"
      ],
      "env": {
        "CLIENT_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "CLIENT_SECRET": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "SITE_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "8c810f00-2fbf-4dfd-af5c-c0b8c4d9b5a1",
    "name": "ThetaBird/mcp-server-axiom-js",
    "description": "MCP Server for Axiom",
    "publisher": {
      "id": "ThetaBird",
      "name": "ThetaBird",
      "url": "https://github.com/ThetaBird/mcp-server-axiom-js"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ThetaBird/mcp-server-axiom-js",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-axiom"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-axiom"
      ],
      "env": {
        "AXIOM_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "AXIOM_ORG_ID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "AXIOM_URL": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "AXIOM_QUERY_RATE": {
          "description": "Optional environment variable",
          "type": "number"
        },
        "AXIOM_QUERY_BURST": {
          "description": "Optional environment variable",
          "type": "number"
        },
        "AXIOM_DATASETS_RATE": {
          "description": "Optional environment variable",
          "type": "number"
        },
        "AXIOM_DATASETS_BURST": {
          "description": "Optional environment variable",
          "type": "number"
        },
        "PORT": {
          "description": "Optional environment variable",
          "type": "number"
        }
      }
    }
  },
  {
    "id": "93b3a0ef-c39b-4395-bf0d-01c003be98b2",
    "name": "JoeBuildsStuff/mcp-jina-ai",
    "description": "Jina AI MCP Server",
    "publisher": {
      "id": "JoeBuildsStuff",
      "name": "JoeBuildsStuff",
      "url": "https://github.com/JoeBuildsStuff/mcp-jina-ai"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/JoeBuildsStuff/mcp-jina-ai",
    "distribution": {
      "type": "npm",
      "package": "jina-ai-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "jina-ai-mcp-server"
      ],
      "env": {
        "JINA_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "9dc6afab-c2fa-4cb7-945f-645731a08022",
    "name": "hmk/attio-mcp-server",
    "description": "attio-mcp-server",
    "publisher": {
      "id": "hmk",
      "name": "hmk",
      "url": "https://github.com/hmk/attio-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/hmk/attio-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "attio-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "attio-mcp-server"
      ],
      "env": {
        "ATTIO_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "a0f693c6-7b7c-452f-b3dc-aed4e1dbbe5f",
    "name": "spences10/mcp-duckduckgo-search",
    "description": "mcp-duckduckgo-search",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-duckduckgo-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-duckduckgo-search",
    "distribution": {
      "type": "npm",
      "package": "mcp-duckduckgo-search"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-duckduckgo-search"
      ],
      "env": {
        "SERPAPI_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "d96582f2-221c-46de-ae38-ccc4b57d248e",
    "name": "blazickjp/arxiv-mcp-server",
    "description": "ArXiv MCP Server",
    "publisher": {
      "id": "blazickjp",
      "name": "blazickjp",
      "url": "https://github.com/blazickjp/arxiv-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/blazickjp/arxiv-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "arxiv-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "arxiv-mcp-server"
      ],
      "env": {
        "ARXIV_STORAGE_PATH": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "fa442c97-1be5-4706-b8f4-9d45ce064f19",
    "name": "f4ww4z/mcp-mysql-server",
    "description": "@f4ww4z/mcp-mysql-server",
    "publisher": {
      "id": "f4ww4z",
      "name": "f4ww4z",
      "url": "https://github.com/f4ww4z/mcp-mysql-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/f4ww4z/mcp-mysql-server",
    "distribution": {
      "type": "npm",
      "package": "@f4ww4z/mcp-mysql-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@f4ww4z/mcp-mysql-server"
      ],
      "env": {
        "MYSQL_HOST": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MYSQL_USER": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MYSQL_PASSWORD": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "MYSQL_DATABASE": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "e1b195a2-cf5b-4b6c-8f1d-1428d73a7d3b",
    "name": "adiom-data/lance-mcp",
    "description": "🗄️ LanceDB MCP Server for LLMS",
    "publisher": {
      "id": "adiom-data",
      "name": "adiom-data",
      "url": "https://github.com/adiom-data/lance-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/adiom-data/lance-mcp",
    "distribution": {
      "type": "npm",
      "package": "lance-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "lance-mcp",
        "$ENVARG_DATABASE_PATH"
      ],
      "env": {
        "ENVARG_DATABASE_PATH": {
          "description": "Path to the local index directory where LanceDB will store the data",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "49499898-c3b6-4004-b52c-c963a7e103df",
    "name": "da-okazaki/mcp-neo4j-server",
    "description": "MCP Neo4j Server",
    "publisher": {
      "id": "alanse",
      "name": "alanse",
      "url": "https://github.com/da-okazaki/mcp-neo4j-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/da-okazaki/mcp-neo4j-server",
    "distribution": {
      "type": "npm",
      "package": "@alanse/mcp-neo4j-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@alanse/mcp-neo4j-server"
      ],
      "env": {
        "NEO4J_URI": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "NEO4J_USERNAME": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "NEO4J_PASSWORD": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "d00155c6-3f6c-4504-95c3-15426e00ebdc",
    "name": "blazickjp/web-browser-mcp-server",
    "description": "## ✨ Features",
    "publisher": {
      "id": "blazickjp",
      "name": "blazickjp",
      "url": "https://github.com/blazickjp/web-browser-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/blazickjp/web-browser-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "web-browser-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "web-browser-mcp-server"
      ],
      "env": {
        "REQUEST_TIMEOUT": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "3b6e5bd3-b7bf-4365-814c-4cdb1a12367f",
    "name": "9Ninety/MCPNotes",
    "description": "📝 MCP Notes",
    "publisher": {
      "id": "9Ninety",
      "name": "9Ninety",
      "url": "https://github.com/9Ninety/MCPNotes"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/9Ninety/MCPNotes",
    "distribution": {
      "type": "npm",
      "package": "mcp-notes"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-notes",
        "--dynamodb",
        "$ENVARG_DYNAMODB_CONNECTION_STRING"
      ],
      "env": {
        "ENVARG_DYNAMODB_CONNECTION_STRING": {
          "description": "DynamoDB connection string in the format: dynamodb://<aws_access_key>:<aws_secret_key>@<region>/<table>",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "d8b5a638-88ad-490f-b1dd-f436b8478a49",
    "name": "spences10/mcp-jinaai-grounding",
    "description": "mcp-jinaai-grounding",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-jinaai-grounding"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-jinaai-grounding",
    "distribution": {
      "type": "npm",
      "package": "mcp-jinaai-grounding"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-jinaai-grounding"
      ],
      "env": {
        "JINAAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "ff920b5c-b7e0-4fae-ab1a-bf0ddda4cc1b",
    "name": "spences10/mcp-jinaai-reader",
    "description": "mcp-jinaai-reader",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-jinaai-reader"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-jinaai-reader",
    "distribution": {
      "type": "npm",
      "package": "mcp-jinaai-reader"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-jinaai-reader"
      ],
      "env": {
        "JINAAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "e40c9995-3ff1-4d0a-a806-ca83140d3065",
    "name": "roychri/mcp-server-asana",
    "description": "",
    "publisher": {
      "id": "roychri",
      "name": "roychri",
      "url": "https://github.com/roychri/mcp-server-asana"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/roychri/mcp-server-asana",
    "distribution": {
      "type": "npm",
      "package": "@roychri/mcp-server-asana"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@roychri/mcp-server-asana"
      ],
      "env": {
        "ASANA_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "a6266b02-fde3-4dae-a675-dae58ec315b1",
    "name": "wopal-cn/mcp-hotnews-server",
    "description": "HotNews MCP Server",
    "publisher": {
      "id": "wopal",
      "name": "wopal",
      "url": "https://github.com/wopal-cn/mcp-hotnews-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/wopal-cn/mcp-hotnews-server",
    "distribution": {
      "type": "npm",
      "package": "@wopal/mcp-server-hotnews"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@wopal/mcp-server-hotnews"
      ],
      "env": {}
    }
  },
  {
    "id": "d736a46c-c094-4d47-a917-0dcc64b8e6dc",
    "name": "mcp-server-time",
    "description": "Time MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "Model Context Protocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/time"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/time",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-time"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-time",
        "--local-time-zone=${LOCAL_TIME_ZONE}"
      ],
      "env": {
        "LOCAL_TIME_ZONE": {
          "description": "Local time zone",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "147b0052-a644-4f56-a162-07af2add42de",
    "name": "tatn/mcp-server-fetch-typescript",
    "description": "mcp-server-fetch-typescript MCP Server",
    "publisher": {
      "id": "tatn",
      "name": "tatn",
      "url": "https://github.com/tatn/mcp-server-fetch-typescript"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tatn/mcp-server-fetch-typescript",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-fetch-typescript"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-fetch-typescript"
      ],
      "env": {}
    }
  },
  {
    "id": "64f407f4-0693-4b46-9f92-6fcf6d21889d",
    "name": "PhialsBasement/nmap-mcp-server",
    "description": "MCP NMAP Server",
    "publisher": {
      "id": "PhialsBasement",
      "name": "PhialsBasement",
      "url": "https://github.com/PhialsBasement/nmap-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/PhialsBasement/nmap-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "mcp-nmap-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-nmap-server"
      ],
      "env": {}
    }
  },
  {
    "id": "42cbce41-64e9-49bb-81df-bdeb09edd381",
    "name": "JetBrains/mcp-jetbrains",
    "description": "JetBrains MCP Proxy Server",
    "publisher": {
      "id": "jetbrains",
      "name": "jetbrains",
      "url": "https://github.com/JetBrains/mcp-jetbrains"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/JetBrains/mcp-jetbrains",
    "distribution": {
      "type": "npm",
      "package": "@jetbrains/mcp-proxy"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@jetbrains/mcp-proxy"
      ],
      "env": {
        "IDE_PORT": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "HOST": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "LOG_ENABLED": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "f5e3b185-3966-4db6-86b7-7bdd748cf61c",
    "name": "iaptic/mcp-server-iaptic",
    "description": "MCP Server for Iaptic",
    "publisher": {
      "id": "iaptic",
      "name": "iaptic",
      "url": "https://github.com/iaptic/mcp-server-iaptic"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/iaptic/mcp-server-iaptic",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-iaptic"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-iaptic",
        "--api-key",
        "$ENVARG_API_KEY",
        "--app-name",
        "$ENVARG_APP_NAME"
      ],
      "env": {
        "ENVARG_API_KEY": {
          "description": "Your Iaptic API key for authentication",
          "type": "input",
          "required": true
        },
        "ENVARG_APP_NAME": {
          "description": "Your Iaptic application name",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "74a0d023-0033-4a3c-921c-1c338322e254",
    "name": "server-memory",
    "description": "Knowledge Graph Memory Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/memory"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/memory",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-memory"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-memory"
      ],
      "env": {
        "MEMORY_FILE_PATH": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "2b9d8acb-3a6f-4c06-8181-0cc67dc22576",
    "name": "Bigsy/maven-mcp-server",
    "description": "Maven Dependencies MCP Server",
    "publisher": {
      "id": "Bigsy",
      "name": "Bigsy",
      "url": "https://github.com/Bigsy/maven-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Bigsy/maven-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "mcp-maven-deps"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-maven-deps"
      ],
      "env": {}
    }
  },
  {
    "id": "750f1e9a-87fb-428a-8ee3-3397d0521f67",
    "name": "erithwik/mcp-hn",
    "description": "Hacker News MCP Server",
    "publisher": {
      "id": "erithwik",
      "name": "erithwik",
      "url": "https://github.com/erithwik/mcp-hn"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/erithwik/mcp-hn",
    "distribution": {
      "type": "pip",
      "package": "mcp-hn"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-hn"
      ],
      "env": {}
    }
  },
  {
    "id": "270cf5d5-0c15-40e9-8a5a-df1793aebbea",
    "name": "farhankaz/redis-mcp",
    "description": "Redis MCP Server",
    "publisher": {
      "id": "farhankaz",
      "name": "farhankaz",
      "url": "https://github.com/farhankaz/redis-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/farhankaz/redis-mcp",
    "distribution": {
      "type": "npm",
      "package": "redis-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "redis-mcp",
        "--redis-host",
        "$ENVARG_REDIS_HOST",
        "--redis-port",
        "$ENVARG_REDIS_PORT"
      ],
      "env": {
        "ENVARG_REDIS_HOST": {
          "description": "Redis server host address",
          "type": "input",
          "required": true
        },
        "ENVARG_REDIS_PORT": {
          "description": "Redis server port number",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "69cd0eab-cf9e-4b8a-9dbb-a5c63b90e337",
    "name": "bmorphism/say-mcp-server",
    "description": "say-mcp-server",
    "publisher": {
      "id": "bmorphism",
      "name": "bmorphism",
      "url": "https://github.com/bmorphism/say-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/bmorphism/say-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "say-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "say-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "13d7865c-c7f4-4e23-b935-c5dd7d7d0a9c",
    "name": "Ejb503/systemprompt-mcp-notion",
    "description": "SystemPrompt MCP Notion Server",
    "publisher": {
      "id": "Ejb503",
      "name": "Ejb503",
      "url": "https://github.com/Ejb503/systemprompt-mcp-notion"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Ejb503/systemprompt-mcp-notion",
    "distribution": {
      "type": "npm",
      "package": "systemprompt-mcp-notion"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "systemprompt-mcp-notion"
      ],
      "env": {
        "SYSTEMPROMPT_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "NOTION_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "7b5b1697-dbd7-43eb-be04-fba936e82e28",
    "name": "tanigami/mcp-server-perplexity",
    "description": "Perplexity MCP Server",
    "publisher": {
      "id": "tanigami",
      "name": "tanigami",
      "url": "https://github.com/tanigami/mcp-server-perplexity"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tanigami/mcp-server-perplexity",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-perplexity"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-perplexity"
      ],
      "env": {
        "PERPLEXITY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "1fa15bf5-1b11-4546-a4e3-d34dba26993b",
    "name": "ac3xx/mcp-servers-kagi",
    "description": "kagi-server MCP Server",
    "publisher": {
      "id": "ac3xx",
      "name": "ac3xx",
      "url": "https://github.com/ac3xx/mcp-servers-kagi"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ac3xx/mcp-servers-kagi",
    "distribution": {
      "type": "npm",
      "package": "kagi-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "kagi-server"
      ],
      "env": {
        "KAGI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "a1f017cc-da4d-4133-b533-23be81a1894e",
    "name": "kazuph/mcp-browser-tabs",
    "description": "MCP Browser Tabs",
    "publisher": {
      "id": "kazuph",
      "name": "kazuph",
      "url": "https://github.com/kazuph/mcp-browser-tabs"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kazuph/mcp-browser-tabs",
    "distribution": {
      "type": "npm",
      "package": "@kazuph/mcp-browser-tabs"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kazuph/mcp-browser-tabs"
      ],
      "env": {}
    }
  },
  {
    "id": "ff34a9d4-f9b4-46a1-aecd-6549bf6ba084",
    "name": "pierrebrunelle/mcp-server-openai",
    "description": "OpenAI MCP Server",
    "publisher": {
      "id": "pierrebrunelle",
      "name": "pierrebrunelle",
      "url": "https://github.com/pierrebrunelle/mcp-server-openai"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/pierrebrunelle/mcp-server-openai",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-openai"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-openai"
      ],
      "env": {
        "OPENAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "8d8a072b-8730-4ff1-871d-76ce712e542c",
    "name": "qwang07/duck-duck-mcp",
    "description": "Duck Duck MCP",
    "publisher": {
      "id": "qwang07",
      "name": "qwang07",
      "url": "https://github.com/qwang07/duck-duck-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/qwang07/duck-duck-mcp",
    "distribution": {
      "type": "npm",
      "package": "duck-duck-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "duck-duck-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "247f48d3-845c-46ea-a520-e837ca52a15d",
    "name": "paulotaylor/voyp-mcp",
    "description": "Voyp Model Context Protocol server",
    "publisher": {
      "id": "paulotaylor",
      "name": "paulotaylor",
      "url": "https://github.com/paulotaylor/voyp-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/paulotaylor/voyp-mcp",
    "distribution": {
      "type": "npm",
      "package": "voyp-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "voyp-mcp"
      ],
      "env": {
        "VOYP_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "75980efa-ebe8-4fc1-8a02-f843ef616a40",
    "name": "smithery-ai/mcp-obsidian",
    "description": "Obsidian Model Context Protocol",
    "publisher": {
      "id": "smithery-ai",
      "name": "smithery-ai",
      "url": "https://github.com/smithery-ai/mcp-obsidian"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/smithery-ai/mcp-obsidian",
    "distribution": {
      "type": "npm",
      "package": "mcp-obsidian"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-obsidian",
        "$ENVARG_VAULT_PATH"
      ],
      "env": {
        "ENVARG_VAULT_PATH": {
          "description": "Path to your Obsidian vault or Markdown notes directory",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "b79c1a73-cc23-47a1-b7d6-6f79690762db",
    "name": "recursechat/mcp-server-apple-shortcuts",
    "description": "Apple Shortcuts MCP Server 🤖",
    "publisher": {
      "id": "recursechat",
      "name": "recursechat",
      "url": "https://github.com/recursechat/mcp-server-apple-shortcuts"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/recursechat/mcp-server-apple-shortcuts",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-apple-shortcuts"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-apple-shortcuts"
      ],
      "env": {}
    }
  },
  {
    "id": "eb354c59-ceff-4353-a550-67c759fd3f9f",
    "name": "hannesrudolph/mcp-ragdocs",
    "description": "RAG Documentation MCP Server",
    "publisher": {
      "id": "hannesrudolph",
      "name": "hannesrudolph",
      "url": "https://github.com/hannesrudolph/mcp-ragdocs"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/hannesrudolph/mcp-ragdocs",
    "distribution": {
      "type": "npm",
      "package": "@hannesrudolph/mcp-ragdocs"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@hannesrudolph/mcp-ragdocs"
      ],
      "env": {
        "OPENAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "QDRANT_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "QDRANT_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "6bb809fc-5fe1-43d3-a86e-646d23d1af55",
    "name": "loonghao/wecom-bot-mcp-server",
    "description": "WeCom Bot MCP Server",
    "publisher": {
      "id": "loonghao",
      "name": "loonghao",
      "url": "https://github.com/loonghao/wecom-bot-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/loonghao/wecom-bot-mcp-server",
    "distribution": {
      "type": "pip",
      "package": "wecom-bot-mcp-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "wecom-bot-mcp-server"
      ],
      "env": {
        "WECOM_WEBHOOK_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "d41a3bfe-4d82-4d47-aa4f-1eb28fbda108",
    "name": "ZeparHyfar/mcp-datetime",
    "description": "mcp-datetime",
    "publisher": {
      "id": "ZeparHyfar",
      "name": "ZeparHyfar",
      "url": "https://github.com/ZeparHyfar/mcp-datetime"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ZeparHyfar/mcp-datetime",
    "distribution": {
      "type": "pip",
      "package": "mcp-datetime"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-datetime"
      ],
      "env": {}
    }
  },
  {
    "id": "61e88609-784b-4ea7-be94-159bcd72f34a",
    "name": "adhikasp/mcp-git-ingest",
    "description": "MCP Git Ingest",
    "publisher": {
      "id": "adhikasp",
      "name": "adhikasp",
      "url": "https://github.com/adhikasp/mcp-git-ingest"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/adhikasp/mcp-git-ingest",
    "distribution": {
      "type": "pip",
      "package": "mcp-git-ingest"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-git-ingest"
      ],
      "env": {}
    }
  },
  {
    "id": "583cfc68-3352-4792-a1c2-d045ffeea5ac",
    "name": "SeanMcLoughlin/mcp-vcd",
    "description": "mcp-vcd",
    "publisher": {
      "id": "SeanMcLoughlin",
      "name": "SeanMcLoughlin",
      "url": "https://github.com/SeanMcLoughlin/mcp-vcd"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/SeanMcLoughlin/mcp-vcd",
    "distribution": {
      "type": "pip",
      "package": "mcp-vcd"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-vcd"
      ],
      "env": {}
    }
  },
  {
    "id": "09662939-5c58-4531-89cc-5953c1c899b5",
    "name": "tokenizin-agency/mcp-npx-fetch",
    "description": "MCP NPX Fetch",
    "publisher": {
      "id": "tokenizin",
      "name": "tokenizin",
      "url": "https://github.com/tokenizin-agency/mcp-npx-fetch"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tokenizin-agency/mcp-npx-fetch",
    "distribution": {
      "type": "npm",
      "package": "@tokenizin/mcp-npx-fetch"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@tokenizin/mcp-npx-fetch"
      ],
      "env": {}
    }
  },
  {
    "id": "8a713b1f-5623-4bbc-873f-c8621c988ef5",
    "name": "henryhawke/mcp-titan",
    "description": "Titan Memory MCP Server",
    "publisher": {
      "id": "henryhawke",
      "name": "henryhawke",
      "url": "https://github.com/henryhawke/mcp-titan"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/henryhawke/mcp-titan",
    "distribution": {
      "type": "npm",
      "package": "@henryhawke/mcp-titan"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@henryhawke/mcp-titan"
      ],
      "env": {}
    }
  },
  {
    "id": "4b36e407-8f8d-4ff7-8564-c16a9cee78b0",
    "name": "makafeli/n8n-workflow-builder",
    "description": "n8n Workflow Builder MCP Server",
    "publisher": {
      "id": "makafeli",
      "name": "makafeli",
      "url": "https://github.com/makafeli/n8n-workflow-builder"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/makafeli/n8n-workflow-builder",
    "distribution": {
      "type": "npm",
      "package": "@makafeli/n8n-workflow-builder"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@makafeli/n8n-workflow-builder"
      ],
      "env": {
        "N8N_HOST": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "N8N_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "89c744c4-b138-4d4a-9d44-1bec7e104d78",
    "name": "kevinwatt/mcp-webhook",
    "description": "MCP Webhook Server",
    "publisher": {
      "id": "kevinwatt",
      "name": "kevinwatt",
      "url": "https://github.com/kevinwatt/mcp-webhook"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kevinwatt/mcp-webhook",
    "distribution": {
      "type": "npm",
      "package": "@kevinwatt/mcp-webhook"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@kevinwatt/mcp-webhook"
      ],
      "env": {
        "WEBHOOK_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "8061cc26-f63d-4ca4-9fc4-d0355ddd0af0",
    "name": "wcgw",
    "description": "Shell and Coding agent for Claude and Chatgpt",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "Model Context Protocol",
      "url": "https://github.com/rusiaaman/wcgw"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/rusiaaman/wcgw",
    "distribution": {
      "type": "pip",
      "package": "wcgw"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "wcgw"
      ],
      "env": {}
    }
  },
  {
    "id": "4677723e-39cb-494e-93c8-ff64beadd52b",
    "name": "server-puppeteer",
    "description": "Puppeteer",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/puppeteer"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/puppeteer",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-puppeteer"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-puppeteer"
      ],
      "env": {}
    }
  },
  {
    "id": "36366832-d506-475c-a0d7-6c5025bf2ad7",
    "name": "btwiuse/npm-search-mcp-server",
    "description": "npm-search MCP Server",
    "publisher": {
      "id": "btwiuse",
      "name": "btwiuse",
      "url": "https://github.com/btwiuse/npm-search-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/btwiuse/npm-search-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "npm-search-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "npm-search-mcp-server"
      ],
      "env": {}
    }
  },
  {
    "id": "92fd06f9-bd2c-408f-a80e-265cdbe0c7e2",
    "name": "BurtTheCoder/mcp-maigret",
    "description": "Maigret MCP Server",
    "publisher": {
      "id": "BurtTheCoder",
      "name": "BurtTheCoder",
      "url": "https://github.com/BurtTheCoder/mcp-maigret"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/BurtTheCoder/mcp-maigret",
    "distribution": {
      "type": "npm",
      "package": "mcp-maigret"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-maigret"
      ],
      "env": {
        "MAIGRET_REPORTS_DIR": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "9d2729c0-6060-4284-8c2a-118a31b42110",
    "name": "spences10/mcp-jinaai-search",
    "description": "mcp-jinaai-search",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-jinaai-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-jinaai-search",
    "distribution": {
      "type": "npm",
      "package": "mcp-jinaai-search"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-jinaai-search"
      ],
      "env": {
        "JINAAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "fd8ebe1f-8202-4d95-8c4f-6a34fe8293a2",
    "name": "ivo-toby/contentful-mcp",
    "description": "Contentful MCP Server",
    "publisher": {
      "id": "ivotoby",
      "name": "ivotoby",
      "url": "https://github.com/ivo-toby/contentful-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ivo-toby/contentful-mcp",
    "distribution": {
      "type": "npm",
      "package": "@ivotoby/contentful-management-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@ivotoby/contentful-management-mcp-server"
      ],
      "env": {
        "CONTENTFUL_MANAGEMENT_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "273d7ec7-1fe9-451c-94f3-a2707715956b",
    "name": "vrknetha/mcp-server-firecrawl",
    "description": "FireCrawl MCP Server",
    "publisher": {
      "id": "vrknetha",
      "name": "vrknetha",
      "url": "https://github.com/vrknetha/mcp-server-firecrawl"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/vrknetha/mcp-server-firecrawl",
    "distribution": {
      "type": "npm",
      "package": "mcp-server-firecrawl"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-server-firecrawl"
      ],
      "env": {
        "FIRE_CRAWL_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "FIRE_CRAWL_API_URL": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "750785b2-31fa-4a01-8a95-9b9f21c78de5",
    "name": "mzxrai/mcp-openai",
    "description": "MCP OpenAI Server",
    "publisher": {
      "id": "mzxrai",
      "name": "mzxrai",
      "url": "https://github.com/mzxrai/mcp-openai"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/mzxrai/mcp-openai",
    "distribution": {
      "type": "npm",
      "package": "@mzxrai/mcp-openai"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@mzxrai/mcp-openai"
      ],
      "env": {
        "OPENAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "bd9b5baf-6332-4349-92e9-3e05c45f45ad",
    "name": "server-gitlab",
    "description": "GitLab MCP Server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "modelcontextprotocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/gitlab"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/gitlab",
    "distribution": {
      "type": "npm",
      "package": "@modelcontextprotocol/server-gitlab"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-gitlab"
      ],
      "env": {
        "GITLAB_PERSONAL_ACCESS_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "GITLAB_API_URL": {
          "description": "Optional environment variable",
          "type": "string"
        }
      }
    }
  },
  {
    "id": "b193f5fd-5066-437e-a727-143567736356",
    "name": "sammcj/mcp-package-docs",
    "description": "Package Documentation MCP Server",
    "publisher": {
      "id": "sammcj",
      "name": "sammcj",
      "url": "https://github.com/sammcj/mcp-package-docs"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/sammcj/mcp-package-docs",
    "distribution": {
      "type": "npm",
      "package": "mcp-package-docs"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-package-docs"
      ],
      "env": {}
    }
  },
  {
    "id": "91f02f30-b3b9-47b7-af52-b81c8138ae6c",
    "name": "xBlueCode/findata-mcp-server",
    "description": "Financial Data - MCP Server",
    "publisher": {
      "id": "xBlueCode",
      "name": "xBlueCode",
      "url": "https://github.com/xBlueCode/findata-mcp-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/xBlueCode/findata-mcp-server",
    "distribution": {
      "type": "npm",
      "package": "findata-mcp-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "findata-mcp-server"
      ],
      "env": {
        "ALPHA_VANTAGE_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "0a9182df-e4e8-4dd9-8bc7-2b8465371111",
    "name": "tumf/mcp-text-editor",
    "description": "MCP Text Editor Server",
    "publisher": {
      "id": "tumf",
      "name": "tumf",
      "url": "https://github.com/tumf/mcp-text-editor"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tumf/mcp-text-editor",
    "distribution": {
      "type": "pip",
      "package": "mcp-text-editor"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-text-editor"
      ],
      "env": {}
    }
  },
  {
    "id": "ad933ca9-a571-4e2b-96d1-e6e417d1d20f",
    "name": "baranwang/mcp-tung-shing",
    "description": "Tung Shing MCP Server",
    "publisher": {
      "id": "baranwang",
      "name": "baranwang",
      "url": "https://github.com/baranwang/mcp-tung-shing"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/baranwang/mcp-tung-shing",
    "distribution": {
      "type": "npm",
      "package": "mcp-tung-shing"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-tung-shing"
      ],
      "env": {}
    }
  },
  {
    "id": "441fa4fe-7bc2-49f5-b5c0-b2204dd2abd1",
    "name": "cloudflare/mcp-server-cloudflare",
    "description": "Cloudflare MCP Server",
    "publisher": {
      "id": "cloudflare",
      "name": "cloudflare",
      "url": "https://github.com/cloudflare/mcp-server-cloudflare"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/cloudflare/mcp-server-cloudflare",
    "distribution": {
      "type": "npm",
      "package": "@cloudflare/mcp-server-cloudflare"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@cloudflare/mcp-server-cloudflare",
        "run",
        "$ENVARG_ACCOUNT_ID"
      ],
      "env": {
        "ENVARG_ACCOUNT_ID": {
          "description": "Your Cloudflare account ID",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "e337b900-2abe-43d5-896f-9bd1e905cb9f",
    "name": "spences10/mcp-perplexity-search",
    "description": "mcp-perplexity-search",
    "publisher": {
      "id": "spences10",
      "name": "spences10",
      "url": "https://github.com/spences10/mcp-perplexity-search"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/spences10/mcp-perplexity-search",
    "distribution": {
      "type": "npm",
      "package": "mcp-perplexity-search"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-perplexity-search"
      ],
      "env": {
        "PERPLEXITY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "ca0fd672-d9fd-4569-a930-25d55ec7ddf0",
    "name": "Hawstein/mcp-server-reddit",
    "description": "MCP Server Reddit",
    "publisher": {
      "id": "Hawstein",
      "name": "Hawstein",
      "url": "https://github.com/Hawstein/mcp-server-reddit"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/Hawstein/mcp-server-reddit",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-reddit"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-reddit"
      ],
      "env": {}
    }
  },
  {
    "id": "b32108d2-c5a3-463e-bfda-3eb799023197",
    "name": "PhialsBasement/CMD-MCP-Server",
    "description": "CMD MCP Server",
    "publisher": {
      "id": "PhialsBasement",
      "name": "PhialsBasement",
      "url": "https://github.com/PhialsBasement/CMD-MCP-Server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/PhialsBasement/CMD-MCP-Server",
    "distribution": {
      "type": "npm",
      "package": "server-cmd"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "server-cmd"
      ],
      "env": {}
    }
  },
  {
    "id": "444c6976-80f7-43d3-a51d-01f27691ad36",
    "name": "anaisbetts/mcp-youtube",
    "description": "YouTube MCP Server",
    "publisher": {
      "id": "anaisbetts",
      "name": "anaisbetts",
      "url": "https://github.com/anaisbetts/mcp-youtube"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/anaisbetts/mcp-youtube",
    "distribution": {
      "type": "npm",
      "package": "@anaisbetts/mcp-youtube"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@anaisbetts/mcp-youtube"
      ],
      "env": {}
    }
  },
  {
    "id": "e388c939-db23-45da-ab6e-fc40d3565b7b",
    "name": "ssut/Remote-MCP",
    "description": "Remote-MCP: Remote Model Context Protocol",
    "publisher": {
      "id": "remote-mcp",
      "name": "remote-mcp",
      "url": "https://github.com/ssut/Remote-MCP"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ssut/Remote-MCP",
    "distribution": {
      "type": "npm",
      "package": "@remote-mcp/client"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@remote-mcp/client"
      ],
      "env": {
        "REMOTE_MCP_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "HTTP_HEADER__Authorization": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "036916c0-a412-49ad-8231-b7203cb703dc",
    "name": "gentoro-GT/mcp-nodejs-server",
    "description": "Gentoro MCP Server",
    "publisher": {
      "id": "gentoro",
      "name": "gentoro",
      "url": "https://github.com/gentoro-GT/mcp-nodejs-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/gentoro-GT/mcp-nodejs-server",
    "distribution": {
      "type": "npm",
      "package": "@gentoro/mcp-nodejs-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@gentoro/mcp-nodejs-server"
      ],
      "env": {
        "GENTORO_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "GENTORO_BRIDGE_UID": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "GENTORO_BASE_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "3a24ad0f-626e-46d0-a70c-be6110ea049f",
    "name": "kiliczsh/mcp-mongo-server",
    "description": "MCP MongoDB Server",
    "publisher": {
      "id": "kiliczsh",
      "name": "kiliczsh",
      "url": "https://github.com/kiliczsh/mcp-mongo-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/kiliczsh/mcp-mongo-server",
    "distribution": {
      "type": "npm",
      "package": "mcp-mongo-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-mongo-server",
        "$ENVARG_MONGODB_URI"
      ],
      "env": {
        "ENVARG_MONGODB_URI": {
          "description": "MongoDB connection string including credentials and database name",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "505c3fad-8f78-4cc1-8d6e-d17b763aaed5",
    "name": "MarkusPfundstein/mcp-obsidian",
    "description": "MCP server for Obsidian",
    "publisher": {
      "id": "MarkusPfundstein",
      "name": "MarkusPfundstein",
      "url": "https://github.com/MarkusPfundstein/mcp-obsidian"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/MarkusPfundstein/mcp-obsidian",
    "distribution": {
      "type": "pip",
      "package": "mcp-obsidian"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-obsidian"
      ],
      "env": {
        "OBSIDIAN_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "36f8d614-896f-4730-90be-6346e0126287",
    "name": "burningion/video-editing-mcp",
    "description": "Video Editor MCP server",
    "publisher": {
      "id": "burningion",
      "name": "burningion",
      "url": "https://github.com/burningion/video-editing-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/burningion/video-editing-mcp",
    "distribution": {
      "type": "pip",
      "package": "video-editor-mcp"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "video-editor-mcp",
        "$ENVARG_API_KEY"
      ],
      "env": {
        "LOAD_PHOTOS_DB": {
          "description": "Optional environment variable",
          "type": "string"
        },
        "ENVARG_API_KEY": {
          "description": "Video Jungle API key from app.video-jungle.com/profile/settings",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "1d345ee9-9723-40e1-a081-00b0f5d57146",
    "name": "qpd-v/mcp-wordcounter",
    "description": "MCP Word Counter",
    "publisher": {
      "id": "qpd-v",
      "name": "qpd-v",
      "url": "https://github.com/qpd-v/mcp-wordcounter"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/qpd-v/mcp-wordcounter",
    "distribution": {
      "type": "npm",
      "package": "mcp-wordcounter"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-wordcounter"
      ],
      "env": {}
    }
  },
  {
    "id": "a34eb08b-7f03-43aa-8580-374207629fb2",
    "name": "adityak74/mcp-scholarly",
    "description": "mcp-scholarly MCP server",
    "publisher": {
      "id": "adityak74",
      "name": "adityak74",
      "url": "https://github.com/adityak74/mcp-scholarly"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/adityak74/mcp-scholarly",
    "distribution": {
      "type": "pip",
      "package": "mcp-scholarly"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-scholarly"
      ],
      "env": {}
    }
  },
  {
    "id": "ad8e1642-b146-45e3-80f8-182861c784db",
    "name": "mcp-server-sentry",
    "description": "mcp-server-sentry: A Sentry MCP server",
    "publisher": {
      "id": "modelcontextprotocol",
      "name": "Model Context Protocol",
      "url": "https://github.com/modelcontextprotocol/servers/tree/main/src/sentry"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/modelcontextprotocol/servers/tree/main/src/sentry",
    "distribution": {
      "type": "pip",
      "package": "mcp-server-sentry"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-server-sentry",
        "--auth-token",
        "$ENVARG_YOUR_SENTRY_TOKEN"
      ],
      "env": {
        "ENVARG_YOUR_SENTRY_TOKEN": {
          "description": "Your Sentry Token",
          "type": "input",
          "required": true
        }
      }
    }
  },
  {
    "id": "456b6e66-48a9-4a5e-b363-7ec526ac763b",
    "name": "ferrislucas/iterm-mcp",
    "description": "iterm-mcp ",
    "publisher": {
      "id": "ferrislucas",
      "name": "ferrislucas",
      "url": "https://github.com/ferrislucas/iterm-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/ferrislucas/iterm-mcp",
    "distribution": {
      "type": "npm",
      "package": "iterm-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "iterm-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "50858510-7c41-46b9-89f9-72ac67e0c2ee",
    "name": "tumf/mcp-shell-server",
    "description": "MCP Shell Server",
    "publisher": {
      "id": "tumf",
      "name": "tumf",
      "url": "https://github.com/tumf/mcp-shell-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tumf/mcp-shell-server",
    "distribution": {
      "type": "pip",
      "package": "mcp-shell-server"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-shell-server"
      ],
      "env": {
        "ALLOW_COMMANDS": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "62f104cd-55d7-467f-9249-aef5fe445fa0",
    "name": "pskill9/hn-server",
    "description": "Hacker News MCP Server",
    "publisher": {
      "id": "pskill9",
      "name": "pskill9",
      "url": "https://github.com/pskill9/hn-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/pskill9/hn-server",
    "distribution": {
      "type": "npm",
      "package": "hn-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "hn-server"
      ],
      "env": {}
    }
  },
  {
    "id": "37a50f63-27da-46af-8a18-6815a458db4f",
    "name": "AbdelStark/bitcoin-mcp",
    "description": "₿itcoin MCP Server",
    "publisher": {
      "id": "AbdelStark",
      "name": "AbdelStark",
      "url": "https://github.com/AbdelStark/bitcoin-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/AbdelStark/bitcoin-mcp",
    "distribution": {
      "type": "npm",
      "package": "bitcoin-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "bitcoin-mcp"
      ],
      "env": {}
    }
  },
  {
    "id": "205ac114-2216-48c0-a714-fd2137e52364",
    "name": "andybrandt/mcp-simple-openai-assistant",
    "description": "MCP Simple OpenAI Assistant",
    "publisher": {
      "id": "andybrandt",
      "name": "andybrandt",
      "url": "https://github.com/andybrandt/mcp-simple-openai-assistant"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/andybrandt/mcp-simple-openai-assistant",
    "distribution": {
      "type": "pip",
      "package": "mcp-simple-openai-assistant"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-simple-openai-assistant"
      ],
      "env": {
        "OPENAI_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "3f3e213d-9407-4bee-8081-f999bae79170",
    "name": "johnneerdael/netskope-mcp",
    "description": "Netskope NPA MCP Server",
    "publisher": {
      "id": "johnneerdael",
      "name": "johnneerdael",
      "url": "https://github.com/johnneerdael/netskope-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/johnneerdael/netskope-mcp",
    "distribution": {
      "type": "npm",
      "package": "@johnneerdael/netskope-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "@johnneerdael/netskope-mcp"
      ],
      "env": {
        "NETSKOPE_BASE_URL": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        },
        "NETSKOPE_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "6433f788-6151-429d-9d51-a662a54a6c76",
    "name": "tavily-ai/tavily-mcp",
    "description": "Tavily MCP Server 🚀",
    "publisher": {
      "id": "tavily-ai",
      "name": "tavily-ai",
      "url": "https://github.com/tavily-ai/tavily-mcp"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/tavily-ai/tavily-mcp",
    "distribution": {
      "type": "npm",
      "package": "tavily-mcp"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "tavily-mcp"
      ],
      "env": {
        "TAVILY_API_KEY": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
    }
  },
  {
    "id": "2f86ea0c-6eb8-4a82-8160-2e5785be8afb",
    "name": "crazyrabbitLTC/mcp-morpho-server",
    "description": "Morpho API MCP Server",
    "publisher": {
      "id": "crazyrabbitLTC",
      "name": "crazyrabbitLTC",
      "url": "https://github.com/crazyrabbitLTC/mcp-morpho-server"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/crazyrabbitLTC/mcp-morpho-server",
    "distribution": {
      "type": "npm",
      "package": "mcp-morpho-server"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-morpho-server"
      ],
      "env": {}
    }
  },
  {
    "id": "365305f6-3cc8-4b81-b0d3-92dbdee826f1",
    "name": "andybrandt/mcp-simple-arxiv",
    "description": "mcp-simple-arxiv",
    "publisher": {
      "id": "andybrandt",
      "name": "andybrandt",
      "url": "https://github.com/andybrandt/mcp-simple-arxiv"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/andybrandt/mcp-simple-arxiv",
    "distribution": {
      "type": "pip",
      "package": "mcp-simple-arxiv"
    },
    "license": "MIT",
    "runtime": "python",
    "config": {
      "command": "uvx",
      "args": [
        "mcp-simple-arxiv"
      ],
      "env": {}
    }
  },
  {
    "id": "c3a6d5f8-412f-4fc2-a484-2ab1ff55a74f",
    "name": "deepfates/mcp-replicate",
    "description": "Replicate MCP Server",
    "publisher": {
      "id": "deepfates",
      "name": "deepfates",
      "url": "https://github.com/deepfates/mcp-replicate"
    },
    "isOfficial": false,
    "sourceUrl": "https://github.com/deepfates/mcp-replicate",
    "distribution": {
      "type": "npm",
      "package": "mcp-replicate"
    },
    "license": "MIT",
    "runtime": "node",
    "config": {
      "command": "npx",
      "args": [
        "-y",
        "mcp-replicate"
      ],
      "env": {
        "REPLICATE_API_TOKEN": {
          "description": "Required environment variable",
          "type": "string",
          "required": true
        }
      }
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