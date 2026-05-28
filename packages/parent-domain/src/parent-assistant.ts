import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiDegradedStateSchema,
  LocalAiModelIdSchema,
  LocalAiProviderIdSchema,
  LocalAiResultIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnavailableReasonSchema,
} from './local-ai-primitives';
import { LocalAiProviderSchedulerJobStatusSchema } from './local-ai-provider-scheduler';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './references';
import { ParentContractSchemaVersionSchema } from './reference-primitives';

const NonEmptyParentAssistantText = Schema.String.pipe(Schema.minLength(1));
const ParentAssistantPositiveCount = Schema.Number.pipe(Schema.int(), Schema.positive());

export const ParentAssistantRequestIdSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantRequestId')
);
export const ParentAssistantThreadIdSchema = NonEmptyParentAssistantText.pipe(Schema.brand('ParentAssistantThreadId'));
export const ParentAssistantMessageIdSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantMessageId')
);
export const ParentAssistantPromptVersionSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantPromptVersion')
);
export const ParentAssistantQuestionSchema = NonEmptyParentAssistantText.pipe(Schema.brand('ParentAssistantQuestion'));
export const ParentAssistantApiProviderIdSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantApiProviderId')
);
export const ParentAssistantCustodyLabelSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantCustodyLabel')
);
export const ParentAssistantRetentionPolicySchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantRetentionPolicy')
);
export const ParentAssistantDeletionPolicySchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantDeletionPolicy')
);
export const ParentAssistantAnswerTextSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantAnswerText')
);
export const ParentAssistantCitationLabelSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantCitationLabel')
);
export const ParentAssistantActionPreviewIdSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantActionPreviewId')
);
export const ParentAssistantRunIdSchema = NonEmptyParentAssistantText.pipe(Schema.brand('ParentAssistantRunId'));
export const ParentAssistantActionIntentIdSchema = NonEmptyParentAssistantText.pipe(
  Schema.brand('ParentAssistantActionIntentId')
);

export const ParentAssistantBackendStateSchema = withParser(
  Schema.Literal('runtime-backed', 'volatile-local', 'contract-required', 'unavailable')
);
export const ParentAssistantProviderStateSchema = withParser(Schema.Literal('configured', 'degraded', 'unavailable'));
export const ParentAssistantAnswerStateSchema = withParser(
  Schema.Literal('answered', 'queued', 'degraded', 'unavailable')
);
export const ParentAssistantApiAuthorizationStateSchema = withParser(Schema.Literal('authorized', 'not-authorized'));
export const ParentAssistantActionPreviewKindSchema = withParser(
  Schema.Literal('none', 'policy-suggestion', 'schedule-change', 'time-limit-change')
);
export const ParentAssistantThreadStateSchema = withParser(Schema.Literal('open', 'archived'));
export const ParentAssistantRunCancelStateSchema = withParser(
  Schema.Literal('cancelled', 'not-running', 'unavailable')
);
export const ParentAssistantActionConfirmStateSchema = withParser(
  Schema.Literal('contract-required', 'not-applied', 'rejected')
);

export const ParentAssistantScopeSchema = withParser(
  Schema.Struct({
    family: FamilyReferenceSchema,
    device: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
  })
);

export const ParentAssistantEvidenceContextSchema = withParser(
  Schema.Struct({
    evidence: ParentEvidenceReferenceSchema,
    citationLabel: ParentAssistantCitationLabelSchema,
    allowedSummary: ParentAssistantAnswerTextSchema,
  })
);

export const ParentAssistantActionPreviewSchema = withParser(
  Schema.Struct({
    previewId: Schema.Union(ParentAssistantActionPreviewIdSchema, Schema.Null),
    actionKind: ParentAssistantActionPreviewKindSchema,
    summary: Schema.Union(ParentAssistantAnswerTextSchema, Schema.Null),
    actionReference: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    requiresControllerLease: Schema.Boolean,
    childAgentContractRequired: Schema.Literal(true),
    enforcementApplied: Schema.Literal(false),
  })
);

