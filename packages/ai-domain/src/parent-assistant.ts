import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiDegradedStateSchema,
  LocalAiModelIdSchema,
  LocalAiProviderIdSchema,
  LocalAiResultIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnavailableReasonSchema,
} from './local-ai-primitives';
import { LocalAiProviderSchedulerJobStatusSchema } from '@ocentra-parent/ai-domain/local-ai-provider-scheduler';
import { LocalAiProviderSchedulerStatusSchema } from '@ocentra-parent/ai-domain/local-ai-provider-scheduler';
import { ParentAssistantRunStateSchema } from './parent-assistant-run-state';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ParentAssistantPositiveCount = Schema.Number.pipe(Schema.int(), Schema.positive());

export const ParentAssistantRequestIdSchema = brandedNonEmptyStringSchema('ParentAssistantRequestId');
export const ParentAssistantThreadIdSchema = brandedNonEmptyStringSchema('ParentAssistantThreadId');
export const ParentAssistantMessageIdSchema = brandedNonEmptyStringSchema('ParentAssistantMessageId');
export const ParentAssistantPromptVersionSchema = brandedNonEmptyStringSchema('ParentAssistantPromptVersion');
export const ParentAssistantQuestionSchema = brandedNonEmptyStringSchema('ParentAssistantQuestion');
export const ParentAssistantApiProviderIdSchema = brandedNonEmptyStringSchema('ParentAssistantApiProviderId');
export const ParentAssistantCustodyLabelSchema = brandedNonEmptyStringSchema('ParentAssistantCustodyLabel');
export const ParentAssistantRetentionPolicySchema = brandedNonEmptyStringSchema('ParentAssistantRetentionPolicy');
export const ParentAssistantDeletionPolicySchema = brandedNonEmptyStringSchema('ParentAssistantDeletionPolicy');
export const ParentAssistantAnswerTextSchema = brandedNonEmptyStringSchema('ParentAssistantAnswerText');
export const ParentAssistantCitationLabelSchema = brandedNonEmptyStringSchema('ParentAssistantCitationLabel');
const ParentAssistantEvidenceCustodyLabelSchema = withParser(
  Schema.Literal('parent-owned-activity-summary', 'parent-owned-activity-event', 'parent-owned-activity-report')
);
const ParentAssistantEvidenceSourceLabelSchema = withParser(
  Schema.Literal('activity-query-store-summary', 'activity-event-citation', 'saved-activity-report-history')
);
export const ParentAssistantActionPreviewIdSchema = brandedNonEmptyStringSchema('ParentAssistantActionPreviewId');
export const ParentAssistantRunIdSchema = brandedNonEmptyStringSchema('ParentAssistantRunId');
export const ParentAssistantActionIntentIdSchema = brandedNonEmptyStringSchema('ParentAssistantActionIntentId');

