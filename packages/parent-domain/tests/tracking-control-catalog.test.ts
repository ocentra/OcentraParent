import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

import {
  BaselineTrackingControlCatalog,
  TrackingControlCapabilities,
  TrackingControlGuideSettingCount,
  TrackingControlProposalSettingCount,
  buildTrackingControlEffectivePolicyPlan,
  decodeTrackingControlEffectivePolicy,
  decodeTrackingControlPolicyValueForCatalog,
  decodeTrackingControlUpdateCommandForCatalog,
  trackingControlCatalogAcceptedOptionCount,
  trackingControlCatalogCanRender,
  trackingControlCatalogGroupCount,
  trackingControlCatalogSectionCount,
  trackingControlCatalogSettingCount,
  trackingControlCatalogSettings,
  trackingControlCatalogSourceOptionCount,
} from '../src/tracking-control-catalog';
import { TrackingControlCatalogSchema } from '../src/tracking-control-catalog-schema';

interface SourceProposal {
  readonly schemaVersion: number;
  readonly proposalStatus: string;
  readonly authoringManifest: {
    readonly sections: readonly SourceSection[];
  };
  readonly capabilityRegistry: {
    readonly capabilities: readonly SourceCapability[];
  };
}

interface SourceSection {
  readonly sectionId: string;
  readonly title: string;
  readonly fields: readonly SourceField[];
}

interface SourceField {
  readonly fieldId: string;
  readonly kind: string;
  readonly question: string;
  readonly writesTo: string;
  readonly options?: readonly SourceOption[];
}

interface SourceOption {
  readonly value: string;
  readonly label: string;
}

interface SourceCapability {
  readonly capabilityId: string;
  readonly state: string;
  readonly affectsFields: readonly string[];
}

interface SourceGuideSetting {
  readonly sectionTitle: string;
  readonly groupTitle: string;
  readonly sourceLine: number;
  readonly sourceText: string;
}

interface SourceGuideSettingParse {
  readonly setting: SourceGuideSetting;
  readonly nextIndex: number;
}

const SourceProposal = readSourceProposal();
const SourceSections = SourceProposal.authoringManifest.sections;
const SourceFields = SourceSections.flatMap((section) => section.fields);
const SourceGuideSettings = readSourceGuideSettings();
const CatalogSettings = trackingControlCatalogSettings()
  .slice()
  .sort((left, right) => left.sourceOrder - right.sourceOrder);
const ProposalSettings = CatalogSettings.filter(
  (setting) => setting.sourceDocument === 'docs/device-location-tracking-schema-proposal.md'
);
const GuideSettings = CatalogSettings.filter(
  (setting) => setting.sourceDocument === 'docs/device-location-tracking-capability-guide.md'
).sort((left, right) => left.sourceLine - right.sourceLine);

describe('tracking-control policy catalog contracts', () => {
  registerSourceCaptureCases();
  registerHierarchyCases();
  registerRenderMetadataCases();
  registerCapabilityTruthCases();
  registerPolicyContractCases();
});

function registerSourceCaptureCases() {
  it('captures every Tracking proposal field and source option in source order', () => {
    expect(SourceProposal.schemaVersion).toBe(1);
    expect(SourceProposal.proposalStatus).toBe('design-proposal-not-runtime-contract');
    expect(SourceSections.length).toBe(7);
    expect(SourceFields.length).toBe(24);
    expect(sourceProposalOptionCount()).toBe(90);
    expect(SourceProposal.capabilityRegistry.capabilities.length).toBe(8);

    expect(TrackingControlCatalogSchema.safeParse(BaselineTrackingControlCatalog).success).toBe(true);
    expect(TrackingControlProposalSettingCount).toBe(24);
    expect(TrackingControlGuideSettingCount).toBe(314);
    expect(trackingControlCatalogSettingCount()).toBe(338);
    expect(trackingControlCatalogSourceOptionCount()).toBe(90);
    expect(ProposalSettings.map((setting) => String(setting.settingId))).toEqual(
      SourceFields.map((field) => field.fieldId)
    );
    expect(ProposalSettings.map((setting) => setting.uiQuestionText)).toEqual(
      SourceFields.map((field) => field.question)
    );
    expect(ProposalSettings.map((setting) => String(setting.writesTo))).toEqual(
      SourceFields.map((field) => field.writesTo)
    );
  });

  it('captures every Tracking capability-guide bullet and capability-matrix row', () => {
    expect(SourceGuideSettings.length).toBe(314);
    expect(GuideSettings.length).toBe(314);
    expect(GuideSettings.map((setting) => setting.sourceText)).toEqual(
      SourceGuideSettings.map((setting) => setting.sourceText)
    );
    expect(GuideSettings.map((setting) => setting.sourceLine)).toEqual(
      SourceGuideSettings.map((setting) => setting.sourceLine)
    );
    expect(guideSectionCounts()).toEqual({
      'Accuracy Sources And Limits': 16,
      'Capability Matrix': 14,
      'Check-In And Safety Prompts': 4,
      'Child-Facing Disclosure': 7,
      'Core Terms': 28,
      'Current Ocentra Parent Posture': 12,
      'Custody, Retention, And Audit': 16,
      'Device Location Permissions': 9,
      'Device Online, Offline, And Battery State': 14,
      'Future UI Rules': 18,
      Geofences: 16,
      'Live Tracking: What Is Possible': 19,
      'Location History: What Is Possible': 16,
      'Missing-Proof Fallbacks': 8,
      'Platform Capability Notes': 50,
      'Policy Modes To Represent Later In UI': 44,
      'Reports And Maps': 14,
      'The Main Capability Truth': 9,
    });
  });
}

