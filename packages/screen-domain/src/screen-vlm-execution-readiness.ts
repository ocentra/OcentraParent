import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import {
  ScreenVlmWorkerJobSchema,
  ScreenVlmWorkerModelId,
  ScreenVlmWorkerResultSchema,
  ScreenVlmWorkerRuntimeRef,
  ScreenVlmWorkerTemplateVersion,
  type ScreenVlmWorkerJob,
  type ScreenVlmWorkerResult,
} from './screen-vlm-worker';
import {
  ScreenVlmExecutionReadinessNonClaimsSchema,
  ScreenVlmExecutionReadinessProofTier,
  ScreenVlmExecutionReadinessSchemaVersion,
  ScreenVlmExecutionReadinessStateSchema,
  ScreenVlmExecutionReadinessStatusSource,
} from './screen-vlm-execution-readiness-values';
import {
  ScreenEvidenceModelIdSchema,
  ScreenEvidenceModelRuntimeRefSchema,
  ScreenEvidenceQueueJobIdSchema,
  ScreenEvidenceResultIdSchema,
  ScreenEvidenceTemplateVersionSchema,
} from './screen-evidence-primitives';
import { ScreenEvidenceCustodyStateSchema } from './screen-evidence-states';

export {
  ScreenVlmExecutionReadinessNonClaimsSchema,
  ScreenVlmExecutionReadinessProofTier,
  ScreenVlmExecutionReadinessSchemaVersion,
  ScreenVlmExecutionReadinessStateSchema,
  ScreenVlmExecutionReadinessStatusSource,
} from './screen-vlm-execution-readiness-values';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const NonEmptyText = Schema.String.pipe(Schema.minLength(1));

export const ScreenVlmQueueHandoffSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmExecutionReadinessSchemaVersion),
    handoffId: NonEmptyText,
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    queuedAt: ActivityTimestampSchema,
    acceptedAt: ActivityTimestampSchema,
    status: Schema.Literal('queued', 'ready', 'running'),
    statusReason: NonEmptyText,
    job: ScreenVlmWorkerJobSchema,
    modelRuntimeRef: ScreenEvidenceModelRuntimeRefSchema,
    modelId: ScreenEvidenceModelIdSchema,
    promptOrTemplateVersion: ScreenEvidenceTemplateVersionSchema,
    queueAccepted: RequiredTrue,
    statusSource: Schema.Literal(ScreenVlmExecutionReadinessStatusSource),
    nonClaims: ScreenVlmExecutionReadinessNonClaimsSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        value.queueJobId === value.job.queueJobId ||
        'Expected VLM execution readiness handoff to reference the same queue job as the worker job'
    ),
    Schema.filter(
      (value) =>
        (value.modelRuntimeRef === value.job.modelRuntimeRef &&
          value.modelRuntimeRef === ScreenVlmWorkerRuntimeRef &&
          value.modelId === value.job.modelId &&
          value.modelId === ScreenVlmWorkerModelId &&
          value.promptOrTemplateVersion === value.job.promptOrTemplateVersion &&
          value.promptOrTemplateVersion === ScreenVlmWorkerTemplateVersion) ||
        'Expected VLM execution readiness handoff to preserve the worker model runtime, model id, and template version'
    ),
    Schema.filter(
      (value) =>
        (value.job.capabilityStatus === 'ready' &&
          value.job.custodyState === 'child-device-temp-queue' &&
          value.job.sourceEvidenceRefs.length > 0 &&
          value.job.localOnly &&
          !value.job.remoteAiUsed &&
          !value.job.rawImageRetained) ||
        'Expected VLM execution readiness handoff to start from ready encrypted temp-queue custody'
    )
  )
);

export const ScreenVlmExecutionStatusRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmExecutionReadinessSchemaVersion),
    statusId: ScreenEvidenceResultIdSchema,
    queueJobId: ScreenEvidenceQueueJobIdSchema,
    updatedAt: ActivityTimestampSchema,
    status: ScreenVlmExecutionReadinessStateSchema,
    statusReason: NonEmptyText,
    modelRuntimeRef: ScreenEvidenceModelRuntimeRefSchema,
    modelId: ScreenEvidenceModelIdSchema,
    promptOrTemplateVersion: ScreenEvidenceTemplateVersionSchema,
    custodyState: ScreenEvidenceCustodyStateSchema,
    queueAccepted: Schema.Boolean,
    result: Schema.Union(ScreenVlmWorkerResultSchema, Schema.Null),
    degradedReasons: Schema.Array(NonEmptyText),
    statusSource: Schema.Literal(ScreenVlmExecutionReadinessStatusSource),
    nonClaims: ScreenVlmExecutionReadinessNonClaimsSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        (value.modelRuntimeRef === ScreenVlmWorkerRuntimeRef &&
          value.modelId === ScreenVlmWorkerModelId &&
          value.promptOrTemplateVersion === ScreenVlmWorkerTemplateVersion) ||
        'Expected VLM execution readiness status rows to preserve the local VLM worker identifiers'
    ),
    Schema.filter(
      (value) =>
        value.status !== 'completed' ||
        (value.queueAccepted &&
          value.result !== null &&
          value.result.queueJobId === value.queueJobId &&
          value.result.imageDeletionState === 'deleted' &&
          value.result.custodyState === 'child-device-query-store' &&
          value.custodyState === 'child-device-query-store' &&
          !value.result.remoteAiUsed &&
          !value.result.rawImageRetained) ||
        'Expected completed VLM readiness rows to include a deleted local worker result in query-store custody'
    ),
    Schema.filter(
      (value) =>
        !['degraded', 'manual-required', 'unavailable'].includes(value.status) ||
        (value.result === null &&
          value.degradedReasons.length > 0 &&
          value.custodyState === 'child-device-temp-queue' &&
          !value.queueAccepted) ||
        'Expected non-completed VLM readiness failure rows to keep encrypted temp custody and list degraded reasons'
    ),
    Schema.filter(
      (value) =>
        !['queued', 'ready', 'running'].includes(value.status) ||
        (value.queueAccepted &&
          value.result === null &&
          value.degradedReasons.length === 0 &&
          value.custodyState === 'child-device-temp-queue') ||
        'Expected queued VLM readiness rows to be accepted without a result while raw evidence remains temp-queue bound'
    )
  )
);

export const ScreenVlmExecutionReadinessProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenVlmExecutionReadinessSchemaVersion),
    proofId: NonEmptyText,
    proofTier: Schema.Literal(ScreenVlmExecutionReadinessProofTier),
    handoffs: Schema.Array(ScreenVlmQueueHandoffSchema),
    statusRows: Schema.Array(ScreenVlmExecutionStatusRowSchema),
    localOnly: RequiredTrue,
    remoteAiUsed: RequiredFalse,
    rawImageRetained: RequiredFalse,
    liveModelExecutionClaimed: RequiredFalse,
    productionVlmQualityClaimed: RequiredFalse,
    portalRuntimeClaimed: RequiredFalse,
    enforcementClaimed: RequiredFalse,
  }).pipe(
    Schema.filter((value) => value.handoffs.length > 0 || 'Expected VLM readiness proof to include queue handoffs'),
    Schema.filter(
      (value) =>
        (value.statusRows.some((row) => row.status === 'queued') &&
          value.statusRows.some((row) => row.status === 'completed') &&
          value.statusRows.some((row) => row.status === 'degraded' || row.status === 'manual-required')) ||
        'Expected VLM readiness proof to cover queued, completed, and degraded/manual status rows'
    ),
    Schema.filter(
      (value) =>
        value.handoffs.every((handoff) =>
          value.statusRows.some((row) => row.queueJobId === handoff.queueJobId && row.queueAccepted)
        ) || 'Expected every VLM readiness handoff to have an accepted status row'
    )
  )
);

export function screenVlmQueueHandoffFromJob(input: {
  readonly job: ScreenVlmWorkerJob;
  readonly handoffId: unknown;
  readonly queuedAt: unknown;
  readonly acceptedAt: unknown;
  readonly statusReason: unknown;
}) {
  return ScreenVlmQueueHandoffSchema.parse({
    schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
    handoffId: input.handoffId,
    queueJobId: input.job.queueJobId,
    queuedAt: input.queuedAt,
    acceptedAt: input.acceptedAt,
    status: 'queued',
    statusReason: input.statusReason,
    job: input.job,
    modelRuntimeRef: input.job.modelRuntimeRef,
    modelId: input.job.modelId,
    promptOrTemplateVersion: input.job.promptOrTemplateVersion,
    queueAccepted: true,
    statusSource: ScreenVlmExecutionReadinessStatusSource,
    nonClaims: screenVlmExecutionReadinessNonClaims(),
  });
}

