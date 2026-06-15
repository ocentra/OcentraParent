import { type Infer } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiRuntimeProviderProofReadModel,
  type LocalAiRuntimeProviderProofEntry,
  type LocalAiRuntimeProviderProofRequirement,
} from './local-ai-runtime-provider-proof';
import {
  ParentAssistantActionConfirmResultSchema,
  ParentAssistantActionPreviewResultSchema,
  ParentAssistantAnswerSchema,
  ParentAssistantProviderStatusSchema,
  type ParentAssistantActionConfirmResult,
  type ParentAssistantAnswer,
  type ParentAssistantProviderStatus,
} from './parent-assistant';
import {
  LocalAiParentAssistantRuntimeProofEntrySchema,
  LocalAiParentAssistantRuntimeProofReadModelSchema,
  type LocalAiParentAssistantRuntimeProofEntry,
  type LocalAiParentAssistantRuntimeProofRequirement,
  type LocalAiParentAssistantRuntimeProofStatus,
} from './local-ai-parent-assistant-runtime-proof';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

type ParentAssistantActionPreviewResult = Infer<typeof ParentAssistantActionPreviewResultSchema>;

const checkedAt = '2026-06-03T10:10:00.000Z';
const runtimeProofCommand = 'node scripts/test/activity-parent-assistant-runtime-proof.mjs';

const SourceProviderProofRequirement = {
  ParentAssistantSubmits: 'parent-assistant-submits-when-allowed',
  Lifecycle: 'queued-degraded-unavailable-lifecycle',
  ProviderStatus: 'provider-status-contract-hardening',
  ChildSafetyPriority: 'child-safety-priority',
} as const satisfies Record<string, LocalAiRuntimeProviderProofRequirement>;

const localAnswerSource = providerProofFor(SourceProviderProofRequirement.ParentAssistantSubmits);
const degradedSource = providerProofFor(SourceProviderProofRequirement.Lifecycle);
const unavailableSource = providerProofFor(SourceProviderProofRequirement.ProviderStatus);
const prioritySource = providerProofFor(SourceProviderProofRequirement.ChildSafetyPriority);

const ProviderStatusByState = {
  completed: {
    providerState: 'configured',
    runState: 'completed',
    schedulerJobStatus: 'complete',
    degradedState: 'none',
    unavailableReason: null,
    apiAccessState: 'not-authorized',
    apiProviderState: 'unavailable',
    apiUnavailableReason: 'local-ai-api-provider-not-authorized',
    routingState: 'local-provider-ready',
    selectedProvider: 'local',
    localProviderState: 'configured',
  },
  degraded: {
    providerState: 'degraded',
    runState: 'degraded',
    schedulerJobStatus: 'degraded',
    degradedState: 'overloaded',
    unavailableReason: null,
    apiAccessState: 'not-authorized',
    apiProviderState: 'unavailable',
    apiUnavailableReason: 'local-ai-api-provider-not-authorized',
    routingState: 'local-provider-degraded',
    selectedProvider: 'local',
    localProviderState: 'degraded',
  },
  queued: {
    providerState: 'degraded',
    runState: 'queued',
    schedulerJobStatus: 'queued',
    degradedState: 'overloaded',
    unavailableReason: null,
    apiAccessState: 'not-authorized',
    apiProviderState: 'unavailable',
    apiUnavailableReason: 'local-ai-api-provider-not-authorized',
    routingState: 'local-provider-degraded',
    selectedProvider: 'local',
    localProviderState: 'degraded',
  },
  unavailable: {
    providerState: 'unavailable',
    runState: 'unavailable',
    schedulerJobStatus: 'unavailable',
    degradedState: 'none',
    unavailableReason: 'local-ai-provider-unconfigured',
    apiAccessState: 'not-authorized',
    apiProviderState: 'unavailable',
    apiUnavailableReason: 'local-ai-api-provider-not-authorized',
    routingState: 'no-provider-available',
    selectedProvider: 'none',
    localProviderState: 'unavailable',
  },
  'api-degraded': {
    providerState: 'unavailable',
    runState: 'unavailable',
    schedulerJobStatus: 'unavailable',
    degradedState: 'none',
    unavailableReason: 'local-ai-provider-unconfigured',
    apiAccessState: 'authorized-degraded',
    apiProviderState: 'degraded',
    apiUnavailableReason: 'parent-authorized-api-provider-degraded',
    routingState: 'api-provider-authorized-degraded',
    selectedProvider: 'none',
    localProviderState: 'unavailable',
  },
} as const;

