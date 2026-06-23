import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  LocalAiDegradedStateSchema,
  LocalAiModelIdSchema,
  LocalAiModelReferenceSchema,
  LocalAiProviderIdSchema,
  LocalAiResourceClassSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnavailableReasonSchema,
} from './ai-primitives';
const LocalAiProviderSchedulerQueueCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const LocalAiPhysicalDeviceIdSchema = brandedNonEmptyStringSchema('LocalAiPhysicalDeviceId');

export const LocalAiProviderSingletonScopeSchema = withParser(Schema.Literal('physical-device'));

export const LocalAiProviderSchedulerLifecycleSchema = withParser(
  Schema.Literal('idle', 'running', 'queued', 'degraded', 'unavailable')
);

export const LocalAiProviderSchedulerJobClassSchema = withParser(
  Schema.Literal('child-safety', 'parent-assistant', 'parent-report')
);

export const LocalAiProviderSchedulerJobStatusSchema = withParser(
  Schema.Literal('accepted', 'running', 'queued', 'degraded', 'unavailable', 'complete')
);

export const LocalAiProviderSchedulerQueueSchema = withParser(
  Schema.Struct({
    childSafetyQueued: LocalAiProviderSchedulerQueueCountSchema,
    parentAssistantQueued: LocalAiProviderSchedulerQueueCountSchema,
    parentReportQueued: LocalAiProviderSchedulerQueueCountSchema,
  })
);

type LocalAiProviderSchedulerQueue = Infer<typeof LocalAiProviderSchedulerQueueSchema>;

const LocalAiProviderSchedulerStatusBaseSchema = Schema.Struct({
  physicalDeviceId: LocalAiPhysicalDeviceIdSchema,
  singletonScope: LocalAiProviderSingletonScopeSchema,
  providerId: LocalAiProviderIdSchema,
  runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
  modelId: LocalAiModelIdSchema,
  modelReference: LocalAiModelReferenceSchema,
  resourceClass: LocalAiResourceClassSchema,
  lifecycleState: LocalAiProviderSchedulerLifecycleSchema,
  currentJobClass: Schema.Union(LocalAiProviderSchedulerJobClassSchema, Schema.Null),
  queue: LocalAiProviderSchedulerQueueSchema,
  duplicateRuntimeBlocked: Schema.Boolean,
  degradedState: LocalAiDegradedStateSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  lastCheckedAt: LocalAiTimestampSchema,
});

type LocalAiProviderSchedulerStatusCandidate = Infer<typeof LocalAiProviderSchedulerStatusBaseSchema>;

export const LocalAiProviderSchedulerStatusSchema = withParser(
  LocalAiProviderSchedulerStatusBaseSchema.pipe(
    Schema.filter(
      (status) =>
        localAiProviderSchedulerStatusIsConsistent(status) ||
        'Expected local AI provider scheduler status to match lifecycle, queue, degradation, and unavailable state'
    )
  )
);

const LocalAiProviderSchedulerDecisionBaseSchema = Schema.Struct({
  physicalDeviceId: LocalAiPhysicalDeviceIdSchema,
  jobClass: LocalAiProviderSchedulerJobClassSchema,
  jobStatus: LocalAiProviderSchedulerJobStatusSchema,
  selectedRuntimeReferenceId: Schema.Union(LocalAiRuntimeReferenceIdSchema, Schema.Null),
  queuePosition: Schema.Union(LocalAiProviderSchedulerQueueCountSchema, Schema.Null),
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  duplicateRuntimeBlocked: Schema.Boolean,
});

type LocalAiProviderSchedulerDecisionCandidate = Infer<typeof LocalAiProviderSchedulerDecisionBaseSchema>;

export const LocalAiProviderSchedulerDecisionSchema = withParser(
  LocalAiProviderSchedulerDecisionBaseSchema.pipe(
    Schema.filter(
      (decision) =>
        localAiProviderSchedulerDecisionIsConsistent(decision) ||
        'Expected local AI provider scheduler decision to expose selected runtime, queue position, or unavailable reason'
    )
  )
);

