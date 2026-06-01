import { spawn } from 'node:child_process';
import { setTimeout as delay } from 'node:timers/promises';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevPort,
  ParentDevValue,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  isLikelyParentAgentOccupant,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import { resolveDebugAgentServicePath } from './agent-service-process.mjs';

const port = ParentDevPort.LanWebSocketSmokeAgent;
const allowedOrigin = createHttpOrigin(ParentDevHost.Loopback);
const healthUrl = createAgentHealthUrl(port);
const wsUrl = createAgentWebSocketUrl(port);
const childDeviceId = 'child-device-integration-lan';
const parentDeviceId = 'parent-device-integration-lan';
const pairingId = 'pairing-integration-lan';
const proofDigest = 'sha256:integration-lan-proof';
const routeId = 'route-integration-lan';
const issuedAt = '2026-05-23T14:40:00.000Z';
const expiresAt = '2099-05-23T14:45:00.000Z';
const controllerLeaseId = 'controller-lease-integration-lan';
const controllerLeaseExpiresAt = '2099-05-23T15:45:00.000Z';
const parentActorId = 'parent-actor-integration-lan';
const parentAuthority = 'active-controller';

await ensurePortFree(port, isLikelyParentAgentOccupant, console.log, ParentDevHost.Wildcard);

const service = spawn(resolveDebugAgentServicePath(), [], {
  cwd: process.cwd(),
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: createAgentAddress(port, ParentDevHost.Wildcard),
    [ParentDevEnv.AgentAllowedOrigins]: allowedOrigin,
    [ParentDevEnv.AgentLocalNetworkEnabled]: ParentDevValue.True,
    OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID: childDeviceId,
  },
  stdio: ['ignore', 'pipe', 'pipe'],
});

const serviceOutput = collectOutput(service);

try {
  await waitForHttp(healthUrl);
  await assertCorsOrigin();
  const received = await runWebSocketSmoke();
  if (!received.includes('agent.health.reported')) {
    throw new Error(`Expected LAN health event, received ${received.join(',')}`);
  }
  console.log(`websocket-lan-smoke-ok:${received.join(',')}`);
} finally {
  stopProcess(service);
}

async function assertCorsOrigin() {
  const response = await fetch(healthUrl, { headers: { Origin: allowedOrigin } });
  const returnedOrigin = response.headers.get('access-control-allow-origin');
  if (returnedOrigin !== allowedOrigin) {
    throw new Error(`Expected LAN CORS origin ${allowedOrigin}, received ${returnedOrigin}`);
  }
}

