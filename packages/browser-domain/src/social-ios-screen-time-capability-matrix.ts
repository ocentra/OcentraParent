import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from '@ocentra-parent/capability-domain/capabilities';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialIosScreenTimeCapabilityBoundarySchema,
  SocialIosScreenTimeCapabilityMatrixSchemaVersionSchema,
  SocialIosScreenTimeCapabilityReasonsSchema,
  type SocialIosScreenTimeCapabilitySurface,
  SocialIosScreenTimeCapabilitySurfaceSchema,
  SocialIosScreenTimeCapabilityStateSchema,
  SocialIosScreenTimePolicyScopeSchema,
  SocialIosScreenTimeProofRefsSchema,
  SocialIosScreenTimeProofStateSchema,
  SocialIosScreenTimeTargetKindSchema,
} from './social-ios-screen-time-capability-matrix-values';

const SocialIosScreenTimeCapabilityRowBaseSchema = Schema.Struct({
  surface: SocialIosScreenTimeCapabilitySurfaceSchema,
  targetKind: SocialIosScreenTimeTargetKindSchema,
  parentCapability: ParentControlCapabilityNameSchema,
  parentCapabilityStatus: ParentControlCapabilityStatusSchema,
  capabilityState: SocialIosScreenTimeCapabilityStateSchema,
  proofState: SocialIosScreenTimeProofStateSchema,
  policyScope: SocialIosScreenTimePolicyScopeSchema,
  proofRefs: SocialIosScreenTimeProofRefsSchema,
  reasons: SocialIosScreenTimeCapabilityReasonsSchema,
  routeLevelProofClaimed: Schema.Boolean,
  perVideoOrReelBlockingClaimed: Schema.Boolean,
  messageContentClaimed: Schema.Boolean,
  accountIdentityClaimed: Schema.Boolean,
  rawApplicationIdentityClaimed: Schema.Boolean,
  screenContentCaptureClaimed: Schema.Boolean,
  deviceActivityRuntimeClaimed: Schema.Boolean,
  managedSettingsRuntimeClaimed: Schema.Boolean,
  entitlementApprovalClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
  uiDeliveredClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialIosScreenTimeCapabilityRowCandidate = Infer<typeof SocialIosScreenTimeCapabilityRowBaseSchema>;

export const SocialIosScreenTimeCapabilityRowSchema = withParser(
  SocialIosScreenTimeCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        socialIosScreenTimeCapabilityRowIsHonest(row) ||
        'Expected iOS Screen Time social capability row to stay entitlement-required, token-selection-required, manual-required, or unavailable'
    )
  )
);

export const SocialIosScreenTimeCapabilityClaimBoundariesSchema = withParser(
  Schema.Struct({
    familyControlsAuthorization: Schema.Literal('not-claimed'),
    rawApplicationIdentity: Schema.Literal('not-claimed'),
    nativeRouteProof: Schema.Literal('not-claimed'),
    perVideoOrReelBlocking: Schema.Literal('not-claimed'),
    messageContent: Schema.Literal('not-claimed'),
    accountIdentity: Schema.Literal('not-claimed'),
    screenContentCapture: Schema.Literal('not-claimed'),
    runtimeAdapter: Schema.Literal('not-claimed'),
    connectorAuthorization: Schema.Literal('not-claimed'),
    uiDelivery: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
    reviewerSummary: SocialIosScreenTimeCapabilityBoundarySchema,
  })
);

const SocialIosScreenTimeCapabilityMatrixBaseSchema = Schema.Struct({
  schemaVersion: SocialIosScreenTimeCapabilityMatrixSchemaVersionSchema,
  generatedAt: ParentTimestampSchema,
  proofRefs: SocialIosScreenTimeProofRefsSchema,
  rows: Schema.Array(SocialIosScreenTimeCapabilityRowSchema),
  claimBoundaries: SocialIosScreenTimeCapabilityClaimBoundariesSchema,
});

type SocialIosScreenTimeCapabilityMatrixCandidate = Infer<typeof SocialIosScreenTimeCapabilityMatrixBaseSchema>;

export const SocialIosScreenTimeCapabilityMatrixSchema = withParser(
  SocialIosScreenTimeCapabilityMatrixBaseSchema.pipe(
    Schema.filter(
      (matrix) =>
        socialIosScreenTimeCapabilityMatrixIsHonest(matrix) ||
        'Expected iOS Screen Time social capability matrix to include required token and shield surfaces without entitlement, route, content, UI, adapter, connector, or enforcement claims'
    )
  )
);

export const decodeSocialIosScreenTimeCapabilityMatrix = Schema.decodeUnknownSync(
  SocialIosScreenTimeCapabilityMatrixSchema
);

