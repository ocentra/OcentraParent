import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BrowserControlAuditEventIdSchema,
  BrowserControlBudgetIdSchema,
  BrowserControlCapabilityIdSchema,
  BrowserControlFieldIdSchema,
  BrowserControlHashIdSchema,
  BrowserControlPolicyIdSchema,
  BrowserControlRequestIdSchema,
  BrowserControlRevisionIdSchema,
  BrowserControlRuleIdSchema,
  BrowserControlScheduleIdSchema,
  type BrowserControlRequestId,
} from './browser-control-identifiers';
import {
  BrowserControlApprovalRequiredForSchema,
  BrowserControlApprovalUnansweredDefaultSchema,
  BrowserControlAuditRequiredFieldSchema,
  BrowserControlBudgetCountingModeSchema,
  BrowserControlCustodyAllowedUseSchema,
  BrowserControlDownloadBlockedTypeSchema,
  BrowserControlEvidenceNeverCollectSchema,
  BrowserControlEvidenceUrlScopeSchema,
  BrowserControlBrowserGameApprovalModeSchema,
  BrowserControlBrowserGamePolicyModeSchema,
  BrowserControlManagedBrowserBridgeRequirementSchema,
  BrowserControlManagedBrowserFamilySchema,
  BrowserControlManagedBrowserIntegrationMechanismSchema,
  BrowserControlManagedBrowserLaunchModeSchema,
  BrowserControlManagedPolicyWriterControlSchema,
  BrowserControlManagedPolicyWriterFallbackSchema,
  BrowserControlManagedBrowserProfileModeSchema,
  BrowserControlReportVisibleFieldSchema,
  BrowserControlRetentionExactUrlSchema,
  BrowserControlRuleActionSchema,
  BrowserControlUnmanagedBrowserClassificationTargetSchema,
} from './browser-control-catalog-values';
import {
  BrowserControlAuditStateSchema,
  BrowserControlApprovalStateSchema,
  BrowserControlCapabilityStateSchema,
  BrowserControlDefaultPostureSchema,
  BrowserControlDownloadStateSchema,
  BrowserControlEvidenceProofLevelSchema,
  BrowserControlExecutionModeSchema,
  BrowserControlFieldValueSchema,
  BrowserControlManagementModeSchema,
  BrowserControlManagedBrowserModeSchema,
  BrowserControlPatchOperationSchema,
  BrowserControlProofFallbackSchema,
  BrowserControlRejectionReasonSchema,
  BrowserControlReportStateSchema,
  BrowserControlRetentionStateSchema,
  BrowserControlSchemaKnownWritesToPathSchema,
  BrowserControlUnmanagedBrowserModeSchema,
  BrowserControlUpdateKindSchema,
  BrowserControlUpdateStatusSchema,
  BrowserControlUrlTargetTypeSchema,
  type BrowserControlUpdateKind,
} from './browser-control-values';
import {
  browserControlManifestAllowsField,
  browserControlManifestAllowsWritesTo,
  type BrowserControlAuthoringManifest,
} from './browser-control-manifest';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const BrowserControlPolicyTextSchema = Schema.String.pipe(Schema.minLength(1));

export const BrowserControlBudgetSchema = withParser(
  Schema.Struct({
    budgetId: BrowserControlBudgetIdSchema,
    dailyMinutes: Schema.Union(Schema.Number, Schema.Null),
  })
);

export const BrowserControlEvidenceRequirementSchema = withParser(
  Schema.Struct({
    urlScope: Schema.optionalWith(BrowserControlEvidenceUrlScopeSchema, {
      default: () => 'none' as const,
    }),
    requiredProof: BrowserControlEvidenceProofLevelSchema,
    proofFallback: Schema.Union(BrowserControlProofFallbackSchema, Schema.Null),
    whenProofUnavailable: Schema.optionalWith(BrowserControlProofFallbackSchema, {
      default: () => 'mark-unavailable' as const,
    }),
    neverCollect: Schema.optionalWith(Schema.Array(BrowserControlEvidenceNeverCollectSchema), {
      default: () => [],
    }),
  })
);

