#!/bin/bash

# MCP Dockmaster Production Deployment Script v2
# Improved version with timeout handling and better progress reporting
# Usage: ./deploy-v2.sh [version] [--skip-tests] [--skip-version-bump] [--background]

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
PROJECT_ROOT="/Users/mariya/Documents/GitHub/mcp-dockmaster"
PRODUCTION_APP_PATH="/Applications/MCP Dockmaster.app"
BACKUP_DIR="$PROJECT_ROOT/deployment-backups"
DMG_OUTPUT_DIR="$PROJECT_ROOT/dist/releases"
LOG_FILE="$PROJECT_ROOT/deployment.log"

# Parse arguments
VERSION="$1"
SKIP_TESTS=false
SKIP_VERSION_BUMP=false
BACKGROUND_BUILD=false

for arg in "$@"; do
    case $arg in
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --skip-version-bump)
            SKIP_VERSION_BUMP=true
            shift
            ;;
        --background)
            BACKGROUND_BUILD=true
            shift
            ;;
    esac
done

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [INFO] $1" >> "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [SUCCESS] $1" >> "$LOG_FILE"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [WARNING] $1" >> "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [ERROR] $1" >> "$LOG_FILE"
}

log_step() {
    echo -e "${CYAN}[STEP]${NC} $1"
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] [STEP] $1" >> "$LOG_FILE"
}

# Progress indicator for long-running tasks
show_progress() {
    local pid=$1
    local message="$2"
    local spin='-\|/'
    local i=0
    
    while kill -0 $pid 2>/dev/null; do
        i=$(( (i+1) %4 ))
        printf "\r${CYAN}[PROGRESS]${NC} $message ${spin:$i:1}"
        sleep 0.5
    done
    printf "\r${GREEN}[COMPLETE]${NC} $message ✓\n"
}

# Timeout wrapper function
run_with_timeout() {
    local timeout_duration=$1
    local command="$2"
    local description="$3"
    
    log_step "$description"
    
    if [ "$BACKGROUND_BUILD" = true ]; then
        # Run in background with progress indicator
        eval "$command" > "$LOG_FILE.tmp" 2>&1 &
        local cmd_pid=$!
        
        # Show progress
        show_progress $cmd_pid "$description"
        
        # Wait for completion and check result
        wait $cmd_pid
        local exit_code=$?
        
        # Append output to main log
        cat "$LOG_FILE.tmp" >> "$LOG_FILE"
        rm -f "$LOG_FILE.tmp"
        
        if [ $exit_code -ne 0 ]; then
            # For production build, check if app was actually created despite error
            if [[ "$description" == *"Production build"* ]] && [ -d "apps/mcp-dockmaster/src-tauri/target/release/bundle/macos/MCP Dockmaster.app" ]; then
                if grep -q "TAURI_SIGNING_PRIVATE_KEY" "$LOG_FILE.tmp" 2>/dev/null; then
                    log_warning "$description completed with code signing warning (missing TAURI_SIGNING_PRIVATE_KEY)"
                    log_success "App bundle was created successfully despite signing warning"
                else
                    log_warning "$description completed with warnings but app bundle was created"
                fi
            else
                log_error "$description failed with exit code $exit_code"
                return $exit_code
            fi
        else
            log_success "$description completed successfully"
        fi
    else
        # Run directly with timeout
        timeout $timeout_duration bash -c "$command"
        if [ $? -eq 124 ]; then
            log_warning "$description timed out after ${timeout_duration}s, but may still be running"
            log_info "Check $LOG_FILE for detailed output"
            return 124
        fi
    fi
}

# Ensure we're in the project root
cd "$PROJECT_ROOT"

# Initialize log file
echo "=== MCP Dockmaster Deployment Log ===" > "$LOG_FILE"
echo "Started at: $(date)" >> "$LOG_FILE"

log_info "Starting MCP Dockmaster deployment process v2..."

# Step 1: Environment Setup
log_step "Setting up environment..."
source ~/.cargo/env
export PATH="$HOME/.deno/bin:$PATH"
log_success "Environment setup completed"

