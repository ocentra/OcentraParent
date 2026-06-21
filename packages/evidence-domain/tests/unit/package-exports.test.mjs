import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const packageJson = JSON.parse(readFileSync(new URL('../../package.json', import.meta.url), 'utf8'));

describe('evidence domain package exports', () => {
  it('keeps the surviving helper surface explicit', () => {
    expect(Object.keys(packageJson.exports).sort()).toEqual(['./custody', './package-info']);
    expect(packageJson.exports['./custody']).toEqual({
      import: './dist/custody.js',
      types: './dist/custody.d.ts',
    });
    expect(packageJson.exports['./package-info']).toEqual({
      import: './dist/package-info.js',
      types: './dist/package-info.d.ts',
    });
  });
});
