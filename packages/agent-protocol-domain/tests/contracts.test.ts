import { expect, it } from 'vitest';
import { AgentCommandEnvelopeSchema, AgentEventEnvelopeSchema, AgentProtocolDefaults } from '../src/contracts';

it('AgentCommandEnvelopeSchema: accepts a portal command for a Windows localhost agent', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-1',
    sentAt: '2026-05-19T00:00:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: 'agent.health.check',
    payload: {},
  });

  expect(parsed.success).toBe(true);
});

it('AgentMessageTargetSchema: accepts a Windows local network agent route', () => {
  const target = AgentProtocolDefaults.Target.LocalNetworkWindowsAgent;

  expect(target.route).toBe('local-network');
});

it('AgentEventEnvelopeSchema: accepts a Rust response event with an optional snapshot', () => {
  const parsed = AgentEventEnvelopeSchema.safeParse({
    schemaVersion: 1,
    eventId: 'evt-1',
    correlationId: 'cmd-1',
    sentAt: '2026-05-19T00:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.health.reported',
    severity: 'info',
    payload: {
      online: true,
    },
    snapshot: null,
  });

  expect(parsed.success).toBe(true);
});

it('AgentCommandEnvelopeSchema: rejects unknown commands', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-1',
    sentAt: '2026-05-19T00:00:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: 'agent.process.kill',
    payload: {},
  });

  expect(parsed.success).toBe(false);
});
