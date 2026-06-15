import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

import {
  BaselineBrowserControlFullCatalog,
  browserControlFullCatalogSettingCount,
  browserControlFullCatalogSettings,
} from '../../src/browser-control-full-catalog';
import { BrowserControlFullCatalogSchema } from '../../src/browser-control-full-catalog-schema';

interface SourceCatalogSetting {
  readonly sectionTitle: string;
  readonly groupTitle: string;
  readonly sourceLine: number;
  readonly sourceText: string;
}

interface SourceCatalogSettingParse {
  readonly setting: SourceCatalogSetting;
  readonly nextIndex: number;
}

const SourceCatalogSettings = readSourceCatalogSettings();
const CatalogSettings = browserControlFullCatalogSettings()
  .slice()
  .sort((left, right) => left.sourceOrder - right.sourceOrder);

describe('browser-control full catalog', () => {
  registerSourceCaptureCases();
  registerHierarchyCases();
  registerQuestionCases();
  registerEffectMetadataCases();
  registerRenderMetadataCases();
  registerCapabilityTruthCases();
  registerInvalidCatalogCases();
});

function registerSourceCaptureCases() {
  it('captures every Browser settings catalog bullet in source order', () => {
    expect(SourceCatalogSettings.length).toBe(1057);
    expect(BaselineBrowserControlFullCatalog.settingCount).toBe(1057);
    expect(browserControlFullCatalogSettingCount()).toBe(1057);
    expect(acceptedOptionCount()).toBe(2131);
    expect(CatalogSettings.map((setting) => setting.sourceText)).toEqual(
      SourceCatalogSettings.map((setting) => setting.sourceText)
    );
    expect(CatalogSettings.map((setting) => setting.sourceLine)).toEqual(
      SourceCatalogSettings.map((setting) => setting.sourceLine)
    );
  });
}

function registerHierarchyCases() {
  it('preserves the source hierarchy as tabs, sections, groups, and controls', () => {
    const catalogSections = sortedCatalogSections();
    const catalogGroups = sortedCatalogGroups();

    expect(BaselineBrowserControlFullCatalog.tabs.map((tab) => tab.tabId)).toEqual([
      'enforcement',
      'rules',
      'schedule',
      'approvals',
      'evidence',
      'reports',
      'data',
      'audit',
      'ai',
      'setup',
      'platform',
    ]);
    expect(sectionCounts(catalogSections)).toEqual(sectionCountsFromSource());
    expect(catalogGroups.map((group) => group.title)).toEqual(groupTitlesFromSource());
    expect(groupTitlesForSection('Global Rule Dimensions')).toEqual([
      'Ownership Scope',
      'Policy Mode',
      'Enforcement Phase',
      'Evidence Requirement',
      'Freshness Requirement',
      'Rule Priority',
    ]);
    expect(settingCountForSection('Browser Discovery Settings')).toBe(29);
    expect(settingCountForSection('Browser Coverage Settings')).toBe(26);
  });
}

function registerQuestionCases() {
  it('turns source settings into UI-presentable questions and accepted answers', () => {
    const enableBrowserControls = settingBySourceText('Enable browser controls.');
    const mode = settingBySourceText('Mode: observe, dry-run, warn/ask, enforce.');
    const ruleTargets = settingBySourceText(
      'Rule targets: exact URL, domain/origin, category, browser process, browser session, capability state.'
    );

    expect(BrowserControlFullCatalogSchema.safeParse(BaselineBrowserControlFullCatalog).success).toBe(true);
    expect(enableBrowserControls.controlKind).toBe('toggle');
    expect(enableBrowserControls.options.map((option) => option.label)).toEqual(['Enabled', 'Disabled']);
    expect(mode.question).toBe('Choose mode.');
    expect(mode.options.map((option) => option.label)).toEqual(['Observe', 'Dry Run', 'Warn', 'Ask', 'Enforce']);
    expect(mode.acceptedOptions.map((option) => option.originalSourceText)).toEqual([
      'Observe',
      'Dry Run',
      'Warn',
      'Ask',
      'Enforce',
    ]);
    expect(ruleTargets.selectionMode).toBe('multi');
    expect(ruleTargets.cardKind).toBe('multi-choice-many');
    expect(ruleTargets.layoutHints).toEqual({
      preferredColumnSpan: 2,
      collapsible: true,
      searchableOptions: true,
      optionGroupCount: 2,
      showAsMatrixWhenLarge: true,
      showSelectedCount: true,
    });
    expect(ruleTargets.options.map((option) => option.label)).toEqual([
      'Exact URL',
      'Domain/origin',
      'Category',
      'Browser Process',
      'Browser Session',
      'Capability State',
    ]);
  });
}

