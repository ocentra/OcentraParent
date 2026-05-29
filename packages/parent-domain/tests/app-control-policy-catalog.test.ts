import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

import {
  AppControlCapabilities,
  BaselineAppControlAuthoringCatalog,
  appControlCatalogAcceptedOptionCount,
  appControlCatalogCanRender,
  appControlCatalogGroupCount,
  appControlCatalogSectionCount,
  appControlCatalogSettingCount,
  appControlCatalogSettings,
  appControlCatalogSourceOptionCount,
  buildAppControlEffectivePolicyPlan,
  decodeAppControlEffectivePolicy,
  decodeAppControlPolicyValueForCatalog,
  decodeAppControlUpdateCommandForCatalog,
} from '../src/app-control-catalog';
import { AppControlAuthoringCatalogSchema } from '../src/app-control-catalog-schema';

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
  readonly affectsSettings: readonly string[];
}

const SourceProposal = readSourceProposal();
const SourceSections = SourceProposal.authoringManifest.sections;
const SourceFields = SourceSections.flatMap((section) => section.fields);
const CatalogSettings = appControlCatalogSettings()
  .slice()
  .sort((left, right) => left.sourceOrder - right.sourceOrder);

describe('app-control policy catalog contracts', () => {
  registerSourceCaptureCases();
  registerHierarchyCases();
  registerRenderMetadataCases();
  registerCapabilityTruthCases();
  registerPolicyContractCases();
});

function registerSourceCaptureCases() {
  it('captures every Apps proposal setting and source option in source order', () => {
    expect(SourceProposal.schemaVersion).toBe(1);
    expect(SourceProposal.proposalStatus).toBe('worker-handoff-proposal-not-runtime-contract');
    expect(SourceSections.length).toBe(9);
    expect(SourceFields.length).toBe(29);
    expect(sourceOptionCount()).toBe(148);
    expect(SourceProposal.capabilityRegistry.capabilities.length).toBe(10);

    expect(AppControlAuthoringCatalogSchema.safeParse(BaselineAppControlAuthoringCatalog).success).toBe(true);
    expect(BaselineAppControlAuthoringCatalog.schemaVersion).toBe('v0.6');
    expect(BaselineAppControlAuthoringCatalog.settingCount).toBe(29);
    expect(appControlCatalogSettingCount()).toBe(29);
    expect(appControlCatalogSourceOptionCount()).toBe(148);
    expect(appControlCatalogAcceptedOptionCount()).toBe(154);
    expect(CatalogSettings.map((setting) => String(setting.settingId))).toEqual(
      SourceFields.map((field) => field.fieldId)
    );
    expect(CatalogSettings.map((setting) => setting.uiQuestionText)).toEqual(
      SourceFields.map((field) => field.question)
    );
    expect(CatalogSettings.map((setting) => String(setting.writesTo))).toEqual(
      SourceFields.map((field) => field.writesTo)
    );
  });
}

function registerHierarchyCases() {
  it('preserves side panel, lane, section, subgroup, setting, and option hierarchy', () => {
    expect(BaselineAppControlAuthoringCatalog.sidePanelCategory).toBe('apps');
    expect(BaselineAppControlAuthoringCatalog.sourceDocuments).toEqual([
      'docs/app-control-capability-guide.md',
      'docs/app-control-schema-proposal.md',
    ]);
    expect(appControlCatalogSectionCount()).toBe(9);
    expect(appControlCatalogGroupCount()).toBe(15);
    expect(BaselineAppControlAuthoringCatalog.sections.map((section) => String(section.sectionId))).toEqual(
      SourceSections.map((section) => section.sectionId)
    );
    expect(sectionFieldCounts()).toEqual({
      'app-management': 3,
      inventory: 4,
      'runtime-evidence': 5,
      'app-rules': 3,
      budgets: 3,
      enforcement: 4,
      'app-lifecycle': 2,
      approvals: 2,
      reports: 3,
    });
    expect(new Set(CatalogSettings.map((setting) => setting.policyLane))).toEqual(
      new Set(['rules', 'evidence', 'schedule', 'audit', 'enforcement', 'setup', 'approvals', 'reports'])
    );
    expect(settingsForSection('runtime-evidence').map((setting) => String(setting.groupId))).toEqual([
      'runtime-sources',
      'runtime-proof',
      'duration-proof',
      'runtime-proof',
      'data-minimization',
    ]);
    expect(settingsForSection('enforcement').map((setting) => String(setting.groupId))).toEqual([
      'strict-actions',
      'strict-actions',
      'strict-actions',
      'strict-actions',
    ]);
    expect(appControlCatalogCanRender()).toBe(true);
  });
}

