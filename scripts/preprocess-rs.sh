#!/usr/bin/env bash
# Preprocesses the raw OpenAPI spec for progenitor (committed as api/openapi.rs.json).
set -euo pipefail
cd "$(dirname "$0")/.."

INPUT="${1:-api/openapi.json}"
OUTPUT="${2:-api/openapi.rs.json}"

python3 - "$INPUT" "$OUTPUT" <<'EOF'
import json, sys

spec = json.load(open(sys.argv[1]))

# 1. Drop multipart operations (progenitor has no multipart support).
for p in ["/v1/audio/transcriptions", "/v1/images/edits"]:
    spec["paths"].pop(p, None)

# 2. Normalize integer booleans (OpenAPI 3.0 allows 0/1; progenitor requires real bools).
def fix(obj):
    if isinstance(obj, dict):
        for k, v in list(obj.items()):
            if k in ("exclusiveMinimum", "exclusiveMaximum", "deprecated", "uniqueItems", "readOnly", "writeOnly", "nullable") and type(v) is int:
                obj[k] = bool(v)
            else:
                fix(v)
    elif isinstance(obj, list):
        for v in obj:
            fix(v)

fix(spec)

# 3. Normalize error responses to a single type (progenitor asserts one error type per operation).
for path, item in spec["paths"].items():
    for m in ("get", "post", "put", "patch", "delete"):
        op = item.get(m)
        if not op:
            continue
        for sc, resp in op.get("responses", {}).items():
            if sc.startswith("2") or sc == "default":
                continue
            content = resp.get("content", {})
            if "application/json" in content:
                content["application/json"] = {"schema": {"$ref": "#/components/schemas/ErrorResponse"}}

json.dump(spec, open(sys.argv[2], "w"), indent=2)
print("preprocessed ->", sys.argv[2])
EOF
