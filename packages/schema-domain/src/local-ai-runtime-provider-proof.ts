import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { DeviceRuntimeRoleSchema } from './device-roles';
import {
  type LocalAiResourceClass,
  LocalAiDegradedState,
  LocalAiDegradedStateSchema,
  LocalAiModelIdSchema,
  LocalAiModelReferenceSchema,
  LocalAiProviderIdSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
} from './ai-primitives';
import {
  LocalAiPhysicalDeviceIdSchema,
  LocalAiProviderSchedulerJobClassSchema,
  LocalAiProviderSchedulerLifecycleSchema,
  LocalAiProviderSchedulerQueueSchema,
  LocalAiProviderSchedulerStatusSchema,
  LocalAiProviderSingletonScopeSchema,
} from './local-ai-provider-scheduler';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
const RuntimeAccessLaneCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const RuntimeLoadCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiRuntimeProviderProofReadModelIdSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeProviderProofReadModelId'
);
export const LocalAiRuntimeProviderProofEntryIdSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeProviderProofEntryId'
);
export const LocalAiRuntimeProviderProofRequirementTextSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeProviderProofRequirementText'
);
export const LocalAiRuntimeProviderProofClaimBoundarySchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeProviderProofClaimBoundary'
);
export const LocalAiRuntimeProviderProofFallbackSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeProviderProofFallback'
);
export const LocalAiRuntimeProviderProofEvidenceLabelSchema = brandedNonEmptyStringSchema(
  'LocalAiRuntimeProviderProofEvidenceLabel'
);

export const LocalAiRuntimeProviderProofRequirementSchema = withParser(
  Schema.Literal(
    'one-ai-provider-role-per-physical-device',
    'shared-parent-child-provider',
    'single-local-runtime-lane',
    'child-safety-priority',
    'queued-degraded-unavailable-lifecycle',
    'parent-assistant-submits-when-allowed',
    'no-duplicate-local-model-load',
    'provider-status-contract-hardening'
  )
);

export const LocalAiRuntimeProviderProofStatusSchema = withParser(
  Schema.Literal('proved', 'degraded', 'unavailable', 'not-claimed')
);

const LocalAiRuntimeProviderProofEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofEntryId: LocalAiRuntimeProviderProofEntryIdSchema,
  requirement: LocalAiRuntimeProviderProofRequirementSchema,
  proofStatus: LocalAiRuntimeProviderProofStatusSchema,
  physicalDeviceId: LocalAiPhysicalDeviceIdSchema,
  singletonScope: LocalAiProviderSingletonScopeSchema,
  providerId: LocalAiProviderIdSchema,
  runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
  modelId: LocalAiModelIdSchema,
  modelReference: LocalAiModelReferenceSchema,
  participatingRoles: Schema.Array(DeviceRuntimeRoleSchema),
  acceptedJobClasses: Schema.Array(LocalAiProviderSchedulerJobClassSchema),
  schedulerLifecycle: LocalAiProviderSchedulerLifecycleSchema,
  sourceSchedulerStatus: LocalAiProviderSchedulerStatusSchema,
  runtimeAccessLaneCount: RuntimeAccessLaneCountSchema,
  runtimeLoadCount: RuntimeLoadCountSchema,
  duplicateRuntimeBlocked: Schema.Boolean,
  childSafetyPriorityProved: Schema.Boolean,
  parentAssistantSubmissionAllowed: Schema.Boolean,
  queue: LocalAiProviderSchedulerQueueSchema,
  degradedState: LocalAiDegradedStateSchema,
  unavailableReason: Schema.Union(NonEmptyStringSchema, Schema.Null),
  evidenceLabel: LocalAiRuntimeProviderProofEvidenceLabelSchema,
  capabilityRequirement: LocalAiRuntimeProviderProofRequirementTextSchema,
  proofRequirement: LocalAiRuntimeProviderProofRequirementTextSchema,
  claimBoundary: LocalAiRuntimeProviderProofClaimBoundarySchema,
  fallbackBehavior: LocalAiRuntimeProviderProofFallbackSchema,
  lastCheckedAt: LocalAiTimestampSchema,
});

type LocalAiRuntimeProviderProofEntryCandidate = Infer<typeof LocalAiRuntimeProviderProofEntryBaseSchema>;