function registerRenderMetadataCases() {
  it('keeps every setting renderable with stable ids, cards, options, scopes, and effects', () => {
    expect(new Set(CatalogSettings.map((setting) => String(setting.settingId))).size).toBe(29);
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
      CatalogSettings.filter((setting) => setting.validationRules.length < 2).map((setting) =>
        String(setting.settingId)
      )
    ).toEqual([]);
    expect(
      CatalogSettings.filter((setting) => optionIdsForSetting(setting).size !== setting.acceptedOptions.length).map(
        (setting) => String(setting.settingId)
      )
    ).toEqual([]);
    expect(countSettingsBy('cardKind')).toEqual({
      'multi-choice-many': 9,
      'number-card': 2,
      'retention-card': 2,
      'single-choice-compact': 6,
      'single-choice-many': 7,
      toggle: 3,
    });
    expect(countSettingsBy('effectStatus')).toEqual({
      'already-represented': 13,
      'manual-required': 3,
      'needs-effect-wiring': 4,
      'permission-limited': 1,
      'permission-required': 1,
      'proof-required': 7,
    });
    expect(countSettingsBy('capabilityState')).toEqual({
      available: 15,
      'manual-required': 3,
      'permission-limited': 2,
      'permission-required': 1,
      protected: 8,
    });
    expect(countSettingsBy('runtimeOwner')).toEqual({
      'child-agent': 13,
      'os-adapter': 5,
      'parent-domain': 7,
      'parent-owned-storage': 3,
      'rust-service': 1,
    });
  });
}

function registerCapabilityTruthCases() {
  it('keeps unknown apps unknown and broad app blocking manual-required', () => {
    const defaultPosture = settingById('app.defaultPosture');
    const unknownHandling = settingById('inventory.unknownHandling');
    const unknownRule = settingById('rules.defaultUnknownRule');
    const enforcementActions = settingById('enforcement.allowedActions');
    const enforcementSettings = CatalogSettings.filter((setting) => setting.policyLane === 'enforcement');

    expect(defaultPosture.effectStatus).toBe('proof-required');
    expect(defaultPosture.runtimeOwner).toBe('child-agent');
    expect(defaultPosture.acceptedOptions.find((option) => option.value === 'block')?.meaning).toContain(
      'platform capability proof'
    );
    expect(unknownHandling.capabilityRequirement).toBe('unknown app policy state');
    expect(unknownHandling.unsafeOrUnsupportedFallback).toContain('Never promote unknown apps');
    expect(unknownRule.effectStatus).toBe('proof-required');
    expect(unknownRule.acceptedOptions.map((option) => option.value)).toContain('block-if-supported');
    expect(unknownRule.acceptedOptions.find((option) => option.defaultSelected)?.value).toBe('ask-first-run');
    expect(enforcementActions.effectStatus).toBe('manual-required');
    expect(enforcementActions.runtimeOwner).toBe('os-adapter');
    expect(enforcementActions.capabilityState).toBe('manual-required');
    expect(enforcementActions.proofRequirement).toContain('Broad app blocking remains manual-required');
    expect(enforcementSettings.map((setting) => setting.runtimeOwner)).not.toContain('portal-only');
  });

  it('maps the capability registry into typed capability states without over-claiming platform control', () => {
    const sourceCapabilities = SourceProposal.capabilityRegistry.capabilities;
    const knownSettingIds = new Set(CatalogSettings.map((setting) => String(setting.settingId)));

    expect(AppControlCapabilities.length).toBe(10);
    expect(AppControlCapabilities.map((capability) => String(capability.capabilityId))).toEqual(
      sourceCapabilities.map((capability) => capability.capabilityId)
    );
    expect(capabilityById('windows-app-inventory').state).toBe('available');
    expect(capabilityById('windows-broad-app-blocking').state).toBe('manual-required');
    expect(capabilityById('android-package-lifecycle').state).toBe('manual-required');
    expect(
      AppControlCapabilities.flatMap((capability) =>
        capability.affectsSettings.map((settingId) => String(settingId))
      ).filter((settingId) => !knownSettingIds.has(settingId))
    ).toEqual([]);
  });
}

function registerPolicyContractCases() {
  registerPolicyDecodeCases();
  registerPolicyRejectionCases();
}

