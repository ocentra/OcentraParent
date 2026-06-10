import assert from 'node:assert/strict';
import { mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const wrapper = 'scripts/dev/ocentra-ledger-compat.mjs';

test('ledger hook claims one active Codex session per lane', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-session-test-'));
  const first = runHook(root, 'session-one', 'SessionStart');
  const second = runHook(root, 'session-two', 'UserPromptSubmit');

  assert.equal(first.status, 0);
  assert.equal(second.status, 0);
  assert.match(first.context, /Active Codex session lease: session-one/u);
  assert.match(second.context, /READ-ONLY: codex-d is already owned by active Codex session session-one/u);
});

function runHook(root, sessionId, hookEventName) {
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
