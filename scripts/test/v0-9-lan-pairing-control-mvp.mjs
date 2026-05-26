import { spawn } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { AgentEventEnvelopeSchema } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevValue,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  isLikelyParentAgentOccupant,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';
import { resolveDebugAgentServicePath, stopProcessTreeAndWait } from './agent-service-process.mjs';

const outputDir = join(process.cwd(), 'test-results', 'v0-9-lan-pairing-control-mvp');
const evidencePath = join(outputDir, 'proof.json');
const allowedOrigin = createHttpOrigin(ParentDevHost.Loopback);
const wrongOrigin = createHttpOrigin(ParentDevHost.Loopback, 9478);
const issuedAt = '2026-05-26T18:20:00.000Z';
const expiresAt = '2099-05-26T18:25:00.000Z';
const platform = 'windows';
const webSocketEventTimeoutMs = 20000;

const agents = [
  {
    label: 'first-child-agent',
    port: 4492,
    childDeviceId: 'child-device-v09-first',
    pairingId: 'pairing-v09-first',
    challengeId: 'challenge-v09-first',
    proofDigest: 'sha256:v09-first-proof',
    routeId: 'route-v09-first-local-network',
    evidenceReferenceIds: 'activity-event-v09-first',
  },
  {
    label: 'second-child-agent',
    port: 4493,
    childDeviceId: 'child-device-v09-second',
    pairingId: 'pairing-v09-second',
    challengeId: 'challenge-v09-second',
    proofDigest: 'sha256:v09-second-proof',
    routeId: 'route-v09-second-local-network',
    evidenceReferenceIds: 'activity-event-v09-second',
  },
];

await rm(outputDir, { recursive: true, force: true });
await mkdir(outputDir, { recursive: true });

for (const agent of agents) {
  await ensurePortFree(agent.port, isLikelyParentAgentOccupant, console.log, ParentDevHost.Wildcard);
}

const services = agents.map(spawnAgentService);
const assertions = [];

try {
  await Promise.all(services.map((service) => waitForHttp(service.healthUrl, service)));
  await assertWrongOriginWebSocketRejected(services[0]);
  assertions.push('wrong-origin-websocket-rejected-before-upgrade');

  const firstLifecycle = await runLanLifecycle(services[0], { revokeAtEnd: true });
  const secondLifecycle = await runLanLifecycle(services[1], { revokeAtEnd: false });
  assertions.push(...firstLifecycle, ...secondLifecycle);

  await assertWrongAgentPortRejected(services[0], services[1]);
  assertions.push('wrong-agent-port-rejected-as-wrong-device');

  await stopProcessTreeAndWait(services[1].child);
  services[1] = spawnAgentService(agents[1]);
  await waitForHttp(services[1].healthUrl, services[1]);
  const restartLifecycle = await runPersistentRestartLifecycle(services[1]);
  assertions.push(...restartLifecycle);

  await writeEvidence(assertions, services);
  console.log(`v0-9-lan-pairing-control-mvp-ok:${assertions.join(',')}`);
} finally {
  await Promise.allSettled(services.map((service) => stopProcessTreeAndWait(service.child)));
}

function spawnAgentService(agent) {
  const registryPath = join(outputDir, `${agent.label}-registry.json`);
  const service = spawn(resolveDebugAgentServicePath(), [], {
    cwd: process.cwd(),
    env: {
      ...process.env,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agent.port, ParentDevHost.Wildcard),
      [ParentDevEnv.AgentAllowedOrigins]: allowedOrigin,
      [ParentDevEnv.AgentLocalNetworkEnabled]: ParentDevValue.True,
      OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID: agent.childDeviceId,
      OCENTRA_PARENT_AGENT_LAN_PAIRING_REGISTRY_PATH: registryPath,
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });

  return {
    ...agent,
    child: service,
    healthUrl: createAgentHealthUrl(agent.port),
    registryPath,
    serviceOutput: collectOutput(service),
    wsUrl: createAgentWebSocketUrl(agent.port),
  };
}