export const ParentAssistantGenerateRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    requestId: ParentAssistantRequestIdSchema,
    threadId: ParentAssistantThreadIdSchema,
    messageId: ParentAssistantMessageIdSchema,
    askedAt: LocalAiTimestampSchema,
    actor: ParentActorReferenceSchema,
    scope: ParentAssistantScopeSchema,
    question: ParentAssistantQuestionSchema,
    evidenceContext: Schema.Array(ParentAssistantEvidenceContextSchema),
    modelId: Schema.Union(LocalAiModelIdSchema, Schema.Null),
    maxOutputTokens: ParentAssistantPositiveCount,
    timeoutMs: ParentAssistantPositiveCount,
  })
);

const ParentAssistantApiProviderBoundaryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  providerId: ParentAssistantApiProviderIdSchema,
  authorizationState: ParentAssistantApiAuthorizationStateSchema,
  custodyLabel: ParentAssistantCustodyLabelSchema,
  retentionPolicy: ParentAssistantRetentionPolicySchema,
  deletionPolicy: ParentAssistantDeletionPolicySchema,
  citations: Schema.Array(ParentAssistantEvidenceContextSchema),
  providerState: ParentAssistantProviderStateSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  childSafetyOrEnforcementUseAllowed: Schema.Literal(false),
});

type ParentAssistantApiProviderBoundaryCandidate = Infer<typeof ParentAssistantApiProviderBoundaryBaseSchema>;

export const ParentAssistantApiProviderBoundarySchema = withParser(
  ParentAssistantApiProviderBoundaryBaseSchema.pipe(
    Schema.filter(
      (boundary) =>
        parentAssistantApiProviderBoundaryIsConsistent(boundary) ||
        'Expected API AI provider use to require parent authorization, citations, custody, retention, deletion, and no child-safety enforcement use'
    )
  )
);

const ParentAssistantAnswerBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  requestId: ParentAssistantRequestIdSchema,
  threadId: ParentAssistantThreadIdSchema,
  messageId: ParentAssistantMessageIdSchema,
  answeredAt: LocalAiTimestampSchema,
  providerId: LocalAiProviderIdSchema,
  modelId: LocalAiModelIdSchema,
  providerState: ParentAssistantProviderStateSchema,
  answerState: ParentAssistantAnswerStateSchema,
  schedulerJobStatus: LocalAiProviderSchedulerJobStatusSchema,
  degradedState: LocalAiDegradedStateSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  localAiResultId: Schema.Union(LocalAiResultIdSchema, Schema.Null),
  answerText: Schema.Union(ParentAssistantAnswerTextSchema, Schema.Null),
  citations: Schema.Array(ParentAssistantEvidenceContextSchema),
  actionPreview: ParentAssistantActionPreviewSchema,
  apiProviderBoundary: ParentAssistantApiProviderBoundarySchema,
  promptVersion: ParentAssistantPromptVersionSchema,
});

type ParentAssistantAnswerCandidate = Infer<typeof ParentAssistantAnswerBaseSchema>;

export const ParentAssistantAnswerSchema = withParser(
  ParentAssistantAnswerBaseSchema.pipe(
    Schema.filter(
      (answer) =>
        parentAssistantAnswerIsConsistent(answer) ||
        'Expected parent assistant answers to cite evidence when answered and expose unavailable reason when unavailable'
    )
  )
);

export const ParentAssistantThreadRecordSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    threadId: ParentAssistantThreadIdSchema,
    title: ParentAssistantAnswerTextSchema,
    state: ParentAssistantThreadStateSchema,
    backendState: ParentAssistantBackendStateSchema,
    createdAt: LocalAiTimestampSchema,
    updatedAt: LocalAiTimestampSchema,
    messageCount: Schema.Number.pipe(Schema.int()),
  })
);

export const ParentAssistantThreadResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    backendState: ParentAssistantBackendStateSchema,
    activeThread: Schema.Union(ParentAssistantThreadRecordSchema, Schema.Null),
    threads: Schema.Array(ParentAssistantThreadRecordSchema),
    reason: Schema.Union(ParentAssistantAnswerTextSchema, Schema.Null),
  })
);

export const ParentAssistantProviderStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    backendState: ParentAssistantBackendStateSchema,
    providerId: LocalAiProviderIdSchema,
    modelId: LocalAiModelIdSchema,
    providerState: ParentAssistantProviderStateSchema,
    schedulerJobStatus: LocalAiProviderSchedulerJobStatusSchema,
    degradedState: LocalAiDegradedStateSchema,
    unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
    queueDepth: Schema.Number.pipe(Schema.int()),
    busy: Schema.Boolean,
    apiProviderBoundary: ParentAssistantApiProviderBoundarySchema,
  })
);

export const ParentAssistantRunCancelResultSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    backendState: ParentAssistantBackendStateSchema,
    threadId: ParentAssistantThreadIdSchema,
    runId: ParentAssistantRunIdSchema,
    cancelState: ParentAssistantRunCancelStateSchema,
    providerState: ParentAssistantProviderStateSchema,
    unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  })
);

const ParentAssistantActionConfirmResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  backendState: ParentAssistantBackendStateSchema,
  actionIntentId: ParentAssistantActionIntentIdSchema,
  previewId: Schema.Union(ParentAssistantActionPreviewIdSchema, Schema.Null),
  actionKind: ParentAssistantActionPreviewKindSchema,
  confirmState: ParentAssistantActionConfirmStateSchema,
  requiresControllerLease: Schema.Literal(true),
  childAgentContractRequired: Schema.Literal(true),
  enforcementApplied: Schema.Literal(false),
  policyWritten: Schema.Literal(false),
  reason: ParentAssistantAnswerTextSchema,
});

type ParentAssistantActionConfirmResultCandidate = Infer<typeof ParentAssistantActionConfirmResultBaseSchema>;

export const ParentAssistantActionConfirmResultSchema = withParser(
  ParentAssistantActionConfirmResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        parentAssistantActionConfirmResultIsSafe(result) ||
        'Expected Parent Assistant action confirm to require controller/child-agent contract and avoid direct enforcement or policy writes'
    )
  )
);

function parentAssistantAnswerIsConsistent(answer: ParentAssistantAnswerCandidate): boolean {
  if (answer.answerState === 'answered') {
    return (
      answer.answerText !== null &&
      answer.citations.length > 0 &&
      answer.unavailableReason === null &&
      answer.providerState === 'configured'
    );
  }

  if (answer.answerState === 'unavailable') {
    return answer.answerText === null && answer.unavailableReason !== null && answer.providerState === 'unavailable';
  }

  if (answer.answerState === 'degraded') {
    return answer.degradedState !== 'none' && answer.providerState === 'degraded';
  }

  return answer.schedulerJobStatus === 'queued' && answer.answerText === null;
}

function parentAssistantApiProviderBoundaryIsConsistent(
  boundary: ParentAssistantApiProviderBoundaryCandidate
): boolean {
  if (boundary.citations.length === 0 || boundary.childSafetyOrEnforcementUseAllowed !== false) {
    return false;
  }

  if (boundary.authorizationState === 'not-authorized') {
    return boundary.providerState === 'unavailable' && boundary.unavailableReason !== null;
  }

  return boundary.providerState !== 'unavailable' || boundary.unavailableReason !== null;
}

function parentAssistantActionConfirmResultIsSafe(result: ParentAssistantActionConfirmResultCandidate): boolean {
  return (
    result.requiresControllerLease === true &&
    result.childAgentContractRequired === true &&
    result.enforcementApplied === false &&
    result.policyWritten === false &&
    result.confirmState === 'contract-required'
  );
}

export type ParentAssistantScope = Infer<typeof ParentAssistantScopeSchema>;
export type ParentAssistantEvidenceContext = Infer<typeof ParentAssistantEvidenceContextSchema>;
export type ParentAssistantActionPreview = Infer<typeof ParentAssistantActionPreviewSchema>;
export type ParentAssistantGenerateRequest = Infer<typeof ParentAssistantGenerateRequestSchema>;
export type ParentAssistantAnswer = Infer<typeof ParentAssistantAnswerSchema>;
export type ParentAssistantApiProviderBoundary = Infer<typeof ParentAssistantApiProviderBoundarySchema>;
export type ParentAssistantThreadRecord = Infer<typeof ParentAssistantThreadRecordSchema>;
export type ParentAssistantThreadResponse = Infer<typeof ParentAssistantThreadResponseSchema>;
export type ParentAssistantProviderStatus = Infer<typeof ParentAssistantProviderStatusSchema>;
export type ParentAssistantRunCancelResult = Infer<typeof ParentAssistantRunCancelResultSchema>;
export type ParentAssistantActionConfirmResult = Infer<typeof ParentAssistantActionConfirmResultSchema>;
