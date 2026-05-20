import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  buildHookResponse,
  buildStopResponse,
  formatAgentContext,
  formatPostToolContext,
} from '../dev/codex-hub-hook.mjs';

function workerContext(overrides = {}) {
  const message = {
    acknowledgedAt: '',
    body: 'Run the worker handoff.',
    createdAt: '2026-05-20T16:00:00.000Z',
    id: 'codex-a-msg-1',
    subject: 'Worker handoff',
  };
  return {
    branch: 'codex/v0.3-capture',
    changedPaths: [],
    hubRoot: 'C:\\Users\\sujan\\.codex\\ocentra-parent-hub',
    hubSummary: 'codex-a | thread=v0.3-capture | message=codex-a-msg-1 | ack=- | locks=- | report=-',
    lane: {
      branch: 'codex/v0.3-capture',
      id: 'codex-a',
      thread: 'v0.3-capture',
    },
    latestReport: undefined,
    mailbox: {
      lockedPaths: [],
      messages: [message],
      reports: [],
    },
    unread: [message],
    ...overrides,
  };
}

function primaryContext() {
  return {
    branch: 'main',
    changedPaths: [],
    hubRoot: 'C:\\Users\\sujan\\.codex\\ocentra-parent-hub',
    hubSummary:
      'primary | thread=primary | message=- | ack=- | locks=- | report=-\n' +
      'codex-a | thread=v0.3-capture | message=codex-a-msg-1 | ack=codex-a-msg-1 | locks=- | report=ready',
    lane: {
      id: 'primary',
      thread: 'primary',
    },
    latestReport: undefined,
    mailbox: {
      lockedPaths: [],
      messages: [],
      reports: [],
    },
    unread: [],
  };
}

test('hub hook injects primary coordination context', () => {
  const context = formatAgentContext(primaryContext());

  assert.match(context, /Current lane: primary/u);
  assert.match(context, /coordinates workers/u);
  assert.match(context, /hub:watch -- --reports/u);
});

test('hub hook injects worker inbox context', () => {
  const context = formatAgentContext(workerContext());

  assert.match(context, /Current lane: codex-a/u);
  assert.match(context, /Unread hub message/u);
  assert.match(context, /npm run hub:ack/u);
  assert.match(context, /npm run hub:report/u);
});

test('hub stop hook continues worker turns with unread hub messages', () => {
  const response = buildStopResponse({ context: workerContext() });

  assert.equal(response.decision, 'block');
  assert.match(response.reason, /Unread hub message/u);
});

test('hub stop hook allows already continued turns', () => {
  const response = buildStopResponse({ context: workerContext(), stopHookActive: true });

  assert.deepEqual(response, { continue: true });
});

test('hub post-tool hook reminds worker lanes to lock dirty paths', () => {
  const context = formatPostToolContext(
    workerContext({
      changedPaths: ['scripts/dev/example.mjs'],
      unread: [],
    })
  );

  assert.match(context, /changed files but no hub file lock/u);
  assert.match(context, /hub:lock/u);
});

test('hub hook formats session response with additional context', () => {
  const response = buildHookResponse({ hook_event_name: 'SessionStart' }, primaryContext());

  assert.equal(response.hookSpecificOutput.hookEventName, 'SessionStart');
  assert.match(response.hookSpecificOutput.additionalContext, /Ocentra Parent hub context/u);
});