export const LocalAiRuntimeProviderProofEntrySchema = withParser(
  LocalAiRuntimeProviderProofEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        localAiRuntimeProviderProofEntryIsConsistent(entry) ||
        'Expected local AI runtime provider proof entries to preserve singleton, priority, lifecycle, and unavailable boundaries'
    )
  )
);

const LocalAiRuntimeProviderProofReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: LocalAiRuntimeProviderProofReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceReadModelIds: Schema.Array(NonEmptyStringSchema),
  entries: Schema.Array(LocalAiRuntimeProviderProofEntrySchema),
});

type LocalAiRuntimeProviderProofReadModelCandidate = Infer<typeof LocalAiRuntimeProviderProofReadModelBaseSchema>;

export const LocalAiRuntimeProviderProofReadModelSchema = withParser(
  LocalAiRuntimeProviderProofReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        localAiRuntimeProviderProofReadModelIsComplete(readModel) ||
        'Expected local AI runtime provider proof read model to contain unique entries for every scheduler/provider proof requirement'
    )
  )
);

function localAiRuntimeProviderProofEntryIsConsistent(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  if (!entryHasUniqueRoles(entry) || entry.runtimeAccessLaneCount !== 1 || entry.runtimeLoadCount > 1) {
    return false;
  }

  if (!entryMatchesSourceScheduler(entry)) {
    return false;
  }

  return requirementValidatorFor(entry.requirement)(entry);
}

function entryHasUniqueRoles(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return new Set(entry.participatingRoles).size === entry.participatingRoles.length;
}

function entryMatchesSourceScheduler(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return (
    entry.physicalDeviceId === entry.sourceSchedulerStatus.physicalDeviceId &&
    entry.singletonScope === entry.sourceSchedulerStatus.singletonScope &&
    entry.providerId === entry.sourceSchedulerStatus.providerId &&
    entry.runtimeReferenceId === entry.sourceSchedulerStatus.runtimeReferenceId &&
    entry.modelId === entry.sourceSchedulerStatus.modelId &&
    entry.modelReference === entry.sourceSchedulerStatus.modelReference &&
    entry.schedulerLifecycle === entry.sourceSchedulerStatus.lifecycleState &&
    entry.duplicateRuntimeBlocked === entry.sourceSchedulerStatus.duplicateRuntimeBlocked
  );
}

function entryHasSharedDeviceProvider(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return entry.participatingRoles.includes('ai-provider') && entry.singletonScope === 'physical-device';
}

function entryHasSharedParentChildProvider(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return (
    entryHasSharedDeviceProvider(entry) &&
    entry.participatingRoles.includes('parent-controller') &&
    entry.participatingRoles.includes('child-agent')
  );
}

function childSafetyPriorityEntryIsConsistent(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return (
    entry.childSafetyPriorityProved &&
    entry.queue.childSafetyQueued > 0 &&
    entry.queue.parentAssistantQueued > 0 &&
    entry.schedulerLifecycle === 'queued' &&
    entry.duplicateRuntimeBlocked
  );
}

function lifecycleEntryIsConsistent(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return (
    entry.proofStatus === 'degraded' &&
    entry.schedulerLifecycle === 'degraded' &&
    entry.degradedState !== LocalAiDegradedState.None &&
    entry.unavailableReason === null
  );
}

function parentAssistantSubmissionEntryIsConsistent(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return (
    entry.parentAssistantSubmissionAllowed &&
    entry.acceptedJobClasses.includes('parent-assistant') &&
    entry.schedulerLifecycle === 'running' &&
    entry.runtimeLoadCount === 1
  );
}

function requirementValidatorFor(requirement: string) {
  return localAiRuntimeProviderProofRequirementValidators[requirement] ?? (() => false);
}

const localAiRuntimeProviderProofRequirementValidators: Record<
  string,
  (entry: LocalAiRuntimeProviderProofEntryCandidate) => boolean
