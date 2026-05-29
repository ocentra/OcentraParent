import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentContractSchemaVersionSchema, ParentDeviceIdSchema, ParentTimestampSchema } from './reference-primitives';

const AppControlCatalogTextSchema = Schema.String.pipe(Schema.minLength(1));

export const AppControlCatalogIdSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlCatalogId'))
);
export const AppControlSectionIdSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlSectionId'))
);
export const AppControlGroupIdSchema = withParser(AppControlCatalogTextSchema.pipe(Schema.brand('AppControlGroupId')));
export const AppControlSettingIdSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlSettingId'))
);
export const AppControlOptionIdSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlOptionId'))
);
export const AppControlWritesToPathSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlWritesToPath'))
);
export const AppControlCapabilityIdSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlCapabilityId'))
);
export const AppControlPolicyDocumentIdSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlPolicyDocumentId'))
);
export const AppControlPolicyHashSchema = withParser(
  AppControlCatalogTextSchema.pipe(Schema.brand('AppControlPolicyHash'))
);

export const AppControlSidePanelCategorySchema = withParser(Schema.Literal('apps'));
export const AppControlUiTabSchema = withParser(
  Schema.Literal('rules', 'schedule', 'approvals', 'enforcement', 'audit', 'evidence', 'reports', 'setup')
);
export const AppControlKindSchema = withParser(
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
export const AppControlCardKindSchema = withParser(
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
export const AppControlEffectStatusSchema = withParser(
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
export const AppControlRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'portal-only',
    'parent-domain',
    'agent-protocol',
    'rust-service',
    'child-agent',
    'os-adapter',
    'manual-proof',
    'parent-owned-storage'
  )
);
export const AppControlCapabilityStateSchema = withParser(
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
export const AppControlTargetScopeSchema = withParser(
  Schema.Literal('family', 'per-child', 'per-device', 'per-platform', 'per-app')
);
export const AppControlEffectModeSchema = withParser(
  Schema.Literal('off', 'observe', 'dry-run', 'warn', 'notify', 'ask', 'limit', 'block', 'enforce', 'audit-only')
);
export const AppControlPolicyValuePrimitiveSchema = Schema.Union(
  Schema.String,
  Schema.Number,
  Schema.Boolean,
  Schema.Array(Schema.String),
  Schema.Null
);

export const AppControlCatalogOptionSchema = withParser(
  Schema.Struct({
    optionId: AppControlOptionIdSchema,
    label: AppControlCatalogTextSchema,
    value: AppControlCatalogTextSchema,
    originalSourceText: AppControlCatalogTextSchema,
    meaning: Schema.Union(AppControlCatalogTextSchema, Schema.Null),
    defaultSelected: Schema.Boolean,
  })
);

export const AppControlCatalogLayoutHintsSchema = withParser(
  Schema.Struct({
    preferredColumnSpan: Schema.Number,
    collapsible: Schema.Boolean,
    searchableOptions: Schema.Boolean,
    optionGroupCount: Schema.Number,
    showAsMatrixWhenLarge: Schema.Boolean,
    showSelectedCount: Schema.Boolean,
  })
);

export const AppControlCatalogRuleSchema = withParser(
  Schema.Struct({
    ruleId: AppControlSettingIdSchema,
    description: AppControlCatalogTextSchema,
  })
);

export const AppControlCatalogSettingSchema = withParser(
  Schema.Struct({
    sidePanelCategory: AppControlSidePanelCategorySchema,
    policyLane: AppControlUiTabSchema,
    sectionId: AppControlSectionIdSchema,
    groupId: AppControlGroupIdSchema,
    settingId: AppControlSettingIdSchema,
    sourceDocument: AppControlCatalogTextSchema,
    sourceHeadingPath: Schema.Array(AppControlCatalogTextSchema),
    sourceSection: AppControlSectionIdSchema,
    sourceGroup: AppControlGroupIdSchema,
    sourceOrder: Schema.Number,
    sourceText: AppControlCatalogTextSchema,
    originalSourceText: AppControlCatalogTextSchema,
    question: AppControlCatalogTextSchema,
    uiQuestionText: AppControlCatalogTextSchema,
    helperText: Schema.Union(AppControlCatalogTextSchema, Schema.Null),
    displayOrder: Schema.Number,
    controlKind: AppControlKindSchema,
    cardKind: AppControlCardKindSchema,
    layoutHints: AppControlCatalogLayoutHintsSchema,
    options: Schema.Array(AppControlCatalogOptionSchema),
    acceptedOptions: Schema.Array(AppControlCatalogOptionSchema),
    targetScopeOptions: Schema.Array(AppControlTargetScopeSchema),
    effectModeOptions: Schema.Array(AppControlEffectModeSchema),
    writesTo: AppControlWritesToPathSchema,
    effectKey: AppControlSettingIdSchema,
    effectStatus: AppControlEffectStatusSchema,
    runtimeOwner: AppControlRuntimeOwnerSchema,
    capabilityState: AppControlCapabilityStateSchema,
    capabilityRequirement: Schema.Union(AppControlCatalogTextSchema, Schema.Null),
    proofRequirement: Schema.Union(AppControlCatalogTextSchema, Schema.Null),
    visibilityConditions: Schema.Array(AppControlCatalogRuleSchema),
    enabledConditions: Schema.Array(AppControlCatalogRuleSchema),
    validationRules: Schema.Array(AppControlCatalogRuleSchema),
    unsafeOrUnsupportedFallback: Schema.Union(AppControlCatalogTextSchema, Schema.Null),
  })
);

export const AppControlCatalogGroupSchema = withParser(
  Schema.Struct({
    groupId: AppControlGroupIdSchema,
    title: AppControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    settings: Schema.Array(AppControlCatalogSettingSchema),
  })
);

export const AppControlCatalogSectionSchema = withParser(
  Schema.Struct({
    sectionId: AppControlSectionIdSchema,
    title: AppControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    policyLane: AppControlUiTabSchema,
    groups: Schema.Array(AppControlCatalogGroupSchema),
  })
);

export const AppControlAuthoringCatalogSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    catalogId: AppControlCatalogIdSchema,
    sidePanelCategory: AppControlSidePanelCategorySchema,
    sourceDocuments: Schema.Array(AppControlCatalogTextSchema),
    settingCount: Schema.Number,
    acceptedOptionCount: Schema.Number,
    targetScopeOptions: Schema.Array(AppControlTargetScopeSchema),
    effectModeOptions: Schema.Array(AppControlEffectModeSchema),
    sections: Schema.Array(AppControlCatalogSectionSchema),
  })
);

