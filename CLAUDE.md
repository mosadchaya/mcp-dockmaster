# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Start for Custom Server Patterns Project

**Current Status**: Phase 1 ✅ Complete | Working on Phase 2

**Next Steps**:
1. Ensure Rust is available: `source "$HOME/.cargo/env"`
2. Check exported servers: `cat exported-servers.json`
3. Continue with Phase 2: Database schema extension for custom server types

**Key Commands**:
```bash
# Build CLI
npx nx build mcp-dockmaster-cli

# Export servers from installed Dockmaster
./apps/mcp-dockmaster-cli/target/debug/mcp-dockmaster-cli export --output servers.json

# Quick export (Node.js, no build needed)
node export-servers.js
```

## Commands

### Development
```bash
# Install all dependencies
npm ci

# Start the desktop app with hot reload
npx nx serve mcp-dockmaster

# Run all tests
npx nx run-many -t test

# Run specific test suites
npx nx test mcp-core           # Test Rust core library
npx nx test:all-rust          # All Rust tests
npx nx test:all-ts            # All TypeScript tests

# Build all projects
npx nx run-many -t build

# Lint all projects
npx nx run-many -t lint
```

### Rust-specific Commands
```bash
# Run Rust tests with single thread (required for database tests)
cargo test -- --test-threads=1

# Check Rust formatting
cargo fmt --check

# Run Rust linter
cargo clippy
```

## Architecture

MCP Dockmaster is a monorepo managed by Nx with the following structure:

### Core Applications

1. **Desktop App** (`apps/mcp-dockmaster/`)
   - Tauri-based desktop application
   - React frontend with Tailwind CSS
   - Uses Tanstack Query for data fetching
   - Zustand for state management
   - Auto-updater built-in

2. **MCP Core Library** (`libs/mcp-core/`)
   - Shared Rust library for all Rust applications
   - SQLite database with Diesel ORM
   - MCP server implementation using `rmcp` SDK
   - Registry management and tool discovery
   - Installation helpers for Claude and Cursor integration

3. **MCP Proxy Server** (`apps/mcp-proxy-server/`)
   - Rust-based proxy for MCP protocol
   - Distributed as sidecar binary with desktop app
   - Uses Tokio for async runtime

### Key Concepts

**MCP (Model Context Protocol)**: A protocol that allows AI assistants to interact with external tools and data sources. MCP servers provide capabilities like file access, API integrations, and computational tools.

**Registry**: A curated marketplace of MCP servers that users can browse and install. Registry data is cached locally and searched using Lunr.js.

**Server Management**: The app manages MCP server lifecycles, including:
- Installation (Node.js, Python, Docker-based servers)
- Configuration (environment variables, arguments)
- Integration with AI assistants (automatic config file updates)
- Process management (start/stop/restart)

### Database Schema

SQLite database managed by Diesel with migrations:
- `servers`: MCP server configurations
- `server_env`: Environment variables for servers
- `server_tools`: Tools exposed by each server
- `app_settings`: User preferences and app configuration

### Frontend Structure

- **Pages**: Home, Registry, About (in `src/pages/`)
- **Components**: Reusable UI components using Radix UI primitives
- **API Layer**: `src/api/` contains queries and mutations using Tanstack Query
- **Tauri Commands**: Exposed in `src-tauri/src/lib.rs`

### Important Files

- `src-tauri/tauri.conf.json`: Tauri configuration
- `libs/mcp-core/src/core/mcp_core.rs`: Core MCP functionality
- `src/lib/mcpClient.ts`: Frontend MCP client interface
- `src/lib/process.ts`: Server process management

## Active Project: Custom Server Patterns Support

### Project Overview
Extending MCP Dockmaster to support custom server patterns beyond standard npm/pip/docker packages. This enables installation of local development servers with complex configurations.

### Current Limitations
- Only supports standard package patterns (npm/pip packages via npx/uvx)
- Cannot install servers from local filesystem paths
- Limited support for complex environment variables and custom launch commands

### Project Setup & Prerequisites

