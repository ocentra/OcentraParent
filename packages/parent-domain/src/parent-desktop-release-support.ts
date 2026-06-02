import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { parentDesktopReleaseSupportReadModelIsHonest } from './parent-desktop-release-support-guards';
import { ParentTimestampSchema } from './reference-primitives';

const NonEmptyReleaseSupportText = Schema.String.pipe(Schema.minLength(1));

export const ParentDesktopReleaseSupportSchemaVersionSchema = withParser(
  Schema.Literal('parent-desktop-release-support-proof')
);
export const ParentDesktopReleaseSupportTargetSchema = withParser(
  Schema.Literal(
    'parent-desktop',
    'parent-mobile',
    'child-desktop',
    'child-android',
    'child-ios',
    'relay',
    'signing',
    'store',
    'support'
  )
);
export const ParentDesktopReleaseSupportStateSchema = withParser(
  Schema.Literal(
    'implemented',
    'scaffold',
    'unsigned-preview',
    'signature-required',
    'manual-required',
    'not-implemented',
    'production-promotion-required',
    'rollback-unavailable',
    'rollback-available',
    'ci-artifact-pending',
    'ci-artifact-present',
    'preview-only',
    'degraded',
    'unavailable'
  )
);
export const ParentDesktopReleaseSupportAuthorityRoleSchema = withParser(Schema.Literal('controller', 'observer'));
export const ParentDesktopReleaseSupportOperationSchema = withParser(
  Schema.Literal('read-service-state', 'read-route-state', 'write-policy', 'approve-request', 'take-controller')
);
export const ParentDesktopReleaseSupportOperationResultSchema = withParser(
  Schema.Literal('completed', 'rejected', 'disabled', 'manual-required')
);
export const ParentDesktopReleaseSupportRejectionReasonSchema = withParser(
  Schema.Literal('observer-read-only', 'controller-proof-required', 'production-promotion-required')
);
export const ParentDesktopReleaseSupportUpdateChannelSchema = withParser(
  Schema.Literal('scaffold', 'unsigned-preview', 'signature-required', 'production')
);
export const ParentDesktopReleaseSupportSigningSurfaceSchema = withParser(
  Schema.Literal('windows-code-signing', 'macos-notarization', 'google-play', 'testflight', 'app-store')
);
export const ParentDesktopReleaseSupportCiRunStatusSchema = withParser(
  Schema.Literal('success', 'failure', 'pending', 'not-checked-local', 'unavailable')
);
export const ParentDesktopReleaseSupportArtifactStateSchema = withParser(
  Schema.Literal('present', 'missing', 'pending', 'manual-required', 'not-checked-local')
);
export const ParentDesktopReleaseSupportReadinessClaimSchema = withParser(
  Schema.Literal('ready', 'preview-only', 'manual-required', 'not-ready')
);
export const ParentDesktopReleaseSupportDiagnosticFieldSchema = withParser(
  Schema.Literal('version', 'commit', 'platform', 'package', 'service', 'route', 'capability', 'degraded-state')
);
export const ParentDesktopReleaseSupportRedactionStateSchema = withParser(Schema.Literal('safe', 'redacted'));

const ReleaseSupportLabelSchema = NonEmptyReleaseSupportText.pipe(Schema.brand('ParentDesktopReleaseSupportLabel'));
const ReleaseSupportRequirementSchema = NonEmptyReleaseSupportText.pipe(
  Schema.brand('ParentDesktopReleaseSupportRequirement')
);
const ReleaseSupportCommandSchema = NonEmptyReleaseSupportText.pipe(Schema.brand('ParentDesktopReleaseSupportCommand'));
const ReleaseSupportProofPathSchema = NonEmptyReleaseSupportText.pipe(
  Schema.brand('ParentDesktopReleaseSupportProofPath')
);
const ReleaseSupportUrlSchema = NonEmptyReleaseSupportText.pipe(Schema.brand('ParentDesktopReleaseSupportUrl'));
const ReleaseSupportValueSchema = NonEmptyReleaseSupportText.pipe(Schema.brand('ParentDesktopReleaseSupportValue'));