> = {
  'one-ai-provider-role-per-physical-device': (entry) =>
    entryHasSharedDeviceProvider(entry) && entry.proofStatus === 'proved',
  'shared-parent-child-provider': (entry) => entryHasSharedParentChildProvider(entry) && entry.proofStatus === 'proved',
  'single-local-runtime-lane': runtimeLaneEntryIsConsistent,
  'child-safety-priority': childSafetyPriorityEntryIsConsistent,
  'queued-degraded-unavailable-lifecycle': lifecycleEntryIsConsistent,
  'parent-assistant-submits-when-allowed': parentAssistantSubmissionEntryIsConsistent,
  'no-duplicate-local-model-load': runtimeLaneEntryIsConsistent,
  'provider-status-contract-hardening': (entry) =>
    entry.sourceSchedulerStatus.unavailableReason === entry.unavailableReason,
};

function runtimeLaneEntryIsConsistent(entry: LocalAiRuntimeProviderProofEntryCandidate): boolean {
  return entry.duplicateRuntimeBlocked && entry.runtimeLoadCount === 1;
}

function localAiRuntimeProviderProofReadModelIsComplete(
  readModel: LocalAiRuntimeProviderProofReadModelCandidate
): boolean {
  if (new Set(readModel.entries.map((entry) => entry.proofEntryId)).size !== readModel.entries.length) {
    return false;
  }

  const requirements = new Set(readModel.entries.map((entry) => entry.requirement));
  for (const requirement of LocalAiRuntimeProviderProofRequirementValues) {
    if (!requirements.has(requirement)) {
      return false;
    }
  }

  return readModel.entries.every((entry) => entry.runtimeLoadCount <= 1);
}

export type LocalAiRuntimeProviderProofReadModelId = typeof LocalAiRuntimeProviderProofReadModelIdSchema.Type;
export type LocalAiRuntimeProviderProofEntryId = typeof LocalAiRuntimeProviderProofEntryIdSchema.Type;
export type LocalAiRuntimeProviderProofRequirementText = typeof LocalAiRuntimeProviderProofRequirementTextSchema.Type;
export type LocalAiRuntimeProviderProofClaimBoundary = typeof LocalAiRuntimeProviderProofClaimBoundarySchema.Type;
export type LocalAiRuntimeProviderProofFallback = typeof LocalAiRuntimeProviderProofFallbackSchema.Type;
export type LocalAiRuntimeProviderProofEvidenceLabel = typeof LocalAiRuntimeProviderProofEvidenceLabelSchema.Type;
export type LocalAiRuntimeProviderProofRequirement = Infer<typeof LocalAiRuntimeProviderProofRequirementSchema>;
export type LocalAiRuntimeProviderProofStatus = Infer<typeof LocalAiRuntimeProviderProofStatusSchema>;
export type LocalAiRuntimeProviderProofEntry = Infer<typeof LocalAiRuntimeProviderProofEntrySchema>;
export type LocalAiRuntimeProviderProofReadModel = Infer<typeof LocalAiRuntimeProviderProofReadModelSchema>;

type SchedulerStatusInput = {
  physicalDeviceId?: string;
  providerId?: string;
  runtimeReferenceId?: string;
  modelId?: string;
  modelReference?: string;
  resourceClass?: LocalAiResourceClass;
  lifecycleState: Infer<typeof LocalAiProviderSchedulerLifecycleSchema>;
  currentJobClass: Infer<typeof LocalAiProviderSchedulerJobClassSchema> | null;
  queue: Infer<typeof LocalAiProviderSchedulerQueueSchema>;
  duplicateRuntimeBlocked: boolean;
  degradedState: Infer<typeof LocalAiDegradedStateSchema>;
  unavailableReason: string | null;
};

type ProviderProofEntryInput = {
  proofEntryId: string;
  requirement: LocalAiRuntimeProviderProofRequirement;
  proofStatus: LocalAiRuntimeProviderProofStatus;
  sourceSchedulerStatus: Infer<typeof LocalAiProviderSchedulerStatusSchema>;
  participatingRoles: ReadonlyArray<typeof DeviceRuntimeRoleSchema.Type>;
  acceptedJobClasses: ReadonlyArray<typeof LocalAiProviderSchedulerJobClassSchema.Type>;
  runtimeLoadCount: number;
  childSafetyPriorityProved: boolean;
  parentAssistantSubmissionAllowed: boolean;
  evidenceLabel: string;
  capabilityRequirement: string;
  proofRequirement: string;
  claimBoundary: string;
  fallbackBehavior: string;
};

