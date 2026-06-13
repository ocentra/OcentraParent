import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

import {
  BrowserControlFieldIdSchema,
  BrowserControlManifestIdSchema,
  BrowserControlOptionIdSchema,
  BrowserControlSectionIdSchema,
} from './browser-control-identifiers';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';

export const BrowserControlFullCatalogControlKindSchema = withParser(
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

export const BrowserControlFullCatalogEffectStatusSchema = withParser(
  Schema.Literal(
    'already-represented',
    'needs-effect-wiring',
    'represented-by-existing-policy-shape',
    'manual-required',
    'unavailable',
    'future-gap',
    'degraded',
    'permission-required',
    'permission-limited',
    'proof-required'
  )
);

export const BrowserControlFullCatalogRuntimeOwnerSchema = withParser(
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

export const BrowserControlFullCatalogCapabilityStateSchema = withParser(
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

export const BrowserControlFullCatalogSidePanelCategorySchema = withParser(Schema.Literal('browser'));

export const BrowserControlFullCatalogUiTabSchema = withParser(
  Schema.Literal(
    'rules',
    'schedule',
    'approvals',
    'enforcement',
    'audit',
    'evidence',
    'setup',
    'reports',
    'platform',
    'data',
    'ai'
  )
);

export const BrowserControlFullCatalogCardKindSchema = withParser(
  Schema.Literal(
    'single-choice-compact',
    'single-choice-many',
    'multi-choice-normal',
    'multi-choice-many',
    'toggle',
    'schedule-card',
    'rule-list-card',
    'target-list-card',
    'retention-card',
    'status-card'
  )
);

export const BrowserControlFullCatalogSelectionModeSchema = withParser(Schema.Literal('single', 'multi'));

export const BrowserControlFullCatalogSectionKindSchema = withParser(
  Schema.Literal('setting-section', 'rule-dimension-section', 'candidate-mvp-section', 'planning-gap-section')
);

export const BrowserControlFullCatalogOptionSchema = withParser(
  Schema.Struct({
    optionId: BrowserControlOptionIdSchema,
    label: NonEmptyStringSchema,
    value: NonEmptyStringSchema,
    originalSourceText: NonEmptyStringSchema,
    meaning: Schema.Union(NonEmptyStringSchema, Schema.Null),
    defaultSelected: Schema.Boolean,
  })
);

export const BrowserControlFullCatalogLayoutHintsSchema = withParser(
  Schema.Struct({
    preferredColumnSpan: Schema.Number,
    collapsible: Schema.Boolean,
    searchableOptions: Schema.Boolean,
    optionGroupCount: Schema.Number,
    showAsMatrixWhenLarge: Schema.Boolean,
    showSelectedCount: Schema.Boolean,
  })
);

export const BrowserControlFullCatalogRuleSchema = withParser(
  Schema.Struct({
    ruleId: BrowserControlFieldIdSchema,
    description: NonEmptyStringSchema,
  })
);

export const BrowserControlFullCatalogSettingSchema = withParser(
  Schema.Struct({
    sidePanelCategory: BrowserControlFullCatalogSidePanelCategorySchema,
    sectionId: BrowserControlSectionIdSchema,
    groupId: BrowserControlSectionIdSchema,
    settingId: BrowserControlFieldIdSchema,
    sourceDocument: NonEmptyStringSchema,
    sourceHeadingPath: Schema.Array(NonEmptyStringSchema),
    sourceSection: BrowserControlSectionIdSchema,
    sourceGroup: BrowserControlSectionIdSchema,
    sourceOrder: Schema.Number,
    sourceLine: Schema.Number,
    sourceText: NonEmptyStringSchema,
    originalSourceText: NonEmptyStringSchema,
    question: NonEmptyStringSchema,
    uiQuestionText: NonEmptyStringSchema,
    helperText: Schema.Union(NonEmptyStringSchema, Schema.Null),
    displayOrder: Schema.Number,
    uiTab: BrowserControlFullCatalogUiTabSchema,
    policyLane: BrowserControlFullCatalogUiTabSchema,
    selectionMode: BrowserControlFullCatalogSelectionModeSchema,
    cardKind: BrowserControlFullCatalogCardKindSchema,
    controlKind: BrowserControlFullCatalogControlKindSchema,
    layoutHints: BrowserControlFullCatalogLayoutHintsSchema,
    options: Schema.Array(BrowserControlFullCatalogOptionSchema),
    acceptedOptions: Schema.Array(BrowserControlFullCatalogOptionSchema),
    targetScopeOptions: Schema.Array(BrowserControlFullCatalogOptionSchema),
    effectModeOptions: Schema.Array(BrowserControlFullCatalogOptionSchema),
    effectKey: BrowserControlFieldIdSchema,
    effectStatus: BrowserControlFullCatalogEffectStatusSchema,
    runtimeOwner: BrowserControlFullCatalogRuntimeOwnerSchema,
    capabilityState: BrowserControlFullCatalogCapabilityStateSchema,
    capabilityRequirement: Schema.Union(NonEmptyStringSchema, Schema.Null),
    proofRequirement: Schema.Union(NonEmptyStringSchema, Schema.Null),
    visibilityConditions: Schema.Array(BrowserControlFullCatalogRuleSchema),
    enabledConditions: Schema.Array(BrowserControlFullCatalogRuleSchema),
    validationRules: Schema.Array(BrowserControlFullCatalogRuleSchema),
    unsafeOrUnsupportedFallback: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

export const BrowserControlFullCatalogGroupSchema = withParser(
  Schema.Struct({
    groupId: BrowserControlSectionIdSchema,
    title: NonEmptyStringSchema,
    sourceOrder: Schema.Number,
    settings: Schema.Array(BrowserControlFullCatalogSettingSchema),
  })
);

export const BrowserControlFullCatalogSectionSchema = withParser(
  Schema.Struct({
    sectionId: BrowserControlSectionIdSchema,
    title: NonEmptyStringSchema,
    sourceOrder: Schema.Number,
    uiTab: BrowserControlFullCatalogUiTabSchema,
    sectionKind: BrowserControlFullCatalogSectionKindSchema,
    groups: Schema.Array(BrowserControlFullCatalogGroupSchema),
  })
);

export const BrowserControlFullCatalogTabSchema = withParser(
  Schema.Struct({
    tabId: BrowserControlFullCatalogUiTabSchema,
    title: NonEmptyStringSchema,
    sourceOrder: Schema.Number,
    sections: Schema.Array(BrowserControlFullCatalogSectionSchema),
  })
);

export const BrowserControlFullCatalogSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    catalogId: BrowserControlManifestIdSchema,
    sidePanelCategory: BrowserControlFullCatalogSidePanelCategorySchema,
    sourceDocument: NonEmptyStringSchema,
    sourceDocuments: Schema.Array(NonEmptyStringSchema),
    settingCount: Schema.Number,
    targetScopeOptions: Schema.Array(BrowserControlFullCatalogOptionSchema),
    effectModeOptions: Schema.Array(BrowserControlFullCatalogOptionSchema),
    tabs: Schema.Array(BrowserControlFullCatalogTabSchema),
  })
);

export type BrowserControlFullCatalogControlKind = Infer<typeof BrowserControlFullCatalogControlKindSchema>;
export type BrowserControlFullCatalogEffectStatus = Infer<typeof BrowserControlFullCatalogEffectStatusSchema>;
export type BrowserControlFullCatalogRuntimeOwner = Infer<typeof BrowserControlFullCatalogRuntimeOwnerSchema>;
export type BrowserControlFullCatalogCapabilityState = Infer<typeof BrowserControlFullCatalogCapabilityStateSchema>;
export type BrowserControlFullCatalogSidePanelCategory = Infer<typeof BrowserControlFullCatalogSidePanelCategorySchema>;
export type BrowserControlFullCatalogUiTab = Infer<typeof BrowserControlFullCatalogUiTabSchema>;
export type BrowserControlFullCatalogCardKind = Infer<typeof BrowserControlFullCatalogCardKindSchema>;
export type BrowserControlFullCatalogSelectionMode = Infer<typeof BrowserControlFullCatalogSelectionModeSchema>;
export type BrowserControlFullCatalogSectionKind = Infer<typeof BrowserControlFullCatalogSectionKindSchema>;
export type BrowserControlFullCatalogOption = Infer<typeof BrowserControlFullCatalogOptionSchema>;
export type BrowserControlFullCatalogLayoutHints = Infer<typeof BrowserControlFullCatalogLayoutHintsSchema>;
export type BrowserControlFullCatalogRule = Infer<typeof BrowserControlFullCatalogRuleSchema>;
export type BrowserControlFullCatalogSetting = Infer<typeof BrowserControlFullCatalogSettingSchema>;
export type BrowserControlFullCatalogGroup = Infer<typeof BrowserControlFullCatalogGroupSchema>;
export type BrowserControlFullCatalogSection = Infer<typeof BrowserControlFullCatalogSectionSchema>;
export type BrowserControlFullCatalogTab = Infer<typeof BrowserControlFullCatalogTabSchema>;
export type BrowserControlFullCatalog = Infer<typeof BrowserControlFullCatalogSchema>;
export type BrowserControlFullCatalogSettingSeed = readonly [
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
];

