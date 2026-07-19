import {
  ParentAgentLanDiscoveryEventKind,
  ParentHostBridgeRuntime,
  type ParentAgentLanDiscoveryEventKind as ParentAgentLanDiscoveryEventKindValue,
  type ParentLanDiscoveryEventHistorySnapshot,
  type ParentLanDiscoveryEventRowSnapshot,
  type ParentRouteEventSnapshot,
  type ParentRouteSnapshot,
} from '../generated/parent-ui-bridge';
import {
  hasRequiredSnapshotEventIdentity,
  isLanReplaySnapshot,
  lanReplayPayloadCandidate,
  parentRouteSnapshotTimestampMs,
  parentRouteTimestampMs,
  type LanReplayPayloadCandidate,
} from './parent-route-event-snapshot';
import { hasCanonicalLanReplayProvenance } from './lan-replay-provenance';

const LAN_DISCOVERY_REPLAY_EVENT_KINDS: ReadonlySet<ParentAgentLanDiscoveryEventKindValue> = new Set(
  Object.values(ParentAgentLanDiscoveryEventKind)
);

export function isReplayBatchBoundToSnapshot(
  snapshots: readonly ParentRouteEventSnapshot[],
  snapshot: ParentRouteSnapshot
): boolean {
  const history = snapshot.liveActivity?.lanAddDeviceReadModel?.discoveryEventHistory;
  const replaySnapshots = snapshots.filter(
    (event) => isLanReplaySnapshot(event, history?.rows ?? []) || isLanDiscoveryReplayCandidate(event)
  );
  if (replaySnapshots.length === 0) {
    return true;
  }
  if (history === undefined || history === null) {
    return false;
  }

  const snapshotTimestamp = parentRouteSnapshotTimestampMs(snapshot);
  const historyGeneratedAt = parentRouteTimestampMs(history.generatedAt);
  if (snapshotTimestamp === null) {
    return false;
  }
  if (historyGeneratedAt === null) {
    return false;
  }
  if (!historyMatchesBatchShape(history, replaySnapshots.length)) {
    return false;
  }
  return replayRowsMatchHistory(replaySnapshots, history, snapshotTimestamp, historyGeneratedAt);
}

function isLanDiscoveryReplayCandidate(event: ParentRouteEventSnapshot): boolean {
  return isGeneratedLanDiscoveryEventKind(event.event);
}

function isGeneratedLanDiscoveryEventKind(value: unknown): value is ParentAgentLanDiscoveryEventKindValue {
  return (
    typeof value === ParentHostBridgeRuntime.StringType &&
    LAN_DISCOVERY_REPLAY_EVENT_KINDS.has(value as ParentAgentLanDiscoveryEventKindValue)
  );
}

function historyMatchesBatchShape(history: ParentLanDiscoveryEventHistorySnapshot, replayCount: number): boolean {
  return [
    history.rows.length === replayCount,
    typeof history.state === ParentHostBridgeRuntime.StringType,
    history.state.length > 0,
  ].every(Boolean);
}

function replayRowsMatchHistory(
  replaySnapshots: readonly ParentRouteEventSnapshot[],
  history: ParentLanDiscoveryEventHistorySnapshot,
  snapshotTimestamp: number,
  historyGeneratedAt: number
): boolean {
  for (const [index, replay] of replaySnapshots.entries()) {
    const row = history.rows[index];
    if (row === undefined) {
      return false;
    }
    if (!replayRowMatchesHistory(replay, row, snapshotTimestamp, historyGeneratedAt)) {
      return false;
    }
  }
  const latestReplay = replaySnapshots.at(-1);
  if (latestReplay === undefined) {
    return false;
  }
  return [
    history.latestEventId === latestReplay.eventId,
    history.latestObservedAt === latestReplay.sentAt,
    parentRouteTimestampMs(history.latestObservedAt) !== null,
  ].every(Boolean);
}

function replayRowMatchesHistory(
  replay: ParentRouteEventSnapshot,
  row: ParentLanDiscoveryEventRowSnapshot,
  snapshotTimestamp: number,
  historyGeneratedAt: number
): boolean {
  if (!hasRequiredSnapshotEventIdentity(replay)) {
    return false;
  }
  const eventTimestamp = parentRouteTimestampMs(replay.sentAt);
  if (eventTimestamp === null) {
    return false;
  }
  const payload = lanReplayPayloadCandidate(replay.payload);
  if (payload === null) {
    return false;
  }
  return [
    eventTimestamp <= snapshotTimestamp,
    eventTimestamp <= historyGeneratedAt,
    hasCanonicalLanReplayProvenance(replay),
    replayPayloadMatchesEvent(payload, replay),
    historyRowMatchesEvent(row, replay, payload),
  ].every(Boolean);
}

function replayPayloadMatchesEvent(payload: LanReplayPayloadCandidate, replay: ParentRouteEventSnapshot): boolean {
  return [
    payload.eventId === replay.eventId,
    payload.eventKind === replay.event,
    payload.occurredAt === replay.sentAt,
  ].every(Boolean);
}

function historyRowMatchesEvent(
  row: ParentLanDiscoveryEventRowSnapshot,
  replay: ParentRouteEventSnapshot,
  payload: LanReplayPayloadCandidate
): boolean {
  return [
    row.eventId === replay.eventId,
    row.eventKind === replay.event,
    row.occurredAt === replay.sentAt,
    row.schemaVersion === payload.schemaVersion,
    (row.previousEventId ?? null) === (payload.previousEventId ?? null),
    (row.scanSessionId ?? null) === (payload.scanSessionId ?? null),
    (row.affectedDeviceId ?? null) === (payload.affectedDeviceId ?? null),
    (row.evidenceId ?? null) === (payload.evidenceId ?? null),
    row.summary === payload.summary,
  ].every(Boolean);
}
