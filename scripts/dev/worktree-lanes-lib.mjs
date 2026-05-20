import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join } from 'node:path';

export const LaneStatus = Object.freeze({
  Blocked: 'blocked',
  FreeDirtyParkBeforeUse: 'free-dirty-park-before-use',
  FreeWarm: 'free-warm',
  Occupied: 'occupied',
  ReadyForReview: 'ready-for-review',
});

export const LaneRole = Object.freeze({
  Codex: 'codex',
  Primary: 'primary',
});

export const LaneCommand = Object.freeze({
  Claim: 'claim',
  Free: 'free',
  Init: 'init',
  Status: 'status',
});

const ledgerFileName = 'ocentra-parent-worktrees.json';

export function defaultLedgerPath(env = process.env) {
  return env.OCENTRA_PARENT_LANE_LEDGER ?? join(homedir(), '.codex', ledgerFileName);
}

export function createDefaultLedger({ repoRoot, repoBranch = 'main', now = new Date() }) {
  const laneRoot = join(homedir(), '.codex', 'worktrees');
  return {
    schema: 'https://ocentra.ca/schemas/ocentra-parent-worktree-lanes.v1.json',
    version: 1,
    repo: 'ocentra/OcentraParent',
    updatedAt: now.toISOString(),
    lanes: [
      {
        id: 'primary',
        role: LaneRole.Primary,
        path: repoRoot,
        status: LaneStatus.Occupied,
        branch: repoBranch,
        task: 'user primary checkout',
        nextAction: 'Do not repurpose without explicit user direction.',
      },
      createReusableLane('codex-a', join(laneRoot, 'ocentra-parent-codex-a', 'OcentraParent')),
      createReusableLane('codex-b', join(laneRoot, 'ocentra-parent-codex-b', 'OcentraParent')),
      createReusableLane('codex-c', join(laneRoot, 'ocentra-parent-codex-c', 'OcentraParent')),
    ],
  };
}

export function createReusableLane(id, path) {
  return {
    id,
    role: LaneRole.Codex,
    path,
    status: LaneStatus.FreeWarm,
    branch: '',
    task: '',
    nextAction: 'Claim with a clean milestone branch before editing.',
  };
}

export function ensureLedger({ ledgerPath, repoRoot, repoBranch = 'main', now = new Date() }) {
  if (existsSync(ledgerPath)) {
    return readLedger(ledgerPath);
  }

  const ledger = createDefaultLedger({ repoRoot, repoBranch, now });
  writeLedger(ledgerPath, ledger);
  return ledger;
}

export function readLedger(ledgerPath) {
  return JSON.parse(readFileSync(ledgerPath, 'utf8'));
}

export function writeLedger(ledgerPath, ledger) {
  mkdirSync(dirname(ledgerPath), { recursive: true });
  writeFileSync(ledgerPath, `${JSON.stringify(ledger, null, 2)}\n`, 'utf8');
}

export function normalizeBranchName(input) {
  const trimmed = input.trim();
  const withoutPrefix = trimmed.replace(/^codex[\\/\s:_-]+/iu, '');
  const slug = withoutPrefix
    .toLowerCase()
    .replace(/[^a-z0-9.]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .replace(/-{2,}/gu, '-');

  if (slug.length === 0) {
    throw new Error('Branch name cannot be empty after normalization.');
  }

  return `codex/${slug}`;
}

export function parseLaneArgs(argv) {
  const [command = LaneCommand.Status, ...tokens] = argv;
  const options = { command };

  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (!token.startsWith('--')) {
      throw new Error(`Unexpected argument: ${token}`);
    }

    const key = token.slice(2);
    const next = tokens[index + 1];
    if (next === undefined || next.startsWith('--')) {
      options[key] = true;
      continue;
    }

    options[key] = next;
    index += 1;
  }

  return options;
}

export function claimLane(
  ledger,
  { laneId, branchInput, task, base = 'origin/main', now = new Date(), force = false }
) {
  const lane = findLane(ledger, laneId);
  if (lane.role === LaneRole.Primary && force !== true) {
    throw new Error('Primary lane cannot be claimed without --force.');
  }
  if (lane.status === LaneStatus.Occupied && force !== true) {
    throw new Error(`Lane is already occupied: ${laneId}`);
  }

  lane.status = LaneStatus.Occupied;
  lane.branch = normalizeBranchName(branchInput);
  lane.base = base;
  lane.task = task;
  lane.claimedAt = now.toISOString();
  lane.nextAction = 'Work in this lane until the branch is merged, parked, or explicitly freed.';
  ledger.updatedAt = now.toISOString();
  return { ledger, lane };
}

export function freeLane(ledger, { laneId, nextAction = 'Reusable after fresh status check.', now = new Date() }) {
  const lane = findLane(ledger, laneId);
  lane.status = LaneStatus.FreeWarm;
  lane.previousBranch = lane.branch;
  lane.branch = '';
  lane.task = '';
  lane.base = '';
  lane.claimedAt = '';
  lane.freedAt = now.toISOString();
  lane.nextAction = nextAction;
  ledger.updatedAt = now.toISOString();
  return { ledger, lane };
}

export function findLane(ledger, laneId) {
  const lane = ledger.lanes.find((candidate) => candidate.id === laneId);
  if (lane === undefined) {
    throw new Error(`Unknown lane: ${laneId}`);
  }
  return lane;
}

export function formatLedgerSummary(ledger, liveStates = []) {
  const liveByLane = new Map(liveStates.map((state) => [state.id, state]));
  return ledger.lanes
    .map((lane) => {
      const live = liveByLane.get(lane.id);
      const liveText = live === undefined ? 'live=not-checked' : `live=${live.summary}`;
      return `${lane.id} | ${lane.status} | ${lane.branch || '-'} | ${lane.task || '-'} | ${liveText}`;
    })
    .join('\n');
}
