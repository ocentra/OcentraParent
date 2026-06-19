import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { parentDesktopReleaseSupportReadModelIsHonest } from './parent-desktop-release-support-guards';
import {
  ParentDesktopReleaseSupportIncidentHandoffSchema,
  type ParentDesktopReleaseSupportIncidentHandoff,
} from './parent-desktop-release-support-incident';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export { ParentDesktopReleaseSupportIncidentHandoffSchema, type ParentDesktopReleaseSupportIncidentHandoff };

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
const ParentDesktopReleaseSupportFrontendSourceSchema = withParser(Schema.Literal('built-portal-dist'));
const ParentDesktopReleaseSupportBackendBoundarySchema = withParser(Schema.Literal('rust-service-boundary'));
const ParentDesktopReleaseSupportServiceLaunchOwnerSchema = withParser(Schema.Literal('package-service-manager'));
const ParentDesktopReleaseSupportPortOwnershipSchema = withParser(Schema.Literal('fixed-loopback'));
const ParentDesktopReleaseSupportPortConflictPolicySchema = withParser(Schema.Literal('no-foreign-process-reclaim'));
const ParentDesktopReleaseSupportProcessOwnershipSchema = withParser(Schema.Literal('parent-shell-only'));
const ParentDesktopReleaseSupportBlankWindowGuardSchema = withParser(Schema.Literal('frontend-dist-required'));
const ParentDesktopReleaseSupportUpdateRollbackPostureSchema = withParser(Schema.Literal('signed-channel-required'));

const ReleaseSupportLabelSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportLabel');
const ReleaseSupportRequirementSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportRequirement');
const ReleaseSupportCommandSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportCommand');
const ReleaseSupportProofPathSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportProofPath');
const ReleaseSupportUrlSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportUrl');
const ReleaseSupportValueSchema = brandedNonEmptyStringSchema('ParentDesktopReleaseSupportValue');

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