const BrowserControlRuleTargetSchema = withParser(
  Schema.Struct({
    kind: BrowserControlUrlTargetTypeSchema,
    values: Schema.Array(BrowserControlPolicyTextSchema),
    matchMode: BrowserControlPolicyTextSchema,
  })
);

const BrowserControlRuleActionPlanSchema = withParser(
  Schema.Struct({
    kind: BrowserControlRuleActionSchema,
    budgetId: Schema.optionalWith(Schema.Union(BrowserControlBudgetIdSchema, Schema.Null), {
      default: () => null,
    }),
    approvalKind: Schema.optionalWith(Schema.Union(BrowserControlApprovalRequiredForSchema, Schema.Null), {
      default: () => null,
    }),
    reasonCode: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
  })
);

const BrowserControlRuleBaseSchema = Schema.Struct({
  ruleId: BrowserControlRuleIdSchema,
  targetType: Schema.optionalWith(Schema.Union(BrowserControlUrlTargetTypeSchema, Schema.Null), {
    default: () => null,
  }),
  targetValue: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
    default: () => null,
  }),
  enabled: Schema.Boolean,
  priority: Schema.optionalWith(Schema.Union(Schema.Number, Schema.Null), {
    default: () => null,
  }),
  target: Schema.optionalWith(Schema.Union(BrowserControlRuleTargetSchema, Schema.Null), {
    default: () => null,
  }),
  action: Schema.optionalWith(Schema.Union(BrowserControlRuleActionPlanSchema, Schema.Null), {
    default: () => null,
  }),
  proofRequirement: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
    default: () => null,
  }),
  scheduleId: Schema.optionalWith(Schema.Union(BrowserControlScheduleIdSchema, Schema.Null), {
    default: () => null,
  }),
  budgetId: Schema.optionalWith(Schema.Union(BrowserControlBudgetIdSchema, Schema.Null), {
    default: () => null,
  }),
  auditLevel: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
    default: () => null,
  }),
});

type BrowserControlRuleCandidate = Infer<typeof BrowserControlRuleBaseSchema>;

export const BrowserControlRuleSchema = withParser(
  BrowserControlRuleBaseSchema.pipe(
    Schema.filter(
      (rule) =>
        browserControlRuleTargetIsSpecified(rule) ||
        'Expected browser-control rule to include flat targetType/targetValue or a structured target'
    )
  )
);

const BrowserControlScheduleSchema = withParser(
  Schema.Struct({
    scheduleId: BrowserControlScheduleIdSchema,
    kind: BrowserControlPolicyTextSchema,
    timezone: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
  })
);

const BrowserControlChildFacingSchema = withParser(
  Schema.Struct({
    showWarnText: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    showBlockReason: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    showAskParentState: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    showTimeLeft: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    showUseManagedBrowserAction: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    hideParentDiagnostics: Schema.optionalWith(Schema.Boolean, { default: () => false }),
  })
);

const BrowserControlPortalAiSchema = withParser(
  Schema.Struct({
    allowSummaries: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    allowPolicyExplanation: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    allowRuleSuggestions: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    allowEvidenceRefs: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    allowRawContent: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    requiresManualReview: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    fallbackWhenUnavailable: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
  })
);

const BrowserControlDiscoverySchema = withParser(
  Schema.Struct({
    scanInstalledBrowsers: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    scanRunningBrowsers: Schema.optionalWith(Schema.Boolean, { default: () => true }),
    detectUnmanagedBrowsers: Schema.optionalWith(Schema.Boolean, { default: () => true }),
  })
);

const BrowserControlPlatformCapabilitySchema = withParser(
  Schema.Struct({
    enabled: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    state: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), { default: () => null }),
    allowedAdapters: Schema.optionalWith(Schema.Array(BrowserControlPolicyTextSchema), { default: () => [] }),
    manualRequiredAdapters: Schema.optionalWith(Schema.Array(BrowserControlPolicyTextSchema), { default: () => [] }),
    authoringOnly: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    mayRunCapture: Schema.optionalWith(Schema.Boolean, { default: () => false }),
    mayConnectToBrowserBridge: Schema.optionalWith(Schema.Boolean, { default: () => false }),
  })
);

