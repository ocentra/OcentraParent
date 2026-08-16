import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import {
  BaselineTrackingControlCatalog,
  TrackingControlCapabilities,
  buildTrackingControlEffectivePolicyPlan,
  decodeTrackingControlUpdateCommandForCatalog,
  trackingControlCatalogSettings,
} from '@ocentra-parent/schema-domain/tracking-control-catalog';

describe('tracking control catalog contracts', () => {
  generatedSurfaceProof();
  catalogPlanProof();
});

function generatedSurfaceProof(): void {
  it('keeps the tracking control catalog as a thin Rust-seeded schema-domain surface', () => {
    const catalogSource = readFileSync(new URL('../../src/tracking-control-catalog.ts', import.meta.url), 'utf8');
    const schemaSource = readFileSync(new URL('../../src/tracking-control-catalog-schema.ts', import.meta.url), 'utf8');
    const metadataSource = readFileSync(
      new URL('../../src/tracking-control-catalog-metadata.ts', import.meta.url),
      'utf8'
    );
    const catalogSettings = trackingControlCatalogSettings(BaselineTrackingControlCatalog);
    const knownSettingIds = new Set(catalogSettings.map((setting) => String(setting.settingId)));
    const capabilitySettingsOutsideCatalog = TrackingControlCapabilities.flatMap((capability) =>
      capability.affectsSettings
        .filter((settingId) => !knownSettingIds.has(String(settingId)))
        .map((settingId) => `${capability.capabilityId}:${String(settingId)}`)
    );

    expect(
      catalogSource.startsWith('/* generated from crates/tracking-core/src/tracking_control_catalog.ts.txt */')
    ).toBe(true);
    expect(
      schemaSource.startsWith('/* generated from crates/tracking-core/src/tracking_control_catalog_schema.ts.txt */')
    ).toBe(true);
    expect(
      metadataSource.startsWith(
        '/* generated from crates/tracking-core/src/tracking_control_catalog_metadata.ts.txt */'
      )
    ).toBe(true);
    expect(catalogSource).toContain('./tracking-control-catalog-data');
    expect(catalogSource).toContain('./tracking-control-catalog-metadata');
    expect(catalogSource).not.toContain('const TrackingControlCatalogJson =');
    expect(catalogSource).not.toContain('const TrackingControlCapabilitiesJson =');
    expect(catalogSettings.length).toBe(BaselineTrackingControlCatalog.settingCount);
    expect(
      catalogSettings.filter((setting) => setting.sourceDocument === BaselineTrackingControlCatalog.sourceDocuments[0])
        .length +
        catalogSettings.filter(
          (setting) => setting.sourceDocument === BaselineTrackingControlCatalog.sourceDocuments[1]
        ).length
    ).toBe(BaselineTrackingControlCatalog.settingCount);
    expect(catalogSettings.reduce((count, setting) => count + setting.options.length, 0)).toBeGreaterThan(0);
    expect(BaselineTrackingControlCatalog.sidePanelCategory).toBe('tracking');
    expect(BaselineTrackingControlCatalog.tabs.length).toBeGreaterThan(0);
    expect(
      catalogSettings.every(
        (setting) =>
          setting.policyLane.length > 0 &&
          setting.controlKind.length > 0 &&
          setting.cardKind.length > 0 &&
          setting.layoutHints.preferredColumnSpan > 0 &&
          setting.targetScopeOptions.length > 0 &&
          setting.effectModeOptions.length > 0 &&
          setting.visibilityConditions.length > 0 &&
          setting.enabledConditions.length > 0 &&
          setting.validationRules.length > 0
      )
    ).toBe(true);
    expect(capabilitySettingsOutsideCatalog).toEqual([
      'android-precise-location:reports.showExactCoordinateRequiresReveal',
      'parent-owned-storage-sync:custody.allowedUses',
      'parent-owned-storage-sync:reports.summaries',
      'parent-owned-storage-sync:retention.locationPointHistory',
      'ocentra-hosted-raw-location-history:custody.allowOcentraHostedRawLocationHistory',
    ]);
  });
}

function catalogPlanProof(): void {
  it('keeps update-command and effective-policy planning aligned to the generated catalog', () => {
    const command = decodeTrackingControlUpdateCommandForCatalog({
      commandType: 'tracking-control.patch',
      targetDeviceId: 'parent-device-local',
      expectedRevision: 7,
      patch: [
        { op: 'replace', path: '/locationPolicy/defaultPosture', value: 'temporary-live' },
        { op: 'replace', path: '/locationPolicy/live/maxSessionMinutes', value: 30 },
        { op: 'replace', path: '/locationPolicy/permissions/whenPermissionMissing', value: 'ask-parent' },
        {
          op: 'replace',
          path: '/locationPolicy/catalogGuide/tracking-guide-custody-retention-and-audit-custody-retention-and-audit-196',
          value: 'represented',
        },
        {
          op: 'replace',
          path: '/locationPolicy/catalogGuide/tracking-guide-custody-retention-and-audit-custody-retention-and-audit-191',
          value: 'represented',
        },
      ],
    });

    const plans = buildTrackingControlEffectivePolicyPlan({
      documentId: BaselineTrackingControlCatalog.catalogId,
      policyKind: 'device-location-tracking',
      schemaVersion: BaselineTrackingControlCatalog.schemaVersion,
      revision: command.expectedRevision,
      targetDeviceId: command.targetDeviceId,
      updatedAt: '2026-06-29T14:44:00Z',
      settings: [
        { settingId: 'location.defaultPosture', value: 'temporary-live' },
        { settingId: 'live.maxSessionMinutes', value: 30 },
        { settingId: 'permissions.whenPermissionMissing', value: 'ask-parent' },
        {
          settingId: 'tracking-guide-custody-retention-and-audit-custody-retention-and-audit-196',
          value: 'represented',
        },
        {
          settingId: 'tracking-guide-custody-retention-and-audit-custody-retention-and-audit-191',
          value: 'represented',
        },
      ],
    });

    expect(command.patch.map((row) => row.path)).toEqual([
      '/locationPolicy/defaultPosture',
      '/locationPolicy/live/maxSessionMinutes',
      '/locationPolicy/permissions/whenPermissionMissing',
      '/locationPolicy/catalogGuide/tracking-guide-custody-retention-and-audit-custody-retention-and-audit-196',
      '/locationPolicy/catalogGuide/tracking-guide-custody-retention-and-audit-custody-retention-and-audit-191',
    ]);
    expect(plans.map((row) => row.settingId)).toEqual([
      'location.defaultPosture',
      'live.maxSessionMinutes',
      'permissions.whenPermissionMissing',
      'tracking-guide-custody-retention-and-audit-custody-retention-and-audit-196',
      'tracking-guide-custody-retention-and-audit-custody-retention-and-audit-191',
    ]);
    expect(plans[0]?.runtimeOwner).toBe('os-adapter');
    expect(plans[1]?.writesTo).toBe('/locationPolicy/live/maxSessionMinutes');
    expect(plans[2]?.effectStatus).toBe('permission-required');
  });
}
