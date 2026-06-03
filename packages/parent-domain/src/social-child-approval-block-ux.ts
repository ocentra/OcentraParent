import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  SocialChildApprovalBlockActionSchema,
  SocialChildApprovalBlockReasonSchema,
  SocialChildApprovalBlockStateSchema,
  SocialChildApprovalBlockSurfaceIdSchema,
  type SocialChildApprovalBlockSurfaceKind,
  SocialChildApprovalBlockSurfaceKindSchema,
  SocialChildApprovalBlockUxSchemaVersionSchema,
} from './social-child-approval-block-ux-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const SocialChildApprovalBlockEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected child approval/block UX evidence refs')
);
const SocialChildApprovalBlockReasonsSchema = Schema.Array(SocialChildApprovalBlockReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected child approval/block UX reasons')
);

const SocialChildApprovalBlockSurfaceBaseSchema = Schema.Struct({
  surfaceId: SocialChildApprovalBlockSurfaceIdSchema,
  surfaceKind: SocialChildApprovalBlockSurfaceKindSchema,
  state: SocialChildApprovalBlockStateSchema,
  primaryAction: SocialChildApprovalBlockActionSchema,
  sourceEvidenceRefs: SocialChildApprovalBlockEvidenceRefsSchema,
  parentApprovalRequestRef: OptionalParentEvidenceRefSchema,
  gatePlanRef: OptionalParentEvidenceRefSchema,
  reasons: SocialChildApprovalBlockReasonsSchema,
  renderedChildUiClaimed: Schema.Boolean,
  notificationDeliveredClaimed: Schema.Boolean,
  browserNavigationBlockedClaimed: Schema.Boolean,
  blockPageRenderedClaimed: Schema.Boolean,
  timeLimitAppliedClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  connectorAuthorizationClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialChildApprovalBlockSurfaceCandidate = Infer<typeof SocialChildApprovalBlockSurfaceBaseSchema>;

export const SocialChildApprovalBlockSurfaceSchema = withParser(
  SocialChildApprovalBlockSurfaceBaseSchema.pipe(
    Schema.filter(
      (surface) =>
        socialChildApprovalBlockSurfaceIsHonest(surface) ||
        'Expected social child approval/block UX surface to stay state/action contract-only without rendered UI or enforcement claims'
    )
  )
);

export const SocialChildApprovalBlockClaimBoundariesSchema = withParser(
  Schema.Struct({
    renderedChildUi: Schema.Literal('not-claimed'),
    notificationDelivery: Schema.Literal('not-claimed'),
    browserNavigationBlock: Schema.Literal('not-claimed'),
    blockPageRender: Schema.Literal('not-claimed'),
    timeLimitApply: Schema.Literal('not-claimed'),
    finalPolicyDecision: Schema.Literal('not-claimed'),
    connectorAuthorization: Schema.Literal('not-claimed'),
    nativeAppControl: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
  })
);

const SocialChildApprovalBlockUxSnapshotBaseSchema = Schema.Struct({
  schemaVersion: SocialChildApprovalBlockUxSchemaVersionSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  generatedAt: ParentTimestampSchema,
  surfaces: Schema.Array(SocialChildApprovalBlockSurfaceSchema),
  claimBoundaries: SocialChildApprovalBlockClaimBoundariesSchema,
});

type SocialChildApprovalBlockUxSnapshotCandidate = Infer<typeof SocialChildApprovalBlockUxSnapshotBaseSchema>;

export const SocialChildApprovalBlockUxSnapshotSchema = withParser(
  SocialChildApprovalBlockUxSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        socialChildApprovalBlockUxSnapshotIsHonest(snapshot) ||
        'Expected child approval/block UX snapshot to include required child-facing states without rendered UI claims'
    )
  )
);

export const decodeSocialChildApprovalBlockUxSnapshot = Schema.decodeUnknownSync(
  SocialChildApprovalBlockUxSnapshotSchema
);

export type SocialChildApprovalBlockSurface = Infer<typeof SocialChildApprovalBlockSurfaceSchema>;
export type SocialChildApprovalBlockUxSnapshot = Infer<typeof SocialChildApprovalBlockUxSnapshotSchema>;

const RequiredSocialChildApprovalBlockSurfaces = [
  'approval-request-pending',
  'blocked-social-route-candidate',
  'warning-social-route-candidate',
  'manual-review-required',
  'time-limit-candidate',
  'native-app-unavailable',
] as const satisfies ReadonlyArray<SocialChildApprovalBlockSurfaceKind>;

type SocialChildApprovalBlockSurfaceValidator = (surface: SocialChildApprovalBlockSurfaceCandidate) => boolean;

const SocialChildApprovalBlockSurfaceValidators = {
  'approval-request-pending': approvalRequestPendingSurfaceIsHonest,
  'blocked-social-route-candidate': blockedSocialRouteCandidateSurfaceIsHonest,
  'warning-social-route-candidate': warningSocialRouteCandidateSurfaceIsHonest,
  'manual-review-required': manualReviewRequiredSurfaceIsHonest,
  'time-limit-candidate': socialTimeLimitCandidateSurfaceIsHonest,
  'native-app-unavailable': nativeAppUnavailableSurfaceIsHonest,
} satisfies Record<SocialChildApprovalBlockSurfaceKind, SocialChildApprovalBlockSurfaceValidator>;

function socialChildApprovalBlockUxSnapshotIsHonest(snapshot: SocialChildApprovalBlockUxSnapshotCandidate): boolean {
  const surfaceKinds = new Set(snapshot.surfaces.map((surface) => surface.surfaceKind));
  return (
    surfaceKinds.size === snapshot.surfaces.length &&
    RequiredSocialChildApprovalBlockSurfaces.every((surfaceKind) => surfaceKinds.has(surfaceKind))
  );
}

function socialChildApprovalBlockSurfaceIsHonest(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  if (socialChildApprovalBlockSurfaceClaimsRuntime(surface)) {
    return false;
  }
  return SocialChildApprovalBlockSurfaceValidators[surface.surfaceKind](surface);
}

function approvalRequestPendingSurfaceIsHonest(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  return (
    surface.state === 'waiting-parent' &&
    surface.primaryAction === 'wait-for-parent' &&
    surface.parentApprovalRequestRef !== null
  );
}

function blockedSocialRouteCandidateSurfaceIsHonest(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  return surface.state === 'blocked-contract-only' && surface.gatePlanRef !== null;
}

function warningSocialRouteCandidateSurfaceIsHonest(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  return surface.state === 'child-readable' && surface.primaryAction === 'acknowledge-warning';
}

function socialTimeLimitCandidateSurfaceIsHonest(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  return surface.state === 'child-readable' && surface.reasons.includes('time-limit-not-applied');
}

function nativeAppUnavailableSurfaceIsHonest(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  return surface.state === 'unavailable' && surface.reasons.includes('native-app-proof-unavailable');
}

function manualReviewRequiredSurfaceIsHonest(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  return surface.state === 'manual-required' && surface.primaryAction === 'manual-review';
}

function socialChildApprovalBlockSurfaceClaimsRuntime(surface: SocialChildApprovalBlockSurfaceCandidate): boolean {
  return (
    surface.renderedChildUiClaimed ||
    surface.notificationDeliveredClaimed ||
    surface.browserNavigationBlockedClaimed ||
    surface.blockPageRenderedClaimed ||
    surface.timeLimitAppliedClaimed ||
    surface.finalPolicyDecisionClaimed ||
    surface.connectorAuthorizationClaimed ||
    surface.nativeAppControlClaimed ||
    surface.enforcementClaimed
  );
}