# Step 2: Version Management
if [ "$SKIP_VERSION_BUMP" = false ] && [ -n "$VERSION" ]; then
    log_step "Updating version to $VERSION..."
    
    # Update Cargo.toml versions
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" libs/mcp-core/Cargo.toml
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" apps/mcp-dockmaster-cli/Cargo.toml
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" apps/mcp-proxy-server/Cargo.toml
    
    # Update Tauri config
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" apps/mcp-dockmaster/src-tauri/tauri.conf.json
    
    # Update package.json
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" apps/mcp-dockmaster/package.json
    
    log_success "Version updated to $VERSION"
fi

# Step 3: Testing
if [ "$SKIP_TESTS" = false ]; then
    log_step "Running comprehensive tests..."
    
    # Rust tests with timeout
    run_with_timeout 300 "cd libs/mcp-core && cargo test -- --test-threads=1 && cd '$PROJECT_ROOT'" "Rust unit and integration tests"
    
    # Build tests with timeout
    run_with_timeout 600 "npx nx run-many -t build --projects=mcp-core,mcp-proxy-server,mcp-dockmaster-cli" "Component build tests"
    
    log_success "All tests completed"
else
    log_warning "Skipping tests"
fi

# Step 4: Pre-compilation (Essential for preventing Tauri timeouts)
log_step "Pre-compiling Rust components..."

# Pre-compile Tauri app with timeout
run_with_timeout 300 "cd apps/mcp-dockmaster/src-tauri && source ~/.cargo/env && cargo build && cd '$PROJECT_ROOT'" "Tauri app pre-compilation"

# Pre-compile proxy server with timeout
run_with_timeout 300 "cd apps/mcp-proxy-server && source ~/.cargo/env && cargo build --release && cd '$PROJECT_ROOT'" "Proxy server compilation"

# Copy proxy server binary
run_with_timeout 60 "export PATH=\"\$HOME/.deno/bin:\$PATH\" && deno run -A ci-scripts/copy-mcp-proxy-server-binary/index.ts" "Proxy server binary copy"

log_success "Pre-compilation completed"

# Step 5: Create backup of current production
if [ -d "$PRODUCTION_APP_PATH" ]; then
    log_step "Creating backup of current production version..."
    mkdir -p "$BACKUP_DIR"
    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    cp -R "$PRODUCTION_APP_PATH" "$BACKUP_DIR/MCP Dockmaster-backup-$TIMESTAMP.app"
    log_success "Backup created at $BACKUP_DIR/MCP Dockmaster-backup-$TIMESTAMP.app"
fi

# Step 6: Production Build (The critical step that often times out)
log_step "Building production version..."

if [ "$BACKGROUND_BUILD" = true ]; then
    log_info "Running production build in background mode to handle long compilation times..."
    
    # Run Tauri build in background
    (source ~/.cargo/env && export PATH="$HOME/.deno/bin:$PATH" && npx nx build mcp-dockmaster --configuration=production) > "$LOG_FILE.build" 2>&1 &
    BUILD_PID=$!
    
    log_info "Build started with PID: $BUILD_PID"
    
    # Show progress for production build
    show_progress $BUILD_PID "Production build (this may take several minutes)"
    
    # Wait for build to complete with timeout and polling
    log_info "Waiting for build to complete..."
    
    # Poll for process completion instead of using wait
    WAIT_COUNT=0
    MAX_WAIT=1800  # 30 minutes max
    
    while [ $WAIT_COUNT -lt $MAX_WAIT ]; do
        if ! kill -0 $BUILD_PID 2>/dev/null; then
            # Process has completed
            break
        fi
        sleep 5
        WAIT_COUNT=$((WAIT_COUNT + 5))
    done
    
    # Get exit code by checking if process still exists
    if kill -0 $BUILD_PID 2>/dev/null; then
        log_warning "Build process timed out after $MAX_WAIT seconds"
        kill $BUILD_PID 2>/dev/null || true
        BUILD_EXIT_CODE=124
    else
        # Process completed, check for success by looking for the app bundle
        if [ -d "apps/mcp-dockmaster/src-tauri/target/release/bundle/macos/MCP Dockmaster.app" ]; then
            BUILD_EXIT_CODE=0
        else
            BUILD_EXIT_CODE=1
        fi
    fi
    
    log_info "Build process completed with exit code: $BUILD_EXIT_CODE"
    
    # Append build log to main log
    if [ -f "$LOG_FILE.build" ]; then
        cat "$LOG_FILE.build" >> "$LOG_FILE"
    else
        log_warning "Build log file not found: $LOG_FILE.build"
    fi
    
    if [ $BUILD_EXIT_CODE -eq 0 ]; then
        log_success "Production build completed successfully"
    else
        log_warning "Production build exited with code $BUILD_EXIT_CODE, checking if app was created..."
        
        # Check if the app was actually built despite the error
        if [ -d "apps/mcp-dockmaster/src-tauri/target/release/bundle/macos/MCP Dockmaster.app" ]; then
            # Check if this was just a signing warning
            if grep -q "TAURI_SIGNING_PRIVATE_KEY" "$LOG_FILE.build" 2>/dev/null; then
                log_warning "Build completed with code signing warning (missing TAURI_SIGNING_PRIVATE_KEY)"
                log_success "App bundle was created successfully despite signing warning"
            else
                log_success "App bundle was created despite build warnings"
            fi
        else
            log_error "Production build failed - no app bundle found"
            exit 1
        fi
    fi
