import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/evidence-domain/contracts';
import {
  ActivityEvidenceDigestSchema,
  ActivityEvidenceIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/evidence-domain/primitives';
import {
  AppGameAppUserModelIdSchema,
  AppGameApplicationTokenRefSchema,
  AppGameBundleIdSchema,
  AppGameDesktopEntryIdSchema,
  AppGameDisplayLabelSchema,
  AppGameExecutablePathRefSchema,
  AppGameFileHashRefSchema,
  AppGameIdentityConfidenceSchema,
  AppGameIdentityDeterministicRefKindSchema,
  AppGameIdentityIdSchema,
  AppGameLauncherAppIdSchema,
  AppGameLauncherManifestIdSchema,
  AppGamePackageIdSchema,
  AppGameParentLabelSchema,
  AppGameProductKindSchema,
  AppGamePublisherSignatureRefSchema,
  AppGameStoreIdSchema,
} from './app-game-identity-primitives';
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
import { AppGameSessionEndReasonSchema, AppGameSessionRollupDateSchema } from './app-game-session-primitives';

export * from './app-game-identity-primitives';
export * from './app-game-inventory';
export * from './app-game-inventory-primitives';
export * from './app-game-launcher';
export * from './app-game-category-risk';
export * from './app-game-foreground';
export * from './app-game-primitives';
export * from './app-game-runtime';
export * from './app-game-session-primitives';

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

const AppGameIdentityBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  identityId: AppGameIdentityIdSchema,
  productKind: AppGameProductKindSchema,
  displayLabel: AppGameDisplayLabelSchema,
  parentLabel: Schema.Union(AppGameParentLabelSchema, Schema.Null),
  confidence: AppGameIdentityConfidenceSchema,
  classificationState: AppGameClassificationStateSchema,
  packageId: Schema.Union(AppGamePackageIdSchema, Schema.Null),
  bundleId: Schema.Union(AppGameBundleIdSchema, Schema.Null),
  appUserModelId: Schema.Union(AppGameAppUserModelIdSchema, Schema.Null),
  desktopEntryId: Schema.Union(AppGameDesktopEntryIdSchema, Schema.Null),
  applicationTokenRef: Schema.Union(AppGameApplicationTokenRefSchema, Schema.Null),
  executablePathRef: Schema.Union(AppGameExecutablePathRefSchema, Schema.Null),
  publisherSignatureRef: Schema.Union(AppGamePublisherSignatureRefSchema, Schema.Null),
  fileHashRef: Schema.Union(AppGameFileHashRefSchema, Schema.Null),
  launcherRef: Schema.Union(AppGameLauncherRefSchema, Schema.Null),
  launcherAppId: Schema.Union(AppGameLauncherAppIdSchema, Schema.Null),
  launcherManifestId: Schema.Union(AppGameLauncherManifestIdSchema, Schema.Null),
  storeId: Schema.Union(AppGameStoreIdSchema, Schema.Null),
  catalogRef: Schema.Union(AppGameCatalogRefSchema, Schema.Null),
  childGameEvidenceClaimId: Schema.Union(AppGameEvidenceClaimIdSchema, Schema.Null),
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const AppGameIdentitySchema = withParser(
  AppGameIdentityBaseSchema.pipe(
    Schema.filter(
      (identity) =>
        appGameIdentityHasRawReference(identity) ||
        (identity.confidence === 'weak' &&
          identity.classificationState === 'unknownProcess' &&
          identity.productKind === 'unknownExecutable') ||
        'Expected display-label-only app/game identity to stay weak and unknown'
    )
  )
    .pipe(
      Schema.filter(
        (identity) =>
          appGameIdentityConfidenceMatchesReferences(identity) ||
          'Expected deterministic and parent-labeled identities to include raw identity refs'
      )
    )
    .pipe(
      Schema.filter(
        (identity) =>
          appGameIdentityLauncherStateIsHonest(identity) ||
          'Expected launcher-only identity to stay launcher or candidate without child-game proof'
      )
    )
);

const AppGameIdentityMergeProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(AppGameSchemaVersion),
  mergeId: AppGameIdentityIdSchema,
  targetIdentity: AppGameIdentitySchema,
  sourceIdentityIds: Schema.Array(AppGameIdentityIdSchema),
  mergeConfidence: AppGameConfidenceSchema,
  displayLabelMatched: Schema.Boolean,
  parentLabelChanged: Schema.Boolean,
  conflictingFileHashRefs: Schema.Boolean,
  sharedDeterministicRefs: Schema.Array(AppGameIdentityDeterministicRefKindSchema),
  evidence: Schema.Array(ActivityEvidenceRefSchema),
});

export const AppGameIdentityMergeProofSchema = withParser(
  AppGameIdentityMergeProofBaseSchema.pipe(
    Schema.filter((merge) => merge.sourceIdentityIds.length >= 2 || 'Expected identity merge to cite source identities')
  )
    .pipe(
      Schema.filter(
        (merge) => !merge.conflictingFileHashRefs || 'Expected conflicting file hashes to block identity merge'
      )
    )
    .pipe(
      Schema.filter(
        (merge) =>
          merge.mergeConfidence <= 0.3 ||
          merge.sharedDeterministicRefs.length > 0 ||
          'Expected non-weak identity merge to share deterministic refs'
      )
    )
    .pipe(
      Schema.filter(
        (merge) =>
          !merge.parentLabelChanged ||
          (merge.targetIdentity.parentLabel !== null && merge.sharedDeterministicRefs.length > 0) ||
          'Expected parent labels to change display only, not raw identity'
      )
    )
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
  endReason: Schema.Union(AppGameSessionEndReasonSchema, Schema.Null),
  runningDurationMs: AppGameNonNegativeDurationSchema,
  foregroundDurationMs: AppGameNonNegativeDurationSchema,
  backgroundDurationMs: AppGameNonNegativeDurationSchema,
  lastForegroundAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  lastBackgroundAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  observationGapMs: AppGameNonNegativeDurationSchema,
  observationCount: AppGameNonNegativeCountSchema,
  evidenceCount: AppGameNonNegativeCountSchema,
  evidence: Schema.Array(ActivityEvidenceRefSchema),
  aiDigestRef: Schema.Union(AppGameAiDigestRefSchema, Schema.Null),
  confidence: AppGameConfidenceSchema,
});

export const AppGameSessionDailyRollupSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    rollupDate: AppGameSessionRollupDateSchema,
    classificationState: AppGameClassificationStateSchema,
    sessionCount: AppGameNonNegativeCountSchema,
    runningDurationMs: AppGameNonNegativeDurationSchema,
    foregroundDurationMs: AppGameNonNegativeDurationSchema,
    backgroundDurationMs: AppGameNonNegativeDurationSchema,
    evidenceCount: AppGameNonNegativeCountSchema,
    sessionIds: Schema.Array(AppGameSessionIdSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  }).pipe(
    Schema.filter(
      (rollup) =>
        rollup.foregroundDurationMs + rollup.backgroundDurationMs === rollup.runningDurationMs ||
        'Expected rollup background duration to equal running duration minus foreground duration'
    )
  )
);

export const AppGameSessionSummarySchema = withParser(
  AppGameSessionSummaryBaseSchema.pipe(
    Schema.filter(
      (session) =>
        session.foregroundDurationMs + session.backgroundDurationMs === session.runningDurationMs ||
        'Expected background duration to equal running duration minus foreground duration'
    )
  )
    .pipe(
      Schema.filter(
        (session) =>
          (session.endedAt === null && session.endReason === null) ||
          (session.endedAt !== null && session.endReason !== null) ||
          'Expected closed sessions to pair endedAt with an end reason'
      )
    )
    .pipe(
      Schema.filter(
        (session) =>
          session.foregroundDurationMs === 0 ||
          session.lastForegroundAt !== null ||
          'Expected foreground duration to cite the last foreground evidence time'
      )
    )
    .pipe(
      Schema.filter(
        (session) =>
          session.backgroundDurationMs === 0 ||
          session.lastBackgroundAt !== null ||
          'Expected background duration to cite the last background evidence time'
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
export type AppGameIdentity = Infer<typeof AppGameIdentitySchema>;
export type AppGameIdentityMergeProof = Infer<typeof AppGameIdentityMergeProofSchema>;
export type AppGameEvidenceClaim = Infer<typeof AppGameEvidenceClaimSchema>;
export type AppGameProcessObservation = Infer<typeof AppGameProcessObservationSchema>;
export type AppGameSessionSummary = Infer<typeof AppGameSessionSummarySchema>;
export type AppGameSessionDailyRollup = Infer<typeof AppGameSessionDailyRollupSchema>;
export type AppGameSessionQuery = Infer<typeof AppGameSessionQuerySchema>;
export type AppGameSessionQueryResult = Infer<typeof AppGameSessionQueryResultSchema>;
export type AppGameSessionReport = Infer<typeof AppGameSessionReportSchema>;
export type AppGameAiDigestReference = Infer<typeof AppGameAiDigestReferenceSchema>;
export type AppGameAiClassificationDigest = Infer<typeof AppGameAiClassificationDigestSchema>;

function appGameIdentityHasRawReference(identity: Infer<typeof AppGameIdentityBaseSchema>): boolean {
  return (
    appGameIdentityHasDeterministicReference(identity) ||
    identity.launcherRef !== null ||
    identity.launcherAppId !== null ||
    identity.launcherManifestId !== null
  );
}

function appGameIdentityHasDeterministicReference(identity: Infer<typeof AppGameIdentityBaseSchema>): boolean {
  return (
    identity.packageId !== null ||
    identity.bundleId !== null ||
    identity.appUserModelId !== null ||
    identity.desktopEntryId !== null ||
    identity.applicationTokenRef !== null ||
    identity.executablePathRef !== null ||
    identity.publisherSignatureRef !== null ||
    identity.fileHashRef !== null ||
    identity.storeId !== null ||
    identity.catalogRef !== null ||
    identity.childGameEvidenceClaimId !== null
  );
}

function appGameIdentityConfidenceMatchesReferences(identity: Infer<typeof AppGameIdentityBaseSchema>): boolean {
  if (identity.confidence === 'deterministic' || identity.confidence === 'parentLabeled') {
    return appGameIdentityHasDeterministicReference(identity) || appGameIdentityIsDeterministicLauncher(identity);
  }

  return true;
}

function appGameIdentityIsDeterministicLauncher(identity: Infer<typeof AppGameIdentityBaseSchema>): boolean {
  return identity.productKind === 'launcher' && appGameIdentityHasRawReference(identity);
}

function appGameIdentityLauncherStateIsHonest(identity: Infer<typeof AppGameIdentityBaseSchema>): boolean {
  if (!appGameIdentityHasOnlyLauncherReferences(identity)) {
    return true;
  }

  return identity.productKind === 'launcher' && identity.classificationState !== 'knownGame';
}

function appGameIdentityHasOnlyLauncherReferences(identity: Infer<typeof AppGameIdentityBaseSchema>): boolean {
  return (
    !appGameIdentityHasDeterministicReference(identity) &&
    (identity.launcherRef !== null || identity.launcherAppId !== null || identity.launcherManifestId !== null)
  );
}
