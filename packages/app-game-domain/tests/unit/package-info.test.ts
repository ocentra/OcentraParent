import { describe, expect, it } from 'vitest';
import { AppGameDomainPackageName } from '../../src/package-info';

describe('app-game domain package boundary', () => {
  it('declares the canonical app and native-game domain package', () => {
    expect(AppGameDomainPackageName).toBe('@ocentra-parent/app-game-domain');
  });
});