function registerHierarchyCases() {
  it('preserves side panel, tabs, sections, groups, settings, and proposal section counts', () => {
    expect(BaselineTrackingControlCatalog.sidePanelCategory).toBe('tracking');
    expect(BaselineTrackingControlCatalog.sourceDocuments).toEqual([
      'docs/device-location-tracking-capability-guide.md',
      'docs/device-location-tracking-schema-proposal.md',
    ]);
    expect(BaselineTrackingControlCatalog.tabs.map((tab) => String(tab.tabId))).toEqual([
      'rules',
      'evidence',
      'live',
      'places',
      'approvals',
      'enforcement',
      'reports',
      'setup',
      'platform',
      'data',
    ]);
    expect(trackingControlCatalogSectionCount()).toBe(81);
    expect(trackingControlCatalogGroupCount()).toBe(102);
    expect(trackingControlCatalogAcceptedOptionCount()).toBe(902);
    expect(proposalSectionCounts()).toEqual({
      alerts: 2,
      'check-ins': 4,
      'last-known': 3,
      'live-tracking': 4,
      'location-management': 3,
      permissions: 4,
      'places-geofences': 4,
    });
    expect(sourceSectionSettingCounts()).toEqual(sourceCatalogSectionSettingCounts());
    expect(trackingControlCatalogCanRender()).toBe(true);
  });
}

function registerRenderMetadataCases() {
  it('keeps every setting renderable with stable ids, cards, options, scopes, and effects', () => {
    expect(new Set(CatalogSettings.map((setting) => String(setting.settingId))).size).toBe(338);
    expect(CatalogSettings.filter((setting) => setting.sourceHeadingPath.length !== 2)).toEqual([]);
    expect(CatalogSettings.filter((setting) => setting.targetScopeOptions.length === 0)).toEqual([]);
    expect(CatalogSettings.filter((setting) => setting.effectModeOptions.length === 0)).toEqual([]);
    expect(CatalogSettings.filter((setting) => setting.visibilityConditions.length === 0)).toEqual([]);
    expect(CatalogSettings.filter((setting) => setting.enabledConditions.length < 2)).toEqual([]);
    expect(CatalogSettings.filter((setting) => setting.validationRules.length < 2)).toEqual([]);
    expect(CatalogSettings.filter((setting) => setting.capabilityRequirement.length === 0)).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => optionIdsForSetting(setting).size !== setting.acceptedOptions.length)
    ).toEqual([]);
    expect(BaselineTrackingControlCatalog.targetScopeOptions.map((option) => option.label)).toEqual([
      'Family',
      'Per Child',
      'Per Device',
      'Per Platform',
      'Per Location Rule',
    ]);
    expect(BaselineTrackingControlCatalog.effectModeOptions.map((option) => option.label)).toEqual([
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
    expect(countSettingsBy('cardKind')).toEqual({
      'geofence-list-card': 17,
      'multi-choice-many': 4,
      'multi-choice-normal': 178,
      'number-card': 19,
      'place-list-card': 7,
      'retention-card': 27,
      'schedule-card': 4,
      'single-choice-compact': 56,
      'single-choice-many': 2,
      'status-card': 14,
      toggle: 10,
    });
    expect(countSettingsBy('effectStatus')).toEqual({
      'already-represented': 64,
      degraded: 38,
      'future-gap': 42,
      'manual-required': 14,
      'needs-effect-wiring': 43,
      'permission-required': 74,
      'proof-required': 61,
      unavailable: 2,
    });
    expect(countSettingsBy('capabilityState')).toEqual({
      available: 107,
      degraded: 38,
      'future-gap': 42,
      'manual-required': 14,
      'permission-required': 74,
      protected: 61,
      unavailable: 2,
    });
    expect(countSettingsBy('runtimeOwner')).toEqual({
      'child-agent': 53,
      'manual-proof': 5,
      'os-adapter': 143,
      'parent-domain': 36,
      'parent-owned-storage': 81,
      'portal-only': 20,
    });
  });
}

