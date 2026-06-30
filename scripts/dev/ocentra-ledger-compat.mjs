#!/usr/bin/env node
import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const repoRoot = git(['rev-parse', '--show-toplevel']);
const ledgerWrapper = process.env.OCENTRA_LEDGER_WRAPPER ?? join(repoRoot, 'scripts', 'dev', 'ocentra-ledger.mjs');
const guardWrapper = join(repoRoot, 'scripts', 'dev', 'ocentra-ledger-guard.mjs');
const [command, ...rawArgs] = process.argv.slice(2);
const options = parseOptions(rawArgs);
const lane = options.lane ?? process.env.LEDGER_LANE ?? process.env.OCENTRA_PARENT_LEDGER_LANE ?? inferLane(repoRoot);

try {
  await main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}

async function main() {
  switch (command) {
    case 'lanes:init':
      runLedger(['init', 'ocentra-parent', '--lane', lane]);
      return;
    case 'lanes:status':
    case 'hub:status':
      runLedger(['doctor']);
      return;
    case 'lanes:claim':
      runLedger(['worker', required('lane'), 'started', required('task')]);
      return;
    case 'lanes:free':
      runLedger(['worker', required('lane'), 'idle', options['next-action'] ?? 'lane released']);
      return;
    case 'lanes:guard':
    case 'hub:guard':
      runCheckedNode(guardWrapper, []);
      return;
    case 'hub:inbox':
      runLedger(['inbox', lane]);
      return;
    case 'hub:watch':
      await watchLedger();
      return;
    case 'hub:hook':
      hookContext();
      return;
    case 'hub:message':
      runLedger(['msg', required('lane'), messageBody()]);
      return;
    case 'hub:ack':
      acknowledgeLatest();
      return;
    case 'hub:heartbeat':
      runLedger(['heartbeat', lane, mapHeartbeatState(options.state), options.note ?? options.summary ?? 'heartbeat']);
      return;
    case 'hub:heartbeats':
      runLedger(['workers']);
      return;
    case 'hub:report':
      reportWithPrimaryNotification();
      return;
    case 'hub:delegate-grant':
      setDelegateGrant(requiredCurrentPromptedThread(true), required('session-id'));
      return;
    case 'hub:delegate-revoke':
      clearDelegateGrant(requiredCurrentPromptedThread(true), required('session-id'));
      return;
    case 'hub:thread-upgrade':
      setThreadMode(requiredCurrentPromptedThread(false), 'manual-only');
      return;
    case 'hub:thread-default':
      setThreadMode(requiredCurrentPromptedThread(false), 'default');
      return;
    case 'hub:thread-mode':
      printThreadModeStatus();
      return;
    case 'hub:lock':
      {
        const paths = splitClaimPaths(required('paths'));
        if (paths.length === 0) {
          throw new Error('hub:lock requires at least one exact file path.');
        }
        if (paths.length > 10) {
          throw new Error('hub:lock supports at most 10 exact file paths at once.');
        }
        runLedger(['claim', lane, ...paths, '--reason', options.reason ?? 'claimed from product repo']);
      }
      return;
    case 'hub:unlock':
      runLedger(['release', lane, ...splitClaimPaths(required('paths'))]);
      return;
    case 'hub:lane-ledger:audit':
    case 'hub:state:sync':
      console.log('Ocentra Ledger keeps live state outside this repo; no product-repo .hub sync is needed.');
      return;
    default:
      throw new Error(`Unknown Ocentra Ledger compatibility command: ${command ?? '(missing)'}`);
  }
}

function acknowledgeLatest() {
  const inbox = runLedgerJson(['inbox', lane]);
  const latest = Array.isArray(inbox) ? inbox.at(-1) : undefined;
  if (latest === undefined || typeof latest.id !== 'string') {
    console.log(`No unread Ledger messages for ${lane}.`);
    return;
  }
  runLedger(['ack', '--lane', lane, latest.id]);
}

