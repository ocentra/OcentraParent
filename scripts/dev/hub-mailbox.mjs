import { execFileSync } from 'node:child_process';
import { pathToFileURL } from 'node:url';

import {
  HubCommand,
  acknowledgeLane,
  defaultHubRoot,
  ensureHub,
  formatHubSummary,
  formatInbox,
  lockLanePaths,
  messageLane,
  parseHubArgs,
  readOrCreateMailbox,
  reportLane,
  splitPathList,
  unlockLanePaths,
  validateHubContext,
} from './hub-mailbox-lib.mjs';
import { defaultLedgerPath, ensureLedger, findLane, findLaneByPath } from './worktree-lanes-lib.mjs';

function git(args) {
  return execFileSync('git', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
}

function repoRoot() {
  return git(['rev-parse', '--show-toplevel']);
}

function currentBranch() {
  return git(['rev-parse', '--abbrev-ref', 'HEAD']);
}

function changedPaths() {
  const output = git(['diff', '--name-only', 'HEAD']);
  return output.length === 0 ? [] : output.split(/\r?\n/u);
}

function context(options) {
  const root = repoRoot();
  const ledgerPath = options.ledger ?? defaultLedgerPath();
  const ledger = ensureLedger({ ledgerPath, repoRoot: root, repoBranch: currentBranch() });
  const hubRoot = options.hub ?? defaultHubRoot();
  ensureHub({ hubRoot, ledger });
  return { hubRoot, ledger, root };
}

function currentLane(ledger, root) {
  return findLaneByPath(ledger, root);
}

function requireOption(options, key) {
  const value = options[key];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`Missing required --${key} option.`);
  }
  return value;
}

function handleStatus(options) {
  const { hubRoot, ledger } = context(options);
  console.log(`hub-root=${hubRoot}`);
  console.log(formatHubSummary({ hubRoot, ledger }));
}

function handleInbox(options) {
  const { hubRoot, ledger, root } = context(options);
  const lane = typeof options.lane === 'string' ? findLane(ledger, options.lane) : currentLane(ledger, root);
  console.log(formatInbox(readOrCreateMailbox(hubRoot, lane)));
}

function handleMessage(options) {
  const { hubRoot, ledger } = context(options);
  const lane = findLane(ledger, requireOption(options, 'lane'));
  const { message } = messageLane({
    body: requireOption(options, 'body'),
    hubRoot,
    lane,
    subject: requireOption(options, 'subject'),
  });
  console.log(`hub-message=${message.id}`);
}

function handleAck(options) {
  const { hubRoot, ledger, root } = context(options);
  const lane = typeof options.lane === 'string' ? findLane(ledger, options.lane) : currentLane(ledger, root);
  const { message } = acknowledgeLane({
    hubRoot,
    lane,
    messageId: typeof options.message === 'string' ? options.message : 'latest',
  });
  console.log(`hub-ack=${message.id}`);
}

function handleReport(options) {
  const { hubRoot, ledger, root } = context(options);
  const lane = typeof options.lane === 'string' ? findLane(ledger, options.lane) : currentLane(ledger, root);
  const { report } = reportLane({
    details: typeof options.details === 'string' ? options.details : '',
    hubRoot,
    lane,
    summary: requireOption(options, 'summary'),
  });
  console.log(`hub-report=${report.id}`);
}

function handleLock(options) {
  const { hubRoot, ledger, root } = context(options);
  const lane = typeof options.lane === 'string' ? findLane(ledger, options.lane) : currentLane(ledger, root);
  const paths = splitPathList(requireOption(options, 'paths'));
  lockLanePaths({
    force: options.force === true,
    hubRoot,
    lane,
    ledger,
    paths,
    reason: typeof options.reason === 'string' ? options.reason : '',
  });
  console.log(`hub-lock=${lane.id}`);
}

function handleUnlock(options) {
  const { hubRoot, ledger, root } = context(options);
  const lane = typeof options.lane === 'string' ? findLane(ledger, options.lane) : currentLane(ledger, root);
  unlockLanePaths({ hubRoot, lane });
  console.log(`hub-unlock=${lane.id}`);
}

function handleGuard(options) {
  const { hubRoot, ledger, root } = context(options);
  const result = validateHubContext({
    changedPaths: changedPaths(),
    hubRoot,
    ledger,
    repoRoot: root,
  });

  if (!result.ok) {
    console.error('hub-guard-failed');
    for (const finding of result.findings) {
      console.error(`- ${finding}`);
    }
    console.error('Run npm run hub:inbox, npm run hub:ack, or npm run hub:lock before committing.');
    process.exit(1);
  }

  console.log(`hub-guard-ok: lane=${result.lane.id}`);
}

export function runHubCli(argv = process.argv.slice(2)) {
  const options = parseHubArgs(argv);
  if (options.command === HubCommand.Status) {
    handleStatus(options);
    return;
  }
  if (options.command === HubCommand.Inbox) {
    handleInbox(options);
    return;
  }
  if (options.command === HubCommand.Message) {
    handleMessage(options);
    return;
  }
  if (options.command === HubCommand.Ack) {
    handleAck(options);
    return;
  }
  if (options.command === HubCommand.Report) {
    handleReport(options);
    return;
  }
  if (options.command === HubCommand.Lock) {
    handleLock(options);
    return;
  }
  if (options.command === HubCommand.Unlock) {
    handleUnlock(options);
    return;
  }
  if (options.command === HubCommand.Guard) {
    handleGuard(options);
    return;
  }
  throw new Error(`Unknown hub command: ${options.command}`);
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    runHubCli();
  } catch (error) {
    console.error(error.message);
    process.exit(1);
  }
}
