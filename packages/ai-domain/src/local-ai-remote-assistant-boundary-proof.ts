import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentActionReferenceSchema, ParentActorReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import { ParentContractSchemaVersion, ParentContractSchemaVersionSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { PolicyDecisionSchema, PolicyReasonCodeSchema, comparePolicyActionStrictness } from './policy';
import { LocalAiSafetyResultSchema } from './local-ai';
import { LocalAiPromptVersionSchema } from './local-ai-primitives';

const NonEmptyRemoteAssistantBoundaryText = Schema.String.pipe(Schema.minLength(1));
const RemoteAssistantBoundaryCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const RemoteAssistantRequestIdSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('RemoteAssistantRequestId')
);
export const RemoteAssistantResultIdSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('RemoteAssistantResultId')
);
export const RemoteAssistantQuestionRefSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('RemoteAssistantQuestionRef')
);
export const RemoteAssistantReportBundleRefSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('RemoteAssistantReportBundleRef')
);
export const RemoteAssistantAnswerRefSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('RemoteAssistantAnswerRef')
);
export const RemoteAssistantUncertaintyCodeSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('RemoteAssistantUncertaintyCode')
);
export const RemoteAssistantFailureReasonSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('RemoteAssistantFailureReason')
);
export const LocalAiRemoteAssistantBoundaryProofIdSchema = NonEmptyRemoteAssistantBoundaryText.pipe(
  Schema.brand('LocalAiRemoteAssistantBoundaryProofId')
);

export const RemoteAssistantCustodyBoundarySchema = withParser(
  Schema.Literal('parent-authorized-report-bundle', 'parent-owned-local-storage')
);

export const RemoteAssistantExecutionStateSchema = withParser(
  Schema.Literal('ready-answer', 'local-only-fallback', 'rejected')
);

export const RemoteAssistantRequestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  requestId: RemoteAssistantRequestIdSchema,
  parentActor: ParentActorReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  questionRef: RemoteAssistantQuestionRefSchema,
  approvedSourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  permittedReportBundleRefs: Schema.Array(RemoteAssistantReportBundleRefSchema),
  custodyBoundary: RemoteAssistantCustodyBoundarySchema,
  modelProviderId: NonEmptyRemoteAssistantBoundaryText.pipe(Schema.brand('RemoteAssistantModelProviderId')),
  modelId: NonEmptyRemoteAssistantBoundaryText.pipe(Schema.brand('RemoteAssistantModelId')),
  promptVersion: LocalAiPromptVersionSchema,
  rawPromptRetained: Schema.Boolean,
  childSafetyDecisionPath: Schema.Boolean,
  parentAuthorizedRemoteUse: Schema.Boolean,
});

type RemoteAssistantRequestCandidate = Infer<typeof RemoteAssistantRequestBaseSchema>;

export const RemoteAssistantRequestSchema = withParser(
  RemoteAssistantRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        remoteAssistantRequestIsAuthorized(request) ||
        'Expected remote assistant request to be parent-authorized, evidence-cited, non-retaining, and outside child safety'
    )
  )
);

export const RemoteAssistantResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  resultId: RemoteAssistantResultIdSchema,
  requestId: RemoteAssistantRequestIdSchema,
  executionState: RemoteAssistantExecutionStateSchema,
  answerRef: Schema.Union(RemoteAssistantAnswerRefSchema, Schema.Null),
  citedEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  uncertaintyCodes: Schema.Array(RemoteAssistantUncertaintyCodeSchema),
  failureReason: Schema.Union(RemoteAssistantFailureReasonSchema, Schema.Null),
  localAiResult: LocalAiSafetyResultSchema,
  localPolicyDecision: PolicyDecisionSchema,
  remoteSuggestedPolicyDecision: Schema.Union(PolicyDecisionSchema, Schema.Null),
  remoteOutputAllowedToOverrideLocalPolicy: Schema.Boolean,
  policyAuthorityClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  childSafetyDecisionPath: Schema.Boolean,
  rawPromptRetained: Schema.Boolean,
  rawModelOutputRetained: Schema.Boolean,
  remoteApiAiUsed: Schema.Boolean,
});

type RemoteAssistantResultCandidate = Infer<typeof RemoteAssistantResultBaseSchema>;

export const RemoteAssistantResultSchema = withParser(
  RemoteAssistantResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        remoteAssistantResultStaysOutsideSafetyAuthority(result) ||
        'Expected remote assistant result to cite evidence, preserve local policy authority, and stay non-enforcing'
    )
  )
);

