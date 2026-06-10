import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { dirname, join, normalize, parse, relative } from 'node:path';

import { findLaneByPath } from './worktree-lanes-lib.mjs';

export const HubCommand = Object.freeze({
  Ack: 'ack',
  Guard: 'guard',
  Heartbeat: 'heartbeat',
  Heartbeats: 'heartbeats',
  Inbox: 'inbox',
  Lock: 'lock',
  Message: 'message',
  Report: 'report',
  Status: 'status',
  Unlock: 'unlock',
  Watch: 'watch',
});

const hubDirectoryName = 'ocentra-parent-hub';
const schema = 'https://ocentra.ca/schemas/ocentra-parent-hub-mailbox.v1.json';

export function defaultHubRoot(env = process.env) {
  return env.OCENTRA_PARENT_HUB_ROOT ?? readProjectHubConfig().legacyHubRoot ?? join(repoStateRoot(), hubDirectoryName);
}

function repoStateRoot(cwd = process.cwd()) {
  return join(findRepoRoot(cwd), '.hub', 'state');
}

function readProjectHubConfig(cwd = process.cwd()) {
  const configPath = join(findRepoRoot(cwd), '.hub', 'hub.config.json');
  if (!existsSync(configPath)) {
    return {};
  }

  try {
    return JSON.parse(readFileSync(configPath, 'utf8'));
  } catch {
    return {};
  }
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

export function parseHubArgs(argv) {
  const [command = HubCommand.Status, ...tokens] = argv;
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

export function laneHubPaths(hubRoot, laneId) {
  const laneRoot = join(hubRoot, 'lanes', laneId);
  return {
    inbox: join(laneRoot, 'inbox.md'),
    heartbeat: join(laneRoot, 'heartbeat.ndjson'),
    laneRoot,
    ownership: join(laneRoot, 'ownership.json'),
    status: join(laneRoot, 'status.md'),
  };
}

export function createLaneMailbox(lane, now = new Date()) {
  return {
    schema,
    version: 1,
    laneId: lane.id,
    owner: lane.owner ?? '',
    thread: lane.thread ?? '',
    activeSessionId: lane.activeSessionId ?? '',
    previousSessionId: lane.previousSessionId ?? '',
    sessionSource: lane.sessionSource ?? '',
    sessionUpdatedAt: lane.sessionUpdatedAt ?? '',
    branch: lane.branch ?? '',
    messages: [],
    reports: [],
    lastAcknowledgedMessageId: '',
    lockedPaths: [],
    lockReason: '',
    updatedAt: now.toISOString(),
  };
}

export function ensureHub({ hubRoot, ledger, now = new Date() }) {
  for (const lane of ledger.lanes) {
    const mailbox = readOrCreateMailbox(hubRoot, lane, now);
    const paths = laneHubPaths(hubRoot, lane.id);
    if (syncMailboxFromLane(mailbox, lane, now) || !existsSync(paths.inbox) || !existsSync(paths.status)) {
      writeMailbox(hubRoot, mailbox);
    }
  }
}

export function readOrCreateMailbox(hubRoot, lane, now = new Date()) {
  const paths = laneHubPaths(hubRoot, lane.id);
  if (existsSync(paths.ownership)) {
    const mailbox = JSON.parse(readFileSync(paths.ownership, 'utf8'));
    syncMailboxFromLane(mailbox, lane, now);
    return mailbox;
  }

  const mailbox = createLaneMailbox(lane, now);
  writeMailbox(hubRoot, mailbox);
  return mailbox;
}

export function writeMailbox(hubRoot, mailbox) {
  const paths = laneHubPaths(hubRoot, mailbox.laneId);
  mkdirSync(paths.laneRoot, { recursive: true });
  const temporaryOwnership = `${paths.ownership}.${process.pid}.tmp`;
  writeFileSync(temporaryOwnership, `${JSON.stringify(mailbox, null, 2)}\n`, 'utf8');
  renameSync(temporaryOwnership, paths.ownership);
  syncLaneMailboxFiles(hubRoot, mailbox);
}

export function messageLane({ body, hubRoot, lane, now = new Date(), subject }) {
  const mailbox = readOrCreateMailbox(hubRoot, lane, now);
  const message = {
    id: createMessageId(lane.id, mailbox.messages.length + 1, now),
    subject,
    body,
    createdAt: now.toISOString(),
    acknowledgedAt: '',
  };
  mailbox.owner = lane.owner ?? mailbox.owner;
  mailbox.thread = lane.thread ?? mailbox.thread;
  syncMailboxSessionFromLane(mailbox, lane);
  mailbox.branch = lane.branch ?? mailbox.branch;
  mailbox.messages.push(message);
  mailbox.updatedAt = now.toISOString();
  writeMailbox(hubRoot, mailbox);
  return { mailbox, message };
}

export function acknowledgeLane({ hubRoot, lane, messageId = 'latest', now = new Date() }) {
  const mailbox = readOrCreateMailbox(hubRoot, lane, now);
  const { index, message } = findMessageWithIndex(mailbox, messageId);
  const acknowledgedAt = now.toISOString();
  for (const candidate of mailbox.messages.slice(0, index + 1)) {
    if (typeof candidate.acknowledgedAt !== 'string' || candidate.acknowledgedAt.length === 0) {
      candidate.acknowledgedAt = acknowledgedAt;
    }
  }
  mailbox.lastAcknowledgedMessageId = message.id;
  mailbox.updatedAt = now.toISOString();
  writeMailbox(hubRoot, mailbox);
  return { mailbox, message };
}

export function reportLane({ details = '', hubRoot, lane, now = new Date(), summary }) {
  const mailbox = readOrCreateMailbox(hubRoot, lane, now);
  const report = {
    id: createReportId(lane.id, mailbox.reports.length + 1, now),
    summary,
    details,
    createdAt: now.toISOString(),
  };
  mailbox.owner = lane.owner ?? mailbox.owner;
  mailbox.thread = lane.thread ?? mailbox.thread;
  syncMailboxSessionFromLane(mailbox, lane);
  mailbox.branch = lane.branch ?? mailbox.branch;
  mailbox.reports.push(report);
  mailbox.updatedAt = now.toISOString();
  writeMailbox(hubRoot, mailbox);
  return { mailbox, report };
}

export function lockLanePaths({ force = false, hubRoot, lane, now = new Date(), paths, reason = '', ledger }) {
  const normalizedPaths = paths.map(normalizeRepoPath).filter((path) => path.length > 0);
  if (normalizedPaths.length === 0) {
    throw new Error('At least one --paths value is required.');
  }

  const conflicts = force ? [] : findLockConflicts({ hubRoot, lane, ledger, paths: normalizedPaths, now });
  if (conflicts.length > 0) {
    throw new Error(`Hub lock conflict: ${conflicts.join('; ')}`);
  }

  const mailbox = readOrCreateMailbox(hubRoot, lane, now);
  mailbox.lockedPaths = normalizedPaths;
  mailbox.lockReason = reason;
  mailbox.lockedAt = now.toISOString();
  mailbox.updatedAt = now.toISOString();
  writeMailbox(hubRoot, mailbox);
  return { mailbox };
}

export function unlockLanePaths({ hubRoot, lane, now = new Date() }) {
  const mailbox = readOrCreateMailbox(hubRoot, lane, now);
  mailbox.lockedPaths = [];
  mailbox.lockReason = '';
  mailbox.lockedAt = '';
  mailbox.updatedAt = now.toISOString();
  writeMailbox(hubRoot, mailbox);
  return { mailbox };
}

export function validateHubContext({ changedPaths, hubRoot, ledger, repoRoot }) {
  const lane = findLaneByPath(ledger, repoRoot);
  const mailbox = readOrCreateMailbox(hubRoot, lane);
  const findings = [];
  const latest = latestMessage(mailbox);

  if (latest !== undefined && latest.id !== mailbox.lastAcknowledgedMessageId) {
    findings.push(`lane ${lane.id} has unread hub message ${latest.id}: ${latest.subject}`);
  }

  const normalizedChangedPaths = changedPaths.map(normalizeRepoPath).filter((path) => path.length > 0);
  if (normalizedChangedPaths.length > 0) {
    findings.push(...validateChangedPaths(lane, mailbox, normalizedChangedPaths));
  }

  return { findings, lane, mailbox, ok: findings.length === 0 };
}

export function formatHubSummary({ hubRoot, ledger, now = new Date() }) {
  ensureHub({ hubRoot, ledger, now });
  return ledger.lanes
    .map((lane) => {
      const mailbox = readOrCreateMailbox(hubRoot, lane, now);
      const latest = latestMessage(mailbox);
      const latestText = latest === undefined ? 'message=-' : `message=${latest.id}`;
      const ackText = mailbox.lastAcknowledgedMessageId || '-';
      const locksText = mailbox.lockedPaths.length === 0 ? '-' : mailbox.lockedPaths.join(',');
      const reportText = latestReport(mailbox)?.summary ?? '-';
      const sessionText = mailbox.activeSessionId || '-';
      return `${lane.id} | thread=${lane.thread || '-'} | session=${sessionText} | ${latestText} | ack=${ackText} | locks=${locksText} | report=${reportText}`;
    })
    .join('\n');
}

export function formatInbox(mailbox) {
  return formatInboxMarkdown(mailbox);
}

export function unreadMessages(mailbox) {
  const acknowledgedIndex =
    mailbox.lastAcknowledgedMessageId.length === 0
      ? -1
      : mailbox.messages.findIndex((message) => message.id === mailbox.lastAcknowledgedMessageId);
  if (acknowledgedIndex >= 0) {
    return mailbox.messages.slice(acknowledgedIndex + 1);
  }
  return mailbox.messages.filter(
    (message) => typeof message.acknowledgedAt !== 'string' || message.acknowledgedAt.length === 0
  );
}

export function splitPathList(value) {
  if (typeof value !== 'string') {
    return [];
  }
  return value
    .split(/[,\n]/u)
    .map((path) => path.trim())
    .filter((path) => path.length > 0);
}

function syncLaneMailboxFiles(hubRoot, mailbox) {
  const paths = laneHubPaths(hubRoot, mailbox.laneId);
  mkdirSync(paths.laneRoot, { recursive: true });
  writeFileSync(paths.inbox, formatInboxMarkdown(mailbox), 'utf8');
  writeFileSync(paths.status, formatStatusMarkdown(mailbox), 'utf8');
}

function syncMailboxFromLane(mailbox, lane, now) {
  const nextOwner = lane.owner ?? '';
  const nextThread = lane.thread ?? '';
  const nextBranch = lane.branch ?? '';
  const nextActiveSessionId = lane.activeSessionId ?? '';
  const nextPreviousSessionId = lane.previousSessionId ?? '';
  const nextSessionSource = lane.sessionSource ?? '';
  const nextSessionUpdatedAt = lane.sessionUpdatedAt ?? '';
  if (
    mailbox.owner === nextOwner &&
    mailbox.thread === nextThread &&
    mailbox.branch === nextBranch &&
    mailbox.activeSessionId === nextActiveSessionId &&
    mailbox.previousSessionId === nextPreviousSessionId &&
    mailbox.sessionSource === nextSessionSource &&
    mailbox.sessionUpdatedAt === nextSessionUpdatedAt
  ) {
    return false;
  }
  mailbox.owner = nextOwner;
  mailbox.thread = nextThread;
  mailbox.branch = nextBranch;
  mailbox.activeSessionId = nextActiveSessionId;
  mailbox.previousSessionId = nextPreviousSessionId;
  mailbox.sessionSource = nextSessionSource;
  mailbox.sessionUpdatedAt = nextSessionUpdatedAt;
  mailbox.updatedAt = now.toISOString();
  return true;
}

function syncMailboxSessionFromLane(mailbox, lane) {
  mailbox.activeSessionId = lane.activeSessionId ?? mailbox.activeSessionId ?? '';
  mailbox.previousSessionId = lane.previousSessionId ?? mailbox.previousSessionId ?? '';
  mailbox.sessionSource = lane.sessionSource ?? mailbox.sessionSource ?? '';
  mailbox.sessionUpdatedAt = lane.sessionUpdatedAt ?? mailbox.sessionUpdatedAt ?? '';
}

function createMessageId(laneId, index, now) {
  return `${laneId}-msg-${stamp(now)}-${index}`;
}

function createReportId(laneId, index, now) {
  return `${laneId}-report-${stamp(now)}-${index}`;
}

function stamp(now) {
  return now.toISOString().replace(/[^0-9a-z]/giu, '');
}

function latestMessage(mailbox) {
  return mailbox.messages.at(-1);
}

function latestReport(mailbox) {
  return mailbox.reports.at(-1);
}

function findMessageWithIndex(mailbox, messageId) {
  const index =
    messageId === 'latest' ? mailbox.messages.length - 1 : mailbox.messages.findIndex((item) => item.id === messageId);
  if (index < 0) {
    throw new Error(`Unknown hub message: ${messageId}`);
  }
  return { index, message: mailbox.messages[index] };
}

function validateChangedPaths(lane, mailbox, changedPaths) {
  if (lane.id === 'primary' && mailbox.lockedPaths.length === 0) {
    return [];
  }
  if (mailbox.lockedPaths.length === 0) {
    return [`lane ${lane.id} has changed files but no hub file lock`];
  }

  return changedPaths
    .filter((path) => !mailbox.lockedPaths.some((lockedPath) => pathMatchesLock(path, lockedPath)))
    .map((path) => `changed path ${path} is outside hub locks for lane ${lane.id}`);
}

function findLockConflicts({ hubRoot, lane, ledger, paths, now }) {
  const conflicts = [];
  for (const candidate of ledger.lanes) {
    if (candidate.id === lane.id) {
      continue;
    }
    const mailbox = readOrCreateMailbox(hubRoot, candidate, now);
    for (const path of paths) {
      const conflict = mailbox.lockedPaths.find(
        (lockedPath) => pathMatchesLock(path, lockedPath) || pathMatchesLock(lockedPath, path)
      );
      if (conflict !== undefined) {
        conflicts.push(`${candidate.id} owns ${conflict}`);
      }
    }
  }
  return conflicts;
}

function pathMatchesLock(path, lockedPath) {
  const normalizedPath = normalizeRepoPath(path);
  const normalizedLock = normalizeRepoPath(lockedPath);
  if (normalizedLock.includes('*')) {
    return wildcardToRegExp(normalizedLock).test(normalizedPath);
  }
  return normalizedPath === normalizedLock || normalizedPath.startsWith(`${normalizedLock}/`);
}

function wildcardToRegExp(pattern) {
  const escaped = pattern.replace(/[.+?^${}()|[\]\\]/gu, '\\$&').replace(/\*/gu, '.*');
  return new RegExp(`^${escaped}$`, 'u');
}

function normalizeRepoPath(path) {
  return normalize(path).replace(/\\/gu, '/').replace(/^\.\//u, '').toLowerCase();
}

function formatInboxMarkdown(mailbox) {
  const lines = [
    `# Lane Inbox: ${mailbox.laneId}`,
    '',
    `Owner: ${mailbox.owner || '-'}`,
    `Thread: ${mailbox.thread || '-'}`,
    `Active session: ${mailbox.activeSessionId || '-'}`,
    '',
  ];
  if (mailbox.messages.length === 0) {
    lines.push('No hub messages.', '');
    return `${lines.join('\n')}`;
  }

  for (const message of mailbox.messages) {
    const read =
      typeof message.acknowledgedAt === 'string' && message.acknowledgedAt.length > 0 ? 'acknowledged' : 'unread';
    lines.push(
      `## ${message.subject}`,
      '',
      `- id: ${message.id}`,
      `- status: ${read}`,
      `- created: ${message.createdAt}`,
      ''
    );
    lines.push(message.body, '');
  }
  return `${lines.join('\n')}`;
}

function formatStatusMarkdown(mailbox) {
  const report = latestReport(mailbox);
  const lines = [
    `# Lane Status: ${mailbox.laneId}`,
    '',
    `Owner: ${mailbox.owner || '-'}`,
    `Thread: ${mailbox.thread || '-'}`,
    `Active session: ${mailbox.activeSessionId || '-'}`,
    `Previous session: ${mailbox.previousSessionId || '-'}`,
    `Session source: ${mailbox.sessionSource || '-'}`,
    `Branch: ${mailbox.branch || '-'}`,
    `Locks: ${mailbox.lockedPaths.length === 0 ? '-' : mailbox.lockedPaths.join(', ')}`,
    `Lock reason: ${mailbox.lockReason || '-'}`,
    '',
  ];

  if (report === undefined) {
    lines.push('No lane reports yet.', '');
    return `${lines.join('\n')}`;
  }

  lines.push(
    `## Latest Report`,
    '',
    `- id: ${report.id}`,
    `- created: ${report.createdAt}`,
    `- summary: ${report.summary}`,
    ''
  );
  if (report.details.length > 0) {
    lines.push(report.details, '');
  }
  return `${lines.join('\n')}`;
}

export function repoRelativePath(repoRoot, path) {
  return relative(repoRoot, path).replace(/\\/gu, '/');
}