export type SocialIosScreenTimeCapabilityRow = Infer<typeof SocialIosScreenTimeCapabilityRowSchema>;
export type SocialIosScreenTimeCapabilityMatrix = Infer<typeof SocialIosScreenTimeCapabilityMatrixSchema>;

const RequiredIosSocialScreenTimeSurfaces = [
  'ios-family-controls-authorization',
  'ios-application-token-selection',
  'ios-web-domain-token-selection',
  'ios-device-activity-monitor',
  'ios-managed-settings-application-shield',
  'ios-managed-settings-web-domain-shield',
] as const satisfies ReadonlyArray<SocialIosScreenTimeCapabilitySurface>;

type SocialIosScreenTimeCapabilityRowValidator = (row: SocialIosScreenTimeCapabilityRowCandidate) => boolean;

const SocialIosScreenTimeCapabilityRowValidators = {
  'ios-family-controls-authorization': iosFamilyControlsCapabilityRowIsHonest,
  'ios-application-token-selection': iosApplicationTokenSelectionRowIsHonest,
  'ios-web-domain-token-selection': iosWebDomainTokenSelectionRowIsHonest,
  'ios-device-activity-monitor': iosDeviceActivityMonitorRowIsHonest,
  'ios-managed-settings-application-shield': iosManagedSettingsShieldRowIsHonest,
  'ios-managed-settings-web-domain-shield': iosManagedSettingsShieldRowIsHonest,
} satisfies Record<SocialIosScreenTimeCapabilitySurface, SocialIosScreenTimeCapabilityRowValidator>;

function socialIosScreenTimeCapabilityMatrixIsHonest(matrix: SocialIosScreenTimeCapabilityMatrixCandidate): boolean {
  const surfaces = new Set(matrix.rows.map((row) => row.surface));
  return (
    surfaces.size === matrix.rows.length &&
    RequiredIosSocialScreenTimeSurfaces.every((surface) => surfaces.has(surface))
  );
}

function socialIosScreenTimeCapabilityRowIsHonest(row: SocialIosScreenTimeCapabilityRowCandidate): boolean {
  if (socialIosScreenTimeCapabilityRowClaimsRuntime(row)) {
    return false;
  }
  return SocialIosScreenTimeCapabilityRowValidators[row.surface](row);
}

function iosFamilyControlsCapabilityRowIsHonest(row: SocialIosScreenTimeCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'family-controls-entitlement' &&
    row.capabilityState === 'entitlement-required' &&
    row.proofState === 'apple-entitlement-required' &&
    row.policyScope === 'manual-review-only'
  );
}

function iosApplicationTokenSelectionRowIsHonest(row: SocialIosScreenTimeCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'family-controls-entitlement' &&
    row.capabilityState === 'token-selection-required' &&
    row.policyScope === 'app-token-level' &&
    rowHasReasons(row, ['opaque-token-required', 'raw-app-identity-unavailable'])
  );
}

function iosWebDomainTokenSelectionRowIsHonest(row: SocialIosScreenTimeCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'family-controls-entitlement' &&
    row.capabilityState === 'token-selection-required' &&
    row.policyScope === 'web-domain-token-level' &&
    rowHasReasons(row, ['web-domain-token-limited'])
  );
}

function iosDeviceActivityMonitorRowIsHonest(row: SocialIosScreenTimeCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'device-activity' &&
    row.capabilityState === 'manual-device-proof-required' &&
    row.proofState === 'apple-entitlement-required' &&
    rowHasReasons(row, ['device-activity-entitlement-required'])
  );
}

function iosManagedSettingsShieldRowIsHonest(row: SocialIosScreenTimeCapabilityRowCandidate): boolean {
  return (
    row.parentCapability === 'screen-time-api' &&
    row.capabilityState === 'manual-device-proof-required' &&
    row.proofState === 'apple-entitlement-required' &&
    rowHasReasons(row, ['managed-settings-entitlement-required', 'shield-state-device-proof-required'])
  );
}

function rowHasReasons(
  row: SocialIosScreenTimeCapabilityRowCandidate,
  reasons: ReadonlyArray<SocialIosScreenTimeCapabilityRowCandidate['reasons'][number]>
): boolean {
  return reasons.every((reason) => row.reasons.includes(reason));
}

function socialIosScreenTimeCapabilityRowClaimsRuntime(row: SocialIosScreenTimeCapabilityRowCandidate): boolean {
  return (
    row.routeLevelProofClaimed ||
    row.perVideoOrReelBlockingClaimed ||
    row.messageContentClaimed ||
    row.accountIdentityClaimed ||
    row.rawApplicationIdentityClaimed ||
    row.screenContentCaptureClaimed ||
    row.deviceActivityRuntimeClaimed ||
    row.managedSettingsRuntimeClaimed ||
    row.entitlementApprovalClaimed ||
    row.platformConnectorClaimed ||
    row.uiDeliveredClaimed ||
    row.enforcementClaimed
  );
}
