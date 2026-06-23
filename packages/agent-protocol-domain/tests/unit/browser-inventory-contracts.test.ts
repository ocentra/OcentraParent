import { expect, it } from 'vitest';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentEventEnvelopeSchema,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';

it('AgentCommand: exposes browser inventory read-model command constant', () => {
  expect(AgentCommand.BrowserInventoryReadModelGet).toBe('agent.browser.inventory.read-model.get');
});

it('AgentEvent: exposes browser inventory read-model reported event constant', () => {
  expect(AgentEvent.BrowserInventoryReadModelReported).toBe('agent.browser.inventory.read-model.reported');
});

it('AgentCommandEnvelopeSchema: accepts browser inventory read-model requests', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-browser-inventory-1',
    sentAt: '2026-06-02T21:00:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: AgentCommand.BrowserInventoryReadModelGet,
    payload: {},
  });

  expect(parsed.success).toBe(true);
});

it('AgentEventEnvelopeSchema: accepts browser inventory read-model reports', () => {
  const parsed = AgentEventEnvelopeSchema.safeParse({
    schemaVersion: 1,
    eventId: 'browser-inventory-read-model-reported',
    correlationId: 'cmd-browser-inventory-1',
    sentAt: '2026-06-02T21:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.BrowserInventoryReadModelReported,
    severity: 'info',
    payload: {
      generatedAt: '2026-06-02T21:00:01Z',
      limit: 20,
      returned: 0,
      latestObservedAt: null,
      capabilityStatus: null,
    },
    snapshot: null,
  });

  expect(parsed.success).toBe(true);
});

it('AgentCommandEnvelopeSchema: rejects misspelled browser inventory commands', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-browser-inventory-1',
    sentAt: '2026-06-02T21:00:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: 'agent.browser.inventory.read-model.fetch',
    payload: {},
  });

  expect(parsed.success).toBe(false);
});

it('AgentEventEnvelopeSchema: rejects misspelled browser inventory events', () => {
  const parsed = AgentEventEnvelopeSchema.safeParse({
    schemaVersion: 1,
    eventId: 'browser-inventory-read-model-reported',
    correlationId: 'cmd-browser-inventory-1',
    sentAt: '2026-06-02T21:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.browser.inventory.read-model.ready',
    severity: 'info',
    payload: {},
    snapshot: null,
  });

  expect(parsed.success).toBe(false);
});
