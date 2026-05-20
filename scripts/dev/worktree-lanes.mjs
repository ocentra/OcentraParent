import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync } from 'node:fs';
import { dirname } from 'node:path';
import { pathToFileURL } from 'node:url';

import {
  LaneCommand,
  claimLane,
  defaultLedgerPath,
  defaultLaneOwner,
  ensureLedger,
  formatLedgerSummary,
  freeLane,
  parseLaneArgs,
  readLedger,
  validateLaneContext,
  writeLedger,
} from './worktree-lanes-lib.mjs';

function git(args, options = {}) {
  const output = execFileSync('git', args, {
    cwd: options.cwd,
    encoding: 'utf8',
    stdio: options.stdio ?? ['ignore', 'pipe', 'pipe'],
  });
  return typeof output === 'string' ? output.trim() : '';
}

function repoRoot() {
  return git(['rev-parse', '--show-toplevel']);
}

function currentBranch() {
  return git(['rev-parse', '--abbrev-ref', 'HEAD']);
}

function liveLaneState(lane) {
  if (!existsSync(lane.path)) {
    return { id: lane.id, summary: 'missing-worktree' };
  }

  try {
    const status = git(['-C', lane.path, 'status', '--short', '--branch']);
    return { id: lane.id, summary: status.replace(/\r?\n/gu, ' | ') };
  } catch (error) {
    return { id: lane.id, summary: `git-error:${error.status ?? 'unknown'}` };
  }
}

function assertCleanExistingWorktree(path) {
  if (!existsSync(path)) {
    return;
  }

  const status = git(['-C', path, 'status', '--porcelain']);
  if (status.length > 0) {
    throw new Error(`Lane worktree is dirty and cannot be reused automatically: ${path}`);
  }
}

function createWorktreeIfRequested(lane, options) {
  if (options['create-worktree'] !== true) {
    return;
  }

  if (existsSync(lane.path)) {
    return;
  }

  mkdirSync(dirname(lane.path), { recursive: true });
  git(['worktree', 'add', '-b', lane.branch, lane.path, lane.base ?? 'origin/main'], { stdio: 'inherit' });
}

function requireOption(options, key) {
  const value = options[key];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`Missing required --${key} option.`);
  }
  return value;
}

function handleInit(options) {
  const ledgerPath = options.ledger ?? defaultLedgerPath();
  const ledger = ensureLedger({ ledgerPath, repoRoot: repoRoot(), repoBranch: currentBranch() });
  console.log(`lane-ledger=${ledgerPath}`);
  console.log(formatLedgerSummary(ledger));
}

function handleStatus(options) {
  const ledgerPath = options.ledger ?? defaultLedgerPath();
  const ledger = ensureLedger({ ledgerPath, repoRoot: repoRoot(), repoBranch: currentBranch() });
  const liveStates = ledger.lanes.map(liveLaneState);
  console.log(`lane-ledger=${ledgerPath}`);
  console.log(formatLedgerSummary(ledger, liveStates));
}

function handleClaim(options) {
  const ledgerPath = options.ledger ?? defaultLedgerPath();
  const laneId = requireOption(options, 'lane');
  const branchInput = requireOption(options, 'branch');
  const task = requireOption(options, 'task');
  const ledger = ensureLedger({ ledgerPath, repoRoot: repoRoot(), repoBranch: currentBranch() });
  const { lane } = claimLane(ledger, {
    laneId,
    branchInput,
    task,
    base: options.base ?? 'origin/main',
    force: options.force === true,
    notes: typeof options.notes === 'string' ? options.notes : '',
    owner: typeof options.owner === 'string' ? options.owner : defaultLaneOwner(),
    thread: typeof options.thread === 'string' ? options.thread : '',
  });

  if (options.force !== true) {
    assertCleanExistingWorktree(lane.path);
  }
  createWorktreeIfRequested(lane, options);
  writeLedger(ledgerPath, ledger);
  console.log(`claimed=${lane.id}`);
  console.log(`branch=${lane.branch}`);
  console.log(`path=${lane.path}`);
}

function handleGuard(options) {
  const ledgerPath = options.ledger ?? defaultLedgerPath();
  const ledger = ensureLedger({ ledgerPath, repoRoot: repoRoot(), repoBranch: currentBranch() });
  const result = validateLaneContext(ledger, {
    branch: currentBranch(),
    laneId: typeof options.lane === 'string' ? options.lane : undefined,
    owner: typeof options.owner === 'string' ? options.owner : undefined,
    repoRoot: repoRoot(),
  });

  if (!result.ok) {
    console.error('lane-guard-failed');
    for (const finding of result.findings) {
      console.error(`- ${finding}`);
    }
    console.error('Run npm run lanes:status, then claim or update the correct lane before editing or committing.');
    process.exit(1);
  }

  console.log(
    `lane-guard-ok: lane=${result.lane.id} owner=${result.lane.owner || '-'} thread=${
      result.lane.thread || '-'
    } branch=${result.lane.branch}`
  );
}

function handleFree(options) {
  const ledgerPath = options.ledger ?? defaultLedgerPath();
  const ledger = readLedger(ledgerPath);
  const { lane } = freeLane(ledger, {
    laneId: requireOption(options, 'lane'),
    nextAction: options['next-action'] ?? 'Reusable after fresh status check.',
  });
  writeLedger(ledgerPath, ledger);
  console.log(`freed=${lane.id}`);
}

export function runLaneCli(argv = process.argv.slice(2)) {
  const options = parseLaneArgs(argv);
  if (options.command === LaneCommand.Init) {
    handleInit(options);
    return;
  }
  if (options.command === LaneCommand.Status) {
    handleStatus(options);
    return;
  }
  if (options.command === LaneCommand.Claim) {
    handleClaim(options);
    return;
  }
  if (options.command === LaneCommand.Free) {
    handleFree(options);
    return;
  }
  if (options.command === LaneCommand.Guard) {
    handleGuard(options);
    return;
  }
  throw new Error(`Unknown lane command: ${options.command}`);
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    runLaneCli();
  } catch (error) {
    console.error(error.message);
    process.exit(1);
  }
}
