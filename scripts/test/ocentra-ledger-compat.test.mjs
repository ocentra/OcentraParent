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
  const second = runHook(root, fakeLedger, 'session-two', 'SessionStart');

  assert.equal(first.status, 0);
  assert.equal(second.status, 0);
  assert.match(first.context, /Active Codex session lease is recorded for this thread/u);
  assert.match(
    second.context,
    /Active Codex session lease could not be refreshed, but this thread may still answer questions and inspect status/u
  );
});

test('explicit user prompts grant writable access without taking lane ownership', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-user-grant-test-'));
  const fakeLedger = writeFakeLedger(root);
  const first = runHook(root, fakeLedger, 'session-one', 'SessionStart');
  const second = runHook(root, fakeLedger, 'session-two', 'UserPromptSubmit');
  const followUp = runHook(root, fakeLedger, 'session-two', 'PostToolUse');

  assert.equal(first.status, 0);
  assert.equal(second.status, 0);
  assert.equal(followUp.status, 0);
  assert.match(second.context, /USER-OVERRIDE:/u);
  assert.match(second.context, /without taking the lane lease/u);
  assert.match(second.context, /session-one/u);
  assert.doesNotMatch(second.context, /Active Codex session lease is held by this thread/u);
  assert.match(followUp.context, /USER-OVERRIDE:/u);
  assert.doesNotMatch(followUp.context, /READ-ONLY:/u);

  const activeSession = JSON.parse(readFileSync(join(root, 'active-session.json'), 'utf8'));
  assert.equal(activeSession.sessionId, 'session-one');
});