const BusyProviderLifecycleStates = new Set(['running', 'queued', 'degraded']);

export const LocalAiParentAssistantRuntimeProofReadModel = LocalAiParentAssistantRuntimeProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'local-ai-parent-assistant-runtime-proof',
  generatedAt: checkedAt,
  sourceReadModelIds: [
    'local-ai-runtime-provider-proof',
    'parent-assistant-answer',
    'parent-assistant-provider-status',
    'parent-assistant-action-preview-confirm',
    'activity-parent-assistant-runtime-proof',
  ],
  entries: [
    runtimeEntry({
      proofEntryId: 'local-ai-parent-assistant-local-answer',
      requirement: 'local-provider-answer-uses-shared-runtime',
      proofStatus: 'proved',
      sourceProviderProof: localAnswerSource,
      parentAssistantAnswer: answerFor('local-answer', localAnswerSource, 'answered'),
      providerStatus: statusFor(localAnswerSource, 'completed'),
      localProviderSelected: true,
      apiProviderSelected: false,
      evidenceLabel: 'Local provider answer uses the same parent-assistant scheduler entry as the provider proof.',
      claimBoundary: 'This proves local runtime routing and citations, not model quality or child-safety authority.',
      fallbackBehavior: 'Return unavailable or degraded when the shared local provider cannot accept assistant work.',
    }),
    runtimeEntry({
      proofEntryId: 'local-ai-parent-assistant-degraded-runtime',
      requirement: 'busy-provider-degrades-without-extra-runtime',
      proofStatus: 'degraded',
      sourceProviderProof: degradedSource,
      parentAssistantAnswer: answerFor('degraded-answer', degradedSource, 'degraded'),
      providerStatus: statusFor(degradedSource, 'degraded'),
      localProviderSelected: true,
      apiProviderSelected: false,
      evidenceLabel: 'Busy provider state reports degraded without admitting another local model runtime.',
      claimBoundary: 'Degraded local assistant state is not policy enforcement or safety approval.',
      fallbackBehavior: 'Keep the assistant degraded until the shared provider lane can accept work.',
    }),
    runtimeEntry({
      proofEntryId: 'local-ai-parent-assistant-unavailable-status',
      requirement: 'provider-unavailable-is-explicit-and-cited',
      proofStatus: 'unavailable',
      sourceProviderProof: unavailableSource,
      parentAssistantAnswer: answerFor('unavailable-answer', unavailableSource, 'unavailable'),
      providerStatus: statusFor(unavailableSource, 'unavailable'),
      localProviderSelected: false,
      apiProviderSelected: false,
      evidenceLabel: 'Unavailable local provider answer and status carry explicit reasons plus citations.',
      claimBoundary: 'Unavailable status must not be promoted to a working local or remote provider.',
      fallbackBehavior: 'Return unavailable with cited evidence context and no answer text.',
    }),
    runtimeEntry({
      proofEntryId: 'local-ai-parent-assistant-child-safety-priority',
      requirement: 'child-safety-priority-keeps-assistant-queued',
      proofStatus: 'proved',
      sourceProviderProof: prioritySource,
      parentAssistantAnswer: answerFor('queued-answer', prioritySource, 'queued'),
      providerStatus: statusFor(prioritySource, 'queued'),
      localProviderSelected: true,
      apiProviderSelected: false,
      evidenceLabel: 'Child-safety queued work remains ahead of queued parent-assistant work on the shared lane.',
      claimBoundary: 'This proves queue priority only, not safety-classifier accuracy.',
      fallbackBehavior: 'Queue assistant work behind child-safety work and expose queued state.',
    }),
    runtimeEntry({
      proofEntryId: 'local-ai-parent-assistant-api-optional-boundary',
      requirement: 'api-provider-remains-optional-parent-authorized-boundary',
      proofStatus: 'not-claimed',
      sourceProviderProof: unavailableSource,
      providerStatus: statusFor(unavailableSource, 'api-degraded'),
      localProviderSelected: false,
      apiProviderSelected: false,
      evidenceLabel: 'API AI remains optional, parent-authorized, citation-bound, and unavailable/degraded only.',
      claimBoundary: 'No real API adapter, default remote retention, child-safety use, or enforcement use is claimed.',
      fallbackBehavior:
        'Use local unavailable/degraded state unless a parent-authorized API adapter is separately proved.',
    }),
    runtimeEntry({
      proofEntryId: 'local-ai-parent-assistant-action-contract-boundary',
      requirement: 'action-preview-confirm-requires-child-contract',
      proofStatus: 'proved',
      sourceProviderProof: localAnswerSource,
      actionPreviewResult: actionPreviewResult(),
      actionConfirmResult: actionConfirmResult(),
      localProviderSelected: false,
      apiProviderSelected: false,
      evidenceLabel:
        'Assistant action preview and confirm require child-agent contract and never write policy directly.',
      claimBoundary: 'This is a preview/confirm boundary, not portal UI, policy execution, or child-device validation.',
      fallbackBehavior:
        'Return contract-required action state until parent confirmation and child-agent validation exist.',
    }),
  ],
});

