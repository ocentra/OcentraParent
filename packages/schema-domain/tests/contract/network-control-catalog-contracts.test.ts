import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

import {
  BaselineNetworkControlCatalog,
  explicitOptionLabels,
  networkControlCatalogCanRender,
  networkControlCatalogSettingCount,
  networkControlCatalogSettings,
  policyLaneFor,
  questionFromSourceText,
} from '../../src/network-control-catalog';

describe('network control catalog Rust-generated metadata surface', () => {
  it('keeps the public network catalog file checked in as a Rust-generated surface', () => {
    const catalogSource = readFileSync(new URL('../../src/network-control-catalog.ts', import.meta.url), 'utf8');
    const rustCatalogSource = readFileSync(
      new URL('../../../../crates/network-core/src/network_control_catalog.ts.txt', import.meta.url),
      'utf8'
    );

    expect(catalogSource).toBe(rustCatalogSource);
  });

  it('keeps the metadata helper file checked in as a Rust-generated surface', () => {
    const metadataSource = readFileSync(
      new URL('../../src/network-control-catalog-metadata.ts', import.meta.url),
      'utf8'
    );
    const rustMetadataSource = readFileSync(
      new URL('../../../../crates/network-core/src/network_control_catalog_metadata.ts.txt', import.meta.url),
      'utf8'
    );
    const settings = networkControlCatalogSettings();
    const protectedSetting = settings.find(
      (setting) =>
        setting.effectStatus === 'proof-required' &&
        setting.capabilityState === 'protected' &&
        setting.proofRequirement?.includes('Exact URL evidence requires managed browser') === true
    );
    const runtimeSetting = settings.find((setting) => setting.runtimeOwner === 'rust-parent-runtime');
    const manualSetting = settings.find(
      (setting) => setting.effectStatus === 'manual-required' && setting.capabilityState === 'manual-required'
    );

    expect(
      metadataSource.startsWith('/* generated from crates/network-core/src/network_control_catalog_metadata.ts.txt */')
    ).toBe(true);
    expect(metadataSource).toBe(rustMetadataSource);
    expect(networkControlCatalogSettingCount()).toBe(BaselineNetworkControlCatalog.settingCount);
    expect(networkControlCatalogCanRender()).toBe(true);
    expect(protectedSetting?.capabilityState).toBe('protected');
    expect(runtimeSetting?.runtimeOwner).toBe('rust-parent-runtime');
    expect(manualSetting?.unsafeOrUnsupportedFallback).toContain('manual-required');
  });

  it('preserves generated helper semantics through the schema-domain edge surface', () => {
    expect(policyLaneFor('Storage', 'Retention', 'Export audit and cache policy.')).toBe('audit');
    expect(questionFromSourceText('Capability matrix row | Capability=Router protection | Status=Ready', null)).toBe(
      'Represent Router protection capability status.'
    );
    expect(explicitOptionLabels('Target domains: gaming, streaming or chat')).toEqual(['Gaming', 'Streaming', 'Chat']);
  });
});
