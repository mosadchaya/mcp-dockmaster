# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## ⚠️ Important: Working Directory
**ALWAYS work from the repository root**: `/Users/mariya/Documents/GitHub/mcp-dockmaster/`
- This file is located at the repository root
- If you're in a subdirectory (e.g., `libs/mcp-core/`), this file won't be found
- Use relative paths from the root: `libs/mcp-core/src/`, `apps/mcp-dockmaster/`, etc.

## Quick Start for Custom Server Patterns Project

**Current Status**: Phase 4 ✅ UI Complete | Custom Server Registration Ready

**Next Steps**:
1. Pre-compile Rust components (see Dev Server Instructions below)
2. Start dev server: `nohup npx nx serve mcp-dockmaster > dev-server.log 2>&1 &`
3. Test custom server registration at: `http://localhost:1420/custom-registry`

**Key Commands**:
```bash
# Build CLI
npx nx build mcp-dockmaster-cli

# Export servers from installed Dockmaster
./apps/mcp-dockmaster-cli/target/debug/mcp-dockmaster-cli export --output servers.json

# Import Claude Desktop custom servers
./apps/mcp-dockmaster-cli/target/debug/mcp-dockmaster-cli import-claude --config "/Users/mariya/Library/Application Support/Claude/claude_desktop_config.json" --output custom-servers.json

# Quick export (Node.js, no build needed)
node export-servers.js
```

## Commands

### Development
```bash
# Install all dependencies
npm ci

# Ensure all tools are available
source ~/.cargo/env && export PATH="$HOME/.deno/bin:$PATH"

# ⚠️ IMPORTANT: Pre-compile Rust first (see Dev Server Instructions above)
# Then start the desktop app with hot reload
nohup npx nx serve mcp-dockmaster > dev-server.log 2>&1 &

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
- ✅ **Deno** (installed via `curl -fsSL https://deno.land/install.sh | sh`)
- ✅ **Node.js** v18+ (already installed)
- ✅ **SQLite3** (for database inspection)

#### Environment Setup
```bash
# Source Rust environment (required for cargo commands)
source ~/.cargo/env

# Add Deno to PATH (required for build scripts)
export PATH="$HOME/.deno/bin:$PATH"

# Install all dependencies
npm ci
```

#### Dev Server Instructions

**⚠️ IMPORTANT**: The initial Rust compilation takes 30-45 seconds and may cause timeouts. Always pre-compile first!

**Pre-compilation Steps** (run these first):
```bash
# 1. Pre-compile Tauri app (takes ~35 seconds first time)
cd /Users/mariya/Documents/GitHub/mcp-dockmaster/apps/mcp-dockmaster/src-tauri
source ~/.cargo/env && cargo build

# 2. Pre-compile proxy server
cd /Users/mariya/Documents/GitHub/mcp-dockmaster/apps/mcp-proxy-server
source ~/.cargo/env && cargo build --release

# 3. Copy proxy server binary
cd /Users/mariya/Documents/GitHub/mcp-dockmaster
export PATH="$HOME/.deno/bin:$PATH" && deno run -A ci-scripts/copy-mcp-proxy-server-binary/index.ts
```

**Start Dev Server**:
```bash
# After pre-compilation, start dev server in background
source ~/.cargo/env && export PATH="$HOME/.deno/bin:$PATH"
nohup npx nx serve mcp-dockmaster > dev-server.log 2>&1 &

# Check if server is running (should return HTTP/1.1 200 OK)
sleep 5 && curl -s -I http://localhost:1420 | head -1
```

**Access the Application**:
- **Web Browser**: `http://localhost:1420`
- **Desktop App**: Opens automatically (native Tauri window)
- **Custom Server Registry**: `http://localhost:1420/custom-registry`

**Troubleshooting**:
- If `ERR_CONNECTION_REFUSED`: Server isn't running, check dev-server.log
- If timeout during start: Always pre-compile Rust components first
- If cargo not found: Run `source ~/.cargo/env`
- If deno not found: Run `export PATH="$HOME/.deno/bin:$PATH"`

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

#### Phase 2: Database Schema Extension ✅ COMPLETED
- [x] Extended SQLite schema for flexible server types (package/local/custom)
- [x] Added complex arguments and environment variable support
- [x] Implemented backward-compatible migrations
- [x] Created validation framework and import functionality

**Phase 2 Outputs:**
- `libs/mcp-core/migrations/sqlite/2025-06-10-000001_add_custom_server_support/` - Database migration
- Extended schema with `server_type`, `working_directory`, `executable_path` fields
- `claude-imports.json` - Successfully imported clanki and mcp-google-sheets-local
- CLI import command: `import-claude` for Claude Desktop config import

**Phase 3 Outputs:**
- `libs/mcp-core/src/validation.rs` - Comprehensive validation framework for custom servers
- `libs/mcp-core/src/core/mcp_core_proxy_ext.rs` - Extended with `register_custom_server` method
- `apps/mcp-dockmaster/src-tauri/src/features/mcp_proxy.rs` - Added `register_custom_server` Tauri command
- Template resolution for environment variables (`$HOME`, `$USER`, `$SHELL`, etc.)
- Runtime dependency validation (Node.js, Python, uv, Docker)
- Path validation with relative/absolute path support
- Security checks to prevent path traversal attacks

**Phase 3 Key Features:**
- **Generic Custom Server Support**: Can register ANY custom server, not just specific examples
- **Runtime Detection**: Validates Node.js, Python, uv, Docker availability before registration
- **Template Variables**: Resolves `$HOME`, `$USER`, `$SHELL` in paths and environment variables
- **Path Safety**: Validates executable permissions and prevents dangerous path operations
- **Command Building**: Intelligently constructs commands based on runtime and executable type
- **Validation Framework**: Comprehensive error reporting with detailed validation messages
- **Lifecycle Integration**: Custom servers start/stop/restart like standard package servers

