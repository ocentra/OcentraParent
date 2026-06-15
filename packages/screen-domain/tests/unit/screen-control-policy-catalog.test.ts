import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

import {
  BaselineScreenControlCatalog,
  screenControlCatalogSettingCount,
  screenControlCatalogSettings,
} from '../../src/screen-control-catalog';
import { ScreenControlCatalogSchema } from '../../src/screen-control-catalog-schema';

interface SourceBullet {
  readonly sectionTitle: string;
  readonly groupTitle: string;
  readonly sourceLine: number;
  readonly sourceText: string;
}

interface SourceBulletParse {
  readonly bullet: SourceBullet;
  readonly nextIndex: number;
}

interface ProposalJson {
  readonly authoringManifest: {
    readonly renderingRules: Record<string, boolean>;
    readonly controlKinds: readonly string[];
    readonly conditionKinds: readonly string[];
    readonly sections: readonly {
      readonly sectionId: string;
      readonly title: string;
      readonly fields: readonly { readonly fieldId: string; readonly question: string }[];
    }[];
  };
  readonly policyValue: {
    readonly screenPolicy: {
      readonly fallbacks: Record<string, string>;
      readonly policyUse: {
        readonly lowConfidenceFallback: string;
        readonly protectedSurfaceFallback: string;
        readonly invalidOutputFallback: string;
      };
      readonly portalAi: { readonly fallbackWhenUnavailable: string };
    };
  };
  readonly effectivePolicy: {
    readonly proofRequirements: Record<string, string>;
    readonly fallbackDecisions: Record<string, string>;
    readonly rulesInPriorityOrder: readonly { readonly ruleId: string }[];
    readonly runtimeTables: {
      readonly visibleCategoryPolicyTargets: Record<string, { readonly defaultAction: string }>;
    };
  };
  readonly updateProtocol: {
    readonly commands: readonly { readonly commandType: string }[];
    readonly agentRules: Record<string, boolean>;
  };
  readonly capabilityRegistry: {
    readonly capabilities: readonly { readonly capabilityId: string }[];
    readonly stateMeanings: Record<string, string>;
  };
}

const GuideBullets = readMarkdownBullets('docs/screen-evidence-analysis-capability-guide.md');
const GuideMatrixRows = readCapabilityMatrixRows();
const ProposalBullets = readMarkdownBullets('docs/screen-evidence-analysis-schema-proposal.md');
const ProposalData = readProposalJson();
const CatalogSettings = screenControlCatalogSettings()
  .slice()
  .sort((left, right) => left.sourceOrder - right.sourceOrder);

describe('screen-control policy catalog', () => {
  registerSourceCountCases();
  registerSourceWordingCases();
  registerRenderShapeCases();
  registerManyOptionCases();
  registerCapabilityTruthCases();
  registerDistributionCases();
  registerInvalidDataCases();
});

function registerSourceCountCases() {
  it('captures every Screen source setting family with exact counts', () => {
    expect(GuideBullets.length).toBe(262);
    expect(GuideMatrixRows.length).toBe(13);
    expect(ProposalBullets.length).toBe(31);
    expect(ProposalData.authoringManifest.sections.length).toBe(11);
    expect(manifestFieldCount()).toBe(48);
    expect(Object.keys(ProposalData.authoringManifest.renderingRules).length).toBe(8);
    expect(ProposalData.authoringManifest.controlKinds.length).toBe(11);
    expect(ProposalData.authoringManifest.conditionKinds.length).toBe(10);
    expect(ProposalData.capabilityRegistry.capabilities.length).toBe(13);
    expect(Object.keys(ProposalData.capabilityRegistry.stateMeanings).length).toBe(12);
    expect(ProposalData.updateProtocol.commands.length).toBe(6);
    expect(Object.keys(ProposalData.updateProtocol.agentRules).length).toBe(17);
    expect(policyFallbackCount()).toBe(18);
    expect(Object.keys(ProposalData.effectivePolicy.proofRequirements).length).toBe(6);
    expect(Object.keys(ProposalData.effectivePolicy.fallbackDecisions).length).toBe(8);
    expect(ProposalData.effectivePolicy.rulesInPriorityOrder.length).toBe(3);
    expect(Object.keys(ProposalData.effectivePolicy.runtimeTables.visibleCategoryPolicyTargets).length).toBe(8);
    expect(BaselineScreenControlCatalog.settingCount).toBe(474);
    expect(screenControlCatalogSettingCount()).toBe(474);
    expect(acceptedOptionCount()).toBe(1153);
    expect(countSettingsBy('sourceKind')).toEqual({
      'agent-rule': 17,
      'authoring-field': 48,
      'capability-guide-bullet': 262,
      'capability-matrix-row': 13,
      'capability-registry-entry': 13,
      'capability-state-meaning': 12,
      'condition-kind': 10,
      'control-kind': 11,
      'effective-fallback': 8,
      'effective-proof-requirement': 6,
      'effective-rule': 3,
      'policy-fallback': 18,
      'rendering-rule': 8,
      'schema-proposal-bullet': 31,
      'update-command': 6,
      'visible-category-target': 8,
    });
  });
}

