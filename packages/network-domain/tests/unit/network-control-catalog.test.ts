import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

import {
  BaselineNetworkControlCatalog,
  NetworkControlCapabilities,
  NetworkControlGuideSettingCount,
  NetworkControlProposalSettingCount,
  buildNetworkControlEffectivePolicyPlan,
  decodeNetworkControlEffectivePolicy,
  decodeNetworkControlPolicyValueForCatalog,
  decodeNetworkControlUpdateCommandForCatalog,
  networkControlCatalogAcceptedOptionCount,
  networkControlCatalogCanRender,
  networkControlCatalogGroupCount,
  networkControlCatalogSectionCount,
  networkControlCatalogSettingCount,
  networkControlCatalogSettings,
  networkControlCatalogSourceOptionCount,
} from '../../src/network-control-catalog';
import { NetworkControlCatalogSchema } from '../../src/network-control-catalog-schema';

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

type SourceOption =
  | string
  | {
      readonly value: string;
      readonly label: string;
    };

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
const CatalogSettings = networkControlCatalogSettings();
const ProposalSettings = CatalogSettings.filter(
  (setting) => setting.sourceDocument === 'docs/network-control-schema-proposal.md'
).sort((left, right) => left.sourceOrder - right.sourceOrder);
const GuideSettings = CatalogSettings.filter(
  (setting) => setting.sourceDocument === 'docs/network-control-capability-guide.md'
).sort((left, right) => left.sourceLine - right.sourceLine);

describe('network-control catalog contracts', () => {
  registerSourceCaptureCases();
  registerHierarchyCases();
  registerRenderMetadataCases();
  registerCapabilityTruthCases();
  registerPolicyDecodeCases();
  registerPolicyRejectionCases();
});

function registerSourceCaptureCases() {
  it('captures every Network proposal field and capability-guide source setting', () => {
    expect(SourceProposal.schemaVersion).toBe(1);
    expect(SourceProposal.proposalStatus).toBe('worker-handoff-design-proposal-not-runtime-contract');
    expect(SourceSections.length).toBe(8);
    expect(SourceFields.length).toBe(21);
    expect(sourceOptionCount()).toBe(145);
    expect(SourceProposal.capabilityRegistry.capabilities.length).toBe(9);
    expect(SourceGuideSettings.length).toBe(342);

    expect(NetworkControlCatalogSchema.safeParse(BaselineNetworkControlCatalog).success).toBe(true);
    expect(BaselineNetworkControlCatalog.schemaVersion).toBe('v0.6');
    expect(BaselineNetworkControlCatalog.sidePanelCategory).toBe('network');
    expect(BaselineNetworkControlCatalog.sourceDocuments).toEqual([
      'docs/network-control-capability-guide.md',
      'docs/network-control-schema-proposal.md',
    ]);
    expect(NetworkControlProposalSettingCount).toBe(21);
    expect(NetworkControlGuideSettingCount).toBe(342);
    expect(BaselineNetworkControlCatalog.settingCount).toBe(363);
    expect(networkControlCatalogSettingCount()).toBe(363);
    expect(networkControlCatalogSourceOptionCount()).toBe(145);
    expect(ProposalSettings.map((setting) => String(setting.settingId))).toEqual(
      SourceFields.map((field) => field.fieldId)
    );
    expect(ProposalSettings.map((setting) => setting.uiQuestionText)).toEqual(
      SourceFields.map((field) => field.question)
    );
    expect(ProposalSettings.map((setting) => String(setting.writesTo))).toEqual(
      SourceFields.map((field) => field.writesTo)
    );
    expect(GuideSettings.map((setting) => setting.sourceText)).toEqual(
      SourceGuideSettings.map((setting) => setting.sourceText)
    );
    expect(GuideSettings.map((setting) => setting.sourceLine)).toEqual(
      SourceGuideSettings.map((setting) => setting.sourceLine)
    );
  });
}

function registerHierarchyCases() {
  it('preserves render hierarchy as category, tab, section, group, setting, and options', () => {
    expect(BaselineNetworkControlCatalog.tabs.map((tab) => tab.tabId)).toEqual([
      'rules',
      'evidence',
      'enforcement',
      'schedule',
      'approvals',
      'reports',
      'audit',
      'setup',
    ]);
    expect(sectionFieldCounts()).toEqual({
      'network-management': 3,
      'evidence-scope': 4,
      'domain-dns': 3,
      'flow-rules': 3,
      'vpn-proxy-tunnel': 2,
      budgets: 2,
      'local-network': 2,
      reports: 2,
    });
    expect(new Set(CatalogSettings.map((setting) => String(setting.settingId))).size).toBe(363);
    expect(networkControlCatalogCanRender()).toBe(true);
    expect(settingsForProposalSection('flow-rules').map((setting) => String(setting.groupId))).toEqual([
      'flow-rules-controls',
      'flow-rules-controls',
      'flow-rules-controls',
    ]);
  });
}

