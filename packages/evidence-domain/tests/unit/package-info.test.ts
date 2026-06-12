import { describe, expect, it } from 'vitest';
import { EvidenceDomainPackageName } from '../../src/package-info';

describe('evidence domain package boundary', () => {
  it('declares the canonical shared evidence domain package', () => {
    expect(EvidenceDomainPackageName).toBe('@ocentra-parent/evidence-domain');
  });
});

