import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from '@ocentra-parent/capability-domain/capabilities';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialAndroidNativeAppCapabilityBoundarySchema,
  SocialAndroidNativeAppCapabilityReasonsSchema,
  SocialAndroidNativeAppCapabilityStateSchema,
  type SocialAndroidNativeAppCapabilitySurface,
  SocialAndroidNativeAppCapabilitySurfaceSchema,
  SocialAndroidNativeAppPolicyScopeSchema,
  SocialAndroidNativeAppProofRefsSchema,
  SocialAndroidNativeAppProofStateSchema,
  SocialAndroidNativeAppTargetKindSchema,
  SocialAndroidNativeAppCapabilityMatrixSchemaVersionSchema,
} from './social-android-native-app-capability-matrix-values';

const SocialAndroidNativeAppCapabilityRowBaseSchema = Schema.Struct({
  surface: SocialAndroidNativeAppCapabilitySurfaceSchema,
  targetKind: SocialAndroidNativeAppTargetKindSchema,
  parentCapability: ParentControlCapabilityNameSchema,
  parentCapabilityStatus: ParentControlCapabilityStatusSchema,
  capabilityState: SocialAndroidNativeAppCapabilityStateSchema,
  proofState: SocialAndroidNativeAppProofStateSchema,
  policyScope: SocialAndroidNativeAppPolicyScopeSchema,
  proofRefs: SocialAndroidNativeAppProofRefsSchema,
  reasons: SocialAndroidNativeAppCapabilityReasonsSchema,
  routeLevelProofClaimed: Schema.Boolean,
  perVideoOrReelBlockingClaimed: Schema.Boolean,
  messageContentClaimed: Schema.Boolean,
  accountIdentityClaimed: Schema.Boolean,
  accessibilityContentCaptureClaimed: Schema.Boolean,
  deviceOwnerEnrollmentClaimed: Schema.Boolean,
  vpnContentInspectionClaimed: Schema.Boolean,
  nativeRuntimeAdapterClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
  uiDeliveredClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialAndroidNativeAppCapabilityRowCandidate = Infer<typeof SocialAndroidNativeAppCapabilityRowBaseSchema>;

export const SocialAndroidNativeAppCapabilityRowSchema = withParser(
  SocialAndroidNativeAppCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialAndroidNativeAppCapabilityRowIsHonest(row) ||
        'Expected Android social native app capability row to stay app-level, manual-required, or unavailable'
    )
  )
);

export const SocialAndroidNativeAppCapabilityClaimBoundariesSchema = withParser(
  Schema.Struct({
    nativeRouteProof: Schema.Literal('not-claimed'),
    perVideoOrReelBlocking: Schema.Literal('not-claimed'),
    messageContent: Schema.Literal('not-claimed'),
    accountIdentity: Schema.Literal('not-claimed'),
    accessibilityContentCapture: Schema.Literal('not-claimed'),
    deviceOwnerEnrollment: Schema.Literal('not-claimed'),
    runtimeAdapter: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
    reviewerSummary: SocialAndroidNativeAppCapabilityBoundarySchema,
  })
);

const SocialAndroidNativeAppCapabilityMatrixBaseSchema = Schema.Struct({
  schemaVersion: SocialAndroidNativeAppCapabilityMatrixSchemaVersionSchema,
  generatedAt: ParentTimestampSchema,
  proofRefs: SocialAndroidNativeAppProofRefsSchema,
  rows: Schema.Array(SocialAndroidNativeAppCapabilityRowSchema),
  claimBoundaries: SocialAndroidNativeAppCapabilityClaimBoundariesSchema,
});

type SocialAndroidNativeAppCapabilityMatrixCandidate = Infer<typeof SocialAndroidNativeAppCapabilityMatrixBaseSchema>;

export const SocialAndroidNativeAppCapabilityMatrixSchema = withParser(
  SocialAndroidNativeAppCapabilityMatrixBaseSchema.pipe(
    Schema.filter(
      (matrix) =>
        socialAndroidNativeAppCapabilityMatrixIsHonest(matrix) ||
        'Expected Android social native app capability matrix to include all required surfaces without native route or enforcement claims'
    )
  )
);