export const LocalAiRuntimeProviderProofRequirementValues = [
  'one-ai-provider-role-per-physical-device',
  'shared-parent-child-provider',
  'single-local-runtime-lane',
  'child-safety-priority',
  'queued-degraded-unavailable-lifecycle',
  'parent-assistant-submits-when-allowed',
  'no-duplicate-local-model-load',
  'provider-status-contract-hardening',
] as const satisfies ReadonlyArray<LocalAiRuntimeProviderProofRequirement>;

const documentedAt = '2026-05-30T15:50:00.000Z';

const readyStatus = schedulerStatus({
  lifecycleState: 'idle',
  currentJobClass: null,
  queue: { childSafetyQueued: 0, parentAssistantQueued: 0, parentReportQueued: 0 },
  duplicateRuntimeBlocked: true,
  degradedState: 'none',
  unavailableReason: null,
});

const runningParentAssistantStatus = schedulerStatus({
  lifecycleState: 'running',
  currentJobClass: 'parent-assistant',
  queue: { childSafetyQueued: 0, parentAssistantQueued: 0, parentReportQueued: 0 },
  duplicateRuntimeBlocked: true,
  degradedState: 'none',
  unavailableReason: null,
});

const queuedPriorityStatus = schedulerStatus({
  lifecycleState: 'queued',
  currentJobClass: 'parent-report',
  queue: { childSafetyQueued: 1, parentAssistantQueued: 1, parentReportQueued: 0 },
  duplicateRuntimeBlocked: true,
  degradedState: 'overloaded',
  unavailableReason: null,
});

const degradedStatus = schedulerStatus({
  lifecycleState: 'degraded',
  currentJobClass: null,
  queue: { childSafetyQueued: 0, parentAssistantQueued: 1, parentReportQueued: 0 },
  duplicateRuntimeBlocked: true,
  degradedState: 'overloaded',
  unavailableReason: null,
});

const unavailableStatus = schedulerStatus({
  providerId: 'local-provider-unconfigured',
  runtimeReferenceId: 'local-ai-runtime-dev-unconfigured',
  modelId: 'safety-model-unconfigured',
  modelReference: 'local-model-cache-unconfigured',
  resourceClass: 'remote-unavailable',
  lifecycleState: 'unavailable',
  currentJobClass: null,
  queue: { childSafetyQueued: 0, parentAssistantQueued: 0, parentReportQueued: 0 },
  duplicateRuntimeBlocked: false,
  degradedState: 'provider-unavailable',
  unavailableReason: 'local-ai-provider-unconfigured',
});

