#!/usr/bin/env bash
# Regenerates the qdivzero-rs client from the live OpenAPI spec and publishes
# a new v1.0.<n> tag when the spec changed. Runs locally or in CI.
set -euo pipefail
cd "$(dirname "$0")/.."

SPEC_URL="${SPEC_URL:-https://api.qdiv0.com/openapi}"

RAW="$(mktemp)"
trap 'rm -f "$RAW"' EXIT

echo "==> fetching spec: $SPEC_URL"
curl -fsSL "$SPEC_URL" -o "$RAW"

if cmp -s "$RAW" api/openapi.json; then
  echo "==> spec unchanged, nothing to do"
  exit 0
fi

echo "==> preprocessing for progenitor"
cp "$RAW" api/openapi.json
scripts/preprocess-rs.sh api/openapi.json api/openapi.rs.json

echo "==> regenerating client"
cargo build

echo "==> verifying"
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

HASH="$(python3 -c 'import hashlib,sys; print(hashlib.sha256(open(sys.argv[1],"rb").read()).hexdigest()[:12])' "$RAW")"
LATEST="$(git tag --sort=-v:refname | grep '^v1\.0\.' | head -n1 || true)"
if [ -z "$LATEST" ]; then
  NEXT="v1.0.1"
else
  BASE="${LATEST#v}"
  REST="${BASE#*.}"
  PATCH="${REST#*.}"
  NEXT="v1.0.$((PATCH + 1))"
fi

echo "==> committing and tagging $NEXT"
git config user.name "${GIT_AUTHOR_NAME:-github-actions[bot]}"
git config user.email "${GIT_AUTHOR_EMAIL:-41898282+github-actions[bot]@users.noreply.github.com}"
git add api/openapi.json api/openapi.rs.json src/lib_gen.rs Cargo.lock
git commit -m "chore: regenerate from OpenAPI spec (hash $HASH)"
git push origin "HEAD:main"
git tag "$NEXT"
git push origin "$NEXT"
echo "==> released $NEXT"
