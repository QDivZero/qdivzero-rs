import json
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class PreprocessTests(unittest.TestCase):
    def test_missing_operation_ids_are_stable_and_unique(self):
        spec = {
            "openapi": "3.0.3",
            "paths": {
                "/health": {"get": {"operationId": "", "responses": {}}},
                "/items/{id}": {"get": {"responses": {}}},
                "/items/{name}": {"get": {"responses": {}}},
                "/upload": {
                    "get": {"operationId": "listUploads", "responses": {}},
                    "post": {"requestBody": {"content": {"multipart/form-data": {}}}, "responses": {}},
                },
                "/mixed": {
                    "post": {
                        "operationId": "mixedResponse",
                        "responses": {
                            "200": {"content": {"application/json": {"schema": {"type": "object"}}}},
                            "204": {"description": "No content"},
                        },
                    }
                },
                "/items": {
                    "post": {
                        "operationId": "createItem",
                        "responses": {"200": {"content": {"application/json": {"example": {"ok": True}}}}},
                        "requestBody": {"content": {"application/json": {"example": {"name": "item"}}}},
                    }
                },
            },
        }
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / "source.json"
            output = Path(directory) / "result.json"
            source.write_text(json.dumps(spec))
            subprocess.run([ROOT / "scripts/preprocess-rs.sh", source, output], check=True)
            result = json.loads(output.read_text())
            ids = [op.get("operationId") for item in result["paths"].values() for op in item.values()]
            self.assertEqual(len(ids), len(set(ids)))
            self.assertTrue(all(ids))
            self.assertIn("get", result["paths"]["/upload"])
            self.assertNotIn("post", result["paths"]["/upload"])
            self.assertEqual(result["paths"]["/items"]["post"]["operationId"], "createItem")
            content = result["paths"]["/items"]["post"]["requestBody"]["content"]["application/json"]
            self.assertEqual(content["schema"], {})
            self.assertEqual(content["example"], {"name": "item"})
            response = result["paths"]["/items"]["post"]["responses"]["200"]["content"]["application/json"]
            self.assertEqual(response["schema"], {})
            mixed = result["paths"]["/mixed"]["post"]["responses"]
            for status in ("200", "204"):
                self.assertIn("application/octet-stream", mixed[status]["content"])
            subprocess.run([ROOT / "scripts/preprocess-rs.sh", source, output], check=True)
            self.assertEqual(result, json.loads(output.read_text()))


if __name__ == "__main__":
    unittest.main()
