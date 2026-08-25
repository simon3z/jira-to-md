#!/usr/bin/env bash
# CI quality gate for jira-to-md.
# Usage: ./ci.sh
set -euo pipefail
cd "$(dirname "$0")"

cargo fmt --all --check
cargo clippy --release --all -- -D warnings
cargo test --release --all

# Cognitive complexity threshold (SonarSource spec).
arborist src/ --threshold 40 --exceeds-only

# Install JS tooling from the committed lockfile, then lint + format the pi extension
npm ci
npm run check

echo "✓ All checks passed."