function registerSourceWordingCases() {
  it('preserves source wording, line numbers, and hierarchy from both docs', () => {
    expect(settingsByKind('capability-guide-bullet').map((setting) => setting.sourceText)).toEqual(
      GuideBullets.map((bullet) => bullet.sourceText)
    );
    expect(settingsByKind('capability-guide-bullet').map((setting) => setting.sourceLine)).toEqual(
      GuideBullets.map((bullet) => bullet.sourceLine)
    );
    expect(settingsByKind('capability-matrix-row').map((setting) => setting.sourceText)).toEqual(
      GuideMatrixRows.map((row) => row.sourceText)
    );
    expect(settingsByKind('schema-proposal-bullet').map((setting) => setting.sourceText)).toEqual(
      ProposalBullets.map((bullet) => bullet.sourceText)
    );
    expect(settingsByKind('authoring-field').map((setting) => setting.sourceText)).toEqual(
      ProposalData.authoringManifest.sections.flatMap((section) => section.fields.map((field) => field.question))
    );
    expect(groupTitlesForSection('Core Terms')).toEqual(['Screen Evidence', 'Screenshot', 'OCR', 'Evidence Reference']);
    expect(groupTitlesForSection('Capability Matrix')).toEqual([
      'Capture a still image',
      'Capture a recording stream',
      'Classify visible activity',
      'Extract OCR snippets',
      'Prove exact URL',
      'Prove app/window context',
      'Prove duration',
      'Detect protected surfaces',
      'Feed local AI',
      'Feed policy',
      'Feed enforcement',
      'Show parent report',
      'Retain raw capture',
    ]);
    expect(settingCountForSection('Authoring Manifest - Screen analysis')).toBe(4);
    expect(settingCountForSection('Authoring Manifest - OCR and vision')).toBe(7);
  });
}

function registerRenderShapeCases() {
  it('is schema-valid and renderable as side-panel, lane, section, group, card, and options', () => {
    expect(ScreenControlCatalogSchema.safeParse(BaselineScreenControlCatalog).success).toBe(true);
    expect(BaselineScreenControlCatalog.sidePanelCategory).toBe('screen');
    expect(BaselineScreenControlCatalog.sourceDocuments).toEqual([
      'docs/screen-evidence-analysis-capability-guide.md',
      'docs/screen-evidence-analysis-schema-proposal.md',
    ]);
    expect(BaselineScreenControlCatalog.tabs.map((tab) => tab.tabId)).toEqual([
      'evidence',
      'rules',
      'schedule',
      'approvals',
      'enforcement',
      'audit',
      'reports',
      'data',
      'ai',
      'setup',
      'platform',
    ]);
    expect(totalSectionCount()).toBe(134);
    expect(totalGroupCount()).toBe(170);
    expect(BaselineScreenControlCatalog.targetScopeOptions.map((option) => option.label)).toEqual([
      'Family',
      'Per Child',
      'Per Device',
      'Per Platform',
    ]);
    expect(BaselineScreenControlCatalog.effectModeOptions.map((option) => option.label)).toEqual([
      'Off',
      'Observe',
      'Dry Run',
      'Notify',
      'Ask',
      'Warn',
      'Limit',
      'Block',
      'Enforce',
      'Audit Only',
    ]);
    expect(new Set(CatalogSettings.map((setting) => setting.settingId)).size).toBe(474);
    expect(
      CatalogSettings.filter((setting) => setting.sidePanelCategory !== 'screen').map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.sourceHeadingPath.length !== 2).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.acceptedOptions.length === 0).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.controlKind.length === 0).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.cardKind.length === 0).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.validationRules.length < 3).map((setting) => setting.settingId)
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => optionIdsAreUnique(setting) === false).map((setting) => setting.settingId)
    ).toEqual([]);
  });
}

