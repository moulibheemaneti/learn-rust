#!/bin/bash

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

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
