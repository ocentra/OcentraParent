import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const ledger = 'scripts/dev/ocentra-ledger.mjs';
const notifier = 'scripts/dev/ocentra-ledger-notifier.mjs';

test('primary notifier wakes for real inbox and worker handoff ledger events', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-notifier-primary-'));
  runLedger(root, ['init', 'ocentra-parent', '--lane', 'primary']);
  runLedger(root, ['msg', 'primary', 'Please review C branch\nbody details']);
  runLedger(root, ['worker', 'codex-c', 'working', 'PR_READY app-game control branch']);
  runLedger(root, ['worker', 'codex-d', 'working', 'BLOCKED browser proof needs review']);
  runLedger(root, ['worker', 'codex-b', 'working', 'STARTED continuation work']);

  const result = runNotifier(root, ['--lane', 'primary', '--json', '--peek']);

  assert.equal(result.status, 0);
  const parsed = JSON.parse(result.stdout);
  assert.equal(parsed.targetLane, 'primary');
  assert.deepEqual(parsed.wakeRequests.map((request) => request.reason).sort(), ['blocked', 'inbox', 'pr-ready']);
  assert.equal(parsed.wakeRequests.find((request) => request.reason === 'blocked').severity, 'high');
  assert.equal(
    parsed.wakeRequests.some((request) => request.summary === 'STARTED continuation work'),
    false
  );
});

test('worker notifier only wakes for that worker inbox', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-notifier-worker-'));
  runLedger(root, ['init', 'ocentra-parent', '--lane', 'primary']);
  runLedger(root, ['msg', 'codex-b', 'Continue screen AI branch']);
  runLedger(root, ['msg', 'codex-c', 'Different lane message']);
  runLedger(root, ['worker', 'codex-c', 'working', 'DONE unrelated worker handoff']);

  const result = runNotifier(root, ['--lane', 'codex-b', '--json', '--peek']);

  assert.equal(result.status, 0);
  const parsed = JSON.parse(result.stdout);
  assert.deepEqual(
    parsed.wakeRequests.map((request) => request.reason),
    ['inbox']
  );
  assert.deepEqual(
    parsed.wakeRequests.map((request) => request.summary),
    ['Continue screen AI branch']
  );
});

test('notifier dedupes previously seen wake requests', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-notifier-dedupe-'));
  const stateFile = join(root, 'state.json');
  runLedger(root, ['init', 'ocentra-parent', '--lane', 'primary']);
  runLedger(root, ['msg', 'primary', 'Review this once']);

  const first = runNotifier(root, ['--lane', 'primary', '--json', '--state-file', stateFile]);
  const second = runNotifier(root, ['--lane', 'primary', '--json', '--state-file', stateFile]);

  assert.equal(first.status, 0);
  assert.equal(second.status, 0);
  assert.equal(JSON.parse(first.stdout).wakeRequests.length, 1);
  assert.equal(JSON.parse(second.stdout).wakeRequests.length, 0);
  assert.equal(existsSync(stateFile), true);
  assert.deepEqual(Object.keys(JSON.parse(readFileSync(stateFile, 'utf8')).seen), [
    JSON.parse(first.stdout).wakeRequests[0].key,
  ]);
});

test('notifier exit-code mode exits 2 when wake requests exist', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-notifier-exit-'));
  runLedger(root, ['init', 'ocentra-parent', '--lane', 'primary']);
  runLedger(root, ['msg', 'primary', 'Wake primary']);

  const result = runNotifier(root, ['--lane', 'primary', '--json', '--peek', '--exit-code']);

  assert.equal(result.status, 2);
  assert.equal(JSON.parse(result.stdout).wakeRequests.length, 1);
});

function runLedger(root, args) {
  const result = spawnSync(process.execPath, [ledger, ...args], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
  assert.equal(result.status, 0, result.stderr);
  return result;
}

function runNotifier(root, args) {
  return spawnSync(process.execPath, [notifier, ...args], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
}