const BrowserControlPlatformsSchema = withParser(
  Schema.Struct({
    windows: Schema.optionalWith(BrowserControlPlatformCapabilitySchema, { default: defaultPlatformCapability }),
    macos: Schema.optionalWith(BrowserControlPlatformCapabilitySchema, { default: defaultPlatformCapability }),
    linux: Schema.optionalWith(BrowserControlPlatformCapabilitySchema, { default: defaultPlatformCapability }),
    android: Schema.optionalWith(BrowserControlPlatformCapabilitySchema, { default: defaultPlatformCapability }),
    ios: Schema.optionalWith(BrowserControlPlatformCapabilitySchema, { default: defaultPlatformCapability }),
    webPortal: Schema.optionalWith(BrowserControlPlatformCapabilitySchema, { default: defaultPlatformCapability }),
  })
);

const BrowserControlFallbacksSchema = withParser(
  Schema.Struct({
    managedProfileMissing: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    bridgeMissing: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    extensionDisabled: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    nativeHostMissing: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    unsupportedBrowser: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    staleEvidence: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    networkAdapterUnavailable: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    processControlUnavailable: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    enforcementFailure: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    childDeviceOffline: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
    platformUnsupported: Schema.optionalWith(Schema.Union(BrowserControlPolicyTextSchema, Schema.Null), {
      default: () => null,
    }),
  })
);

export const BrowserControlPolicyValueBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  policyId: BrowserControlPolicyIdSchema,
  enabled: Schema.Boolean,
  executionMode: Schema.optionalWith(BrowserControlExecutionModeSchema, {
    default: () => 'observe' as const,
  }),
  defaultPosture: BrowserControlDefaultPostureSchema,
  fallbackPosture: Schema.Union(BrowserControlDefaultPostureSchema, Schema.Null),
  managementMode: BrowserControlManagementModeSchema,
  discovery: Schema.optionalWith(BrowserControlDiscoverySchema, {
    default: defaultDiscovery,
  }),
  managedBrowser: Schema.Struct({
    mode: BrowserControlManagedBrowserModeSchema,
    allowedFamilies: Schema.optionalWith(Schema.Array(BrowserControlManagedBrowserFamilySchema), {
      default: () => [],
    }),
    launchMode: Schema.optionalWith(BrowserControlManagedBrowserLaunchModeSchema, {
      default: () => 'manual' as const,
    }),
    profileMode: Schema.optionalWith(BrowserControlManagedBrowserProfileModeSchema, {
      default: () => 'persistent-managed-profile' as const,
    }),
    bridgeRequirements: Schema.optionalWith(Schema.Array(BrowserControlManagedBrowserBridgeRequirementSchema), {
      default: () => [],
    }),
    integrationMechanisms: Schema.optionalWith(Schema.Array(BrowserControlManagedBrowserIntegrationMechanismSchema), {
      default: () => [],
    }),
    policyWriterControls: Schema.optionalWith(Schema.Array(BrowserControlManagedPolicyWriterControlSchema), {
      default: () => [],
    }),
    policyWriterFallback: Schema.optionalWith(BrowserControlManagedPolicyWriterFallbackSchema, {
      default: () => 'manual-required' as const,
    }),
  }),
  unmanagedBrowser: Schema.Struct({
    mode: BrowserControlUnmanagedBrowserModeSchema,
    graceSeconds: Schema.optionalWith(Schema.Number, {
      default: () => 0,
    }),
    allowRecoverLaunchUrl: Schema.optionalWith(Schema.Boolean, {
      default: () => false,
    }),
    classificationTargets: Schema.optionalWith(Schema.Array(BrowserControlUnmanagedBrowserClassificationTargetSchema), {
      default: () => [],
    }),
  }),
  evidence: BrowserControlEvidenceRequirementSchema,
  rules: Schema.Struct({
    allowedTargetTypes: Schema.Array(BrowserControlUrlTargetTypeSchema),
    allowedActions: Schema.optionalWith(Schema.Array(BrowserControlRuleActionSchema), {
      default: () => [],
    }),
    items: Schema.optionalWith(Schema.Array(BrowserControlRuleSchema), {
      default: () => [],
    }),
    entries: Schema.optionalWith(Schema.Array(BrowserControlRuleSchema), {
      default: () => [],
    }),
    urlAllowList: Schema.optionalWith(Schema.Array(BrowserControlPolicyTextSchema), {
      default: () => [],
    }),
    urlBlockList: Schema.optionalWith(Schema.Array(BrowserControlPolicyTextSchema), {
      default: () => [],
    }),
  }),
  budgets: Schema.Struct({
    enabled: Schema.optionalWith(Schema.Boolean, {
      default: () => true,
    }),
    defaultDailyMinutes: Schema.Union(Schema.Number, Schema.Null),
    countingMode: Schema.optionalWith(BrowserControlBudgetCountingModeSchema, {
      default: () => 'foreground-browser-time' as const,
    }),
  }),
  browserGames: Schema.optionalWith(
    Schema.Struct({
      educationalGameMode: Schema.optionalWith(BrowserControlBrowserGamePolicyModeSchema, {
        default: () => 'allow' as const,
      }),
      unknownGameMode: Schema.optionalWith(BrowserControlBrowserGamePolicyModeSchema, {
        default: () => 'ask-parent' as const,
      }),
      cloudGamingApproval: Schema.optionalWith(BrowserControlBrowserGameApprovalModeSchema, {
        default: () => 'ask-parent' as const,
      }),
      purchaseAccountApproval: Schema.optionalWith(BrowserControlBrowserGameApprovalModeSchema, {
        default: () => 'ask-parent' as const,
      }),
      unblockedPortalMode: Schema.optionalWith(BrowserControlBrowserGamePolicyModeSchema, {
        default: () => 'warn' as const,
      }),
      webglCanvasMode: Schema.optionalWith(BrowserControlBrowserGamePolicyModeSchema, {
        default: () => 'observe' as const,
      }),
      defaultDailyMinutes: Schema.optionalWith(Schema.Union(Schema.Number, Schema.Null), {
        default: () => 30,
      }),
    }),
    {
      default: defaultBrowserGames,
    }
  ),
  downloads: Schema.Struct({
    mode: Schema.optionalWith(BrowserControlDownloadStateSchema, {
      default: () => 'not-configured' as const,
    }),
    blockedTypes: Schema.optionalWith(Schema.Array(BrowserControlDownloadBlockedTypeSchema), {
      default: () => [],
    }),
    state: Schema.optionalWith(BrowserControlDownloadStateSchema, {
      default: () => 'not-configured' as const,
    }),
  }),
  approvals: Schema.Struct({
    requiredFor: Schema.optionalWith(Schema.Array(BrowserControlApprovalRequiredForSchema), {
      default: () => [],
    }),
    unansweredDefault: Schema.optionalWith(BrowserControlApprovalUnansweredDefaultSchema, {
      default: () => 'deny' as const,
    }),
    state: Schema.optionalWith(BrowserControlApprovalStateSchema, {
      default: () => 'not-required' as const,
    }),
  }),
  reports: Schema.Struct({
    visibleFields: Schema.optionalWith(Schema.Array(BrowserControlReportVisibleFieldSchema), {
      default: () => [],
    }),
    state: Schema.optionalWith(BrowserControlReportStateSchema, {
      default: () => 'disabled' as const,
    }),
  }),
  audit: Schema.Struct({
    requiredFields: Schema.optionalWith(Schema.Array(BrowserControlAuditRequiredFieldSchema), {
      default: () => [],
    }),
    state: Schema.optionalWith(BrowserControlAuditStateSchema, {
      default: () => 'local-only' as const,
    }),
  }),
  retention: Schema.Struct({
    exactUrl: Schema.optionalWith(BrowserControlRetentionExactUrlSchema, {
      default: () => 'fresh-only' as const,
    }),
    state: Schema.optionalWith(BrowserControlRetentionStateSchema, {
      default: () => 'none' as const,
    }),
  }),
  custody: Schema.optionalWith(
    Schema.Struct({
      allowedUses: Schema.Array(BrowserControlCustodyAllowedUseSchema),
    }),
    {
      default: () => ({ allowedUses: [] }),
    }
  ),
  schedules: Schema.optionalWith(Schema.Array(BrowserControlScheduleSchema), {
    default: () => [],
  }),
  childFacing: Schema.optionalWith(BrowserControlChildFacingSchema, {
    default: defaultChildFacing,
  }),
  portalAi: Schema.optionalWith(BrowserControlPortalAiSchema, {
    default: defaultPortalAi,
  }),
  platforms: Schema.optionalWith(BrowserControlPlatformsSchema, {
    default: defaultPlatforms,
  }),
  fallbacks: Schema.optionalWith(BrowserControlFallbacksSchema, {
    default: defaultFallbacks,
  }),
});