async function runLanLifecycle(service, { revokeAtEnd }) {
  const socket = await openWebSocket(service, allowedOrigin);
  try {
    const labels = [];
    const unpaired = await sendCommand(socket, buildHealthCommand(service, 'unpaired-health', {}));
    assertEvent(unpaired, 'agent.command.rejected');
    assertPayloadValue(unpaired.payload, 'rejectionReason', 'anonymous');
    labels.push(`${service.label}:anonymous-rejected`);

    const paired = await sendCommand(socket, buildPairingCommand(service));
    assertEvent(paired, 'agent.lan-pairing.status.reported');
    assertPayloadValue(paired.payload, 'auditEventType', 'pairing-proof-accepted');
    assertPayloadValue(paired.payload, 'trustedDeviceIds', service.childDeviceId);
    assertPayloadValue(paired.payload, 'selectedChildDeviceId', '');
    assertLanSupportSurface(paired.payload);
    labels.push(`${service.label}:pairing-proof-accepted-unselected`);

    const beforeSelection = await sendCommand(
      socket,
      buildHealthCommand(
        service,
        'before-selection-health',
        intentPayload(service, 'intent-before-selection', 'rule-query')
      )
    );
    assertEvent(beforeSelection, 'agent.command.rejected');
    assertPayloadValue(beforeSelection.payload, 'rejectionReason', 'unselected-device');
    labels.push(`${service.label}:unselected-control-rejected`);

    const selected = await sendCommand(socket, buildRouteSelectCommand(service, 'intent-route-select'));
    assertEvent(selected, 'agent.lan-pairing.status.reported');
    assertPayloadValue(selected.payload, 'auditEventType', 'route-selected');
    assertPayloadValue(selected.payload, 'authenticationState', 'paired');
    assertPayloadValue(selected.payload, 'selectedChildDeviceId', service.childDeviceId);
    assertPayloadValue(selected.payload, 'selectedRouteId', service.routeId);
    labels.push(`${service.label}:route-selected`);

    const accepted = await sendCommand(
      socket,
      buildHealthCommand(
        service,
        'accepted-rule-query',
        intentPayload(service, 'intent-accepted-rule-query', 'rule-query')
      )
    );
    assertEvent(accepted, 'agent.health.reported');
    assertAcceptedControl(accepted.payload, 'rule-query', service);
    labels.push(`${service.label}:rule-query-accepted`);

    const replayed = await sendCommand(
      socket,
      buildHealthCommand(
        service,
        'replayed-rule-query',
        intentPayload(service, 'intent-accepted-rule-query', 'rule-query')
      )
    );
    assertEvent(replayed, 'agent.command.rejected');
    assertPayloadValue(replayed.payload, 'rejectionReason', 'replayed');
    labels.push(`${service.label}:replay-rejected`);

    if (revokeAtEnd) {
      const revoked = await sendCommand(socket, buildRouteRevokeCommand(service, 'intent-route-revoke'));
      assertEvent(revoked, 'agent.lan-pairing.status.reported');
      assertPayloadValue(revoked.payload, 'auditEventType', 'pairing-revoked');
      assertPayloadValue(revoked.payload, 'pairingState', 'revoked');
      labels.push(`${service.label}:route-revoked`);

      const afterRevoke = await sendCommand(
        socket,
        buildHealthCommand(service, 'after-revoke-health', intentPayload(service, 'intent-after-revoke', 'rule-update'))
      );
      assertEvent(afterRevoke, 'agent.command.rejected');
      assertPayloadValue(afterRevoke.payload, 'rejectionReason', 'revoked');
      labels.push(`${service.label}:revoked-control-rejected`);
    }

    return labels;
  } finally {
    socket.close();
  }
}

