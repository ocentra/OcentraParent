import assert from 'node:assert/strict';
import { join } from 'node:path';
import { test } from 'node:test';

import {
  LaneStatus,
  claimLane,
  createDefaultLedger,
  formatLedgerSummary,
  freeLane,
  normalizeBranchName,
  parseLaneArgs,
  validateLaneContext,
} from '../dev/worktree-lanes-lib.mjs';
import { runLaneCli } from '../dev/worktree-lanes.mjs';

const fixedDate = new Date('2026-05-20T14:00:00.000Z');

test('worktree lane branch normalization accepts milestone names', () => {
  assert.equal(
    normalizeBranchName('V0.3 Windows Process And Window Activity Capture'),
    'codex/v0.3-windows-process-and-window-activity-capture'
  );
  assert.equal(normalizeBranchName('codex/V0.7 Local AI Policy Evaluator'), 'codex/v0.7-local-ai-policy-evaluator');
  assert.equal(normalizeBranchName('main'), 'main');
  assert.equal(normalizeBranchName('production'), 'production');
});

test('worktree lane defaults keep the primary checkout protected', () => {
  const ledger = createDefaultLedger({
    repoRoot: 'E:\\OcentraParent',
    repoBranch: 'codex/worktree-lane-hub',
    now: fixedDate,
  });

  assert.equal(ledger.repo, 'ocentra/OcentraParent');
  assert.equal(ledger.lanes[0].id, 'primary');
  assert.equal(ledger.lanes[0].status, LaneStatus.Occupied);
  assert.equal(ledger.lanes[0].branch, 'codex/worktree-lane-hub');
  assert.equal(ledger.lanes[0].path, 'E:\\OcentraParent');
  assert.deepEqual(
    ledger.lanes.slice(1).map((lane) => lane.id),
    ['codex-a', 'codex-b', 'codex-c']
  );
  assert.equal(
    ledger.lanes[1].path.endsWith(join('.codex', 'worktrees', 'ocentra-parent-codex-a', 'OcentraParent')),
    true
  );
});

test('worktree lane claim records branch task and base', () => {
  const ledger = createDefaultLedger({ repoRoot: 'E:\\OcentraParent', now: fixedDate });
  const { lane } = claimLane(ledger, {
    laneId: 'codex-a',
    branchInput: 'V0.4 Windows Network And Domain Observation',
    task: 'V0.4 network observation',
    base: 'origin/main',
    now: fixedDate,
    notes: 'adapter lane',
    owner: 'codex-agent',
    thread: 'thread-42',
  });

  assert.equal(lane.status, LaneStatus.Occupied);
  assert.equal(lane.branch, 'codex/v0.4-windows-network-and-domain-observation');
  assert.equal(lane.task, 'V0.4 network observation');
  assert.equal(lane.owner, 'codex-agent');
  assert.equal(lane.thread, 'thread-42');
  assert.equal(lane.notes, 'adapter lane');
  assert.equal(lane.base, 'origin/main');
  assert.equal(lane.claimedAt, fixedDate.toISOString());
});

test('worktree lane claim rejects occupied lanes without force', () => {
  const ledger = createDefaultLedger({ repoRoot: 'E:\\OcentraParent', now: fixedDate });

  assert.throws(
    () =>
      claimLane(ledger, {
        laneId: 'primary',
        branchInput: 'V0.1 Foundation And Evidence Contracts',
        task: 'should not claim primary',
      }),
    /Primary lane cannot be claimed/u
  );
});

test('worktree lane free parks previous branch and clears task', () => {
  const ledger = createDefaultLedger({ repoRoot: 'E:\\OcentraParent', now: fixedDate });
  claimLane(ledger, {
    laneId: 'codex-b',
    branchInput: 'V0.5 Live Activity Portal View',
    task: 'portal view',
    now: fixedDate,
  });

  const { lane } = freeLane(ledger, {
    laneId: 'codex-b',
    nextAction: 'Ready for next milestone.',
    now: fixedDate,
  });

  assert.equal(lane.status, LaneStatus.FreeWarm);
  assert.equal(lane.previousBranch, 'codex/v0.5-live-activity-portal-view');
  assert.equal(lane.branch, '');
  assert.equal(lane.owner, '');
  assert.equal(lane.thread, '');
  assert.equal(lane.task, '');
  assert.equal(lane.nextAction, 'Ready for next milestone.');
});

test('worktree lane args parse boolean and value options', () => {
  assert.deepEqual(
    parseLaneArgs(['claim', '--lane', 'codex-a', '--create-worktree', '--branch', 'V0.3', '--task', 'capture']),
    {
      command: 'claim',
      lane: 'codex-a',
      'create-worktree': true,
      branch: 'V0.3',
      task: 'capture',
    }
  );
});

test('worktree lane summary includes live status when supplied', () => {
  const ledger = createDefaultLedger({ repoRoot: 'E:\\OcentraParent', now: fixedDate });
  const summary = formatLedgerSummary(ledger, [{ id: 'primary', summary: '## main' }]);

  assert.match(summary, /primary \| occupied \| owner=user \| thread=primary \| main/u);
  assert.match(summary, /codex-a \| free-warm \| owner=- \| thread=- \| - \| -/u);
  assert.match(summary, /live=## main/u);
  assert.match(summary, /live=not-checked/u);
});

test('worktree lane CLI rejects unknown commands before git execution', () => {
  assert.throws(() => runLaneCli(['unknown-command']), /Unknown lane command/u);
});

test('worktree lane guard accepts matching occupied checkout', () => {
  const ledger = createDefaultLedger({ repoRoot: 'E:\\OcentraParent', repoBranch: 'main', now: fixedDate });
  const result = validateLaneContext(ledger, {
    branch: 'main',
    owner: 'user',
    repoRoot: 'e:/ocentraparent',
  });

  assert.equal(result.ok, true);
  assert.equal(result.lane.id, 'primary');
  assert.deepEqual(result.findings, []);
});

test('worktree lane guard reports branch and owner drift', () => {
  const ledger = createDefaultLedger({ repoRoot: 'E:\\OcentraParent', repoBranch: 'main', now: fixedDate });
  const result = validateLaneContext(ledger, {
    branch: 'codex/wrong',
    owner: 'other-agent',
    repoRoot: 'E:\\OcentraParent',
  });

  assert.equal(result.ok, false);
  assert.match(result.findings.join('\n'), /expects branch main/u);
  assert.match(result.findings.join('\n'), /owner is user/u);
});
