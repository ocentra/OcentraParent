import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  BrowserGameChildCheckingBlockActionSchema,
  BrowserGameChildCheckingBlockReasonSchema,
  BrowserGameChildCheckingBlockStateSchema,
  BrowserGameChildCheckingBlockSurfaceIdSchema,
  type BrowserGameChildCheckingBlockSurfaceKind,
  BrowserGameChildCheckingBlockSurfaceKindSchema,
  BrowserGameChildCheckingBlockTextToken,
  BrowserGameChildCheckingBlockTextTokenSchema,
  BrowserGameChildCheckingBlockUxSchemaVersionSchema,
} from './browser-game-child-checking-block-ux-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const BrowserGameChildEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game child UX evidence refs')
);
const BrowserGameChildReasonsSchema = Schema.Array(BrowserGameChildCheckingBlockReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game child UX reasons')
);

const BrowserGameChildCheckingBlockSurfaceBaseSchema = Schema.Struct({
  surfaceId: BrowserGameChildCheckingBlockSurfaceIdSchema,
  surfaceKind: BrowserGameChildCheckingBlockSurfaceKindSchema,
  state: BrowserGameChildCheckingBlockStateSchema,
  primaryAction: BrowserGameChildCheckingBlockActionSchema,
  primaryTextToken: BrowserGameChildCheckingBlockTextTokenSchema,
  sourceEvidenceRefs: BrowserGameChildEvidenceRefsSchema,
  gameEvidenceRef: OptionalParentEvidenceRefSchema,
  analysisRef: OptionalParentEvidenceRefSchema,
  policyCandidateRef: OptionalParentEvidenceRefSchema,
  parentApprovalRequestRef: OptionalParentEvidenceRefSchema,
  adapterProofRef: OptionalParentEvidenceRefSchema,
  reasons: BrowserGameChildReasonsSchema,
  rawChildCopyClaimed: Schema.Boolean,
  renderedChildUiClaimed: Schema.Boolean,
  notificationDeliveredClaimed: Schema.Boolean,
  browserNavigationBlockedClaimed: Schema.Boolean,
  blockPageRenderedClaimed: Schema.Boolean,
  timeLimitAppliedClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameChildCheckingBlockSurfaceCandidate = Infer<typeof BrowserGameChildCheckingBlockSurfaceBaseSchema>;

export const BrowserGameChildCheckingBlockSurfaceSchema = withParser(
  BrowserGameChildCheckingBlockSurfaceBaseSchema.pipe(
    Schema.filter(
      (surface) =>
        browserGameChildCheckingBlockSurfaceIsHonest(surface) ||
        'Expected browser-game child UX surface to stay contract-only without rendered UI, runtime block, native game, cloud-frame, or enforcement claims'
    )
  )
);

export const BrowserGameChildCheckingBlockClaimBoundariesSchema = withParser(
  Schema.Struct({
    rawChildCopy: Schema.Literal('not-claimed'),
    renderedChildUi: Schema.Literal('not-claimed'),
    notificationDelivery: Schema.Literal('not-claimed'),
    browserNavigationBlock: Schema.Literal('not-claimed'),
    blockPageRender: Schema.Literal('not-claimed'),
    timeLimitApply: Schema.Literal('not-claimed'),
    finalPolicyDecision: Schema.Literal('not-claimed'),
    cloudFrameAnalysis: Schema.Literal('not-claimed'),
    nativeGameControl: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
  })
);

const BrowserGameChildCheckingBlockUxSnapshotBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameChildCheckingBlockUxSchemaVersionSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  generatedAt: ParentTimestampSchema,
  surfaces: Schema.Array(BrowserGameChildCheckingBlockSurfaceSchema),
  claimBoundaries: BrowserGameChildCheckingBlockClaimBoundariesSchema,
});

type BrowserGameChildCheckingBlockUxSnapshotCandidate = Infer<typeof BrowserGameChildCheckingBlockUxSnapshotBaseSchema>;

export const BrowserGameChildCheckingBlockUxSnapshotSchema = withParser(
  BrowserGameChildCheckingBlockUxSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        browserGameChildCheckingBlockUxSnapshotIsHonest(snapshot) ||
        'Expected browser-game child UX snapshot to include required game checking/block states without UI or enforcement claims'
    )
  )
);

export const decodeBrowserGameChildCheckingBlockUxSnapshot = Schema.decodeUnknownSync(
  BrowserGameChildCheckingBlockUxSnapshotSchema
);

export type BrowserGameChildCheckingBlockSurface = Infer<typeof BrowserGameChildCheckingBlockSurfaceSchema>;
export type BrowserGameChildCheckingBlockUxSnapshot = Infer<typeof BrowserGameChildCheckingBlockUxSnapshotSchema>;

