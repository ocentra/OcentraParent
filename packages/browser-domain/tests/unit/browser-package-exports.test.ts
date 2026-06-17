import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const BrowserDomainBrowserPlanExportPaths = [
  './browser-ai-child-ux-schemas',
  './browser-ai-parent-explanation-schemas',
  './browser-ai-provider-fallback-schemas',
  './browser-url-intelligence-schemas',
  './browser-url-intelligence',
  './browser-url-metadata-schemas',
  './browser-youtube-metadata',
  './browser-video-metadata',
  './browser-social-platform-route-schemas',
  './browser-social-url-patterns',
  './browser-social-account-flow-schemas',
  './browser-social-form-shape-detector',
  './browser-social-account-identity-registry',
  './browser-social-feed-route-classification',
  './browser-social-video-metadata',
  './browser-social-ai-analysis-schemas',
  './browser-social-ai-analysis-result-builder',
  './browser-social-riskbenefit-values',
  './browser-social-riskbenefit-signals',
  './browser-social-account-creation-gate',
  './browser-social-feed-video-route-gate-values',
  './browser-social-feed-video-route-gate-guards',
  './browser-social-feed-video-route-gate',
  './browser-social-unmanaged-bypass-detector-values',
  './browser-social-unmanaged-bypass-detector',
] as const;

const PackageRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const PackageJsonPath = resolve(PackageRoot, 'package.json');

describe('browser-domain browser plan package exports', () => {
  it('exposes browser-plan AI and social contracts as public package subpaths', () => {
    const packageJson = JSON.parse(readFileSync(PackageJsonPath, 'utf8')) as {
      readonly exports?: Record<string, unknown>;
    };

    expect(packageJson.exports).toBeTypeOf('object');
    expect(packageJson.exports?.['./*']).toEqual({
      import: './dist/*.js',
      types: './dist/*.d.ts',
    });

    for (const exportPath of BrowserDomainBrowserPlanExportPaths) {
      const moduleName = exportPath.slice(2);
      expect(existsSync(resolve(PackageRoot, 'src', `${moduleName}.ts`))).toBe(true);
    }
  });
});
