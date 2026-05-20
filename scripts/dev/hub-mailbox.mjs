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
  unreadMessages,
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

function parseWatchIntervalMs(options) {
  const rawValue = options['interval-ms'] ?? options.interval ?? '5000';
  const parsed = Number.parseInt(rawValue, 10);
  if (!Number.isFinite(parsed) || parsed < 1000) {
    throw new Error('--interval-ms must be an integer of at least 1000.');
  }
  return parsed;
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

function handleWatch(options) {
  const { hubRoot, ledger, root } = context(options);
  const intervalMs = parseWatchIntervalMs(options);
  const once = options.once === true;
  const autoAck = options.ack === true;
  const quiet = options.quiet === true;
  const watchReports = options.reports === true;
  const lanes = watchReports
    ? reportWatchLanes({ ledger, options, root })
    : [watchMessageLane({ ledger, options, root })];
  const printedMessageIds = new Set();
  const printedReportIds = new Set();

  console.log(
    `hub-watch-start: mode=${watchReports ? 'reports' : 'messages'} lanes=${lanes
      .map((lane) => lane.id)
      .join(',')} interval-ms=${intervalMs} ack=${autoAck ? 'on' : 'off'}`
  );

  const checkHub = () => {
    const now = new Date();
    if (watchReports) {
      checkReports({ hubRoot, lanes, now, printedReportIds, quiet });
    } else {
      checkInbox({ autoAck, hubRoot, lane: lanes[0], now, printedMessageIds, quiet });
    }

    if (once) {
      process.exit(0);
    }
  };

  checkHub();
  if (!once) {
    setInterval(checkHub, intervalMs);
  }
}

function watchMessageLane({ ledger, options, root }) {
  return typeof options.lane === 'string' ? findLane(ledger, options.lane) : currentLane(ledger, root);
}

function reportWatchLanes({ ledger, options, root }) {
  if (typeof options.lane === 'string') {
    return [findLane(ledger, options.lane)];
  }
  const lane = currentLane(ledger, root);
  if (lane.id !== 'primary') {
    return [lane];
  }
  return ledger.lanes.filter((candidate) => candidate.id !== 'primary');
}

function checkInbox({ autoAck, hubRoot, lane, now, printedMessageIds, quiet }) {
  const mailbox = readOrCreateMailbox(hubRoot, lane, now);
  const unread = unreadMessages(mailbox);
  const newUnread = unread.filter((message) => !printedMessageIds.has(message.id));

  for (const message of newUnread) {
    printedMessageIds.add(message.id);
    console.log(formatWatchMessage({ lane, message }));
  }

  if (autoAck && unread.length > 0) {
    const latestUnread = unread.at(-1);
    acknowledgeLane({ hubRoot, lane, messageId: latestUnread.id, now });
    console.log(`hub-watch-ack=${latestUnread.id}`);
  } else if (newUnread.length === 0 && !quiet) {
    console.log(`hub-watch-waiting: lane=${lane.id} unread=${unread.length} checked=${now.toISOString()}`);
  }
}

function checkReports({ hubRoot, lanes, now, printedReportIds, quiet }) {
  let printedCount = 0;
  for (const lane of lanes) {
    const mailbox = readOrCreateMailbox(hubRoot, lane, now);
    for (const report of mailbox.reports) {
      if (!printedReportIds.has(report.id)) {
        printedReportIds.add(report.id);
        printedCount += 1;
        console.log(formatWatchReport({ lane, report }));
      }
    }
  }

  if (printedCount === 0 && !quiet) {
    console.log(
      `hub-watch-waiting: mode=reports lanes=${lanes.map((lane) => lane.id).join(',')} checked=${now.toISOString()}`
    );
  }
}

function formatWatchMessage({ lane, message }) {
  return [
    `hub-watch-message: lane=${lane.id} id=${message.id}`,
    `subject=${message.subject}`,
    `created=${message.createdAt}`,
    '',
    message.body,
  ].join('\n');
}

function formatWatchReport({ lane, report }) {
  return [
    `hub-watch-report: lane=${lane.id} id=${report.id}`,
    `summary=${report.summary}`,
    `created=${report.createdAt}`,
    '',
    report.details,
  ].join('\n');
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
  if (options.command === HubCommand.Watch) {
    handleWatch(options);
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