function registerManyOptionCases() {
  it('keeps many-option controls grouped, searchable, collapsible, and countable', () => {
    const reportFields = settingBySourceText('Which fields should parent reports show?');
    const proofFallback = settingBySourceText('What if screen proof is unavailable?');

    expect(reportFields.controlKind).toBe('multi-choice');
    expect(reportFields.cardKind).toBe('multi-choice-many');
    expect(reportFields.acceptedOptions.length).toBe(25);
    expect(reportFields.layoutHints).toEqual({
      preferredColumnSpan: 2,
      collapsible: true,
      searchableOptions: true,
      optionGroupCount: 7,
      showAsMatrixWhenLarge: true,
      showSelectedCount: true,
    });
    expect(reportFields.acceptedOptions.slice(0, 5).map((option) => option.label)).toEqual([
      'Setting State',
      'Capability State',
      'Capture Reason',
      'Capture Scope',
      'Category Candidates',
    ]);
    expect(proofFallback.cardKind).toBe('multi-choice-many');
    expect(proofFallback.acceptedOptions.map((option) => option.label)).toEqual([
      'Allow',
      'Observe',
      'Warn',
      'Ask',
      'Block Until Ready',
      'Mark Unavailable',
      'Default mark-unavailable',
    ]);
  });
}

function registerCapabilityTruthCases() {
  it('marks Screen high-sensitivity truth boundaries honestly', () => {
    const localPath = settingBySourceText('local OCR/vision returns schema-valid output;');
    const exactUrl = settingBySourceText(
      'Capability Prove exact URL: full screen No; active window No; managed browser/window Only if browser evidence proves it separately; local OCR/vision No; important limit Pixels can show text that looks like a URL, but that is not managed tab proof..'
    );
    const recording = settingBySourceText('Allow continuous screen recording?');
    const hostedProcessing = settingBySourceText('Allow Ocentra-hosted processing of child screen images?');
    const rawReports = settingBySourceText('Show raw screenshots in parent reports by default?');
    const rawRetention = settingBySourceText('Retain raw screenshots or recordings?');
    const policyUse = settingBySourceText('Allow screen summaries to be used by policy?');

    expect(localPath.runtimeOwner).toBe('local-ai-runtime');
    expect(localPath.proofRequirement).toBe('schema-valid-local-analysis-output-with-confidence-and-redaction-state');
    expect(exactUrl.effectStatus).toBe('proof-required');
    expect(exactUrl.capabilityRequirement).toBe('managed-browser-evidence-required-for-exact-web-claims');
    expect(exactUrl.proofRequirement).toBe('managed-browser-evidence-required');
    expect(recording.effectStatus).toBe('unavailable');
    expect(hostedProcessing.effectStatus).toBe('unavailable');
    expect(rawReports.effectStatus).toBe('unavailable');
    expect(rawRetention.effectStatus).toBe('unavailable');
    expect(recording.unsafeOrUnsupportedFallback).toBe(
      'Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.'
    );
    expect(policyUse.effectStatus).toBe('proof-required');
    expect(policyUse.proofRequirement).toBe(
      'validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision'
    );
  });
}

function registerDistributionCases() {
  it('reports exact card, effect, capability, and runtime ownership distribution', () => {
    expect(countSettingsBy('cardKind')).toEqual({
      'duration-card': 8,
      'multi-choice-many': 20,
      'multi-choice-normal': 1,
      'number-card': 6,
      'retention-card': 24,
      'rule-list-card': 7,
      'schedule-card': 10,
      'single-choice-compact': 39,
      'status-card': 115,
      'target-list-card': 1,
      'threshold-card': 8,
      toggle: 235,
    });
    expect(countSettingsBy('effectStatus')).toEqual({
      'already-represented': 86,
      degraded: 16,
      'future-gap': 27,
      'manual-required': 40,
      'needs-effect-wiring': 168,
      'permission-limited': 3,
      'permission-required': 57,
      'proof-required': 68,
      unavailable: 9,
    });
    expect(countSettingsBy('capabilityState')).toEqual({
      available: 320,
      degraded: 16,
      disabled: 2,
      'future-gap': 27,
      'manual-required': 40,
      'permission-limited': 3,
      'permission-required': 37,
      protected: 20,
      unavailable: 9,
    });
    expect(countSettingsBy('runtimeOwner')).toEqual({
      'agent-protocol': 12,
      'child-agent': 52,
      'local-ai-runtime': 129,
      'manual-proof': 6,
      'os-adapter': 117,
      'parent-owned-storage': 85,
      'portal-only': 73,
    });
  });
}

