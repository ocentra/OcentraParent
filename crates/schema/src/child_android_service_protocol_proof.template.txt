/* generated from crates/schema/src/child_android_service_protocol_proof_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildAndroidServiceProtocolProofSchemaVersionSchema = withParser(
  Schema.Literal('child-android-service-protocol-capability-proof')
);
export const ChildAndroidServiceSurfaceNameSchema = withParser(
  Schema.Literal(
    'foreground-service-status',
    'storage-protocol-bridge',
    'status-export-surface',
    'usage-stats-capability-label',
    'accessibility-capability-label',
    'vpn-dns-capability-label',
    'device-owner-capability-label',
    'managed-profile-capability-label'
  )
);
export const ChildAndroidServiceCapabilityLabelSchema = withParser(
  Schema.Literal('implemented', 'scaffold-only', 'manual-required', 'permission-required', 'unavailable', 'blocked')
);
export const ChildAndroidServiceProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'package-local-scaffold', 'permission-required', 'not-implemented', 'blocked')
);
export const ChildAndroidServiceRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'android-native-wrapper',
    'android-foreground-service',
    'android-os-permission',
    'android-accessibility-service',
    'android-vpn-service',
    'android-policy-provider',
    'agent-protocol',
    'status-export-bundle',
    'manual-device-proof'
  )
);
export const ChildAndroidServiceRuntimeStateSchema = withParser(
  Schema.Literal('declared-started-by-package', 'package-local-scaffold', 'not-implemented')
);
export const ChildAndroidServiceStatusExportStateSchema = withParser(
  Schema.Literal('package-local-bundle', 'not-external-transport')
);
export const ChildAndroidServiceProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.android.service.status.get',
    'child.android.service.capability.labels.get',
    'child.android.service.status.export.get',
    'child.android.service.protocol.proof.get'
  )
);
export const ChildAndroidServiceProtocolEventSchema = withParser(
  Schema.Literal(
    'child.android.service.status.reported',
    'child.android.service.capability.labels.reported',
    'child.android.service.status.export.reported',
    'child.android.service.protocol.proof.reported'
  )
);
export const ChildAndroidServiceStatusExportFieldSchema = withParser(
  Schema.Literal(
    'schemaVersion',
    'packageId',
    'nativeBridgeClass',
    'foregroundServiceStatus',
    'storageBridgeState',
    'capabilityLabels',
    'commands',
    'events'
  )
);

const ChildAndroidServicePackageIdSchema = brandedNonEmptyStringSchema('ChildAndroidServicePackageId');
const ChildAndroidServiceClassNameSchema = brandedNonEmptyStringSchema('ChildAndroidServiceClassName');
const ChildAndroidServiceRequirementSchema = brandedNonEmptyStringSchema('ChildAndroidServiceRequirement');
const ChildAndroidServiceBoundarySchema = brandedNonEmptyStringSchema('ChildAndroidServiceBoundary');
const ChildAndroidServiceIdentifierSchema = brandedNonEmptyStringSchema('ChildAndroidServiceIdentifier');

export const ChildAndroidForegroundServiceProofSchema = withParser(
  Schema.Struct({
    packageId: ChildAndroidServicePackageIdSchema,
    serviceClass: ChildAndroidServiceClassNameSchema,
    notificationChannelId: ChildAndroidServiceIdentifierSchema,
    notificationId: Schema.Number,
    foregroundServiceType: ChildAndroidServiceIdentifierSchema,
    serviceStatus: ChildAndroidServiceRuntimeStateSchema,
    runtimeOwner: ChildAndroidServiceRuntimeOwnerSchema,
    proofRequirement: ChildAndroidServiceRequirementSchema,
    claimBoundary: ChildAndroidServiceBoundarySchema,
  })
);

export const ChildAndroidServiceProtocolBridgeProofSchema = withParser(
  Schema.Struct({
    packageId: ChildAndroidServicePackageIdSchema,
    nativeBridgeClass: ChildAndroidServiceClassNameSchema,
    storageBridgeClass: ChildAndroidServiceClassNameSchema,
    bridgeState: ChildAndroidServiceRuntimeStateSchema,
    storageBridgeState: ChildAndroidServiceRuntimeStateSchema,
    externalTransportState: ChildAndroidServiceRuntimeStateSchema,
    commands: Schema.Array(ChildAndroidServiceProtocolCommandSchema),
    events: Schema.Array(ChildAndroidServiceProtocolEventSchema),
    runtimeOwner: ChildAndroidServiceRuntimeOwnerSchema,
    proofRequirement: ChildAndroidServiceRequirementSchema,
    claimBoundary: ChildAndroidServiceBoundarySchema,
  })
);

export const ChildAndroidServiceStatusExportProofSchema = withParser(
  Schema.Struct({
    exportState: ChildAndroidServiceStatusExportStateSchema,
    fields: Schema.Array(ChildAndroidServiceStatusExportFieldSchema),
    runtimeOwner: ChildAndroidServiceRuntimeOwnerSchema,
    proofRequirement: ChildAndroidServiceRequirementSchema,
    claimBoundary: ChildAndroidServiceBoundarySchema,
  })
);

export const ChildAndroidServiceSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: ChildAndroidServiceSurfaceNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    capabilityLabel: ChildAndroidServiceCapabilityLabelSchema,
    proofState: ChildAndroidServiceProofStateSchema,
    runtimeOwner: ChildAndroidServiceRuntimeOwnerSchema,
    proofRequirement: ChildAndroidServiceRequirementSchema,
    claimBoundary: ChildAndroidServiceBoundarySchema,
  })
);

export const ChildAndroidServiceClaimBoundariesSchema = withParser(
  Schema.Struct({
    foregroundService: ChildAndroidServiceBoundarySchema,
    storageProtocolBridge: ChildAndroidServiceBoundarySchema,
    statusExport: ChildAndroidServiceBoundarySchema,
    usageStats: ChildAndroidServiceBoundarySchema,
    accessibility: ChildAndroidServiceBoundarySchema,
    vpnDns: ChildAndroidServiceBoundarySchema,
    deviceOwner: ChildAndroidServiceBoundarySchema,
    managedProfile: ChildAndroidServiceBoundarySchema,
    externalTransport: ChildAndroidServiceBoundarySchema,
    childAndroidServiceRuntime: ChildAndroidServiceBoundarySchema,
  })
);

const ChildAndroidServiceProtocolReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildAndroidServiceProtocolProofSchemaVersionSchema,
  foregroundService: ChildAndroidForegroundServiceProofSchema,
  protocolBridgeProof: ChildAndroidServiceProtocolBridgeProofSchema,
  statusExportProof: ChildAndroidServiceStatusExportProofSchema,
  serviceSurfaces: Schema.Array(ChildAndroidServiceSurfaceProofSchema),
  claimBoundaries: ChildAndroidServiceClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildAndroidServiceProtocolReadModelCandidate = Infer<typeof ChildAndroidServiceProtocolReadModelBaseSchema>;

export const ChildAndroidServiceProtocolReadModelSchema = withParser(
  ChildAndroidServiceProtocolReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childAndroidServiceProtocolReadModelIsHonest(readModel) ||
        'Expected Child Android service/protocol proof to keep the Android service package-local, storage bridge scaffold-only, status export as a local Bundle, and UsageStats/accessibility/VPN-DNS/device-owner/managed-profile as permission-required, unavailable, or blocked without device proof'
    )
  )
);

const RequiredSurfaces = [
  'foreground-service-status',
  'storage-protocol-bridge',
  'status-export-surface',
  'usage-stats-capability-label',
  'accessibility-capability-label',
  'vpn-dns-capability-label',
  'device-owner-capability-label',
  'managed-profile-capability-label',
] as const satisfies ReadonlyArray<ChildAndroidServiceSurfaceName>;

const RequiredCommands = [
  'child.android.service.status.get',
  'child.android.service.capability.labels.get',
  'child.android.service.status.export.get',
  'child.android.service.protocol.proof.get',
] as const satisfies ReadonlyArray<ChildAndroidServiceProtocolCommand>;

const RequiredEvents = [
  'child.android.service.status.reported',
  'child.android.service.capability.labels.reported',
  'child.android.service.status.export.reported',
  'child.android.service.protocol.proof.reported',
] as const satisfies ReadonlyArray<ChildAndroidServiceProtocolEvent>;

const RequiredExportFields = [
  'schemaVersion',
  'packageId',
  'nativeBridgeClass',
  'foregroundServiceStatus',
  'storageBridgeState',
  'capabilityLabels',
  'commands',
  'events',
] as const satisfies ReadonlyArray<ChildAndroidServiceStatusExportField>;

function childAndroidServiceProtocolReadModelIsHonest(
  readModel: ChildAndroidServiceProtocolReadModelCandidate
): boolean {
  return (
    foregroundServiceProofIsHonest(readModel.foregroundService) &&
    protocolBridgeProofIsHonest(readModel.protocolBridgeProof) &&
    statusExportProofIsHonest(readModel.statusExportProof) &&
    serviceSurfaceProofsAreHonest(readModel.serviceSurfaces)
  );
}

function foregroundServiceProofIsHonest(proof: ChildAndroidForegroundServiceProof): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.serviceClass === 'ca.ocentra.parent.agent/.OcentraParentAgentService' &&
    proof.notificationChannelId === 'ocentra_parent_agent' &&
    proof.notificationId === 4477 &&
    proof.foregroundServiceType === 'dataSync' &&
    proof.serviceStatus === 'declared-started-by-package' &&
    proof.runtimeOwner === 'android-foreground-service'
  );
}

function protocolBridgeProofIsHonest(proof: ChildAndroidServiceProtocolBridgeProof): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.nativeBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidServiceProtocolProof' &&
    proof.storageBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof' &&
    proof.bridgeState === 'package-local-scaffold' &&
    proof.storageBridgeState === 'package-local-scaffold' &&
    proof.externalTransportState === 'not-implemented' &&
    proof.runtimeOwner === 'android-native-wrapper' &&
    requiredValuesArePresent(proof.commands, RequiredCommands) &&
    requiredValuesArePresent(proof.events, RequiredEvents)
  );
}

function statusExportProofIsHonest(proof: ChildAndroidServiceStatusExportProof): boolean {
  return (
    proof.exportState === 'package-local-bundle' &&
    proof.runtimeOwner === 'status-export-bundle' &&
    requiredValuesArePresent(proof.fields, RequiredExportFields)
  );
}

function serviceSurfaceProofsAreHonest(proofs: ReadonlyArray<ChildAndroidServiceSurfaceProof>): boolean {
  const bySurface = new Map(proofs.map((entry) => [entry.surface, entry] as const));
  return (
    bySurface.size === proofs.length && RequiredSurfaces.every((surface) => surfaceProofIsHonest(bySurface, surface))
  );
}

function surfaceProofIsHonest(
  bySurface: ReadonlyMap<ChildAndroidServiceSurfaceName, ChildAndroidServiceSurfaceProof>,
  surface: ChildAndroidServiceSurfaceName
): boolean {
  const proof = bySurface.get(surface);
  if (!proof) {
    return false;
  }
  switch (surface) {
    case 'foreground-service-status':
      return surfaceMatches(
        proof,
        'foreground-mobile-service',
        'manual-required',
        'scaffold-only',
        'ci-mechanical-proof'
      );
    case 'storage-protocol-bridge':
      return surfaceMatches(proof, 'typed-protocol-bridge', 'scaffold', 'scaffold-only', 'ci-mechanical-proof');
    case 'status-export-surface':
      return surfaceMatches(proof, 'typed-protocol-bridge', 'scaffold', 'scaffold-only', 'package-local-scaffold');
    case 'usage-stats-capability-label':
      return surfaceMatches(proof, 'usage-stats', 'manual-required', 'permission-required', 'permission-required');
    case 'accessibility-capability-label':
      return surfaceMatches(proof, 'accessibility-service', 'not-implemented', 'unavailable', 'not-implemented');
    case 'vpn-dns-capability-label':
      return surfaceMatches(proof, 'vpn-dns-filtering', 'not-implemented', 'unavailable', 'not-implemented');
    case 'device-owner-capability-label':
      return surfaceMatches(proof, 'device-owner-policy', 'manual-required', 'blocked', 'blocked');
    case 'managed-profile-capability-label':
      return surfaceMatches(proof, 'managed-profile', 'manual-required', 'blocked', 'blocked');
  }
}

function surfaceMatches(
  proof: ChildAndroidServiceSurfaceProof,
  parentCapability: ChildAndroidServiceSurfaceProof['parentCapability'],
  parentCapabilityStatus: ChildAndroidServiceSurfaceProof['parentCapabilityStatus'],
  capabilityLabel: ChildAndroidServiceSurfaceProof['capabilityLabel'],
  proofState: ChildAndroidServiceSurfaceProof['proofState']
): boolean {
  return (
    proof.parentCapability === parentCapability &&
    proof.parentCapabilityStatus === parentCapabilityStatus &&
    proof.capabilityLabel === capabilityLabel &&
    proof.proofState === proofState
  );
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildAndroidServiceSurfaceName = Infer<typeof ChildAndroidServiceSurfaceNameSchema>;
export type ChildAndroidServiceCapabilityLabel = Infer<typeof ChildAndroidServiceCapabilityLabelSchema>;
export type ChildAndroidServiceProofState = Infer<typeof ChildAndroidServiceProofStateSchema>;
export type ChildAndroidServiceRuntimeOwner = Infer<typeof ChildAndroidServiceRuntimeOwnerSchema>;
export type ChildAndroidServiceRuntimeState = Infer<typeof ChildAndroidServiceRuntimeStateSchema>;
export type ChildAndroidServiceStatusExportState = Infer<typeof ChildAndroidServiceStatusExportStateSchema>;
export type ChildAndroidServiceProtocolCommand = Infer<typeof ChildAndroidServiceProtocolCommandSchema>;
export type ChildAndroidServiceProtocolEvent = Infer<typeof ChildAndroidServiceProtocolEventSchema>;
export type ChildAndroidServiceStatusExportField = Infer<typeof ChildAndroidServiceStatusExportFieldSchema>;
export type ChildAndroidForegroundServiceProof = Infer<typeof ChildAndroidForegroundServiceProofSchema>;
export type ChildAndroidServiceProtocolBridgeProof = Infer<typeof ChildAndroidServiceProtocolBridgeProofSchema>;
export type ChildAndroidServiceStatusExportProof = Infer<typeof ChildAndroidServiceStatusExportProofSchema>;
export type ChildAndroidServiceSurfaceProof = Infer<typeof ChildAndroidServiceSurfaceProofSchema>;
export type ChildAndroidServiceClaimBoundaries = Infer<typeof ChildAndroidServiceClaimBoundariesSchema>;
export type ChildAndroidServiceProtocolReadModel = Infer<typeof ChildAndroidServiceProtocolReadModelSchema>;
