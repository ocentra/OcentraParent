import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

import { ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const ScreenControlCatalogIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenControlCatalogId')
);
export const ScreenControlSectionIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenControlSectionId')
);
export const ScreenControlSettingIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenControlSettingId')
);
export const ScreenControlOptionIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenControlOptionId')
);
export const ScreenControlRuleIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenControlRuleId')
);

export const ScreenControlCatalogSourceKindSchema = withParser(
  Schema.Literal(
    'capability-guide-bullet',
    'capability-matrix-row',
    'schema-proposal-bullet',
    'authoring-field',
    'rendering-rule',
    'control-kind',
    'condition-kind',
    'capability-registry-entry',
    'capability-state-meaning',
    'update-command',
    'agent-rule',
    'policy-fallback',
    'effective-proof-requirement',
    'effective-fallback',
    'effective-rule',
    'visible-category-target'
  )
);

export const ScreenControlCatalogControlKindSchema = withParser(
  Schema.Literal(
    'toggle',
    'single-choice',
    'multi-choice',
    'number',
    'duration',
    'schedule',
    'threshold',
    'retention',
    'target-list',
    'rule-list',
    'action-list',
    'read-only-status'
  )
);

export const ScreenControlCatalogCardKindSchema = withParser(
  Schema.Literal(
    'toggle',
    'single-choice-compact',
    'single-choice-many',
    'multi-choice-normal',
    'multi-choice-many',
    'number-card',
    'duration-card',
    'schedule-card',
    'threshold-card',
    'retention-card',
    'rule-list-card',
    'target-list-card',
    'status-card'
  )
);

export const ScreenControlCatalogSelectionModeSchema = withParser(
  Schema.Literal('single', 'multi', 'numeric', 'schedule', 'status')
);

export const ScreenControlCatalogUiTabSchema = withParser(
  Schema.Literal(
    'rules',
    'schedule',
    'approvals',
    'enforcement',
    'audit',
    'evidence',
    'reports',
    'data',
    'ai',
    'setup',
    'platform'
  )
);

export const ScreenControlCatalogSectionKindSchema = withParser(
  Schema.Literal(
    'capability-guide-section',
    'proposal-authoring-section',
    'proposal-runtime-section',
    'capability-registry-section',
    'update-protocol-section'
  )
);

export const ScreenControlCatalogEffectStatusSchema = withParser(
  Schema.Literal(
    'already-represented',
    'needs-effect-wiring',
    'manual-required',
    'unavailable',
    'future-gap',
    'degraded',
    'permission-required',
    'permission-limited',
    'proof-required'
  )
);

