import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

import {
  BaselineGameControlAuthoringManifest,
  BaselineGameControlEffectivePolicyDocument,
  BaselineGameControlPolicyUpdateCommands,
  BaselineGameControlPolicyValueDocument,
  gameControlCapabilityStateCount,
  gameControlCatalogAcceptedOptionCount,
  gameControlCatalogGroups,
  gameControlCatalogSections,
  gameControlCatalogSettings,
  gameControlCatalogSettingsByCardType,
  gameControlCatalogSettingsByEffectStatus,
  parseCompleteGameControlPolicyValueDocument,
} from '../../src/game-control-catalog';
import {
  GameControlAuthoringManifestSchema,
  GameControlPolicyUpdateCommandSchema,
  GameControlPolicyValueDocumentSchema,
} from '../../src/game-control-catalog-schema';

interface SourceField {
  readonly sectionId: string;
  readonly sectionTitle: string;
  readonly fieldId: string;
  readonly kind: string;
  readonly question: string;
  readonly optionValues: readonly string[];
}

const SourceFields = readSourceFields();
const CatalogSettings = gameControlCatalogSettings()
  .slice()
  .sort((left, right) => left.displayOrder - right.displayOrder);

describe('game-control policy catalog', () => {
  registerSourceCaptureCases();
  registerHierarchyCases();
  registerOptionMetadataCases();
  registerCapabilityTruthCases();
  registerReportCountCases();
  registerPolicyDocumentCases();
});

function registerSourceCaptureCases() {
  it('captures every Games authoring manifest section, setting, and source option', () => {
    expect(SourceFields.length).toBe(33);
    expect(sourceSectionIds()).toEqual([
      'game-management',
      'inventory',
      'session-evidence',
      'native-games',
      'launcher-games',
      'browser-cloud-games',
      'game-rules',
      'budgets',
      'approvals',
      'reports',
      'audit',
    ]);
    expect(sourceOptionCount()).toBe(213);
    expect(BaselineGameControlAuthoringManifest.settingCount).toBe(33);
    expect(BaselineGameControlAuthoringManifest.acceptedOptionCount).toBe(213);
    expect(gameControlCatalogAcceptedOptionCount()).toBe(217);
    expect(CatalogSettings.map((setting) => setting.settingId)).toEqual(SourceFields.map((field) => field.fieldId));
    expect(CatalogSettings.map((setting) => setting.uiQuestionText)).toEqual(
      SourceFields.map((field) => field.question)
    );
  });
}

function registerHierarchyCases() {
  it('preserves the source hierarchy for C/UI rendering', () => {
    expect(BaselineGameControlAuthoringManifest.sidePanelCategory).toBe('games');
    expect(BaselineGameControlAuthoringManifest.sourceDocuments).toEqual([
      'docs/game-control-capability-guide.md',
      'docs/game-control-schema-proposal.md',
    ]);
    expect(BaselineGameControlAuthoringManifest.lanes.map((lane) => lane.laneId)).toEqual([
      'rules',
      'schedule',
      'approvals',
      'enforcement',
      'audit',
      'evidence',
      'reports',
    ]);
    expect(gameControlCatalogSections().length).toBe(11);
    expect(gameControlCatalogGroups().length).toBe(11);
    expect(gameControlCatalogSections().map((section) => [section.sectionId, section.groups.length])).toEqual([
      ['game-management', 1],
      ['inventory', 1],
      ['game-rules', 1],
      ['budgets', 1],
      ['approvals', 1],
      ['native-games', 1],
      ['launcher-games', 1],
      ['browser-cloud-games', 1],
      ['audit', 1],
      ['session-evidence', 1],
      ['reports', 1],
    ]);
  });
}

function registerOptionMetadataCases() {
  it('keeps option values, stable ids, and many-option render metadata', () => {
    const launcherKinds = settingById('launchers.supportedKinds');
    const strictActions = settingById('nativeGames.strictActions');
    const browserEvidence = settingById('browserCloud.allowedEvidence');

    expect(GameControlAuthoringManifestSchema.safeParse(BaselineGameControlAuthoringManifest).success).toBe(true);
    expect(new Set(CatalogSettings.map((setting) => setting.settingId)).size).toBe(33);
    expect(
      CatalogSettings.filter((setting) => setting.controlType === undefined || setting.uiCardType === undefined)
    ).toEqual([]);
    expect(launcherKinds.acceptedOptions.map((option) => option.value)).toEqual(
      sourceFieldById('launchers.supportedKinds').optionValues
    );
    expect(launcherKinds.uiCardType).toBe('many-option-multi-choice');
    expect(launcherKinds.layoutHints).toEqual({
      preferredColumnSpan: 2,
      collapsible: true,
      searchableOptions: true,
      optionGroupCount: 3,
      showAsMatrixWhenLarge: true,
      showSelectedCount: true,
    });
    expect(strictActions.acceptedOptions.map((option) => option.value)).toContain('terminate-accessible-process');
    expect(browserEvidence.acceptedOptions.map((option) => option.value)).toContain('network-flow-service-hint');
  });
}

