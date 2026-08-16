import { describe, expect, it } from 'vitest';
import { AgentLogSnapshotSchema } from '@ocentra-parent/logging-domain/logging-contracts';
import {
  decodeParentAgentCommandEnvelope,
  ParentAgentCommand,
  type ParentAgentCommandEnvelope,
  type ParentAgentMessageTarget,
  ParentAgentPeerDefaults,
  ParentAgentProtocolRuntime,
  ParentAgentTargetDefaults,
} from '../../generated/parent-ui-bridge';

const AgentLogSnapshotFixture = {
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
};

const HealthCheckCommandEnvelope = {
  schemaVersion: ParentAgentProtocolRuntime.SchemaVersion,
  messageId: 'cmd-portal-log-contract',
  sentAt: '2026-05-19T00:00:00Z',
  source: ParentAgentPeerDefaults.PortalDev,
  target: ParentAgentTargetDefaults.LocalhostWindowsAgent,
  command: ParentAgentCommand.HealthCheck,
  payload: {},
};

describe('portal agent log contract', () => {
  it('accepts the localhost agent payload before rendering', () => {
    const parsed = AgentLogSnapshotSchema.safeParse(AgentLogSnapshotFixture);

    expect(parsed.success).toBe(true);
  });

  it('creates typed WebSocket commands for the localhost agent', () => {
    const command = createHealthCheckCommand();
    const parsed = decodeParentAgentCommandEnvelope(command);

    expect(parsed).toEqual(command);
    expect(command.target.route).toBe('localhost');
    expect(command.target.platform).toBe('windows');
  });

  it('creates typed WebSocket commands for a local network agent', () => {
    const command = createHealthCheckCommand(ParentAgentTargetDefaults.LocalNetworkWindowsAgent);
    const parsed = decodeParentAgentCommandEnvelope(command);

    expect(parsed).toEqual(command);
    expect(command.target.route).toBe('local-network');
  });
});

function createHealthCheckCommand(
  target: ParentAgentMessageTarget = ParentAgentTargetDefaults.LocalhostWindowsAgent
): ParentAgentCommandEnvelope {
  return decodeParentAgentCommandEnvelope({
    ...HealthCheckCommandEnvelope,
    target,
  });
}
