#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

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
# Progress bar
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

# ============================================================
# Step 1: Clean binaries and assembly files
# ============================================================
section "Cleaning binaries and assembly files"

CODE_DIR="./code"

if [ ! -d "$CODE_DIR" ]; then
  fail "Directory '$CODE_DIR' does not exist."
  exit 1
fi

# Collect files to remove
FILES=$(find "$CODE_DIR" -type f \( ! -name "*.*" -o -name "*.s" \))

if [ -z "$FILES" ]; then
  warn "No binaries or assembly files found."
else
  total=$(echo "$FILES" | wc -l)
  current=0

  echo -e "${BOLD}Found $total files to remove...${NC}"

  while IFS= read -r file; do
    ((current++))
    rm -f "$file"
    progress_bar $current $total
    echo -ne " Removing: ${file}\r"
    sleep 0.05  # just to make progress visible
  done <<< "$FILES"

  echo "" # newline after progress bar
  success "Removed $total files (binary + assembly)"
fi

# ============================================================
# Step 2: Clean all 'target' folders inside code folder and its subfolders
# ============================================================
section "Cleaning 'target' folders"

TARGET_DIRS=$(find "$CODE_DIR" -type d -name "target")

if [ -z "$TARGET_DIRS" ]; then
  warn "No 'target' folders found."
else
  total=$(echo "$TARGET_DIRS" | wc -l)
  current=0

  echo -e "${BOLD}Found $total 'target' folders to remove...${NC}"

  while IFS= read -r dir; do
    ((current++))
    rm -rf "$dir"
    progress_bar $current $total
    echo -ne " Removing: ${dir}\r"
    sleep 0.05  # just to make progress visible
  done <<< "$TARGET_DIRS"

  echo "" # newline after progress bar
  success "Removed $total 'target' folders"
fi