function providerProofFor(requirement: LocalAiRuntimeProviderProofRequirement): LocalAiRuntimeProviderProofEntry {
  const entry = LocalAiRuntimeProviderProofReadModel.entries.find((candidate) => candidate.requirement === requirement);
  if (entry === undefined) {
    throw new Error(`Missing local AI runtime provider proof entry: ${requirement}`);
  }
  return entry;
}

function runtimeEntry(input: {
  proofEntryId: string;
  requirement: LocalAiParentAssistantRuntimeProofRequirement;
  proofStatus: LocalAiParentAssistantRuntimeProofStatus;
  sourceProviderProof: LocalAiRuntimeProviderProofEntry;
  parentAssistantAnswer?: ParentAssistantAnswer;
  providerStatus?: ParentAssistantProviderStatus;
  actionPreviewResult?: ParentAssistantActionPreviewResult;
  actionConfirmResult?: ParentAssistantActionConfirmResult;
  localProviderSelected: boolean;
  apiProviderSelected: boolean;
  evidenceLabel: string;
  claimBoundary: string;
  fallbackBehavior: string;
}): LocalAiParentAssistantRuntimeProofEntry {
  const {
    sourceProviderProof,
    parentAssistantAnswer,
    providerStatus,
    actionPreviewResult,
    actionConfirmResult,
    ...schemaInput
  } = input;
  return LocalAiParentAssistantRuntimeProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    sourceProviderProofRequirement: sourceProviderProof.requirement,
    sourceProviderProofEntryId: sourceProviderProof.proofEntryId,
    schedulerLifecycle: sourceProviderProof.schedulerLifecycle,
    parentAssistantAnswer: parentAssistantAnswer ?? null,
    providerStatus: providerStatus ?? null,
    actionPreviewResult: actionPreviewResult ?? null,
    actionConfirmResult: actionConfirmResult ?? null,
    remoteAiOptional: true,
    evidenceCitationRequired: true,
    childSafetyOrEnforcementUseAllowed: false,
    runtimeProofCommand,
    lastCheckedAt: checkedAt,
    ...schemaInput,
  });
}

