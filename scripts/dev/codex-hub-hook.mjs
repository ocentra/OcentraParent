import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

import {
  defaultHubRoot,
  ensureHub,
  formatHubSummary,
  readOrCreateMailbox,
  unreadMessages,
} from './hub-mailbox-lib.mjs';
import {
  defaultLedgerPath,
  ensureLedger,
  findLaneByPath,
  recordLaneSession,
  writeLedger,
} from './worktree-lanes-lib.mjs';

const HookEvent = Object.freeze({
  PostToolUse: 'PostToolUse',
  SessionStart: 'SessionStart',
  Stop: 'Stop',
  UserPromptSubmit: 'UserPromptSubmit',
});

export function buildHookResponse(input, context = undefined) {
  const eventName = normalizeEventName(input.hook_event_name);
  const hubContext = context ?? loadHubContext(input, eventName);

  if (eventName === HookEvent.SessionStart || eventName === HookEvent.UserPromptSubmit) {
    return additionalContextResponse(eventName, formatAgentContext(hubContext));
  }

  if (eventName === HookEvent.PostToolUse) {
    const contextText = formatPostToolContext(hubContext);
    return contextText.length === 0 ? undefined : additionalContextResponse(eventName, contextText);
  }

  if (eventName === HookEvent.Stop) {
    return buildStopResponse({ context: hubContext, stopHookActive: input.stop_hook_active === true });
  }

  return undefined;
}

export function buildStopResponse({ context, stopHookActive = false }) {
  if (stopHookActive) {
    return { continue: true };
  }

  if (context.lane.id !== 'primary' && context.unread.length > 0) {
    return continueTurn(
      [
        `Hub coordination is not complete for ${context.lane.id}.`,
        `Unread hub message(s): ${formatMessageList(context.unread)}.`,
        'Run npm run hub:inbox, acknowledge with npm run hub:ack, follow the instruction, and report with npm run hub:report when done.',
      ].join(' ')
    );
  }

  if (context.lane.id !== 'primary' && context.changedPaths.length > 0 && context.mailbox.lockedPaths.length === 0) {
    return continueTurn(
      [
        `Worker lane ${context.lane.id} has changed files but no hub file lock.`,
        'Run npm run hub:lock for the intended paths or report the blocker before ending the turn.',
      ].join(' ')
    );
  }

  if (context.lane.id !== 'primary' && context.changedPaths.length > 0 && context.latestReport === undefined) {
    return continueTurn(
      [
        `Worker lane ${context.lane.id} has changed files but no hub report.`,
        'Run npm run hub:report with changed paths, validation, blockers, and branch state before ending the turn.',
      ].join(' ')
    );
  }

  return { continue: true };
}

export function formatAgentContext(context) {
  if (context.lane.id === 'primary') {
    return [
      'Ocentra Parent hub context:',
      `- Current lane: primary (${context.branch}). This chat coordinates workers and integrates finished work; do not do feature coding here unless explicitly instructed.`,
      ...formatSessionLines(context),
      '- Worker summary:',
      indent(context.hubSummary),
      '- To send work: npm run hub:message -- --lane codex-a --subject "..." --body "..."',
      '- To watch worker reports without manual polling: npm run hub:watch -- --reports --interval-ms 1000',
    ].join('\n');
  }

  const unreadText =
    context.unread.length === 0
      ? '- No unread hub messages for this worker lane.'
      : [
          `- Unread hub message(s): ${formatMessageList(context.unread)}.`,
          '- Before doing other work: run npm run hub:inbox, then npm run hub:ack after reading.',
          '- Follow the latest hub instruction, then report back with npm run hub:report.',
        ].join('\n');
  const latestMessage = context.mailbox.messages.at(-1);
  const latestText =
    latestMessage === undefined
      ? '- Latest hub message: none.'
      : `- Latest hub message: ${latestMessage.id} (${latestMessage.subject}).`;
  const latestReport =
    context.latestReport === undefined
      ? '- Latest worker report: none.'
      : `- Latest worker report: ${context.latestReport.id} (${context.latestReport.summary}).`;
  const acknowledgedMessage =
    typeof context.mailbox.lastAcknowledgedMessageId === 'string' &&
    context.mailbox.lastAcknowledgedMessageId.length > 0
      ? `- Latest acknowledged hub message: ${context.mailbox.lastAcknowledgedMessageId}.`
      : '- Latest acknowledged hub message: none.';

  return [
    'Ocentra Parent worker hub context:',
    `- Current lane: ${context.lane.id}; thread=${context.lane.thread || '-'}; branch=${context.branch}.`,
    ...formatSessionLines(context),
    latestText,
    acknowledgedMessage,
    unreadText,
    `- Current locks: ${context.mailbox.lockedPaths.length === 0 ? '-' : context.mailbox.lockedPaths.join(', ')}.`,
    latestReport,
    '- Worker protocol: acknowledge hub messages, lock intended paths before editing, validate locally, report back with npm run hub:report, and do not merge to main.',
  ].join('\n');
}

