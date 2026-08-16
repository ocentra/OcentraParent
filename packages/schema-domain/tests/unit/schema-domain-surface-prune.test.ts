import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

describe('schema-domain browser and tracking surface prune', () => {
  it('does not export dead browser and tracking subpaths', () => {
    const packageJsonText = readFileSync(new URL('../../package.json', import.meta.url), 'utf8').replace(/^\uFEFF/, '');
    const packageJson = JSON.parse(packageJsonText) as {
      exports: Record<string, unknown>;
    };

    const removedExportPaths = [
      './browser-control-catalog-values',
      './browser-control-full-catalog',
      './browser-control-full-catalog-data-0',
      './browser-control-full-catalog-data-1',
      './browser-control-full-catalog-data-2',
      './browser-control-full-catalog-data-3',
      './browser-control-full-catalog-data-4',
      './browser-control-full-catalog-metadata',
      './browser-control-full-catalog-schema',
      './browser-ai-analysis-schemas',
      './tracking-geofence',
      './network-control-catalog-data',
      './network-control-catalog-metadata',
      './network-control-catalog-schema',
      './app-game-category-risk-policy-routing-rules',
      './app-game-child-runtime-transport-receipt',
      './app-game-source-freshness-policy-consumption-data',
      './app-game-source-freshness-preview-gate',
      './app-game-source-gated-policy-preview-read-model',
      './billing-checkout-portal-boundary',
      './billing-invoice-tax-refund-dispute',
      './billing-parent-visible-summary',
      './billing-support-admin-common-values',
      './agent-command-event-contracts',
      './agent-lan',
      './agent-lan-add-device',
      './agent-lan-pairing-challenge',
      './agent-protocol-defaults',
      './agent-tracking-retention-settings-write-command',
      './capability-data',
      './mobile-child-agent-capability-proof',
      './lan-pairing-browser-runtime',
      './lan-pairing-values',
      './lan-product-proof',
      './lan-production-household-proof',
      './lan-relay-spine',
      './lan-source-matrix',
      './authority',
      './data-export-delete-lifecycle',
      './data-export-delete-lifecycle-guards',
      './enforcement-readiness',
      './tracking-primitives',
    ];

    expect(removedExportPaths.map((path) => packageJson.exports[path])).toEqual(
      removedExportPaths.map(() => undefined)
    );
    expect(packageJson.exports['./browser-control-manifest']).toMatchObject({
      import: './dist/browser-control-manifest.js',
      types: './dist/browser-control-manifest.d.ts',
    });
    expect(packageJson.exports['./network-control-catalog']).toMatchObject({
      import: './dist/network-control-catalog.js',
      types: './dist/network-control-catalog.d.ts',
    });
    expect(packageJson.exports['./tracking-control-catalog']).toMatchObject({
      import: './dist/tracking-control-catalog.js',
      types: './dist/tracking-control-catalog.d.ts',
    });
  });
});

describe('schema-domain active app-game exports', () => {
  it('keeps the active app-game contract exports intact', () => {
    const packageJsonText = readFileSync(new URL('../../package.json', import.meta.url), 'utf8').replace(/^\uFEFF/, '');
    const packageJson = JSON.parse(packageJsonText) as {
      exports: Record<string, unknown>;
    };

    expect(packageJson.exports['./app-game-policy-target-compiler']).toMatchObject({
      import: './dist/app-game-policy-target-compiler.js',
      types: './dist/app-game-policy-target-compiler.d.ts',
    });
    expect(packageJson.exports['./app-game-source-freshness-policy-consumption']).toMatchObject({
      import: './dist/app-game-source-freshness-policy-consumption.js',
      types: './dist/app-game-source-freshness-policy-consumption.d.ts',
    });
    expect(packageJson.exports['./app-game-policy-preview-handoff']).toMatchObject({
      import: './dist/app-game-policy-preview-handoff.js',
      types: './dist/app-game-policy-preview-handoff.d.ts',
    });
  });
});

