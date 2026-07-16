/* generated from crates/schema/src/child_android_storage_protocol_proof_ts.rs */

import { type Infer, brandedNonEmptyStringSchema, Schema, withParser } from './effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './family-reference-primitives';

export const ChildAndroidStorageProtocolProofSchemaVersionSchema = withParser(
  Schema.Literal('child-android-storage-protocol-capability-proof')
);
export const ChildAndroidStorageSurfaceNameSchema = withParser(
  Schema.Literal(
    'app-private-files',
    'encrypted-evidence-journal',
    'sqlite-query-store',
    'parent-owned-export',
    'ocentra-hosted-child-activity-storage',
    'protocol-storage-snapshot'
  )
);
export const ChildAndroidStorageProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'package-local-scaffold', 'manual-required', 'planned', 'not-implemented')
);
export const ChildAndroidStorageRuntimeOwnerSchema = withParser(
  Schema.Literal(
    'android-native-wrapper',
    'android-app-private-storage',
    'child-agent-runtime',
    'parent-owned-storage',
    'agent-protocol',
    'ocentra-hosted-service',
    'manual-device-proof'
  )
);
export const ChildAndroidStorageCustodySchema = withParser(
  Schema.Literal('child-device-local', 'parent-owned-local', 'ocentra-hosted', 'none')
);
export const ChildAndroidStorageDefaultModeSchema = withParser(
  Schema.Literal('package-local-app-private', 'disabled', 'parent-owned-export-only', 'not-default')
);
export const ChildAndroidRawChildActivityStorageStateSchema = withParser(
  Schema.Literal('not-collected', 'temporary-local-only', 'not-default')
);
export const ChildAndroidStorageProtocolCommandSchema = withParser(
  Schema.Literal(
    'child.android.storage.snapshot.get',
    'child.android.storage.capability.proof.get',
    'child.android.storage.protocol.proof.get'
  )
);
export const ChildAndroidStorageProtocolEventSchema = withParser(
  Schema.Literal(
    'child.android.storage.snapshot.reported',
    'child.android.storage.capability.proof.reported',
    'child.android.storage.protocol.proof.reported'
  )
);
export const ChildAndroidStorageProtocolBridgeStateSchema = withParser(
  Schema.Literal('package-local-scaffold', 'not-implemented')
);

const ChildAndroidStoragePackageIdSchema = brandedNonEmptyStringSchema('ChildAndroidStoragePackageId');
const ChildAndroidStorageClassNameSchema = brandedNonEmptyStringSchema('ChildAndroidStorageClassName');
const ChildAndroidStorageRequirementSchema = brandedNonEmptyStringSchema('ChildAndroidStorageRequirement');
const ChildAndroidStorageBoundarySchema = brandedNonEmptyStringSchema('ChildAndroidStorageBoundary');

export const ChildAndroidStorageSurfaceProofSchema = withParser(
  Schema.Struct({
    surface: ChildAndroidStorageSurfaceNameSchema,
    parentCapability: ParentControlCapabilityNameSchema,
    parentCapabilityStatus: ParentControlCapabilityStatusSchema,
    proofState: ChildAndroidStorageProofStateSchema,
    runtimeOwner: ChildAndroidStorageRuntimeOwnerSchema,
    custody: ChildAndroidStorageCustodySchema,
    defaultStorageMode: ChildAndroidStorageDefaultModeSchema,
    rawChildActivityStorage: ChildAndroidRawChildActivityStorageStateSchema,
    proofRequirement: ChildAndroidStorageRequirementSchema,
    claimBoundary: ChildAndroidStorageBoundarySchema,
  })
);

export const ChildAndroidStorageProtocolBridgeProofSchema = withParser(
  Schema.Struct({
    packageId: ChildAndroidStoragePackageIdSchema,
    nativeBridgeClass: ChildAndroidStorageClassNameSchema,
    bridgeState: ChildAndroidStorageProtocolBridgeStateSchema,
    externalTransportState: ChildAndroidStorageProtocolBridgeStateSchema,
    commands: Schema.Array(ChildAndroidStorageProtocolCommandSchema),
    events: Schema.Array(ChildAndroidStorageProtocolEventSchema),
    runtimeOwner: ChildAndroidStorageRuntimeOwnerSchema,
    proofRequirement: ChildAndroidStorageRequirementSchema,
    claimBoundary: ChildAndroidStorageBoundarySchema,
  })
);

