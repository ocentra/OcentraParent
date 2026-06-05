import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';

const TrackingAuthorityTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingAuthorityHardControlProofSchemaVersionSchema = withParser(
  Schema.Literal('tracking-authority-hard-control-proof')
);
export const TrackingAuthorityHardControlSurfaceSchema = withParser(
  Schema.Literal(
    'android-device-owner-location-control',
    'android-managed-profile-location-control',
    'ios-supervised-mdm-location-control',
    'macos-mdm-location-control',
    'windows-applocker-app-control-location-control'
  )
);
export const TrackingAuthorityHardControlPlatformSchema = withParser(
  Schema.Literal('android', 'ios', 'macos', 'windows')
);
export const TrackingAuthorityHardControlAuthorityStateSchema = withParser(Schema.Literal('authority_required'));
export const TrackingAuthorityHardControlArtifactStateSchema = withParser(
  Schema.Literal(
    'missing-device-owner-proof',
    'missing-managed-profile-proof',
    'missing-supervised-mdm-proof',
    'missing-mdm-proof',
    'missing-app-control-policy-proof'
  )
);

const TrackingAuthorityProofRefSchema = TrackingAuthorityTextSchema.pipe(
  Schema.brand('TrackingAuthorityHardControlProofRef')
);
const TrackingAuthorityRequirementSchema = TrackingAuthorityTextSchema.pipe(
  Schema.brand('TrackingAuthorityHardControlRequirement')
);
const TrackingAuthorityBoundarySchema = TrackingAuthorityTextSchema.pipe(
  Schema.brand('TrackingAuthorityHardControlBoundary')
);

export const TrackingAuthorityHardControlSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: TrackingAuthorityHardControlSurfaceSchema,
    platform: TrackingAuthorityHardControlPlatformSchema,
    authorityRequirement: TrackingAuthorityHardControlAuthorityStateSchema,
    requiredArtifact: TrackingAuthorityRequirementSchema,
    currentArtifactState: TrackingAuthorityHardControlArtifactStateSchema,
    proofRef: TrackingAuthorityProofRefSchema,
    authorityEnrolled: Schema.Literal(false),
    hardControlClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
    claimBoundary: TrackingAuthorityBoundarySchema,
  })
);

export const TrackingAuthorityHardControlSummarySchema = withParser(
  Schema.Struct({
    surfaceCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    authorityRequiredRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    authorityEnrolledRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    hardControlClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    childDeviceRuntimeClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    physicalDeviceClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    productClaimReadyRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  })
);

const TrackingAuthorityHardControlReadModelBaseSchema = Schema.Struct({
  schemaVersion: TrackingAuthorityHardControlProofSchemaVersionSchema,
  updatedAt: ParentTimestampSchema,
  surfaces: Schema.Array(TrackingAuthorityHardControlSurfaceProofSchema),
  summary: TrackingAuthorityHardControlSummarySchema,
});

type TrackingAuthorityHardControlReadModelCandidate = Infer<typeof TrackingAuthorityHardControlReadModelBaseSchema>;

export const TrackingAuthorityHardControlReadModelSchema = withParser(
  TrackingAuthorityHardControlReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingAuthorityHardControlReadModelIsHonest(readModel) ||
        'Expected tracking authority hard-control proof to keep every Device Owner, managed profile, supervised/MDM, and AppLocker/App Control row authority_required with no enrolled-device, hard-control, child-runtime, physical-device, or product-ready claim'
    )
  )
);

const RequiredSurfaces = [
  'android-device-owner-location-control',
  'android-managed-profile-location-control',
  'ios-supervised-mdm-location-control',
  'macos-mdm-location-control',
  'windows-applocker-app-control-location-control',
] as const satisfies ReadonlyArray<TrackingAuthorityHardControlSurface>;

const SurfaceExpectations = {
  'android-device-owner-location-control': {
    platform: 'android',
    currentArtifactState: 'missing-device-owner-proof',
    requiredArtifact: 'Android Device Owner enrollment evidence with location/control authority',
  },
  'android-managed-profile-location-control': {
    platform: 'android',
    currentArtifactState: 'missing-managed-profile-proof',
    requiredArtifact: 'Android managed-profile owner evidence with location/control authority',
  },
  'ios-supervised-mdm-location-control': {
    platform: 'ios',
    currentArtifactState: 'missing-supervised-mdm-proof',
    requiredArtifact: 'iOS supervised device or MDM entitlement evidence for location/control authority',
  },
  'macos-mdm-location-control': {
    platform: 'macos',
    currentArtifactState: 'missing-mdm-proof',
    requiredArtifact: 'macOS MDM profile or equivalent management evidence for location/control authority',
  },
  'windows-applocker-app-control-location-control': {
    platform: 'windows',
    currentArtifactState: 'missing-app-control-policy-proof',
    requiredArtifact: 'Windows AppLocker or App Control policy evidence with child-device authority',
  },
} as const satisfies Record<
  TrackingAuthorityHardControlSurface,
  {
    readonly platform: TrackingAuthorityHardControlPlatform;
    readonly currentArtifactState: TrackingAuthorityHardControlArtifactState;
    readonly requiredArtifact: string;
  }