export const LocalAiRemoteAssistantBoundaryProofSchema = withParser(
  Schema.Struct({
    proofId: LocalAiRemoteAssistantBoundaryProofIdSchema,
    generatedAt: NonEmptyRemoteAssistantBoundaryText,
    readyRequest: RemoteAssistantRequestSchema,
    readyResult: RemoteAssistantResultSchema,
    fallbackResult: RemoteAssistantResultSchema,
    validationSummary: Schema.Struct({
      approvedEvidenceReferenceCount: RemoteAssistantBoundaryCountSchema,
      citedEvidenceReferenceCount: RemoteAssistantBoundaryCountSchema,
      permittedReportBundleCount: RemoteAssistantBoundaryCountSchema,
      rejectedOverclaimCount: RemoteAssistantBoundaryCountSchema,
    }),
    rejectedReasonCodes: Schema.Array(PolicyReasonCodeSchema),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.readyRequest.approvedSourceEvidenceReferences.length ===
          proof.validationSummary.approvedEvidenceReferenceCount &&
        proof.readyResult.citedEvidenceReferences.length === proof.validationSummary.citedEvidenceReferenceCount &&
        proof.readyRequest.permittedReportBundleRefs.length === proof.validationSummary.permittedReportBundleCount &&
        proof.fallbackResult.executionState === 'local-only-fallback' &&
        proof.rejectedReasonCodes.length === proof.validationSummary.rejectedOverclaimCount
    )
  )
);

export type RemoteAssistantRequest = Infer<typeof RemoteAssistantRequestSchema>;
export type RemoteAssistantResult = Infer<typeof RemoteAssistantResultSchema>;
export type LocalAiRemoteAssistantBoundaryProof = Infer<typeof LocalAiRemoteAssistantBoundaryProofSchema>;

const generatedAt = '2026-06-06T09:36:00.000Z';

const evidenceReference = {
  evidenceReferenceId: 'evidence:local-ai:screen-summary-parent-bundle',
  kind: 'query-store-summary',
  observedAt: '2026-06-06T09:35:00.000Z',
} as const;

const parentActor = {
  actorId: 'parent:remote-assistant-reviewer',
  role: 'parent',
} as const;

const parentAction = {
  actionReferenceId: 'parent-action:authorize-remote-assistant-report',
  actor: parentActor,
  policyVersion: 'policy-version:remote-assistant-boundary',
  createdAt: generatedAt,
} as const;

const localAiResult = LocalAiSafetyResultSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  resultId: 'local-ai-result:remote-boundary-source',
  requestId: 'local-ai-request:remote-boundary-source',
  action: 'warn',
  confidence: 0.72,
  unknownState: 'none',
  degradedState: 'none',
  reasonCodes: ['local-ai-remote-boundary-source'],
  explanationReference: 'local-ai-explanation:remote-boundary-source',
  evidenceReferences: [evidenceReference],
  parentRuleReferences: ['policy-rule:video-warn'],
  memoryReferences: [],
  graphReferences: [],
  modelRuntime: {
    runtimeReferenceId: 'runtime:local-ai-remote-boundary',
    providerId: 'local-provider-llama-cli',
    modelId: 'screen-safety-local-model',
    modelReference: 'artifact:screen_safety_local_model',
    privacyMode: 'local-only',
    adapterBoundary: 'local-adapter-ready',
    executionState: 'dry-run-ready',
    providerSource: 'local-model-cache',
    loadState: 'loaded',
    capabilityFlags: ['safety-decision', 'classification'],
    resourceClass: 'cpu',
    degradedState: 'none',
    lastCheckedAt: generatedAt,
    unavailableReason: null,
  },
  promptVersion: 'prompt:screen-safety:v1',
  expiresAt: null,
});

const localPolicyDecision = PolicyDecisionSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionId: 'policy-decision:local-ai-remote-boundary',
  action: 'block',
  reasonCodes: ['parent-rule-stricter-than-ai'],
  evidenceReferences: [evidenceReference],
  ruleIds: ['policy-rule:video-block'],
  localAiResultId: localAiResult.resultId,
  dryRun: true,
  enforcementHandoffState: 'disabled',
  expiresAt: null,
});

const remoteSuggestedPolicyDecision = PolicyDecisionSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  decisionId: 'policy-decision:remote-assistant-suggested',
  action: 'allow',
  reasonCodes: ['remote-assistant-suggested-context'],
  evidenceReferences: [evidenceReference],
  ruleIds: ['policy-rule:video-warn'],
  localAiResultId: localAiResult.resultId,
  dryRun: true,
  enforcementHandoffState: 'disabled',
  expiresAt: null,
});