function registerRenderMetadataCases() {
  it('keeps every setting renderable with stable ids, cards, scopes, effects, and capability metadata', () => {
    expect(networkControlCatalogSectionCount()).toBe(64);
    expect(networkControlCatalogGroupCount()).toBe(107);
    expect(networkControlCatalogAcceptedOptionCount()).toBe(884);
    expect(
      CatalogSettings.filter((setting) => setting.sourceHeadingPath.length !== 2).map((setting) =>
        String(setting.settingId)
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.targetScopeOptions.length === 0).map((setting) =>
        String(setting.settingId)
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.effectModeOptions.length === 0).map((setting) =>
        String(setting.settingId)
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.acceptedOptions.length === 0).map((setting) =>
        String(setting.settingId)
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => optionIdsForSetting(setting).size !== setting.acceptedOptions.length).map(
        (setting) => String(setting.settingId)
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => setting.validationRules.length < 2).map((setting) =>
        String(setting.settingId)
      )
    ).toEqual([]);
    expect(countSettingsBy('cardKind')).toEqual({
      toggle: 147,
      'multi-choice-many': 19,
      'single-choice-compact': 3,
      'multi-choice-normal': 151,
      'status-card': 20,
      'number-card': 15,
      'schedule-card': 2,
      'retention-card': 6,
    });
    expect(countSettingsBy('effectStatus')).toEqual({
      'needs-effect-wiring': 197,
      'proof-required': 51,
      'manual-required': 34,
      degraded: 22,
      'already-represented': 49,
      'permission-required': 6,
      'future-gap': 4,
    });
    expect(countSettingsBy('capabilityState')).toEqual({
      available: 246,
      protected: 51,
      'manual-required': 34,
      degraded: 22,
      'permission-required': 6,
      'future-gap': 4,
    });
    expect(countSettingsBy('runtimeOwner')).toEqual({
      'os-adapter': 182,
      'local-ai-runtime': 84,
      'child-agent': 16,
      'parent-domain': 12,
      'portal-only': 51,
      'parent-owned-storage': 18,
    });
  });
}

function registerCapabilityTruthCases() {
  it('keeps Network truth boundaries honest for exact URLs, HTTPS content, portal ownership, and strict blocking', () => {
    const exactUrl = guideSettingByText('Capability matrix row | Capability=Attribute exact URL');
    const decryptedContent = guideSettingByText('page body, chat content, search terms');
    const manualBlocking = guideSettingByText('broad network/domain blocking remains manual-required');
    const portalBoundary = guideSettingByText('Portal UI authors rules and shows capability states');

    expect(exactUrl.effectStatus).toBe('proof-required');
    expect(exactUrl.capabilityRequirement).toBe('managed-browser-or-explicit-url-filter-proof');
    expect(exactUrl.unsafeOrUnsupportedFallback).toContain('Hide or disable exact URL controls');
    expect(decryptedContent.proofRequirement).toContain('must not collect decrypted content');
    expect(decryptedContent.unsafeOrUnsupportedFallback).toContain('Never collect decrypted content');
    expect(manualBlocking.effectStatus).toBe('manual-required');
    expect(manualBlocking.capabilityState).toBe('manual-required');
    expect(portalBoundary.runtimeOwner).toBe('portal-only');
    expect(portalBoundary.unsafeOrUnsupportedFallback).toContain('Portal renders');
  });

  it('maps the proposal capability registry into standard capability states without hiding cross-topic dependencies', () => {
    expect(NetworkControlCapabilities.length).toBe(9);
    expect(NetworkControlCapabilities.map((capability) => String(capability.capabilityId))).toEqual(
      SourceProposal.capabilityRegistry.capabilities.map((capability) => capability.capabilityId)
    );
    expect(capabilityById('ip-helper-endpoint-snapshot').state).toBe('available');
    expect(capabilityById('windows-firewall-rule-control').state).toBe('manual-required');
    expect(capabilityById('router-network-control').state).toBe('unavailable');
    expect(capabilityById('cloud-relay-network-authoring').state).toBe('disabled');
    expect(capabilityById('cloud-relay-network-authoring').affectsSettings.map(String)).toContain(
      'custody.allowedUses'
    );
  });
}