export const ChildAndroidStorageClaimBoundariesSchema = withParser(
  Schema.Struct({
    appPrivateFiles: ChildAndroidStorageBoundarySchema,
    encryptedEvidenceJournal: ChildAndroidStorageBoundarySchema,
    sqliteQueryStore: ChildAndroidStorageBoundarySchema,
    parentOwnedExport: ChildAndroidStorageBoundarySchema,
    ocentraHostedStorage: ChildAndroidStorageBoundarySchema,
    protocolTransport: ChildAndroidStorageBoundarySchema,
    childAndroidStoragePersistence: ChildAndroidStorageBoundarySchema,
  })
);

const ChildAndroidStorageProtocolReadModelBaseSchema = Schema.Struct({
  schemaVersion: ChildAndroidStorageProtocolProofSchemaVersionSchema,
  protocolBridgeProof: ChildAndroidStorageProtocolBridgeProofSchema,
  storageSurfaces: Schema.Array(ChildAndroidStorageSurfaceProofSchema),
  claimBoundaries: ChildAndroidStorageClaimBoundariesSchema,
  updatedAt: ParentTimestampSchema,
});

type ChildAndroidStorageProtocolReadModelCandidate = Infer<typeof ChildAndroidStorageProtocolReadModelBaseSchema>;

export const ChildAndroidStorageProtocolReadModelSchema = withParser(
  ChildAndroidStorageProtocolReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        childAndroidStorageProtocolReadModelIsHonest(readModel) ||
        'Expected Child Android storage/protocol proof to keep storage package-local, raw child activity uploads off by default, encrypted journal/query store unimplemented, parent-owned export planned, hosted storage not-default, and external transport unimplemented'
    )
  )
);

const RequiredStorageSurfaces = [
  'app-private-files',
  'encrypted-evidence-journal',
  'sqlite-query-store',
  'parent-owned-export',
  'ocentra-hosted-child-activity-storage',
  'protocol-storage-snapshot',
] as const satisfies ReadonlyArray<ChildAndroidStorageSurfaceName>;

const RequiredCommands = [
  'child.android.storage.snapshot.get',
  'child.android.storage.capability.proof.get',
  'child.android.storage.protocol.proof.get',
] as const satisfies ReadonlyArray<ChildAndroidStorageProtocolCommand>;

const RequiredEvents = [
  'child.android.storage.snapshot.reported',
  'child.android.storage.capability.proof.reported',
  'child.android.storage.protocol.proof.reported',
] as const satisfies ReadonlyArray<ChildAndroidStorageProtocolEvent>;

function childAndroidStorageProtocolReadModelIsHonest(
  readModel: ChildAndroidStorageProtocolReadModelCandidate
): boolean {
  return (
    storageProtocolBridgeProofIsHonest(readModel.protocolBridgeProof) &&
    storageSurfaceProofsAreHonest(readModel.storageSurfaces)
  );
}

function storageProtocolBridgeProofIsHonest(bridgeProof: ChildAndroidStorageProtocolBridgeProof): boolean {
  return (
    bridgeProof.packageId === 'ca.ocentra.parent.agent' &&
    bridgeProof.nativeBridgeClass === 'ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof' &&
    bridgeProof.bridgeState === 'package-local-scaffold' &&
    bridgeProof.externalTransportState === 'not-implemented' &&
    bridgeProof.runtimeOwner === 'android-native-wrapper' &&
    requiredValuesArePresent(bridgeProof.commands, RequiredCommands) &&
    requiredValuesArePresent(bridgeProof.events, RequiredEvents)
  );
}

function storageSurfaceProofsAreHonest(proofs: ReadonlyArray<ChildAndroidStorageSurfaceProof>): boolean {
  const bySurface = new Map(proofs.map((entry) => [entry.surface, entry] as const));
  if (bySurface.size !== proofs.length || bySurface.size !== RequiredStorageSurfaces.length) {
    return false;
  }

  return RequiredStorageSurfaces.every((surface) => {
    const proof = bySurface.get(surface);
    return Boolean(proof && storageSurfaceProofIsHonest(proof));
  });
}

function storageSurfaceProofIsHonest(proof: ChildAndroidStorageSurfaceProof): boolean {
  switch (proof.surface) {
    case 'app-private-files':
      return appPrivateFilesProofIsHonest(proof);
    case 'protocol-storage-snapshot':
      return protocolStorageSnapshotProofIsHonest(proof);
    case 'encrypted-evidence-journal':
      return childRuntimeStorageGapIsHonest(proof, 'temporary-local-only');
    case 'sqlite-query-store':
      return childRuntimeStorageGapIsHonest(proof, 'not-collected');
    case 'parent-owned-export':
      return parentOwnedExportProofIsHonest(proof);
    case 'ocentra-hosted-child-activity-storage':
      return hostedStorageProofIsHonest(proof);
  }
  return false;
}

