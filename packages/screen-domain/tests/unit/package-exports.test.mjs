import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const packageJson = JSON.parse(readFileSync(new URL('../../package.json', import.meta.url), 'utf8'));

describe('screen domain package exports', () => {
  it('keeps the surviving screen-domain runtime surface explicit', () => {
    expect(Object.keys(packageJson.exports).sort()).toEqual([
      './package-info',
      './screen-ai-enforcement-handoff-guard-proof',
      './screen-child-disclosure-page',
      './screen-evidence-family-hub-routing',
      './screen-intelligence-router',
    ]);
    expect(packageJson.exports['./package-info']).toEqual({
      import: './dist/package-info.js',
      types: './dist/package-info.d.ts',
    });
    expect(packageJson.exports['./screen-ai-enforcement-handoff-guard-proof']).toEqual({
      import: './dist/screen-ai-enforcement-handoff-guard-proof.js',
      types: './dist/screen-ai-enforcement-handoff-guard-proof.d.ts',
    });
    expect(packageJson.exports['./screen-child-disclosure-page']).toEqual({
      import: './dist/screen-child-disclosure-page.js',
      types: './dist/screen-child-disclosure-page.d.ts',
    });
    expect(packageJson.exports['./screen-evidence-family-hub-routing']).toEqual({
      import: './dist/screen-evidence-family-hub-routing.js',
      types: './dist/screen-evidence-family-hub-routing.d.ts',
    });
    expect(packageJson.exports['./screen-intelligence-router']).toEqual({
      import: './dist/screen-intelligence-router.js',
      types: './dist/screen-intelligence-router.d.ts',
    });
  });
});
