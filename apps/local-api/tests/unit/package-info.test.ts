import { describe, expect, it } from 'vitest';

import { LocalApiPackage } from '../../src/package-info';

describe('local-api package info', () => {
  it('names the local agent-service API contract boundary', () => {
    expect(LocalApiPackage).toEqual({
      Name: '@ocentra-parent/local-api',
      Boundary: 'agent-service-local-api-contracts',
    });
  });
});
