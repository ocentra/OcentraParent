import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  buildHookResponse,
  buildStopResponse,
  formatAgentContext,
  formatPostToolContext,
  shouldRecordSessionForHook,
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
      activeSessionId: '',
      lastAcknowledgedMessageId: '',
      lockedPaths: [],
      messages: [message],
      previousSessionId: '',
      reports: [],
      sessionSource: '',
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
      activeSessionId: '',
      id: 'primary',
      previousSessionId: '',
      thread: 'primary',
    },
    latestReport: undefined,
    mailbox: {
      activeSessionId: '',
      lastAcknowledgedMessageId: '',
      lockedPaths: [],
      messages: [],
      previousSessionId: '',
      reports: [],
      sessionSource: '',
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

test('hub hook marks replacement primary chats as coordination continuations', () => {
  const context = formatAgentContext({
    ...primaryContext(),
    mailbox: {
      activeSessionId: '019e-primary-new',
      lastAcknowledgedMessageId: '',
      lockedPaths: [],
      messages: [],
      previousSessionId: '019e-primary-old',
      reports: [],
      sessionSource: 'SessionStart:startup',
    },
    previousSessionId: '019e-primary-old',
    sessionId: '019e-primary-new',
    sessionRecordChanged: true,
    sessionSource: 'SessionStart:startup',
  });

  assert.match(context, /Current lane: primary/u);
  assert.match(context, /Active Codex thread\/session: 019e-primary-new/u);
  assert.match(context, /previous active session was 019e-primary-old/u);
  assert.match(context, /Worker summary/u);
});

test('hub hook injects worker inbox context', () => {
  const context = formatAgentContext(workerContext());

  assert.match(context, /Current lane: codex-a/u);
  assert.match(context, /Unread hub message/u);
  assert.match(context, /npm run hub:ack/u);
  assert.match(context, /npm run hub:report/u);
});

test('hub hook marks replacement worker chats as lane continuations', () => {
  const context = formatAgentContext(
    workerContext({
      mailbox: {
        activeSessionId: '019e-worker-new',
        lastAcknowledgedMessageId: 'codex-a-msg-1',
        lockedPaths: [],
        messages: [],
        previousSessionId: '019e-worker-old',
        reports: [],
        sessionSource: 'SessionStart:startup',
      },
      previousSessionId: '019e-worker-old',
      sessionId: '019e-worker-new',
      sessionRecordChanged: true,
      sessionSource: 'SessionStart:startup',
      unread: [],
    })
  );

  assert.match(context, /Active Codex thread\/session: 019e-worker-new/u);
  assert.match(context, /previous active session was 019e-worker-old/u);
  assert.match(context, /do not rerun already acknowledged hub messages/u);
  assert.match(context, /Latest acknowledged hub message: codex-a-msg-1/u);
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

test('hub hook records sessions from any hook event carrying a session id', () => {
  assert.equal(shouldRecordSessionForHook({ eventName: 'SessionStart', sessionId: '019e-worker' }), true);
  assert.equal(shouldRecordSessionForHook({ eventName: 'UserPromptSubmit', sessionId: '019e-worker' }), true);
  assert.equal(shouldRecordSessionForHook({ eventName: 'PostToolUse', sessionId: '019e-worker' }), true);
  assert.equal(shouldRecordSessionForHook({ eventName: 'Stop', sessionId: '019e-worker' }), true);
  assert.equal(shouldRecordSessionForHook({ eventName: 'Stop', sessionId: '' }), false);
  assert.equal(shouldRecordSessionForHook({ eventName: '', sessionId: '019e-worker' }), false);
});