type BrowserControlPolicyValueCandidate = Infer<typeof BrowserControlPolicyValueBaseSchema>;

export const BrowserControlPolicyValueSchema = withParser(
  BrowserControlPolicyValueBaseSchema.pipe(
    Schema.filter(
      (policy) =>
        browserControlLimitPostureIsConsistent(policy) ||
        'Expected limit posture to include a daily budget or fallback posture'
    ),
    Schema.filter(
      (policy) =>
        browserControlExactUrlPolicyIsHonest(policy) ||
        'Expected exact URL browser rules to require managed proof or an explicit proof fallback'
    ),
    Schema.filter(
      (policy) =>
        browserControlBrowserGameLimitIsConsistent(policy) ||
        'Expected browser-game limit modes to include a daily browser-game budget or fallback posture'
    )
  )
);

export const BrowserControlTargetProofRequirementSchema = withParser(
  Schema.Literal(
    'none',
    'managed-exact-url',
    'domain-or-managed-url',
    'classifier-category',
    'url-shape-metadata',
    'social-route-evidence',
    'browser-game-runtime-signal',
    'browser-policy-writer',
    'process-detection',
    'download-evidence',
    'capability-state',
    'adapter-action'
  )
);

export const BrowserControlActionExecutionStateSchema = withParser(
  Schema.Literal(
    'observe-only',
    'dry-run-no-execution',
    'deterministic-parent-policy',
    'adapter-ready',
    'manual-required',
    'unavailable'
  )
);

export const BrowserControlAiAuthoritySchema = withParser(Schema.Literal('parent-policy-only', 'ai-candidate-only'));

export const BrowserControlEffectiveRuleSchema = withParser(
  Schema.Struct({
    ruleId: BrowserControlRuleIdSchema,
    targetType: BrowserControlUrlTargetTypeSchema,
    targetValue: BrowserControlPolicyTextSchema,
    defaultPosture: BrowserControlDefaultPostureSchema,
    evidence: BrowserControlEvidenceRequirementSchema,
    action: BrowserControlRuleActionSchema,
    targetProofRequirement: BrowserControlTargetProofRequirementSchema,
    capabilityState: BrowserControlCapabilityStateSchema,
    actionExecution: BrowserControlActionExecutionStateSchema,
    aiAuthority: BrowserControlAiAuthoritySchema,
    compileNote: BrowserControlPolicyTextSchema,
  })
);

export const BrowserControlEffectivePolicyBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  policyId: BrowserControlPolicyIdSchema,
  revisionId: BrowserControlRevisionIdSchema,
  compiledHash: BrowserControlHashIdSchema,
  compiledAt: ParentTimestampSchema,
  executionMode: BrowserControlExecutionModeSchema,
  defaultPosture: BrowserControlDefaultPostureSchema,
  fallbackPosture: Schema.Union(BrowserControlDefaultPostureSchema, Schema.Null),
  discovery: BrowserControlDiscoverySchema,
  budgets: Schema.Struct({
    enabled: Schema.optionalWith(Schema.Boolean, {
      default: () => true,
    }),
    defaultDailyMinutes: Schema.Union(Schema.Number, Schema.Null),
    countingMode: Schema.optionalWith(BrowserControlBudgetCountingModeSchema, {
      default: () => 'foreground-browser-time' as const,
    }),
  }),
  rules: Schema.Array(BrowserControlEffectiveRuleSchema),
});

