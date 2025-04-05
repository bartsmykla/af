#!/usr/bin/env bash

# common::check_env checks that a given environment variable is set and non-empty.
# It logs which variable is being checked, and exits with an error if the variable is missing.
common::check_env() {
  local var=$1
  printf 'Checking env var: %s\n' "$var"

  if [[ -z "${!var:-}" ]]; then
    printf 'Missing env var: %s\n' "$var" >&2
    exit 1
  fi
}
