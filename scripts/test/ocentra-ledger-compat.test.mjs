import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const wrapper = 'scripts/dev/ocentra-ledger-compat.mjs';

test('ledger hook claims one active Codex session per lane', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-session-test-'));
  const fakeLedger = writeFakeLedger(root);
  const first = runHook(root, fakeLedger, 'session-one', 'SessionStart');
  const second = runHook(root, fakeLedger, 'session-two', 'UserPromptSubmit');

  assert.equal(first.status, 0);
  assert.equal(second.status, 0);
  assert.match(first.context, /Active Codex session lease is held by this thread/u);
  assert.match(second.context, /READ-ONLY: this lane is already owned by another active Codex session/u);
});

test('worker PR_READY reports also notify the primary inbox', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-report-notify-test-'));
  const fakeLedger = writeReportFakeLedger(root);
  const result = spawnSync(process.execPath, [wrapper, 'hub:report', '--summary', 'PR_READY app-game branch'], {
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
  });

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
  const result = spawnSync(process.execPath, [wrapper, 'hub:report', '--summary', 'STARTED app-game branch'], {
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
  });

  assert.equal(result.status, 0);
  const calls = JSON.parse(readFileSync(join(root, 'calls.json'), 'utf8'));
  assert.deepEqual(
    calls.map((call) => call.command),
    ['report']
  );
});

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