function answerFor(
  suffix: string,
  source: LocalAiRuntimeProviderProofEntry,
  state: 'answered' | 'degraded' | 'queued' | 'unavailable'
): ParentAssistantAnswer {
  const providerState = state === 'answered' ? 'configured' : state === 'unavailable' ? 'unavailable' : 'degraded';
  const unavailableReason = state === 'unavailable' ? source.unavailableReason : null;
  return ParentAssistantAnswerSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    requestId: `parent-assistant-runtime-${suffix}-request`,
    threadId: 'parent-assistant-runtime-proof-thread',
    messageId: `parent-assistant-runtime-${suffix}-message`,
    answeredAt: checkedAt,
    providerId: source.providerId,
    modelId: source.modelId,
    providerState,
    answerState: state,
    runState: state === 'answered' ? 'completed' : state,
    schedulerJobStatus: state === 'answered' ? 'complete' : state,
    degradedState: state === 'answered' || state === 'unavailable' ? 'none' : 'overloaded',
    unavailableReason,
    localAiResultId: state === 'answered' ? 'local-ai-result-parent-assistant-runtime-proof' : null,
    answerText: state === 'answered' ? 'Local assistant answer cited parent-owned activity summary.' : null,
    citations: [activityCitation()],
    actionPreview: preview(),
    apiProviderBoundary: apiBoundary('not-authorized', 'unavailable', 'local-ai-api-provider-not-authorized'),
    providerRoute:
      state === 'answered'
        ? providerRoute('local-provider-ready', 'local', 'configured', 'unavailable', 'not-authorized')
        : state === 'unavailable'
          ? providerRoute('no-provider-available', 'none', 'unavailable', 'unavailable', 'not-authorized')
          : providerRoute('local-provider-degraded', 'local', 'degraded', 'unavailable', 'not-authorized'),
    promptVersion: 'parent-assistant-local-runtime-proof-v1',
  });
}

function statusFor(
  source: LocalAiRuntimeProviderProofEntry,
  state: 'completed' | 'degraded' | 'queued' | 'unavailable' | 'api-degraded'
): ParentAssistantProviderStatus {
  const status = ProviderStatusByState[state];
  return ParentAssistantProviderStatusSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    backendState: 'runtime-backed',
    providerId: source.providerId,
    modelId: source.modelId,
    providerState: status.providerState,
    runState: status.runState,
    schedulerJobStatus: status.schedulerJobStatus,
    schedulerStatus: source.sourceSchedulerStatus,
    degradedState: status.degradedState,
    unavailableReason: status.unavailableReason,
    queueDepth: source.queue.childSafetyQueued + source.queue.parentAssistantQueued + source.queue.parentReportQueued,
    busy: BusyProviderLifecycleStates.has(source.schedulerLifecycle),
    apiProviderBoundary: apiBoundary(status.apiAccessState, status.apiProviderState, status.apiUnavailableReason),
    providerRoute: providerRoute(
      status.routingState,
      status.selectedProvider,
      status.localProviderState,
      status.apiProviderState,
      status.apiAccessState
    ),
  });
}

function actionPreviewResult(): ParentAssistantActionPreviewResult {
  return ParentAssistantActionPreviewResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    backendState: 'contract-required',
    actionIntentId: 'parent-assistant-action-intent-runtime-proof',
    previewState: 'draft',
    preview: preview(),
    evidenceContext: [activityCitation()],
    previewRequired: true,
    previewSatisfied: true,
    rawAssistantProseAccepted: false,
    parentConfirmationRequired: true,
    parentConfirmationRecorded: false,
    childAgentValidationState: 'child-agent-contract-required',
    sourceRefs: [activityCitation().evidence],
    auditReason: 'Preview generated from cited parent-owned Activity evidence.',
    requiresControllerLease: true,
    childAgentContractRequired: true,
    enforcementApplied: false,
    policyWritten: false,
    reason: 'Assistant action remains a draft until parent confirmation and child-agent validation.',
  });
}

