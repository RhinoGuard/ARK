#!/usr/bin/env bash
set -euo pipefail

if [[ "${GITHUB_EVENT_NAME:-}" != "pull_request" ]]; then
    echo "Commit-message validation runs on pull requests."
    exit 0
fi

base_ref="${GITHUB_BASE_REF:?GITHUB_BASE_REF must be set for pull_request events}"
git fetch --no-tags origin "${base_ref}:refs/remotes/origin/${base_ref}"
base_sha="$(git merge-base "refs/remotes/origin/${base_ref}" HEAD)"
commit_shas="$(git rev-list --no-merges "${base_sha}..HEAD")"

if [[ -z "${commit_shas}" ]]; then
    echo "No non-merge commits found in the pull request."
    exit 0
fi

pattern='^(feat|fix|refactor|perf|test|docs|build|ci|chore|revert|security)(\([a-z0-9][a-z0-9._/-]*\))?!?: .{1,72}$'
failed=0

while IFS= read -r commit_sha; do
    [[ -n "${commit_sha}" ]] || continue
    subject="$(git log -1 --format=%s "${commit_sha}")"

    if [[ ! "${subject}" =~ ${pattern} ]]; then
        printf 'Invalid commit subject: %s\n' "${subject}"
        failed=1
    else
        printf 'Valid commit subject: %s\n' "${subject}"
    fi
done <<< "${commit_shas}"

if [[ "${failed}" -ne 0 ]]; then
    echo "Use a Conventional Commit header: type(scope): summary; scope may be omitted for cross-cutting changes (summary <= 72 characters)."
    exit 1
fi
