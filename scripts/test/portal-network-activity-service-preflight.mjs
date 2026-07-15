import { randomUUID } from 'node:crypto';
import { setTimeout as delay } from 'node:timers/promises';

import { PortalNetworkActivitySeed, seedPortalNetworkActivityStore } from './portal-network-activity-seed.mjs';
import { createPortalSmokeCommandEnvelope } from './websocket-command-envelope.mjs';
import { parseAgentEventEnvelope } from './websocket-event-envelope.mjs';

const preflightTimeoutMs = 30_000;
const requestTimeoutMs = 5_000;
const retryDelayMs = 500;
const networkFlowReadModelCommandName = 'agent.network.flow.read-model.get';
const networkFlowReadModelEventName = 'agent.network.flow.read-model.reported';

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
      reject(new Error(`Timed out waiting for ${networkFlowReadModelEventName}`));
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
          if (event.event !== networkFlowReadModelEventName) {
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
  return createPortalSmokeCommandEnvelope(`cmd-network-flow-${randomUUID()}`, networkFlowReadModelCommandName, {});
}

async function handleMessageData(data) {
  const text = await messageDataText(data);
  return parseAgentEventEnvelope(JSON.parse(text));
}

async function messageDataText(data) {
  if (typeof data === 'string') {
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
    .filter((evidenceId) => typeof evidenceId === 'string');
}

function networkFlowDigest(event) {
  const rawDigest = event.payload.activityDigest;
  if (typeof rawDigest !== 'string') {
    return null;
  }
  return JSON.parse(rawDigest);
}

function describeNetworkFlowEvent(event, evidenceIds) {
  return JSON.stringify({
    event: event.event,
    returned: event.payload.returned,
    activeRows: event.payload.activeRows,
    latestEventId: event.payload.latestEventId,
    latestObservedAt: event.payload.latestObservedAt,
    destinationDomain: event.payload.destinationDomain,
    processName: event.payload.processName,
    evidenceIds,
    seededEvidencePresent: evidenceIds.includes(PortalNetworkActivitySeed.EvidenceId),
    seededRowIsLatest: event.payload.latestEventId === PortalNetworkActivitySeed.EventId,
  });
}

function isSeededNetworkFlowUiPayload(event, evidenceIds) {
  return (
    evidenceIds.includes(PortalNetworkActivitySeed.EvidenceId) &&
    event.payload.latestEventId === PortalNetworkActivitySeed.EventId
  );
}
