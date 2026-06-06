import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const SourceGatedPolicyPreviewExportPaths = [
  './app-game-source-gated-policy-preview-read-model',
  './app-game-source-gated-policy-preview-timer-handoff',
  './app-game-source-gated-policy-preview-timer-status',
] as const;

const PackageDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const PackageJsonPath = resolve(PackageDir, 'package.json');

describe('app/game source-gated policy preview package exports', () => {
  it('exposes source-gated preview contracts as public parent-domain subpaths', () => {
    const packageJson = JSON.parse(readFileSync(PackageJsonPath, 'utf8')) as {
      readonly exports?: unknown;
    };

    expect(packageJson.exports).toBeTypeOf('object');
    const exportsObject = packageJson.exports as Record<(typeof SourceGatedPolicyPreviewExportPaths)[number], unknown>;

    for (const exportPath of SourceGatedPolicyPreviewExportPaths) {
      const moduleName = exportPath.slice(2);
      expect(exportsObject[exportPath]).toEqual({
        import: `./dist/${moduleName}.js`,
        types: `./dist/${moduleName}.d.ts`,
      });
    }
  });

  it('builds the exported source-gated preview dist artifacts', () => {
    for (const exportPath of SourceGatedPolicyPreviewExportPaths) {
      const moduleName = exportPath.slice(2);
      expect(existsSync(resolve(PackageDir, 'dist', `${moduleName}.js`))).toBe(true);
      expect(existsSync(resolve(PackageDir, 'dist', `${moduleName}.d.ts`))).toBe(true);
    }
  });
});
