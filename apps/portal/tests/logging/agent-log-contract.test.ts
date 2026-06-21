import { describe, expect, it } from 'vitest';
import { createAgentCommand } from '@ocentra-parent/agent-protocol-domain/agent-message-codec';
import { AgentCommandEnvelopeSchema } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { AgentLogSnapshotSchema } from '@ocentra-parent/schema-domain/logging-contracts';

describe('portal agent log contract', () => {
  it('accepts the localhost agent payload before rendering', () => {
    const parsed = AgentLogSnapshotSchema.safeParse({
      schemaVersion: 1,
      agent: {
        deviceId: 'local-dev-agent',
        hostname: 'devbox',
        platform: 'windows',
        serviceVersion: '0.1.0',
      },
      entries: [
        {
          schemaVersion: 1,
          id: 'dev-localhost-api-ready',
          timestamp: '2026-05-19T00:00:00Z',
          level: 'info',
          source: 'agent-service',
          message: 'Agent service localhost API is reachable.',
          fields: {
            captureEnabled: false,
            policyEngineEnabled: false,
            mode: 'dev',
            pid: 1000,
            remoteSync: null,
          },
        },
      ],
    });

    expect(parsed.success).toBe(true);
  });

  it('creates typed WebSocket commands for the localhost agent', () => {
    const command = createAgentCommand('agent.health.check');
    const parsed = AgentCommandEnvelopeSchema.safeParse(command);

    expect(parsed.success).toBe(true);
    expect(command.target.route).toBe('localhost');
    expect(command.target.platform).toBe('windows');
  });

  it('creates typed WebSocket commands for a local network agent', () => {
    const command = createAgentCommand('agent.health.check', {}, AgentProtocolDefaults.Target.LocalNetworkWindowsAgent);
    const parsed = AgentCommandEnvelopeSchema.safeParse(command);

    expect(parsed.success).toBe(true);
    expect(command.target.route).toBe('local-network');
  });
});