const RequiredBrowserGameChildCheckingBlockSurfaces = [
  'checking-unknown-game',
  'approval-required-game',
  'blocked-game-candidate',
  'educational-game-allowed',
  'game-time-limit-candidate',
  'cloud-gaming-manual-required',
  'native-game-control-unavailable',
] as const satisfies ReadonlyArray<BrowserGameChildCheckingBlockSurfaceKind>;

function browserGameChildCheckingBlockUxSnapshotIsHonest(
  snapshot: BrowserGameChildCheckingBlockUxSnapshotCandidate
): boolean {
  const surfaceKinds = new Set(snapshot.surfaces.map((surface) => surface.surfaceKind));
  return (
    surfaceKinds.size === snapshot.surfaces.length &&
    RequiredBrowserGameChildCheckingBlockSurfaces.every((surfaceKind) => surfaceKinds.has(surfaceKind))
  );
}

function browserGameChildCheckingBlockSurfaceIsHonest(surface: BrowserGameChildCheckingBlockSurfaceCandidate): boolean {
  if (browserGameChildCheckingBlockSurfaceClaimsRuntime(surface)) {
    return false;
  }
  if (!textTokenMatchesSurfaceKind(surface)) {
    return false;
  }
  if (surface.surfaceKind === 'checking-unknown-game') {
    return (
      surface.state === 'checking-contract-only' &&
      surface.primaryAction === 'wait-for-classification' &&
      surface.analysisRef !== null
    );
  }
  if (surface.surfaceKind === 'approval-required-game') {
    return (
      surface.state === 'waiting-parent' &&
      surface.primaryAction === 'wait-for-parent' &&
      surface.parentApprovalRequestRef !== null
    );
  }
  if (surface.surfaceKind === 'blocked-game-candidate') {
    return (
      surface.state === 'blocked-contract-only' &&
      surface.primaryAction === 'open-safe-back' &&
      surface.policyCandidateRef !== null
    );
  }
  if (surface.surfaceKind === 'educational-game-allowed') {
    return (
      surface.state === 'child-readable' &&
      surface.primaryAction === 'acknowledge' &&
      surface.reasons.includes('educational-game-allowed-contract')
    );
  }
  if (surface.surfaceKind === 'game-time-limit-candidate') {
    return (
      surface.state === 'child-readable' &&
      surface.primaryAction === 'acknowledge' &&
      surface.reasons.includes('time-limit-not-applied')
    );
  }
  if (surface.surfaceKind === 'cloud-gaming-manual-required') {
    return surface.state === 'manual-required' && surface.reasons.includes('cloud-gaming-proof-manual-required');
  }
  return surface.state === 'unavailable' && surface.reasons.includes('native-game-proof-unavailable');
}

function browserGameChildCheckingBlockSurfaceClaimsRuntime(
  surface: BrowserGameChildCheckingBlockSurfaceCandidate
): boolean {
  return (
    surface.rawChildCopyClaimed ||
    surface.renderedChildUiClaimed ||
    surface.notificationDeliveredClaimed ||
    surface.browserNavigationBlockedClaimed ||
    surface.blockPageRenderedClaimed ||
    surface.timeLimitAppliedClaimed ||
    surface.finalPolicyDecisionClaimed ||
    surface.cloudFrameAnalysisClaimed ||
    surface.nativeGameControlClaimed ||
    surface.enforcementClaimed
  );
}

function textTokenMatchesSurfaceKind(surface: BrowserGameChildCheckingBlockSurfaceCandidate): boolean {
  switch (surface.surfaceKind) {
    case 'checking-unknown-game':
      return surface.primaryTextToken === BrowserGameChildCheckingBlockTextToken.Checking;
    case 'approval-required-game':
      return surface.primaryTextToken === BrowserGameChildCheckingBlockTextToken.Approval;
    case 'blocked-game-candidate':
      return surface.primaryTextToken === BrowserGameChildCheckingBlockTextToken.Blocked;
    case 'educational-game-allowed':
      return surface.primaryTextToken === BrowserGameChildCheckingBlockTextToken.EducationalAllowed;
    case 'game-time-limit-candidate':
      return surface.primaryTextToken === BrowserGameChildCheckingBlockTextToken.TimeLimited;
    case 'cloud-gaming-manual-required':
      return surface.primaryTextToken === BrowserGameChildCheckingBlockTextToken.Manual;
    case 'native-game-control-unavailable':
      return surface.primaryTextToken === BrowserGameChildCheckingBlockTextToken.Unavailable;
  }
}
