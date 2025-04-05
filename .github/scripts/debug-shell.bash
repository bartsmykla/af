#!/usr/bin/env bash

set -euo pipefail

source "${BASH_SOURCE%/*}/_common.bash"

common::check_env TUNSHELL_SECRET

url_init="${URL_INIT:-https://lets.tunshell.com/init.sh}"
tunshell_relay="${TUNSHELL_RELAY:-eu.relay.tunshell.com}"
url_api_sessions="${URL_API_SESSIONS:-https://$tunshell_relay/api/sessions}"

tunshell_keys="$(curl --silent --show-error --fail --request POST "$url_api_sessions")"
peer1_key="$(echo "$tunshell_keys" | jq --raw-output .peer1_key)"
peer2_key="$(echo "$tunshell_keys" | jq --raw-output .peer2_key)"

printf '# Debug Shell:\nsh %s L %s %s %s\n' \
  "\$(curl --silent --show-error --fail $url_init | psub)" \
  "$peer2_key" \
  "\$TUNSHELL_SECRET" \
  "$tunshell_relay"

curl --silent --show-error --fail "$url_init" | sh -s -- T "$peer1_key" "$TUNSHELL_SECRET" "$tunshell_relay"