test('prompted coordinator threads can delegate writable access to subagent sessions', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-delegate-grant-test-'));
  const fakeLedger = writeFakeLedger(root);
  const ownerHook = runHook(root, fakeLedger, 'session-one', 'SessionStart');
  const promptedCoordinatorHook = runHook(root, fakeLedger, 'session-two', 'UserPromptSubmit');

  assert.equal(ownerHook.status, 0);
  assert.equal(promptedCoordinatorHook.status, 0);

  const grant = spawnSync(process.execPath, [wrapper, 'hub:delegate-grant', '--session-id', 'session-three'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(grant.status, 0);
  assert.match(grant.stdout, /delegate-grant-set: lane=codex-d session=session-three delegated-by=session-two/u);

  const delegatedHook = runHook(root, fakeLedger, 'session-three', 'PostToolUse');
  assert.equal(delegatedHook.status, 0);
  assert.match(delegatedHook.context, /COORDINATED-DELEGATE-GRANT:/u);
  assert.match(delegatedHook.context, /session-two/u);
  assert.doesNotMatch(delegatedHook.context, /READ-ONLY:/u);

  const activeSession = JSON.parse(readFileSync(join(root, 'active-session.json'), 'utf8'));
  assert.equal(activeSession.sessionId, 'session-one');
});

test('delegate revoke returns delegated sessions to read-only mode', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-delegate-revoke-test-'));
  const fakeLedger = writeFakeLedger(root);
  runHook(root, fakeLedger, 'session-one', 'SessionStart');
  runHook(root, fakeLedger, 'session-two', 'UserPromptSubmit');

  const grant = spawnSync(process.execPath, [wrapper, 'hub:delegate-grant', '--session-id', 'session-three'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
  assert.equal(grant.status, 0);

  const revoke = spawnSync(process.execPath, [wrapper, 'hub:delegate-revoke', '--session-id', 'session-three'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(revoke.status, 0);
  assert.match(revoke.stdout, /delegate-grant-cleared: lane=codex-d session=session-three/u);

  const delegatedHook = runHook(root, fakeLedger, 'session-three', 'PostToolUse');
  assert.equal(delegatedHook.status, 0);
  assert.match(delegatedHook.context, /READ-ONLY:/u);
});

test('manual-only thread mode limits automatic lane refresh to explicit user prompts', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-manual-only-test-'));
  const fakeLedger = writeFakeLedger(root);
  const initialPrompt = runHook(root, fakeLedger, 'session-one', 'UserPromptSubmit');

  assert.equal(initialPrompt.status, 0);
  assert.match(initialPrompt.context, /Active Codex session lease is held by this thread/u);

  const setMode = spawnSync(process.execPath, [wrapper, 'hub:thread-upgrade'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(setMode.status, 0);
  assert.match(setMode.stdout, /thread-mode-set: lane=codex-d session=session-one mode=manual-only/u);

  writeFileSync(join(root, 'active-session.json'), JSON.stringify({ sessionId: 'session-two' }));

  const autoHook = runHook(root, fakeLedger, 'session-one', 'PostToolUse');
  assert.equal(autoHook.status, 0);
  assert.match(autoHook.context, /MANUAL-ONLY:/u);
  assert.match(autoHook.context, /PostToolUse/u);
  assert.doesNotMatch(autoHook.context, /Active Codex session lease is held by this thread/u);
  assert.doesNotMatch(autoHook.context, /READ-ONLY: this lane is already owned by another active Codex session/u);

  writeFileSync(join(root, 'active-session.json'), JSON.stringify({ sessionId: 'session-one' }));

  const promptedHook = runHook(root, fakeLedger, 'session-one', 'UserPromptSubmit');
  assert.equal(promptedHook.status, 0);
  assert.match(promptedHook.context, /MANUAL-ONLY:/u);
  assert.match(promptedHook.context, /explicit user prompts/u);
  assert.match(promptedHook.context, /Active Codex session lease is held by this thread/u);

  const resetMode = spawnSync(process.execPath, [wrapper, 'hub:thread-default'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(resetMode.status, 0);
  assert.match(resetMode.stdout, /thread-mode-set: lane=codex-d session=session-one mode=default/u);

  const postResetHook = runHook(root, fakeLedger, 'session-one', 'PostToolUse');
  assert.equal(postResetHook.status, 0);
  assert.doesNotMatch(postResetHook.context, /MANUAL-ONLY:/u);
  assert.match(postResetHook.context, /Active Codex session lease is held by this thread/u);
});

test('thread upgrade refuses explicit session targeting and other-thread retargets', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-thread-upgrade-guard-test-'));
  const fakeLedger = writeFakeLedger(root);

  const initialPrompt = runHook(root, fakeLedger, 'session-one', 'UserPromptSubmit');
  assert.equal(initialPrompt.status, 0);

  const explicitTarget = spawnSync(process.execPath, [wrapper, 'hub:thread-upgrade', '--session-id', 'session-two'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(explicitTarget.status, 1);
  assert.match(explicitTarget.stderr, /current thread/u);

  const duplicateSession = runHook(root, fakeLedger, 'session-two', 'SessionStart');
  assert.equal(duplicateSession.status, 0);
  assert.match(duplicateSession.context, /READ-ONLY:/u);

  const wrongThread = spawnSync(process.execPath, [wrapper, 'hub:thread-upgrade'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(wrongThread.status, 1);
  assert.match(wrongThread.stderr, /current thread after a real user prompt/u);
  assert.match(wrongThread.stderr, /session-two/u);
  assert.match(wrongThread.stderr, /session-one/u);
});

test('thread mode status inspects lane state without requiring a fresh prompt', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-ledger-thread-mode-status-test-'));
  const fakeLedger = writeFakeLedger(root);

  const initialPrompt = runHook(root, fakeLedger, 'session-one', 'UserPromptSubmit');
  assert.equal(initialPrompt.status, 0);

  const setMode = spawnSync(process.execPath, [wrapper, 'hub:thread-upgrade'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(setMode.status, 0);

  writeFileSync(join(root, 'active-session.json'), JSON.stringify({ sessionId: 'session-two' }));
  const duplicateHook = runHook(root, fakeLedger, 'session-two', 'PostToolUse');
  assert.equal(duplicateHook.status, 0);

  const status = spawnSync(process.execPath, [wrapper, 'hub:thread-mode'], {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      LEDGER_ROOT: root,
      LEDGER_LANE: 'codex-d',
      OCENTRA_LEDGER_WRAPPER: fakeLedger,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });

  assert.equal(status.status, 0);
  assert.match(status.stdout, /thread-mode: lane=codex-d/u);
  assert.match(status.stdout, /active-session=session-two/u);
  assert.match(status.stdout, /active-mode=default/u);
  assert.match(status.stdout, /latest-user-prompt-session=session-one/u);
  assert.match(status.stdout, /latest-user-prompt-mode=manual-only/u);
  assert.match(status.stdout, /write-grants=none/u);
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

if (command !== 'session' || lane !== 'codex-d') {
  console.error('unexpected fake ledger command');
  process.exit(2);
}

const active = existsSync(store) ? JSON.parse(readFileSync(store, 'utf8')) : undefined;

if (action === 'release') {
  if (active?.sessionId === sessionId) {
    writeFileSync(store, JSON.stringify({}));
  }
  console.log(JSON.stringify({ ok: true, releasedSessionId: sessionId }));
  process.exit(0);
}

if (action === 'claim') {
  if (active === undefined || typeof active.sessionId !== 'string' || active.sessionId === sessionId) {
    writeFileSync(store, JSON.stringify({ sessionId }));
    console.log(JSON.stringify({ sessionId }));
    process.exit(0);
  }

  console.log(JSON.stringify({ activeSession: active }));
  process.exit(1);
}

console.error('unexpected fake ledger action');
process.exit(2);
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
