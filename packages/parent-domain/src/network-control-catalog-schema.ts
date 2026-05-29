import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

import { ParentContractSchemaVersionSchema, ParentDeviceIdSchema, ParentTimestampSchema } from './reference-primitives';

const NetworkControlCatalogTextSchema = Schema.String.pipe(Schema.minLength(1));
const NonEmptyNetworkControlIdSchema = Schema.String.pipe(Schema.minLength(1));

export const NetworkControlCatalogIdSchema = withParser(
  NonEmptyNetworkControlIdSchema.pipe(Schema.brand('NetworkControlCatalogId'))
);
export const NetworkControlSectionIdSchema = withParser(
  NonEmptyNetworkControlIdSchema.pipe(Schema.brand('NetworkControlSectionId'))
);
export const NetworkControlGroupIdSchema = withParser(
  NonEmptyNetworkControlIdSchema.pipe(Schema.brand('NetworkControlGroupId'))
);
export const NetworkControlSettingIdSchema = withParser(
  NonEmptyNetworkControlIdSchema.pipe(Schema.brand('NetworkControlSettingId'))
);
export const NetworkControlOptionIdSchema = withParser(
  NonEmptyNetworkControlIdSchema.pipe(Schema.brand('NetworkControlOptionId'))
);
export const NetworkControlCapabilityIdSchema = withParser(
  NonEmptyNetworkControlIdSchema.pipe(Schema.brand('NetworkControlCapabilityId'))
);
export const NetworkControlWritesToPathSchema = withParser(
  NonEmptyNetworkControlIdSchema.pipe(Schema.brand('NetworkControlWritesToPath'))
);

const NetworkControlSidePanelCategorySchema = withParser(Schema.Literal('network'));

