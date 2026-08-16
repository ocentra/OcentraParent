import assert from 'node:assert/strict';
import { existsSync, renameSync, rmSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { describe, it } from 'node:test';

const workspaceRoot = process.cwd();
const generatedLoggingContractsArtifact = path.join(
  workspaceRoot,
  'packages/schema-domain/dist/generated-logging-contracts.js'
);
const generatedLoggingContractsBackup = `${generatedLoggingContractsArtifact}.proof-trace-test-backup-${process.pid}`;
const smokeScript = path.join(workspaceRoot, 'scripts/dev/logging-proof-trace-smoke.mjs');

describe('logging proof-trace smoke', () => {
  it('builds the missing schema-domain artifact before loading proof-trace dependencies', () => {
    const artifactExistedBeforeTest = existsSync(generatedLoggingContractsArtifact);
    if (artifactExistedBeforeTest) {
      renameSync(generatedLoggingContractsArtifact, generatedLoggingContractsBackup);
    }

    try {
      const result = spawnSync(process.execPath, [smokeScript, '--verify-schema-prerequisite'], {
        cwd: workspaceRoot,
        encoding: 'utf8',
        windowsHide: true,
      });

      assert.equal(result.status, 0, result.stderr);
      assert.equal(existsSync(generatedLoggingContractsArtifact), true);
      assert.deepEqual(JSON.parse(result.stdout), { generatedLoggingContractsArtifact });
    } finally {
      rmSync(generatedLoggingContractsArtifact, { force: true });
      if (artifactExistedBeforeTest) {
        renameSync(generatedLoggingContractsBackup, generatedLoggingContractsArtifact);
      }
    }
  });
});
