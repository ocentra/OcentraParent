import { describe, expect, it } from 'vitest';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentCommandNameLiteral,
  AgentCommandNameSchema,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentEventNameLiteral,
  AgentEventNameSchema,
  AgentLanPairingSupportedWebSocketCommand,
  decodeAgentDeviceId,
  decodeAgentMessageId,
  decodeAgentTimestamp,
  decodeAgentWebSocketUrl,
  decodeSerializedAgentMessage,
  isAgentProtocolLogText,
} from '../../src/agent-command-event-contracts';
import {
  AgentLanBrowserRuntimeCommandNameLiteral,
  AgentLanBrowserRuntimeEventNameLiteral,
} from '../../src/lan-pairing-browser-runtime';
import { AgentProtocolSchemaVersion } from '../../src/event-primitives';

describe('agent command and event contracts', () => {
  it('accepts canonical command and event names, including LAN browser runtime names', () => {
    expect(AgentCommandNameSchema.parse(AgentCommandNameLiteral.HealthCheck)).toBe(
      AgentCommandNameLiteral.HealthCheck
    );
    expect(
      AgentCommandNameSchema.parse(AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan)
    ).toBe(AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan);
    expect(AgentEventNameSchema.parse(AgentEventNameLiteral.ConnectionReady)).toBe(
      AgentEventNameLiteral.ConnectionReady
    );
    expect(
      AgentEventNameSchema.parse(AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported)
    ).toBe(AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported);
    expect(AgentCommandNameSchema.safeParse('agent.unknown.command').success).toBe(false);
    expect(AgentEventNameSchema.safeParse('agent.unknown.event').success).toBe(false);
  });

  it('validates command and event envelopes with structured log payloads', () => {
    const commandEnvelope = AgentCommandEnvelopeSchema.safeParse({
      schemaVersion: AgentProtocolSchemaVersion,
      messageId: 'cmd-contract-1',
      sentAt: '2026-06-20T19:35:00Z',
      source: {
        peerId: 'portal-dev',
        role: 'portal',
      },
      target: {
        deviceId: 'local-dev-agent',
        platform: 'windows',
        route: 'localhost',
      },
      command: AgentCommandNameLiteral.HealthCheck,
      payload: {
        limit: 1,
      },
    });
    const eventEnvelope = AgentEventEnvelopeSchema.safeParse({
      schemaVersion: AgentProtocolSchemaVersion,
      eventId: 'evt-contract-1',
      correlationId: 'corr-contract-1',
      sentAt: '2026-06-20T19:35:01Z',
      source: {
        peerId: 'agent-service',
        role: 'agent-service',
      },
      target: {
        peerId: 'portal-dev',
        role: 'portal',
      },
      event: AgentEventNameLiteral.HealthReported,
      severity: 'info',
      payload: {
        message: 'healthy',
      },
      snapshot: null,
    });
    const invalidEventEnvelope = AgentEventEnvelopeSchema.safeParse({
      schemaVersion: AgentProtocolSchemaVersion,
      eventId: 'evt-contract-2',
      correlationId: 'corr-contract-2',
      sentAt: '2026-06-20T19:35:02Z',
      source: {
        peerId: 'agent-service',
        role: 'agent-service',
      },
      target: {
        peerId: 'portal-dev',
        role: 'portal',
      },
      event: 'agent.invalid.reported',
      severity: 'info',
      payload: {},
      snapshot: null,
    });

    expect(commandEnvelope.success).toBe(true);
    expect(eventEnvelope.success).toBe(true);
    expect(invalidEventEnvelope.success).toBe(false);
  });

  it('recognizes protocol log text safely', () => {
    expect(isAgentProtocolLogText('hello')).toBe(true);
    expect(isAgentProtocolLogText(42)).toBe(false);
    expect(isAgentProtocolLogText({ message: 'hello' })).toBe(false);
  });

  it('exposes parsed command and event value maps for shared transport use', () => {
    expect(AgentCommand.HealthCheck).toBe(AgentCommandNameLiteral.HealthCheck);
    expect(AgentCommand.LanPairingBrowserDiscoveryScan).toBe(
      AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan
    );
    expect(AgentLanPairingSupportedWebSocketCommand.AddDeviceRequest).toBe(
      AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest
    );
    expect(AgentEvent.ConnectionReady).toBe(AgentEventNameLiteral.ConnectionReady);
    expect(AgentEvent.LanPairingAddDeviceReported).toBe(
      AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported
    );
  });

  it('decodes shared branded transport primitives', () => {
    expect(decodeAgentDeviceId('child-device-1')).toBe('child-device-1');
    expect(decodeAgentMessageId('cmd-contract-3')).toBe('cmd-contract-3');
    expect(decodeAgentTimestamp('2026-06-20T19:35:03Z')).toBe(
      '2026-06-20T19:35:03Z'
    );
    expect(decodeAgentWebSocketUrl('ws://127.0.0.1:4477/api/dev/ws')).toBe(
      'ws://127.0.0.1:4477/api/dev/ws'
    );
    expect(
      decodeSerializedAgentMessage(
        '{"schemaVersion":"v1","messageId":"cmd-contract-3","payload":{"ok":true}}'
      )
    ).toBe(
      '{"schemaVersion":"v1","messageId":"cmd-contract-3","payload":{"ok":true}}'
    );
  });
});