export const ParentAssistantBackendStateSchema = withParser(
  Schema.Literal('runtime-backed', 'durable-local', 'volatile-local', 'contract-required', 'unavailable')
);
export const ParentAssistantProviderStateSchema = withParser(Schema.Literal('configured', 'degraded', 'unavailable'));
export const ParentAssistantAnswerStateSchema = withParser(
  Schema.Literal('answered', 'queued', 'degraded', 'unavailable')
);
const ParentAssistantProviderSelectionSchema = withParser(Schema.Literal('local', 'api', 'none'));
const ParentAssistantProviderRoutingStateSchema = withParser(
  Schema.Literal(
    'local-provider-ready',
    'local-provider-degraded',
    'local-provider-unavailable',
    'api-provider-authorized-unavailable',
    'api-provider-authorized-degraded',
    'no-provider-available'
  )
);
const ParentAssistantApiAuthorizationStateSchema = withParser(Schema.Literal('authorized', 'not-authorized'));
const ParentAssistantApiProviderAccessStateSchema = withParser(
  Schema.Literal('not-authorized', 'authorized-unavailable', 'authorized-degraded')
);
const ParentAssistantApiProviderCustodyStateSchema = withParser(Schema.Literal('parent-owned-citations-only'));
const ParentAssistantApiProviderRetentionStateSchema = withParser(
  Schema.Literal('no-retention-without-parent-authorization', 'parent-authorized-no-default-retention')
);
const ParentAssistantApiProviderDeletionStateSchema = withParser(
  Schema.Literal('delete-provider-cache-on-parent-request')
);
export const ParentAssistantActionPreviewKindSchema = withParser(
  Schema.Literal('none', 'policy-suggestion', 'schedule-change', 'time-limit-change')
);
const ParentAssistantActionPreviewStateSchema = withParser(Schema.Literal('draft', 'unavailable', 'rejected'));
export const ParentAssistantThreadStateSchema = withParser(Schema.Literal('open', 'archived'));
export const ParentAssistantRunCancelStateSchema = withParser(
  Schema.Literal('cancelled', 'not-running', 'unavailable')
);
export const ParentAssistantActionConfirmStateSchema = withParser(
  Schema.Literal('contract-required', 'not-applied', 'rejected')
);
const ParentAssistantChildAgentValidationStateSchema = withParser(
  Schema.Literal(
    'child-agent-contract-required',
    'child-agent-offline',
    'child-agent-unavailable',
    'child-agent-degraded'
  )
);
const ParentAssistantActionAuditReasonSchema = brandedNonEmptyStringSchema('ParentAssistantActionAuditReason');

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
    custodyLabel: ParentAssistantEvidenceCustodyLabelSchema,
    sourceLabel: ParentAssistantEvidenceSourceLabelSchema,
    rawChildEvidenceIncluded: Schema.Literal(false),
    directEnforcementAllowed: Schema.Literal(false),
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

const ParentAssistantActionPreviewResultBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  backendState: ParentAssistantBackendStateSchema,
  actionIntentId: ParentAssistantActionIntentIdSchema,
  previewState: ParentAssistantActionPreviewStateSchema,
  preview: ParentAssistantActionPreviewSchema,
  evidenceContext: Schema.Array(ParentAssistantEvidenceContextSchema),
  previewRequired: Schema.Literal(true),
  previewSatisfied: Schema.Boolean,
  rawAssistantProseAccepted: Schema.Literal(false),
  parentConfirmationRequired: Schema.Literal(true),
  parentConfirmationRecorded: Schema.Literal(false),
  childAgentValidationState: ParentAssistantChildAgentValidationStateSchema,
  sourceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  auditReason: ParentAssistantActionAuditReasonSchema,
  requiresControllerLease: Schema.Boolean,
  childAgentContractRequired: Schema.Literal(true),
  enforcementApplied: Schema.Literal(false),
  policyWritten: Schema.Literal(false),
  reason: ParentAssistantAnswerTextSchema,
});

type ParentAssistantActionPreviewResultCandidate = Infer<typeof ParentAssistantActionPreviewResultBaseSchema>;

