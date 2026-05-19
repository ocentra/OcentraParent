import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';

const scannerPath = fileURLToPath(new URL('../security/scan-staged-secrets.mjs', import.meta.url));

function withTempRepo(testBody) {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-parent-secret-scan-'));
  try {
    return testBody(root);
  } finally {
    rmSync(root, { force: true, recursive: true });
  }
}

function runScanner(cwd, args = []) {
  return spawnSync(process.execPath, [scannerPath, ...args], {
    cwd,
    encoding: 'utf8',
  });
}

test('repository secret scan rejects forbidden sensitive filenames', () => {
  withTempRepo((root) => {
    writeFileSync(join(root, '.env'), 'OCENTRA_PARENT_TOKEN=local-only\n', 'utf8');

    const result = runScanner(root, ['--repo']);

    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /\.env: forbidden sensitive file path/u);
  });
});

test('staged secret scan rejects forbidden sensitive filenames', () => {
  withTempRepo((root) => {
    assert.equal(spawnSync('git', ['init'], { cwd: root }).status, 0);
    writeFileSync(join(root, '.env'), 'OCENTRA_PARENT_TOKEN=local-only\n', 'utf8');
    assert.equal(spawnSync('git', ['add', '.env'], { cwd: root }).status, 0);

    const result = runScanner(root);

    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /\.env: forbidden sensitive file path/u);
  });
});

test('repository secret scan allows environment templates while still scanning their contents', () => {
  withTempRepo((root) => {
    writeFileSync(join(root, '.env.example'), 'OCENTRA_PARENT_AGENT_PORT=4477\n', 'utf8');

    const result = runScanner(root, ['--repo']);

    assert.equal(result.status, 0);
    assert.match(result.stdout, /Secret scan passed for 1 file\(s\)\./u);
  });
});