export const NetworkControlUiTabSchema = withParser(
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

export const NetworkControlKindSchema = withParser(
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

export const NetworkControlCardKindSchema = withParser(
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
    'status-card',
    'number-card'
  )
);

const NetworkControlSelectionModeSchema = withParser(Schema.Literal('single', 'multi'));

export const NetworkControlEffectStatusSchema = withParser(
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

export const NetworkControlRuntimeOwnerSchema = withParser(
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

export const NetworkControlCapabilityStateSchema = withParser(
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

export const NetworkControlOptionSchema = withParser(
  Schema.Struct({
    optionId: NetworkControlOptionIdSchema,
    label: NetworkControlCatalogTextSchema,
    value: NetworkControlCatalogTextSchema,
    originalSourceText: NetworkControlCatalogTextSchema,
    meaning: Schema.Union(NetworkControlCatalogTextSchema, Schema.Null),
    defaultSelected: Schema.Boolean,
  })
);

const NetworkControlLayoutHintsSchema = withParser(
  Schema.Struct({
    preferredColumnSpan: Schema.Number,
    collapsible: Schema.Boolean,
    searchableOptions: Schema.Boolean,
    optionGroupCount: Schema.Number,
    showAsMatrixWhenLarge: Schema.Boolean,
    showSelectedCount: Schema.Boolean,
  })
);

const NetworkControlRuleSchema = withParser(
  Schema.Struct({
    ruleId: NetworkControlSettingIdSchema,
    description: NetworkControlCatalogTextSchema,
  })
);

export const NetworkControlCatalogSettingSchema = withParser(
  Schema.Struct({
    sidePanelCategory: NetworkControlSidePanelCategorySchema,
    policyLane: NetworkControlUiTabSchema,
    sectionId: NetworkControlSectionIdSchema,
    groupId: NetworkControlGroupIdSchema,
    settingId: NetworkControlSettingIdSchema,
    sourceDocument: NetworkControlCatalogTextSchema,
    sourceHeadingPath: Schema.Array(NetworkControlCatalogTextSchema),
    sourceSection: NetworkControlSectionIdSchema,
    sourceGroup: NetworkControlGroupIdSchema,
    sourceOrder: Schema.Number,
    sourceLine: Schema.Number,
    sourceText: NetworkControlCatalogTextSchema,
    originalSourceText: NetworkControlCatalogTextSchema,
    question: NetworkControlCatalogTextSchema,
    uiQuestionText: NetworkControlCatalogTextSchema,
    helperText: NetworkControlCatalogTextSchema,
    displayOrder: Schema.Number,
    selectionMode: NetworkControlSelectionModeSchema,
    controlKind: NetworkControlKindSchema,
    cardKind: NetworkControlCardKindSchema,
    layoutHints: NetworkControlLayoutHintsSchema,
    options: Schema.Array(NetworkControlOptionSchema),
    acceptedOptions: Schema.Array(NetworkControlOptionSchema),
    targetScopeOptions: Schema.Array(NetworkControlOptionSchema),
    effectModeOptions: Schema.Array(NetworkControlOptionSchema),
    writesTo: NetworkControlWritesToPathSchema,
    effectKey: NetworkControlSettingIdSchema,
    effectStatus: NetworkControlEffectStatusSchema,
    runtimeOwner: NetworkControlRuntimeOwnerSchema,
    capabilityState: NetworkControlCapabilityStateSchema,
    capabilityRequirement: NetworkControlCatalogTextSchema,
    proofRequirement: Schema.Union(NetworkControlCatalogTextSchema, Schema.Null),
    visibilityConditions: Schema.Array(NetworkControlRuleSchema),
    enabledConditions: Schema.Array(NetworkControlRuleSchema),
    validationRules: Schema.Array(NetworkControlRuleSchema),
    unsafeOrUnsupportedFallback: NetworkControlCatalogTextSchema,
  })
);

export const NetworkControlCatalogGroupSchema = withParser(
  Schema.Struct({
    groupId: NetworkControlGroupIdSchema,
    title: NetworkControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    settings: Schema.Array(NetworkControlCatalogSettingSchema),
  })
);

export const NetworkControlCatalogSectionSchema = withParser(
  Schema.Struct({
    sectionId: NetworkControlSectionIdSchema,
    title: NetworkControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    policyLane: NetworkControlUiTabSchema,
    groups: Schema.Array(NetworkControlCatalogGroupSchema),
  })
);

export const NetworkControlCatalogTabSchema = withParser(
  Schema.Struct({
    tabId: NetworkControlUiTabSchema,
    title: NetworkControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    sections: Schema.Array(NetworkControlCatalogSectionSchema),
  })
);

export const NetworkControlCatalogSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    catalogId: NetworkControlCatalogIdSchema,
    sidePanelCategory: NetworkControlSidePanelCategorySchema,
    sourceDocuments: Schema.Array(NetworkControlCatalogTextSchema),
    settingCount: Schema.Number,
    acceptedOptionCount: Schema.Number,
    targetScopeOptions: Schema.Array(NetworkControlOptionSchema),
    effectModeOptions: Schema.Array(NetworkControlOptionSchema),
    tabs: Schema.Array(NetworkControlCatalogTabSchema),
  })
);

export const NetworkControlCapabilitySchema = withParser(
  Schema.Struct({
    capabilityId: NetworkControlCapabilityIdSchema,
    state: NetworkControlCapabilityStateSchema,
    sourceState: NetworkControlCatalogTextSchema,
    proof: NetworkControlCatalogTextSchema,
    affectsSettings: Schema.Array(NetworkControlSettingIdSchema),
  })
);

const NetworkControlPolicyValuePrimitiveSchema = Schema.Union(
  Schema.String,
  Schema.Number,
  Schema.Boolean,
  Schema.Array(Schema.String),
  Schema.Null
);

export const NetworkControlPolicyValueSchema = withParser(
  Schema.Struct({
    documentId: NetworkControlCatalogIdSchema,
    policyKind: Schema.Literal('network-control'),
    schemaVersion: ParentContractSchemaVersionSchema,
    revision: Schema.Number,
    targetDeviceId: ParentDeviceIdSchema,
    updatedAt: ParentTimestampSchema,
    settings: Schema.Array(
      Schema.Struct({
        settingId: NetworkControlSettingIdSchema,
        value: NetworkControlPolicyValuePrimitiveSchema,
      })
    ),
  })
);

export const NetworkControlEffectivePolicySchema = withParser(
  Schema.Struct({
    documentId: NetworkControlCatalogIdSchema,
    compiledFromPolicyId: NetworkControlCatalogIdSchema,
    schemaVersion: ParentContractSchemaVersionSchema,
    effectivePolicyHash: NetworkControlCatalogTextSchema,
    compiledAt: ParentTimestampSchema,
    runtimeOwner: NetworkControlRuntimeOwnerSchema,
    plans: Schema.Array(
      Schema.Struct({
        settingId: NetworkControlSettingIdSchema,
        writesTo: NetworkControlWritesToPathSchema,
        effectStatus: NetworkControlEffectStatusSchema,
        runtimeOwner: NetworkControlRuntimeOwnerSchema,
        capabilityState: NetworkControlCapabilityStateSchema,
        fallback: NetworkControlCatalogTextSchema,
      })
    ),
  })
);

export const NetworkControlUpdateCommandSchema = withParser(
  Schema.Struct({
    commandType: Schema.Literal(
      'network-control.get',
      'network-control.preview',
      'network-control.patch',
      'network-control.replace',
      'network-control.acknowledge',
      'network-control.reject',
      'network-control.rollback',
      'network-control.capability-refresh'
    ),
    targetDeviceId: ParentDeviceIdSchema,
    expectedRevision: Schema.Number,
    patch: Schema.Array(
      Schema.Struct({
        op: Schema.Literal('replace'),
        path: NetworkControlWritesToPathSchema,
        value: NetworkControlPolicyValuePrimitiveSchema,
      })
    ),
  })
);

export type NetworkControlUiTab = Infer<typeof NetworkControlUiTabSchema>;
export type NetworkControlKind = Infer<typeof NetworkControlKindSchema>;
export type NetworkControlCardKind = Infer<typeof NetworkControlCardKindSchema>;
export type NetworkControlSelectionMode = Infer<typeof NetworkControlSelectionModeSchema>;
export type NetworkControlEffectStatus = Infer<typeof NetworkControlEffectStatusSchema>;
export type NetworkControlRuntimeOwner = Infer<typeof NetworkControlRuntimeOwnerSchema>;
export type NetworkControlCapabilityState = Infer<typeof NetworkControlCapabilityStateSchema>;
export type NetworkControlOption = Infer<typeof NetworkControlOptionSchema>;
export type NetworkControlLayoutHints = Infer<typeof NetworkControlLayoutHintsSchema>;
export type NetworkControlRule = Infer<typeof NetworkControlRuleSchema>;
export type NetworkControlCatalogSetting = Infer<typeof NetworkControlCatalogSettingSchema>;
export type NetworkControlCatalogGroup = Infer<typeof NetworkControlCatalogGroupSchema>;
export type NetworkControlCatalogSection = Infer<typeof NetworkControlCatalogSectionSchema>;
export type NetworkControlCatalogTab = Infer<typeof NetworkControlCatalogTabSchema>;
export type NetworkControlCatalog = Infer<typeof NetworkControlCatalogSchema>;
export type NetworkControlCapability = Infer<typeof NetworkControlCapabilitySchema>;
export type NetworkControlPolicyValue = Infer<typeof NetworkControlPolicyValueSchema>;
export type NetworkControlEffectivePolicy = Infer<typeof NetworkControlEffectivePolicySchema>;
export type NetworkControlUpdateCommand = Infer<typeof NetworkControlUpdateCommandSchema>;