function registerCapabilityTruthCases() {
  it('marks capability truth boundaries honestly', () => {
    const launcherKinds = settingById('launchers.supportedKinds');
    const browserMode = settingById('browserCloud.mode');
    const networkHint = settingById('browserCloud.allowedEvidence');
    const strictActions = settingById('nativeGames.strictActions');
    const custody = settingById('custody.allowedUses');

    expect(launcherKinds.effectStatus).toBe('manual-required');
    expect(launcherKinds.proofRequirement).toBe(
      'Launcher proof must not treat launcher-only activity as active gameplay.'
    );
    expect(browserMode.effectStatus).toBe('proof-required');
    expect(browserMode.proofRequirement).toBe(
      'Browser-game title proof requires managed browser URL/title evidence or explicit platform integration.'
    );
    expect(networkHint.capabilityRequirement).toBe('managed-browser-boundary-or-cloud-client-surface-proof');
    expect(strictActions.capabilityState).toBe('protected');
    expect(strictActions.unsafeOrUnsupportedFallback).toBe(
      'Require explicit proof before strict enforcement; otherwise use observe, ask, or audit-only fallback.'
    );
    expect(custody.runtimeOwner).toBe('parent-owned-storage');
    expect(BaselineGameControlAuthoringManifest.capabilityTruths.map((truth) => truth.originalSourceText)).toContain(
      'Network-only evidence can suggest a service or domain but usually cannot prove exact title.'
    );
  });
}

function registerReportCountCases() {
  it('provides expected report counts for card types, effect states, and capability states', () => {
    expect(gameControlCatalogSettingsByCardType()).toEqual({
      'compact-single-choice': 2,
      'many-option-multi-choice': 15,
      'many-option-single-choice': 9,
      'retention-card': 1,
      'rule-list-card': 1,
      'status-card': 3,
      'toggle-card': 2,
    });
    expect(gameControlCatalogSettingsByEffectStatus()).toEqual({
      'already-represented': 5,
      degraded: 3,
      'manual-required': 3,
      'needs-wiring': 15,
      'proof-required': 7,
    });
    expect(gameControlCapabilityStateCount()).toEqual({
      available: 4,
      degraded: 1,
      'manual-required': 4,
    });
  });
}

function registerPolicyDocumentCases() {
  it('validates complete policy values, effective policy documents, and update commands', () => {
    const missingSetting = {
      ...BaselineGameControlPolicyValueDocument,
      settings: BaselineGameControlPolicyValueDocument.settings.slice(1),
    };
    const invalidOption = JSON.parse(JSON.stringify(BaselineGameControlPolicyValueDocument)) as {
      settings: [{ selectedOptionIds: string[] }];
    };
    invalidOption.settings[0].selectedOptionIds = ['game-control-option-invalid'];
    const invalidCommand = {
      ...BaselineGameControlPolicyUpdateCommands[0],
      commandType: 'game-policy.delete.requested',
    };

    expect(GameControlPolicyValueDocumentSchema.safeParse(BaselineGameControlPolicyValueDocument).success).toBe(true);
    expect(parseCompleteGameControlPolicyValueDocument(BaselineGameControlPolicyValueDocument)).toEqual(
      BaselineGameControlPolicyValueDocument
    );
    expect(() => parseCompleteGameControlPolicyValueDocument(missingSetting)).toThrow(
      'Game policy value document must include every authoring manifest setting.'
    );
    expect(() => parseCompleteGameControlPolicyValueDocument(invalidOption)).toThrow(
      'Invalid game policy option game-control-option-invalid for game.enabled.'
    );
    expect(BaselineGameControlEffectivePolicyDocument.settings.length).toBe(33);
    expect(BaselineGameControlPolicyUpdateCommands.map((command) => command.commandType)).toEqual([
      'game-policy.get.requested',
      'game-policy.preview.requested',
      'game-policy.patch.requested',
      'game-policy.replace.requested',
      'game-policy.acknowledge.requested',
      'game-policy.reject.requested',
      'game-policy.rollback.requested',
      'game-policy.capability-refresh.requested',
    ]);
    expect(GameControlPolicyUpdateCommandSchema.safeParse(invalidCommand).success).toBe(false);
  });
}

function readSourceFields(): SourceField[] {
  const proposal = JSON.parse(sourceProposalBlock()) as {
    authoringManifest: {
      sections: Array<{
        sectionId: string;
        title: string;
        fields: Array<{
          fieldId: string;
          kind: string;
          question: string;
          options?: Array<string | { value?: string; label?: string }>;
        }>;
      }>;
    };
  };
  return proposal.authoringManifest.sections.flatMap((section) =>
    section.fields.map((field) => ({
      sectionId: section.sectionId,
      sectionTitle: section.title,
      fieldId: field.fieldId,
      kind: field.kind,
      question: field.question,
      optionValues: (field.options ?? []).map((option) =>
        typeof option === 'string' ? option : (option.value ?? option.label ?? '')
      ),
    }))
  );
}

function sourceProposalBlock() {
  const docPath = join(process.cwd(), '..', '..', 'docs', 'game-control-schema-proposal.md');
  const doc = readFileSync(docPath, 'utf8');
  const match = /```json\n([\s\S]*?)\n```/u.exec(doc);
  if (match === null) {
    throw new Error('Missing game-control proposal JSON block.');
  }
  return match[1] ?? '';
}

function sourceSectionIds() {
  return [...new Set(SourceFields.map((field) => field.sectionId))];
}

function sourceOptionCount() {
  return SourceFields.reduce((count, field) => count + field.optionValues.length, 0);
}

function sourceFieldById(fieldId: string) {
  const field = SourceFields.find((candidate) => candidate.fieldId === fieldId);
  if (field === undefined) {
    throw new Error(`Missing source field ${fieldId}`);
  }
  return field;
}

function settingById(settingId: string) {
  const setting = CatalogSettings.find((candidate) => candidate.settingId === settingId);
  if (setting === undefined) {
    throw new Error(`Missing game control setting ${settingId}`);
  }
  return setting;
}
