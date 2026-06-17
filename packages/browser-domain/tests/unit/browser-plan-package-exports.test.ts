import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const BrowserDomainBrowserPlanExportPaths = [
  './browser-game-account-purchase-gate-values',
  './browser-game-account-purchase-gate',
  './browser-game-ai-analysis-values',
  './browser-game-ai-analysis',
  './browser-game-android-ios-capability-matrix-values',
  './browser-game-android-ios-capability-matrix',
  './browser-game-child-checking-block-ux-values',
  './browser-game-child-checking-block-ux',
  './browser-game-cloud-gaming-gate-values',
  './browser-game-cloud-gaming-gate',
  './browser-game-cloud-pattern-library-values',
  './browser-game-cloud-pattern-library',
  './browser-game-dashboard-ux-values',
  './browser-game-dashboard-ux',
  './browser-game-educational-classifier-values',
  './browser-game-educational-classifier',
  './browser-game-hidden-analysis-profile-safety-guards',
  './browser-game-hidden-analysis-profile-safety-values',
  './browser-game-hidden-analysis-profile-safety',
  './browser-game-hold-block-adapter-guards',
  './browser-game-hold-block-adapter-values',
  './browser-game-hold-block-adapter',
  './browser-game-journal-sqlite-read-model-guards',
  './browser-game-journal-sqlite-read-model-values',
  './browser-game-journal-sqlite-read-model',
  './browser-game-memory-cache-values',
  './browser-game-memory-cache',
  './browser-game-metadata-extractor-values',
  './browser-game-metadata-extractor',
  './browser-game-platform-route-contract-values',
  './browser-game-platform-route-contracts',
  './browser-game-policy-compiler-values',
  './browser-game-policy-compiler',
  './browser-game-portal-pattern-library-values',
  './browser-game-portal-pattern-library',
  './browser-game-riskbenefit-signal-values',
  './browser-game-riskbenefit-signal',
  './browser-game-runtime-signal-detector-values',
  './browser-game-runtime-signal-detector',
  './browser-game-ugc-multiplayer-chat-risk-values',
  './browser-game-ugc-multiplayer-chat-risk',
  './browser-game-unblocked-site-detection-values',
  './browser-game-unblocked-site-detection',
  './browser-game-url-shape-parser-values',
  './browser-game-url-shape-parser',
  './social-android-native-app-capability-matrix-values',
  './social-android-native-app-capability-matrix',
  './social-audit-explanation-read-model-values',
  './social-audit-explanation-read-model',
  './social-child-approval-block-ux-values',
  './social-child-approval-block-ux',
  './social-dashboard-ux-values',
  './social-dashboard-ux',
  './social-decision-memory-cache-values',
  './social-decision-memory-cache',
  './social-ios-screen-time-capability-matrix-values',
  './social-ios-screen-time-capability-matrix',
  './social-parent-approval',
  './social-platform-connector-authorization-values',
  './social-platform-connector-authorization',
  './social-policy-compiler-values',
  './social-policy-compiler',
] as const;

const PackageRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const PackageJsonPath = resolve(PackageRoot, 'package.json');

describe('browser-domain browser plan package exports', () => {
  it('exposes social and browser-game contracts as public package subpaths', () => {
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
