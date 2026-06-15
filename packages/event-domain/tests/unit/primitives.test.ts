import { describe, expect, it } from 'vitest';

import { AgentMessageTargetSchema, AgentProtocolSchemaVersion } from '../../src/primitives';

describe('event-domain primitives', () => {
  it('parses shared agent message targets through the canonical event domain package', () => {
    const target = AgentMessageTargetSchema.parse({
      deviceId: 'device-alpha',
      platform: 'windows',
      route: 'localhost',
    });

    expect(target.deviceId).toBe('device-alpha');
    expect(AgentProtocolSchemaVersion).toBe(1);
  });
});
