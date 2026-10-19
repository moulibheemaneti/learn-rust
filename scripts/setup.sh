#!/bin/bash

# Other bash points:

: '
# The -e flag enables interpretation of backslash escapes (like \033 for colors) in the string.
'

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Step 1: Check if bun exists or not. If not then install the bun.
section "Checking for Bun"

# Explanation:
#   > /dev/null      : Redirects standard output (stdout) to /dev/null, discarding it.
#   2>&1             : Redirects standard error (stderr, file descriptor 2) to wherever stdout (1) is going (here, /dev/null).
#   So, both stdout and stderr from 'bun --version' are suppressed.
if ! bun --version > /dev/null 2>&1; then
  warn "Bun not found. Attempting to install Bun..."
  curl -fsSL https://bun.sh/install | bash
  export BUN_INSTALL="$HOME/.bun"
  export PATH="$BUN_INSTALL/bin:$PATH"
  success "Bun version $(bun --version) installed."
else
  success "Bun version $(bun --version) already installed."
fi

# Step 2: Check if rust exists or not. If not then install the rust.
section "Checking for Rust"

if ! rustc --version > /dev/null 2>&1; then
  warn "Rust not found. Attempting to install Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  success "Rust version $(rustc --version) installed and set as default."
else
  success "Rust version $(rustc --version) already installed and set as default."
fi

# Step 3: Check if cargo exists or not. If not then install the cargo.
section "Checking for Cargo"

if ! cargo --version > /dev/null 2>&1; then
  warn "Cargo not found. Attempting to install Cargo..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  success "Cargo version $(cargo --version) installed and set as default."
else
  success "Cargo version $(cargo --version) already installed and set as default."
fi

# Step 4: Install dependencies
section "Installing dependencies with Bun"

if ! bun install --frozen-lockfile; then
  fail "Failed to install dependencies with Bun."
  exit 1
else
  success "Dependencies installed successfully with Bun."
fi

print_line
echo -e "${GREEN}${BOLD}✨ Setup complete! ✨${NC}"
print_line
