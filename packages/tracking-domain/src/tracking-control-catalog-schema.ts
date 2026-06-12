import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

import { ParentContractSchemaVersionSchema, ParentDeviceIdSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const TrackingControlCatalogTextSchema = Schema.String.pipe(Schema.minLength(1));
const NonEmptyTrackingControlIdSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingControlCatalogIdSchema = withParser(
  NonEmptyTrackingControlIdSchema.pipe(Schema.brand('TrackingControlCatalogId'))
);
export const TrackingControlSectionIdSchema = withParser(
  NonEmptyTrackingControlIdSchema.pipe(Schema.brand('TrackingControlSectionId'))
);
export const TrackingControlGroupIdSchema = withParser(
  NonEmptyTrackingControlIdSchema.pipe(Schema.brand('TrackingControlGroupId'))
);
export const TrackingControlSettingIdSchema = withParser(
  NonEmptyTrackingControlIdSchema.pipe(Schema.brand('TrackingControlSettingId'))
);
export const TrackingControlOptionIdSchema = withParser(
  NonEmptyTrackingControlIdSchema.pipe(Schema.brand('TrackingControlOptionId'))
);
export const TrackingControlCapabilityIdSchema = withParser(
  NonEmptyTrackingControlIdSchema.pipe(Schema.brand('TrackingControlCapabilityId'))
);
export const TrackingControlWritesToPathSchema = withParser(
  NonEmptyTrackingControlIdSchema.pipe(Schema.brand('TrackingControlWritesToPath'))
);

const TrackingControlSidePanelCategorySchema = withParser(Schema.Literal('tracking'));

export const TrackingControlUiTabSchema = withParser(
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
    'live',
    'places'
  )
);

export const TrackingControlKindSchema = withParser(
  Schema.Literal(
    'toggle',
    'single-choice',
    'multi-choice',
    'number',
    'duration',
    'schedule',
    'rule-list',
    'target-list',
    'place-list',
    'geofence-list',
    'retention',
    'action-list',
    'read-only-status'
  )
);

export const TrackingControlCardKindSchema = withParser(
  Schema.Literal(
    'single-choice-compact',
    'single-choice-many',
    'multi-choice-normal',
    'multi-choice-many',
    'toggle',
    'schedule-card',
    'rule-list-card',
    'target-list-card',
    'place-list-card',
    'geofence-list-card',
    'retention-card',
    'status-card',
    'number-card'
  )
);

const TrackingControlSelectionModeSchema = withParser(Schema.Literal('single', 'multi'));