function appPrivateFilesProofIsHonest(proof: ChildAndroidStorageSurfaceProof): boolean {
  return (
    proof.parentCapability === 'local-storage' &&
    proof.parentCapabilityStatus === 'scaffold' &&
    proof.proofState === 'package-local-scaffold' &&
    proof.runtimeOwner === 'android-app-private-storage' &&
    proof.custody === 'child-device-local' &&
    proof.defaultStorageMode === 'package-local-app-private' &&
    proof.rawChildActivityStorage === 'not-collected'
  );
}

function protocolStorageSnapshotProofIsHonest(proof: ChildAndroidStorageSurfaceProof): boolean {
  return (
    proof.parentCapability === 'typed-protocol-bridge' &&
    proof.parentCapabilityStatus === 'scaffold' &&
    proof.proofState === 'ci-mechanical-proof' &&
    proof.runtimeOwner === 'agent-protocol' &&
    proof.custody === 'none' &&
    proof.defaultStorageMode === 'disabled' &&
    proof.rawChildActivityStorage === 'not-collected'
  );
}

function parentOwnedExportProofIsHonest(proof: ChildAndroidStorageSurfaceProof): boolean {
  return (
    proof.parentCapability === 'local-storage' &&
    proof.parentCapabilityStatus === 'planned' &&
    proof.proofState === 'planned' &&
    proof.runtimeOwner === 'parent-owned-storage' &&
    proof.custody === 'parent-owned-local' &&
    proof.defaultStorageMode === 'parent-owned-export-only' &&
    proof.rawChildActivityStorage === 'not-collected'
  );
}

function hostedStorageProofIsHonest(proof: ChildAndroidStorageSurfaceProof): boolean {
  return (
    proof.parentCapability === 'local-storage' &&
    proof.parentCapabilityStatus === 'not-implemented' &&
    proof.proofState === 'not-implemented' &&
    proof.runtimeOwner === 'ocentra-hosted-service' &&
    proof.custody === 'ocentra-hosted' &&
    proof.defaultStorageMode === 'not-default' &&
    proof.rawChildActivityStorage === 'not-default'
  );
}

function childRuntimeStorageGapIsHonest(
  proof: ChildAndroidStorageSurfaceProof,
  rawChildActivityStorage: ChildAndroidRawChildActivityStorageState
): boolean {
  return (
    proof.parentCapability === 'local-storage' &&
    proof.parentCapabilityStatus === 'not-implemented' &&
    proof.proofState === 'not-implemented' &&
    proof.runtimeOwner === 'child-agent-runtime' &&
    proof.custody === 'child-device-local' &&
    proof.defaultStorageMode === 'disabled' &&
    proof.rawChildActivityStorage === rawChildActivityStorage
  );
}

function requiredValuesArePresent<Value extends string>(
  values: ReadonlyArray<Value>,
  required: ReadonlyArray<Value>
): boolean {
  const valueSet = new Set(values);
  return valueSet.size === values.length && required.every((value) => valueSet.has(value));
}

export type ChildAndroidStorageSurfaceName = Infer<typeof ChildAndroidStorageSurfaceNameSchema>;
export type ChildAndroidStorageProofState = Infer<typeof ChildAndroidStorageProofStateSchema>;
export type ChildAndroidStorageRuntimeOwner = Infer<typeof ChildAndroidStorageRuntimeOwnerSchema>;
export type ChildAndroidStorageCustody = Infer<typeof ChildAndroidStorageCustodySchema>;
export type ChildAndroidStorageDefaultMode = Infer<typeof ChildAndroidStorageDefaultModeSchema>;
export type ChildAndroidRawChildActivityStorageState = Infer<typeof ChildAndroidRawChildActivityStorageStateSchema>;
export type ChildAndroidStorageProtocolCommand = Infer<typeof ChildAndroidStorageProtocolCommandSchema>;
export type ChildAndroidStorageProtocolEvent = Infer<typeof ChildAndroidStorageProtocolEventSchema>;
export type ChildAndroidStorageProtocolBridgeState = Infer<typeof ChildAndroidStorageProtocolBridgeStateSchema>;
export type ChildAndroidStorageSurfaceProof = Infer<typeof ChildAndroidStorageSurfaceProofSchema>;
export type ChildAndroidStorageProtocolBridgeProof = Infer<typeof ChildAndroidStorageProtocolBridgeProofSchema>;
export type ChildAndroidStorageClaimBoundaries = Infer<typeof ChildAndroidStorageClaimBoundariesSchema>;
export type ChildAndroidStorageProtocolReadModel = Infer<typeof ChildAndroidStorageProtocolReadModelSchema>;