async function runPersistentRestartLifecycle(service) {
  const socket = await openWebSocket(service, allowedOrigin);
  try {
    const labels = [];
    const restartStatus = await sendCommand(socket, buildLoopbackStatusCommand(service, 'restart-status'));
    assertEvent(restartStatus, 'agent.lan-pairing.status.reported');
    assertPayloadValue(restartStatus.payload, 'pairingState', 'paired');
    assertPayloadValue(restartStatus.payload, 'authenticationState', 'unpaired');
    assertPayloadValue(restartStatus.payload, 'trustedDeviceIds', service.childDeviceId);
    assertPayloadValue(restartStatus.payload, 'selectedChildDeviceId', '');
    assertPayloadValue(restartStatus.payload, 'persistenceMode', 'local-json-registry');
    assertPayloadValue(restartStatus.payload, 'restartBehavior', 'restore-trusted-registry-unselected');
    labels.push(`${service.label}:restart-restores-trusted-unselected`);

    const unselectedAfterRestart = await sendCommand(
      socket,
      buildHealthCommand(
        service,
        'restart-unselected-health',
        intentPayload(service, 'intent-after-restart-unselected', 'approval-decision')
      )
    );
    assertEvent(unselectedAfterRestart, 'agent.command.rejected');
    assertPayloadValue(unselectedAfterRestart.payload, 'rejectionReason', 'unselected-device');
    labels.push(`${service.label}:restart-unselected-control-rejected`);

    const selectedAfterRestart = await sendCommand(
      socket,
      buildRouteSelectCommand(service, 'intent-route-select-after-restart')
    );
    assertEvent(selectedAfterRestart, 'agent.lan-pairing.status.reported');
    assertPayloadValue(selectedAfterRestart.payload, 'selectedChildDeviceId', service.childDeviceId);
    labels.push(`${service.label}:restart-route-reselected`);

    const acceptedAfterRestart = await sendCommand(
      socket,
      buildHealthCommand(
        service,
        'restart-accepted-approval',
        intentPayload(service, 'intent-after-restart-approval', 'approval-decision')
      )
    );
    assertEvent(acceptedAfterRestart, 'agent.health.reported');
    assertAcceptedControl(acceptedAfterRestart.payload, 'approval-decision', service);
    labels.push(`${service.label}:restart-approval-accepted`);

    return labels;
  } finally {
    socket.close();
  }
}

async function assertWrongAgentPortRejected(firstService, secondService) {
  const socket = await openWebSocket(firstService, allowedOrigin);
  try {
    const wrongPort = await sendCommand(
      socket,
      buildHealthCommand(
        secondService,
        'wrong-agent-port-health',
        intentPayload(secondService, 'intent-wrong-agent-port', 'health-query')
      )
    );
    assertEvent(wrongPort, 'agent.command.rejected');
    assertPayloadValue(wrongPort.payload, 'rejectionReason', 'wrong-device');
  } finally {
    socket.close();
  }
}

async function assertWrongOriginWebSocketRejected(service) {
  await new Promise((resolve, reject) => {
    const socket = new WebSocket(service.wsUrl, { headers: { Origin: wrongOrigin } });
    const timer = setTimeout(() => {
      socket.close();
      reject(new Error(`${service.label} accepted wrong origin longer than expected`));
    }, 5000);

    socket.addEventListener('open', () => {
      clearTimeout(timer);
      socket.close();
      reject(new Error(`${service.label} unexpectedly opened with wrong origin`));
    });
    socket.addEventListener('error', () => {
      clearTimeout(timer);
      resolve();
    });
    socket.addEventListener('close', () => {
      clearTimeout(timer);
      resolve();
    });
  });
}

async function openWebSocket(service, origin) {
  const socket = new WebSocket(service.wsUrl, { headers: { Origin: origin } });
  await new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      socket.close();
      reject(new Error(`${service.label} WebSocket open timed out`));
    }, webSocketEventTimeoutMs);
    socket.addEventListener('open', () => {
      clearTimeout(timer);
      resolve();
    });
    socket.addEventListener('error', () => {
      clearTimeout(timer);
      reject(new Error(`${service.label} WebSocket failed to open`));
    });
  });
  return socket;
}

async function sendCommand(socket, command) {
  socket.send(JSON.stringify(command));
  for (;;) {
    const event = await nextEvent(socket, command.messageId);
    if (event.event !== 'agent.connection.ready') {
      return event;
    }
  }
}

function nextEvent(socket, messageId) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      socket.removeEventListener('message', onMessage);
      reject(new Error(`Timed out waiting for LAN pairing event after ${messageId}`));
    }, webSocketEventTimeoutMs);
    const onMessage = (message) => {
      clearTimeout(timer);
      try {
        resolve(AgentEventEnvelopeSchema.parse(JSON.parse(String(message.data))));
      } catch (error) {
        reject(error);
      }
    };
    socket.addEventListener('message', onMessage, { once: true });
  });
}

async function waitForHttp(url, service) {
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
  throw new Error(`Timed out waiting for ${url}\n${service.serviceOutput()}`);
}