export const TrackingControlEffectStatusSchema = withParser(
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

export const TrackingControlRuntimeOwnerSchema = withParser(
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

export const TrackingControlCapabilityStateSchema = withParser(
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

export const TrackingControlOptionSchema = withParser(
  Schema.Struct({
    optionId: TrackingControlOptionIdSchema,
    label: TrackingControlCatalogTextSchema,
    value: TrackingControlCatalogTextSchema,
    originalSourceText: TrackingControlCatalogTextSchema,
    meaning: Schema.Union(TrackingControlCatalogTextSchema, Schema.Null),
    defaultSelected: Schema.Boolean,
  })
);

const TrackingControlLayoutHintsSchema = withParser(
  Schema.Struct({
    preferredColumnSpan: Schema.Number,
    collapsible: Schema.Boolean,
    searchableOptions: Schema.Boolean,
    optionGroupCount: Schema.Number,
    showAsMatrixWhenLarge: Schema.Boolean,
    showSelectedCount: Schema.Boolean,
  })
);

const TrackingControlRuleSchema = withParser(
  Schema.Struct({
    ruleId: TrackingControlSettingIdSchema,
    description: TrackingControlCatalogTextSchema,
  })
);

export const TrackingControlCatalogSettingSchema = withParser(
  Schema.Struct({
    sidePanelCategory: TrackingControlSidePanelCategorySchema,
    policyLane: TrackingControlUiTabSchema,
    sectionId: TrackingControlSectionIdSchema,
    groupId: TrackingControlGroupIdSchema,
    settingId: TrackingControlSettingIdSchema,
    sourceDocument: TrackingControlCatalogTextSchema,
    sourceHeadingPath: Schema.Array(TrackingControlCatalogTextSchema),
    sourceSection: TrackingControlSectionIdSchema,
    sourceGroup: TrackingControlGroupIdSchema,
    sourceOrder: Schema.Number,
    sourceLine: Schema.Number,
    sourceText: TrackingControlCatalogTextSchema,
    originalSourceText: TrackingControlCatalogTextSchema,
    question: TrackingControlCatalogTextSchema,
    uiQuestionText: TrackingControlCatalogTextSchema,
    helperText: TrackingControlCatalogTextSchema,
    displayOrder: Schema.Number,
    selectionMode: TrackingControlSelectionModeSchema,
    controlKind: TrackingControlKindSchema,
    cardKind: TrackingControlCardKindSchema,
    layoutHints: TrackingControlLayoutHintsSchema,
    options: Schema.Array(TrackingControlOptionSchema),
    acceptedOptions: Schema.Array(TrackingControlOptionSchema),
    targetScopeOptions: Schema.Array(TrackingControlOptionSchema),
    effectModeOptions: Schema.Array(TrackingControlOptionSchema),
    writesTo: TrackingControlWritesToPathSchema,
    effectKey: TrackingControlSettingIdSchema,
    effectStatus: TrackingControlEffectStatusSchema,
    runtimeOwner: TrackingControlRuntimeOwnerSchema,
    capabilityState: TrackingControlCapabilityStateSchema,
    capabilityRequirement: TrackingControlCatalogTextSchema,
    proofRequirement: Schema.Union(TrackingControlCatalogTextSchema, Schema.Null),
    visibilityConditions: Schema.Array(TrackingControlRuleSchema),
    enabledConditions: Schema.Array(TrackingControlRuleSchema),
    validationRules: Schema.Array(TrackingControlRuleSchema),
    unsafeOrUnsupportedFallback: TrackingControlCatalogTextSchema,
  })
);

export const TrackingControlCatalogGroupSchema = withParser(
  Schema.Struct({
    groupId: TrackingControlGroupIdSchema,
    title: TrackingControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    settings: Schema.Array(TrackingControlCatalogSettingSchema),
  })
);

export const TrackingControlCatalogSectionSchema = withParser(
  Schema.Struct({
    sectionId: TrackingControlSectionIdSchema,
    title: TrackingControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    policyLane: TrackingControlUiTabSchema,
    groups: Schema.Array(TrackingControlCatalogGroupSchema),
  })
);

export const TrackingControlCatalogTabSchema = withParser(
  Schema.Struct({
    tabId: TrackingControlUiTabSchema,
    title: TrackingControlCatalogTextSchema,
    sourceOrder: Schema.Number,
    sections: Schema.Array(TrackingControlCatalogSectionSchema),
  })
);

export const TrackingControlCatalogSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    catalogId: TrackingControlCatalogIdSchema,
    sidePanelCategory: TrackingControlSidePanelCategorySchema,
    sourceDocuments: Schema.Array(TrackingControlCatalogTextSchema),
    settingCount: Schema.Number,
    acceptedOptionCount: Schema.Number,
    targetScopeOptions: Schema.Array(TrackingControlOptionSchema),
    effectModeOptions: Schema.Array(TrackingControlOptionSchema),
    tabs: Schema.Array(TrackingControlCatalogTabSchema),
  })
);

export const TrackingControlCapabilitySchema = withParser(
  Schema.Struct({
    capabilityId: TrackingControlCapabilityIdSchema,
    state: TrackingControlCapabilityStateSchema,
    sourceState: TrackingControlCatalogTextSchema,
    proof: TrackingControlCatalogTextSchema,
    affectsSettings: Schema.Array(TrackingControlSettingIdSchema),
  })
);

const TrackingControlPolicyValuePrimitiveSchema = Schema.Union(
  Schema.String,
  Schema.Number,
  Schema.Boolean,
  Schema.Array(Schema.String),
  Schema.Null
);