>;

export function buildTrackingAuthorityHardControlReadModel(
  updatedAt: string = '2026-06-05T19:30:00.000Z'
): TrackingAuthorityHardControlReadModel {
  const surfaces = RequiredSurfaces.map((surface) => surfaceProofFor(surface));
  return TrackingAuthorityHardControlReadModelSchema.parse({
    schemaVersion: 'tracking-authority-hard-control-proof',
    updatedAt,
    surfaces,
    summary: summarize(surfaces),
  });
}

function surfaceProofFor(surface: TrackingAuthorityHardControlSurface): TrackingAuthorityHardControlSurfaceProof {
  const expected = SurfaceExpectations[surface];
  return TrackingAuthorityHardControlSurfaceProofSchema.parse({
    surface,
    platform: expected.platform,
    authorityRequirement: 'authority_required',
    requiredArtifact: expected.requiredArtifact,
    currentArtifactState: expected.currentArtifactState,
    proofRef: `authority-hard-control:${surface}`,
    authorityEnrolled: false,
    hardControlClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceClaimed: false,
    productClaimReady: false,
    claimBoundary:
      'CI proof records this row as authority_required until a real enrolled-device or managed-policy artifact is attached.',
  });
}

function summarize(
  surfaces: ReadonlyArray<TrackingAuthorityHardControlSurfaceProof>
): TrackingAuthorityHardControlSummary {
  return TrackingAuthorityHardControlSummarySchema.parse({
    surfaceCount: surfaces.length,
    authorityRequiredRows: surfaces.filter((surface) => surface.authorityRequirement === 'authority_required').length,
    authorityEnrolledRows: surfaces.filter((surface) => surface.authorityEnrolled).length,
    hardControlClaimedRows: surfaces.filter((surface) => surface.hardControlClaimed).length,
    childDeviceRuntimeClaimedRows: surfaces.filter((surface) => surface.childDeviceRuntimeClaimed).length,
    physicalDeviceClaimedRows: surfaces.filter((surface) => surface.physicalDeviceClaimed).length,
    productClaimReadyRows: surfaces.filter((surface) => surface.productClaimReady).length,
  });
}

function trackingAuthorityHardControlReadModelIsHonest(
  readModel: TrackingAuthorityHardControlReadModelCandidate
): boolean {
  const bySurface = new Map(readModel.surfaces.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === readModel.surfaces.length &&
    RequiredSurfaces.every((surface) => surfaceProofIsHonest(bySurface.get(surface), surface)) &&
    readModel.summary.surfaceCount === RequiredSurfaces.length &&
    readModel.summary.authorityRequiredRows === RequiredSurfaces.length &&
    readModel.summary.authorityEnrolledRows === 0 &&
    readModel.summary.hardControlClaimedRows === 0 &&
    readModel.summary.childDeviceRuntimeClaimedRows === 0 &&
    readModel.summary.physicalDeviceClaimedRows === 0 &&
    readModel.summary.productClaimReadyRows === 0
  );
}

function surfaceProofIsHonest(
  proof: TrackingAuthorityHardControlSurfaceProof | undefined,
  surface: TrackingAuthorityHardControlSurface
): boolean {
  const expected = SurfaceExpectations[surface];
  return Boolean(
    proof &&
    proof.platform === expected.platform &&
    proof.authorityRequirement === 'authority_required' &&
    proof.requiredArtifact === expected.requiredArtifact &&
    proof.currentArtifactState === expected.currentArtifactState &&
    proof.authorityEnrolled === false &&
    proof.hardControlClaimed === false &&
    proof.childDeviceRuntimeClaimed === false &&
    proof.physicalDeviceClaimed === false &&
    proof.productClaimReady === false
  );
}

export type TrackingAuthorityHardControlSurface = Infer<typeof TrackingAuthorityHardControlSurfaceSchema>;
export type TrackingAuthorityHardControlPlatform = Infer<typeof TrackingAuthorityHardControlPlatformSchema>;
export type TrackingAuthorityHardControlArtifactState = Infer<typeof TrackingAuthorityHardControlArtifactStateSchema>;
export type TrackingAuthorityHardControlSurfaceProof = Infer<typeof TrackingAuthorityHardControlSurfaceProofSchema>;
export type TrackingAuthorityHardControlSummary = Infer<typeof TrackingAuthorityHardControlSummarySchema>;
export type TrackingAuthorityHardControlReadModel = Infer<typeof TrackingAuthorityHardControlReadModelSchema>;