function registerEffectMetadataCases() {
  it('marks target scope, observe/enforce posture, and later effect wiring explicitly', () => {
    expect(BaselineBrowserControlFullCatalog.sidePanelCategory).toBe('browser');
    expect(BaselineBrowserControlFullCatalog.sourceDocuments).toEqual([
      'docs/browser-policy-settings-catalog.md',
      'docs/browser-control-schema-proposal.md',
      'docs/managed-unmanaged-browser.md',
      'docs/browser-control-coverage-matrix.md',
    ]);
    expect(BaselineBrowserControlFullCatalog.targetScopeOptions.map((option) => option.label)).toEqual([
      'Family',
      'Per Child',
      'Per Device',
      'Per Platform',
      'Per Browser',
      'Per Network',
    ]);
    expect(BaselineBrowserControlFullCatalog.effectModeOptions.map((option) => option.label)).toEqual([
      'Off',
      'Observe',
      'Dry Run',
      'Warn',
      'Notify',
      'Ask',
      'Limit',
      'Block',
      'Enforce',
      'Audit Only',
    ]);
    expect(
      CatalogSettings.filter((setting) => setting.question.length === 0).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.options.length === 0).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(new Set(CatalogSettings.map((setting) => setting.settingId)).size).toBe(1057);
  });
}

function registerRenderMetadataCases() {
  it('keeps source and render metadata on every setting', () => {
    expect(
      CatalogSettings.filter((setting) => setting.sidePanelCategory !== 'browser').map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.sourceDocument !== 'docs/browser-policy-settings-catalog.md').map(
        (setting) => setting.settingId
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.sourceHeadingPath.length !== 2).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.options.length !== setting.acceptedOptions.length).map(
        (setting) => setting.settingId
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => (setting.capabilityRequirement ?? '').length === 0).map(
        (setting) => setting.settingId
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter(
        (setting) => setting.effectStatus === 'proof-required' && setting.proofRequirement === null
      ).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.validationRules.length < 2).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(countSettingsBy('cardKind')).toEqual({
      'multi-choice-many': 8,
      'multi-choice-normal': 1,
      'single-choice-compact': 24,
      'single-choice-many': 2,
      toggle: 1022,
    });
    expect(countSettingsBy('effectStatus')).toEqual({
      'already-represented': 163,
      degraded: 73,
      'future-gap': 22,
      'manual-required': 167,
      'needs-effect-wiring': 551,
      'permission-required': 51,
      'proof-required': 30,
    });
    expect(countSettingsBy('capabilityState')).toEqual({
      available: 714,
      degraded: 73,
      'future-gap': 22,
      'manual-required': 167,
      'permission-required': 51,
      protected: 30,
    });
  });
}

function registerCapabilityTruthCases() {
  it('marks capability proof and unsupported fallbacks honestly', () => {
    const managedSetup = settingBySourceText('Install managed browser support.');
    const exactUrl = settingBySourceText(
      'Rule targets: exact URL, domain/origin, category, browser process, browser session, capability state.'
    );
    const futureGap = settingBySourceText('What settings belong in guide text instead of controls?');

    expect(managedSetup.effectStatus).toBe('manual-required');
    expect(managedSetup.capabilityState).toBe('manual-required');
    expect(managedSetup.unsafeOrUnsupportedFallback).toBe(
      'Disable or degrade until manual setup/proof confirms the required browser capability.'
    );
    expect(exactUrl.effectStatus).toBe('proof-required');
    expect(exactUrl.capabilityRequirement).toBe('managed-browser-or-explicit-browser-integration');
    expect(exactUrl.proofRequirement).toBe('managed-browser-or-explicit-browser-integration');
    expect(futureGap.effectStatus).toBe('future-gap');
    expect(futureGap.capabilityState).toBe('future-gap');
  });
}

function registerInvalidCatalogCases() {
  it('rejects invalid catalog enum values rather than accepting arbitrary UI data', () => {
    const invalidCategory = JSON.parse(JSON.stringify(BaselineBrowserControlFullCatalog)) as {
      sidePanelCategory: string;
    };
    invalidCategory.sidePanelCategory = 'apps';

    const invalidRuntimeOwner = JSON.parse(JSON.stringify(BaselineBrowserControlFullCatalog)) as {
      tabs: [
        {
          sections: [
            {
              groups: [
                {
                  settings: [{ runtimeOwner: string }];
                },
              ];
            },
          ];
        },
      ];
    };
    invalidRuntimeOwner.tabs[0].sections[0].groups[0].settings[0].runtimeOwner = 'portal-runtime';

    expect(BrowserControlFullCatalogSchema.safeParse(invalidCategory).success).toBe(false);
    expect(BrowserControlFullCatalogSchema.safeParse(invalidRuntimeOwner).success).toBe(false);
  });
}

