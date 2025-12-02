#!/usr/bin/env bash

set -euo pipefail

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
fi
