import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const packageJson = JSON.parse(readFileSync(new URL('../../package.json', import.meta.url), 'utf8'));

describe('capability domain package exports', () => {
  it('keeps the export surface explicit', () => {
    expect(Object.keys(packageJson.exports).sort()).toEqual([
      './capabilities',
      './capability-data',
      './package-info',
    ]);
    expect(packageJson.exports['./capabilities']).toEqual({
      import: './dist/src/capabilities.js',
      types: './dist/src/capabilities.d.ts',
    });
    expect(packageJson.exports['./capability-data']).toEqual({
      import: './dist/src/capability-data.js',
      types: './dist/src/capability-data.d.ts',
    });
    expect(packageJson.exports['./package-info']).toEqual({
      import: './dist/src/package-info.js',
      types: './dist/src/package-info.d.ts',
    });
  });
});