function readSourceCatalogSettings(): SourceCatalogSetting[] {
  const docPath = join(process.cwd(), '..', '..', 'docs', 'browser-policy-settings-catalog.md');
  const lines = readFileSync(docPath, 'utf8').split(/\r?\n/u);
  const excludedSections = new Set([
    'Schema Proposal Snapshot',
    'How To Read This Catalog',
    'Related Existing Thinking',
  ]);
  const settings: SourceCatalogSetting[] = [];
  let sectionTitle = '';
  let groupTitle = '';
  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index] ?? '';
    const parsedSectionTitle = sectionTitleFromLine(line);
    const parsedGroupTitle = groupTitleFromLine(line);
    const parsedSetting = sourceSettingFromLine(lines, index, sectionTitle, groupTitle, excludedSections);
    if (parsedSectionTitle !== null) {
      sectionTitle = parsedSectionTitle;
      groupTitle = sectionTitle;
    } else if (parsedGroupTitle !== null) {
      groupTitle = parsedGroupTitle;
    } else if (parsedSetting !== null) {
      settings.push(parsedSetting.setting);
      index = parsedSetting.nextIndex;
    }
  }
  return settings;
}

function sectionTitleFromLine(line: string) {
  return /^## (.+)$/u.exec(line)?.[1] ?? null;
}

function groupTitleFromLine(line: string) {
  return /^### (.+)$/u.exec(line)?.[1] ?? null;
}

function sourceSettingFromLine(
  lines: readonly string[],
  index: number,
  sectionTitle: string,
  groupTitle: string,
  excludedSections: ReadonlySet<string>
): SourceCatalogSettingParse | null {
  const settingMatch = /^- (.+)$/u.exec(lines[index] ?? '');
  if (settingMatch === null || excludedSections.has(sectionTitle)) {
    return null;
  }
  const sourceLine = index + 1;
  let sourceText = settingMatch[1] ?? '';
  let cursor = index + 1;
  while (cursor < lines.length) {
    const continuation = continuationLine(lines[cursor] ?? '');
    if (continuation === null) {
      break;
    }
    sourceText = `${sourceText} ${continuation}`;
    cursor += 1;
  }
  return {
    setting: { sectionTitle, groupTitle, sourceLine, sourceText },
    nextIndex: cursor - 1,
  };
}

function continuationLine(line: string) {
  const match = /^\s{2,}(\S.*)$/u.exec(line);
  if (match === null || /^\s*-\s/u.test(line)) {
    return null;
  }
  return match[1]?.trim() ?? null;
}

function sortedCatalogSections() {
  return BaselineBrowserControlFullCatalog.tabs
    .flatMap((tab) => tab.sections)
    .slice()
    .sort((left, right) => left.sourceOrder - right.sourceOrder);
}

function sortedCatalogGroups() {
  return sortedCatalogSections()
    .flatMap((section) => section.groups)
    .slice()
    .sort((left, right) => left.sourceOrder - right.sourceOrder);
}

function sectionCounts(sections: ReturnType<typeof sortedCatalogSections>) {
  return sections.map((section) => [
    section.title,
    section.groups.reduce((count, group) => count + group.settings.length, 0),
  ]);
}

function sectionCountsFromSource() {
  const counts = new Map<string, number>();
  for (const setting of SourceCatalogSettings) {
    counts.set(setting.sectionTitle, (counts.get(setting.sectionTitle) ?? 0) + 1);
  }
  return [...counts.entries()];
}

function groupTitlesFromSource() {
  const groupKeys = new Map<string, string>();
  for (const setting of SourceCatalogSettings) {
    groupKeys.set(`${setting.sectionTitle}/${setting.groupTitle}`, setting.groupTitle);
  }
  return [...groupKeys.values()];
}

function groupTitlesForSection(sectionTitle: string) {
  const section = sortedCatalogSections().find((candidate) => candidate.title === sectionTitle);
  if (section === undefined) {
    throw new Error(`Missing section ${sectionTitle}`);
  }
  return section.groups.map((group) => group.title);
}

function settingCountForSection(sectionTitle: string) {
  const section = sortedCatalogSections().find((candidate) => candidate.title === sectionTitle);
  if (section === undefined) {
    throw new Error(`Missing section ${sectionTitle}`);
  }
  return section.groups.reduce((count, group) => count + group.settings.length, 0);
}

function acceptedOptionCount() {
  return CatalogSettings.reduce((count, setting) => count + setting.acceptedOptions.length, 0);
}

function countSettingsBy(key: 'cardKind' | 'effectStatus' | 'capabilityState') {
  const counts = new Map<string, number>();
  for (const setting of CatalogSettings) {
    counts.set(setting[key], (counts.get(setting[key]) ?? 0) + 1);
  }
  return Object.fromEntries([...counts.entries()].sort(([left], [right]) => left.localeCompare(right)));
}

function settingBySourceText(sourceText: string) {
  const setting = CatalogSettings.find((candidate) => candidate.sourceText === sourceText);
  if (setting === undefined) {
    throw new Error(`Missing catalog setting ${sourceText}`);
  }
  return setting;
}
