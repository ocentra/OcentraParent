/* generated from crates/schema/src/child_signing_store_device_owner_matrix.rs */

import { Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const ChildSigningStoreDeviceOwnerMatrixContractRuntime = {
  SchemaVersion: 'child-signing-store-device-owner-matrix-proof',
} as const;

export type GeneratedChildSigningStoreDeviceOwnerMatrixSchemaVersion = 'child-signing-store-device-owner-matrix-proof';
export const GeneratedChildArtifactMatrixPathSchema = brandedNonEmptyStringSchema('GeneratedChildArtifactMatrixPath');
export const GeneratedChildArtifactMatrixBoundarySchema = brandedNonEmptyStringSchema(
  'GeneratedChildArtifactMatrixBoundary'
);
export const GeneratedChildArtifactMatrixTimestampSchema = brandedNonEmptyStringSchema(
  'GeneratedChildArtifactMatrixTimestamp'
);

export type GeneratedChildArtifactMatrixPath = typeof GeneratedChildArtifactMatrixPathSchema.Type;
export type GeneratedChildArtifactMatrixBoundary = typeof GeneratedChildArtifactMatrixBoundarySchema.Type;
export type GeneratedChildArtifactMatrixTimestamp = typeof GeneratedChildArtifactMatrixTimestampSchema.Type;

export type GeneratedChildArtifactMatrixPlatform = 'windows' | 'macos' | 'linux' | 'android' | 'ios';
export type GeneratedChildArtifactMatrixArtifactKind =
  | 'windows-msi-service-package'
  | 'macos-launchd-pkg'
  | 'linux-systemd-deb'
  | 'android-debug-apk'
  | 'ios-simulator-app-zip';
export type GeneratedChildArtifactMatrixDistributionMode =
  | 'direct-msi-download'
  | 'direct-pkg-download'
  | 'direct-deb-download'
  | 'debug-apk-sideload'
  | 'unsigned-simulator-zip';
export type GeneratedChildArtifactMatrixArtifactProofState =
  | 'ci-mechanical-proof'
  | 'ci-package-only'
  | 'simulator-scaffold';
export type GeneratedChildArtifactMatrixProofSource =
  | 'windows-release-script'
  | 'macos-service-package-proof'
  | 'linux-service-package-proof'
  | 'android-device-proof-gate'
  | 'ios-entitlement-proof';
export type GeneratedChildArtifactMatrixSigningState = 'unsigned' | 'debug-signed' | 'signing-disabled';
export type GeneratedChildArtifactMatrixStoreDistributionState = 'not-applicable' | 'planned';
export type GeneratedChildArtifactMatrixManagementState =
  | 'not-applicable'
  | 'manual-required'
  | 'device-proof-required';

export const GeneratedChildArtifactMatrixPlatforms = [
  'windows',
  'macos',
  'linux',
  'android',
  'ios',
] as const satisfies readonly GeneratedChildArtifactMatrixPlatform[];
export const GeneratedChildArtifactMatrixArtifactKinds = [
  'windows-msi-service-package',
  'macos-launchd-pkg',
  'linux-systemd-deb',
  'android-debug-apk',
  'ios-simulator-app-zip',
] as const satisfies readonly GeneratedChildArtifactMatrixArtifactKind[];
export const GeneratedChildArtifactMatrixDistributionModes = [
  'direct-msi-download',
  'direct-pkg-download',
  'direct-deb-download',
  'debug-apk-sideload',
  'unsigned-simulator-zip',
] as const satisfies readonly GeneratedChildArtifactMatrixDistributionMode[];
export const GeneratedChildArtifactMatrixArtifactProofStates = [
  'ci-mechanical-proof',
  'ci-package-only',
  'simulator-scaffold',
] as const satisfies readonly GeneratedChildArtifactMatrixArtifactProofState[];
export const GeneratedChildArtifactMatrixProofSources = [
  'windows-release-script',
  'macos-service-package-proof',
  'linux-service-package-proof',
  'android-device-proof-gate',
  'ios-entitlement-proof',
] as const satisfies readonly GeneratedChildArtifactMatrixProofSource[];
export const GeneratedChildArtifactMatrixSigningStates = [
  'unsigned',
  'debug-signed',
  'signing-disabled',
] as const satisfies readonly GeneratedChildArtifactMatrixSigningState[];
export const GeneratedChildArtifactMatrixStoreDistributionStates = [
  'not-applicable',
  'planned',
] as const satisfies readonly GeneratedChildArtifactMatrixStoreDistributionState[];
export const GeneratedChildArtifactMatrixManagementStates = [
  'not-applicable',
  'manual-required',
  'device-proof-required',
] as const satisfies readonly GeneratedChildArtifactMatrixManagementState[];

export const GeneratedChildArtifactMatrixPlatformSchema = withParser(
  Schema.Literal('windows', 'macos', 'linux', 'android', 'ios')
);
export const GeneratedChildArtifactMatrixArtifactKindSchema = withParser(
  Schema.Literal(
    'windows-msi-service-package',
    'macos-launchd-pkg',
    'linux-systemd-deb',
    'android-debug-apk',
    'ios-simulator-app-zip'
  )
);
export const GeneratedChildArtifactMatrixDistributionModeSchema = withParser(
  Schema.Literal(
    'direct-msi-download',
    'direct-pkg-download',
    'direct-deb-download',
    'debug-apk-sideload',
    'unsigned-simulator-zip'
  )
);
export const GeneratedChildArtifactMatrixArtifactProofStateSchema = withParser(
  Schema.Literal('ci-mechanical-proof', 'ci-package-only', 'simulator-scaffold')
);
export const GeneratedChildArtifactMatrixProofSourceSchema = withParser(
  Schema.Literal(
    'windows-release-script',
    'macos-service-package-proof',
    'linux-service-package-proof',
    'android-device-proof-gate',
    'ios-entitlement-proof'
  )
);
export const GeneratedChildArtifactMatrixSigningStateSchema = withParser(
  Schema.Literal('unsigned', 'debug-signed', 'signing-disabled')
);
export const GeneratedChildArtifactMatrixStoreDistributionStateSchema = withParser(
  Schema.Literal('not-applicable', 'planned')
);
export const GeneratedChildArtifactMatrixManagementStateSchema = withParser(
  Schema.Literal('not-applicable', 'manual-required', 'device-proof-required')
);

export interface GeneratedChildArtifactMatrixRow {
  readonly platform: GeneratedChildArtifactMatrixPlatform;
  readonly artifactKind: GeneratedChildArtifactMatrixArtifactKind;
  readonly distributionMode: GeneratedChildArtifactMatrixDistributionMode;
  readonly artifactProofState: GeneratedChildArtifactMatrixArtifactProofState;
  readonly artifactPackageRef: GeneratedChildArtifactMatrixPath;
  readonly proofSource: GeneratedChildArtifactMatrixProofSource;
  readonly proofRefs: readonly GeneratedChildArtifactMatrixPath[];
  readonly signingState: GeneratedChildArtifactMatrixSigningState;
  readonly storeDistributionState: GeneratedChildArtifactMatrixStoreDistributionState;
  readonly deviceOwnerState: GeneratedChildArtifactMatrixManagementState;
  readonly managedProfileState: GeneratedChildArtifactMatrixManagementState;
  readonly supervisionState: GeneratedChildArtifactMatrixManagementState;
  readonly signingBoundary: GeneratedChildArtifactMatrixBoundary;
  readonly storeBoundary: GeneratedChildArtifactMatrixBoundary;
  readonly managementBoundary: GeneratedChildArtifactMatrixBoundary;
  readonly claimBoundary: GeneratedChildArtifactMatrixBoundary;
}

export const GeneratedChildArtifactMatrixRowSchema = withParser(
  Schema.Struct({
    platform: GeneratedChildArtifactMatrixPlatformSchema,
    artifactKind: GeneratedChildArtifactMatrixArtifactKindSchema,
    distributionMode: GeneratedChildArtifactMatrixDistributionModeSchema,
    artifactProofState: GeneratedChildArtifactMatrixArtifactProofStateSchema,
    artifactPackageRef: GeneratedChildArtifactMatrixPathSchema,
    proofSource: GeneratedChildArtifactMatrixProofSourceSchema,
    proofRefs: Schema.Array(GeneratedChildArtifactMatrixPathSchema),
    signingState: GeneratedChildArtifactMatrixSigningStateSchema,
    storeDistributionState: GeneratedChildArtifactMatrixStoreDistributionStateSchema,
    deviceOwnerState: GeneratedChildArtifactMatrixManagementStateSchema,
    managedProfileState: GeneratedChildArtifactMatrixManagementStateSchema,
    supervisionState: GeneratedChildArtifactMatrixManagementStateSchema,
    signingBoundary: GeneratedChildArtifactMatrixBoundarySchema,
    storeBoundary: GeneratedChildArtifactMatrixBoundarySchema,
    managementBoundary: GeneratedChildArtifactMatrixBoundarySchema,
    claimBoundary: GeneratedChildArtifactMatrixBoundarySchema,
  })
);

export interface GeneratedChildArtifactMatrixClaimBoundaries {
  readonly genericMatrix: GeneratedChildArtifactMatrixBoundary;
  readonly signingParity: GeneratedChildArtifactMatrixBoundary;
  readonly storeParity: GeneratedChildArtifactMatrixBoundary;
  readonly managementParity: GeneratedChildArtifactMatrixBoundary;
  readonly parentParity: GeneratedChildArtifactMatrixBoundary;
}

export const GeneratedChildArtifactMatrixClaimBoundariesSchema = withParser(
  Schema.Struct({
    genericMatrix: GeneratedChildArtifactMatrixBoundarySchema,
    signingParity: GeneratedChildArtifactMatrixBoundarySchema,
    storeParity: GeneratedChildArtifactMatrixBoundarySchema,
    managementParity: GeneratedChildArtifactMatrixBoundarySchema,
    parentParity: GeneratedChildArtifactMatrixBoundarySchema,
  })
);

export interface GeneratedChildSigningStoreDeviceOwnerMatrixProof {
  readonly schemaVersion: typeof ChildSigningStoreDeviceOwnerMatrixContractRuntime.SchemaVersion;
  readonly checkedAt: GeneratedChildArtifactMatrixTimestamp;
  readonly rows: readonly GeneratedChildArtifactMatrixRow[];
  readonly claimBoundaries: GeneratedChildArtifactMatrixClaimBoundaries;
}

export const GeneratedChildSigningStoreDeviceOwnerMatrixProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ChildSigningStoreDeviceOwnerMatrixContractRuntime.SchemaVersion),
    checkedAt: GeneratedChildArtifactMatrixTimestampSchema,
    rows: Schema.Array(GeneratedChildArtifactMatrixRowSchema),
    claimBoundaries: GeneratedChildArtifactMatrixClaimBoundariesSchema,
  })
);