async function watchLedger() {
  const intervalMs = Number.parseInt(options['interval-ms'] ?? options.interval ?? '5000', 10);
  if (!Number.isFinite(intervalMs) || intervalMs < 1000) {
    throw new Error('--interval-ms must be an integer of at least 1000.');
  }
  const once = options.once === true;
  const reports = options.reports === true;
  const seen = new Set();

  const tick = () => {
    const state = runLedgerJson(['materialize']);
    const items = reports ? (state.reports ?? []) : (state.lanes?.[lane]?.inbox ?? []);
    const visible = Array.isArray(items) ? items : [];
    for (const item of visible) {
      const key = item.eventId ?? item.messageId ?? JSON.stringify(item);
      if (!seen.has(key)) {
        seen.add(key);
        console.log(JSON.stringify(item, null, 2));
      }
    }
    if (!options.quiet) {
      console.log(
        `ledger-watch: lane=${lane} mode=${reports ? 'reports' : 'inbox'} checked=${new Date().toISOString()}`
      );
    }
  };

  tick();
  if (once) {
    return;
  }
  setInterval(tick, intervalMs);
}

function hookContext() {
  const input = readStdinJson();
  const eventName = typeof input.hook_event_name === 'string' ? input.hook_event_name : 'hook';
  const session = claimHookSession(input, eventName);
  const context = [
    'Ocentra Ledger coordination context:',
    `- Hook event: ${eventName}.`,
    '- Current lane is configured for this checkout. Set LEDGER_LANE to override lane identity when needed.',
    ...session.context,
    '- State root is external to the product repo. Use npm run ledger:root to inspect it.',
    '- Check work with npm run ledger:doctor, npm run hub:inbox, npm run hub:heartbeats, npm run ledger:workers, and npm run ledger:tasks.',
    '- Send work with npm run hub:message -- --lane codex-b --subject "..." --body "..."; acknowledge with npm run hub:ack.',
    '- Claim exact file paths only with npm run hub:lock -- --paths "file/a.ts,file/b.ts" --reason "..."; claim at most 10 files at once and guard with npm run hub:guard before commit.',
    '- Report STARTED, BLOCKED, PR_READY, DONE, and handoffs through npm run hub:report or the typed ledger worker/task/status commands.',
  ].join('\n');

  console.log(
    JSON.stringify({
      hookSpecificOutput: {
        additionalContext: context,
        hookEventName: normalizeHookEvent(eventName),
      },
    })
  );
}

function claimHookSession(input, eventName) {
  const rawSessionId = input.session_id ?? input.sessionId;
  const sessionId =
    typeof rawSessionId === 'string' && rawSessionId.length > 0
      ? rawSessionId.replace(/[^A-Za-z0-9._-]/gu, '_')
      : undefined;
  if (lane === 'primary' || sessionId === undefined) {
    return { context: [] };
  }

  const state = readCompatThreadState();
  state.latestHookSessionId = sessionId;
  state.latestHookEventName = eventName;
  if (eventName === 'UserPromptSubmit') {
    const nextMode =
      state.latestUserPromptSessionId === sessionId && state.latestUserPromptMode === 'manual-only'
        ? 'manual-only'
        : 'default';
    state.latestUserPromptSessionId = sessionId;
    state.latestUserPromptMode = nextMode;
  }

  const manualOnlyActive =
    state.latestUserPromptSessionId === sessionId &&
    state.latestUserPromptMode === 'manual-only' &&
    eventName !== 'UserPromptSubmit';
  const claimResult = manualOnlyActive
    ? { ok: false, skipped: true }
    : tryRunLedgerJson([
        'session',
        'claim',
        lane,
        sessionId,
        '--ttl-seconds',
        '7200',
        '--summary',
        `${eventName} hook active`,
      ]);
  const activeSessionId = readActiveSessionId();
  writeCompatThreadState(state);

  return {
    context: buildHookContextLines({
      eventName,
      sessionId,
      state,
      activeSessionId,
      claimResult,
    }),
  };
}