export const ParentDesktopReleaseSupportAuthorityOperationSchema = withParser(
  Schema.Struct({
    operation: ParentDesktopReleaseSupportOperationSchema,
    result: ParentDesktopReleaseSupportOperationResultSchema,
    authorityRole: ParentDesktopReleaseSupportAuthorityRoleSchema,
    rejectionReason: Schema.Union(ParentDesktopReleaseSupportRejectionReasonSchema, Schema.Null),
    proofRequirement: ReleaseSupportRequirementSchema,
  })
);

export const ParentDesktopReleaseSupportMobileBridgeBoundarySchema = withParser(
  Schema.Struct({
    parentMobileState: ParentDesktopReleaseSupportStateSchema,
    childAndroidAgentState: ParentDesktopReleaseSupportStateSchema,
    childIosAgentState: ParentDesktopReleaseSupportStateSchema,
    parentMobileClaim: ReleaseSupportLabelSchema,
    childAgentNonClaim: ReleaseSupportLabelSchema,
  })
);

export const ParentDesktopReleaseSupportUpdateStateSchema = withParser(
  Schema.Struct({
    channel: ParentDesktopReleaseSupportUpdateChannelSchema,
    packageState: ParentDesktopReleaseSupportStateSchema,
    signingState: ParentDesktopReleaseSupportStateSchema,
    rollbackState: ParentDesktopReleaseSupportStateSchema,
    productionPromotionState: ParentDesktopReleaseSupportStateSchema,
    proofRequirement: ReleaseSupportRequirementSchema,
  })
);

export const ParentDesktopReleaseSupportSigningStoreStateSchema = withParser(
  Schema.Struct({
    surface: ParentDesktopReleaseSupportSigningSurfaceSchema,
    state: ParentDesktopReleaseSupportStateSchema,
    credentialState: ParentDesktopReleaseSupportStateSchema,
    proofRequirement: ReleaseSupportRequirementSchema,
  })
);

export const ParentDesktopReleaseSupportCapabilityRowSchema = withParser(
  Schema.Struct({
    target: ParentDesktopReleaseSupportTargetSchema,
    packageState: ParentDesktopReleaseSupportStateSchema,
    serviceState: ParentDesktopReleaseSupportStateSchema,
    routeState: ParentDesktopReleaseSupportStateSchema,
    capabilityState: ParentDesktopReleaseSupportStateSchema,
    proofLevel: ParentDesktopReleaseSupportReadinessClaimSchema,
    nonClaim: ReleaseSupportLabelSchema,
  })
);

export const ParentDesktopReleaseSupportCiArtifactProofSchema = withParser(
  Schema.Struct({
    workflowName: ReleaseSupportLabelSchema,
    runStatus: ParentDesktopReleaseSupportCiRunStatusSchema,
    artifactState: ParentDesktopReleaseSupportArtifactStateSchema,
    packageReadinessClaim: ParentDesktopReleaseSupportReadinessClaimSchema,
    checkedBy: ReleaseSupportCommandSchema,
    runUrl: Schema.Union(ReleaseSupportUrlSchema, Schema.Null),
  })
);

export const ParentDesktopReleaseSupportDiagnosticEntrySchema = withParser(
  Schema.Struct({
    field: ParentDesktopReleaseSupportDiagnosticFieldSchema,
    value: ReleaseSupportValueSchema,
    redactionState: ParentDesktopReleaseSupportRedactionStateSchema,
  })
);

export const ParentDesktopReleaseSupportDiagnosticsSchema = withParser(
  Schema.Struct({
    outputState: ParentDesktopReleaseSupportStateSchema,
    entries: Schema.Array(ParentDesktopReleaseSupportDiagnosticEntrySchema),
    redactedFields: Schema.Array(ReleaseSupportLabelSchema),
  })
);

