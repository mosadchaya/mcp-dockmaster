# MCP Dockmaster Monorepo

This is a monorepo for the MCP Dockmaster project, managed with NX.

## Structure

- `apps/mcp-dockmaster`: The main Tauri application
- `apps/mcp-proxy-server`: The MCP proxy server

## Getting Started

### Prerequisites

- Node.js (v18 or later)
- npm (v8 or later)

### Installation

1. Clone the repository
2. Install dependencies:

```bash
npm install
```

## Development

### Running applications

To run the Dockmaster application:

```bash
npx nx dev mcp-dockmaster
# or for Tauri development
npx nx tauri:dev mcp-dockmaster
```

To build the MCP Runner:

```bash
npx nx build mcp-proxy-server
```

### Running commands across all projects

```bash
# Build all projects
npm run build

# Run tests across all projects
npm run test
```

## Using NX

### Running tasks

```bash
# Run a task for a specific project
npx nx <task> <project>

# Examples:
npx nx build mcp-dockmaster
npx nx dev mcp-dockmaster
```

### Visualizing the project graph

```bash
npx nx graph
```

### Running tasks in parallel

```bash
npx nx run-many --target=build --parallel=3
```

### Affected commands

```bash
# Run tasks only for projects affected by changes
npx nx affected --target=build
```

## Learn More

- [NX Documentation](https://nx.dev) 