function buildPairingCommand(service) {
  return buildCommand(service, 'pairing-proof-submit', 'agent.lan-pairing.proof.submit', {
    pairingId: service.pairingId,
    challengeId: service.challengeId,
    childDeviceId: service.childDeviceId,
    parentDeviceId: 'parent-device-v09',
    routeId: service.routeId,
    origin: allowedOrigin,
    proofDigest: service.proofDigest,
    evidenceReferenceIds: service.evidenceReferenceIds,
    startedAt: issuedAt,
    staleAt: expiresAt,
  });
}

function buildRouteSelectCommand(service, intentId) {
  return buildCommand(
    service,
    intentId,
    'agent.lan-pairing.route.select',
    intentPayload(service, intentId, 'configuration-update')
  );
}

function buildRouteRevokeCommand(service, intentId) {
  return buildCommand(
    service,
    intentId,
    'agent.lan-pairing.route.revoke',
    intentPayload(service, intentId, 'configuration-update')
  );
}

function buildHealthCommand(service, messageSuffix, payload) {
  return buildCommand(service, messageSuffix, 'agent.health.check', payload);
}

function buildLoopbackStatusCommand(service, messageSuffix) {
  return {
    ...buildCommand(service, messageSuffix, 'agent.lan-pairing.status.get', {}),
    target: { deviceId: service.childDeviceId, platform, route: 'localhost' },
  };
}

function buildCommand(service, messageSuffix, command, payload) {
  return {
    schemaVersion: 1,
    messageId: `${service.label}-${messageSuffix}`,
    sentAt: new Date().toISOString(),
    source: { peerId: 'portal-dev', role: 'portal' },
    target: { deviceId: service.childDeviceId, platform, route: 'local-network' },
    command,
    payload,
  };
}

function intentPayload(service, intentId, intentKind) {
  return {
    intentId,
    intentKind,
    pairingId: service.pairingId,
    childDeviceId: service.childDeviceId,
    routeId: service.routeId,
    origin: allowedOrigin,
    proofDigest: service.proofDigest,
    evidenceReferenceIds: service.evidenceReferenceIds,
    startedAt: issuedAt,
    staleAt: expiresAt,
  };
}

function assertLanSupportSurface(payload) {
  assertPayloadValue(payload, 'transport', 'websocket');
  assertPayloadValue(
    payload,
    'supportedWebSocketCommands',
    'agent.lan-pairing.proof.submit,agent.lan-pairing.route.select,agent.lan-pairing.route.revoke,agent.lan-pairing.status.get'
  );
  assertPayloadValue(payload, 'discoveryStatus', 'planned-unsupported');
  assertPayloadValue(payload, 'challengeStatus', 'planned-unsupported');
  assertPayloadValue(payload, 'proofPreviewStatus', 'planned-unsupported');
  assertPayloadValue(payload, 'persistenceMode', 'local-json-registry');
  assertPayloadValue(payload, 'proofMode', 'direct-proof-submit');
}

function assertAcceptedControl(payload, intentKind, service) {
  assertPayloadValue(payload, 'controlState', 'accepted');
  assertPayloadValue(payload, 'auditEventType', 'control-accepted');
  assertPayloadValue(payload, 'authenticationState', 'paired');
  assertPayloadValue(payload, 'intentKind', intentKind);
  assertPayloadValue(payload, 'routeId', service.routeId);
  assertPayloadValue(payload, 'evidenceReferenceIds', service.evidenceReferenceIds);
}

function assertEvent(event, expected) {
  if (event.event !== expected) {
    throw new Error(`Expected event ${expected}, received ${event.event}`);
  }
}

function assertPayloadValue(payload, key, expected) {
  if (payload[key] !== expected) {
    throw new Error(`Expected LAN payload ${key}=${expected}, received ${payload[key]}`);
  }
}

async function writeEvidence(assertions, services) {
  await writeFile(
    evidencePath,
    `${JSON.stringify(
      {
        checkedAt: new Date().toISOString(),
        allowedOrigin,
        assertions,
        services: services.map((service) => ({
          label: service.label,
          port: service.port,
          childDeviceId: service.childDeviceId,
          registryPath: service.registryPath,
        })),
      },
      null,
      2
    )}\n`
  );
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}
