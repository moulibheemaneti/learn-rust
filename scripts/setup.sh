#!/bin/bash

# Other bash points:

: '
# The -e flag enables interpretation of backslash escapes (like \033 for colors) in the string.
'

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# NVM setup
section "Checking for nvm & .nvmrc Node version"

NVM_DIR=$HOME/.nvm

# Step 1: Check if source of NVM exists or not. If not throw an error.

# Explanation:
#   > /dev/null      : Redirects standard output (stdout) to /dev/null, discarding it.
#   2>&1             : Redirects standard error (stderr, file descriptor 2) to wherever stdout (1) is going (here, /dev/null).
#   So, both stdout and stderr from 'source' are suppressed.
if ! source "$NVM_DIR/nvm.sh" > /dev/null 2>&1; then
  fail "Unable to load NVM. File not found at: ${BOLD}$NVM_DIR/nvm.sh${NC}"
  exit 1
fi

# Step 2: Execute the "nvm use" command and then if errors come, try to install using "nvm install".
if ! nvm use > /dev/null 2>&1; then
  warn "Attempting to use the Node.js version specified in .nvmrc..."

  if ! nvm install > /dev/null 2>&1; then
    fail "Unable to install the specified Node.js version with NVM. Please ensure the version in .nvmrc is available."
    exit 1
  fi
  nvm use > /dev/null 2>&1
  success "Node.js version $(nvm version) installed and set as default."
else
  success "Node.js version $(nvm version) already installed and set as default."
fi

# Step 3: Check if rust exists or not. If not then install the rust.
section "Checking for Rust"

if ! rustc --version > /dev/null 2>&1; then
  warn "Rust not found. Attempting to install Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  success "Rust version $(rustc --version) installed and set as default."
else
  success "Rust version $(rustc --version) already installed and set as default."
fi

# Step 4: Check if cargo exists or not. If not then install the cargo.
section "Checking for Cargo"

if ! cargo --version > /dev/null 2>&1; then
  warn "Cargo not found. Attempting to install Cargo..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  success "Cargo version $(cargo --version) installed and set as default."
else
  success "Cargo version $(cargo --version) already installed and set as default."
fi

# Step 5: Enable Corepack
section "Enabling Corepack"

if command -v corepack > /dev/null 2>&1; then
  if corepack enable > /dev/null 2>&1; then
    success "Corepack enabled successfully."
  else
    warn "Corepack is installed but could not be enabled. You may need to enable it manually."
  fi
else
  warn "Corepack is not available with your Node.js installation. Skipping Corepack enable step."
fi

# Step 6: Install Node.js dependencies
section "Installing Node.js dependencies"

if [ -f "yarn.lock" ]; then
  # Remove other lock files to avoid conflicts
  if [ -f "package-lock.json" ]; then
    rm -f package-lock.json
    warn "Removed package-lock.json to avoid conflicts with yarn.lock."
  fi
  if [ -f "pnpm-lock.yaml" ]; then
    rm -f pnpm-lock.yaml
    warn "Removed pnpm-lock.yaml to avoid conflicts with yarn.lock."
  fi
  if ! yarn install --immutable; then
    fail "Failed to install Node.js dependencies with yarn."
    exit 1
  else
    success "Node.js dependencies installed successfully with yarn."
  fi
elif [ -f "package-lock.json" ]; then
  if ! npm ci; then
    fail "Failed to install Node.js dependencies with npm."
    exit 1
  else
    success "Node.js dependencies installed successfully with npm."
  fi
elif [ -f "pnpm-lock.yaml" ]; then
  if ! pnpm install --frozen-lockfile; then
    fail "Failed to install Node.js dependencies with pnpm."
    exit 1
  else
    success "Node.js dependencies installed successfully with pnpm."
  fi
else
  warn "No lockfile found. Proceeding to install Node.js dependencies with yarn."
  if ! yarn install; then
    fail "Failed to install Node.js dependencies with yarn."
    exit 1
  else
    success "Node.js dependencies installed successfully with yarn."
  fi
fi

print_line
echo -e "${GREEN}${BOLD}✨ Setup complete! ✨${NC}"
print_line
