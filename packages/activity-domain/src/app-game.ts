import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityEvidenceDigestSchema, ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  AppGameAiActionHintSchema,
  AppGameAiDigestRefSchema,
  AppGameCapabilityStatusSchema,
  AppGameCatalogReadyStateSchema,
  AppGameCatalogRefSchema,
  AppGameClassificationStateSchema,
  AppGameConfidenceSchema,
  AppGameDisplayNameSchema,
  AppGameEvidenceClaimIdSchema,
  AppGameEvidenceClaimKindSchema,
  AppGameExecutablePathSchema,
  AppGameForegroundStateSchema,
  AppGameInventoryEntryIdSchema,
  AppGameIdentityStrengthSchema,
  AppGameLauncherKindSchema,
  AppGameLauncherRefSchema,
  AppGameNonNegativeCountSchema,
  AppGameNonNegativeDurationSchema,
  AppGameObservationModeSchema,
  AppGameProcessIdentitySchema,
  AppGameProcessNameSchema,
  AppGameRuntimeStateSchema,
  AppGameSchemaVersion,
  AppGameSessionIdSchema,
  AppGameUnavailableReasonSchema,
} from './app-game-primitives';

export * from './app-game-primitives';

export const AppGameInventoryEntrySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    inventoryEntryId: AppGameInventoryEntryIdSchema,
    observedAt: ActivityTimestampSchema,
    displayName: AppGameDisplayNameSchema,
    executablePath: Schema.Union(AppGameExecutablePathSchema, Schema.Null),
    launcherKind: Schema.Union(AppGameLauncherKindSchema, Schema.Null),
    launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
    catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
    catalogReadyState: AppGameCatalogReadyStateSchema,
    classificationState: AppGameClassificationStateSchema,
    capabilityStatus: AppGameCapabilityStatusSchema,
    confidence: AppGameConfidenceSchema,
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

const AppGameEvidenceClaimBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  claimId: AppGameEvidenceClaimIdSchema,
  observedAt: ActivityTimestampSchema,
  claimKind: AppGameEvidenceClaimKindSchema,
  observationMode: AppGameObservationModeSchema,
  displayName: AppGameDisplayNameSchema,
  identityStrength: AppGameIdentityStrengthSchema,
  classificationState: AppGameClassificationStateSchema,
  catalogReadyState: AppGameCatalogReadyStateSchema,
  runtimeState: AppGameRuntimeStateSchema,
  foregroundState: AppGameForegroundStateSchema,
  inventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
  processIdentity: Schema.Union(AppGameProcessIdentitySchema, Schema.Null),
  launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  confidence: AppGameConfidenceSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const AppGameEvidenceClaimSchema = withParser(
  AppGameEvidenceClaimBaseSchema.pipe(
    Schema.filter(
      (claim) =>
        claim.identityStrength !== 'displayNameOnly' ||
        (claim.confidence <= 0.3 &&
          claim.inventoryEntryId === null &&
          claim.processIdentity === null &&
          claim.launcherRef === null &&
          claim.catalogRef === null) ||
        'Expected display-name-only app/game identity to remain weak and unlinked'
    )
  )
    .pipe(
      Schema.filter(
        (claim) =>
          claim.claimKind !== 'inventory' ||
          (claim.runtimeState === 'notClaimed' && claim.foregroundState === 'notClaimed') ||
          'Expected inventory evidence to avoid running or foreground claims'
      )
    )
    .pipe(
      Schema.filter(
        (claim) =>
          claim.claimKind !== 'launcher' ||
          claim.classificationState !== 'knownGame' ||
          claim.identityStrength === 'childGameProof' ||
          'Expected launcher evidence to require child-game proof before known-game classification'
      )
    )
);

export const AppGameProcessObservationSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    observedAt: ActivityTimestampSchema,
    processIdentity: AppGameProcessIdentitySchema,
    processId: AppGameNonNegativeCountSchema,
    processName: AppGameProcessNameSchema,
    executablePath: Schema.Union(AppGameExecutablePathSchema, Schema.Null),
    foregroundState: AppGameForegroundStateSchema,
    observationMode: AppGameObservationModeSchema,
    classificationState: AppGameClassificationStateSchema,
    inventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
    launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
    catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
    confidence: AppGameConfidenceSchema,
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

const AppGameSessionSummaryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  sessionId: AppGameSessionIdSchema,
  primaryProcessIdentity: AppGameProcessIdentitySchema,
  displayName: AppGameDisplayNameSchema,
  classificationState: AppGameClassificationStateSchema,
  catalogReadyState: AppGameCatalogReadyStateSchema,
  inventoryEntryId: Schema.Union(AppGameInventoryEntryIdSchema, Schema.Null),
  launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  startedAt: ActivityTimestampSchema,
  lastObservedAt: ActivityTimestampSchema,
  endedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  runningDurationMs: AppGameNonNegativeDurationSchema,
  foregroundDurationMs: AppGameNonNegativeDurationSchema,
  backgroundDurationMs: AppGameNonNegativeDurationSchema,
  observationCount: AppGameNonNegativeCountSchema,
  evidenceCount: AppGameNonNegativeCountSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
  aiDigestRef: Schema.Union(AppGameAiDigestRefSchema, Schema.Null),
  confidence: AppGameConfidenceSchema,
});

export const AppGameSessionSummarySchema = withParser(
  AppGameSessionSummaryBaseSchema.pipe(
    Schema.filter(
      (session) =>
        session.foregroundDurationMs + session.backgroundDurationMs <= session.runningDurationMs ||
        'Expected foreground and background duration within running duration'
    )
  )
);

export const AppGameSessionQuerySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    limit: AppGameNonNegativeCountSchema,
    includeEnded: Schema.Boolean,
    classificationStates: Schema.Array(AppGameClassificationStateSchema),
  })
);

export const AppGameSessionQueryResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    limit: AppGameNonNegativeCountSchema,
    returned: AppGameNonNegativeCountSchema,
    catalogReadyState: AppGameCatalogReadyStateSchema,
    firstObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    lastObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    sessions: Schema.Array(AppGameSessionSummarySchema),
  })
);

export const AppGameSessionReportSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    limit: AppGameNonNegativeCountSchema,
    returned: AppGameNonNegativeCountSchema,
    catalogReadyState: AppGameCatalogReadyStateSchema,
    firstObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    lastObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    mostRecentSessionId: Schema.Union(AppGameSessionIdSchema, Schema.Null),
    mostRecentClassificationState: Schema.Union(AppGameClassificationStateSchema, Schema.Null),
    mostRecentProcessIdentity: Schema.Union(AppGameProcessIdentitySchema, Schema.Null),
    mostRecentDisplayName: Schema.Union(AppGameDisplayNameSchema, Schema.Null),
    mostRecentRunningDurationMs: Schema.Union(AppGameNonNegativeDurationSchema, Schema.Null),
    mostRecentForegroundDurationMs: Schema.Union(AppGameNonNegativeDurationSchema, Schema.Null),
    mostRecentEvidenceCount: Schema.Union(AppGameNonNegativeCountSchema, Schema.Null),
  })
);

export const AppGameAiDigestReferenceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    digestRef: AppGameAiDigestRefSchema,
    digest: Schema.Union(ActivityEvidenceDigestSchema, Schema.Null),
    generatedAt: ActivityTimestampSchema,
    confidence: AppGameConfidenceSchema,
    sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema),
    sourceSessionIds: Schema.Array(AppGameSessionIdSchema),
    unavailableReason: Schema.Union(AppGameUnavailableReasonSchema, Schema.Null),
  })
);

export const AppGameAiClassificationDigestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    digestRef: AppGameAiDigestRefSchema,
    digest: Schema.Union(ActivityEvidenceDigestSchema, Schema.Null),
    generatedAt: ActivityTimestampSchema,
    classificationState: AppGameClassificationStateSchema,
    confidence: AppGameConfidenceSchema,
    actionHints: Schema.Array(AppGameAiActionHintSchema),
    sourceEvidenceIds: Schema.Array(ActivityEvidenceIdSchema),
    sourceSessionIds: Schema.Array(AppGameSessionIdSchema),
    unavailableReason: Schema.Union(AppGameUnavailableReasonSchema, Schema.Null),
  })
);

export type AppGameInventoryEntry = Infer<typeof AppGameInventoryEntrySchema>;
export type AppGameEvidenceClaim = Infer<typeof AppGameEvidenceClaimSchema>;
export type AppGameProcessObservation = Infer<typeof AppGameProcessObservationSchema>;
export type AppGameSessionSummary = Infer<typeof AppGameSessionSummarySchema>;
export type AppGameSessionQuery = Infer<typeof AppGameSessionQuerySchema>;
export type AppGameSessionQueryResult = Infer<typeof AppGameSessionQueryResultSchema>;
export type AppGameSessionReport = Infer<typeof AppGameSessionReportSchema>;
export type AppGameAiDigestReference = Infer<typeof AppGameAiDigestReferenceSchema>;
export type AppGameAiClassificationDigest = Infer<typeof AppGameAiClassificationDigestSchema>;
