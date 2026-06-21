import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const BrowserDomainBrowserPlanExportPaths = [
  './browser-policy-questionnaire-forest',
  './browser-url-intelligence',
] as const;

const PackageRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const PackageJsonPath = resolve(PackageRoot, 'package.json');

describe('browser-domain browser plan package exports', () => {
  it('exposes explicit browser-plan package subpaths without wildcard leakage', () => {
    const packageJson = JSON.parse(readFileSync(PackageJsonPath, 'utf8')) as {
      readonly exports?: Record<string, unknown>;
    };

    expect(packageJson.exports).toBeTypeOf('object');
    expect(packageJson.exports?.['./*']).toBeUndefined();

    for (const exportPath of BrowserDomainBrowserPlanExportPaths) {
      const moduleName = exportPath.slice(2);
      expect(packageJson.exports?.[exportPath]).toEqual({
        import: `./dist/${moduleName}.js`,
        types: `./dist/${moduleName}.d.ts`,
      });
      expect(existsSync(resolve(PackageRoot, 'src', `${moduleName}.ts`))).toBe(true);
    }
  });
});