function registerCapabilityTruthCases() {
  it('keeps Tracking truth boundaries explicit rather than over-claiming live or exact location', () => {
    const lastKnownBoundary = guideSettingByText('Last known location is not proof of current location.');
    const continuousTrail = guideSettingByText('Capability matrix row | Capability=Exact continuous movement trail');
    const hostedHistory = guideSettingByText(
      'Ocentra-hosted services may route minimal notification or relay metadata'
    );
    const locationDecision = guideSettingByText('Capability matrix row | Capability=Enforce location-based policy');
    const portalPolicyEvaluation = guideSettingByText('Portal-side policy evaluation.');
    const staleGuess = guideSettingByText('Guessing current location from stale last-known evidence.');

    expect(lastKnownBoundary.effectStatus).toBe('proof-required');
    expect(lastKnownBoundary.proofRequirement).toContain('Current or live location claims require source');
    expect(lastKnownBoundary.unsafeOrUnsupportedFallback).toContain('never imply stale last-known evidence is current');
    expect(continuousTrail.effectStatus).toBe('manual-required');
    expect(continuousTrail.proofRequirement).toContain('last-known evidence must be labeled stale');
    expect(hostedHistory.runtimeOwner).toBe('parent-owned-storage');
    expect(hostedHistory.unsafeOrUnsupportedFallback).toContain('never silently upload location history');
    expect(locationDecision.runtimeOwner).toBe('child-agent');
    expect(locationDecision.proofRequirement).toContain('deterministic child-agent evaluation');
    expect(locationDecision.unsafeOrUnsupportedFallback).toContain('degraded');
    expect(portalPolicyEvaluation.effectStatus).toBe('unavailable');
    expect(portalPolicyEvaluation.runtimeOwner).toBe('portal-only');
    expect(portalPolicyEvaluation.unsafeOrUnsupportedFallback).toContain('child-agent/local runtime owns');
    expect(staleGuess.effectStatus).toBe('unavailable');
    expect(staleGuess.unsafeOrUnsupportedFallback).toContain('never imply stale last-known');
  });

  it('maps the proposal capability registry into typed states and preserves future custody gaps', () => {
    expect(TrackingControlCapabilities.length).toBe(8);
    expect(TrackingControlCapabilities.map((capability) => String(capability.capabilityId))).toEqual(
      SourceProposal.capabilityRegistry.capabilities.map((capability) => capability.capabilityId)
    );
    expect(capabilityById('android-foreground-location').state).toBe('available');
    expect(capabilityById('android-background-location').state).toBe('permission-required');
    expect(capabilityById('android-geofencing').state).toBe('manual-required');
    expect(capabilityById('parent-owned-storage-sync').state).toBe('future-gap');
    expect(capabilityById('ocentra-hosted-raw-location-history').state).toBe('protected');
    expect(capabilityById('ocentra-hosted-raw-location-history').affectsSettings.map(String)).toEqual([
      'custody.allowOcentraHostedRawLocationHistory',
    ]);
    expect(sourceCapabilityStateCounts()).toEqual({
      'blocked-by-default': 1,
      'manual-required': 3,
      'not-implemented': 1,
      'permission-required': 2,
      ready: 1,
    });
  });
}

function registerPolicyContractCases() {
  registerPolicyDecodeCases();
  registerPolicyRejectionCases();
}

