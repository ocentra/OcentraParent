import { spawn } from 'node:child_process';
import { createServer } from 'node:net';
import { join, relative } from 'node:path';
import { mkdir, mkdtemp, readFile, writeFile } from 'node:fs/promises';
import { setTimeout as delay } from 'node:timers/promises';

import { resolveDebugAgentServicePath, stopProcessTreeAndWait } from './agent-service-process.mjs';

const evidenceDirectory = join(process.cwd(), 'test-results', 'v0-8-enforcement-timer-recovery-mvp');
const timeoutMs = envNumber('OCENTRA_PARENT_V08_TIMER_RECOVERY_TIMEOUT_MS', 20_000);

const ids = {
  policyDecisionId: 'decision-v08-timer-recovery',
  actionId: 'action-v08-timer-recovery',
  resultId: 'result-v08-timer-recovery',
  auditEventId: 'audit-v08-timer-recovery',
  timerEventId: 'timer-v08-timer-recovery',
  parentActionReferenceId: 'parent-action-v08-timer-recovery',
};

await main();

async function main() {
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await mkdir(evidenceDirectory, { recursive: true });
  const runRoot = await mkdtemp(join(evidenceDirectory, 'run-'));
  const agentPort = await freePort();
  let service = spawnAgentService(runRoot, agentPort);
  let serviceOutput = collectOutput(service);

  try {
    await waitForHealth(agentPort, serviceOutput);
    const executeEvent = await requestEvent(agentPort, commandEnvelope('execute'));
    const executeAssertion = assertExecuteEvent(executeEvent);

    await stopProcessTreeAndWait(service);
    service = spawnAgentService(runRoot, agentPort);
    serviceOutput = collectOutput(service);
    await waitForHealth(agentPort, serviceOutput);

    const recoverEvent = await requestEvent(agentPort, commandEnvelope('recover'));
    const recoverAssertion = assertRecoverEvent(recoverEvent);
    const cancelEvent = await requestEvent(agentPort, commandEnvelope('cancel'));
    const cancelAssertion = assertCancelEvent(cancelEvent);
    const unavailableEvent = await requestEvent(agentPort, commandEnvelope('recover'));
    const unavailableAssertion = assertUnavailableEvent(unavailableEvent);
    const journalText = await readFile(join(runRoot, 'activity.ndjson'), 'utf8');
    if (journalText.includes(executeAssertion.policyDecisionId)) {
      throw new Error('Encrypted journal contains plaintext timer policy decision id.');
    }

    const evidence = {
      schemaVersion: 1,
      generatedAt: new Date().toISOString(),
      platform: process.platform,
      agentEndpoint: 'loopback-redacted',
      runRoot: relative(process.cwd(), runRoot),
      assertions: {
        execute: executeAssertion,
        recover: recoverAssertion,
        cancel: cancelAssertion,
        unavailable: unavailableAssertion,
      },
      events: {
        execute: eventSummary(executeEvent),
        recover: eventSummary(recoverEvent),
        cancel: eventSummary(cancelEvent),
        unavailable: eventSummary(unavailableEvent),
      },
      artifacts: {
        activityJournal: relative(process.cwd(), join(runRoot, 'activity.ndjson')),
        activityStore: relative(process.cwd(), join(runRoot, 'activity.sqlite')),
        timerStatePath: relative(process.cwd(), join(runRoot, 'enforcement-timers.json')),
        devLogDirectory: relative(process.cwd(), join(runRoot, 'logs')),
      },
    };
    const evidencePath = join(
      evidenceDirectory,
      `${evidence.generatedAt.replaceAll(':', '-').replaceAll('.', '-')}.json`
    );
    await writeFile(evidencePath, `${JSON.stringify(evidence, null, 2)}\n`);
    printSummary(evidencePath, evidence.assertions);
  } finally {
    await stopProcessTreeAndWait(service);
  }
}

