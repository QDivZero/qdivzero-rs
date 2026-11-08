#!/usr/bin/env bash
# Preprocesses the raw OpenAPI spec for progenitor (committed as api/openapi.rs.json).
set -euo pipefail
cd "$(dirname "$0")/.."

INPUT="${1:-api/openapi.json}"
OUTPUT="${2:-api/openapi.rs.json}"

python3 - "$INPUT" "$OUTPUT" <<'EOF'
import hashlib, json, re, sys

spec = json.load(open(sys.argv[1]))
methods = ("get", "post", "put", "patch", "delete", "head", "options", "trace")

# 1. Drop multipart operations (progenitor has no multipart support).
for path, item in list(spec["paths"].items()):
    for method in methods:
        op = item.get(method)
        if isinstance(op, dict) and "multipart/form-data" in op.get("requestBody", {}).get("content", {}):
            del item[method]
    if not any(method in item for method in methods):
        del spec["paths"][path]

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

# 3. Give operations without IDs a deterministic, unique ID for progenitor.
used_ids = {
    op["operationId"]
    for item in spec["paths"].values()
    for method, op in item.items()
    if method in methods and isinstance(op, dict) and op.get("operationId")
}
for path, item in sorted(spec["paths"].items()):
    for method in methods:
        op = item.get(method)
        if not isinstance(op, dict) or str(op.get("operationId") or "").strip():
            continue
        slug = re.sub(r"[^a-zA-Z0-9]+", "_", path).strip("_").lower()
        digest = hashlib.sha256(f"{method} {path}".encode()).hexdigest()[:10]
        candidate = f"generated_{method}_{slug}_{digest}"
        while candidate in used_ids:
            candidate += "_"
        op["operationId"] = candidate
        used_ids.add(candidate)

# 4. Preserve unconstrained bodies when the source supplies only examples.
for item in spec["paths"].values():
    for method in methods:
        op = item.get(method)
        if not isinstance(op, dict):
            continue
        for media in op.get("requestBody", {}).get("content", {}).values():
            media.setdefault("schema", {})
        for response in op.get("responses", {}).values():
            for media in response.get("content", {}).values():
                media.setdefault("schema", {})

# 5. Progenitor cannot represent multiple successful response types. Preserve
#    every success status and body as a raw byte stream when types differ.
def response_kind(response):
    content = response.get("content", {})
    if "application/json" in content:
        return ("json", json.dumps(content["application/json"]["schema"], sort_keys=True))
    return ("raw",) if content else ("empty",)

for item in spec["paths"].values():
    for method in methods:
        op = item.get(method)
        if not isinstance(op, dict):
            continue
        successful = [response for status, response in op.get("responses", {}).items() if status.startswith("2")]
        if len({response_kind(response) for response in successful}) > 1:
            for response in successful:
                response["content"] = {"application/octet-stream": {"schema": {"type": "string", "format": "binary"}}}

# 6. Normalize error responses to a single type (progenitor asserts one error type per operation).
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