export const ParentAssistantActionPreviewResultSchema = withParser(
  ParentAssistantActionPreviewResultBaseSchema.pipe(
    Schema.filter(
      (result) =>
        parentAssistantActionPreviewResultIsSafe(result) ||
        'Expected Parent Assistant action preview to stay a draft and avoid direct enforcement or policy writes'
    )
  )
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

const ParentAssistantApiAuthorizationContextBaseSchema = Schema.Struct({
  authorizationState: ParentAssistantApiAuthorizationStateSchema,
  parentAuthorizationRequired: Schema.Literal(true),
  evidenceCitationRequired: Schema.Literal(true),
  custodyLabel: ParentAssistantCustodyLabelSchema,
  retentionState: ParentAssistantApiProviderRetentionStateSchema,
  deletionState: ParentAssistantApiProviderDeletionStateSchema,
});

type ParentAssistantApiAuthorizationContextCandidate = Infer<typeof ParentAssistantApiAuthorizationContextBaseSchema>;

export const ParentAssistantApiAuthorizationContextSchema = withParser(
  ParentAssistantApiAuthorizationContextBaseSchema.pipe(
    Schema.filter(
      (context) =>
        parentAssistantApiAuthorizationContextIsComplete(context) ||
        'Expected API AI authorization context to prove explicit parent authorization, custody, retention, deletion, and evidence-citation requirements'
    )
  )
);

const ParentAssistantApiProviderBoundaryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  providerId: ParentAssistantApiProviderIdSchema,
  authorizationState: ParentAssistantApiAuthorizationStateSchema,
  accessState: ParentAssistantApiProviderAccessStateSchema,
  parentAuthorizationRequired: Schema.Literal(true),
  evidenceCitationRequired: Schema.Literal(true),
  custodyLabel: ParentAssistantCustodyLabelSchema,
  custodyState: ParentAssistantApiProviderCustodyStateSchema,
  retentionPolicy: ParentAssistantRetentionPolicySchema,
  retentionState: ParentAssistantApiProviderRetentionStateSchema,
  deletionPolicy: ParentAssistantDeletionPolicySchema,
  deletionState: ParentAssistantApiProviderDeletionStateSchema,
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

const ParentAssistantProviderRouteBaseSchema = Schema.Struct({
  routingState: ParentAssistantProviderRoutingStateSchema,
  selectedProvider: ParentAssistantProviderSelectionSchema,
  localProviderState: ParentAssistantProviderStateSchema,
  apiProviderState: ParentAssistantProviderStateSchema,
  apiAccessState: ParentAssistantApiProviderAccessStateSchema,
  evidenceCitationRequired: Schema.Literal(true),
  remoteAiOptional: Schema.Literal(true),
  childSafetyOrEnforcementUseAllowed: Schema.Literal(false),
  reason: ParentAssistantAnswerTextSchema,
});

type ParentAssistantProviderRouteCandidate = Infer<typeof ParentAssistantProviderRouteBaseSchema>;
type ParentAssistantProviderRouteExpectation = Readonly<{
  selectedProvider: ParentAssistantProviderRouteCandidate['selectedProvider'];
  localProviderState: ParentAssistantProviderRouteCandidate['localProviderState'];
  apiAccessState?: ParentAssistantProviderRouteCandidate['apiAccessState'];
  apiProviderState?: ParentAssistantProviderRouteCandidate['apiProviderState'];
}>;

const ParentAssistantProviderRouteExpectations = {
  'local-provider-ready': {
    selectedProvider: 'local',
    localProviderState: 'configured',
  },
  'local-provider-degraded': {
    selectedProvider: 'local',
    localProviderState: 'degraded',
  },
  'local-provider-unavailable': {
    selectedProvider: 'none',
    localProviderState: 'unavailable',
    apiAccessState: 'not-authorized',
  },
  'api-provider-authorized-unavailable': {
    selectedProvider: 'none',
    localProviderState: 'unavailable',
    apiAccessState: 'authorized-unavailable',
    apiProviderState: 'unavailable',
  },
  'api-provider-authorized-degraded': {
    selectedProvider: 'none',
    localProviderState: 'unavailable',
    apiAccessState: 'authorized-degraded',
    apiProviderState: 'degraded',
  },
  'no-provider-available': {
    selectedProvider: 'none',
    localProviderState: 'unavailable',
    apiAccessState: 'not-authorized',
    apiProviderState: 'unavailable',
  },
} as const satisfies Record<
  ParentAssistantProviderRouteCandidate['routingState'],
  ParentAssistantProviderRouteExpectation
>;

const ParentAssistantProviderRouteSchema = withParser(
  ParentAssistantProviderRouteBaseSchema.pipe(
    Schema.filter(
      (route) =>
        parentAssistantProviderRouteIsConsistent(route) ||
        'Expected Parent Assistant provider route to keep remote AI optional and disallow child-safety or enforcement use'
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
  runState: ParentAssistantRunStateSchema,
  schedulerJobStatus: LocalAiProviderSchedulerJobStatusSchema,
  degradedState: LocalAiDegradedStateSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  localAiResultId: Schema.Union(LocalAiResultIdSchema, Schema.Null),
  answerText: Schema.Union(ParentAssistantAnswerTextSchema, Schema.Null),
  citations: Schema.Array(ParentAssistantEvidenceContextSchema),
  actionPreview: ParentAssistantActionPreviewSchema,
  apiProviderBoundary: ParentAssistantApiProviderBoundarySchema,
  providerRoute: ParentAssistantProviderRouteSchema,
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

const ParentAssistantProviderStatusBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  backendState: ParentAssistantBackendStateSchema,
  providerId: LocalAiProviderIdSchema,
  modelId: LocalAiModelIdSchema,
  providerState: ParentAssistantProviderStateSchema,
  runState: ParentAssistantRunStateSchema,
  schedulerJobStatus: LocalAiProviderSchedulerJobStatusSchema,
  schedulerStatus: LocalAiProviderSchedulerStatusSchema,
  degradedState: LocalAiDegradedStateSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  queueDepth: Schema.Number.pipe(Schema.int()),
  busy: Schema.Boolean,
  apiProviderBoundary: ParentAssistantApiProviderBoundarySchema,
  providerRoute: ParentAssistantProviderRouteSchema,
});

type ParentAssistantProviderStatusCandidate = Infer<typeof ParentAssistantProviderStatusBaseSchema>;

export const ParentAssistantProviderStatusSchema = withParser(
  ParentAssistantProviderStatusBaseSchema.pipe(
    Schema.filter(
      (status) =>
        parentAssistantProviderRouteMatchesStatus(status) ||
        'Expected Parent Assistant provider status route to match local provider and API boundary states'
    )
  )
);

export const ParentAssistantRunCancelResultSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    backendState: ParentAssistantBackendStateSchema,
    threadId: ParentAssistantThreadIdSchema,
    runId: ParentAssistantRunIdSchema,
    cancelState: ParentAssistantRunCancelStateSchema,
    runState: ParentAssistantRunStateSchema,
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
  previewRequired: Schema.Literal(true),
  previewSatisfied: Schema.Boolean,
  rawAssistantProseAccepted: Schema.Literal(false),
  parentConfirmationRequired: Schema.Literal(true),
  parentConfirmationRecorded: Schema.Literal(false),
  childAgentValidationState: ParentAssistantChildAgentValidationStateSchema,
  sourceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  auditReason: ParentAssistantActionAuditReasonSchema,
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
  if (!parentAssistantProviderRouteMatchesAnswer(answer)) {
    return false;
  }

  switch (answer.answerState) {
    case 'answered':
      return answeredParentAssistantAnswerIsConsistent(answer);
    case 'unavailable':
      return unavailableParentAssistantAnswerIsConsistent(answer);
    case 'degraded':
      return degradedParentAssistantAnswerIsConsistent(answer);
    case 'queued':
      return queuedParentAssistantAnswerIsConsistent(answer);
  }

  return false;
}

function answeredParentAssistantAnswerIsConsistent(answer: ParentAssistantAnswerCandidate): boolean {
  return (
    answer.answerText !== null &&
    answer.citations.length > 0 &&
    answer.unavailableReason === null &&
    answer.providerState === 'configured' &&
    answer.runState === 'completed'
  );
}

function unavailableParentAssistantAnswerIsConsistent(answer: ParentAssistantAnswerCandidate): boolean {
  return (
    answer.answerText === null &&
    answer.unavailableReason !== null &&
    answer.providerState === 'unavailable' &&
    answer.runState === 'unavailable'
  );
}

function degradedParentAssistantAnswerIsConsistent(answer: ParentAssistantAnswerCandidate): boolean {
  return (
    answer.degradedState !== 'none' &&
    answer.providerState === 'degraded' &&
    (answer.runState === 'degraded' || answer.runState === 'failed')
  );
}

function queuedParentAssistantAnswerIsConsistent(answer: ParentAssistantAnswerCandidate): boolean {
  return answer.schedulerJobStatus === 'queued' && answer.answerText === null && answer.runState === 'queued';
}

function parentAssistantApiProviderBoundaryIsConsistent(
  boundary: ParentAssistantApiProviderBoundaryCandidate
): boolean {
  if (!parentAssistantApiProviderBoundaryHasRequiredProof(boundary)) {
    return false;
  }

  return parentAssistantApiProviderAccessStateIsConsistent(boundary);
}

function parentAssistantApiProviderBoundaryHasRequiredProof(
  boundary: ParentAssistantApiProviderBoundaryCandidate
): boolean {
  return (
    boundary.citations.length > 0 &&
    boundary.childSafetyOrEnforcementUseAllowed === false &&
    boundary.parentAuthorizationRequired === true &&
    boundary.evidenceCitationRequired === true
  );
}

function parentAssistantApiAuthorizationContextIsComplete(
  context: ParentAssistantApiAuthorizationContextCandidate
): boolean {
  return (
    context.authorizationState === 'authorized' &&
    context.parentAuthorizationRequired === true &&
    context.evidenceCitationRequired === true &&
    context.custodyLabel === 'parent-authorized-api-ai' &&
    context.retentionState === 'parent-authorized-no-default-retention' &&
    context.deletionState === 'delete-provider-cache-on-parent-request'
  );
}

function parentAssistantApiProviderAccessStateIsConsistent(
  boundary: ParentAssistantApiProviderBoundaryCandidate
): boolean {
  switch (boundary.accessState) {
    case 'not-authorized':
      return notAuthorizedApiProviderBoundaryIsConsistent(boundary);
    case 'authorized-unavailable':
      return authorizedUnavailableApiProviderBoundaryIsConsistent(boundary);
    case 'authorized-degraded':
      return authorizedDegradedApiProviderBoundaryIsConsistent(boundary);
  }

  return false;
}

function parentAssistantProviderRouteIsConsistent(route: ParentAssistantProviderRouteCandidate): boolean {
  if (!parentAssistantProviderRouteKeepsSafetyBoundaries(route)) {
    return false;
  }

  if (!parentAssistantProviderRouteApiStateIsConsistent(route)) {
    return false;
  }

  return parentAssistantProviderRouteMatchesExpectedState(
    route,
    ParentAssistantProviderRouteExpectations[route.routingState]
  );
}

function parentAssistantProviderRouteKeepsSafetyBoundaries(route: ParentAssistantProviderRouteCandidate): boolean {
  return (
    route.evidenceCitationRequired === true &&
    route.remoteAiOptional === true &&
    route.childSafetyOrEnforcementUseAllowed === false
  );
}

function parentAssistantProviderRouteMatchesExpectedState(
  route: ParentAssistantProviderRouteCandidate,
  expected: ParentAssistantProviderRouteExpectation
): boolean {
  return (
    route.selectedProvider === expected.selectedProvider &&
    route.localProviderState === expected.localProviderState &&
    optionalParentAssistantProviderRouteValueMatches(route.apiAccessState, expected.apiAccessState) &&
    optionalParentAssistantProviderRouteValueMatches(route.apiProviderState, expected.apiProviderState)
  );
}

function optionalParentAssistantProviderRouteValueMatches<Value>(actual: Value, expected: Value | undefined): boolean {
  return expected === undefined || actual === expected;
}

function parentAssistantProviderRouteApiStateIsConsistent(route: ParentAssistantProviderRouteCandidate): boolean {
  if (route.apiAccessState === 'authorized-degraded') {
    return route.apiProviderState === 'degraded';
  }

  return route.apiProviderState === 'unavailable';
}

function parentAssistantProviderRouteMatchesAnswer(answer: ParentAssistantAnswerCandidate): boolean {
  return (
    answer.providerRoute.localProviderState === answer.providerState &&
    parentAssistantProviderRouteMatchesBoundary(answer.providerRoute, answer.apiProviderBoundary)
  );
}

function parentAssistantProviderRouteMatchesStatus(status: ParentAssistantProviderStatusCandidate): boolean {
  return (
    status.providerRoute.localProviderState === status.providerState &&
    parentAssistantProviderRouteMatchesBoundary(status.providerRoute, status.apiProviderBoundary)
  );
}

function parentAssistantProviderRouteMatchesBoundary(
  route: ParentAssistantProviderRouteCandidate,
  boundary: ParentAssistantApiProviderBoundaryCandidate
): boolean {
  return route.apiAccessState === boundary.accessState && route.apiProviderState === boundary.providerState;
}

function notAuthorizedApiProviderBoundaryIsConsistent(boundary: ParentAssistantApiProviderBoundaryCandidate): boolean {
  return (
    boundary.authorizationState === 'not-authorized' &&
    boundary.providerState === 'unavailable' &&
    boundary.unavailableReason !== null &&
    boundary.retentionState === 'no-retention-without-parent-authorization'
  );
}

function authorizedUnavailableApiProviderBoundaryIsConsistent(
  boundary: ParentAssistantApiProviderBoundaryCandidate
): boolean {
  return (
    boundary.authorizationState === 'authorized' &&
    boundary.providerState === 'unavailable' &&
    boundary.unavailableReason !== null &&
    boundary.retentionState === 'parent-authorized-no-default-retention'
  );
}

function authorizedDegradedApiProviderBoundaryIsConsistent(
  boundary: ParentAssistantApiProviderBoundaryCandidate
): boolean {
  return (
    boundary.authorizationState === 'authorized' &&
    boundary.providerState === 'degraded' &&
    boundary.unavailableReason !== null &&
    boundary.retentionState === 'parent-authorized-no-default-retention'
  );
}

function parentAssistantActionConfirmResultIsSafe(result: ParentAssistantActionConfirmResultCandidate): boolean {
  if (!parentAssistantActionConfirmBaseIsSafe(result)) {
    return false;
  }

  return result.confirmState === 'rejected'
    ? parentAssistantRejectedActionConfirmIsSafe(result)
    : parentAssistantContractRequiredActionConfirmIsSafe(result);
}

function parentAssistantActionConfirmBaseIsSafe(result: ParentAssistantActionConfirmResultCandidate): boolean {
  return (
    result.previewRequired === true &&
    result.rawAssistantProseAccepted === false &&
    result.parentConfirmationRequired === true &&
    result.parentConfirmationRecorded === false &&
    result.childAgentContractRequired === true &&
    result.enforcementApplied === false &&
    result.policyWritten === false &&
    result.sourceRefs.length > 0
  );
}

function parentAssistantRejectedActionConfirmIsSafe(result: ParentAssistantActionConfirmResultCandidate): boolean {
  return result.previewSatisfied === false && result.previewId === null;
}

function parentAssistantContractRequiredActionConfirmIsSafe(
  result: ParentAssistantActionConfirmResultCandidate
): boolean {
  return (
    result.requiresControllerLease === true &&
    result.confirmState === 'contract-required' &&
    result.previewSatisfied === true &&
    result.previewId !== null &&
    result.childAgentValidationState === 'child-agent-contract-required'
  );
}

function parentAssistantActionPreviewResultIsSafe(result: ParentAssistantActionPreviewResultCandidate): boolean {
  if (!parentAssistantActionPreviewBaseIsSafe(result)) {
    return false;
  }

  return parentAssistantActionPreviewEvidenceIsSafe(result);
}

function parentAssistantActionPreviewBaseIsSafe(result: ParentAssistantActionPreviewResultCandidate): boolean {
  return (
    result.previewRequired === true &&
    result.previewSatisfied === true &&
    result.rawAssistantProseAccepted === false &&
    result.parentConfirmationRequired === true &&
    result.parentConfirmationRecorded === false &&
    result.childAgentContractRequired === true &&
    result.enforcementApplied === false &&
    result.policyWritten === false &&
    result.preview.enforcementApplied === false
  );
}

function parentAssistantActionPreviewEvidenceIsSafe(result: ParentAssistantActionPreviewResultCandidate): boolean {
  return (
    (result.previewState !== 'draft' || result.preview.previewId !== null) &&
    result.evidenceContext.length > 0 &&
    result.sourceRefs.length > 0
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

