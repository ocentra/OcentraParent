import { appendFileSync, existsSync, mkdirSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

import { laneHubPaths, readOrCreateMailbox, unreadMessages } from './hub-mailbox-lib.mjs';

const aggregateHeartbeatFileName = 'worker-heartbeats.ndjson';

export function aggregateHeartbeatPath(hubRoot) {
  return join(hubRoot, aggregateHeartbeatFileName);
}

export function recordLaneHeartbeat({
  event = '',
  hubRoot,
  lane,
  mailbox = undefined,
  note = '',
  now = new Date(),
  state = 'alive',
}) {
  const currentMailbox = mailbox ?? readOrCreateMailbox(hubRoot, lane, now);
  const entry = buildHeartbeatEntry({ event, lane, mailbox: currentMailbox, note, now, state });
  const paths = laneHubPaths(hubRoot, lane.id);
  const aggregatePath = aggregateHeartbeatPath(hubRoot);

  appendNdjson(paths.heartbeat, entry);
  if (lane.id !== 'primary') {
    appendNdjson(aggregatePath, entry);
  }

  return { aggregatePath, entry, lanePath: paths.heartbeat };
}

export function latestLaneHeartbeat(hubRoot, laneId) {
  return readLatestNdjson(laneHubPaths(hubRoot, laneId).heartbeat);
}

export function formatHeartbeatSummary({ hubRoot, lanes, now = new Date() }) {
  return lanes
    .map((lane) => {
      const heartbeat = latestLaneHeartbeat(hubRoot, lane.id);
      if (heartbeat === undefined) {
        return `${lane.id} | heartbeat=-`;
      }
      const ageSeconds = heartbeatAgeSeconds(heartbeat, now);
      const note = heartbeat.note ? ` | note=${heartbeat.note}` : '';
      return `${lane.id} | heartbeat=${heartbeat.ts} | age=${ageSeconds}s | state=${heartbeat.state || '-'} | session=${
        heartbeat.session || '-'
      } | branch=${heartbeat.branch || '-'}${note}`;
    })
    .join('\n');
}

function buildHeartbeatEntry({ event, lane, mailbox, note, now, state }) {
  const latestMessage = mailbox.messages.at(-1);
  const latestReport = mailbox.reports.at(-1);
  return {
    ts: now.toISOString(),
    lane: lane.id,
    owner: lane.owner ?? mailbox.owner ?? '',
    thread: lane.thread ?? mailbox.thread ?? '',
    session: lane.activeSessionId ?? mailbox.activeSessionId ?? '',
    branch: lane.branch ?? mailbox.branch ?? '',
    state,
    event,
    lastMessage: latestMessage?.id ?? '',
    lastAck: mailbox.lastAcknowledgedMessageId ?? '',
    latestReport: latestReport?.summary ?? '',
    unread: unreadMessages(mailbox).length,
    locks: mailbox.lockedPaths.length,
    note,
  };
}

function appendNdjson(path, entry) {
  mkdirSync(dirname(path), { recursive: true });
  appendFileSync(path, `${JSON.stringify(entry)}\n`, 'utf8');
}

function readLatestNdjson(path) {
  if (!existsSync(path)) {
    return undefined;
  }
  const lines = readFileSync(path, 'utf8')
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  if (lines.length === 0) {
    return undefined;
  }
  return JSON.parse(lines.at(-1));
}

function heartbeatAgeSeconds(heartbeat, now) {
  const timestamp = Date.parse(heartbeat.ts);
  if (!Number.isFinite(timestamp)) {
    return -1;
  }
  return Math.max(0, Math.floor((now.getTime() - timestamp) / 1000));
}
