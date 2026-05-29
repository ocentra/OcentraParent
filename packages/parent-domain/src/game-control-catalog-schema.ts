import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

import { ParentContractSchemaVersionSchema } from './reference-primitives';

const NonEmptyGameControlText = Schema.String.pipe(Schema.minLength(1));

export const GameControlCatalogIdSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlCatalogId'))
);
export const GameControlSectionIdSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlSectionId'))
);
export const GameControlGroupIdSchema = withParser(NonEmptyGameControlText.pipe(Schema.brand('GameControlGroupId')));
export const GameControlSettingIdSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlSettingId'))
);
export const GameControlOptionIdSchema = withParser(NonEmptyGameControlText.pipe(Schema.brand('GameControlOptionId')));
export const GameControlEffectKeySchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlEffectKey'))
);
export const GameControlWritePathSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlWritePath'))
);
export const GameControlRuleIdSchema = withParser(NonEmptyGameControlText.pipe(Schema.brand('GameControlRuleId')));
export const GameControlCapabilityIdSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlCapabilityId'))
);
export const GameControlPolicyDocumentIdSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlPolicyDocumentId'))
);
export const GameControlPolicyRevisionSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlPolicyRevision'))
);
export const GameControlPolicyHashSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlPolicyHash'))
);
export const GameControlCommandIdSchema = withParser(
  NonEmptyGameControlText.pipe(Schema.brand('GameControlCommandId'))
);

const GameControlPolicyKindSchema = withParser(Schema.Literal('game-control'));

export const GameControlSourceDocumentSchema = withParser(
  Schema.Literal('docs/game-control-capability-guide.md', 'docs/game-control-schema-proposal.md')
);

const GameControlSidePanelCategorySchema = withParser(Schema.Literal('games'));

const GameControlPolicyLaneSchema = withParser(
  Schema.Literal('rules', 'schedule', 'approvals', 'enforcement', 'audit', 'evidence', 'reports', 'data')
);

const GameControlControlTypeSchema = withParser(
  Schema.Literal(
    'toggle',
    'single-choice',
    'multi-choice',
    'number',
    'duration',
    'schedule',
    'rule-list',
    'target-list',
    'retention',
    'action-list',
    'read-only-status'
  )
);

const GameControlUiCardTypeSchema = withParser(
  Schema.Literal(
    'compact-single-choice',
    'many-option-single-choice',
    'normal-multi-choice',
    'many-option-multi-choice',
    'toggle-card',
    'schedule-card',
    'rule-list-card',
    'target-list-card',
    'retention-card',
    'status-card'
  )
);

const GameControlEffectStatusSchema = withParser(
  Schema.Literal(
    'already-represented',
    'needs-wiring',
    'manual-required',
    'unavailable',
    'future-gap',
    'degraded',
    'permission-required',
    'permission-limited',
    'proof-required'
  )
);

const GameControlRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'portal-only',
    'parent-domain',
    'agent-protocol',
    'rust-service',
    'child-agent',
    'os-adapter',
    'manual-proof',
    'parent-owned-storage',
    'local-ai-runtime'
  )
);

const GameControlCapabilityStateSchema = withParser(
  Schema.Literal(
    'available',
    'disabled',
    'unsupported',
    'permission-required',
    'permission-limited',
    'protected',
    'degraded',
    'manual-required',
    'future-gap',
    'unavailable'
  )
);

const GameControlTargetScopeSchema = withParser(
  Schema.Literal(
    'family',
    'per-child',
    'per-device',
    'per-platform',
    'per-app',
    'per-game',
    'per-browser',
    'per-network'
  )
);

const GameControlEffectModeSchema = withParser(
  Schema.Literal('off', 'observe', 'dry-run', 'warn', 'notify', 'ask', 'limit', 'block', 'enforce', 'audit-only')
);

const GameControlUpdateCommandTypeSchema = withParser(
  Schema.Literal(
    'game-policy.get.requested',
    'game-policy.preview.requested',
    'game-policy.patch.requested',
    'game-policy.replace.requested',
    'game-policy.acknowledge.requested',
    'game-policy.reject.requested',
    'game-policy.rollback.requested',
    'game-policy.capability-refresh.requested'
  )
);

const GameControlOptionSchema = withParser(
  Schema.Struct({
    optionId: GameControlOptionIdSchema,
    label: NonEmptyGameControlText,
    value: NonEmptyGameControlText,
    originalSourceText: NonEmptyGameControlText,
    meaning: Schema.Union(NonEmptyGameControlText, Schema.Null),
    defaultSelected: Schema.Boolean,
  })
);

const GameControlLayoutHintsSchema = withParser(
  Schema.Struct({
    preferredColumnSpan: Schema.Number,
    collapsible: Schema.Boolean,
    searchableOptions: Schema.Boolean,
    optionGroupCount: Schema.Number,
    showAsMatrixWhenLarge: Schema.Boolean,
    showSelectedCount: Schema.Boolean,
  })
);

