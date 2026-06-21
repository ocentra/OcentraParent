import { randomUUID } from 'node:crypto';
import { setTimeout as delay } from 'node:timers/promises';

import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentEventEnvelopeSchema,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';

import { PortalNetworkActivitySeed, seedPortalNetworkActivityStore } from './portal-network-activity-seed.mjs';

const preflightTimeoutMs = 30_000;
const requestTimeoutMs = 5_000;
const retryDelayMs = 500;

export async function assertAgentNetworkActivityReadModel(webSocketUrl, activityDbPath) {
  const startedAt = Date.now();
  let lastSummary = 'no read-model response received';

  while (Date.now() - startedAt < preflightTimeoutMs) {
    seedPortalNetworkActivityStore(activityDbPath);
    try {
      const event = await requestNetworkFlowReadModel(webSocketUrl);
      const evidenceIds = networkFlowEvidenceIds(event);
      if (isSeededNetworkFlowUiPayload(event, evidenceIds)) {
        return event;
      }
      lastSummary = describeNetworkFlowEvent(event, evidenceIds);
    } catch (error) {
      lastSummary = error instanceof Error ? error.message : String(error);
    }
    await delay(retryDelayMs);
  }

  throw new Error(
    [
      'Rust service network read-model did not expose the seeded portal evidence ref before Playwright.',
      `expectedEvidence=${PortalNetworkActivitySeed.EvidenceId}`,
      `lastResponse=${lastSummary}`,
    ].join('\n')
  );
}

export async function describeAgentNetworkActivityReadModel(webSocketUrl) {
  const event = await requestNetworkFlowReadModel(webSocketUrl);
  const evidenceIds = networkFlowEvidenceIds(event);
  return describeNetworkFlowEvent(event, evidenceIds);
}

function requestNetworkFlowReadModel(webSocketUrl) {
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(webSocketUrl);
    const timeout = setTimeout(() => {
      cleanup();
      socket.close();
      reject(new Error(`Timed out waiting for ${AgentEvent.NetworkFlowReadModelReported}`));
    }, requestTimeoutMs);

    const cleanup = () => {
      clearTimeout(timeout);
      socket.removeEventListener('open', onOpen);
      socket.removeEventListener('message', onMessage);
      socket.removeEventListener('error', onError);
    };
    const onOpen = () => {
      socket.send(JSON.stringify(networkFlowReadModelCommand()));
    };
    const onMessage = (message) => {
      void handleMessageData(message.data)
        .then((event) => {
          if (event.event !== AgentEvent.NetworkFlowReadModelReported) {
            return;
          }
          cleanup();
          socket.close();
          resolve(event);
        })
        .catch((error) => {
          cleanup();
          socket.close();
          reject(error);
        });
    };
    const onError = () => {
      cleanup();
      reject(new Error(`WebSocket preflight failed for ${webSocketUrl}`));
    };

    socket.addEventListener('open', onOpen);
    socket.addEventListener('message', onMessage);
    socket.addEventListener('error', onError);
  });
}

function networkFlowReadModelCommand() {
  return AgentCommandEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    messageId: `${AgentProtocolDefaults.MessageIdPrefix}${randomUUID()}`,
    sentAt: new Date().toISOString(),
    source: AgentProtocolDefaults.Peer.PortalDev,
    target: AgentProtocolDefaults.Target.LocalhostWindowsAgent,
    command: AgentCommand.NetworkFlowReadModelGet,
    payload: {},
  });
}

async function handleMessageData(data) {
  const text = await messageDataText(data);
  return AgentEventEnvelopeSchema.parse(JSON.parse(text));
}

async function messageDataText(data) {
  if (typeof data === AgentProtocolDefaults.Primitive.String) {
    return data;
  }
  if (data instanceof ArrayBuffer) {
    return new TextDecoder().decode(data);
  }
  if (ArrayBuffer.isView(data)) {
    return new TextDecoder().decode(data);
  }
  if (typeof data?.text === 'function') {
    return data.text();
  }
  return String(data);
}

function networkFlowEvidenceIds(event) {
  const digest = networkFlowDigest(event);
  if (!Array.isArray(digest?.evidence)) {
    return [];
  }
  return digest.evidence
    .map((reference) => reference?.evidenceId)
    .filter((evidenceId) => typeof evidenceId === AgentProtocolDefaults.Primitive.String);
}

function networkFlowDigest(event) {
  const rawDigest = event.payload[AgentProtocolDefaults.Field.ActivityDigest];
  if (typeof rawDigest !== AgentProtocolDefaults.Primitive.String) {
    return null;
  }
  return JSON.parse(rawDigest);
}

function describeNetworkFlowEvent(event, evidenceIds) {
  return JSON.stringify({
    event: event.event,
    returned: event.payload[AgentProtocolDefaults.Field.Returned],
    activeRows: event.payload[AgentProtocolDefaults.Field.ActiveRows],
    latestEventId: event.payload[AgentProtocolDefaults.Field.LatestEventId],
    latestObservedAt: event.payload[AgentProtocolDefaults.Field.LatestObservedAt],
    destinationDomain: event.payload[AgentProtocolDefaults.Field.DestinationDomain],
    processName: event.payload[AgentProtocolDefaults.Field.ProcessName],
    evidenceIds,
    seededEvidencePresent: evidenceIds.includes(PortalNetworkActivitySeed.EvidenceId),
    seededRowIsLatest: event.payload[AgentProtocolDefaults.Field.LatestEventId] === PortalNetworkActivitySeed.EventId,
  });
}

function isSeededNetworkFlowUiPayload(event, evidenceIds) {
  return (
    evidenceIds.includes(PortalNetworkActivitySeed.EvidenceId) &&
    event.payload[AgentProtocolDefaults.Field.LatestEventId] === PortalNetworkActivitySeed.EventId
  );
}