export function formatPostToolContext(context) {
  if (context.lane.id === 'primary') {
    return '';
  }
  if (context.changedPaths.length === 0) {
    return '';
  }
  if (context.mailbox.lockedPaths.length > 0) {
    return '';
  }
  return [
    `Worker lane ${context.lane.id} now has changed files but no hub file lock.`,
    `Changed path(s): ${context.changedPaths.slice(0, 8).join(', ')}${context.changedPaths.length > 8 ? ', ...' : ''}.`,
    'Before continuing edits, run npm run hub:lock -- --paths "path/or/package" --reason "short scope".',
  ].join(' ');
}

function loadHubContext(input, eventName) {
  const cwd = typeof input.cwd === 'string' && input.cwd.length > 0 ? input.cwd : process.cwd();
  const repoRoot = git(cwd, ['rev-parse', '--show-toplevel']);
  const branch = git(repoRoot, ['rev-parse', '--abbrev-ref', 'HEAD']);
  const ledgerPath = defaultLedgerPath();
  const ledger = ensureLedger({ ledgerPath, repoRoot, repoBranch: branch });
  const lane = findLaneByPath(ledger, repoRoot);
  const sessionId = typeof input.session_id === 'string' ? input.session_id : '';
  const sessionSource = sessionSourceText({ eventName, source: input.source });
  const shouldRecordSession = eventName === HookEvent.SessionStart || eventName === HookEvent.UserPromptSubmit;
  const sessionRecord =
    sessionId.length === 0 || !shouldRecordSession
      ? { changed: false, previousSessionId: lane.activeSessionId ?? '' }
      : recordLaneSession(ledger, { laneId: lane.id, sessionId, source: sessionSource });
  if (sessionRecord.changed) {
    writeLedger(ledgerPath, ledger);
  }
  const hubRoot = defaultHubRoot();
  ensureHub({ hubRoot, ledger });
  const mailbox = readOrCreateMailbox(hubRoot, lane);
  return {
    branch,
    changedPaths: gitLines(repoRoot, ['diff', '--name-only', 'HEAD']),
    hubRoot,
    hubSummary: formatHubSummary({ hubRoot, ledger }),
    lane,
    latestReport: mailbox.reports.at(-1),
    mailbox,
    previousSessionId: sessionRecord.previousSessionId,
    sessionId,
    sessionRecordChanged: sessionRecord.changed,
    sessionSource,
    unread: unreadMessages(mailbox),
  };
}

function normalizeEventName(value) {
  if (typeof value !== 'string') {
    return '';
  }
  return value
    .split(/[_-]/u)
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join('');
}

function additionalContextResponse(eventName, additionalContext) {
  return {
    hookSpecificOutput: {
      additionalContext,
      hookEventName: eventName,
    },
  };
}

function continueTurn(reason) {
  return {
    decision: 'block',
    reason,
  };
}

function formatMessageList(messages) {
  return messages.map((message) => `${message.id} (${message.subject})`).join(', ');
}

function formatSessionLines(context) {
  const sessionId = context.sessionId || context.lane.activeSessionId || context.mailbox.activeSessionId || '';
  if (sessionId.length === 0) {
    return [];
  }

  const previousSessionId =
    context.previousSessionId || context.lane.previousSessionId || context.mailbox.previousSessionId || '';
  const sourceText = context.sessionSource || context.lane.sessionSource || context.mailbox.sessionSource || '-';
  const lines = [`- Active Codex thread/session: ${sessionId} (${sourceText}).`];
  if (context.sessionRecordChanged === true && previousSessionId.length > 0 && previousSessionId !== sessionId) {
    lines.push(`- This is a new chat for the same lane; previous active session was ${previousSessionId}.`);
  }
  lines.push(
    '- Use hub ack/report state and git status as source of truth; do not rerun already acknowledged hub messages just because this chat is new.'
  );
  return lines;
}

function sessionSourceText({ eventName, source }) {
  const sourceText = typeof source === 'string' && source.length > 0 ? source : 'unknown';
  return `${eventName || 'Unknown'}:${sourceText}`;
}

function indent(value) {
  return value
    .split(/\r?\n/u)
    .map((line) => `  ${line}`)
    .join('\n');
}

function git(cwd, args) {
  return execFileSync('git', args, { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
}

function gitLines(cwd, args) {
  const output = git(cwd, args);
  return output.length === 0 ? [] : output.split(/\r?\n/u);
}

function readStdin() {
  return readFileSync(0, 'utf8');
}

function main() {
  const input = JSON.parse(readStdin());
  const response = buildHookResponse(input);
  if (response !== undefined) {
    process.stdout.write(`${JSON.stringify(response)}\n`);
  }
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    main();
  } catch (error) {
    const fallback = {
      systemMessage: `Ocentra hub hook failed: ${error instanceof Error ? error.message : String(error)}`,
    };
    process.stdout.write(`${JSON.stringify(fallback)}\n`);
  }
}
