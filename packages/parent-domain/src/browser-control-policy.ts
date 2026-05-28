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
  type BrowserControlRequestId,
} from './browser-control-identifiers';
import {
  BrowserControlAuditStateSchema,
  BrowserControlApprovalStateSchema,
  BrowserControlCapabilityStateSchema,
  BrowserControlDefaultPostureSchema,
  BrowserControlDownloadStateSchema,
  BrowserControlEvidenceProofLevelSchema,
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
    requiredProof: BrowserControlEvidenceProofLevelSchema,
    proofFallback: Schema.Union(BrowserControlProofFallbackSchema, Schema.Null),
  })
);

export const BrowserControlRuleSchema = withParser(
  Schema.Struct({
    ruleId: BrowserControlRuleIdSchema,
    targetType: BrowserControlUrlTargetTypeSchema,
    targetValue: BrowserControlPolicyTextSchema,
    enabled: Schema.Boolean,
  })
);

export const BrowserControlPolicyValueBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  policyId: BrowserControlPolicyIdSchema,
  enabled: Schema.Boolean,
  defaultPosture: BrowserControlDefaultPostureSchema,
  fallbackPosture: Schema.Union(BrowserControlDefaultPostureSchema, Schema.Null),
  managementMode: BrowserControlManagementModeSchema,
  managedBrowser: Schema.Struct({
    mode: BrowserControlManagedBrowserModeSchema,
  }),
  unmanagedBrowser: Schema.Struct({
    mode: BrowserControlUnmanagedBrowserModeSchema,
  }),
  evidence: BrowserControlEvidenceRequirementSchema,
  rules: Schema.Struct({
    allowedTargetTypes: Schema.Array(BrowserControlUrlTargetTypeSchema),
    entries: Schema.Array(BrowserControlRuleSchema),
  }),
  budgets: Schema.Struct({
    defaultDailyMinutes: Schema.Union(Schema.Number, Schema.Null),
  }),
  downloads: Schema.Struct({
    state: BrowserControlDownloadStateSchema,
  }),
  approvals: Schema.Struct({
    state: BrowserControlApprovalStateSchema,
  }),
  reports: Schema.Struct({
    state: BrowserControlReportStateSchema,
  }),
  audit: Schema.Struct({
    state: BrowserControlAuditStateSchema,
  }),
  retention: Schema.Struct({
    state: BrowserControlRetentionStateSchema,
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
    )
  )
);

export const BrowserControlEffectiveRuleSchema = withParser(
  Schema.Struct({
    ruleId: BrowserControlRuleIdSchema,
    targetType: BrowserControlUrlTargetTypeSchema,
    targetValue: BrowserControlPolicyTextSchema,
    defaultPosture: BrowserControlDefaultPostureSchema,
    evidence: BrowserControlEvidenceRequirementSchema,
  })
);

export const BrowserControlEffectivePolicyBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  policyId: BrowserControlPolicyIdSchema,
  revisionId: BrowserControlRevisionIdSchema,
  compiledHash: BrowserControlHashIdSchema,
  compiledAt: ParentTimestampSchema,
  defaultPosture: BrowserControlDefaultPostureSchema,
  fallbackPosture: Schema.Union(BrowserControlDefaultPostureSchema, Schema.Null),
  budgets: Schema.Struct({
    defaultDailyMinutes: Schema.Union(Schema.Number, Schema.Null),
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

export const BrowserControlPatchSchema = withParser(
  Schema.Struct({
    op: BrowserControlPatchOperationSchema,
    fieldId: BrowserControlFieldIdSchema,
    writesTo: BrowserControlSchemaKnownWritesToPathSchema,
    value: BrowserControlFieldValueSchema,
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
export type BrowserControlEvidenceRequirement = Infer<typeof BrowserControlEvidenceRequirementSchema>;
export type BrowserControlRule = Infer<typeof BrowserControlRuleSchema>;
export type BrowserControlPolicyValue = Infer<typeof BrowserControlPolicyValueSchema>;
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
    policy.defaultPosture !== 'limit' || policy.budgets.defaultDailyMinutes !== null || policy.fallbackPosture !== null
  );
}

function browserControlEffectiveLimitPostureIsConsistent(policy: BrowserControlEffectivePolicyCandidate): boolean {
  return (
    policy.defaultPosture !== 'limit' || policy.budgets.defaultDailyMinutes !== null || policy.fallbackPosture !== null
  );
}

function browserControlExactUrlPolicyIsHonest(policy: BrowserControlPolicyValueCandidate): boolean {
  if (!policy.rules.allowedTargetTypes.includes('exact-url')) {
    return true;
  }
  if (policy.evidence.proofFallback !== null) {
    return true;
  }
  return (
    policy.managedBrowser.mode === 'required-for-exact-rules' &&
    policy.evidence.requiredProof === 'fresh-managed-active-tab'
  );
}
