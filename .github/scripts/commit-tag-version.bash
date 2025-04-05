#!/usr/bin/env bash

set -euo pipefail

source "${BASH_SOURCE%/*}/_common.bash"

common::check_env GITHUB_REPOSITORY
common::check_env GITHUB_REF_NAME
common::check_env NEW_TAG

# Prepare --field flags for each changed file
files=()
while IFS= read -r f; do
  files+=(--field "files[][path]=$f")
  files+=(--field "files[][contents]=$(base64 -w0 "$f")")
done < <(git status --porcelain | awk '{print $2}')

# Commit changes
new_sha=$(
  gh api graphql \
    --jq '.data.createCommitOnBranch.commit.oid' \
    --field 'query=@.github/api/queries/createCommit.graphql' \
    --field "githubRepository=$GITHUB_REPOSITORY" \
    --field "branchName=$GITHUB_REF_NAME" \
    --field "expectedHeadOid=$(git rev-parse HEAD)" \
    --field "commitMessage=$NEW_TAG" \
    "${files[@]}"
)

# Create a new tag
gh api graphql \
  --field 'query=@.github/api/queries/createRef.graphql' \
  --field "repoId=$(gh repo view "$GITHUB_REPOSITORY" --json id --jq .id)" \
  --field "name=refs/tags/$NEW_TAG" \
  --field "sha=$new_sha"
