import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityTimestampSchema } from './evidence-primitives';
import {
  AppGameCatalogReadyStateSchema,
  AppGameCatalogRefSchema,
  AppGameCapabilityStatusSchema,
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

const AppGameForegroundEvidenceIdSchema = brandedNonEmptyStringSchema('AppGameForegroundEvidenceId');
const AppGameWindowRefSchema = brandedNonEmptyStringSchema('AppGameWindowRef');
const AppGameWindowTitleRefSchema = brandedNonEmptyStringSchema('AppGameWindowTitleRef');
const AppGameWindowTitleCaptureStateSchema = withParser(
  Schema.Literal('titleRef', 'titleOmitted', 'permissionLimited', 'adapterError', 'notClaimed')
);
const AppGameContentKnowledgeStateSchema = withParser(Schema.Literal('notClaimed'));

const AppGameForegroundEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  foregroundEvidenceId: AppGameForegroundEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  processIdentity: AppGameProcessIdentitySchema,
  processId: AppGameNonNegativeCountSchema,
  processName: AppGameProcessNameSchema,
  inventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
  launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  windowRef: Schema.Union(AppGameWindowRefSchema, Schema.Null),
  windowTitleRef: Schema.Union(AppGameWindowTitleRefSchema, Schema.Null),
  titleCaptureState: AppGameWindowTitleCaptureStateSchema,
  foregroundStartedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  foregroundEndedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  foregroundDurationMs: AppGameNonNegativeDurationSchema,
  runtimeState: AppGameRuntimeStateSchema,
  foregroundState: AppGameForegroundStateSchema,
  observationMode: AppGameObservationModeSchema,
  classificationState: AppGameClassificationStateSchema,
  catalogReadyState: AppGameCatalogReadyStateSchema,
  capabilityStatus: AppGameCapabilityStatusSchema,
  contentKnowledgeState: AppGameContentKnowledgeStateSchema,
  confidence: AppGameConfidenceSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const AppGameForegroundEvidenceSchema = withParser(
  AppGameForegroundEvidenceBaseSchema.pipe(
    Schema.filter(
      (foreground) =>
        foreground.observationMode === 'foregroundWindow' ||
        'Expected foreground evidence to use the foreground-window observation mode'
    )
  )
    .pipe(
      Schema.filter(
        (foreground) =>
          foreground.contentKnowledgeState === 'notClaimed' ||
          'Expected foreground evidence to avoid content knowledge claims'
      )
    )
    .pipe(
      Schema.filter(
        (foreground) =>
          foreground.foregroundState !== 'foreground' ||
          (foreground.foregroundStartedAt !== null && foreground.foregroundEndedAt === null) ||
          'Expected foreground evidence to start an open foreground interval'
      )
    )
    .pipe(
      Schema.filter(
        (foreground) =>
          foreground.foregroundState !== 'background' ||
          foreground.foregroundEndedAt !== null ||
          'Expected background foreground rows to close an interval or gap'
      )
    )
    .pipe(
      Schema.filter(
        (foreground) =>
          foreground.titleCaptureState !== 'titleRef' ||
          foreground.windowTitleRef !== null ||
          'Expected title-ref state to include a window title ref'
      )
    )
    .pipe(
      Schema.filter(
        (foreground) =>
          foreground.windowTitleRef === null ||
          foreground.titleCaptureState === 'titleRef' ||
          'Expected title refs to stay behind title-ref capture state'
      )
    )
);

export type AppGameForegroundEvidence = Infer<typeof AppGameForegroundEvidenceSchema>;