type BrowserControlEffectivePolicyCandidate = Infer<typeof BrowserControlEffectivePolicyBaseSchema>;

export const BrowserControlEffectivePolicySchema = withParser(
  BrowserControlEffectivePolicyBaseSchema.pipe(
    Schema.filter(
      (policy) =>
        browserControlEffectiveLimitPostureIsConsistent(policy) ||
        'Expected effective limit posture to include a daily budget or fallback posture'
    )
  )
);

export const BrowserControlCapabilitySchema = withParser(
  Schema.Struct({
    capabilityId: BrowserControlCapabilityIdSchema,
    state: BrowserControlCapabilityStateSchema,
    label: BrowserControlPolicyTextSchema,
    affectedWritesTo: Schema.Array(BrowserControlSchemaKnownWritesToPathSchema),
    checkedAt: ParentTimestampSchema,
    reason: Schema.Union(BrowserControlPolicyTextSchema, Schema.Null),
  })
);

export const BrowserControlCapabilityRegistrySchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    generatedAt: ParentTimestampSchema,
    capabilities: Schema.Array(BrowserControlCapabilitySchema),
  })
);

const BrowserControlPatchValueSchema = Schema.Union(
  BrowserControlFieldValueSchema,
  Schema.Array(BrowserControlRuleSchema)
);

export const BrowserControlPatchSchema = withParser(
  Schema.Struct({
    op: BrowserControlPatchOperationSchema,
    fieldId: BrowserControlFieldIdSchema,
    writesTo: BrowserControlSchemaKnownWritesToPathSchema,
    value: BrowserControlPatchValueSchema,
  })
);

export const BrowserControlGetPolicyRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: BrowserControlRequestIdSchema,
    kind: Schema.Literal('get'),
    policyId: BrowserControlPolicyIdSchema,
  })
);

export const BrowserControlPreviewPolicyRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: BrowserControlRequestIdSchema,
    kind: Schema.Literal('preview'),
    policy: BrowserControlPolicyValueSchema,
  })
);

export const BrowserControlPatchPolicyRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: BrowserControlRequestIdSchema,
    kind: Schema.Literal('patch'),
    policyId: BrowserControlPolicyIdSchema,
    baseRevisionId: BrowserControlRevisionIdSchema,
    patches: Schema.Array(BrowserControlPatchSchema),
  })
);

export const BrowserControlReplacePolicyRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: BrowserControlRequestIdSchema,
    kind: Schema.Literal('replace'),
    baseRevisionId: Schema.Union(BrowserControlRevisionIdSchema, Schema.Null),
    policy: BrowserControlPolicyValueSchema,
  })
);

export const BrowserControlRollbackPolicyRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: BrowserControlRequestIdSchema,
    kind: Schema.Literal('rollback'),
    policyId: BrowserControlPolicyIdSchema,
    targetRevisionId: BrowserControlRevisionIdSchema,
  })
);

export const BrowserControlUpdateRequestSchema = withParser(
  Schema.Union(
    BrowserControlGetPolicyRequestSchema,
    BrowserControlPreviewPolicyRequestSchema,
    BrowserControlPatchPolicyRequestSchema,
    BrowserControlReplacePolicyRequestSchema,
    BrowserControlRollbackPolicyRequestSchema
  )
);

export const BrowserControlUpdateResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: BrowserControlRequestIdSchema,
    kind: BrowserControlUpdateKindSchema,
    status: BrowserControlUpdateStatusSchema,
    policy: Schema.Union(BrowserControlPolicyValueSchema, Schema.Null),
    effectivePolicy: Schema.Union(BrowserControlEffectivePolicySchema, Schema.Null),
    capabilityRegistry: Schema.Union(BrowserControlCapabilityRegistrySchema, Schema.Null),
    rejectionReason: Schema.Union(BrowserControlRejectionReasonSchema, Schema.Null),
    auditEventId: Schema.Union(BrowserControlAuditEventIdSchema, Schema.Null),
    message: Schema.Union(BrowserControlPolicyTextSchema, Schema.Null),
  })
);

