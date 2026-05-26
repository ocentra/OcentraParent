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
  return new Promise((resolve, reject) => {
    const events = [];
    let unpairedRejected = false;
    let routeSelected = false;
    const socket = new WebSocket(wsUrl, { headers: { Origin: allowedOrigin } });
    const timer = setTimeout(() => {
      socket.close();
      reject(new Error('LAN WebSocket smoke timed out'));
    }, 10000);

    socket.addEventListener('open', () => {
      socket.send(JSON.stringify(buildUnpairedHealthCommand()));
    });

    socket.addEventListener('message', (message) => {
      const parsed = AgentEventEnvelopeSchema.parse(JSON.parse(String(message.data)));
      events.push(parsed.event);
      if (parsed.event === 'agent.command.rejected') {
        if (!unpairedRejected) {
          assertUnpairedControlRejected(parsed.payload);
          unpairedRejected = true;
          socket.send(JSON.stringify(buildPairingCommand()));
          return;
        }
        clearTimeout(timer);
        socket.close();
        reject(new Error(`LAN WebSocket smoke rejected command: ${JSON.stringify(parsed.payload)}`));
        return;
      }
      if (parsed.event === 'agent.lan-pairing.status.reported') {
        assertLanSupportSurface(parsed.payload);
        if (!routeSelected) {
          routeSelected = true;
          socket.send(JSON.stringify(buildRouteSelectCommand()));
          return;
        }
        socket.send(JSON.stringify(buildPairedHealthCommand()));
        return;
      }
      if (parsed.event === 'agent.health.reported') {
        assertPayloadValue(parsed.payload, 'intentKind', 'rule-query');
        assertPairedControlAccepted(parsed.payload);
        clearTimeout(timer);
        socket.close();
        resolve(events);
      }
    });

    socket.addEventListener('error', () => {
      clearTimeout(timer);
      reject(new Error('LAN WebSocket smoke failed'));
    });
  });
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
}

function assertLanSupportSurface(payload) {
  assertPayloadValue(payload, 'transport', 'websocket');
  assertPayloadValue(
    payload,
    'supportedWebSocketCommands',
    'agent.lan-pairing.proof.submit,agent.lan-pairing.route.select,agent.lan-pairing.route.revoke,agent.lan-pairing.status.get'
  );
  assertPayloadValue(
    payload,
    'unsupportedHttpEndpoints',
    '/api/lan-pairing/discovery,/api/lan-pairing/challenge,/api/lan-pairing/proof,/api/lan-pairing/control,/api/lan-pairing/registry'
  );
  assertPayloadValue(payload, 'discoveryStatus', 'planned-unsupported');
  assertPayloadValue(payload, 'challengeStatus', 'planned-unsupported');
  assertPayloadValue(payload, 'proofPreviewStatus', 'planned-unsupported');
  assertPayloadValue(payload, 'persistenceMode', 'in-memory-fail-closed');
  assertPayloadValue(payload, 'proofMode', 'direct-proof-submit');
  assertPayloadValue(
    payload,
    'routeRequirements',
    'paired-device,allowed-origin,target-device-match,route-id-match,unexpired-intent,non-replayed-intent,unrevoked-pairing,selected-device-reachable'
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