export const LocalAiRuntimeProviderProofReadModel = LocalAiRuntimeProviderProofReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'local-ai-runtime-provider-proof',
  generatedAt: documentedAt,
  sourceReadModelIds: ['local-ai-provider-scheduler', 'device-role-runtime-read-model', 'parent-assistant-runtime'],
  entries: [
    providerProofEntry({
      proofEntryId: 'local-ai-proof-single-provider-role',
      requirement: 'one-ai-provider-role-per-physical-device',
      proofStatus: 'proved',
      sourceSchedulerStatus: readyStatus,
      participatingRoles: ['ai-provider'],
      acceptedJobClasses: ['child-safety', 'parent-assistant', 'parent-report'],
      runtimeLoadCount: 1,
      childSafetyPriorityProved: false,
      parentAssistantSubmissionAllowed: false,
      evidenceLabel: 'Device role read model declares one ai-provider role for the physical device.',
      capabilityRequirement: 'Physical-device id and singleton scope must identify the provider lane.',
      proofRequirement: 'Typed contract rejects duplicate roles and duplicate runtime loads.',
      claimBoundary: 'This proves a local physical-device provider role, not LAN provider pooling.',
      fallbackBehavior: 'Return unavailable when no local provider runtime can be configured.',
    }),
    providerProofEntry({
      proofEntryId: 'local-ai-proof-shared-parent-child-provider',
      requirement: 'shared-parent-child-provider',
      proofStatus: 'proved',
      sourceSchedulerStatus: readyStatus,
      participatingRoles: ['parent-controller', 'child-agent', 'ai-provider'],
      acceptedJobClasses: ['child-safety', 'parent-assistant', 'parent-report'],
      runtimeLoadCount: 1,
      childSafetyPriorityProved: false,
      parentAssistantSubmissionAllowed: false,
      evidenceLabel: 'Parent and child roles point at the same provider id and runtime reference.',
      capabilityRequirement: 'Parent and child roles must share one local provider on the physical device.',
      proofRequirement: 'Scheduler state preserves identical physicalDeviceId, providerId, and runtimeReferenceId.',
      claimBoundary: 'This is same-device local sharing, not cross-device LAN AI routing.',
      fallbackBehavior: 'Degrade or queue jobs instead of starting a second model runtime.',
    }),
    providerProofEntry({
      proofEntryId: 'local-ai-proof-single-runtime-lane',
      requirement: 'single-local-runtime-lane',
      proofStatus: 'proved',
      sourceSchedulerStatus: queuedPriorityStatus,
      participatingRoles: ['parent-controller', 'child-agent', 'ai-provider'],
      acceptedJobClasses: ['child-safety', 'parent-assistant', 'parent-report'],
      runtimeLoadCount: 1,
      childSafetyPriorityProved: false,
      parentAssistantSubmissionAllowed: false,
      evidenceLabel: 'Queued scheduler state blocks duplicate runtime admission while one lane is active.',
      capabilityRequirement: 'One local model runtime lane per physical device.',
      proofRequirement: 'Service scheduler queue tests assert max active local generation jobs stays one.',
      claimBoundary: 'This does not claim model quality, classifier quality, or remote provider access.',
      fallbackBehavior: 'Queue lower-priority jobs or return degraded when the lane is busy.',
    }),
    providerProofEntry({
      proofEntryId: 'local-ai-proof-child-safety-priority',
      requirement: 'child-safety-priority',
      proofStatus: 'proved',
      sourceSchedulerStatus: queuedPriorityStatus,
      participatingRoles: ['child-agent', 'ai-provider'],
      acceptedJobClasses: ['child-safety', 'parent-assistant', 'parent-report'],
      runtimeLoadCount: 1,
      childSafetyPriorityProved: true,
      parentAssistantSubmissionAllowed: false,
      evidenceLabel: 'Child-safety queued work is ordered ahead of parent-assistant work.',
      capabilityRequirement: 'Child-safety jobs must outrank parent assistant jobs on the shared lane.',
      proofRequirement: 'Rust service scheduler test observes parent-report, child-safety, parent-assistant order.',
      claimBoundary: 'This is scheduler priority proof, not a child-safety model accuracy claim.',
      fallbackBehavior: 'Keep parent-assistant work queued or degraded until child-safety work can run first.',
    }),
    providerProofEntry({
      proofEntryId: 'local-ai-proof-queued-degraded-lifecycle',
      requirement: 'queued-degraded-unavailable-lifecycle',
      proofStatus: 'degraded',
      sourceSchedulerStatus: degradedStatus,
      participatingRoles: ['parent-controller', 'ai-provider'],
      acceptedJobClasses: ['parent-assistant'],
      runtimeLoadCount: 1,
      childSafetyPriorityProved: false,
      parentAssistantSubmissionAllowed: true,
      evidenceLabel: 'Busy provider state reports degraded/overloaded without starting another runtime.',
      capabilityRequirement: 'Queued and degraded state must be explicit for Portal and runtime clients.',
      proofRequirement: 'Typed scheduler status and parent assistant runtime tests preserve queued degraded states.',
      claimBoundary: 'Degraded state is a runtime availability claim, not enforcement or safety approval.',
      fallbackBehavior: 'Return queued/degraded answers with no local AI result id when the provider is busy.',
    }),
    providerProofEntry({
      proofEntryId: 'local-ai-proof-parent-assistant-submit',
      requirement: 'parent-assistant-submits-when-allowed',
      proofStatus: 'proved',
      sourceSchedulerStatus: runningParentAssistantStatus,
      participatingRoles: ['parent-controller', 'ai-provider'],
      acceptedJobClasses: ['parent-assistant'],
      runtimeLoadCount: 1,
      childSafetyPriorityProved: false,
      parentAssistantSubmissionAllowed: true,
      evidenceLabel: 'Parent assistant uses the shared local provider scheduler when local execution is allowed.',
      capabilityRequirement:
        'Configured local runtime, local-only privacy mode, and allowed parent-assistant job class.',
      proofRequirement: 'Parent assistant runtime submits through the same scheduler instead of bypassing the lane.',
      claimBoundary: 'Parent assistant local submission does not authorize API/remote providers by default.',
      fallbackBehavior: 'Return unavailable or degraded when local runtime config is missing or the lane is busy.',
    }),
    providerProofEntry({
      proofEntryId: 'local-ai-proof-no-duplicate-model-load',
      requirement: 'no-duplicate-local-model-load',
      proofStatus: 'proved',
      sourceSchedulerStatus: queuedPriorityStatus,
      participatingRoles: ['parent-controller', 'child-agent', 'ai-provider'],
      acceptedJobClasses: ['child-safety', 'parent-assistant', 'parent-report'],
      runtimeLoadCount: 1,
      childSafetyPriorityProved: true,
      parentAssistantSubmissionAllowed: true,
      evidenceLabel: 'Runtime load count remains one while parent and child jobs share the provider lane.',
      capabilityRequirement: 'No duplicate local model load for the same physical device.',
      proofRequirement: 'Contract and service tests fail when runtimeLoadCount exceeds one.',
      claimBoundary: 'This does not claim memory sharing across different physical devices.',
      fallbackBehavior: 'Block duplicate runtime admission and queue/degrade additional work.',
    }),
    providerProofEntry({
      proofEntryId: 'local-ai-proof-status-contract-hardening',
      requirement: 'provider-status-contract-hardening',
      proofStatus: 'unavailable',
      sourceSchedulerStatus: unavailableStatus,
      participatingRoles: ['ai-provider'],
      acceptedJobClasses: [],
      runtimeLoadCount: 0,
      childSafetyPriorityProved: false,
      parentAssistantSubmissionAllowed: false,
      evidenceLabel: 'Unavailable provider state carries explicit provider, runtime, model, and reason fields.',
      capabilityRequirement: 'Unavailable/degraded provider status must be schema-valid and reasoned.',
      proofRequirement: 'Typed contracts reject unavailable provider status without an unavailable reason.',
      claimBoundary: 'Unavailable status is honest and must not be promoted to a working provider.',
      fallbackBehavior: 'Return unavailable with local-ai-provider-unconfigured and no selected runtime.',
    }),
  ],
});

