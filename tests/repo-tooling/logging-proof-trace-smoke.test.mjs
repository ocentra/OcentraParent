import assert from 'node:assert/strict';
import { existsSync, rmSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { test } from 'node:test';

const workspaceRoot = process.cwd();
const generatedLoggingContractsArtifact = path.join(
  workspaceRoot,
  'packages/schema-domain/dist/generated-logging-contracts.js'
);
const smokeScript = path.join(workspaceRoot, 'scripts/dev/logging-proof-trace-smoke.mjs');

test('logging proof-trace smoke builds its missing schema-domain artifact before importing it', () => {
  rmSync(path.dirname(generatedLoggingContractsArtifact), { force: true, recursive: true });

  const result = spawnSync(process.execPath, [smokeScript, '--verify-schema-prerequisite'], {
    cwd: workspaceRoot,
    encoding: 'utf8',
    windowsHide: true,
  });

  assert.equal(result.status, 0, result.stderr);
  assert.equal(existsSync(generatedLoggingContractsArtifact), true);
  assert.deepEqual(JSON.parse(result.stdout), { generatedLoggingContractsArtifact });
});
