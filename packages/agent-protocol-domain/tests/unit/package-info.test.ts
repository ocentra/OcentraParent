import { describe, expect, it } from 'vitest';
import { AgentProtocolDomainPackage } from '../../src/package-info';

describe('agent-protocol-domain package info', () => {
  it('AgentProtocolDomainPackage: identifies the agent protocol contract boundary', () => {
    expect(AgentProtocolDomainPackage.Boundary).toBe('agent-command-event-contracts');
  });
});
