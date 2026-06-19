import { describe, expect, it } from 'vitest';
import { countProductionProofValues } from '../../src/production-proof-shape';

describe('production proof shape helpers', () => {
  it('counts expected proof values while preserving zero-count entries', () => {
    expect(countProductionProofValues(['ready', 'ready', 'failed'], ['ready', 'failed', 'manual-required'])).toEqual({
      ready: 2,
      failed: 1,
      'manual-required': 0,
    });
    expect(countProductionProofValues([], ['queued', 'running'])).toEqual({
      queued: 0,
      running: 0,
    });
  });
});
