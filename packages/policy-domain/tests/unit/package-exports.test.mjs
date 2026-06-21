import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const packageJson = JSON.parse(readFileSync(new URL('../../package.json', import.meta.url), 'utf8'));

describe('policy domain package exports', () => {
  it('keeps the surviving export surface explicit', () => {
    expect(Object.keys(packageJson.exports).sort()).toEqual(['./package-info']);
    expect(packageJson.exports['./package-info']).toEqual({
      import: './dist/src/package-info.js',
      types: './dist/src/package-info.d.ts',
    });
  });
});