#### Required Tools
- ✅ **Rust** (installed via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- ✅ **Node.js** v18+ (already installed)
- ✅ **SQLite3** (for database inspection)

#### Key Paths
- **Installed Dockmaster DB**: `/Users/mariya/Library/Application Support/com.mcp-dockmaster.desktop/mcp_dockmaster.db`
- **Claude Desktop Config**: `/Users/mariya/Library/Application Support/Claude/claude_desktop_config.json`
- **Local Fork DB**: `/Users/mariya/Library/Application Support/com.mcp.dockmaster/mcp_dockmaster.db`

#### Export Tools Created
1. **Node.js Export Script**: `export-servers.js` - Quick export without building
2. **Rust CLI Export**: `apps/mcp-dockmaster-cli/` with export command
   - Build: `npx nx build mcp-dockmaster-cli`
   - Run: `./apps/mcp-dockmaster-cli/target/debug/mcp-dockmaster-cli export --output <file>`

### Implementation Phases

#### Phase 1: Migration Foundation ✅ COMPLETED
- [x] Analyzed current database schema in `libs/mcp-core/src/schema/`
- [x] Created migration utilities in `libs/mcp-core/src/database/migration.rs`
- [x] Implemented export functionality for server configurations
- [x] Tested with existing server installations (exported 11 servers)

**Phase 1 Outputs:**
- `exported-servers.json` - Contains all 11 servers from installed Dockmaster
- `libs/mcp-core/src/database/migration.rs` - Rust migration module
- Export functionality integrated into CLI

#### Phase 2: Database Schema Extension ⏳
- [ ] Extend SQLite schema for flexible server types (package/local/custom)
- [ ] Add complex arguments and environment variable support
- [ ] Implement backward-compatible migrations
- [ ] Create validation framework

#### Phase 3: Backend API Extensions ⏳
- [ ] Extend Rust commands in `src-tauri/src/lib.rs`
- [ ] Implement path validation and template resolution
- [ ] Add Claude Desktop config import/export
- [ ] Create lifecycle management for custom patterns

#### Phase 4: Frontend UI Enhancement ⏳
- [ ] Design custom server configuration forms
- [ ] Add file browser for local path selection
- [ ] Create environment variable editor with templates
- [ ] Build import wizard for Claude Desktop configs

#### Phase 5: Proxy Server Updates ⏳
- [ ] Extend `apps/mcp-proxy-server/` for custom server types
- [ ] Implement argument/env template resolution
- [ ] Add error handling for custom patterns
- [ ] Ensure proxy architecture compatibility

#### Phase 6: Migration & Testing ⏳
- [ ] Migrate example custom servers (clanki, mcp-google-sheets-local)
- [ ] Test various server patterns
- [ ] Performance validation
- [ ] Documentation and examples

### Key Files for This Project
- Database schema: `libs/mcp-core/migrations/sqlite/`
- Server models: `libs/mcp-core/src/models/`
- Backend commands: `src-tauri/src/lib.rs`, `src-tauri/src/features/`
- Frontend forms: `src/components/InstalledServers.tsx`
- Proxy implementation: `apps/mcp-proxy-server/src/server/`

### Custom Servers to Support

From Claude Desktop config (`claude_desktop_config.json`):

1. **clanki** (Local Node.js server)
   ```json
   {
     "command": "node",
     "args": ["/Users/mariya/Documents/GitHub/clanki/build/index.js"]
   }
   ```
   - Uses direct file path instead of package
   - Requires Node.js runtime

2. **mcp-google-sheets-local** (Local Python server with complex args)
   ```json
   {
     "command": "uv",
     "args": ["run", "--directory", "/Users/mariya/Documents/GitHub/mcp-google-sheets", "mcp-google-sheets"],
     "env": {
       "SERVICE_ACCOUNT_PATH": "/Users/mariya/Documents/Claude/ai-tools-461922-40cad3b52259.json",
       "DRIVE_FOLDER_ID": "1eHtude9t5F5TJ4zrLyT1awBB-Iprjzq-"
     }
   }
   ```
   - Uses `uv run` with directory argument
   - Complex environment variables with file paths
   - Not a standard pip package installation