export type BrowserControlBudget = Infer<typeof BrowserControlBudgetSchema>;
export type BrowserControlDiscovery = Infer<typeof BrowserControlDiscoverySchema>;
export type BrowserControlEvidenceRequirement = Infer<typeof BrowserControlEvidenceRequirementSchema>;
export type BrowserControlRule = Infer<typeof BrowserControlRuleSchema>;
export type BrowserControlPolicyValue = Infer<typeof BrowserControlPolicyValueSchema>;
export type BrowserControlTargetProofRequirement = Infer<typeof BrowserControlTargetProofRequirementSchema>;
export type BrowserControlActionExecutionState = Infer<typeof BrowserControlActionExecutionStateSchema>;
export type BrowserControlAiAuthority = Infer<typeof BrowserControlAiAuthoritySchema>;
export type BrowserControlEffectiveRule = Infer<typeof BrowserControlEffectiveRuleSchema>;
export type BrowserControlEffectivePolicy = Infer<typeof BrowserControlEffectivePolicySchema>;
export type BrowserControlCapability = Infer<typeof BrowserControlCapabilitySchema>;
export type BrowserControlCapabilityRegistry = Infer<typeof BrowserControlCapabilityRegistrySchema>;
export type BrowserControlPatch = Infer<typeof BrowserControlPatchSchema>;
export type BrowserControlUpdateRequest = Infer<typeof BrowserControlUpdateRequestSchema>;
export type BrowserControlUpdateResponse = Infer<typeof BrowserControlUpdateResponseSchema>;

export const decodeBrowserControlPolicyValue = Schema.decodeUnknownSync(BrowserControlPolicyValueSchema);
export const decodeBrowserControlEffectivePolicy = Schema.decodeUnknownSync(BrowserControlEffectivePolicySchema);
export const decodeBrowserControlCapabilityRegistry = Schema.decodeUnknownSync(BrowserControlCapabilityRegistrySchema);
export const decodeBrowserControlUpdateRequest = Schema.decodeUnknownSync(BrowserControlUpdateRequestSchema);
export const decodeBrowserControlUpdateResponse = Schema.decodeUnknownSync(BrowserControlUpdateResponseSchema);

export function browserControlManifestAllowsPatch(
  manifest: BrowserControlAuthoringManifest,
  patch: BrowserControlPatch
): boolean {
  return (
    browserControlManifestAllowsField(manifest, patch.fieldId) &&
    browserControlManifestAllowsWritesTo(manifest, patch.writesTo)
  );
}

export function browserControlManifestAllowsPatchRequest(
  manifest: BrowserControlAuthoringManifest,
  request: BrowserControlPatchPolicyRequest
): boolean {
  return request.patches.every((patch) => browserControlManifestAllowsPatch(manifest, patch));
}

export function browserControlCreateScaffoldUnavailableResponse(
  requestId: BrowserControlRequestId,
  kind: BrowserControlUpdateKind
): BrowserControlUpdateResponse {
  return BrowserControlUpdateResponseSchema.parse({
    schemaVersion: 'v0.6',
    requestId,
    kind,
    status: 'rejected',
    policy: null,
    effectivePolicy: null,
    capabilityRegistry: null,
    rejectionReason: 'scaffold-unavailable',
    auditEventId: null,
    message: 'Browser policy persistence and compiler are not implemented in this scaffold slice.',
  });
}

export type BrowserControlPatchPolicyRequest = Infer<typeof BrowserControlPatchPolicyRequestSchema>;

function browserControlLimitPostureIsConsistent(policy: BrowserControlPolicyValueCandidate): boolean {
  return (
    policy.defaultPosture !== 'limit' ||
    (policy.budgets.enabled && policy.budgets.defaultDailyMinutes !== null) ||
    policy.fallbackPosture !== null
  );
}