function buildHookContextLines({ eventName, sessionId, state, activeSessionId, claimResult }) {
  const lines = [];
  const delegatedBy = state.delegateGrants[sessionId] ?? null;
  const promptedSession = state.latestUserPromptSessionId;
  const promptedOverride = promptedSession === sessionId && activeSessionId !== null && activeSessionId !== sessionId;
  const manualOnlyActive = promptedSession === sessionId && state.latestUserPromptMode === 'manual-only';

  if (manualOnlyActive) {
    lines.push(
      eventName === 'UserPromptSubmit'
        ? '- MANUAL-ONLY: this thread keeps write access only on explicit user prompts.'
        : '- MANUAL-ONLY: automatic lane refresh is disabled for this thread except on explicit user prompts.'
    );
  }

  if (!manualOnlyActive && promptedOverride) {
    lines.push(
      `- USER-OVERRIDE: explicit user prompt keeps this thread writable without taking the lane lease. Active lane session remains ${activeSessionId}.`
    );
  }

  if (delegatedBy !== null) {
    lines.push(
      `- COORDINATED-DELEGATE-GRANT: writable access delegated by ${delegatedBy} without taking the lane lease.`
    );
  }

  if (claimResult.ok) {
    lines.push(
      eventName === 'SessionStart'
        ? '- Active Codex session lease is recorded for this thread; exact-file claims are the write gate.'
        : '- Active Codex session lease is held by this thread; exact-file claims are the write gate.'
    );
    return lines;
  }

  if (claimResult.skipped === true) {
    return lines;
  }

  if (promptedOverride || delegatedBy !== null) {
    return lines;
  }

  if (state.latestUserPromptSessionId === null) {
    lines.push(
      '- Active Codex session lease could not be refreshed, but this thread may still answer questions and inspect status; exact-file claims are the write gate.'
    );
    return lines;
  }

  lines.push(
    `- READ-ONLY: this lane is already owned by another active Codex session (${activeSessionId ?? state.latestUserPromptSessionId}).`
  );
  return lines;
}

function requiredCurrentPromptedThread(allowTargetSessionId) {
  if (!allowTargetSessionId && options['session-id'] !== undefined) {
    throw new Error('Thread mode commands only operate on the current thread after a real user prompt.');
  }
  const state = readCompatThreadState();
  if (state.latestUserPromptSessionId === null) {
    throw new Error('Thread mode commands require the current thread after a real user prompt.');
  }
  if (state.latestHookSessionId !== state.latestUserPromptSessionId) {
    throw new Error(
      `Thread mode commands require the current thread after a real user prompt. latest-hook-session=${state.latestHookSessionId ?? 'none'} latest-user-prompt-session=${state.latestUserPromptSessionId}.`
    );
  }
  return state.latestUserPromptSessionId;
}

function setDelegateGrant(delegatedBy, targetSessionId) {
  const state = readCompatThreadState();
  state.delegateGrants[targetSessionId] = delegatedBy;
  writeCompatThreadState(state);
  console.log(`delegate-grant-set: lane=${lane} session=${targetSessionId} delegated-by=${delegatedBy}`);
}

function clearDelegateGrant(_delegatedBy, targetSessionId) {
  const state = readCompatThreadState();
  delete state.delegateGrants[targetSessionId];
  writeCompatThreadState(state);
  console.log(`delegate-grant-cleared: lane=${lane} session=${targetSessionId}`);
}

function setThreadMode(sessionId, mode) {
  const state = readCompatThreadState();
  state.latestUserPromptSessionId = sessionId;
  state.latestUserPromptMode = mode;
  writeCompatThreadState(state);
  console.log(`thread-mode-set: lane=${lane} session=${sessionId} mode=${mode}`);
}

function printThreadModeStatus() {
  const state = readCompatThreadState();
  const activeSessionId = readActiveSessionId();
  const activeMode =
    activeSessionId !== null && activeSessionId === state.latestUserPromptSessionId
      ? state.latestUserPromptMode
      : 'default';
  const writeGrants = Object.entries(state.delegateGrants);
  const summary = [
    `thread-mode: lane=${lane}`,
    `active-session=${activeSessionId ?? 'none'}`,
    `active-mode=${activeMode}`,
    `latest-user-prompt-session=${state.latestUserPromptSessionId ?? 'none'}`,
    `latest-user-prompt-mode=${state.latestUserPromptMode}`,
    `write-grants=${writeGrants.length === 0 ? 'none' : writeGrants.map(([sessionId, delegatedBy]) => `${sessionId}:${delegatedBy}`).join(',')}`,
  ].join(' ');
  console.log(summary);
}

