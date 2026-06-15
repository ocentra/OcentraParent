import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const wrapper = 'scripts/dev/ocentra-ledger-compat.mjs';

test('ledger hook records a thread without turning duplicate chats read-only', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-session-test-'));
  const fakeLedger = writeFakeLedger(root);
  const first = runHook(root, fakeLedger, 'session-one', 'SessionStart');
  const second = runHook(root, fakeLedger, 'session-two', 'UserPromptSubmit');

  assert.equal(first.status, 0);
  assert.equal(second.status, 0);
  assert.match(first.context, /Active Codex session lease is recorded for this thread/u);
  assert.match(
    second.context,
    /Active Codex session lease could not be refreshed, but this thread may still answer questions and inspect status/u
  );
});

test('worker PR_READY reports also notify the primary inbox', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-report-notify-test-'));
  const fakeLedger = writeReportFakeLedger(root);
  const result = spawnSync(
    process.execPath,
    [wrapper, 'hub:report', '--summary', 'PR_READY app-game branch', '--details', prReadyDetails()],
    {
      cwd: process.cwd(),
      encoding: 'utf8',
      env: {
        ...process.env,
        LEDGER_ROOT: root,
        LEDGER_LANE: 'codex-c',
        OCENTRA_LEDGER_WRAPPER: fakeLedger,
      },
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    }
  );

  assert.equal(result.status, 0);
  const calls = JSON.parse(readFileSync(join(root, 'calls.json'), 'utf8'));
  assert.deepEqual(
    calls.map((call) => call.command),
    ['report', 'msg']
  );
  assert.deepEqual(calls[0].args.slice(0, 3), ['report', '--lane', 'codex-c']);
  assert.equal(calls[1].args[0], 'msg');
  assert.equal(calls[1].args[1], 'primary');
  assert.match(calls[1].args[2], /Worker report from codex-c: PR_READY app-game branch/u);
});

test('ordinary worker reports do not notify the primary inbox', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-report-quiet-test-'));
  const fakeLedger = writeReportFakeLedger(root);
  const result = spawnSync(
    process.execPath,
    [wrapper, 'hub:report', '--summary', 'STARTED app-game branch', '--details', startedDetails()],
    {
      cwd: process.cwd(),
      encoding: 'utf8',
      env: {
        ...process.env,
        LEDGER_ROOT: root,
        LEDGER_LANE: 'codex-c',
        OCENTRA_LEDGER_WRAPPER: fakeLedger,
      },
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    }
  );

  assert.equal(result.status, 0);
  const calls = JSON.parse(readFileSync(join(root, 'calls.json'), 'utf8'));
  assert.deepEqual(
    calls.map((call) => call.command),
    ['report']
  );
});

test('lifecycle worker reports require structured metadata', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-report-schema-test-'));
  const fakeLedger = writeReportFakeLedger(root);
  const result = spawnSync(
    process.execPath,
    [wrapper, 'hub:report', '--summary', 'BLOCKED app-game branch', '--details', 'lane: codex-c'],
    {
      cwd: process.cwd(),
      encoding: 'utf8',
      env: {
        ...process.env,
        LEDGER_ROOT: root,
        LEDGER_LANE: 'codex-c',
        OCENTRA_LEDGER_WRAPPER: fakeLedger,
      },
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    }
  );

  assert.equal(result.status, 1);
  assert.match(result.stderr, /BLOCKED reports require structured fields/u);
  const callsPath = join(root, 'calls.json');
  assert.equal(existsSync(callsPath), false);
});

function startedDetails() {
  return [
    'lane: codex-c',
    'threadId: thread-123',
    'assignedBy: primary',
    'plan: sample-plan',
    'workpack: WP01',
    'worktree: E:\\OcentraParent',
    'branch: codex/sample-branch',
    'scope: docs/agent/HUB_LEDGER_MESSAGING.md',
    'startedAt: 2026-06-15T15:20:00Z',
    'nextAction: verify contract',
  ].join('\n');
}

function prReadyDetails() {
  return [
    'lane: codex-c',
    'threadId: thread-123',
    'assignedBy: primary',
    'plan: sample-plan',
    'workpack: WP01',
    'worktree: E:\\OcentraParent',
    'branch: codex/sample-branch',
    'scope: docs/agent/HUB_LEDGER_MESSAGING.md',
    'validation: node --test scripts/test/ocentra-ledger-compat.test.mjs',
    'commit: 61bd396',
    'proof: wrapper validation and inbox notification',
  ].join('\n');
}

function writeFakeLedger(root) {
  const fakeLedger = join(root, 'fake-ledger.mjs');
  writeFileSync(
    fakeLedger,
    `
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const [, , command, action, lane, sessionId] = process.argv;
const store = join(process.env.LEDGER_ROOT, 'active-session.json');

if (command !== 'session' || action !== 'claim' || lane !== 'codex-d') {
  console.error('unexpected fake ledger command');
  process.exit(2);
}

const active = existsSync(store) ? JSON.parse(readFileSync(store, 'utf8')) : undefined;
if (active === undefined || active.sessionId === sessionId) {
  writeFileSync(store, JSON.stringify({ sessionId }));
  console.log(JSON.stringify({ sessionId }));
  process.exit(0);
}

console.log(JSON.stringify({ activeSession: active }));
process.exit(1);
`.trimStart()
  );
  return fakeLedger;
}

function writeReportFakeLedger(root) {
  const fakeLedger = join(root, 'fake-report-ledger.mjs');
  writeFileSync(
    fakeLedger,
    `
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const args = process.argv.slice(2);
const store = join(process.env.LEDGER_ROOT, 'calls.json');
const calls = existsSync(store) ? JSON.parse(readFileSync(store, 'utf8')) : [];
calls.push({ command: args[0], args });
writeFileSync(store, JSON.stringify(calls));
console.log(JSON.stringify({ ok: true }));
`.trimStart()
  );
  return fakeLedger;
}

function runHook(root, fakeLedger, sessionId, hookEventName) {
  const result = spawnSync(process.execPath, [wrapper, 'hub:hook'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    input: JSON.stringify({
      hook_event_name: hookEventName,
      session_id: sessionId,
    }),
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['pipe', 'pipe', 'pipe'],
    windowsHide: true,
  });
  const parsed = JSON.parse(result.stdout);
  return {
    status: result.status,
    stderr: result.stderr,
    context: parsed.hookSpecificOutput.additionalContext,
  };
}