function registerPolicyDecodeCases() {
  it('decodes policy values, update commands, and effective plans against known Network catalog settings', () => {
    const policy = decodeNetworkControlPolicyValueForCatalog({
      documentId: 'network-policy-1',
      policyKind: 'network-control',
      schemaVersion: 'v0.6',
      revision: 2,
      targetDeviceId: 'device-1',
      updatedAt: '2026-05-29T00:00:00.000Z',
      settings: [
        { settingId: 'network.enabled', value: true },
        { settingId: 'network.defaultPosture', value: 'observe' },
        { settingId: 'rules.allowedActions', value: ['allow', 'observe', 'warn'] },
      ],
    });
    const plans = buildNetworkControlEffectivePolicyPlan(policy);
    const effective = decodeNetworkControlEffectivePolicy({
      documentId: 'network-effective-1',
      compiledFromPolicyId: 'network-policy-1',
      schemaVersion: 'v0.6',
      effectivePolicyHash: 'hash-1',
      compiledAt: '2026-05-29T00:01:00.000Z',
      runtimeOwner: 'child-agent',
      plans,
    });
    const command = decodeNetworkControlUpdateCommandForCatalog({
      commandType: 'network-control.patch',
      targetDeviceId: 'device-1',
      expectedRevision: 2,
      patch: [{ op: 'replace', path: '/networkPolicy/defaultPosture', value: 'warn' }],
    });

    expect(policy.settings.length).toBe(3);
    expect(effective.plans.map((plan) => String(plan.settingId))).toEqual([
      'network.enabled',
      'network.defaultPosture',
      'rules.allowedActions',
    ]);
    expect(command.patch[0]?.path).toBe('/networkPolicy/defaultPosture');
  });
}

function registerPolicyRejectionCases() {
  it('rejects unknown settings, duplicate settings, unknown update paths, and invalid enum values', () => {
    expect(() =>
      decodeNetworkControlPolicyValueForCatalog({
        documentId: 'network-policy-1',
        policyKind: 'network-control',
        schemaVersion: 'v0.6',
        revision: 1,
        targetDeviceId: 'device-1',
        updatedAt: '2026-05-29T00:00:00.000Z',
        settings: [{ settingId: 'network.unknown', value: true }],
      })
    ).toThrow('Unknown network control setting id');
    expect(() =>
      decodeNetworkControlPolicyValueForCatalog({
        documentId: 'network-policy-1',
        policyKind: 'network-control',
        schemaVersion: 'v0.6',
        revision: 1,
        targetDeviceId: 'device-1',
        updatedAt: '2026-05-29T00:00:00.000Z',
        settings: [
          { settingId: 'network.enabled', value: true },
          { settingId: 'network.enabled', value: false },
        ],
      })
    ).toThrow('Duplicate network control setting id');
    expect(() =>
      decodeNetworkControlUpdateCommandForCatalog({
        commandType: 'network-control.patch',
        targetDeviceId: 'device-1',
        expectedRevision: 1,
        patch: [{ op: 'replace', path: '/networkPolicy/unknown', value: true }],
      })
    ).toThrow('Unknown network control writesTo path');

    const invalidCatalog = JSON.parse(JSON.stringify(BaselineNetworkControlCatalog)) as {
      sidePanelCategory: string;
    };
    invalidCatalog.sidePanelCategory = 'browser';
    expect(NetworkControlCatalogSchema.safeParse(invalidCatalog).success).toBe(false);
  });
}

function readSourceProposal(): SourceProposal {
  const markdown = readFileSync(join(process.cwd(), '..', '..', 'docs', 'network-control-schema-proposal.md'), 'utf8');
  const jsonBlock = markdown.match(/```json\n([\s\S]*?)\n```/u);
  if (jsonBlock === null) {
    throw new Error('Missing JSON block in network-control schema proposal.');
  }
  return JSON.parse(jsonBlock[1] ?? '{}') as SourceProposal;
}

function readSourceGuideSettings(): SourceGuideSetting[] {
  const lines = readFileSync(
    join(process.cwd(), '..', '..', 'docs', 'network-control-capability-guide.md'),
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
  const headings = ['Capability', 'What can be possible', 'Required layer', 'Important limit'];
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

function sourceOptionCount() {
  return SourceFields.reduce((count, field) => count + (field.options?.length ?? 0), 0);
}

function sectionFieldCounts() {
  return Object.fromEntries(SourceSections.map((section) => [section.sectionId, section.fields.length]));
}

function settingsForProposalSection(sectionId: string) {
  return ProposalSettings.filter((setting) => String(setting.sectionId) === sectionId);
}

function guideSettingByText(sourceText: string) {
  const setting = GuideSettings.find((candidate) => candidate.sourceText.includes(sourceText));
  if (setting === undefined) {
    throw new Error(`Missing guide setting ${sourceText}`);
  }
  return setting;
}

function capabilityById(capabilityId: string) {
  const capability = NetworkControlCapabilities.find((candidate) => String(candidate.capabilityId) === capabilityId);
  if (capability === undefined) {
    throw new Error(`Missing capability ${capabilityId}`);
  }
  return capability;
}

function countSettingsBy(property: 'cardKind' | 'effectStatus' | 'capabilityState') {
  const counts: Record<string, number> = {};
  for (const setting of CatalogSettings) {
    const value = String(setting[property]);
    counts[value] = (counts[value] ?? 0) + 1;
  }
  return counts;
}

function optionIdsForSetting(setting: (typeof CatalogSettings)[number]) {
  return new Set(setting.acceptedOptions.map((option) => String(option.optionId)));
}