function registerPolicyDecodeCases() {
  it('decodes policy values, update commands, and effective plans against known catalog settings', () => {
    const policy = decodeTrackingControlPolicyValueForCatalog({
      documentId: 'tracking-policy-1',
      policyKind: 'device-location-tracking',
      schemaVersion: 'v0.6',
      revision: 3,
      targetDeviceId: 'device-1',
      updatedAt: '2026-05-29T00:00:00.000Z',
      settings: [
        { settingId: 'location.enabled', value: true },
        { settingId: 'permissions.minimumPermission', value: 'foreground-precise' },
        { settingId: 'live.mode', value: 'parent-started-temporary' },
      ],
    });
    const plans = buildTrackingControlEffectivePolicyPlan(policy);
    const effective = decodeTrackingControlEffectivePolicy({
      documentId: 'tracking-effective-1',
      compiledFromPolicyId: 'tracking-policy-1',
      schemaVersion: 'v0.6',
      effectivePolicyHash: 'tracking-hash-1',
      compiledAt: '2026-05-29T00:01:00.000Z',
      runtimeOwner: 'child-agent',
      plans,
    });
    const command = decodeTrackingControlUpdateCommandForCatalog({
      commandType: 'tracking-control.patch',
      targetDeviceId: 'device-1',
      expectedRevision: 3,
      patch: [{ op: 'replace', path: '/locationPolicy/defaultPosture', value: 'check-in-only' }],
    });

    expect(policy.settings.length).toBe(3);
    expect(effective.plans.map((plan) => plan.effectStatus)).toEqual([
      'needs-effect-wiring',
      'permission-required',
      'proof-required',
    ]);
    expect(command.patch[0]?.path).toBe('/locationPolicy/defaultPosture');
  });
}

function registerPolicyRejectionCases() {
  it('rejects unknown settings, duplicate settings, unknown update paths, and invalid catalog enum values', () => {
    expect(() =>
      decodeTrackingControlPolicyValueForCatalog({
        documentId: 'tracking-policy-1',
        policyKind: 'device-location-tracking',
        schemaVersion: 'v0.6',
        revision: 1,
        targetDeviceId: 'device-1',
        updatedAt: '2026-05-29T00:00:00.000Z',
        settings: [{ settingId: 'location.unknown', value: true }],
      })
    ).toThrow('Unknown tracking control setting id');
    expect(() =>
      decodeTrackingControlPolicyValueForCatalog({
        documentId: 'tracking-policy-1',
        policyKind: 'device-location-tracking',
        schemaVersion: 'v0.6',
        revision: 1,
        targetDeviceId: 'device-1',
        updatedAt: '2026-05-29T00:00:00.000Z',
        settings: [
          { settingId: 'location.enabled', value: true },
          { settingId: 'location.enabled', value: false },
        ],
      })
    ).toThrow('Duplicate tracking control setting id');
    expect(() =>
      decodeTrackingControlUpdateCommandForCatalog({
        commandType: 'tracking-control.patch',
        targetDeviceId: 'device-1',
        expectedRevision: 1,
        patch: [{ op: 'replace', path: '/locationPolicy/unknown', value: true }],
      })
    ).toThrow('Unknown tracking control writesTo path');

    const invalidCatalog = JSON.parse(JSON.stringify(BaselineTrackingControlCatalog)) as {
      sidePanelCategory: string;
    };
    invalidCatalog.sidePanelCategory = 'browser';
    expect(TrackingControlCatalogSchema.safeParse(invalidCatalog).success).toBe(false);
  });
}

function readSourceProposal(): SourceProposal {
  const markdown = readFileSync(
    join(process.cwd(), '..', '..', 'docs', 'device-location-tracking-schema-proposal.md'),
    'utf8'
  );
  const jsonBlock = markdown.match(/```json\n([\s\S]*?)\n```/u);
  if (jsonBlock === null) {
    throw new Error('Missing JSON block in device-location-tracking schema proposal.');
  }
  return JSON.parse(jsonBlock[1] ?? '{}') as SourceProposal;
}