function actionConfirmResult(): ParentAssistantActionConfirmResult {
  return ParentAssistantActionConfirmResultSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    backendState: 'contract-required',
    actionIntentId: 'parent-assistant-action-intent-runtime-proof',
    previewId: 'parent-assistant-action-preview-runtime-proof',
    actionKind: 'policy-suggestion',
    confirmState: 'contract-required',
    previewRequired: true,
    previewSatisfied: true,
    rawAssistantProseAccepted: false,
    parentConfirmationRequired: true,
    parentConfirmationRecorded: false,
    childAgentValidationState: 'child-agent-contract-required',
    sourceRefs: [activityCitation().evidence],
    auditReason: 'Parent confirmation cannot write policy until child-agent validation is wired.',
    requiresControllerLease: true,
    childAgentContractRequired: true,
    enforcementApplied: false,
    policyWritten: false,
    reason: 'Child-agent contract is required before any policy write or enforcement handoff.',
  });
}

function preview() {
  return {
    previewId: 'parent-assistant-action-preview-runtime-proof',
    actionKind: 'policy-suggestion',
    summary: 'Draft policy suggestion preview only.',
    actionReference: null,
    requiresControllerLease: true,
    childAgentContractRequired: true,
    enforcementApplied: false,
  } as const;
}

function activityCitation() {
  return {
    evidence: {
      evidenceReferenceId: 'parent-assistant-runtime-proof-activity-summary',
      kind: 'query-store-summary',
      observedAt: checkedAt,
    },
    citationLabel: 'Activity report',
    allowedSummary: 'Parent-owned activity summary citation; raw child evidence excluded.',
    custodyLabel: 'parent-owned-activity-report',
    sourceLabel: 'saved-activity-report-history',
    rawChildEvidenceIncluded: false,
    directEnforcementAllowed: false,
  } as const;
}

function apiBoundary(
  accessState: 'not-authorized' | 'authorized-degraded',
  providerState: 'unavailable' | 'degraded',
  unavailableReason: string
) {
  const authorized = accessState !== 'not-authorized';
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerId: authorized ? 'parent-authorized-api-ai-provider' : 'parent-assistant-api-provider-unavailable',
    authorizationState: authorized ? 'authorized' : 'not-authorized',
    accessState,
    parentAuthorizationRequired: true,
    evidenceCitationRequired: true,
    custodyLabel: authorized ? 'parent-authorized-api-ai' : 'parent-api-ai-not-authorized',
    custodyState: 'parent-owned-citations-only',
    retentionPolicy: authorized
      ? 'parent-authorized-no-default-retention'
      : 'no-retention-without-parent-authorization',
    retentionState: authorized ? 'parent-authorized-no-default-retention' : 'no-retention-without-parent-authorization',
    deletionPolicy: 'delete-provider-cache-on-parent-request',
    deletionState: 'delete-provider-cache-on-parent-request',
    citations: [activityCitation()],
    providerState,
    unavailableReason,
    childSafetyOrEnforcementUseAllowed: false,
  } as const;
}

function providerRoute(
  routingState:
    | 'local-provider-ready'
    | 'local-provider-degraded'
    | 'no-provider-available'
    | 'api-provider-authorized-degraded',
  selectedProvider: 'local' | 'none',
  localProviderState: 'configured' | 'degraded' | 'unavailable',
  apiProviderState: 'unavailable' | 'degraded',
  apiAccessState: 'not-authorized' | 'authorized-degraded'
) {
  return {
    routingState,
    selectedProvider,
    localProviderState,
    apiProviderState,
    apiAccessState,
    evidenceCitationRequired: true,
    remoteAiOptional: true,
    childSafetyOrEnforcementUseAllowed: false,
    reason: 'Provider route preserves local-first assistant runtime and optional API boundary.',
  } as const;
}
