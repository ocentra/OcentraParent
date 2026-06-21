import { describe, expect, it } from 'vitest';
import { AgentProtocolDomainPackage } from '../../src/package-info';

describe('agent-protocol-domain package info', () => {
  it('AgentProtocolDomainPackage: identifies the schema-domain-backed adapter boundary', () => {
    expect(AgentProtocolDomainPackage.Boundary).toBe('schema-domain-protocol-adapters');
  });
});