function registerInvalidDataCases() {
  it('rejects invalid arbitrary UI data through the Effect Schema contract', () => {
    const invalidCategory = JSON.parse(JSON.stringify(BaselineScreenControlCatalog)) as {
      sidePanelCategory: string;
    };
    invalidCategory.sidePanelCategory = 'browser';

    const invalidRuntimeOwner = JSON.parse(JSON.stringify(BaselineScreenControlCatalog)) as {
      tabs: [{ sections: [{ groups: [{ settings: [{ runtimeOwner: string }] }] }] }];
    };
    invalidRuntimeOwner.tabs[0].sections[0].groups[0].settings[0].runtimeOwner = 'portal-runtime';

    const invalidEffectStatus = JSON.parse(JSON.stringify(BaselineScreenControlCatalog)) as {
      tabs: [{ sections: [{ groups: [{ settings: [{ effectStatus: string }] }] }] }];
    };
    invalidEffectStatus.tabs[0].sections[0].groups[0].settings[0].effectStatus = 'secret-capture-supported';

    expect(ScreenControlCatalogSchema.safeParse(invalidCategory).success).toBe(false);
    expect(ScreenControlCatalogSchema.safeParse(invalidRuntimeOwner).success).toBe(false);
    expect(ScreenControlCatalogSchema.safeParse(invalidEffectStatus).success).toBe(false);
  });
}

function readMarkdownBullets(repoRelativePath: string): SourceBullet[] {
  const lines = readRepoFile(repoRelativePath).split(/\r?\n/u);
  const bullets: SourceBullet[] = [];
  let sectionTitle = repoRelativePath.endsWith('schema-proposal.md')
    ? 'Proposal Overview'
    : 'Capability Guide Overview';
  let groupTitle = sectionTitle;
  for (let index = 0; index < lines.length; index += 1) {
    const parsedSectionTitle = sectionTitleFromLine(lines[index] ?? '');
    const parsedGroupTitle = groupTitleFromLine(lines[index] ?? '');
    const parsedBullet = sourceBulletFromLine(lines, index, sectionTitle, groupTitle);
    if (parsedSectionTitle !== null) {
      sectionTitle = parsedSectionTitle;
      groupTitle = parsedSectionTitle;
    } else if (parsedGroupTitle !== null) {
      groupTitle = parsedGroupTitle;
    } else if (parsedBullet !== null) {
      bullets.push(parsedBullet.bullet);
      index = parsedBullet.nextIndex;
    }
  }
  return bullets;
}

function readCapabilityMatrixRows(): SourceBullet[] {
  const lines = readRepoFile('docs/screen-evidence-analysis-capability-guide.md').split(/\r?\n/u);
  const rows: SourceBullet[] = [];
  let inMatrix = false;
  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index] ?? '';
    if (/^## Capability Matrix$/u.test(line)) {
      inMatrix = true;
      continue;
    }
    if (inMatrix && /^## /u.test(line)) {
      break;
    }
    if (!inMatrix || !/^\| /u.test(line) || /^\| -/u.test(line) || /^\| Capability/u.test(line)) {
      continue;
    }
    const cells = line
      .split('|')
      .slice(1, -1)
      .map((cell) => cell.trim());
    rows.push({
      sectionTitle: 'Capability Matrix',
      groupTitle: cells[0] ?? '',
      sourceLine: index + 1,
      sourceText: `Capability ${cells[0]}: full screen ${cells[1]}; active window ${cells[2]}; managed browser/window ${cells[3]}; local OCR/vision ${cells[4]}; important limit ${cells[5]}.`,
    });
  }
  return rows;
}

function readProposalJson(): ProposalJson {
  const match = /```json\n([\s\S]*?)\n```/u.exec(readRepoFile('docs/screen-evidence-analysis-schema-proposal.md'));
  if (match === null) {
    throw new Error('Missing Screen schema proposal JSON block.');
  }
  return JSON.parse(match[1] ?? '') as ProposalJson;
}