export function screenVlmQueuedStatusFromHandoff(input: {
  readonly handoff: ScreenVlmQueueHandoff;
  readonly statusId: unknown;
  readonly updatedAt: unknown;
}) {
  return ScreenVlmExecutionStatusRowSchema.parse({
    schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
    statusId: input.statusId,
    queueJobId: input.handoff.queueJobId,
    updatedAt: input.updatedAt,
    status: 'queued',
    statusReason: input.handoff.statusReason,
    modelRuntimeRef: input.handoff.modelRuntimeRef,
    modelId: input.handoff.modelId,
    promptOrTemplateVersion: input.handoff.promptOrTemplateVersion,
    custodyState: 'child-device-temp-queue',
    queueAccepted: true,
    result: null,
    degradedReasons: [],
    statusSource: ScreenVlmExecutionReadinessStatusSource,
    nonClaims: screenVlmExecutionReadinessNonClaims(),
  });
}

export function screenVlmCompletedStatusFromResult(input: {
  readonly result: ScreenVlmWorkerResult;
  readonly statusId: unknown;
}) {
  return ScreenVlmExecutionStatusRowSchema.parse({
    schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
    statusId: input.statusId,
    queueJobId: input.result.queueJobId,
    updatedAt: input.result.analyzedAt,
    status: 'completed',
    statusReason: input.result.summary,
    modelRuntimeRef: input.result.modelRuntimeRef,
    modelId: input.result.modelId,
    promptOrTemplateVersion: input.result.promptOrTemplateVersion,
    custodyState: input.result.custodyState,
    queueAccepted: true,
    result: input.result,
    degradedReasons: [],
    statusSource: ScreenVlmExecutionReadinessStatusSource,
    nonClaims: screenVlmExecutionReadinessNonClaims(),
  });
}

export function screenVlmManualRequiredStatus(input: {
  readonly queueJobId: unknown;
  readonly statusId: unknown;
  readonly updatedAt: unknown;
  readonly statusReason: unknown;
  readonly degradedReasons: readonly unknown[];
}) {
  return ScreenVlmExecutionStatusRowSchema.parse({
    schemaVersion: ScreenVlmExecutionReadinessSchemaVersion,
    statusId: input.statusId,
    queueJobId: input.queueJobId,
    updatedAt: input.updatedAt,
    status: 'manual-required',
    statusReason: input.statusReason,
    modelRuntimeRef: ScreenVlmWorkerRuntimeRef,
    modelId: ScreenVlmWorkerModelId,
    promptOrTemplateVersion: ScreenVlmWorkerTemplateVersion,
    custodyState: 'child-device-temp-queue',
    queueAccepted: false,
    result: null,
    degradedReasons: input.degradedReasons,
    statusSource: ScreenVlmExecutionReadinessStatusSource,
    nonClaims: screenVlmExecutionReadinessNonClaims(),
  });
}

export function screenVlmExecutionReadinessNonClaims() {
  return ScreenVlmExecutionReadinessNonClaimsSchema.parse({
    liveModelExecutionClaimed: false,
    productionVlmQualityClaimed: false,
    portalRuntimeClaimed: false,
    enforcementClaimed: false,
    remoteAiUsed: false,
    rawImageRetained: false,
  });
}

export type ScreenVlmExecutionReadinessState = Infer<typeof ScreenVlmExecutionReadinessStateSchema>;
export type ScreenVlmExecutionReadinessNonClaims = Infer<typeof ScreenVlmExecutionReadinessNonClaimsSchema>;
export type ScreenVlmQueueHandoff = Infer<typeof ScreenVlmQueueHandoffSchema>;
export type ScreenVlmExecutionStatusRow = Infer<typeof ScreenVlmExecutionStatusRowSchema>;
export type ScreenVlmExecutionReadinessProof = Infer<typeof ScreenVlmExecutionReadinessProofSchema>;
