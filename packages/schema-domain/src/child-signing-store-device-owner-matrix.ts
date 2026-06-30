/* thin adapter over Rust-generated child signing/store/device-owner matrix contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ChildSigningStoreDeviceOwnerMatrixContractRuntime,
  GeneratedChildArtifactMatrixArtifactKinds,
  GeneratedChildArtifactMatrixArtifactProofStates,
  GeneratedChildArtifactMatrixDistributionModes,
  GeneratedChildArtifactMatrixManagementStates,
  GeneratedChildArtifactMatrixPlatforms,
  GeneratedChildArtifactMatrixProofSources,
  GeneratedChildArtifactMatrixSigningStates,
  GeneratedChildArtifactMatrixStoreDistributionStates,
  GeneratedChildSigningStoreDeviceOwnerMatrixProof,
  type GeneratedChildArtifactMatrixPlatform,
  type GeneratedChildSigningStoreDeviceOwnerMatrixProof as GeneratedChildSigningStoreDeviceOwnerMatrixProofShape,
} from './generated/child-signing-store-device-owner-matrix-contracts';

export const ChildSigningStoreDeviceOwnerMatrixSchemaVersionSchema = withParser(
  Schema.Literal(ChildSigningStoreDeviceOwnerMatrixContractRuntime.SchemaVersion)
);

export const RequiredChildArtifactMatrixPlatforms = [...GeneratedChildArtifactMatrixPlatforms] as const;

const ChildArtifactMatrixPlatformSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixPlatforms)
);
const ChildArtifactMatrixArtifactKindSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixArtifactKinds)
);
const ChildArtifactMatrixDistributionModeSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixDistributionModes)
);
const ChildArtifactMatrixArtifactProofStateSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixArtifactProofStates)
);
const ChildArtifactMatrixProofSourceSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixProofSources)
);
const ChildArtifactMatrixSigningStateSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixSigningStates)
);
const ChildArtifactMatrixStoreDistributionStateSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixStoreDistributionStates)
);
const ChildArtifactMatrixManagementStateSchema = withParser(
  Schema.Literal(...GeneratedChildArtifactMatrixManagementStates)
);

const ChildArtifactMatrixPathSchema = brandedNonEmptyStringSchema('ChildArtifactMatrixPath');
const ChildArtifactMatrixBoundarySchema = brandedNonEmptyStringSchema('ChildArtifactMatrixBoundary');
const ChildArtifactMatrixTimestampSchema = brandedNonEmptyStringSchema('ChildArtifactMatrixTimestamp');

function hasExactPlatformCoverage(
  rows: ReadonlyArray<{ platform: GeneratedChildArtifactMatrixPlatform }>
): boolean {
  if (rows.length !== RequiredChildArtifactMatrixPlatforms.length) {
    return false;
  }

  const platforms = rows.map((row) => row.platform);
  return RequiredChildArtifactMatrixPlatforms.every(
    (platform) => platforms.filter((value) => value === platform).length === 1
  );
}

const ChildArtifactMatrixRowBaseSchema = Schema.Struct({
  platform: ChildArtifactMatrixPlatformSchema,
  artifactKind: ChildArtifactMatrixArtifactKindSchema,
  distributionMode: ChildArtifactMatrixDistributionModeSchema,
  artifactProofState: ChildArtifactMatrixArtifactProofStateSchema,
  artifactPackageRef: ChildArtifactMatrixPathSchema,
  proofSource: ChildArtifactMatrixProofSourceSchema,
  proofRefs: Schema.Array(ChildArtifactMatrixPathSchema).pipe(
    Schema.filter(
      (proofRefs) => proofRefs.length > 0 || 'Expected every artifact row to keep at least one proof reference'
    )
  ),
  signingState: ChildArtifactMatrixSigningStateSchema,
  storeDistributionState: ChildArtifactMatrixStoreDistributionStateSchema,
  deviceOwnerState: ChildArtifactMatrixManagementStateSchema,
  managedProfileState: ChildArtifactMatrixManagementStateSchema,
  supervisionState: ChildArtifactMatrixManagementStateSchema,
  signingBoundary: ChildArtifactMatrixBoundarySchema,
  storeBoundary: ChildArtifactMatrixBoundarySchema,
  managementBoundary: ChildArtifactMatrixBoundarySchema,
  claimBoundary: ChildArtifactMatrixBoundarySchema,
});

export const ChildArtifactMatrixRowSchema = withParser(ChildArtifactMatrixRowBaseSchema);

export const ChildArtifactMatrixClaimBoundariesSchema = withParser(
  Schema.Struct({
    genericMatrix: ChildArtifactMatrixBoundarySchema,
    signingParity: ChildArtifactMatrixBoundarySchema,
    storeParity: ChildArtifactMatrixBoundarySchema,
    managementParity: ChildArtifactMatrixBoundarySchema,
    parentParity: ChildArtifactMatrixBoundarySchema,
  })
);

const ChildSigningStoreDeviceOwnerMatrixBaseSchema = Schema.Struct({
  schemaVersion: ChildSigningStoreDeviceOwnerMatrixSchemaVersionSchema,
  checkedAt: ChildArtifactMatrixTimestampSchema,
  rows: Schema.Array(ChildArtifactMatrixRowSchema).pipe(
    Schema.filter(
      (rows) =>
        hasExactPlatformCoverage(rows) ||
        'Expected exactly one generated matrix row for windows, macos, linux, android, and ios'
    )
  ),
  claimBoundaries: ChildArtifactMatrixClaimBoundariesSchema,
});

export const ChildSigningStoreDeviceOwnerMatrixSchema = withParser(
  ChildSigningStoreDeviceOwnerMatrixBaseSchema
);

export type ChildArtifactMatrixRow = Infer<typeof ChildArtifactMatrixRowSchema>;
export type ChildArtifactMatrixClaimBoundaries = Infer<typeof ChildArtifactMatrixClaimBoundariesSchema>;
export type ChildSigningStoreDeviceOwnerMatrixProof = Infer<
  typeof ChildSigningStoreDeviceOwnerMatrixSchema
> &
  GeneratedChildSigningStoreDeviceOwnerMatrixProofShape;

export const ChildSigningStoreDeviceOwnerMatrixProofReadModel =
  ChildSigningStoreDeviceOwnerMatrixSchema.parse(GeneratedChildSigningStoreDeviceOwnerMatrixProof);
