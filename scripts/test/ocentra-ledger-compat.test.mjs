import assert from 'node:assert/strict';
import { mkdtempSync, writeFileSync } from 'node:fs';
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
  assert.match(first.context, /Active Codex session lease: session-one/u);
  assert.match(second.context, /READ-ONLY: codex-d is already owned by active Codex session session-one/u);
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