const GameControlConditionSchema = withParser(
  Schema.Struct({
    ruleId: GameControlRuleIdSchema,
    description: NonEmptyGameControlText,
  })
);

const GameControlSettingSchema = withParser(
  Schema.Struct({
    sidePanelCategory: GameControlSidePanelCategorySchema,
    policyLane: GameControlPolicyLaneSchema,
    sourceSection: GameControlSectionIdSchema,
    sourceGroup: GameControlGroupIdSchema,
    sectionId: GameControlSectionIdSchema,
    groupId: GameControlGroupIdSchema,
    settingId: GameControlSettingIdSchema,
    sourceDocument: GameControlSourceDocumentSchema,
    sourceHeadingPath: Schema.Array(NonEmptyGameControlText),
    originalSourceText: NonEmptyGameControlText,
    uiQuestionText: NonEmptyGameControlText,
    helperText: Schema.Union(NonEmptyGameControlText, Schema.Null),
    displayOrder: Schema.Number,
    controlType: GameControlControlTypeSchema,
    uiCardType: GameControlUiCardTypeSchema,
    layoutHints: GameControlLayoutHintsSchema,
    acceptedOptions: Schema.Array(GameControlOptionSchema),
    targetScopeOptions: Schema.Array(GameControlTargetScopeSchema),
    effectModeOptions: Schema.Array(GameControlEffectModeSchema),
    writesTo: GameControlWritePathSchema,
    effectKey: GameControlEffectKeySchema,
    effectStatus: GameControlEffectStatusSchema,
    runtimeOwner: GameControlRuntimeOwnerSchema,
    capabilityState: GameControlCapabilityStateSchema,
    capabilityRequirement: Schema.Union(NonEmptyGameControlText, Schema.Null),
    proofRequirement: Schema.Union(NonEmptyGameControlText, Schema.Null),
    visibilityConditions: Schema.Array(GameControlConditionSchema),
    enabledConditions: Schema.Array(GameControlConditionSchema),
    validationRules: Schema.Array(GameControlConditionSchema),
    unsafeOrUnsupportedFallback: Schema.Union(NonEmptyGameControlText, Schema.Null),
  })
);

const GameControlGroupSchema = withParser(
  Schema.Struct({
    groupId: GameControlGroupIdSchema,
    title: NonEmptyGameControlText,
    displayOrder: Schema.Number,
    settings: Schema.Array(GameControlSettingSchema),
  })
);

const GameControlSectionSchema = withParser(
  Schema.Struct({
    sectionId: GameControlSectionIdSchema,
    title: NonEmptyGameControlText,
    purpose: NonEmptyGameControlText,
    displayOrder: Schema.Number,
    policyLane: GameControlPolicyLaneSchema,
    groups: Schema.Array(GameControlGroupSchema),
  })
);

const GameControlLaneSchema = withParser(
  Schema.Struct({
    laneId: GameControlPolicyLaneSchema,
    title: NonEmptyGameControlText,
    displayOrder: Schema.Number,
    sections: Schema.Array(GameControlSectionSchema),
  })
);

const GameControlCapabilityTruthSchema = withParser(
  Schema.Struct({
    truthId: GameControlRuleIdSchema,
    sourceDocument: GameControlSourceDocumentSchema,
    sourceHeadingPath: Schema.Array(NonEmptyGameControlText),
    originalSourceText: NonEmptyGameControlText,
    appliesToSettingIds: Schema.Array(GameControlSettingIdSchema),
    capabilityState: GameControlCapabilityStateSchema,
  })
);

const GameControlCapabilityEntrySchema = withParser(
  Schema.Struct({
    capabilityId: GameControlCapabilityIdSchema,
    state: GameControlCapabilityStateSchema,
    proofRequirement: NonEmptyGameControlText,
    affectsSettingIds: Schema.Array(GameControlSettingIdSchema),
  })
);

export const GameControlCapabilityRegistrySchema = withParser(
  Schema.Struct({
    registryId: GameControlCatalogIdSchema,
    sidePanelCategory: GameControlSidePanelCategorySchema,
    sourceDocument: GameControlSourceDocumentSchema,
    capabilities: Schema.Array(GameControlCapabilityEntrySchema),
  })
);

export const GameControlAuthoringManifestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    manifestId: GameControlCatalogIdSchema,
    policyKind: GameControlPolicyKindSchema,
    sidePanelCategory: GameControlSidePanelCategorySchema,
    sourceDocuments: Schema.Array(GameControlSourceDocumentSchema),
    settingCount: Schema.Number,
    acceptedOptionCount: Schema.Number,
    targetScopeOptions: Schema.Array(GameControlTargetScopeSchema),
    effectModeOptions: Schema.Array(GameControlEffectModeSchema),
    lanes: Schema.Array(GameControlLaneSchema),
    capabilityTruths: Schema.Array(GameControlCapabilityTruthSchema),
    capabilityRegistry: GameControlCapabilityRegistrySchema,
  })
);

