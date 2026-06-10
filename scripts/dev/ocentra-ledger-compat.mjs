#!/usr/bin/env node
import { execFileSync, spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

const repoRoot = git(['rev-parse', '--show-toplevel']);
const ledgerWrapper = join(repoRoot, 'scripts', 'dev', 'ocentra-ledger.mjs');
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
      runLedger(['report', '--lane', lane, reportBody()]);
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
  const eventName = typeof input.hook_event_name === 'string' ? input.hook_event_name : 'hook';
  const session = claimHookSession(input, eventName);
  const context = [
    'Ocentra Ledger coordination context:',
    `- Hook event: ${eventName}.`,
    `- Current lane: ${lane}. Set LEDGER_LANE to override lane identity for this checkout.`,
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
        hookEventName: normalizeHookEvent(eventName),
      },
    })
  );
}

function claimHookSession(input, eventName) {
  const rawSessionId = input.session_id ?? input.sessionId ?? process.env.CODEX_SESSION_ID;
  const sessionId =
    typeof rawSessionId === 'string' && rawSessionId.length > 0
      ? rawSessionId.replace(/[^A-Za-z0-9._-]/gu, '_')
      : undefined;
  if (lane === 'primary' || sessionId === undefined) {
    return { context: [] };
  }

  const result = tryRunLedgerJson([
    'session',
    'claim',
    lane,
    sessionId,
    '--ttl-seconds',
    '7200',
    '--summary',
    `${eventName} hook active`,
  ]);
  if (result.ok) {
    return {
      context: [
        `- Active Codex session lease: ${sessionId}. This thread owns ${lane} until another explicit session takes over or the lease expires.`,
      ],
    };
  }
  const active = result.value?.activeSession;
  const activeSessionId = typeof active?.sessionId === 'string' ? active.sessionId : 'another session';
  return {
    context: [
      `- READ-ONLY: ${lane} is already owned by active Codex session ${activeSessionId}. You may answer questions and inspect status, but do not ack mail, edit files, claim paths, heartbeat, or report work from this thread unless the user explicitly retargets this lane.`,
    ],
  };
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
