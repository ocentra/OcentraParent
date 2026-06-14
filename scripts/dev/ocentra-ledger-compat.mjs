#!/usr/bin/env node
import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

const repoRoot = git(['rev-parse', '--show-toplevel']);
const ledgerWrapper = process.env.OCENTRA_LEDGER_WRAPPER ?? join(repoRoot, 'scripts', 'dev', 'ocentra-ledger.mjs');
const guardWrapper = join(repoRoot, 'scripts', 'dev', 'ocentra-ledger-guard.mjs');
const [command, ...rawArgs] = process.argv.slice(2);
const options = parseOptions(rawArgs);
const lane = options.lane ?? process.env.LEDGER_LANE ?? process.env.OCENTRA_PARENT_LEDGER_LANE ?? inferLane(repoRoot);
const defaultThreadMode = 'default';
const manualOnlyThreadMode = 'manual-only';
const userPromptHookEvent = 'UserPromptSubmit';
let cachedCompatibilityStateRoot;

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
    case 'hub:thread-mode':
      configureCurrentThreadMode();
      return;
    case 'hub:thread-upgrade':
      configureCurrentThreadMode(manualOnlyThreadMode);
      return;
    case 'hub:thread-default':
      configureCurrentThreadMode(defaultThreadMode);
      return;
    case 'hub:delegate-grant':
      configureDelegateGrant();
      return;
    case 'hub:delegate-revoke':
      revokeDelegateGrant();
      return;
    case 'hub:lock':
      runLedger(['claim', lane, required('paths'), '--reason', options.reason ?? 'claimed from product repo']);
      return;
    case 'hub:unlock':
      runLedger(['release', lane, required('paths')]);
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
  const eventName = normalizeHookEvent(typeof input.hook_event_name === 'string' ? input.hook_event_name : 'hook');
  const session = claimHookSession(input, eventName);
  const context = [
    'Ocentra Ledger coordination context:',
    `- Hook event: ${eventName}.`,
    '- Current lane is configured for this checkout. Set LEDGER_LANE to override lane identity when needed.',
    ...session.context,
    '- State root is external to the product repo. Use npm run ledger:root to inspect it.',
    '- Check work with npm run ledger:doctor, npm run hub:inbox, npm run hub:heartbeats, npm run ledger:workers, and npm run ledger:tasks.',
    '- Send work with npm run hub:message -- --lane codex-b --subject "..." --body "..."; acknowledge with npm run hub:ack.',
    '- Claim paths with npm run hub:lock -- --paths "path/or/glob" --reason "..."; guard with npm run hub:guard before commit.',
    '- Report STARTED, BLOCKED, PR-ready, DONE, and handoffs through npm run hub:report or the typed ledger worker/task/status commands.',
  ].join('\n');

  console.log(
    JSON.stringify({
      hookSpecificOutput: {
        additionalContext: context,
        hookEventName: eventName,
      },
    })
  );
}

function claimHookSession(input, eventName) {
  const sessionId = sanitizeSessionId(input.session_id ?? input.sessionId);
  if (sessionId !== undefined) {
    rememberHookSession(sessionId, eventName);
  }

  const threadMode = sessionId === undefined ? defaultThreadMode : readThreadMode(sessionId);
  const threadModeContext = buildThreadModeContext(threadMode, eventName);
  const writeGrant = sessionId === undefined ? undefined : readWriteGrant(sessionId);

  if (lane === 'primary' || sessionId === undefined) {
    return { context: compactContext(threadModeContext) };
  }

  if (threadMode === manualOnlyThreadMode && eventName !== userPromptHookEvent) {
    return { context: compactContext(threadModeContext, buildWriteGrantContext(writeGrant)) };
  }

  const result = claimLedgerSession(sessionId, eventName);
  if (result.ok) {
    return {
      context: compactContext(
        threadModeContext,
        '- Active Codex session lease is held by this thread until another session claims it or the lease expires.'
      ),
    };
  }
  if (eventName === userPromptHookEvent) {
    const promptGrant = grantWriteAccessForUserPrompt(sessionId, result.value?.activeSession?.sessionId);
    return {
      context: compactContext(threadModeContext, buildWriteGrantContext(promptGrant)),
    };
  }
  if (writeGrant !== undefined) {
    return {
      context: compactContext(threadModeContext, buildWriteGrantContext(writeGrant)),
    };
  }
  return {
    context: compactContext(
      threadModeContext,
      '- READ-ONLY: this lane is already owned by another active Codex session. You may answer questions and inspect status, but do not ack mail, edit files, claim paths, heartbeat, or report work from this thread unless the user explicitly prompts this thread or a prompted coordinator grants it write authority.'
    ),
  };
}