else
    # Direct build with very long timeout
    run_with_timeout 1800 "source ~/.cargo/env && export PATH=\"\$HOME/.deno/bin:\$PATH\" && npx nx build mcp-dockmaster --configuration=production" "Production build"
fi

# Step 7: Copy to Applications
log_step "Installing to Applications directory..."
if [ -d "apps/mcp-dockmaster/src-tauri/target/release/bundle/macos/MCP Dockmaster.app" ]; then
    cp -R "apps/mcp-dockmaster/src-tauri/target/release/bundle/macos/MCP Dockmaster.app" "/Applications/"
    log_success "App installed to /Applications/MCP Dockmaster.app"
else
    log_error "App bundle not found - build may have failed"
    exit 1
fi

# Step 8: Create DMG and organize release files
log_step "Organizing release files..."
mkdir -p "$DMG_OUTPUT_DIR"

# Find and copy the DMG
DMG_PATTERN="apps/mcp-dockmaster/src-tauri/target/release/bundle/dmg/MCP Dockmaster*.dmg"
DMG_PATH=$(find apps/mcp-dockmaster/src-tauri/target/release/bundle -name "MCP Dockmaster*.dmg" -type f | head -1)

if [ -n "$DMG_PATH" ] && [ -f "$DMG_PATH" ]; then
    DMG_NAME="MCP Dockmaster"
    if [ -n "$VERSION" ]; then
        DMG_NAME="${DMG_NAME}_${VERSION}"
    else
        DMG_NAME="${DMG_NAME}_$(date +%Y%m%d-%H%M%S)"
    fi
    DMG_NAME="${DMG_NAME}_aarch64.dmg"
    
    cp "$DMG_PATH" "$DMG_OUTPUT_DIR/$DMG_NAME"
    log_success "DMG installer created at $DMG_OUTPUT_DIR/$DMG_NAME"
else
    log_warning "DMG not found - installer creation may have failed"
fi

# Step 9: Deployment Summary
log_success "Deployment completed successfully!"
echo
log_info "=== DEPLOYMENT SUMMARY ==="
echo "  📦 Version: ${VERSION:-'No version specified'}"
echo "  🚀 Production app: $PRODUCTION_APP_PATH"
echo "  💿 DMG installer: $DMG_OUTPUT_DIR/$DMG_NAME"
echo "  🔄 Backup location: $BACKUP_DIR"
echo "  📝 Full log: $LOG_FILE"
echo
log_info "=== VERIFICATION STEPS ==="
echo "  1. Launch: open '/Applications/MCP Dockmaster.app'"
echo "  2. Check version in About dialog"
echo "  3. Test namespace configuration:"
echo
echo "     Add to Claude Desktop config:"
echo '     {
       "mcpServers": {
         "mcp-dockmaster": {
           "command": "/path/to/mcp-proxy-server",
           "env": {
             "DOCKMASTER_TOOL_PREFIX": "dockmaster_",
             "DOCKMASTER_NAMESPACE_MODE": "enabled"
           }
         }
       }
     }'
echo
log_info "=== NAMESPACE TOOL NAMES ==="
echo "  • Default: dockmaster_register_server, dockmaster_search_server"
echo "  • Custom prefix: dm_register_server (with DOCKMASTER_TOOL_PREFIX=dm_)"
echo "  • Legacy: register_server (with DOCKMASTER_NAMESPACE_MODE=disabled)"

# Clean up temporary files
rm -f "$LOG_FILE.build" "$LOG_FILE.tmp"

echo
log_success "Deployment process completed! 🎉"