export const GameControlPolicyValueSettingSchema = withParser(
  Schema.Struct({
    settingId: GameControlSettingIdSchema,
    writesTo: GameControlWritePathSchema,
    selectedOptionIds: Schema.Array(GameControlOptionIdSchema),
    booleanValue: Schema.Union(Schema.Boolean, Schema.Null),
    numericValue: Schema.Union(Schema.Number, Schema.Null),
    ruleItemCount: Schema.Number,
  })
);

export const GameControlPolicyValueDocumentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    policyKind: GameControlPolicyKindSchema,
    documentId: GameControlPolicyDocumentIdSchema,
    revision: GameControlPolicyRevisionSchema,
    manifestId: GameControlCatalogIdSchema,
    targetScopes: Schema.Array(GameControlTargetScopeSchema),
    settings: Schema.Array(GameControlPolicyValueSettingSchema),
  })
);

export const GameControlEffectivePolicySettingSchema = withParser(
  Schema.Struct({
    settingId: GameControlSettingIdSchema,
    effectKey: GameControlEffectKeySchema,
    effectStatus: GameControlEffectStatusSchema,
    runtimeOwner: GameControlRuntimeOwnerSchema,
    capabilityState: GameControlCapabilityStateSchema,
    proofRequirement: Schema.Union(NonEmptyGameControlText, Schema.Null),
    fallbackDecision: NonEmptyGameControlText,
  })
);

export const GameControlEffectivePolicyDocumentSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    policyKind: GameControlPolicyKindSchema,
    documentId: GameControlPolicyDocumentIdSchema,
    compiledFromDocumentId: GameControlPolicyDocumentIdSchema,
    compiledFromRevision: GameControlPolicyRevisionSchema,
    effectivePolicyHash: GameControlPolicyHashSchema,
    targetScopes: Schema.Array(GameControlTargetScopeSchema),
    settings: Schema.Array(GameControlEffectivePolicySettingSchema),
  })
);

export const GameControlPolicyUpdateCommandSchema = withParser(
  Schema.Struct({
    commandId: GameControlCommandIdSchema,
    commandType: GameControlUpdateCommandTypeSchema,
    policyKind: GameControlPolicyKindSchema,
    targetScopes: Schema.Array(GameControlTargetScopeSchema),
    expectedRevision: Schema.Union(GameControlPolicyRevisionSchema, Schema.Null),
    purpose: NonEmptyGameControlText,
  })
);

export type GameControlPolicyLane = Infer<typeof GameControlPolicyLaneSchema>;
export type GameControlControlType = Infer<typeof GameControlControlTypeSchema>;
export type GameControlUiCardType = Infer<typeof GameControlUiCardTypeSchema>;
export type GameControlEffectStatus = Infer<typeof GameControlEffectStatusSchema>;
export type GameControlRuntimeOwner = Infer<typeof GameControlRuntimeOwnerSchema>;
export type GameControlCapabilityState = Infer<typeof GameControlCapabilityStateSchema>;
export type GameControlTargetScope = Infer<typeof GameControlTargetScopeSchema>;
export type GameControlEffectMode = Infer<typeof GameControlEffectModeSchema>;
export type GameControlOption = Infer<typeof GameControlOptionSchema>;
export type GameControlSetting = Infer<typeof GameControlSettingSchema>;
export type GameControlGroup = Infer<typeof GameControlGroupSchema>;
export type GameControlSection = Infer<typeof GameControlSectionSchema>;
export type GameControlLane = Infer<typeof GameControlLaneSchema>;
export type GameControlCapabilityTruth = Infer<typeof GameControlCapabilityTruthSchema>;
export type GameControlAuthoringManifest = Infer<typeof GameControlAuthoringManifestSchema>;
export type GameControlPolicyValueDocument = Infer<typeof GameControlPolicyValueDocumentSchema>;
export type GameControlEffectivePolicyDocument = Infer<typeof GameControlEffectivePolicyDocumentSchema>;

export type GameControlCatalogOptionSeed = readonly [value: string, label: string, meaning: string | null];

export interface GameControlCatalogSettingSeed {
  readonly sectionId: string;
  readonly sectionTitle: string;
  readonly sectionPurpose: string;
  readonly sectionOrder: number;
  readonly groupId: string;
  readonly groupTitle: string;
  readonly groupOrder: number;
  readonly settingId: string;
  readonly settingOrder: number;
  readonly controlType: string;
  readonly writesTo: string;
  readonly question: string;
  readonly defaultValue: unknown;
  readonly options: readonly GameControlCatalogOptionSeed[];
}