export const ParentDesktopReleaseSupportPackageRuntimeEvidenceSchema = withParser(
  Schema.Struct({
    packageFrontendSource: ParentDesktopReleaseSupportFrontendSourceSchema,
    backendBoundary: ParentDesktopReleaseSupportBackendBoundarySchema,
    serviceLaunchOwner: ParentDesktopReleaseSupportServiceLaunchOwnerSchema,
    serviceHealthState: ParentDesktopReleaseSupportStateSchema,
    connectOrDegradeState: ParentDesktopReleaseSupportStateSchema,
    fixedAgentAddress: ReleaseSupportLabelSchema,
    portOwnership: ParentDesktopReleaseSupportPortOwnershipSchema,
    portConflictPolicy: ParentDesktopReleaseSupportPortConflictPolicySchema,
    processOwnership: ParentDesktopReleaseSupportProcessOwnershipSchema,
    blankWindowGuard: ParentDesktopReleaseSupportBlankWindowGuardSchema,
    updateRollbackPosture: ParentDesktopReleaseSupportUpdateRollbackPostureSchema,
    artifactState: ParentDesktopReleaseSupportArtifactStateSchema,
    supportDiagnosticState: ParentDesktopReleaseSupportStateSchema,
    nonClaim: ReleaseSupportLabelSchema,
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

export const ParentDesktopReleaseSupportPackagePreviewArtifactNameSchema = withParser(
  Schema.Literal(
    'ocentra-parent-windows-x64-preview',
    'ocentra-parent-linux-amd64-preview',
    'ocentra-parent-macos-preview',
    'ocentra-parent-android-preview',
    'ocentra-parent-ios-simulator-preview'
  )
);

export const ParentDesktopReleaseSupportPackagePreviewArtifactSchema = withParser(
  Schema.Struct({
    artifactName: ParentDesktopReleaseSupportPackagePreviewArtifactNameSchema,
    runStatus: ParentDesktopReleaseSupportCiRunStatusSchema,
    artifactState: ParentDesktopReleaseSupportArtifactStateSchema,
    packageReadinessClaim: ParentDesktopReleaseSupportReadinessClaimSchema,
    manualProofRequirement: ReleaseSupportRequirementSchema,
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

const ParentDesktopReleaseSupportRunbookSectionSchema = withParser(
  Schema.Literal(
    'rollback-triage',
    'rollback-failure-status',
    'diagnostics-redaction',
    'manual-platform-proof',
    'support-escalation-boundary'
  )
);

const ParentDesktopReleaseSupportUpdaterRollbackRowSchema = withParser(
  Schema.Struct({
    channel: ParentDesktopReleaseSupportUpdateChannelSchema,
    rollbackState: ParentDesktopReleaseSupportStateSchema,
    failureStatusState: ParentDesktopReleaseSupportStateSchema,
    manualRequiredState: ParentDesktopReleaseSupportStateSchema,
    proofRequirement: ReleaseSupportRequirementSchema,
  })
);

const ParentDesktopReleaseSupportRunbookStatusSchema = withParser(
  Schema.Struct({
    draftRunbookState: ParentDesktopReleaseSupportStateSchema,
    productionRunbookState: ParentDesktopReleaseSupportStateSchema,
    rollbackTriageState: ParentDesktopReleaseSupportStateSchema,
    requiredSections: Schema.Array(ParentDesktopReleaseSupportRunbookSectionSchema),
    proofReferences: Schema.Array(ReleaseSupportProofPathSchema),
    nonClaim: ReleaseSupportLabelSchema,
  })
);

const ParentDesktopReleaseSupportUpdaterRollbackRunbookProofSchema = withParser(
  Schema.Struct({
    proof: Schema.Literal('v8-updater-rollback-runbook-status'),
    updaterRows: Schema.Array(ParentDesktopReleaseSupportUpdaterRollbackRowSchema),
    runbookStatus: ParentDesktopReleaseSupportRunbookStatusSchema,
    claimBoundary: ReleaseSupportLabelSchema,
    manualRequiredGaps: Schema.Array(ReleaseSupportLabelSchema),
  })
);

export const ParentDesktopReleaseSupportReadinessGateSchema = withParser(
  Schema.Struct({
    gate: Schema.Literal('v8-production-release-support-readiness'),
    packagePreviewArtifacts: Schema.Array(ParentDesktopReleaseSupportPackagePreviewArtifactSchema),
    supportDiagnosticsState: ParentDesktopReleaseSupportStateSchema,
    supportRunbookState: ParentDesktopReleaseSupportStateSchema,
    updaterRollbackExecutionState: ParentDesktopReleaseSupportStateSchema,
    signingStoreProofState: ParentDesktopReleaseSupportStateSchema,
    productionPublishingState: ParentDesktopReleaseSupportStateSchema,
    claimBoundary: ReleaseSupportLabelSchema,
    proofReferences: Schema.Array(ReleaseSupportProofPathSchema),
    manualRequiredGaps: Schema.Array(ReleaseSupportLabelSchema),
  })
);

const ParentDesktopReleaseSupportReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentDesktopReleaseSupportSchemaVersionSchema,
  observerAuthority: Schema.Array(ParentDesktopReleaseSupportAuthorityOperationSchema),
  mobileBridgeBoundary: ParentDesktopReleaseSupportMobileBridgeBoundarySchema,
  packageRuntimeEvidence: ParentDesktopReleaseSupportPackageRuntimeEvidenceSchema,
  updateStates: Schema.Array(ParentDesktopReleaseSupportUpdateStateSchema),
  signingStoreStates: Schema.Array(ParentDesktopReleaseSupportSigningStoreStateSchema),
  platformCapabilityMatrix: Schema.Array(ParentDesktopReleaseSupportCapabilityRowSchema),
  ciArtifactProof: ParentDesktopReleaseSupportCiArtifactProofSchema,
  supportDiagnostics: ParentDesktopReleaseSupportDiagnosticsSchema,
  supportIncidentHandoff: ParentDesktopReleaseSupportIncidentHandoffSchema,
  manualRunbook: Schema.Array(ParentDesktopReleaseSupportManualRunbookEntrySchema),
  productionReadinessGate: ParentDesktopReleaseSupportReadinessGateSchema,
  updaterRollbackRunbookProof: ParentDesktopReleaseSupportUpdaterRollbackRunbookProofSchema,
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
export type ParentDesktopReleaseSupportPackageRuntimeEvidence = Infer<
  typeof ParentDesktopReleaseSupportPackageRuntimeEvidenceSchema
>;
export type ParentDesktopReleaseSupportCapabilityRow = Infer<typeof ParentDesktopReleaseSupportCapabilityRowSchema>;
export type ParentDesktopReleaseSupportCiArtifactProof = Infer<typeof ParentDesktopReleaseSupportCiArtifactProofSchema>;
export type ParentDesktopReleaseSupportDiagnostics = Infer<typeof ParentDesktopReleaseSupportDiagnosticsSchema>;
export type ParentDesktopReleaseSupportManualRunbookEntry = Infer<
  typeof ParentDesktopReleaseSupportManualRunbookEntrySchema
>;