#### Phase 3: Backend API Extensions ✅ COMPLETED
- [x] Extend Tauri commands to support custom server types (local/custom)
- [ ] Add server registration API for local filesystem servers (UI + CLI)
- [x] Implement comprehensive path validation:
  - [x] Support both relative and absolute paths
  - [x] Validate executable files exist and are executable
  - [x] Validate working directories exist
  - [x] Check runtime dependencies (Node.js, Python, uv, etc.)
- [x] Create lifecycle management for custom server patterns (start/stop/restart)
- [x] Add environment variable template resolution (`$HOME`, `$USER`, etc.)
- [ ] Extend CLI with `add-custom-server` command

**Phase 3 Success Criteria:**
- Users can add **any** custom server through Dockmaster UI/CLI (not just specific examples)
- System validates required runtimes (Node.js, Python, uv, etc.) are available
- Environment variables like `$HOME`, `$USER` are properly resolved
- Both relative (`./build/index.js`) and absolute paths work
- Custom servers start/stop/restart like standard package servers

**Phase 3 CLI Interface:**
```bash
# Generic pattern for adding any custom server
./mcp-dockmaster-cli add-custom-server \
  --name <server-name> \
  --description "<description>" \
  --type <package|local|custom> \
  --runtime <node|python|docker|custom> \
  [--executable <path>] \
  [--command <command>] \
  [--args <arg1,arg2,arg3>] \
  [--working-dir <path>] \
  [--env KEY=value] \
  [--env KEY2=value2]

# Examples:
# Local Node.js server
./mcp-dockmaster-cli add-custom-server \
  --name my-node-server --type local --runtime node \
  --executable ./dist/server.js --working-dir /path/to/project

# Local Python server with uv
./mcp-dockmaster-cli add-custom-server \
  --name my-python-server --type local --runtime python \
  --command uv --args "run,--directory,$HOME/project,server" \
  --env API_KEY=secret --env DATA_PATH=$HOME/data

# Custom binary
./mcp-dockmaster-cli add-custom-server \
  --name my-binary --type custom --runtime custom \
  --executable /usr/local/bin/my-server --args "--port,8080"
```

#### Phase 4: Frontend UI Enhancement ✅ COMPLETED
- [x] Add "Custom Server Registry" to sidebar navigation (under "MCP Server Registry")
- [x] Create Custom Server Registry page with similar layout to MCP Server Registry
- [x] Move "Import from Github" button to Custom Server Registry page
- [x] Add "Add Custom Server" button with modal popup form (same style as Import dialog)
- [x] Design custom server configuration form with all fields:
  - [x] Name (required), Description (required)
  - [x] Server Type dropdown (package/local/custom)
  - [x] Runtime dropdown (node/python/docker/custom)
  - [x] Command (optional), Executable Path (optional, file input)
  - [x] Arguments (optional, text input), Working Directory (optional, text input)
  - [x] Environment Variables (optional, key-value editor with add/remove)
- [x] Implement form validation and integration with register_custom_server API

**Phase 4 Outputs:**
- `apps/mcp-dockmaster/src/components/CustomServerRegistry.tsx` - Complete custom server management UI
- `apps/mcp-dockmaster/src/components/ui/select.tsx` - Radix UI Select component integration
- `apps/mcp-dockmaster/src/components/icons.tsx` - Added CustomServerIcon
- Updated navigation in `App.tsx` with new route and sidebar item
- Removed GitHub import from standard Registry component (now in Custom Registry)

**Phase 4 Key Features:**
- **Complete Custom Server UI**: Full-featured modal form with all configuration options
- **Visual Server Type Selection**: Dropdowns for server types and runtimes with validation
- **Environment Variable Management**: Dynamic key-value editor with add/remove functionality
- **GitHub Import Integration**: Moved GitHub import to appropriate custom server section
- **Form Validation**: Client-side validation with error display and user feedback
- **Responsive Design**: Modal layout with proper scrolling for complex forms

#### Phase 5: Proxy Server Updates ⏳
- [ ] Extend `apps/mcp-proxy-server/` for custom server types
- [ ] Implement argument/env template resolution
- [ ] Add error handling for custom patterns
- [ ] Ensure proxy architecture compatibility

#### Phase 6: Migration & Testing ⏳
- [ ] Test with example custom servers (clanki, mcp-google-sheets-local as test cases)
- [ ] Test various server patterns and edge cases
- [ ] Performance validation with mixed server types
- [ ] Documentation and examples for custom server patterns

### Key Files for This Project
- Database schema: `libs/mcp-core/migrations/sqlite/`
- Server models: `libs/mcp-core/src/models/`
- Backend commands: `src-tauri/src/lib.rs`, `src-tauri/src/features/`
- Frontend forms: `src/components/InstalledServers.tsx`
- Proxy implementation: `apps/mcp-proxy-server/src/server/`

### Custom Servers to Support

From Claude Desktop config (`claude_desktop_config.json`) - **✅ Successfully Imported**:

1. **clanki** (Local Node.js server) ✅ 
   ```json
   {
     "command": "node",
     "args": ["/Users/mariya/Documents/GitHub/clanki/build/index.js"]
   }
   ```
   - ✅ Detected as `server_type: "local"`, `tools_type: "node"`
   - ✅ Executable path captured: `/Users/mariya/Documents/GitHub/clanki/build/index.js`

2. **mcp-google-sheets-local** (Local Python server with complex args) ✅
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
   - ✅ Detected as `server_type: "local"`, `tools_type: "python"`
   - ✅ Working directory captured: `/Users/mariya/Documents/GitHub/mcp-google-sheets`
   - ✅ Environment variables imported with metadata