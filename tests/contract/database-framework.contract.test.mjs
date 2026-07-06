import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

test("database manifest follows sdkwork-database module shape", () => {
  const manifest = JSON.parse(
    readFileSync(path.join(root, "database/database.manifest.json"), "utf8"),
  );
  assert.equal(manifest.kind, "sdkwork.database.module");
  assert.equal(manifest.moduleId, "settings");
  assert.equal(manifest.tablePrefix, "stg_");
});