function messageBody() {
  const subject = options.subject ?? 'Ledger message';
  const body = options.body ?? positionalText();
  return `${subject}\n\n${body}`.trim();
}

function reportBody() {
  const summary = options.summary ?? positionalText();
  const details = options.details;
  validateLifecycleReport(summary, details);
  return details === undefined ? summary : `${summary}\n\n${details}`;
}

function validateLifecycleReport(summary, details) {
  const kind = lifecycleReportKind(summary);
  if (kind === undefined) {
    return;
  }
  if (details === undefined || details.trim().length === 0) {
    throw new Error(
      `${kind} reports require a structured --details block with lane, threadId, assignedBy, plan, workpack, worktree, branch, and scope.`
    );
  }

  const fields = parseMetadataFields(details);
  const required = ['lane', 'threadid', 'assignedby', 'plan', 'workpack', 'worktree', 'branch', 'scope'];
  const stateRequired =
    {
      STARTED: ['startedat'],
      BLOCKED: ['blocker'],
      PR_READY: ['validation'],
      DONE: ['validation', 'commit'],
    }[kind] ?? [];
  const missing = [...required, ...stateRequired].filter((field) => !hasNonEmptyField(fields, field));
  if (missing.length > 0) {
    throw new Error(
      `${kind} reports require structured fields: ${missing.join(', ')}. Use key: value lines in --details.`
    );
  }
}

function lifecycleReportKind(summary) {
  const firstLine = summary.split(/\r?\n/u)[0]?.trim() ?? '';
  const match = firstLine.match(/^(STARTED|BLOCKED|PR(?:[_ -]?READY)|DONE)\b/iu);
  if (match === null) {
    return undefined;
  }
  const token = match[1].replace(/[\s-]/gu, '_').toUpperCase();
  return token === 'PR_READY' ? 'PR_READY' : token;
}

function parseMetadataFields(details) {
  const fields = new Map();
  for (const rawLine of details.split(/\r?\n/gu)) {
    const line = rawLine.trim().replace(/^[*-]\s+/u, '');
    if (line.length === 0) {
      continue;
    }
    const match = line.match(/^([A-Za-z][A-Za-z0-9_-]*)\s*[:=]\s*(.+)$/u);
    if (match === null) {
      continue;
    }
    fields.set(match[1].toLowerCase(), match[2].trim());
  }
  return fields;
}

function hasNonEmptyField(fields, field) {
  return (fields.get(field) ?? '').trim().length > 0;
}

function reportWithPrimaryNotification() {
  const body = reportBody();
  runLedger(['report', '--lane', lane, body]);
  if (shouldNotifyPrimary(body)) {
    runLedger(['msg', 'primary', primaryNotificationBody(body)]);
  }
}

function shouldNotifyPrimary(body) {
  if (lane === 'primary' || options['no-primary-notify'] === true) {
    return false;
  }
  const firstLine = body.split(/\r?\n/u)[0]?.trim() ?? '';
  return /^(?:PR[-_ ]?READY|DONE|BLOCKED)\b/iu.test(firstLine);
}

function primaryNotificationBody(body) {
  const firstLine = body.split(/\r?\n/u)[0]?.trim() || 'Worker report';
  return `Worker report from ${lane}: ${firstLine}\n\n${body}`;
}

function positionalText() {
  return options._.join(' ').trim();
}

function splitClaimPaths(value) {
  return splitPathList(value).map((item) => item.replace(/\\/gu, '/'));
}

function splitPathList(value) {
  return value
    .split(/[,\n]/u)
    .map((item) => item.trim())
    .filter((item) => item.length > 0);
}

function required(name) {
  const value = options[name];
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`Missing required --${name} option.`);
  }
  return value;
}

function mapHeartbeatState(value) {
  switch (value) {
    case 'alive':
    case 'hook':
      return 'online';
    case undefined:
      return 'online';
    default:
      return value;
  }
}

function runLedger(args) {
  const result = runNode(ledgerWrapper, args);
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}

