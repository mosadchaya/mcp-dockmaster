# Custom Server Patterns Project Status

## 🎯 Goal
Enable MCP Dockmaster to support custom server installations beyond standard npm/pip packages, specifically:
- Local filesystem paths (e.g., `/path/to/server.js`)
- Complex command structures (e.g., `uv run --directory /path`)
- Custom environment variables

## 📍 Current Status: Phase 1 ✅ COMPLETE

### What's Been Done
1. **Environment Setup**
   - ✅ Rust installed (v1.87.0)
   - ✅ Dependencies installed (`npm ci`)
   - ✅ CLI built successfully

2. **Export Functionality**
   - ✅ Created `libs/mcp-core/src/database/migration.rs`
   - ✅ Built CLI with export command
   - ✅ Created Node.js export script as backup
   - ✅ Successfully exported 11 servers from production Dockmaster

3. **Key Files Created/Modified**
   - `libs/mcp-core/src/database/migration.rs` - Rust migration module
   - `apps/mcp-dockmaster-cli/src/export.rs` - CLI export command
   - `export-servers.js` - Quick Node.js export script
   - `exported-servers.json` - Contains all 11 exported servers

## 🚀 Next: Phase 2 - Database Schema Extension

### Goals
- Extend schema to support server types: `package`, `local`, `custom`
- Add fields for working directory, complex args, template variables
- Maintain backward compatibility

### Key Decisions Needed
1. How to structure the new server type enum?
2. Should we add a new table or extend the existing `servers` table?
3. How to handle template variables in paths/env vars?

## 📝 Quick Commands

```bash
# Activate Rust (if needed after terminal restart)
source "$HOME/.cargo/env"

# Build the CLI
npx nx build mcp-dockmaster-cli

# Run export
./apps/mcp-dockmaster-cli/target/debug/mcp-dockmaster-cli export --output new-export.json

# Quick export without building
node export-servers.js output.json

# View current database schema
sqlite3 "/Users/mariya/Library/Application Support/com.mcp-dockmaster.desktop/mcp_dockmaster.db" ".schema"

# Check Claude Desktop config
cat "/Users/mariya/Library/Application Support/Claude/claude_desktop_config.json" | jq
```

## 🔗 Important Paths
- **Production DB**: `/Users/mariya/Library/Application Support/com.mcp-dockmaster.desktop/mcp_dockmaster.db`
- **Local Fork DB**: `/Users/mariya/Library/Application Support/com.mcp.dockmaster/mcp_dockmaster.db`
- **Claude Config**: `/Users/mariya/Library/Application Support/Claude/claude_desktop_config.json`

## 📦 Custom Servers to Migrate
1. **clanki**: Local Node.js server at `/Users/mariya/Documents/GitHub/clanki/build/index.js`
2. **mcp-google-sheets-local**: Python server with `uv run --directory` and env vars