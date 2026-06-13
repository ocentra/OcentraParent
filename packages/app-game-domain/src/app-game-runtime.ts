import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import {
  AppGameExecutablePathRefSchema,
  AppGameFileHashRefSchema,
  AppGamePublisherSignatureRefSchema,
} from './app-game-identity-primitives';
import {
  AppGameCapabilityStatusSchema,
  AppGameCatalogReadyStateSchema,
  AppGameCatalogRefSchema,
  AppGameClassificationStateSchema,
  AppGameConfidenceSchema,
  AppGameForegroundStateSchema,
  AppGameInventoryEntryIdSchema,
  AppGameLauncherRefSchema,
  AppGameNonNegativeCountSchema,
  AppGameNonNegativeDurationSchema,
  AppGameObservationModeSchema,
  AppGameProcessIdentitySchema,
  AppGameProcessNameSchema,
  AppGameRuntimeStateSchema,
  AppGameSchemaVersion,
} from './app-game-primitives';

const AppGameRuntimeEvidenceIdSchema = brandedNonEmptyStringSchema('AppGameRuntimeEvidenceId');

const AppGameRuntimeEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  runtimeEvidenceId: AppGameRuntimeEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  processIdentity: AppGameProcessIdentitySchema,
  processId: AppGameNonNegativeCountSchema,
  parentProcessId: Schema.Union(AppGameNonNegativeCountSchema, Schema.Null),
  processName: AppGameProcessNameSchema,
  executablePathRef: Schema.Union(AppGameExecutablePathRefSchema, Schema.Null),
  publisherSignatureRef: Schema.Union(AppGamePublisherSignatureRefSchema, Schema.Null),
  fileHashRef: Schema.Union(AppGameFileHashRefSchema, Schema.Null),
  inventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
  launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  startedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  exitedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  runningDurationMs: AppGameNonNegativeDurationSchema,
  runtimeState: AppGameRuntimeStateSchema,
  foregroundState: AppGameForegroundStateSchema,
  observationMode: AppGameObservationModeSchema,
  classificationState: AppGameClassificationStateSchema,
  catalogReadyState: AppGameCatalogReadyStateSchema,
  capabilityStatus: AppGameCapabilityStatusSchema,
  confidence: AppGameConfidenceSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const AppGameRuntimeEvidenceSchema = withParser(
  AppGameRuntimeEvidenceBaseSchema.pipe(
    Schema.filter(
      (runtime) => runtime.foregroundState === 'notClaimed' || 'Expected runtime evidence to avoid foreground claims'
    )
  )
    .pipe(
      Schema.filter(
        (runtime) =>
          runtime.observationMode !== 'processExit' ||
          (runtime.runtimeState === 'notRunning' && runtime.exitedAt !== null) ||
          'Expected process-exit runtime evidence to close the runtime state'
      )
    )
    .pipe(
      Schema.filter(
        (runtime) =>
          runtime.observationMode !== 'processStart' ||
          (runtime.runtimeState === 'running' && runtime.startedAt !== null && runtime.exitedAt === null) ||
          'Expected process-start runtime evidence to start a running state'
      )
    )
);

export type AppGameRuntimeEvidence = Infer<typeof AppGameRuntimeEvidenceSchema>;
