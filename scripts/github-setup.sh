#!/usr/bin/env bash
# github-setup.sh — Initialize mdANSI as a GitHub repository
# Run from the mdANSI project root.
#
# Prerequisites:
#   - gh CLI authenticated (gh auth status)
#   - git configured (user.name, user.email)
#
# Usage:
#   chmod +x scripts/github-setup.sh
#   ./scripts/github-setup.sh

set -euo pipefail

REPO_NAME="mdANSI"
REPO_OWNER="justinhuangcode"
DESCRIPTION="A blazing-fast Markdown-to-ANSI terminal renderer with built-in syntax highlighting"

echo "=== mdANSI GitHub Repository Setup ==="
echo ""

# ── Step 1: Initialize local git repo ─────────────────────────────────
if [ ! -d .git ]; then
    echo "[1/6] Initializing git repository..."
    git init
    git add -A
    git commit -m "Initial commit: mdANSI v0.1.0

Markdown-to-ANSI terminal renderer with:
- Built-in syntax highlighting (200+ languages via syntect)
- Full GFM support (tables, task lists, strikethrough, footnotes)
- Streaming mode for LLM output
- TOML-configurable themes
- Unicode-aware text wrapping with CJK/emoji support
- OSC-8 hyperlinks
- Single static binary (~4MB)"
else
    echo "[1/6] Git repository already initialized."
fi

# ── Step 2: Create GitHub repository ──────────────────────────────────
echo "[2/6] Creating GitHub repository..."
if gh repo view "${REPO_OWNER}/${REPO_NAME}" &>/dev/null; then
    echo "  Repository ${REPO_OWNER}/${REPO_NAME} already exists."
else
    gh repo create "${REPO_OWNER}/${REPO_NAME}" \
        --public \
        --description "${DESCRIPTION}" \
        --homepage "https://crates.io/crates/mdansi" \
        --source . \
        --remote origin
    echo "  Repository created."
fi

# ── Step 3: Push to GitHub ────────────────────────────────────────────
echo "[3/6] Pushing to GitHub..."
git branch -M main
git push -u origin main

# ── Step 4: Configure repository settings via GitHub API ──────────────
echo "[4/6] Configuring repository settings..."
gh api -X PATCH "repos/${REPO_OWNER}/${REPO_NAME}" \
    -f has_issues=true \
    -f has_projects=false \
    -f has_wiki=false \
    -f allow_squash_merge=true \
    -f allow_merge_commit=false \
    -f allow_rebase_merge=true \
    -f delete_branch_on_merge=true \
    --silent
echo "  Settings configured (squash merge, delete branch on merge, no wiki)."

# ── Step 5: Set repository topics ─────────────────────────────────────
echo "[5/6] Setting repository topics..."
gh api -X PUT "repos/${REPO_OWNER}/${REPO_NAME}/topics" \
    -f names='["markdown","ansi","terminal","cli","rust","syntax-highlighting","llm","streaming","theme"]' \
    --silent
echo "  Topics set."

# ── Step 6: Create branch protection for main ─────────────────────────
echo "[6/6] Setting up branch protection..."
gh api -X PUT "repos/${REPO_OWNER}/${REPO_NAME}/branches/main/protection" \
    --input - <<'EOF' --silent
{
    "required_status_checks": {
        "strict": true,
        "contexts": ["Check", "Test (ubuntu-latest)", "Clippy", "Format"]
    },
    "enforce_admins": false,
    "required_pull_request_reviews": null,
    "restrictions": null
}
EOF
echo "  Branch protection configured (CI checks required)."

echo ""
echo "=== Setup complete! ==="
echo ""
echo "Repository: https://github.com/${REPO_OWNER}/${REPO_NAME}"
echo ""
echo "Next steps:"
echo "  1. Verify CI passes: gh run list"
echo "  2. Publish to crates.io: cargo publish"
echo "  3. Create a release: gh release create v0.1.0 --generate-notes"
