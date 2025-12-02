#!/usr/bin/env bash

set -euo pipefail

# shellcheck source=./_common.bash
source "${BASH_SOURCE%/*}/_common.bash"

common::check_env NEW_VERSION

# Update Cargo.toml
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update Cargo.lock
awk -v new_version="$NEW_VERSION" '
  $0 == "[[package]]" { in_package = 1; print; next }
  in_package && /^name = "af"/ { found_af = 1; print; next }
  in_package && found_af && /^version = / {
    print "version = \"" new_version "\""
    in_package = 0; found_af = 0
    next
  }
  { print }
' Cargo.lock > Cargo.lock.tmp && mv Cargo.lock.tmp Cargo.lock

# Update flake.nix if it exists
if [[ -f flake.nix ]]; then
  sed -i "s/version = \"[^\"]*\";/version = \"$NEW_VERSION\";/" flake.nix
  sed -i "s/rev = \"v[^\"]*\";/rev = \"v$NEW_VERSION\";/" flake.nix

  # Update the source hash by prefetching from GitHub
  owner=$(sed -n 's/.*owner = "\([^"]*\)".*/\1/p' flake.nix | head -1)
  repo=$(sed -n 's/.*repo = "\([^"]*\)".*/\1/p' flake.nix | head -1)
  current_commit=$(git rev-parse HEAD)

  # Use nix-prefetch-url to get the hash for the tarball
  new_hash=$(nix-prefetch-url --unpack "https://github.com/$owner/$repo/archive/$current_commit.tar.gz" 2>/dev/null | tail -1)

  # Convert to SRI hash format (sha256-...)
  new_hash_sri=$(nix hash convert --hash-algo sha256 --to sri "$new_hash")

  # Update the hash in flake.nix
  sed -i "s|hash = \"sha256-[^\"]*\";|hash = \"$new_hash_sri\";|" flake.nix

  # Note: cargoLock.lockFile = ./Cargo.lock is used, so no cargoHash update needed
fi