function sourceBulletFromLine(
  lines: readonly string[],
  index: number,
  sectionTitle: string,
  groupTitle: string
): SourceBulletParse | null {
  const settingMatch = /^- (.+)$/u.exec(lines[index] ?? '');
  if (settingMatch === null || sectionTitle === 'Source References') {
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
    bullet: { sectionTitle, groupTitle, sourceLine, sourceText },
    nextIndex: cursor - 1,
  };
}

function sectionTitleFromLine(line: string) {
  return /^## (.+)$/u.exec(line)?.[1] ?? null;
}

function groupTitleFromLine(line: string) {
  return /^### (.+)$/u.exec(line)?.[1] ?? null;
}

function continuationLine(line: string) {
  const match = /^\s{2,}(\S.*)$/u.exec(line);
  if (match === null || /^\s*-\s/u.test(line)) {
    return null;
  }
  return match[1]?.trim() ?? null;
}

function readRepoFile(repoRelativePath: string) {
  return readFileSync(join(process.cwd(), '..', '..', repoRelativePath), 'utf8');
}

function manifestFieldCount() {
  return ProposalData.authoringManifest.sections.reduce((count, section) => count + section.fields.length, 0);
}

function policyFallbackCount() {
  return (
    Object.keys(ProposalData.policyValue.screenPolicy.fallbacks).length +
    [
      ProposalData.policyValue.screenPolicy.policyUse.lowConfidenceFallback,
      ProposalData.policyValue.screenPolicy.policyUse.protectedSurfaceFallback,
      ProposalData.policyValue.screenPolicy.policyUse.invalidOutputFallback,
      ProposalData.policyValue.screenPolicy.portalAi.fallbackWhenUnavailable,
    ].length
  );
}

function acceptedOptionCount() {
  return CatalogSettings.reduce((count, setting) => count + setting.acceptedOptions.length, 0);
}

function totalSectionCount() {
  return BaselineScreenControlCatalog.tabs.reduce((count, tab) => count + tab.sections.length, 0);
}

function totalGroupCount() {
  return BaselineScreenControlCatalog.tabs.reduce(
    (count, tab) => count + tab.sections.reduce((sectionCount, section) => sectionCount + section.groups.length, 0),
    0
  );
}

function settingsByKind(sourceKind: string) {
  return CatalogSettings.filter((setting) => setting.sourceKind === sourceKind);
}

function groupTitlesForSection(sectionTitle: string) {
  const sections = BaselineScreenControlCatalog.tabs
    .flatMap((tab) => tab.sections)
    .filter((candidate) => candidate.title === sectionTitle);
  if (sections.length === 0) {
    throw new Error(`Missing Screen catalog section ${sectionTitle}`);
  }
  return [
    ...new Map(
      sections
        .flatMap((section) => section.groups)
        .sort((left, right) => left.sourceOrder - right.sourceOrder)
        .map((group) => [group.title, group.title])
    ).values(),
  ];
}

function settingCountForSection(sectionTitle: string) {
  const sections = BaselineScreenControlCatalog.tabs
    .flatMap((tab) => tab.sections)
    .filter((candidate) => candidate.title === sectionTitle);
  if (sections.length === 0) {
    throw new Error(`Missing Screen catalog section ${sectionTitle}`);
  }
  return sections.reduce(
    (count, section) => count + section.groups.reduce((groupCount, group) => groupCount + group.settings.length, 0),
    0
  );
}

function settingBySourceText(sourceText: string) {
  const setting = CatalogSettings.find((candidate) => candidate.sourceText === sourceText);
  if (setting === undefined) {
    throw new Error(`Missing Screen catalog setting ${sourceText}`);
  }
  return setting;
}

function optionIdsAreUnique(setting: (typeof CatalogSettings)[number]) {
  return new Set(setting.acceptedOptions.map((option) => option.optionId)).size === setting.acceptedOptions.length;
}

function countSettingsBy(key: 'sourceKind' | 'cardKind' | 'effectStatus' | 'capabilityState' | 'runtimeOwner') {
  const counts = new Map<string, number>();
  for (const setting of CatalogSettings) {
    counts.set(setting[key], (counts.get(setting[key]) ?? 0) + 1);
  }
  return Object.fromEntries([...counts.entries()].sort(([left], [right]) => left.localeCompare(right)));
}
