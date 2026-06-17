import { describe, expect, it } from 'vitest';

import { browserControlFullCatalogSettings } from '../../src/browser-control-full-catalog';
import {
  BrowserControlCandidateMvpItems,
  BrowserControlCatalogMajorSections,
  BrowserControlCoverageMatrix,
  BrowserControlCoverageMatrixSchema,
  type BrowserControlCoverageEntry,
} from '../../src/browser-control-coverage-matrix';
import { BrowserControlWritesToPath } from '../../src/browser-control-values';
import { browserPolicyForestSourceSettingIds } from '../../src/browser-policy-questionnaire-forest';
import { BrowserInventoryPlatformMatrix } from '../../src/browser-platform-inventory-matrix';
import { PolicyCompilerCapabilityState } from '@ocentra-parent/policy-domain/policy-compiler';

describe('browser control coverage matrix', () => {
  it('covers every candidate MVP item and major catalog section exactly once', () => {
    const matrix = BrowserControlCoverageMatrixSchema.parse(BrowserControlCoverageMatrix);
    const candidateItems = entriesByKind(matrix, 'candidate-mvp').map((entry) => entry.catalogItem);
    const majorSections = entriesByKind(matrix, 'catalog-section').map((entry) => entry.catalogSection);

    expect(candidateItems).toEqual([...BrowserControlCandidateMvpItems]);
    expect(majorSections).toEqual([...BrowserControlCatalogMajorSections]);
    expect(new Set(candidateItems).size).toBe(BrowserControlCandidateMvpItems.length);
    expect(new Set(majorSections).size).toBe(BrowserControlCatalogMajorSections.length);
  });

  it('keeps direct-control, capability, and docs-only rows honest', () => {
    const matrix = BrowserControlCoverageMatrixSchema.parse(BrowserControlCoverageMatrix);
    const implemented = matrix.filter((entry) => entry.coverageStatus === 'implemented-manifest-control');
    const capability = matrix.filter((entry) => entry.coverageStatus === 'represented-through-capability');
    const unsupported = matrix.filter(
      (entry) =>
        entry.coverageStatus === 'documentation-only' ||
        entry.coverageStatus === 'future-gap' ||
        entry.coverageStatus === 'unavailable'
    );
    const docsOnly = matrix.filter((entry) => entry.coverageStatus === 'documentation-only');

    expect(implemented.every((entry) => entry.manifestFieldIds.length > 0 && entry.writesTo.length > 0)).toBe(true);
    expect(capability.every((entry) => entry.capabilityState !== null)).toBe(true);
    expect(matrix.every((entry) => entry.compilerCapabilityState !== null)).toBe(true);
    expect(implemented.every((entry) => entry.compilerCapabilityState === PolicyCompilerCapabilityState.Supported)).toBe(
      true
    );
    expect(
      capability.every(
        (entry) =>
          entry.compilerCapabilityState ===
          (entry.capabilityState === 'manual-required'
            ? PolicyCompilerCapabilityState.ManualRequired
            : PolicyCompilerCapabilityState.Supported)
      )
    ).toBe(true);
    expect(unsupported.every((entry) => entry.compilerCapabilityState === PolicyCompilerCapabilityState.Unsupported)).toBe(
      true
    );
    expect(
      docsOnly.every(
        (entry) =>
          entry.manifestFieldIds.length === 0 &&
          entry.writesTo.length === 0 &&
          entry.policyShape === null &&
          entry.capabilityState === null &&
          entry.compilerCapabilityState === PolicyCompilerCapabilityState.Unsupported
      )
    ).toBe(true);
  });

  it('stays aligned with catalog sections and questionnaire source coverage', () => {
    const sourceSections = new Set(browserControlFullCatalogSettings().map((setting) => setting.sourceHeadingPath[0]));
    const questionnaireSettingIds = new Set([...browserPolicyForestSourceSettingIds().values()].flat());
    const catalogSettingIds = browserControlFullCatalogSettings().map((setting) => setting.settingId);

    expect(
      entriesByKind(BrowserControlCoverageMatrix, 'catalog-section')
        .filter((entry) => entry.coverageStatus !== 'documentation-only' && entry.coverageStatus !== 'future-gap')
        .every((entry) => sourceSections.has(entry.catalogSection))
    ).toBe(true);
    expect([...questionnaireSettingIds].sort()).toEqual(catalogSettingIds.slice().sort());
  });

  it('keeps managed and unmanaged browser coverage aligned with platform reference states', () => {
    const managedCoverage = coverageEntryByCatalogItem(
      'Choose covered browsers: Edge, Chrome, Chrome for Testing, unsupported as unmanaged.'
    );
    const managedSetupSection = coverageSectionByName('Managed Browser Setup Settings');
    const unmanagedSection = coverageSectionByName('Unmanaged Browser Handling Settings');
    const platformEntries = BrowserInventoryPlatformMatrix.entries;

    expect(managedCoverage.coverageStatus).toBe('implemented-manifest-control');
    expect(managedCoverage.manifestFieldIds).toEqual([
      'managedBrowser.allowedFamilies',
      'unmanagedBrowser.classificationTargets',
    ]);
    expect(managedCoverage.writesTo).toEqual([
      BrowserControlWritesToPath.ManagedBrowserAllowedFamilies,
      BrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
    ]);
    expect(managedSetupSection.coverageStatus).toBe('represented-through-capability');
    expect(managedSetupSection.capabilityState).toBe('manual-required');
    expect(managedSetupSection.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.ManualRequired);
    expect(unmanagedSection.coverageStatus).toBe('implemented-manifest-control');
    expect(unmanagedSection.capabilityState).toBeNull();
    expect(unmanagedSection.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.Supported);
    expect(platformEntries.some((entry) => entry.platform === 'windows' && entry.managementTier === 'managed')).toBe(
      true
    );
    expect(platformEntries.some((entry) => entry.managementTier === 'owned-shell')).toBe(true);
    expect(platformEntries.some((entry) => entry.managementTier === 'unsupported')).toBe(true);
  });
});

function entriesByKind(
  entries: readonly BrowserControlCoverageEntry[],
  kind: BrowserControlCoverageEntry['coverageKind']
): BrowserControlCoverageEntry[] {
  return entries.filter((entry) => entry.coverageKind === kind);
}

function coverageEntryByCatalogItem(catalogItem: string): BrowserControlCoverageEntry {
  const entry = BrowserControlCoverageMatrix.find(
    (candidate) => candidate.coverageKind === 'candidate-mvp' && candidate.catalogItem === catalogItem
  );
  if (entry === undefined) {
    throw new Error(`Missing coverage entry for ${catalogItem}`);
  }
  return entry;
}

function coverageSectionByName(catalogSection: string): BrowserControlCoverageEntry {
  const entry = BrowserControlCoverageMatrix.find(
    (candidate) => candidate.coverageKind === 'catalog-section' && candidate.catalogSection === catalogSection
  );
  if (entry === undefined) {
    throw new Error(`Missing coverage section ${catalogSection}`);
  }
  return entry;
}