function configureCurrentThreadMode(modeOverride) {
  rejectExplicitSessionTarget();
  const requestedMode = modeOverride ?? (typeof options.mode === 'string' ? options.mode : undefined);

  if (requestedMode === undefined) {
    console.log(describeCurrentThreadMode());
    return;
  }

  const sessionId = resolveCurrentThreadModeSessionId();
  const mode = normalizeThreadModeValue(requestedMode);
  writeCompatibilityJson(threadModePath(sessionId), {
    lane,
    sessionId,
    mode,
    scope: 'current-thread-user-prompt',
    updatedAt: new Date().toISOString(),
  });
  console.log(`thread-mode-set: lane=${lane} session=${sessionId} mode=${mode}`);
}

function configureDelegateGrant() {
  const delegatorSessionId = resolveCurrentThreadModeSessionId();
  const sessionId = requiredSessionIdOption();
  writeCompatibilityJson(writeGrantPath(sessionId), {
    lane,
    sessionId,
    source: 'delegate',
    delegatedBySessionId: delegatorSessionId,
    reason: options.reason ?? 'coordinator delegated write authority',
    paths: typeof options.paths === 'string' ? options.paths : undefined,
    updatedAt: new Date().toISOString(),
  });
  console.log(`delegate-grant-set: lane=${lane} session=${sessionId} delegated-by=${delegatorSessionId}`);
}

function revokeDelegateGrant() {
  resolveCurrentThreadModeSessionId();
  const sessionId = requiredSessionIdOption();
  removeCompatibilityJson(writeGrantPath(sessionId));
  console.log(`delegate-grant-cleared: lane=${lane} session=${sessionId}`);
}

function rejectExplicitSessionTarget() {
  if (options['session-id'] !== undefined) {
    throw new Error(
      'Thread mode changes may only target the current thread. Re-run this command from the thread you want to change after an explicit user prompt.'
    );
  }
}

function resolveCurrentThreadModeSessionId() {
  const latestPrompt = readCompatibilityJson(latestUserPromptSessionPath());
  const promptSessionId = sanitizeSessionId(latestPrompt?.sessionId);
  if (promptSessionId === undefined) {
    throw new Error(
      `No current UserPromptSubmit session is recorded for lane ${lane}. Ask the target thread to run this after a user prompt.`
    );
  }

  const lastHook = readCompatibilityJson(lastHookSessionPath());
  const activeSessionId = sanitizeSessionId(lastHook?.sessionId);
  if (activeSessionId === undefined || activeSessionId !== promptSessionId) {
    throw new Error(
      `Thread mode changes may only be made from the current thread after a real user prompt. Active hook session=${activeSessionId ?? 'unknown'} latest user prompt session=${promptSessionId}. Ask the target thread directly.`
    );
  }

  return promptSessionId;
}

function describeCurrentThreadMode() {
  const lastHook = readCompatibilityJson(lastHookSessionPath());
  const activeSessionId = sanitizeSessionId(lastHook?.sessionId);
  const latestPrompt = readCompatibilityJson(latestUserPromptSessionPath());
  const promptSessionId = sanitizeSessionId(latestPrompt?.sessionId);
  const activeMode = activeSessionId === undefined ? defaultThreadMode : readThreadMode(activeSessionId);
  const promptMode = promptSessionId === undefined ? defaultThreadMode : readThreadMode(promptSessionId);
  const writeGrantSessions = listWriteGrantSessions();

  return [
    `thread-mode: lane=${lane}`,
    `active-session=${activeSessionId ?? 'unknown'}`,
    `active-mode=${activeMode}`,
    `latest-user-prompt-session=${promptSessionId ?? 'unknown'}`,
    `latest-user-prompt-mode=${promptMode}`,
    `write-grants=${writeGrantSessions.length === 0 ? 'none' : writeGrantSessions.join(',')}`,
  ].join(' ');
}

function readThreadMode(sessionId) {
  const record = readCompatibilityJson(threadModePath(sessionId));
  if (typeof record?.mode !== 'string') {
    return defaultThreadMode;
  }
  try {
    return normalizeThreadModeValue(record.mode);
  } catch {
    return defaultThreadMode;
  }
}

function rememberHookSession(sessionId, eventName) {
  const payload = {
    lane,
    sessionId,
    eventName,
    updatedAt: new Date().toISOString(),
  };
  writeCompatibilityJson(lastHookSessionPath(), payload);
  if (eventName === userPromptHookEvent) {
    writeCompatibilityJson(latestUserPromptSessionPath(), payload);
  }
}

function buildThreadModeContext(threadMode, eventName) {
  if (threadMode !== manualOnlyThreadMode) {
    return undefined;
  }
  if (eventName === userPromptHookEvent) {
    return '- MANUAL-ONLY: this thread accepts writable work only from explicit user prompts. This prompt may claim or refresh the lane lease for this session.';
  }
  return `- MANUAL-ONLY: this thread accepts writable work only from explicit user prompts. Auto hooks like ${eventName} do not claim or refresh the lane lease for this session.`;
}

function compactContext(...lines) {
  return lines.filter((line) => typeof line === 'string' && line.length > 0);
}

