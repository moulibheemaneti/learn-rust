#!/bin/bash

# ============================================================
# Common Utilities for Scripts
# ============================================================

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# ============================================================
# Utility Functions
# ============================================================

# Decorative line
print_line() {
  echo -e "${MAGENTA}────────────────────────────────────────────────────────────${NC}"
}

# Section header
section() {
  print_line
  echo -e "${BOLD}${BLUE}$1${NC}"
  print_line
}

# Success message
success() {
  echo -e "${GREEN}${BOLD}✔ $1${NC}"
}

# Warning message
warn() {
  echo -e "${YELLOW}${BOLD}⚠ $1${NC}"
}

# Error message
fail() {
  echo -e "${RED}${BOLD}✖ $1${NC}"
}

# ============================================================
# Progress bar (used in clean-binaries.sh)
# ============================================================
progress_bar() {
  local current=$1
  local total=$2
  local width=40  # bar width
  local percent=$(( current * 100 / total ))
  local filled=$(( current * width / total ))
  local empty=$(( width - filled ))

  # Build bar string
  local bar=$(printf "%${filled}s" | tr ' ' '#')
  local spaces=$(printf "%${empty}s")

  echo -ne "${CYAN}[${bar}${spaces}] ${percent}%${NC}\r"
}