function runLedgerJson(args) {
  const result = spawnSync(process.execPath, [ledgerWrapper, ...args], {
    cwd: repoRoot,
    encoding: 'utf8',
    env: process.env,
    stdio: ['ignore', 'pipe', 'inherit'],
    windowsHide: true,
  });
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
  return JSON.parse(result.stdout);
}

function tryRunLedgerJson(args) {
  const result = spawnSync(process.execPath, [ledgerWrapper, ...args], {
    cwd: repoRoot,
    encoding: 'utf8',
    env: process.env,
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
  let value;
  try {
    value = result.stdout.trim().length === 0 ? undefined : JSON.parse(result.stdout);
  } catch {
    value = undefined;
  }
  return {
    ok: (result.status ?? 1) === 0,
    value,
    stderr: result.stderr,
  };
}

function runNode(scriptPath, args) {
  return spawnSync(process.execPath, [scriptPath, ...args], {
    cwd: repoRoot,
    env: process.env,
    stdio: 'inherit',
    windowsHide: true,
  });
}

function runCheckedNode(scriptPath, args) {
  const result = runNode(scriptPath, args);
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}

function parseOptions(args) {
  const parsed = { _: [] };
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === '--') {
      continue;
    }
    if (!arg.startsWith('--')) {
      parsed._.push(arg);
      continue;
    }
    const equalsIndex = arg.indexOf('=');
    if (equalsIndex > 2) {
      parsed[arg.slice(2, equalsIndex)] = arg.slice(equalsIndex + 1);
      continue;
    }
    const key = arg.slice(2);
    const next = args[index + 1];
    if (next === undefined || next.startsWith('--')) {
      parsed[key] = true;
      continue;
    }
    parsed[key] = next;
    index += 1;
  }
  return parsed;
}

function readStdinJson() {
  try {
    const text = readFileSync(0, 'utf8').trim();
    return text.length === 0 ? {} : JSON.parse(text);
  } catch {
    return {};
  }
}

function compatStateRoot() {
  return process.env.LEDGER_ROOT ?? null;
}

function compatStatePath(fileName) {
  const root = compatStateRoot();
  return root === null ? null : join(root, fileName);
}

function readCompatThreadState() {
  const path = compatStatePath('thread-mode-state.json');
  if (path === null || !existsSync(path)) {
    return {
      latestHookSessionId: null,
      latestHookEventName: null,
      latestUserPromptSessionId: null,
      latestUserPromptMode: 'default',
      delegateGrants: {},
    };
  }

  const parsed = JSON.parse(readFileSync(path, 'utf8'));
  return {
    latestHookSessionId: typeof parsed.latestHookSessionId === 'string' ? parsed.latestHookSessionId : null,
    latestHookEventName: typeof parsed.latestHookEventName === 'string' ? parsed.latestHookEventName : null,
    latestUserPromptSessionId:
      typeof parsed.latestUserPromptSessionId === 'string' ? parsed.latestUserPromptSessionId : null,
    latestUserPromptMode: parsed.latestUserPromptMode === 'manual-only' ? 'manual-only' : 'default',
    delegateGrants:
      parsed.delegateGrants !== null && typeof parsed.delegateGrants === 'object' ? parsed.delegateGrants : {},
  };
}

function writeCompatThreadState(state) {
  const path = compatStatePath('thread-mode-state.json');
  if (path === null) {
    return;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify(state, null, 2));
}

function readActiveSessionId() {
  const path = compatStatePath('active-session.json');
  if (path === null || !existsSync(path)) {
    return null;
  }
  try {
    const parsed = JSON.parse(readFileSync(path, 'utf8'));
    return typeof parsed.sessionId === 'string' && parsed.sessionId.length > 0 ? parsed.sessionId : null;
  } catch {
    return null;
  }
}

function normalizeHookEvent(value) {
  return value
    .split(/[_-]/u)
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join('');
}

function inferLane(path) {
  const normalized = path.replace(/\\/gu, '/');
  const match = normalized.match(/(?:^|[/_-])((?:codex-[a-z])|(?:E-[A-Z]))(?:$|[/_-])/u);
  return match?.[1] ?? 'primary';
}

function git(args) {
  return execFileSync('git', args, {
    cwd: process.cwd(),
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
}
