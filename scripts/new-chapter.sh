#!/bin/bash

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Script title
section "📚 NEW CHAPTER CREATOR"

# Check if argument is provided
if [ $# -eq 0 ]; then
    fail "No chapter number provided"
    echo -e "${CYAN}Usage:${NC} $0 <chapter_number>"
    echo -e "${CYAN}Example:${NC} $0 2"
    exit 1
fi

# Get the chapter number from argument
chapter_num=$1

section "🔍 VALIDATION"

# Validate chapter number (must be a positive integer between 1-99)
if ! [[ "$chapter_num" =~ ^[0-9]+$ ]] || [ "$chapter_num" -le 0 ] || [ "$chapter_num" -gt 99 ]; then
    fail "Chapter number must be a positive integer between 1-99"
    exit 1
fi

success "Chapter number '$chapter_num' is valid"

# Format chapter number with leading zero if needed (pad to 2 digits)
if [ ${#chapter_num} -eq 1 ]; then
    chapter_num="0$chapter_num"
fi

# Create chapter directory name
chapter_dir="chapter$chapter_num"

section "📁 DIRECTORY SETUP"

# Get the script directory and project root
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
project_root="$(dirname "$script_dir")"

# Create directories in docs and code folders
docs_dir="$project_root/docs/$chapter_dir"
code_dir="$project_root/code/$chapter_dir"

echo -e "${CYAN}Project root:${NC} $project_root"
echo -e "${CYAN}Chapter directory:${NC} $chapter_dir"

section "🏗️  CREATING DIRECTORIES"

# Create docs directory
if [ -d "$docs_dir" ]; then
    warn "Docs directory already exists: $docs_dir"
else
    mkdir -p "$docs_dir"
    success "Created docs directory: $docs_dir"
fi

# Create code directory
if [ -d "$code_dir" ]; then
    warn "Code directory already exists: $code_dir"
else
    mkdir -p "$code_dir"
    success "Created code directory: $code_dir"
fi

section "🎉 COMPLETION"

success "Chapter $chapter_num setup complete!"
echo -e "${CYAN}You can now add your documentation and code files to:${NC}"
echo -e "  📖 ${GREEN}$docs_dir${NC}"
echo -e "  💻 ${GREEN}$code_dir${NC}"
print_line
