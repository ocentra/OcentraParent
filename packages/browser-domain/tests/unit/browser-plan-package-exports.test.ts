import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const BrowserDomainRetiredSchemaOwnerExportPaths = [
  './browser-game-hidden-analysis-profile-safety',
  './browser-game-policy-compiler',
  './browser-game-url-shape-parser',
  './social-policy-compiler',
] as const;

const BrowserDomainRetiredExportPaths = [
  './browser-control-manifest',
  './browser-game-hidden-analysis-profile-safety-guards',
  './social-alert-report-intent',
  './social-alert-report-parent-surface-intent-proof',
  './social-alert-report-provider-preflight-proof',
  './social-alert-report-provider-receipt-boundary-proof',
  './social-alert-report-provider-receipt-ingestion-readiness',
  './social-alert-report-provider-status-handoff-proof',
  './social-android-native-app-capability-matrix',
  './social-child-approval-block-ux',
  './social-decision-memory-cache',
  './social-ios-screen-time-capability-matrix',
  './social-parent-approval',
  './social-parent-sensitivity-settings',
  './social-platform-connector-authorization',
] as const;

const BrowserDomainLocalBehaviorFiles = [
  'browser-game-hidden-analysis-loader-planner',
  'browser-game-policy-candidate-compiler',
  'browser-game-url-shape-evaluator',
  'social-policy-candidate-compiler',
] as const;

const PackageRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const PackageJsonPath = resolve(PackageRoot, 'package.json');

describe('browser-domain browser plan package exports', () => {
  it('retires duplicated schema-owner subpaths while keeping local behavior internal', () => {
    const packageJson = JSON.parse(readFileSync(PackageJsonPath, 'utf8')) as {
      readonly exports?: Record<string, unknown>;
    };

    expect(packageJson.exports).toBeTypeOf('object');
    expect(packageJson.exports?.['./*']).toBeUndefined();

    for (const exportPath of BrowserDomainRetiredSchemaOwnerExportPaths) {
      const moduleName = exportPath.slice(2);
      expect(packageJson.exports?.[exportPath]).toBeUndefined();
      expect(existsSync(resolve(PackageRoot, 'src', `${moduleName}.ts`))).toBe(false);
    }

    for (const exportPath of BrowserDomainRetiredExportPaths) {
      const moduleName = exportPath.slice(2);
      expect(packageJson.exports?.[exportPath]).toBeUndefined();
      expect(existsSync(resolve(PackageRoot, 'src', `${moduleName}.ts`))).toBe(false);
    }

    for (const moduleName of BrowserDomainLocalBehaviorFiles) {
      expect(packageJson.exports?.[`./${moduleName}`]).toBeUndefined();
      expect(existsSync(resolve(PackageRoot, 'src', `${moduleName}.ts`))).toBe(true);
    }
  });
});