describe('schema-domain policy and enforcement surface prune', () => {
  it('does not export dead policy, enforcement, or evidence subpaths', () => {
    const packageJsonText = readFileSync(new URL('../../package.json', import.meta.url), 'utf8').replace(/^\uFEFF/, '');
    const packageJson = JSON.parse(packageJsonText) as {
      exports: Record<string, unknown>;
    };

    expect(packageJson.exports['./evidence-contracts']).toBeUndefined();
    expect(packageJson.exports['./tracking-evidence']).toBeUndefined();
    expect(packageJson.exports['./tracking-location-policy']).toBeUndefined();
    expect(packageJson.exports['./tracking-location-policy-platform-proof']).toBeUndefined();
    expect(packageJson.exports['./tracking-location-policy-primitives']).toBeUndefined();
    expect(packageJson.exports['./tracking-location-policy-types']).toBeUndefined();
    expect(packageJson.exports['./browser-control-baseline-manifest']).toBeUndefined();
    expect(packageJson.exports['./enforcement']).toBeUndefined();
    expect(packageJson.exports['./enforcement-policy-dispatch']).toBeUndefined();
    expect(packageJson.exports['./enforcement-proof-shape']).toBeUndefined();
    expect(packageJson.exports['./policy']).toBeUndefined();
    expect(packageJson.exports['./policy-authority']).toBeUndefined();
    expect(packageJson.exports['./policy-literal-contracts']).toBeUndefined();
    expect(packageJson.exports['./v0-8-enforcement-product-control-spine']).toBeUndefined();
  });
});

describe('schema-domain social and support surface prune', () => {
  it('does not export dead social, screen, or support subpaths', () => {
    const packageJsonText = readFileSync(new URL('../../package.json', import.meta.url), 'utf8').replace(/^\uFEFF/, '');
    const packageJson = JSON.parse(packageJsonText) as {
      exports: Record<string, unknown>;
    };

    expect(packageJson.exports['./social-android-native-app-capability-matrix']).toBeUndefined();
    expect(packageJson.exports['./social-child-approval-block-ux']).toBeUndefined();
    expect(packageJson.exports['./social-dashboard-ux']).toBeUndefined();
    expect(packageJson.exports['./social-decision-memory-cache']).toBeUndefined();
    expect(packageJson.exports['./social-ios-screen-time-capability-matrix']).toBeUndefined();
    expect(packageJson.exports['./social-parent-approval']).toBeUndefined();
    expect(packageJson.exports['./social-platform-connector-authorization']).toBeUndefined();
    expect(packageJson.exports['./child-managed-service-respawn-proof']).toBeUndefined();
    expect(packageJson.exports['./screen-child-disclosure']).toBeUndefined();
    expect(packageJson.exports['./screen-ai-stricter-parent-rule-proof']).toBeUndefined();
    expect(packageJson.exports['./screen-control-catalog']).toBeUndefined();
    expect(packageJson.exports['./screen-evidence-read-model']).toBeUndefined();
    expect(packageJson.exports['./screen-policy-evidence-chain']).toBeUndefined();
    expect(packageJson.exports['./social-alert-report-provider-receipt-ingestion-readiness']).toBeUndefined();
    expect(packageJson.exports['./browser-policy-questionnaire-forest-data']).toBeUndefined();
    expect(packageJson.exports['./browser-social-ai-analysis-values']).toBeUndefined();
    expect(packageJson.exports['./browser-social-riskbenefit-values']).toBeUndefined();
    expect(packageJson.exports['./browser-social-ai-analysis-schemas']).toBeUndefined();
    expect(packageJson.exports['./browser-social-riskbenefit-signals']).toBeUndefined();
    expect(packageJson.exports['./browser-url-metadata-schemas']).toBeUndefined();
    expect(packageJson.exports['./support-backend-upload-status']).toBeUndefined();
    expect(packageJson.exports['./support-incident-workflow']).toBeUndefined();
    expect(packageJson.exports['./support-bundle-redaction']).toBeUndefined();
    expect(packageJson.exports['./support-proof-contract']).toBeUndefined();
  });

  it('keeps the active social, screen, and support exports intact', () => {
    const packageJsonText = readFileSync(new URL('../../package.json', import.meta.url), 'utf8').replace(/^\uFEFF/, '');
    const packageJson = JSON.parse(packageJsonText) as {
      exports: Record<string, unknown>;
    };

    expect(packageJson.exports['./social-alert-report-intent']).toMatchObject({
      import: './dist/social-alert-report-intent.js',
      types: './dist/social-alert-report-intent.d.ts',
    });
  });
});
