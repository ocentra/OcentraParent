import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { runCommand } from '../../scripts/git-hooks/run-precommit-validation.mjs';

const realChildTimeoutMs = 30_000;

async function waitForFileContent(filePath, timeoutMs = 2_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      if (readFileSync(filePath).length > 0) return;
    } catch {}
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
  throw new Error(`fixture did not write ${filePath}`);
}

test('pre-commit runner waits for a real child command to complete', async () => {
  const result = await runCommand(process.execPath, ['-e', "process.stdout.write('runner-complete')"], {
    timeoutMs: realChildTimeoutMs,
  });

  assert.equal(result.status, 0);
  assert.equal(result.timedOut, false);
  assert.equal(result.error, undefined);
});

test('pre-commit runner returns the exact nonzero child exit status', async () => {
  const result = await runCommand(process.execPath, ['-e', 'process.exit(23)'], {
    timeoutMs: realChildTimeoutMs,
  });

  assert.equal(result.status, 23);
  assert.equal(result.timedOut, false);
  assert.equal(result.error, undefined);
});

test('pre-commit runner terminates an overdue child command deterministically', async () => {
  const startedAt = Date.now();
  const result = await runCommand(process.execPath, ['-e', 'setInterval(() => {}, 1_000)'], { timeoutMs: 100 });
  const elapsedMs = Date.now() - startedAt;

  assert.equal(result.timedOut, true);
  assert.notEqual(result.status, 0);
  assert.ok(elapsedMs < 5_000, `timed-out command must settle promptly; elapsed=${elapsedMs}ms`);
});

test('pre-commit runner timeout does not orphan a nested child process', async () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-precommit-runner-'));
  const heartbeatPath = join(root, 'heartbeat.log');
  const fixturePath = join(root, 'nested-child.mjs');
  const childSource = `import { appendFileSync } from 'node:fs'; setInterval(() => appendFileSync(${JSON.stringify(heartbeatPath)}, 'x'), 25);`;
  writeFileSync(
    fixturePath,
    `import { spawn } from 'node:child_process'; spawn(process.execPath, ['-e', ${JSON.stringify(childSource)}], { stdio: 'ignore' }); setInterval(() => {}, 1_000);`,
    'utf8'
  );

  try {
    const completion = runCommand(process.execPath, [fixturePath], { timeoutMs: 300 });
    await waitForFileContent(heartbeatPath);
    const result = await completion;
    const sizeAfterTermination = readFileSync(heartbeatPath).length;
    await new Promise((resolve) => setTimeout(resolve, 300));

    assert.equal(result.timedOut, true);
    assert.equal(readFileSync(heartbeatPath).length, sizeAfterTermination);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
