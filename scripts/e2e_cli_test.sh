#!/bin/bash
# End-to-End CLI Verification Script for Cadence (cad)
set -e

echo "=================================================="
echo "🧪 Running Cadence End-to-End Verification Suite..."
echo "=================================================="

SANDBOX_DIR="/tmp/cadence-test-repo"
REMOTE_DIR="/tmp/cadence-dummy-remote.git"

# Cleanup any previous runs
rm -rf "$SANDBOX_DIR" "$REMOTE_DIR"

echo "\n[Step 1/6] Setting up sandbox environment..."
mkdir -p "$REMOTE_DIR"
git init --bare "$REMOTE_DIR" > /dev/null

mkdir -p "$SANDBOX_DIR"
cd "$SANDBOX_DIR"
git init > /dev/null
git checkout -b main > /dev/null
git remote add origin "$REMOTE_DIR"
git config user.name "Cadence Tester"
git config user.email "tester@example.com"

# Seed initial commit
echo "seed" > seed.txt
git add -A
git commit -m "Initial commit\n\nRelease-Date: 2026-01-01" > /dev/null
git push -u origin main > /dev/null

# Create .cadence.json config
cat << 'EOF' > .cadence.json
{
  "repo_path": ".",
  "remote": "origin",
  "branch": "main",
  "timezone": "Asia/Kolkata"
}
EOF

TODAY=$(date +"%Y-%m-%d")
echo "✔ Sandbox setup complete at $SANDBOX_DIR"

echo "\n[Step 2/6] Testing 'cad commit' (Today's Date: $TODAY)..."
echo "feature 1" > file1.txt
cadence commit "Add feature 1"
git log -1 | grep -q "Release-Date: $TODAY"
echo "✔ 'cad commit' assigned correct Release-Date: $TODAY"

echo "\n[Step 3/6] Testing 'cad commit --date' (Future Date: 2026-12-31)..."
echo "feature 2" > file2.txt
cadence commit "Add future feature" --date 2026-12-31
git log -1 | grep -q "Release-Date: 2026-12-31"
echo "✔ 'cad commit --date 2026-12-31' assigned correct future trailer"

echo "\n[Step 4/6] Testing 'cad status'..."
cadence status

echo "\n[Step 5/6] Testing 'cad push' (Push Pacing Decision)..."
cadence push

LOCAL_COUNT=$(git rev-list --count HEAD)
REMOTE_COUNT=$(git --git-dir="$REMOTE_DIR" rev-list --count main)

echo "Local commits: $LOCAL_COUNT | Remote commits: $REMOTE_COUNT"
if [ "$REMOTE_COUNT" -eq 2 ] && [ "$LOCAL_COUNT" -eq 3 ]; then
    echo "✔ Push Pacing SUCCESS: Due commit pushed to remote; future commit retained in local queue!"
else
    echo "❌ Push Pacing FAILED: Unexpected commit counts."
    exit 1
fi

echo "\n[Step 6/6] Testing 'cad relabel'..."
cadence relabel HEAD 2026-11-30
git log -1 | grep -q "Release-Date: 2026-11-30"
echo "✔ 'cad relabel' updated commit trailer to 2026-11-30"

# Cleanup sandbox
cd /tmp
rm -rf "$SANDBOX_DIR" "$REMOTE_DIR"

echo "\n=================================================="
echo "✅ All Cadence E2E CLI Checks Passed Successfully!"
echo "=================================================="
