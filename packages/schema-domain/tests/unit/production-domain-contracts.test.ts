import { existsSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { describe, expect, it } from 'vitest';

const schemaDomainSrc = resolve(import.meta.dirname, '../../src');
const productionDomainSrc = resolve(import.meta.dirname, '../../../production-domain/src');

const requiredCentralCopies = readdirSync(productionDomainSrc)
  .filter((name) => name.endsWith('.ts'))
  .filter((name) => name !== 'package-info.ts')
  .sort();

describe('production-domain schema centralization packet', () => {
  it('adds schema-domain copies for every copy-safe production-domain source file', () => {
    const missingCopies = requiredCentralCopies.filter((name) => !existsSync(resolve(schemaDomainSrc, name)));
    expect(missingCopies).toEqual([]);
  });

  it('imports every schema-domain production copy cleanly', async () => {
    for (const name of requiredCentralCopies) {
      const moduleUrl = pathToFileURL(resolve(schemaDomainSrc, name)).href;
      await expect(import(moduleUrl)).resolves.toBeTypeOf('object');
    }
  });
});