function browserControlEffectiveLimitPostureIsConsistent(policy: BrowserControlEffectivePolicyCandidate): boolean {
  return (
    policy.defaultPosture !== 'limit' ||
    (policy.budgets.enabled && policy.budgets.defaultDailyMinutes !== null) ||
    policy.fallbackPosture !== null
  );
}

function browserControlRuleTargetIsSpecified(rule: BrowserControlRuleCandidate): boolean {
  return (rule.targetType !== null && rule.targetValue !== null) || rule.target !== null;
}

function browserControlExactUrlPolicyIsHonest(policy: BrowserControlPolicyValueCandidate): boolean {
  const authoredRules = [...policy.rules.items, ...policy.rules.entries];
  if (!policy.rules.allowedTargetTypes.includes('exact-url') && !authoredRules.some(ruleUsesExactUrlTarget)) {
    return true;
  }
  if (policy.evidence.proofFallback !== null) {
    return true;
  }
  if (policy.evidence.whenProofUnavailable !== 'mark-unavailable') {
    return true;
  }
  return (
    (policy.managedBrowser.mode === 'required-for-exact-rules' ||
      policy.managedBrowser.mode === 'required-for-all-browsing') &&
    policy.evidence.requiredProof === 'fresh-managed-active-tab'
  );
}

function ruleUsesExactUrlTarget(rule: BrowserControlRuleCandidate): boolean {
  return rule.targetType === 'exact-url' || rule.target?.kind === 'exact-url';
}

function browserControlBrowserGameLimitIsConsistent(policy: BrowserControlPolicyValueCandidate): boolean {
  const gameLimitSelected =
    policy.browserGames.educationalGameMode === 'limit' ||
    policy.browserGames.unknownGameMode === 'limit' ||
    policy.browserGames.unblockedPortalMode === 'limit' ||
    policy.browserGames.webglCanvasMode === 'limit';
  return !gameLimitSelected || policy.browserGames.defaultDailyMinutes !== null || policy.fallbackPosture !== null;
}

function defaultBrowserGames() {
  return {
    educationalGameMode: 'allow' as const,
    unknownGameMode: 'ask-parent' as const,
    cloudGamingApproval: 'ask-parent' as const,
    purchaseAccountApproval: 'ask-parent' as const,
    unblockedPortalMode: 'warn' as const,
    webglCanvasMode: 'observe' as const,
    defaultDailyMinutes: 30,
  };
}

function defaultChildFacing() {
  return {
    showWarnText: false,
    showBlockReason: false,
    showAskParentState: false,
    showTimeLeft: false,
    showUseManagedBrowserAction: false,
    hideParentDiagnostics: false,
  };
}

function defaultDiscovery() {
  return {
    scanInstalledBrowsers: false,
    scanRunningBrowsers: true,
    detectUnmanagedBrowsers: true,
  };
}

function defaultPortalAi() {
  return {
    allowSummaries: false,
    allowPolicyExplanation: false,
    allowRuleSuggestions: false,
    allowEvidenceRefs: false,
    allowRawContent: false,
    requiresManualReview: false,
    fallbackWhenUnavailable: null,
  };
}

function defaultPlatformCapability() {
  return {
    enabled: false,
    state: null,
    allowedAdapters: [],
    manualRequiredAdapters: [],
    authoringOnly: false,
    mayRunCapture: false,
    mayConnectToBrowserBridge: false,
  };
}

function defaultPlatforms() {
  return {
    windows: defaultPlatformCapability(),
    macos: defaultPlatformCapability(),
    linux: defaultPlatformCapability(),
    android: defaultPlatformCapability(),
    ios: defaultPlatformCapability(),
    webPortal: defaultPlatformCapability(),
  };
}

function defaultFallbacks() {
  return {
    managedProfileMissing: null,
    bridgeMissing: null,
    extensionDisabled: null,
    nativeHostMissing: null,
    unsupportedBrowser: null,
    staleEvidence: null,
    networkAdapterUnavailable: null,
    processControlUnavailable: null,
    enforcementFailure: null,
    childDeviceOffline: null,
    platformUnsupported: null,
  };
}
