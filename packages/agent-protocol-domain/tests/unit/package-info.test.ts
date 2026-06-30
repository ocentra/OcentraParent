import { describe, expect, it } from 'vitest';
import { AgentProtocolDomainPackage } from '../../src/package-info';

describe('agent-protocol-domain package info', () => {
  it('AgentProtocolDomainPackage: identifies the generated thin adapter boundary', () => {
    expect(AgentProtocolDomainPackage.Boundary).toBe('generated-thin-protocol-adapter');
  });
});