function readSourceGuideSettings(): SourceGuideSetting[] {
  const lines = readFileSync(
    join(process.cwd(), '..', '..', 'docs', 'device-location-tracking-capability-guide.md'),
    'utf8'
  ).split(/\r?\n/u);
  const excludedSections = new Set(['Source References']);
  const settings: SourceGuideSetting[] = [];
  let sectionTitle = '';
  let groupTitle = '';
  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index] ?? '';
    const parsedSectionTitle = sectionTitleFromLine(line);
    const parsedGroupTitle = groupTitleFromLine(line);
    const parsedTableSetting = guideTableSettingFromLine(line, index, sectionTitle, groupTitle, excludedSections);
    const parsedSetting = guideSettingFromLine(lines, index, sectionTitle, groupTitle, excludedSections);
    if (parsedSectionTitle !== null) {
      sectionTitle = parsedSectionTitle;
      groupTitle = sectionTitle;
    } else if (parsedGroupTitle !== null) {
      groupTitle = parsedGroupTitle;
    } else if (parsedTableSetting !== null) {
      settings.push(parsedTableSetting.setting);
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

function guideTableSettingFromLine(
  line: string,
  index: number,
  sectionTitle: string,
  groupTitle: string,
  excludedSections: ReadonlySet<string>
): SourceGuideSettingParse | null {
  if (excludedSections.has(sectionTitle) || sectionTitle !== 'Capability Matrix') {
    return null;
  }
  const cells = tableCellsFromLine(line);
  if (cells === null || cells[0] === 'Capability') {
    return null;
  }
  return {
    setting: { sectionTitle, groupTitle, sourceLine: index + 1, sourceText: capabilityMatrixSourceText(cells) },
    nextIndex: index,
  };
}

function guideSettingFromLine(
  lines: readonly string[],
  index: number,
  sectionTitle: string,
  groupTitle: string,
  excludedSections: ReadonlySet<string>
): SourceGuideSettingParse | null {
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

function tableCellsFromLine(line: string) {
  if (!/^\|.*\|\s*$/u.test(line)) {
    return null;
  }
  const cells = line
    .trim()
    .slice(1, -1)
    .split('|')
    .map((cell) => cell.trim());
  if (cells.length < 2 || cells.every((cell) => /^-+$/u.test(cell.replace(/\s+/gu, '')))) {
    return null;
  }
  return cells;
}

function capabilityMatrixSourceText(cells: readonly string[]) {
  const headings = [
    'Capability',
    'Mobile child agent',
    'Desktop/laptop child agent',
    'Required layer',
    'Important limit',
  ];
  return `Capability matrix row | ${cells
    .map((cell, index) => `${headings[index] ?? `Column ${index + 1}`}=${cell}`)
    .join(' | ')}`;
}

function continuationLine(line: string) {
  const match = /^\s{2,}(\S.*)$/u.exec(line);
  if (match === null || /^\s*-\s/u.test(line)) {
    return null;
  }
  return match[1]?.trim() ?? null;
}

function sourceProposalOptionCount() {
  return SourceFields.reduce((count, field) => count + (field.options?.length ?? 0), 0);
}

function proposalSectionCounts() {
  return sortedObject(
    Object.fromEntries(
      SourceSections.map((section) => [
        section.sectionId,
        ProposalSettings.filter((setting) => String(setting.sectionId) === section.sectionId).length,
      ])
    )
  );
}

function guideSectionCounts() {
  const counts: Record<string, number> = {};
  for (const setting of SourceGuideSettings) {
    counts[setting.sectionTitle] = (counts[setting.sectionTitle] ?? 0) + 1;
  }
  return sortedObject(counts);
}

function sourceSectionSettingCounts() {
  const counts: Record<string, number> = {};
  for (const setting of [...ProposalSettings, ...GuideSettings]) {
    const title = setting.sourceHeadingPath[0] ?? String(setting.sectionId);
    counts[title] = (counts[title] ?? 0) + 1;
  }
  return sortedObject(counts);
}

function sourceCatalogSectionSettingCounts() {
  const counts: Record<string, number> = {};
  for (const section of SourceSections) {
    counts[section.title] = section.fields.length;
  }
  for (const setting of SourceGuideSettings) {
    counts[setting.sectionTitle] = (counts[setting.sectionTitle] ?? 0) + 1;
  }
  return sortedObject(counts);
}

function sourceCapabilityStateCounts() {
  const counts: Record<string, number> = {};
  for (const capability of SourceProposal.capabilityRegistry.capabilities) {
    counts[capability.state] = (counts[capability.state] ?? 0) + 1;
  }
  return sortedObject(counts);
}

function guideSettingByText(sourceText: string) {
  const setting = GuideSettings.find((candidate) => candidate.sourceText.includes(sourceText));
  if (setting === undefined) {
    throw new Error(`Missing guide setting ${sourceText}`);
  }
  return setting;
}

function capabilityById(capabilityId: string) {
  const capability = TrackingControlCapabilities.find((candidate) => String(candidate.capabilityId) === capabilityId);
  if (capability === undefined) {
    throw new Error(`Missing capability ${capabilityId}`);
  }
  return capability;
}

function countSettingsBy(property: 'cardKind' | 'effectStatus' | 'capabilityState' | 'runtimeOwner') {
  const counts: Record<string, number> = {};
  for (const setting of CatalogSettings) {
    const value = String(setting[property]);
    counts[value] = (counts[value] ?? 0) + 1;
  }
  return sortedObject(counts);
}

function optionIdsForSetting(setting: (typeof CatalogSettings)[number]) {
  return new Set(setting.acceptedOptions.map((option) => String(option.optionId)));
}

function sortedObject(input: Record<string, number>) {
  return Object.fromEntries(Object.entries(input).sort(([left], [right]) => left.localeCompare(right)));
}
