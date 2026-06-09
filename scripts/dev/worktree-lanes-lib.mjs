import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join, normalize, parse } from 'node:path';

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
  Guard: 'guard',
  Init: 'init',
  Status: 'status',
});

const ledgerFileName = 'ocentra-parent-worktrees.json';

export function defaultLedgerPath(env = process.env) {
  return env.OCENTRA_PARENT_LANE_LEDGER ?? join(repoStateRoot(), 'worktree-lanes.json');
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
        owner: 'user',
        thread: 'primary',
        task: 'user primary checkout',
        nextAction: 'Do not repurpose without explicit user direction.',
      },
      createReusableLane('codex-a', join(laneRoot, 'ocentra-parent-codex-a', 'OcentraParent')),
      createReusableLane('codex-b', join(laneRoot, 'ocentra-parent-codex-b', 'OcentraParent')),
      createReusableLane('codex-c', join(laneRoot, 'ocentra-parent-codex-c', 'OcentraParent')),
    ],
  };
}

function repoStateRoot(cwd = process.cwd()) {
  return join(findRepoRoot(cwd), '.hub', 'state');
}

function findRepoRoot(cwd) {
  let current = normalize(cwd);
  const root = parse(current).root;

  while (current.length > 0) {
    if (existsSync(join(current, '.git')) && existsSync(join(current, 'package.json'))) {
      return current;
    }
    if (current === root) {
      break;
    }
    current = dirname(current);
  }

  return cwd;
}

export function createReusableLane(id, path) {
  return {
    id,
    role: LaneRole.Codex,
    path,
    status: LaneStatus.FreeWarm,
    branch: '',
    owner: '',
    thread: '',
    activeSessionId: '',
    previousSessionId: '',
    sessionSource: '',
    sessionUpdatedAt: '',
    task: '',
    notes: '',
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
  if (trimmed === 'main' || trimmed === 'production') {
    return trimmed;
  }

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

export function defaultLaneOwner(env = process.env) {
  return env.OCENTRA_PARENT_LANE_OWNER ?? env.USERNAME ?? env.USER ?? 'codex';
}

export function claimLane(
  ledger,
  {
    laneId,
    branchInput,
    task,
    base = 'origin/main',
    now = new Date(),
    force = false,
    owner = defaultLaneOwner(),
    thread = '',
    notes = '',
  }
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
  lane.owner = owner;
  lane.thread = thread;
  lane.activeSessionId = '';
  lane.previousSessionId = '';
  lane.sessionSource = '';
  lane.sessionUpdatedAt = '';
  lane.task = task;
  lane.notes = notes;
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
  lane.owner = '';
  lane.thread = '';
  lane.activeSessionId = '';
  lane.previousSessionId = '';
  lane.sessionSource = '';
  lane.sessionUpdatedAt = '';
  lane.task = '';
  lane.notes = '';
  lane.base = '';
  lane.claimedAt = '';
  lane.freedAt = now.toISOString();
  lane.nextAction = nextAction;
  ledger.updatedAt = now.toISOString();
  return { ledger, lane };
}

export function recordLaneSession(ledger, { laneId, now = new Date(), sessionId, source = '' }) {
  if (typeof sessionId !== 'string' || sessionId.length === 0) {
    return { changed: false, lane: findLane(ledger, laneId), previousSessionId: '' };
  }

  const lane = findLane(ledger, laneId);
  const previousSessionId = lane.activeSessionId ?? '';
  const sessionChanged = previousSessionId !== sessionId;
  const sourceChanged = (lane.sessionSource ?? '') !== source;

  if (!sessionChanged && !sourceChanged) {
    return { changed: false, lane, previousSessionId };
  }

  if (sessionChanged) {
    lane.previousSessionId = previousSessionId;
  }
  lane.activeSessionId = sessionId;
  lane.sessionSource = source;
  lane.sessionUpdatedAt = now.toISOString();
  ledger.updatedAt = now.toISOString();
  return { changed: true, lane, previousSessionId };
}

export function validateLaneContext(ledger, { repoRoot, branch, laneId, owner }) {
  const lane = laneId === undefined ? findLaneByPath(ledger, repoRoot) : findLane(ledger, laneId);
  const findings = [];

  if (normalizeLanePath(lane.path) !== normalizeLanePath(repoRoot)) {
    findings.push(`lane ${lane.id} points at ${lane.path}, not current checkout ${repoRoot}`);
  }
  if (lane.status !== LaneStatus.Occupied) {
    findings.push(`lane ${lane.id} is ${lane.status}, not ${LaneStatus.Occupied}`);
  }
  if (lane.branch !== branch) {
    findings.push(`lane ${lane.id} expects branch ${lane.branch || '-'}, current branch is ${branch}`);
  }
  if (owner !== undefined && lane.owner !== owner) {
    findings.push(`lane ${lane.id} owner is ${lane.owner || '-'}, not ${owner}`);
  }

  return { lane, findings, ok: findings.length === 0 };
}

export function findLaneByPath(ledger, repoRoot) {
  const normalizedRoot = normalizeLanePath(repoRoot);
  const lane = ledger.lanes.find((candidate) => normalizeLanePath(candidate.path) === normalizedRoot);
  if (lane === undefined) {
    throw new Error(`Current checkout is not registered in lane ledger: ${repoRoot}`);
  }
  return lane;
}

function normalizeLanePath(path) {
  return normalize(path).replace(/\\/gu, '/').toLowerCase();
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
      const ownerText = lane.owner === undefined || lane.owner.length === 0 ? '-' : lane.owner;
      const threadText = lane.thread === undefined || lane.thread.length === 0 ? '-' : lane.thread;
      const nextText = lane.nextAction === undefined || lane.nextAction.length === 0 ? '-' : lane.nextAction;
      const sessionText =
        lane.activeSessionId === undefined || lane.activeSessionId.length === 0 ? '-' : lane.activeSessionId;
      return `${lane.id} | ${lane.status} | owner=${ownerText} | thread=${threadText} | ${lane.branch || '-'} | ${
        lane.task || '-'
      } | next=${nextText} | session=${sessionText} | ${liveText}`;
    })
    .join('\n');
}
