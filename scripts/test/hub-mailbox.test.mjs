import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import {
  acknowledgeLane,
  ensureHub,
  formatHubSummary,
  lockLanePaths,
  messageLane,
  parseHubArgs,
  readOrCreateMailbox,
  reportLane,
  splitPathList,
  validateHubContext,
} from '../dev/hub-mailbox-lib.mjs';
import { claimLane, createDefaultLedger } from '../dev/worktree-lanes-lib.mjs';

const fixedDate = new Date('2026-05-20T16:00:00.000Z');

function tempHubRoot() {
  return mkdtempSync(join(tmpdir(), 'ocentra-parent-hub-test-'));
}

function claimedLedger() {
  const ledger = createDefaultLedger({ repoRoot: 'E:\\OcentraParent', repoBranch: 'main', now: fixedDate });
  claimLane(ledger, {
    branchInput: 'V0.3 Windows Process And Window Activity Capture',
    laneId: 'codex-a',
    now: fixedDate,
    owner: 'codex',
    task: 'capture',
    thread: 'v0.3-capture',
  });
  return ledger;
}

test('hub mailbox creates readable lane files', () => {
  const hubRoot = tempHubRoot();
  const ledger = claimedLedger();

  ensureHub({ hubRoot, ledger, now: fixedDate });

  assert.equal(existsSync(join(hubRoot, 'lanes', 'codex-a', 'inbox.md')), true);
  assert.equal(existsSync(join(hubRoot, 'lanes', 'codex-a', 'status.md')), true);
  assert.equal(existsSync(join(hubRoot, 'lanes', 'codex-a', 'ownership.json')), true);
});

test('hub messages require lane acknowledgement', () => {
  const hubRoot = tempHubRoot();
  const ledger = claimedLedger();
  const lane = ledger.lanes.find((candidate) => candidate.id === 'codex-a');

  const { message } = messageLane({
    body: 'Stay inside V0.3 capture files.',
    hubRoot,
    lane,
    now: fixedDate,
    subject: 'V0.3 scope',
  });
  const failed = validateHubContext({
    changedPaths: [],
    hubRoot,
    ledger,
    repoRoot: lane.path,
  });
  assert.equal(failed.ok, false);
  assert.match(failed.findings.join('\n'), /unread hub message/u);

  acknowledgeLane({ hubRoot, lane, messageId: message.id, now: fixedDate });
  const passed = validateHubContext({
    changedPaths: [],
    hubRoot,
    ledger,
    repoRoot: lane.path,
  });
  assert.equal(passed.ok, true);
});

test('hub mailbox syncs lane owner thread and branch changes', () => {
  const hubRoot = tempHubRoot();
  const ledger = claimedLedger();
  const lane = ledger.lanes.find((candidate) => candidate.id === 'codex-a');

  ensureHub({ hubRoot, ledger, now: fixedDate });
  lane.owner = 'new-owner';
  lane.thread = 'new-thread';
  lane.branch = 'codex/new-branch';

  const mailbox = readOrCreateMailbox(hubRoot, lane, fixedDate);
  assert.equal(mailbox.owner, 'new-owner');
  assert.equal(mailbox.thread, 'new-thread');
  assert.equal(mailbox.branch, 'codex/new-branch');
});

test('hub guard enforces changed files against lane locks', () => {
  const hubRoot = tempHubRoot();
  const ledger = claimedLedger();
  const lane = ledger.lanes.find((candidate) => candidate.id === 'codex-a');

  lockLanePaths({
    hubRoot,
    lane,
    ledger,
    now: fixedDate,
    paths: ['crates/agent-service', 'packages/activity-domain'],
    reason: 'capture lane',
  });

  const passed = validateHubContext({
    changedPaths: ['crates/agent-service/src/capture.rs'],
    hubRoot,
    ledger,
    repoRoot: lane.path,
  });
  assert.equal(passed.ok, true);

  const failed = validateHubContext({
    changedPaths: ['docs/product-roadmap.md'],
    hubRoot,
    ledger,
    repoRoot: lane.path,
  });
  assert.equal(failed.ok, false);
  assert.match(failed.findings.join('\n'), /outside hub locks/u);
});

test('hub lock rejects overlapping ownership across lanes', () => {
  const hubRoot = tempHubRoot();
  const ledger = claimedLedger();
  const laneA = ledger.lanes.find((candidate) => candidate.id === 'codex-a');
  const laneB = ledger.lanes.find((candidate) => candidate.id === 'codex-b');

  lockLanePaths({
    hubRoot,
    lane: laneA,
    ledger,
    now: fixedDate,
    paths: ['docs/architecture'],
    reason: 'first lane',
  });

  assert.throws(
    () =>
      lockLanePaths({
        hubRoot,
        lane: laneB,
        ledger,
        now: fixedDate,
        paths: ['docs/architecture/worktree-lanes.md'],
        reason: 'second lane',
      }),
    /Hub lock conflict/u
  );
});

test('hub report and summary expose latest lane state', () => {
  const hubRoot = tempHubRoot();
  const ledger = claimedLedger();
  const lane = ledger.lanes.find((candidate) => candidate.id === 'codex-a');

  reportLane({
    details: 'Mapped adapter boundary.',
    hubRoot,
    lane,
    now: fixedDate,
    summary: 'Capture design ready',
  });

  const summary = formatHubSummary({ hubRoot, ledger, now: fixedDate });
  const status = readFileSync(join(hubRoot, 'lanes', 'codex-a', 'status.md'), 'utf8');
  assert.match(summary, /Capture design ready/u);
  assert.match(status, /Mapped adapter boundary/u);
});

test('hub args and path list parsing support command scripts', () => {
  assert.deepEqual(parseHubArgs(['message', '--lane', 'codex-a', '--subject', 'Scope', '--body', 'Read docs.']), {
    body: 'Read docs.',
    command: 'message',
    lane: 'codex-a',
    subject: 'Scope',
  });
  assert.deepEqual(splitPathList('crates/agent-service, packages/activity-domain'), [
    'crates/agent-service',
    'packages/activity-domain',
  ]);
});

test('hub inbox renders latest message text', () => {
  const hubRoot = tempHubRoot();
  const ledger = claimedLedger();
  const lane = ledger.lanes.find((candidate) => candidate.id === 'codex-a');

  messageLane({
    body: 'Check journal write path.',
    hubRoot,
    lane,
    now: fixedDate,
    subject: 'Capture handoff',
  });

  const mailbox = readOrCreateMailbox(hubRoot, lane, fixedDate);
  assert.match(readFileSync(join(hubRoot, 'lanes', 'codex-a', 'inbox.md'), 'utf8'), /Capture handoff/u);
  assert.equal(mailbox.messages.length, 1);
});
