import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

describe('schema-domain surface prune', () => {
  it('does not export dead browser and tracking subpaths', () => {
    const packageJsonText = readFileSync(new URL('../../package.json', import.meta.url), 'utf8').replace(/^\uFEFF/, '');
    const packageJson = JSON.parse(packageJsonText) as {
      exports: Record<string, unknown>;
    };

    expect(packageJson.exports['./browser-control-catalog-values']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog-data-0']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog-data-1']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog-data-2']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog-data-3']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog-data-4']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog-metadata']).toBeUndefined();
    expect(packageJson.exports['./browser-control-full-catalog-schema']).toBeUndefined();
    expect(packageJson.exports['./browser-ai-analysis-schemas']).toBeUndefined();
    expect(packageJson.exports['./tracking-geofence']).toBeUndefined();
    expect(packageJson.exports['./browser-control-manifest']).toMatchObject({
      import: './dist/browser-control-manifest.js',
      types: './dist/browser-control-manifest.d.ts',
    });
    expect(packageJson.exports['./network-control-catalog-data']).toBeUndefined();
    expect(packageJson.exports['./network-control-catalog-metadata']).toBeUndefined();
    expect(packageJson.exports['./network-control-catalog-schema']).toBeUndefined();
    expect(packageJson.exports['./app-game-category-risk-policy-routing-rules']).toBeUndefined();
    expect(packageJson.exports['./app-game-child-runtime-transport-receipt']).toBeUndefined();
    expect(packageJson.exports['./app-game-source-freshness-policy-consumption-data']).toBeUndefined();
    expect(packageJson.exports['./app-game-source-freshness-preview-gate']).toBeUndefined();
    expect(packageJson.exports['./app-game-source-gated-policy-preview-read-model']).toBeUndefined();
    expect(packageJson.exports['./billing-checkout-portal-boundary']).toBeUndefined();
    expect(packageJson.exports['./billing-invoice-tax-refund-dispute']).toBeUndefined();
    expect(packageJson.exports['./billing-parent-visible-summary']).toBeUndefined();
    expect(packageJson.exports['./billing-support-admin-common-values']).toBeUndefined();
    expect(packageJson.exports['./agent-command-event-contracts']).toBeUndefined();
    expect(packageJson.exports['./agent-lan']).toBeUndefined();
    expect(packageJson.exports['./agent-lan-add-device']).toBeUndefined();
    expect(packageJson.exports['./agent-lan-pairing-challenge']).toBeUndefined();
    expect(packageJson.exports['./agent-protocol-defaults']).toBeUndefined();
    expect(packageJson.exports['./agent-tracking-retention-settings-write-command']).toBeUndefined();
    expect(packageJson.exports['./capability-data']).toBeUndefined();
    expect(packageJson.exports['./mobile-child-agent-capability-proof']).toBeUndefined();
    expect(packageJson.exports['./lan-pairing-browser-runtime']).toBeUndefined();
    expect(packageJson.exports['./lan-pairing-values']).toBeUndefined();
    expect(packageJson.exports['./lan-product-proof']).toBeUndefined();
    expect(packageJson.exports['./lan-production-household-proof']).toBeUndefined();
    expect(packageJson.exports['./lan-relay-spine']).toBeUndefined();
    expect(packageJson.exports['./lan-source-matrix']).toBeUndefined();
    expect(packageJson.exports['./authority']).toBeUndefined();
    expect(packageJson.exports['./data-export-delete-lifecycle']).toBeUndefined();
    expect(packageJson.exports['./data-export-delete-lifecycle-guards']).toBeUndefined();
    expect(packageJson.exports['./enforcement-readiness']).toBeUndefined();
    expect(packageJson.exports['./tracking-primitives']).toBeUndefined();
    expect(packageJson.exports['./network-control-catalog']).toMatchObject({
      import: './dist/network-control-catalog.js',
      types: './dist/network-control-catalog.d.ts',
    });
    expect(packageJson.exports['./tracking-control-catalog']).toMatchObject({
      import: './dist/tracking-control-catalog.js',
      types: './dist/tracking-control-catalog.d.ts',
    });
  });

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