async function waitForHttp(url) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < 30000) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}\n${serviceOutput()}`);
}

function runWebSocketSmoke() {
  const events = [];
  return withTimeout(
    runLanWebSocketSmoke(events),
    45000,
    () => `LAN WebSocket smoke timed out after events=${events.join(',') || '<none>'}`
  );
}

async function runLanWebSocketSmoke(events) {
  const pairing = await sendLanCommand(buildPairingCommand(), events);
  if (pairing.event !== 'agent.lan-pairing.status.reported') {
    throw new Error(`Expected LAN pairing status after proof, received ${pairing.event}`);
  }
  assertLanSupportSurface(pairing.payload);

  const routeSelection = await sendLanCommand(buildRouteSelectCommand(), events);
  if (routeSelection.event !== 'agent.lan-pairing.status.reported') {
    throw new Error(`Expected LAN route selection status, received ${routeSelection.event}`);
  }
  assertLanSupportSurface(routeSelection.payload);

  const pairedHealth = await sendLanCommand(buildPairedHealthCommand(), events);
  if (pairedHealth.event !== 'agent.health.reported') {
    throw new Error(`Expected paired LAN health report, received ${pairedHealth.event}`);
  }
  assertPayloadValue(pairedHealth.payload, 'intentKind', 'rule-query');
  assertPairedControlAccepted(pairedHealth.payload);

  const anonymous = await sendLanCommand(buildUnpairedHealthCommand(), events);
  if (anonymous.event !== 'agent.command.rejected') {
    throw new Error(`Expected anonymous LAN command rejection, received ${anonymous.event}`);
  }
  assertUnpairedControlRejected(anonymous.payload);

  return events;
}

function sendLanCommand(command, events) {
  return withTimeout(
    new Promise((resolve, reject) => {
      const socket = new WebSocket(wsUrl, { headers: { Origin: allowedOrigin } });
      let result;
      let settled = false;

      socket.addEventListener('open', () => {
        socket.send(JSON.stringify(command));
      });

      socket.addEventListener('message', (message) => {
        let parsed;
        try {
          parsed = AgentEventEnvelopeSchema.parse(JSON.parse(String(message.data)));
        } catch (error) {
          if (!settled) {
            settled = true;
            socket.close();
            reject(error);
          }
          return;
        }
        events.push(parsed.event);
        if (parsed.event === 'agent.connection.ready') {
          return;
        }
        result = parsed;
        socket.close();
      });

      socket.addEventListener('close', () => {
        if (settled) {
          return;
        }
        settled = true;
        if (result !== undefined) {
          resolve(result);
          return;
        }
        reject(new Error(`LAN WebSocket ${command.command} closed before a command response`));
      });

      socket.addEventListener('error', () => {
        if (settled) {
          return;
        }
        settled = true;
        reject(new Error(`LAN WebSocket ${command.command} failed`));
      });
    }),
    15000,
    () => `LAN WebSocket ${command.command} timed out after events=${events.join(',') || '<none>'}`
  );
}

function assertUnpairedControlRejected(payload) {
  assertPayloadValue(payload, 'controlState', 'rejected');
  assertPayloadValue(payload, 'auditEventType', 'control-rejected');
  assertPayloadValue(payload, 'authenticationState', 'unauthenticated');
  assertPayloadValue(payload, 'rejectionReason', 'anonymous');
}

function assertPairedControlAccepted(payload) {
  assertPayloadValue(payload, 'controlState', 'accepted');
  assertPayloadValue(payload, 'auditEventType', 'control-accepted');
  assertPayloadValue(payload, 'authenticationState', 'paired');
  assertPayloadValue(payload, 'evidenceReferenceCount', 1);
  assertPayloadValue(payload, 'evidenceReferenceIds', 'activity-event-lan-control-1');
  assertPayloadValue(payload, 'controllerLeaseId', controllerLeaseId);
  assertPayloadValue(payload, 'controllerDeviceId', parentDeviceId);
  assertPayloadValue(payload, 'parentActorId', parentActorId);
}

function assertLanSupportSurface(payload) {
  assertPayloadValue(payload, 'transport', 'websocket');
  assertPayloadValue(
    payload,
    'supportedWebSocketCommands',
    'agent.lan-pairing.proof.submit,agent.lan-pairing.route.select,agent.lan-pairing.route.revoke,agent.lan-pairing.status.get,agent.lan-pairing.browser-discovery.scan,agent.lan-pairing.add-device.request,agent.lan-pairing.controller-lease.renew,agent.lan-pairing.controller-lease.release,agent.lan-pairing.controller-lease.takeover,agent.lan-ai.provider.status.get,agent.lan-ai.job.submit'
  );
  assertPayloadValue(
    payload,
    'unsupportedHttpEndpoints',
    '/api/lan-pairing/discovery,/api/lan-pairing/challenge,/api/lan-pairing/proof,/api/lan-pairing/control,/api/lan-pairing/registry'
  );
  assertPayloadValue(payload, 'discoveryStatus', 'websocket-direct');
  assertPayloadValue(payload, 'challengeStatus', 'websocket-direct');
  assertPayloadValue(payload, 'proofPreviewStatus', 'websocket-direct');
  assertPayloadValue(payload, 'lanAiProviderStatus', 'websocket-direct');
  assertPayloadValue(payload, 'lanAiJobStatus', 'websocket-direct');
  assertPayloadValue(payload, 'persistenceMode', 'in-memory-fail-closed');
  assertPayloadValue(payload, 'proofMode', 'direct-proof-submit');
  assertPayloadValue(
    payload,
    'routeRequirements',
    'paired-device,allowed-origin,target-device-match,route-id-match,unexpired-intent,non-replayed-intent,unrevoked-pairing,active-controller-lease,selected-device-reachable,parent-write-authority,lan-ai-job-authorized,discovery-state-explicit,route-recovery-persisted'
  );
  assertPayloadValue(
    payload,
    'manualProofGaps',
    'manual-lan-bind-proof,manual-firewall-proof,manual-physical-device-proof'
  );
}

function assertPayloadValue(payload, key, expected) {
  if (payload[key] !== expected) {
    throw new Error(`Expected LAN payload ${key}=${expected}, received ${payload[key]}`);
  }
}

function buildPairingCommand() {
  return buildCommand('cmd-integration-lan-pairing', 'agent.lan-pairing.proof.submit', {
    pairingId,
    challengeId: 'challenge-integration-lan',
    childDeviceId,
    parentDeviceId,
    routeId,
    origin: allowedOrigin,
    proofDigest,
    evidenceReferenceIds: 'activity-event-lan-control-1',
    startedAt: issuedAt,
    staleAt: expiresAt,
  });
}

function buildUnpairedHealthCommand() {
  return buildCommand('cmd-integration-lan-unpaired-health', 'agent.health.check', {});
}

function buildPairedHealthCommand() {
  return buildCommand('cmd-integration-lan-health', 'agent.health.check', {
    intentId: 'intent-integration-lan-health',
    intentKind: 'rule-query',
    pairingId,
    childDeviceId,
    routeId,
    origin: allowedOrigin,
    proofDigest,
    evidenceReferenceIds: 'activity-event-lan-control-1',
    startedAt: issuedAt,
    staleAt: expiresAt,
    controllerLeaseId,
    controllerDeviceId: parentDeviceId,
    parentActorId,
    parentAuthority,
    controllerLeaseIssuedAt: issuedAt,
    controllerLeaseExpiresAt,
  });
}

function buildRouteSelectCommand() {
  return buildCommand('cmd-integration-lan-route-select', 'agent.lan-pairing.route.select', {
    intentId: 'intent-integration-lan-route-select',
    intentKind: 'configuration-update',
    pairingId,
    childDeviceId,
    routeId,
    origin: allowedOrigin,
    proofDigest,
    startedAt: issuedAt,
    staleAt: expiresAt,
    controllerLeaseId,
    controllerDeviceId: parentDeviceId,
    parentActorId,
    parentAuthority,
    controllerLeaseIssuedAt: issuedAt,
    controllerLeaseExpiresAt,
  });
}

function buildCommand(messageId, command, payload) {
  return {
    schemaVersion: 1,
    messageId,
    sentAt: new Date().toISOString(),
    source: { peerId: 'portal-dev', role: 'portal' },
    target: { deviceId: childDeviceId, platform: 'windows', route: 'local-network' },
    command,
    payload,
  };
}

function stopProcess(child) {
  if (process.platform === 'win32' && child.pid !== undefined) {
    spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore' });
    return;
  }
  child.kill();
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}

function withTimeout(promise, timeoutMs, message) {
  let timer;
  return Promise.race([
    promise,
    new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(typeof message === 'function' ? message() : message)), timeoutMs);
    }),
  ]).finally(() => clearTimeout(timer));
}