export const ParentDesktopReleaseSupportManualRunbookEntrySchema = withParser(
  Schema.Struct({
    target: ParentDesktopReleaseSupportTargetSchema,
    hostOrDevice: ReleaseSupportLabelSchema,
    commandOrUiAction: ReleaseSupportCommandSchema,
    permissions: ReleaseSupportRequirementSchema,
    packageVersion: ReleaseSupportLabelSchema,
    logsScreenshotsProofJson: ReleaseSupportProofPathSchema,
    knownGaps: Schema.Array(ReleaseSupportLabelSchema),
  })
);

const ParentDesktopReleaseSupportReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentDesktopReleaseSupportSchemaVersionSchema,
  observerAuthority: Schema.Array(ParentDesktopReleaseSupportAuthorityOperationSchema),
  mobileBridgeBoundary: ParentDesktopReleaseSupportMobileBridgeBoundarySchema,
  updateStates: Schema.Array(ParentDesktopReleaseSupportUpdateStateSchema),
  signingStoreStates: Schema.Array(ParentDesktopReleaseSupportSigningStoreStateSchema),
  platformCapabilityMatrix: Schema.Array(ParentDesktopReleaseSupportCapabilityRowSchema),
  ciArtifactProof: ParentDesktopReleaseSupportCiArtifactProofSchema,
  supportDiagnostics: ParentDesktopReleaseSupportDiagnosticsSchema,
  manualRunbook: Schema.Array(ParentDesktopReleaseSupportManualRunbookEntrySchema),
  updatedAt: ParentTimestampSchema,
});

export type ParentDesktopReleaseSupportReadModel = Infer<typeof ParentDesktopReleaseSupportReadModelBaseSchema>;

export const ParentDesktopReleaseSupportReadModelSchema = withParser(
  ParentDesktopReleaseSupportReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        parentDesktopReleaseSupportReadModelIsHonest(readModel) ||
        'Expected parent desktop release support proof to keep observer writes rejected, mobile child-agent claims separate, unsigned preview and production promotion states explicit, support diagnostics redacted, and CI artifact readiness honest'
    )
  )
);

export type ParentDesktopReleaseSupportTarget = Infer<typeof ParentDesktopReleaseSupportTargetSchema>;
export type ParentDesktopReleaseSupportState = Infer<typeof ParentDesktopReleaseSupportStateSchema>;
export type ParentDesktopReleaseSupportOperation = Infer<typeof ParentDesktopReleaseSupportOperationSchema>;
export type ParentDesktopReleaseSupportSigningSurface = Infer<typeof ParentDesktopReleaseSupportSigningSurfaceSchema>;
export type ParentDesktopReleaseSupportDiagnosticField = Infer<typeof ParentDesktopReleaseSupportDiagnosticFieldSchema>;
export type ParentDesktopReleaseSupportAuthorityOperation = Infer<
  typeof ParentDesktopReleaseSupportAuthorityOperationSchema
>;
export type ParentDesktopReleaseSupportMobileBridgeBoundary = Infer<
  typeof ParentDesktopReleaseSupportMobileBridgeBoundarySchema
>;
export type ParentDesktopReleaseSupportUpdateState = Infer<typeof ParentDesktopReleaseSupportUpdateStateSchema>;
export type ParentDesktopReleaseSupportSigningStoreState = Infer<
  typeof ParentDesktopReleaseSupportSigningStoreStateSchema
>;
export type ParentDesktopReleaseSupportCapabilityRow = Infer<typeof ParentDesktopReleaseSupportCapabilityRowSchema>;
export type ParentDesktopReleaseSupportCiArtifactProof = Infer<typeof ParentDesktopReleaseSupportCiArtifactProofSchema>;
export type ParentDesktopReleaseSupportDiagnostics = Infer<typeof ParentDesktopReleaseSupportDiagnosticsSchema>;
export type ParentDesktopReleaseSupportManualRunbookEntry = Infer<
  typeof ParentDesktopReleaseSupportManualRunbookEntrySchema
>;