function spawnAgentService(runRoot, agentPort) {
  return spawn(resolveDebugAgentServicePath(), [], {
    cwd: process.cwd(),
    env: {
      ...process.env,
      OCENTRA_PARENT_AGENT_ADDR: `127.0.0.1:${agentPort}`,
      OCENTRA_PARENT_ACTIVITY_DB_PATH: join(runRoot, 'activity.sqlite'),
      OCENTRA_PARENT_ACTIVITY_JOURNAL_PATH: join(runRoot, 'activity.ndjson'),
      OCENTRA_PARENT_ACTIVITY_JOURNAL_KEY_PATH: join(runRoot, 'activity.key'),
      OCENTRA_PARENT_AGENT_ENFORCEMENT_TIMER_STATE_PATH: join(runRoot, 'enforcement-timers.json'),
      OCENTRA_PARENT_DEV_LOG_DIR: join(runRoot, 'logs'),
    },
    stdio: ['ignore', 'pipe', 'pipe'],
    windowsHide: true,
  });
}

async function waitForHealth(agentPort, serviceOutput) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(`http://127.0.0.1:${agentPort}/health`);
      if (response.ok) {
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for service health. ${serviceOutput()}`);
}

function requestEvent(agentPort, command) {
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(`ws://127.0.0.1:${agentPort}/api/dev/ws`);
    const timer = setTimeout(() => {
      socket.close();
      reject(new Error(`Timed out waiting for ${command.command}.`));
    }, timeoutMs);

    socket.addEventListener('open', () => {
      socket.send(JSON.stringify(command));
    });
    socket.addEventListener('message', (message) => {
      const event = JSON.parse(String(message.data));
      if (event.event === 'agent.connection.ready') {
        return;
      }
      clearTimeout(timer);
      socket.close();
      resolve(event);
    });
    socket.addEventListener('error', () => {
      clearTimeout(timer);
      reject(new Error(`WebSocket error while requesting ${command.command}.`));
    });
  });
}

function assertExecuteEvent(event) {
  assertEventName(event, 'agent.enforcement.audit.reported');
  assertPayloadValue(event.payload, 'enforcementTimerEventKind', 'created');
  assertPayloadValue(event.payload, 'enforcementStatus', 'no-op');
  assertStored(event.payload);
  return {
    policyDecisionId: ids.policyDecisionId,
    actionId: ids.actionId,
    timerEventKind: event.payload.enforcementTimerEventKind,
    status: event.payload.enforcementStatus,
    statePersisted: typeof event.payload.enforcementTimerState === 'string',
    databaseReady: event.payload.databaseReady,
    eventsStored: event.payload.eventsStored,
  };
}

function assertRecoverEvent(event) {
  assertEventName(event, 'agent.enforcement.timer.reported');
  assertPayloadValue(event.payload, 'enforcementTimerEventKind', 'restart-recovered');
  assertPayloadValue(event.payload, 'enforcementStatus', 'no-op');
  assertStored(event.payload);
  const timer = JSON.parse(event.payload.enforcementTimerEvent);
  if (timer.actionId !== ids.actionId || timer.policyDecisionId !== ids.policyDecisionId) {
    throw new Error(`Recovered timer did not preserve identity: ${JSON.stringify(timer)}`);
  }
  if (timer.recoveredAfterRestart !== true) {
    throw new Error('Recovered timer did not set recoveredAfterRestart=true.');
  }
  return {
    actionId: timer.actionId,
    policyDecisionId: timer.policyDecisionId,
    timerEventKind: timer.timerEventKind,
    recoveredAfterRestart: timer.recoveredAfterRestart,
    statePersisted: typeof event.payload.enforcementTimerState === 'string',
    databaseReady: event.payload.databaseReady,
    eventsStored: event.payload.eventsStored,
  };
}

function assertCancelEvent(event) {
  assertEventName(event, 'agent.enforcement.timer.reported');
  assertPayloadValue(event.payload, 'enforcementTimerEventKind', 'cancelled');
  assertPayloadValue(event.payload, 'enforcementStatus', 'superseded');
  assertStored(event.payload);
  const audit = JSON.parse(event.payload.enforcementAuditEvent);
  if (audit.auditEventKind !== 'cancelled') {
    throw new Error(`Expected cancelled audit, received ${audit.auditEventKind}`);
  }
  if (audit.parentOverride?.actionReferenceId !== ids.parentActionReferenceId) {
    throw new Error(`Expected parent override reference, received ${JSON.stringify(audit.parentOverride)}`);
  }
  if (event.payload.enforcementTimerState !== null) {
    throw new Error('Cancel event should clear persisted active timer state.');
  }
  return {
    actionId: audit.action.actionId,
    auditEventKind: audit.auditEventKind,
    parentOverrideId: audit.parentOverride.actionReferenceId,
    stateCleared: true,
    databaseReady: event.payload.databaseReady,
    eventsStored: event.payload.eventsStored,
  };
}