export const AppControlCapabilitySchema = withParser(
  Schema.Struct({
    capabilityId: AppControlCapabilityIdSchema,
    state: AppControlCapabilityStateSchema,
    proof: AppControlCatalogTextSchema,
    source: AppControlCatalogTextSchema,
    affectsSettings: Schema.Array(AppControlSettingIdSchema),
  })
);

export const AppControlPolicyValueSchema = withParser(
  Schema.Struct({
    documentId: AppControlPolicyDocumentIdSchema,
    policyKind: Schema.Literal('app-control'),
    schemaVersion: ParentContractSchemaVersionSchema,
    revision: Schema.Number,
    targetDeviceId: ParentDeviceIdSchema,
    updatedAt: ParentTimestampSchema,
    settings: Schema.Array(
      Schema.Struct({
        settingId: AppControlSettingIdSchema,
        value: AppControlPolicyValuePrimitiveSchema,
      })
    ),
  })
);

export const AppControlEffectivePolicySchema = withParser(
  Schema.Struct({
    documentId: AppControlPolicyDocumentIdSchema,
    compiledFromPolicyId: AppControlPolicyDocumentIdSchema,
    schemaVersion: ParentContractSchemaVersionSchema,
    effectivePolicyHash: AppControlPolicyHashSchema,
    compiledAt: ParentTimestampSchema,
    runtimeOwner: AppControlRuntimeOwnerSchema,
    plans: Schema.Array(
      Schema.Struct({
        settingId: AppControlSettingIdSchema,
        effectStatus: AppControlEffectStatusSchema,
        runtimeOwner: AppControlRuntimeOwnerSchema,
        fallback: AppControlCatalogTextSchema,
      })
    ),
  })
);

export const AppControlUpdateCommandSchema = withParser(
  Schema.Struct({
    commandType: AppControlCatalogTextSchema,
    targetDeviceId: ParentDeviceIdSchema,
    expectedRevision: Schema.Union(Schema.Number, Schema.Null),
    patch: Schema.Array(
      Schema.Struct({
        op: Schema.Literal('replace'),
        path: AppControlWritesToPathSchema,
        value: AppControlPolicyValuePrimitiveSchema,
      })
    ),
  })
);

export type AppControlAuthoringCatalog = Infer<typeof AppControlAuthoringCatalogSchema>;
export type AppControlKind = Infer<typeof AppControlKindSchema>;
export type AppControlCardKind = Infer<typeof AppControlCardKindSchema>;
export type AppControlCapabilityState = Infer<typeof AppControlCapabilityStateSchema>;
export type AppControlCatalogOption = Infer<typeof AppControlCatalogOptionSchema>;
export type AppControlCatalogLayoutHints = Infer<typeof AppControlCatalogLayoutHintsSchema>;
export type AppControlCatalogRule = Infer<typeof AppControlCatalogRuleSchema>;
export type AppControlCatalogSetting = Infer<typeof AppControlCatalogSettingSchema>;
export type AppControlCatalogGroup = Infer<typeof AppControlCatalogGroupSchema>;
export type AppControlCatalogSection = Infer<typeof AppControlCatalogSectionSchema>;
export type AppControlCapability = Infer<typeof AppControlCapabilitySchema>;
export type AppControlPolicyValue = Infer<typeof AppControlPolicyValueSchema>;
export type AppControlEffectivePolicy = Infer<typeof AppControlEffectivePolicySchema>;
export type AppControlUpdateCommand = Infer<typeof AppControlUpdateCommandSchema>;