export const TrackingControlPolicyValueSchema = withParser(
  Schema.Struct({
    documentId: TrackingControlCatalogIdSchema,
    policyKind: Schema.Literal('device-location-tracking'),
    schemaVersion: ParentContractSchemaVersionSchema,
    revision: Schema.Number,
    targetDeviceId: ParentDeviceIdSchema,
    updatedAt: ParentTimestampSchema,
    settings: Schema.Array(
      Schema.Struct({
        settingId: TrackingControlSettingIdSchema,
        value: TrackingControlPolicyValuePrimitiveSchema,
      })
    ),
  })
);

export const TrackingControlEffectivePolicySchema = withParser(
  Schema.Struct({
    documentId: TrackingControlCatalogIdSchema,
    compiledFromPolicyId: TrackingControlCatalogIdSchema,
    schemaVersion: ParentContractSchemaVersionSchema,
    effectivePolicyHash: TrackingControlCatalogTextSchema,
    compiledAt: ParentTimestampSchema,
    runtimeOwner: TrackingControlRuntimeOwnerSchema,
    plans: Schema.Array(
      Schema.Struct({
        settingId: TrackingControlSettingIdSchema,
        writesTo: TrackingControlWritesToPathSchema,
        effectStatus: TrackingControlEffectStatusSchema,
        runtimeOwner: TrackingControlRuntimeOwnerSchema,
        capabilityState: TrackingControlCapabilityStateSchema,
        fallback: TrackingControlCatalogTextSchema,
      })
    ),
  })
);

export const TrackingControlUpdateCommandSchema = withParser(
  Schema.Struct({
    commandType: Schema.Literal(
      'tracking-control.get',
      'tracking-control.preview',
      'tracking-control.patch',
      'tracking-control.replace',
      'tracking-control.acknowledge',
      'tracking-control.reject',
      'tracking-control.rollback',
      'tracking-control.capability-refresh',
      'tracking-control.live-session-start',
      'tracking-control.live-session-stop',
      'tracking-control.check-in-request'
    ),
    targetDeviceId: ParentDeviceIdSchema,
    expectedRevision: Schema.Number,
    patch: Schema.Array(
      Schema.Struct({
        op: Schema.Literal('replace'),
        path: TrackingControlWritesToPathSchema,
        value: TrackingControlPolicyValuePrimitiveSchema,
      })
    ),
  })
);

export type TrackingControlUiTab = Infer<typeof TrackingControlUiTabSchema>;
export type TrackingControlKind = Infer<typeof TrackingControlKindSchema>;
export type TrackingControlCardKind = Infer<typeof TrackingControlCardKindSchema>;
export type TrackingControlSelectionMode = Infer<typeof TrackingControlSelectionModeSchema>;
export type TrackingControlEffectStatus = Infer<typeof TrackingControlEffectStatusSchema>;
export type TrackingControlRuntimeOwner = Infer<typeof TrackingControlRuntimeOwnerSchema>;
export type TrackingControlCapabilityState = Infer<typeof TrackingControlCapabilityStateSchema>;
export type TrackingControlOption = Infer<typeof TrackingControlOptionSchema>;
export type TrackingControlLayoutHints = Infer<typeof TrackingControlLayoutHintsSchema>;
export type TrackingControlRule = Infer<typeof TrackingControlRuleSchema>;
export type TrackingControlCatalogSetting = Infer<typeof TrackingControlCatalogSettingSchema>;
export type TrackingControlCatalogGroup = Infer<typeof TrackingControlCatalogGroupSchema>;
export type TrackingControlCatalogSection = Infer<typeof TrackingControlCatalogSectionSchema>;
export type TrackingControlCatalogTab = Infer<typeof TrackingControlCatalogTabSchema>;
export type TrackingControlCatalog = Infer<typeof TrackingControlCatalogSchema>;
export type TrackingControlCapability = Infer<typeof TrackingControlCapabilitySchema>;
export type TrackingControlPolicyValue = Infer<typeof TrackingControlPolicyValueSchema>;
export type TrackingControlEffectivePolicy = Infer<typeof TrackingControlEffectivePolicySchema>;
export type TrackingControlUpdateCommand = Infer<typeof TrackingControlUpdateCommandSchema>;