export const LocalAiRemoteAssistantBoundaryProof = LocalAiRemoteAssistantBoundaryProofSchema.parse({
  proofId: 'local-ai-remote-assistant-boundary-proof',
  generatedAt,
  readyRequest: {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    requestId: 'remote-assistant-request:parent-report-review',
    parentActor,
    parentAction,
    questionRef: 'question:why-was-video-blocked',
    approvedSourceEvidenceReferences: [evidenceReference],
    permittedReportBundleRefs: ['report-bundle:parent-owned-local-ai-review'],
    custodyBoundary: 'parent-authorized-report-bundle',
    modelProviderId: 'remote-provider:parent-authorized-api',
    modelId: 'remote-model:explanation-only',
    promptVersion: 'prompt:remote-assistant-report:v1',
    rawPromptRetained: false,
    childSafetyDecisionPath: false,
    parentAuthorizedRemoteUse: true,
  },
  readyResult: {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'remote-assistant-result:parent-report-review',
    requestId: 'remote-assistant-request:parent-report-review',
    executionState: 'ready-answer',
    answerRef: 'answer:remote-assistant-parent-report-review',
    citedEvidenceReferences: [evidenceReference],
    uncertaintyCodes: ['remote-answer-non-authoritative'],
    failureReason: null,
    localAiResult,
    localPolicyDecision,
    remoteSuggestedPolicyDecision,
    remoteOutputAllowedToOverrideLocalPolicy: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    childSafetyDecisionPath: false,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    remoteApiAiUsed: true,
  },
  fallbackResult: {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    resultId: 'remote-assistant-result:local-only-fallback',
    requestId: 'remote-assistant-request:parent-report-review',
    executionState: 'local-only-fallback',
    answerRef: null,
    citedEvidenceReferences: [evidenceReference],
    uncertaintyCodes: ['remote-assistant-unavailable'],
    failureReason: 'remote-provider-unavailable',
    localAiResult,
    localPolicyDecision,
    remoteSuggestedPolicyDecision: null,
    remoteOutputAllowedToOverrideLocalPolicy: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    childSafetyDecisionPath: false,
    rawPromptRetained: false,
    rawModelOutputRetained: false,
    remoteApiAiUsed: false,
  },
  validationSummary: {
    approvedEvidenceReferenceCount: 1,
    citedEvidenceReferenceCount: 1,
    permittedReportBundleCount: 1,
    rejectedOverclaimCount: 4,
  },
  rejectedReasonCodes: [
    'remote-assistant-child-safety-path-rejected',
    'remote-assistant-no-evidence-rejected',
    'remote-assistant-raw-retention-rejected',
    'remote-assistant-policy-override-rejected',
  ],
});

function remoteAssistantRequestIsAuthorized(request: RemoteAssistantRequestCandidate): boolean {
  return (
    request.parentAuthorizedRemoteUse &&
    !request.childSafetyDecisionPath &&
    !request.rawPromptRetained &&
    request.approvedSourceEvidenceReferences.length > 0 &&
    request.permittedReportBundleRefs.length > 0
  );
}

function remoteAssistantResultStaysOutsideSafetyAuthority(result: RemoteAssistantResultCandidate): boolean {
  if (
    result.policyAuthorityClaimed ||
    result.enforcementClaimed ||
    result.childSafetyDecisionPath ||
    result.rawPromptRetained ||
    result.rawModelOutputRetained ||
    result.remoteOutputAllowedToOverrideLocalPolicy
  ) {
    return false;
  }

  if (!resultCitesLocalDecisionEvidence(result)) {
    return false;
  }

  return remoteSuggestionCannotWeakenLocalPolicy(result);
}

function resultCitesLocalDecisionEvidence(result: RemoteAssistantResultCandidate): boolean {
  const localEvidenceIds = new Set(
    result.localPolicyDecision.evidenceReferences.map((reference) => reference.evidenceReferenceId)
  );

  return (
    String(result.localAiResult.resultId) === String(result.localPolicyDecision.localAiResultId) &&
    result.citedEvidenceReferences.length > 0 &&
    result.citedEvidenceReferences.every((reference) => localEvidenceIds.has(reference.evidenceReferenceId))
  );
}

function remoteSuggestionCannotWeakenLocalPolicy(result: RemoteAssistantResultCandidate): boolean {
  if (result.remoteSuggestedPolicyDecision === null) {
    return true;
  }

  return (
    comparePolicyActionStrictness(result.localPolicyDecision.action, result.remoteSuggestedPolicyDecision.action) >= 0
  );
}