export const ScreenControlCatalogRuntimeOwnerSchema = withParser(
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

export const ScreenControlCatalogCapabilityStateSchema = withParser(
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

export const ScreenControlCatalogSidePanelCategorySchema = withParser(Schema.Literal('screen'));

export const ScreenControlCatalogOptionSchema = withParser(
  Schema.Struct({
    optionId: ScreenControlOptionIdSchema,
    label: NonEmptyStringSchema,
    value: NonEmptyStringSchema,
    originalSourceText: NonEmptyStringSchema,
    meaning: Schema.Union(NonEmptyStringSchema, Schema.Null),
    defaultSelected: Schema.Boolean,
  })
);

export const ScreenControlCatalogLayoutHintsSchema = withParser(
  Schema.Struct({
    preferredColumnSpan: Schema.Number,
    collapsible: Schema.Boolean,
    searchableOptions: Schema.Boolean,
    optionGroupCount: Schema.Number,
    showAsMatrixWhenLarge: Schema.Boolean,
    showSelectedCount: Schema.Boolean,
  })
);

export const ScreenControlCatalogRuleSchema = withParser(
  Schema.Struct({
    ruleId: ScreenControlRuleIdSchema,
    description: NonEmptyStringSchema,
  })
);

export const ScreenControlCatalogSettingSchema = withParser(
  Schema.Struct({
    sidePanelCategory: ScreenControlCatalogSidePanelCategorySchema,
    sourceKind: ScreenControlCatalogSourceKindSchema,
    sectionId: ScreenControlSectionIdSchema,
    groupId: ScreenControlSectionIdSchema,
    settingId: ScreenControlSettingIdSchema,
    sourceDocument: NonEmptyStringSchema,
    sourceHeadingPath: Schema.Array(NonEmptyStringSchema),
    sourceSection: ScreenControlSectionIdSchema,
    sourceGroup: ScreenControlSectionIdSchema,
    sourceOrder: Schema.Number,
    sourceLine: Schema.Number,
    sourceText: NonEmptyStringSchema,
    originalSourceText: NonEmptyStringSchema,
    question: NonEmptyStringSchema,
    uiQuestionText: NonEmptyStringSchema,
    helperText: Schema.Union(NonEmptyStringSchema, Schema.Null),
    displayOrder: Schema.Number,
    uiTab: ScreenControlCatalogUiTabSchema,
    policyLane: ScreenControlCatalogUiTabSchema,
    selectionMode: ScreenControlCatalogSelectionModeSchema,
    cardKind: ScreenControlCatalogCardKindSchema,
    controlKind: ScreenControlCatalogControlKindSchema,
    layoutHints: ScreenControlCatalogLayoutHintsSchema,
    acceptedOptions: Schema.Array(ScreenControlCatalogOptionSchema),
    targetScopeOptions: Schema.Array(ScreenControlCatalogOptionSchema),
    effectModeOptions: Schema.Array(ScreenControlCatalogOptionSchema),
    effectKey: ScreenControlSettingIdSchema,
    effectStatus: ScreenControlCatalogEffectStatusSchema,
    runtimeOwner: ScreenControlCatalogRuntimeOwnerSchema,
    capabilityState: ScreenControlCatalogCapabilityStateSchema,
    capabilityRequirement: Schema.Union(NonEmptyStringSchema, Schema.Null),
    proofRequirement: Schema.Union(NonEmptyStringSchema, Schema.Null),
    visibilityConditions: Schema.Array(ScreenControlCatalogRuleSchema),
    enabledConditions: Schema.Array(ScreenControlCatalogRuleSchema),
    validationRules: Schema.Array(ScreenControlCatalogRuleSchema),
    unsafeOrUnsupportedFallback: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

export const ScreenControlCatalogGroupSchema = withParser(
  Schema.Struct({
    groupId: ScreenControlSectionIdSchema,
    title: NonEmptyStringSchema,
    sourceOrder: Schema.Number,
    settings: Schema.Array(ScreenControlCatalogSettingSchema),
  })
);

export const ScreenControlCatalogSectionSchema = withParser(
  Schema.Struct({
    sectionId: ScreenControlSectionIdSchema,
    title: NonEmptyStringSchema,
    sourceOrder: Schema.Number,
    uiTab: ScreenControlCatalogUiTabSchema,
    sectionKind: ScreenControlCatalogSectionKindSchema,
    groups: Schema.Array(ScreenControlCatalogGroupSchema),
  })
);

export const ScreenControlCatalogTabSchema = withParser(
  Schema.Struct({
    tabId: ScreenControlCatalogUiTabSchema,
    title: NonEmptyStringSchema,
    sourceOrder: Schema.Number,
    sections: Schema.Array(ScreenControlCatalogSectionSchema),
  })
);

export const ScreenControlCatalogSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    catalogId: ScreenControlCatalogIdSchema,
    sidePanelCategory: ScreenControlCatalogSidePanelCategorySchema,
    sourceDocuments: Schema.Array(NonEmptyStringSchema),
    settingCount: Schema.Number,
    targetScopeOptions: Schema.Array(ScreenControlCatalogOptionSchema),
    effectModeOptions: Schema.Array(ScreenControlCatalogOptionSchema),
    tabs: Schema.Array(ScreenControlCatalogTabSchema),
  })
);

export const decodeScreenControlCatalog = Schema.decodeUnknownSync(ScreenControlCatalogSchema);

export type ScreenControlCatalogSourceKind = Infer<typeof ScreenControlCatalogSourceKindSchema>;
export type ScreenControlCatalogControlKind = Infer<typeof ScreenControlCatalogControlKindSchema>;
export type ScreenControlCatalogCardKind = Infer<typeof ScreenControlCatalogCardKindSchema>;
export type ScreenControlCatalogSelectionMode = Infer<typeof ScreenControlCatalogSelectionModeSchema>;
export type ScreenControlCatalogUiTab = Infer<typeof ScreenControlCatalogUiTabSchema>;
export type ScreenControlCatalogSectionKind = Infer<typeof ScreenControlCatalogSectionKindSchema>;
export type ScreenControlCatalogEffectStatus = Infer<typeof ScreenControlCatalogEffectStatusSchema>;
export type ScreenControlCatalogRuntimeOwner = Infer<typeof ScreenControlCatalogRuntimeOwnerSchema>;
export type ScreenControlCatalogCapabilityState = Infer<typeof ScreenControlCatalogCapabilityStateSchema>;
export type ScreenControlCatalogOption = Infer<typeof ScreenControlCatalogOptionSchema>;
export type ScreenControlCatalogLayoutHints = Infer<typeof ScreenControlCatalogLayoutHintsSchema>;
export type ScreenControlCatalogRule = Infer<typeof ScreenControlCatalogRuleSchema>;
export type ScreenControlCatalogSetting = Infer<typeof ScreenControlCatalogSettingSchema>;
export type ScreenControlCatalogGroup = Infer<typeof ScreenControlCatalogGroupSchema>;
export type ScreenControlCatalogSection = Infer<typeof ScreenControlCatalogSectionSchema>;
export type ScreenControlCatalogTab = Infer<typeof ScreenControlCatalogTabSchema>;
export type ScreenControlCatalog = Infer<typeof ScreenControlCatalogSchema>;
export type ScreenControlCatalogSettingSeed = readonly [
  sourceKind: ScreenControlCatalogSourceKind,
  sourceDocument: string,
  sectionId: string,
  sectionTitle: string,
  sectionOrder: number,
  groupId: string,
  groupTitle: string,
  groupOrder: number,
  settingId: string,
  sourceOrder: number,
  sourceLine: number,
  sourceText: string,
  controlKind: ScreenControlCatalogControlKind,
  acceptedOptions: readonly string[],
];