export const decodeSocialAndroidNativeAppCapabilityMatrix = Schema.decodeUnknownSync(
  SocialAndroidNativeAppCapabilityMatrixSchema
);

export type SocialAndroidNativeAppCapabilityRow = Infer<typeof SocialAndroidNativeAppCapabilityRowSchema>;
export type SocialAndroidNativeAppCapabilityMatrix = Infer<typeof SocialAndroidNativeAppCapabilityMatrixSchema>;

const RequiredAndroidSocialSurfaces = [
  'android-package-visibility',
  'android-usage-stats-foreground',
  'android-accessibility-route-hints',
  'android-vpn-domain-hints',
  'android-device-owner-app-control',
  'android-managed-profile-config',
] as const satisfies ReadonlyArray<SocialAndroidNativeAppCapabilitySurface>;

function socialAndroidNativeAppCapabilityMatrixIsHonest(matrix: SocialAndroidNativeAppCapabilityMatrixCandidate) {
  return RequiredAndroidSocialSurfaces.every((surface) => matrix.rows.some((row) => row.surface === surface));
}

function socialAndroidNativeAppCapabilityRowIsHonest(row: SocialAndroidNativeAppCapabilityRowCandidate): boolean {
  if (socialAndroidNativeAppCapabilityRowClaimsRuntime(row)) {
    return false;
  }

  if (row.surface === 'android-package-visibility') {
    return androidPackageVisibilityRowIsHonest(row);
  }
  if (row.surface === 'android-usage-stats-foreground') {
    return androidUsageStatsForegroundRowIsHonest(row);
  }
  if (row.surface === 'android-accessibility-route-hints') {
    return androidAccessibilityRouteHintsRowIsHonest(row);
  }
  if (row.surface === 'android-vpn-domain-hints') {
    return androidVpnDomainHintsRowIsHonest(row);
  }
  return androidManualDeviceProofRowIsHonest(row);
}

function androidPackageVisibilityRowIsHonest(row: SocialAndroidNativeAppCapabilityRowCandidate): boolean {
  return (
    (row.capabilityState === 'app-level-capable-with-proof' &&
      row.proofState === 'existing-parent-domain-proof-ref' &&
      row.policyScope === 'app-level-only') ||
    (row.capabilityState === 'manual-required' &&
      row.proofState === 'manual-device-proof-required' &&
      row.policyScope === 'manual-review-only')
  );
}

function androidUsageStatsForegroundRowIsHonest(row: SocialAndroidNativeAppCapabilityRowCandidate): boolean {
  return row.capabilityState === 'permission-required' && row.proofState === 'permission-grant-required';
}

function androidAccessibilityRouteHintsRowIsHonest(row: SocialAndroidNativeAppCapabilityRowCandidate): boolean {
  return row.capabilityState !== 'app-level-capable-with-proof' && row.reasons.includes('route-level-unavailable');
}

function androidVpnDomainHintsRowIsHonest(row: SocialAndroidNativeAppCapabilityRowCandidate): boolean {
  return row.policyScope === 'domain-level-only' && row.reasons.includes('vpn-domain-only');
}

function androidManualDeviceProofRowIsHonest(row: SocialAndroidNativeAppCapabilityRowCandidate): boolean {
  return (
    row.capabilityState === 'manual-required' &&
    row.proofState === 'manual-device-proof-required' &&
    row.policyScope !== 'app-level-only'
  );
}

function socialAndroidNativeAppCapabilityRowClaimsRuntime(row: SocialAndroidNativeAppCapabilityRowCandidate): boolean {
  return (
    row.routeLevelProofClaimed ||
    row.perVideoOrReelBlockingClaimed ||
    row.messageContentClaimed ||
    row.accountIdentityClaimed ||
    row.accessibilityContentCaptureClaimed ||
    row.deviceOwnerEnrollmentClaimed ||
    row.vpnContentInspectionClaimed ||
    row.nativeRuntimeAdapterClaimed ||
    row.platformConnectorClaimed ||
    row.uiDeliveredClaimed ||
    row.enforcementClaimed
  );
}