function registerPolicyDecodeCases() {
  it('decodes policy values, update commands, and effective plans against known catalog settings', () => {
    const policy = decodeAppControlPolicyValueForCatalog({
      documentId: 'app-policy-1',
      policyKind: 'app-control',
      schemaVersion: 'v0.6',
      revision: 2,
      targetDeviceId: 'device-1',
      updatedAt: '2026-05-28T00:00:00.000Z',
      settings: [
        { settingId: 'app.enabled', value: true },
        { settingId: 'app.defaultPosture', value: 'observe' },
        { settingId: 'inventory.sources', value: ['os-installed-apps', 'desktop-shortcuts'] },
      ],
    });
    const plans = buildAppControlEffectivePolicyPlan(policy);
    const effective = decodeAppControlEffectivePolicy({
      documentId: 'app-effective-1',
      compiledFromPolicyId: 'app-policy-1',
      schemaVersion: 'v0.6',
      effectivePolicyHash: 'hash-1',
      compiledAt: '2026-05-28T00:01:00.000Z',
      runtimeOwner: 'child-agent',
      plans,
    });
    const command = decodeAppControlUpdateCommandForCatalog({
      commandType: 'app-control.patch',
      targetDeviceId: 'device-1',
      expectedRevision: 2,
      patch: [{ op: 'replace', path: '/appPolicy/defaultPosture', value: 'warn' }],
    });

    expect(policy.settings.length).toBe(3);
    expect(effective.plans.map((plan) => plan.effectStatus)).toEqual([
      'already-represented',
      'proof-required',
      'permission-limited',
    ]);
    expect(command.patch[0]?.path).toBe('/appPolicy/defaultPosture');
  });
}

function registerPolicyRejectionCases() {
  it('rejects unknown settings, duplicate settings, unknown update paths, and invalid enum values', () => {
    expect(() =>
      decodeAppControlPolicyValueForCatalog({
        documentId: 'app-policy-1',
        policyKind: 'app-control',
        schemaVersion: 'v0.6',
        revision: 1,
        targetDeviceId: 'device-1',
        updatedAt: '2026-05-28T00:00:00.000Z',
        settings: [{ settingId: 'app.unknown', value: true }],
      })
    ).toThrow('Unknown app control setting id');
    expect(() =>
      decodeAppControlPolicyValueForCatalog({
        documentId: 'app-policy-1',
        policyKind: 'app-control',
        schemaVersion: 'v0.6',
        revision: 1,
        targetDeviceId: 'device-1',
        updatedAt: '2026-05-28T00:00:00.000Z',
        settings: [
          { settingId: 'app.enabled', value: true },
          { settingId: 'app.enabled', value: false },
        ],
      })
    ).toThrow('Duplicate app control setting id');
    expect(() =>
      decodeAppControlUpdateCommandForCatalog({
        commandType: 'app-control.patch',
        targetDeviceId: 'device-1',
        expectedRevision: 1,
        patch: [{ op: 'replace', path: '/appPolicy/unknown', value: true }],
      })
    ).toThrow('Unknown app control writesTo path');

    const invalidCatalog = JSON.parse(JSON.stringify(BaselineAppControlAuthoringCatalog)) as {
      sidePanelCategory: string;
    };
    invalidCatalog.sidePanelCategory = 'browser';
    expect(AppControlAuthoringCatalogSchema.safeParse(invalidCatalog).success).toBe(false);
  });
}

function readSourceProposal(): SourceProposal {
  const markdown = readFileSync(join(process.cwd(), '..', '..', 'docs', 'app-control-schema-proposal.md'), 'utf8');
  const jsonBlock = markdown.match(/```json\n([\s\S]*?)\n```/u);
  if (jsonBlock === null) {
    throw new Error('Missing JSON block in app-control schema proposal.');
  }
  return JSON.parse(jsonBlock[1] ?? '{}') as SourceProposal;
}

function sourceOptionCount() {
  return SourceFields.reduce((count, field) => count + (field.options?.length ?? 0), 0);
}

function sectionFieldCounts() {
  return Object.fromEntries(
    BaselineAppControlAuthoringCatalog.sections.map((section) => [
      String(section.sectionId),
      section.groups.reduce((count, group) => count + group.settings.length, 0),
    ])
  );
}

function settingsForSection(sectionId: string) {
  return CatalogSettings.filter((setting) => String(setting.sectionId) === sectionId);
}

function settingById(settingId: string) {
  const setting = CatalogSettings.find((candidate) => String(candidate.settingId) === settingId);
  if (setting === undefined) {
    throw new Error(`Missing setting ${settingId}`);
  }
  return setting;
}

function capabilityById(capabilityId: string) {
  const capability = AppControlCapabilities.find((candidate) => String(candidate.capabilityId) === capabilityId);
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
  return counts;
}

function optionIdsForSetting(setting: (typeof CatalogSettings)[number]) {
  return new Set(setting.acceptedOptions.map((option) => String(option.optionId)));
}