function localAiProviderSchedulerStatusIsConsistent(status: LocalAiProviderSchedulerStatusCandidate): boolean {
  switch (status.lifecycleState) {
    case 'unavailable':
      return unavailableStatusIsConsistent(status);
    case 'running':
      return status.currentJobClass !== null && status.unavailableReason === null;
    case 'queued':
      return totalQueuedJobs(status.queue) > 0 && status.unavailableReason === null;
    case 'degraded':
      return status.degradedState !== 'none' && status.unavailableReason === null;
    case 'idle':
      return idleStatusIsConsistent(status);
  }

  return false;
}

function unavailableStatusIsConsistent(status: LocalAiProviderSchedulerStatusCandidate): boolean {
  return (
    status.currentJobClass === null &&
    totalQueuedJobs(status.queue) === 0 &&
    status.unavailableReason !== null &&
    status.degradedState === 'provider-unavailable'
  );
}

function idleStatusIsConsistent(status: LocalAiProviderSchedulerStatusCandidate): boolean {
  return (
    status.currentJobClass === null &&
    totalQueuedJobs(status.queue) === 0 &&
    status.unavailableReason === null &&
    status.degradedState === 'none'
  );
}

function localAiProviderSchedulerDecisionIsConsistent(decision: LocalAiProviderSchedulerDecisionCandidate): boolean {
  switch (decision.jobStatus) {
    case 'unavailable':
      return unavailableDecisionIsConsistent(decision);
    case 'queued':
      return queuedDecisionIsConsistent(decision);
    case 'running':
    case 'accepted':
    case 'complete':
      return activeRuntimeDecisionIsConsistent(decision);
    case 'degraded':
      return degradedDecisionIsConsistent(decision);
  }

  return false;
}

function unavailableDecisionIsConsistent(decision: LocalAiProviderSchedulerDecisionCandidate): boolean {
  return (
    decision.selectedRuntimeReferenceId === null &&
    decision.queuePosition === null &&
    decision.unavailableReason !== null &&
    decision.duplicateRuntimeBlocked === false
  );
}

function queuedDecisionIsConsistent(decision: LocalAiProviderSchedulerDecisionCandidate): boolean {
  return (
    runtimeDecisionHasSelectedRuntime(decision) &&
    decision.queuePosition !== null &&
    runtimeDecisionBlocksDuplicateRuntime(decision)
  );
}

function activeRuntimeDecisionIsConsistent(decision: LocalAiProviderSchedulerDecisionCandidate): boolean {
  return (
    runtimeDecisionHasSelectedRuntime(decision) &&
    decision.queuePosition === null &&
    runtimeDecisionBlocksDuplicateRuntime(decision)
  );
}

function degradedDecisionIsConsistent(decision: LocalAiProviderSchedulerDecisionCandidate): boolean {
  return runtimeDecisionHasSelectedRuntime(decision) && runtimeDecisionBlocksDuplicateRuntime(decision);
}

function runtimeDecisionHasSelectedRuntime(decision: LocalAiProviderSchedulerDecisionCandidate): boolean {
  return decision.selectedRuntimeReferenceId !== null && decision.unavailableReason === null;
}

function runtimeDecisionBlocksDuplicateRuntime(decision: LocalAiProviderSchedulerDecisionCandidate): boolean {
  return decision.duplicateRuntimeBlocked;
}

function totalQueuedJobs(queue: LocalAiProviderSchedulerQueue): number {
  return queue.childSafetyQueued + queue.parentAssistantQueued + queue.parentReportQueued;
}

export type LocalAiPhysicalDeviceId = typeof LocalAiPhysicalDeviceIdSchema.Type;
export type LocalAiProviderSingletonScope = Infer<typeof LocalAiProviderSingletonScopeSchema>;
export type LocalAiProviderSchedulerLifecycle = Infer<typeof LocalAiProviderSchedulerLifecycleSchema>;
export type LocalAiProviderSchedulerJobClass = Infer<typeof LocalAiProviderSchedulerJobClassSchema>;
export type LocalAiProviderSchedulerJobStatus = Infer<typeof LocalAiProviderSchedulerJobStatusSchema>;
export type LocalAiProviderSchedulerStatus = Infer<typeof LocalAiProviderSchedulerStatusSchema>;
export type LocalAiProviderSchedulerDecision = Infer<typeof LocalAiProviderSchedulerDecisionSchema>;