function buildWriteGrantContext(grant) {
  if (grant === undefined) {
    return undefined;
  }
  if (grant.source === 'delegate') {
    return `- COORDINATED-DELEGATE-GRANT: this session is writable because session ${grant.delegatedBySessionId ?? 'unknown'} delegated lane write authority for this worker. The active lane lease may still belong to another session; single-owner auto-hook rules still apply at the lease level.`;
  }

  const laneLeaseSessionId = sanitizeSessionId(grant.laneLeaseSessionId);
  const leaseContext =
    laneLeaseSessionId === undefined
      ? ' The active lease may still belong to another session.'
      : ` The active lease remains with session ${laneLeaseSessionId}.`;
  return `- USER-SUPERUSER-OVERRIDE: this explicit user prompt granted writable authority to this thread without taking the lane lease.${leaseContext} Single-owner lane rules still apply to AI auto hooks.`;
}

function normalizeThreadModeValue(value) {
  switch (value) {
    case 'default':
    case 'normal':
    case 'auto':
      return defaultThreadMode;
    case 'manual-only':
    case 'manual':
    case 'user-driven':
      return manualOnlyThreadMode;
    default:
      throw new Error(`Unsupported thread mode: ${value}`);
  }
}

function sanitizeSessionId(value) {
  if (typeof value !== 'string' || value.length === 0) {
    return undefined;
  }
  return value.replace(/[^A-Za-z0-9._-]/gu, '_');
}

function threadModePath(sessionId) {
  return compatibilityStatePath('thread-modes', `${safeFileToken(lane)}--${safeFileToken(sessionId)}.json`);
}

function writeGrantPath(sessionId) {
  return compatibilityStatePath('write-grants', `${safeFileToken(lane)}--${safeFileToken(sessionId)}.json`);
}

function lastHookSessionPath() {
  return compatibilityStatePath('hook-sessions', `${safeFileToken(lane)}--last-hook.json`);
}

function latestUserPromptSessionPath() {
  return compatibilityStatePath('hook-sessions', `${safeFileToken(lane)}--last-user-prompt.json`);
}

function compatibilityStatePath(...segments) {
  return join(compatibilityStateRoot(), 'compat', 'codex-thread-mode', ...segments);
}

function compatibilityStateRoot() {
  if (cachedCompatibilityStateRoot !== undefined) {
    return cachedCompatibilityStateRoot;
  }

  const explicitRoot = process.env.LEDGER_ROOT;
  if (typeof explicitRoot === 'string' && explicitRoot.trim().length > 0) {
    cachedCompatibilityStateRoot = resolve(explicitRoot.trim());
    return cachedCompatibilityStateRoot;
  }

  const rootInfo = runLedgerJson(['root']);
  if (typeof rootInfo?.root !== 'string' || rootInfo.root.length === 0) {
    throw new Error('Ledger root did not return a usable path.');
  }
  cachedCompatibilityStateRoot = resolve(rootInfo.root);
  return cachedCompatibilityStateRoot;
}

function readCompatibilityJson(path) {
  if (!existsSync(path)) {
    return undefined;
  }
  try {
    return JSON.parse(readFileSync(path, 'utf8'));
  } catch {
    return undefined;
  }
}

function writeCompatibilityJson(path, value) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function removeCompatibilityJson(path) {
  rmSync(path, { force: true });
}

function readWriteGrant(sessionId) {
  return readCompatibilityJson(writeGrantPath(sessionId));
}

function grantWriteAccessForUserPrompt(sessionId, laneLeaseSessionId) {
  const record = {
    lane,
    sessionId,
    source: 'user-prompt',
    grantedBySessionId: sessionId,
    laneLeaseSessionId: sanitizeSessionId(laneLeaseSessionId) ?? null,
    updatedAt: new Date().toISOString(),
  };
  writeCompatibilityJson(writeGrantPath(sessionId), record);
  return record;
}

function listWriteGrantSessions() {
  const directory = compatibilityStatePath('write-grants');
  if (!existsSync(directory)) {
    return [];
  }

  return readdirSync(directory)
    .filter((entry) => entry.startsWith(`${safeFileToken(lane)}--`) && entry.endsWith('.json'))
    .map((entry) => readCompatibilityJson(join(directory, entry)))
    .map((record) => sanitizeSessionId(record?.sessionId))
    .filter((sessionId) => sessionId !== undefined)
    .sort();
}

function requiredSessionIdOption() {
  const sessionId = sanitizeSessionId(required('session-id'));
  if (sessionId === undefined) {
    throw new Error('Missing required --session-id option.');
  }
  return sessionId;
}

function safeFileToken(value) {
  return value.replace(/[^A-Za-z0-9._-]/gu, '_');
}

function messageBody() {
  const subject = options.subject ?? 'Ledger message';
  const body = options.body ?? positionalText();
  return `${subject}\n\n${body}`.trim();
}

function reportBody() {
  const summary = options.summary ?? positionalText();
  const details = options.details;
  return details === undefined ? summary : `${summary}\n\n${details}`;
}

function claimLedgerSession(sessionId, eventName) {
  return tryRunLedgerJson([
    'session',
    'claim',
    lane,
    sessionId,
    '--ttl-seconds',
    '7200',
    '--summary',
    `${eventName} hook active`,
  ]);
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
