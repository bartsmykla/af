#!/usr/bin/env bash

common::check_env() {
  local var=$1
  [[ -z "${!var:-}" ]] && { printf 'Missing env var: %s\n' "$var" >&2; exit 1; }
}
