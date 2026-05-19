import { describe, expect, it } from 'vitest';
import { AgentLogSnapshotSchema } from '../src/contracts';
import { LoggingDomainPackage } from '../src/package-info';

describe('logging-domain package info', () => {
  it('LoggingDomainPackage: identifies the operational logging boundary', () => {
    expect(LoggingDomainPackage.Boundary).toBe('operational-logging-contracts');
  });

  it('AgentLogSnapshotSchema: accepts the Rust localhost log snapshot contract', () => {
    const parsed = AgentLogSnapshotSchema.parse({
      schemaVersion: 1,
      agent: {
        deviceId: 'local-dev',
        hostname: 'devbox',
        platform: 'windows',
        serviceVersion: '0.1.0',
      },
      entries: [
        {
          id: 'dev-startup',
          timestamp: '2026-05-19T00:00:00Z',
          level: 'info',
          source: 'agent-service',
          message: 'Agent service localhost API is reachable.',
          fields: {
            captureEnabled: false,
            pid: 1000,
            mode: 'dev',
            remoteSync: null,
          },
        },
      ],
    });

    expect(parsed.entries[0]?.source).toBe('agent-service');
  });

  it('AgentLogSnapshotSchema: rejects unknown log levels', () => {
    const parsed = AgentLogSnapshotSchema.safeParse({
      schemaVersion: 1,
      agent: {
        deviceId: 'local-dev',
        hostname: 'devbox',
        platform: 'windows',
        serviceVersion: '0.1.0',
      },
      entries: [
        {
          id: 'bad-level',
          timestamp: '2026-05-19T00:00:00Z',
          level: 'notice',
          source: 'agent-service',
          message: 'This level is not part of the contract.',
          fields: {},
        },
      ],
    });

    expect(parsed.success).toBe(false);
  });
});
