#!/bin/bash

set -e

# Extract current version from Cargo.toml
CURRENT_VERSION=$(grep -m1 'version = ' Cargo.toml | cut -d '"' -f2)
echo "Current version: $CURRENT_VERSION"

# Ask for new version
read -p "Enter new version (or press enter to use current version): " NEW_VERSION
NEW_VERSION=${NEW_VERSION:-$CURRENT_VERSION}

# Update version in Cargo.toml
sed -i '' "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Commit changes
git add Cargo.toml
git commit -m "Bump version to $NEW_VERSION"

# Create tag
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

echo "Changes committed and tagged as v$NEW_VERSION"
echo "Run 'git push && git push --tags' to trigger the release workflow"