function assertUnavailableEvent(event) {
  assertEventName(event, 'agent.enforcement.timer.reported');
  assertPayloadValue(event.payload, 'available', false);
  assertPayloadValue(event.payload, 'reason', 'enforcement-active-timer-state-required');
  assertPayloadValue(event.payload, 'enforcementStatus', 'unavailable');
  assertPayloadValue(event.payload, 'enforcementTimerEventKind', 'recovery-needed');
  return {
    available: event.payload.available,
    reason: event.payload.reason,
    status: event.payload.enforcementStatus,
    timerEventKind: event.payload.enforcementTimerEventKind,
  };
}

function assertStored(payload) {
  if (payload.databaseReady !== true || Number(payload.eventsStored) < 1) {
    throw new Error(`Expected journal/store proof, payload=${JSON.stringify(payload)}`);
  }
}

function assertEventName(event, expected) {
  if (event.event !== expected) {
    throw new Error(`Expected ${expected}, received ${event.event}: ${JSON.stringify(event.payload)}`);
  }
}

function assertPayloadValue(payload, key, expected) {
  if (payload[key] !== expected) {
    throw new Error(`Expected ${key}=${expected}, received ${payload[key]}`);
  }
}

function commandEnvelope(kind) {
  const now = new Date();
  const expiresAt = new Date(now.getTime() + 300_000);
  const base = {
    schemaVersion: 1,
    messageId: `cmd-v08-timer-${kind}`,
    sentAt: now.toISOString(),
    source: { peerId: 'portal-dev', role: 'portal' },
    target: { deviceId: 'local-dev-agent', platform: 'windows', route: 'localhost' },
  };
  if (kind === 'execute') {
    return {
      ...base,
      command: 'agent.enforcement.execute',
      payload: {
        ...commonPayload(now, kind),
        policyAction: 'ask-parent',
        targetType: 'device',
        targetId: 'local-dev-agent',
        targetValue: 'local-dev-agent',
        dryRun: false,
        reasonCodes: 'parent-approval-required',
        ruleIds: 'rule-v08-timer-recovery',
        evidenceReferenceIds: 'evidence-v08-timer-recovery',
        expiresAt: expiresAt.toISOString(),
        enforcementIntentId: 'intent-v08-timer-recovery',
      },
    };
  }
  if (kind === 'cancel') {
    return {
      ...base,
      command: 'agent.enforcement.override.cancel',
      payload: {
        ...commonPayload(now, kind),
        parentActionReferenceId: ids.parentActionReferenceId,
        parentActorId: 'parent-v08-timer-recovery',
        parentActorRole: 'parent',
        parentActionCreatedAt: now.toISOString(),
      },
    };
  }
  return {
    ...base,
    command: 'agent.enforcement.timer.recover',
    payload: commonPayload(now, kind),
  };
}

function commonPayload(now, kind) {
  return {
    policyDecisionId: ids.policyDecisionId,
    policyVersion: 'policy-v08-timer-recovery',
    requestedAt: now.toISOString(),
    enforcementActionId: ids.actionId,
    enforcementResultId: `${ids.resultId}-${kind}`,
    enforcementAuditEventId: `${ids.auditEventId}-${kind}`,
    enforcementTimerEventId: `${ids.timerEventId}-${kind}`,
  };
}

async function freePort() {
  const server = createServer();
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });
  const port = server.address().port;
  await new Promise((resolve) => server.close(resolve));
  return port;
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}

function envNumber(name, fallback) {
  const parsed = Number(process.env[name]);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: process.cwd(), stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} exited with ${code}`));
    });
    child.once('error', reject);
  });
}

function eventSummary(event) {
  return {
    event: event.event,
    severity: event.severity,
    payload: event.payload,
  };
}

function printSummary(evidencePath, assertions) {
  console.log('v0-8-enforcement-timer-recovery-mvp-ok=true');
  console.log(`evidence=${evidencePath}`);
  console.log(
    `execute=${assertions.execute.timerEventKind}/${assertions.execute.status} recover=${assertions.recover.timerEventKind} cancel=${assertions.cancel.auditEventKind} unavailable=${assertions.unavailable.reason}`
  );
}