export const GeneratedChildSigningStoreDeviceOwnerMatrixProof =
  GeneratedChildSigningStoreDeviceOwnerMatrixProofSchema.parse({
    schemaVersion: 'child-signing-store-device-owner-matrix-proof',
    checkedAt: '2026-06-28T19:45:00.000Z',
    rows: [
      {
        platform: 'windows',
        artifactKind: 'windows-msi-service-package',
        distributionMode: 'direct-msi-download',
        artifactProofState: 'ci-mechanical-proof',
        artifactPackageRef: 'target/release-packages/ocentra-parent-agent-windows-x64-latest.msi',
        proofSource: 'windows-release-script',
        proofRefs: ['scripts/release/windows/build-agent-package.ps1'],
        signingState: 'unsigned',
        storeDistributionState: 'not-applicable',
        deviceOwnerState: 'not-applicable',
        managedProfileState: 'not-applicable',
        supervisionState: 'not-applicable',
        signingBoundary:
          'Windows MSI packaging script signs the updater manifest but does not Authenticode-sign the child MSI or service binaries in this proof surface',
        storeBoundary:
          'Windows child artifact is a direct MSI download; no Microsoft Store or other store publication is claimed',
        managementBoundary:
          'Windows child artifact has no device-owner, managed-profile, or supervision claim in this matrix',
        claimBoundary:
          'Windows row proves MSI/service packaging and signed update-manifest wiring only; it does not prove signed child artifacts, store publication, or parent-client parity',
      },
      {
        platform: 'macos',
        artifactKind: 'macos-launchd-pkg',
        distributionMode: 'direct-pkg-download',
        artifactProofState: 'ci-mechanical-proof',
        artifactPackageRef: 'target/release-packages/macos/ocentra-parent-agent-macos-latest.pkg',
        proofSource: 'macos-service-package-proof',
        proofRefs: [
          'packages/schema-domain/src/child-macos-service-package-proof.ts',
          'scripts/test/child-macos-service-package-proof.mjs',
          'scripts/release/macos/build-agent-package.sh',
        ],
        signingState: 'unsigned',
        storeDistributionState: 'not-applicable',
        deviceOwnerState: 'not-applicable',
        managedProfileState: 'not-applicable',
        supervisionState: 'not-applicable',
        signingBoundary:
          'macOS child package stays unsigned in this proof surface because no codesign or productsign artifact is attached',
        storeBoundary:
          'macOS child artifact is a direct pkg download; no Mac App Store or other store publication is claimed',
        managementBoundary:
          'macOS child artifact has no device-owner, managed-profile, or supervision claim in this matrix',
        claimBoundary:
          'macOS row proves launchd pkg packaging only; it does not prove notarization, store publication, uninstall cleanup, or parent-client parity',
      },
      {
        platform: 'linux',
        artifactKind: 'linux-systemd-deb',
        distributionMode: 'direct-deb-download',
        artifactProofState: 'ci-mechanical-proof',
        artifactPackageRef: 'target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb',
        proofSource: 'linux-service-package-proof',
        proofRefs: [
          'packages/schema-domain/src/child-linux-service-package-proof.ts',
          'scripts/test/child-linux-service-package-proof.mjs',
          'scripts/release/linux/build-agent-package.sh',
        ],
        signingState: 'unsigned',
        storeDistributionState: 'not-applicable',
        deviceOwnerState: 'not-applicable',
        managedProfileState: 'not-applicable',
        supervisionState: 'not-applicable',
        signingBoundary:
          'the child Linux package is unsigned in this proof surface because no debsig, dpkg-sig, GPG, or repository signature artifact is attached',
        storeBoundary:
          'Linux child artifact is a direct .deb download; no apt repository, Snap, or other store publication is claimed',
        managementBoundary:
          'Linux child artifact has no device-owner, managed-profile, or supervision claim in this matrix',
        claimBoundary:
          'Linux row proves systemd .deb packaging and baseline metadata only; it does not prove signed repositories, store publication, or parent-client parity',
      },
      {
        platform: 'android',
        artifactKind: 'android-debug-apk',
        distributionMode: 'debug-apk-sideload',
        artifactProofState: 'ci-package-only',
        artifactPackageRef: 'target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk',
        proofSource: 'android-device-proof-gate',
        proofRefs: [
          'packages/schema-domain/src/child-android-device-proof-artifact-gate.ts',
          'scripts/test/child-android-device-proof-artifact-gate.mjs',
          'scripts/release/android/build-agent-package.mjs',
        ],
        signingState: 'debug-signed',
        storeDistributionState: 'planned',
        deviceOwnerState: 'manual-required',
        managedProfileState: 'manual-required',
        supervisionState: 'not-applicable',
        signingBoundary:
          'Android child artifact is a debug APK build; Play Store signing remains planned and not collected',
        storeBoundary:
          'Android Play Store distribution remains planned and not collected; debug APK proof does not claim release-track publication',
        managementBoundary:
          'Android device-owner and managed-profile states remain manual-required without enrollment evidence',
        claimBoundary:
          'Android row proves debug APK package output only; it does not prove device-owner, managed-profile, Play Store distribution, or parent-client parity',
      },
      {
        platform: 'ios',
        artifactKind: 'ios-simulator-app-zip',
        distributionMode: 'unsigned-simulator-zip',
        artifactProofState: 'simulator-scaffold',
        artifactPackageRef: 'target/release-packages/ios/ocentra-child-agent-ios-simulator-latest.zip',
        proofSource: 'ios-entitlement-proof',
        proofRefs: [
          'packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts',
          'scripts/test/child-ios-entitlement-capability-proof.mjs',
          'scripts/release/ios/build-simulator-app.sh',
        ],
        signingState: 'signing-disabled',
        storeDistributionState: 'planned',
        deviceOwnerState: 'not-applicable',
        managedProfileState: 'not-applicable',
        supervisionState: 'device-proof-required',
        signingBoundary:
          'iOS simulator package is built with code signing disabled; Apple signing, provisioning, and entitlements remain manual-required',
        storeBoundary:
          'iOS TestFlight and App Store distribution remain device-proof-required or planned; simulator ZIP proof does not claim store publication',
        managementBoundary:
          'iOS supervision remains device-proof-required; no device-owner or managed-profile claim exists for the child iOS slice',
        claimBoundary:
          'iOS row proves simulator scaffold packaging only; it does not prove Apple provisioning, supervision parity, hidden daemon authority, or parent-client parity',
      },
    ],
    claimBoundaries: {
      genericMatrix:
        'matrix rows summarize platform-specific package or proof artifacts only; they do not replace platform-specific package, device, or store proof',
      signingParity:
        'artifact signing states stay row-specific; signed update manifests, debug APK signatures, or unsigned simulator builds do not imply cross-platform signing parity',
      storeParity:
        'store states stay row-specific; direct-download rows do not claim Microsoft Store, Mac App Store, Linux repository publication, Play Store, TestFlight, or App Store publication',
      managementParity:
        'device-owner, managed-profile, and supervision states stay platform-specific and manual-required, device-proof-required, or not-applicable unless a row proves otherwise',
      parentParity:
        'child artifact matrix does not imply parent-client parity, hidden daemons, or broader child runtime readiness',
    },
  });