function schedulerStatus(status: SchedulerStatusInput): Infer<typeof LocalAiProviderSchedulerStatusSchema> {
  return LocalAiProviderSchedulerStatusSchema.parse({
    physicalDeviceId: 'physical-device-local',
    singletonScope: 'physical-device',
    providerId: 'local-provider-llama-cli',
    runtimeReferenceId: 'local-ai-runtime-local-llama-cli',
    modelId: 'gemma-4-e2b-it-q4-k-m',
    modelReference: 'artifact:gemma_4_e2b_it_q4_k_m',
    resourceClass: 'cpu',
    lastCheckedAt: documentedAt,
    ...status,
  });
}

function providerProofEntry(entry: ProviderProofEntryInput): LocalAiRuntimeProviderProofEntry {
  return LocalAiRuntimeProviderProofEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    physicalDeviceId: entry.sourceSchedulerStatus.physicalDeviceId,
    singletonScope: entry.sourceSchedulerStatus.singletonScope,
    providerId: entry.sourceSchedulerStatus.providerId,
    runtimeReferenceId: entry.sourceSchedulerStatus.runtimeReferenceId,
    modelId: entry.sourceSchedulerStatus.modelId,
    modelReference: entry.sourceSchedulerStatus.modelReference,
    schedulerLifecycle: entry.sourceSchedulerStatus.lifecycleState,
    runtimeAccessLaneCount: 1,
    queue: entry.sourceSchedulerStatus.queue,
    degradedState: entry.sourceSchedulerStatus.degradedState,
    unavailableReason: entry.sourceSchedulerStatus.unavailableReason,
    duplicateRuntimeBlocked: entry.sourceSchedulerStatus.duplicateRuntimeBlocked,
    lastCheckedAt: documentedAt,
    ...entry,
  });
}

export const decodeLocalAiRuntimeProviderProofEntry = Schema.decodeUnknownSync(LocalAiRuntimeProviderProofEntrySchema);
export const decodeLocalAiRuntimeProviderProofReadModel = Schema.decodeUnknownSync(
  LocalAiRuntimeProviderProofReadModelSchema
);
