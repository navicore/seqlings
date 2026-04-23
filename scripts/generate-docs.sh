#!/bin/bash
# Regenerate docs/README.md from the repo-root README.md.
# Convert `##` headers to bold so they don't clutter the mdBook sidebar.
set -e
sed 's/^## \(.*\)/**\1**/' README.md > docs/README.md
