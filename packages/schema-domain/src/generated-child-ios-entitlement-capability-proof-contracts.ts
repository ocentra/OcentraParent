/* generated from crates/schema/src/child_ios_entitlement_capability_proof.rs */

export const ChildIosEntitlementCapabilityContractRuntime = {
  SchemaVersion: 'child-ios-entitlement-capability-proof',
} as const;

export type GeneratedChildIosEntitlementCapabilityProofSchemaVersion = 'child-ios-entitlement-capability-proof';
export type GeneratedChildIosEntitlementBundleId = string;
export type GeneratedChildIosEntitlementClassName = string;
export type GeneratedChildIosEntitlementRequirement = string;
export type GeneratedChildIosEntitlementBoundary = string;
export type GeneratedChildIosEntitlementTimestamp = string;

export type GeneratedChildIosEntitlementParentCapability =
  | 'package-lifecycle'
  | 'typed-protocol-bridge'
  | 'family-controls-entitlement'
  | 'device-activity'
  | 'screen-time-api'
  | 'network-extension'
  | 'notifications'
  | 'background-execution'
  | 'signing-entitlements'
  | 'testflight-distribution'
  | 'store-distribution';
export type GeneratedChildIosEntitlementParentCapabilityStatus = 'manual-required' | 'scaffold' | 'planned';
export type GeneratedChildIosEntitlementSurfaceName =
  | 'simulator-app-target'
  | 'bundle-identifier'
  | 'status-surface'
  | 'family-controls-entitlement'
  | 'device-activity-framework'
  | 'screen-time-api'
  | 'network-extension'
  | 'notifications-permission'
  | 'background-execution'
  | 'provisioning-profile'
  | 'supervision-state'
  | 'signing-entitlements'
  | 'testflight-distribution'
  | 'physical-device-proof'
  | 'app-store-distribution';
export type GeneratedChildIosEntitlementProofState =
  | 'ci-mechanical-proof'
  | 'simulator-scaffold'
  | 'manual-required'
  | 'entitlement-required'
  | 'signing-required'
  | 'device-proof-required'
  | 'not-declared'
  | 'not-implemented'
  | 'planned';
export type GeneratedChildIosEntitlementRuntimeOwner =
  | 'ios-xcode-project'
  | 'ios-info-plist'
  | 'ios-swift-scaffold'
  | 'ios-simulator-build-script'
  | 'apple-simulator-host'
  | 'apple-entitlement'
  | 'apple-device-framework'
  | 'apple-network-extension'
  | 'apple-notification-permission'
  | 'apple-background-mode'
  | 'apple-signing'
  | 'apple-testflight'
  | 'apple-device-proof'
  | 'app-store-connect';
export type GeneratedChildIosEntitlementDeclarationState =
  | 'declared-in-project'
  | 'declared-in-plist'
  | 'scaffold-status-label'
  | 'not-declared'
  | 'not-applicable';
export type GeneratedChildIosEntitlementPackagePhase =
  | 'xcode-project-target'
  | 'bundle-identifier'
  | 'simulator-build-script'
  | 'status-view'
  | 'info-plist'
  | 'simulator-build'
  | 'simulator-launch'
  | 'device-install'
  | 'device-launch'
  | 'testflight-install'
  | 'signing-profile'
  | 'entitlement-review'
  | 'recovery-behavior';
export type GeneratedChildIosEntitlementProtocolCommand =
  | 'child.ios.entitlement.capability.snapshot.get'
  | 'child.ios.entitlement.package.proof.get'
  | 'child.ios.entitlement.manual-proof.get';
export type GeneratedChildIosEntitlementProtocolEvent =
  | 'child.ios.entitlement.capability.snapshot.reported'
  | 'child.ios.entitlement.package.proof.reported'
  | 'child.ios.entitlement.manual-proof.reported';
export type GeneratedChildIosEntitlementBridgeState = 'simulator-scaffold' | 'not-implemented';

export const GeneratedChildIosEntitlementParentCapabilities = [
  'package-lifecycle',
  'typed-protocol-bridge',
  'family-controls-entitlement',
  'device-activity',
  'screen-time-api',
  'network-extension',
  'notifications',
  'background-execution',
  'signing-entitlements',
  'testflight-distribution',
  'store-distribution',
] as const satisfies readonly GeneratedChildIosEntitlementParentCapability[];
export const GeneratedChildIosEntitlementParentCapabilityStatuses = [
  'manual-required',
  'scaffold',
  'planned',
] as const satisfies readonly GeneratedChildIosEntitlementParentCapabilityStatus[];
export const GeneratedChildIosEntitlementSurfaceNames = [
  'simulator-app-target',
  'bundle-identifier',
  'status-surface',
  'family-controls-entitlement',
  'device-activity-framework',
  'screen-time-api',
  'network-extension',
  'notifications-permission',
  'background-execution',
  'provisioning-profile',
  'supervision-state',
  'signing-entitlements',
  'testflight-distribution',
  'physical-device-proof',
  'app-store-distribution',
] as const satisfies readonly GeneratedChildIosEntitlementSurfaceName[];
export const GeneratedChildIosEntitlementProofStates = [
  'ci-mechanical-proof',
  'simulator-scaffold',
  'manual-required',
  'entitlement-required',
  'signing-required',
  'device-proof-required',
  'not-declared',
  'not-implemented',
  'planned',
] as const satisfies readonly GeneratedChildIosEntitlementProofState[];
export const GeneratedChildIosEntitlementRuntimeOwners = [
  'ios-xcode-project',
  'ios-info-plist',
  'ios-swift-scaffold',
  'ios-simulator-build-script',
  'apple-simulator-host',
  'apple-entitlement',
  'apple-device-framework',
  'apple-network-extension',
  'apple-notification-permission',
  'apple-background-mode',
  'apple-signing',
  'apple-testflight',
  'apple-device-proof',
  'app-store-connect',
] as const satisfies readonly GeneratedChildIosEntitlementRuntimeOwner[];
export const GeneratedChildIosEntitlementDeclarationStates = [
  'declared-in-project',
  'declared-in-plist',
  'scaffold-status-label',
  'not-declared',
  'not-applicable',
] as const satisfies readonly GeneratedChildIosEntitlementDeclarationState[];
export const GeneratedChildIosEntitlementPackagePhases = [
  'xcode-project-target',
  'bundle-identifier',
  'simulator-build-script',
  'status-view',
  'info-plist',
  'simulator-build',
  'simulator-launch',
  'device-install',
  'device-launch',
  'testflight-install',
  'signing-profile',
  'entitlement-review',
  'recovery-behavior',
] as const satisfies readonly GeneratedChildIosEntitlementPackagePhase[];
export const GeneratedChildIosEntitlementProtocolCommands = [
  'child.ios.entitlement.capability.snapshot.get',
  'child.ios.entitlement.package.proof.get',
  'child.ios.entitlement.manual-proof.get',
] as const satisfies readonly GeneratedChildIosEntitlementProtocolCommand[];
export const GeneratedChildIosEntitlementProtocolEvents = [
  'child.ios.entitlement.capability.snapshot.reported',
  'child.ios.entitlement.package.proof.reported',
  'child.ios.entitlement.manual-proof.reported',
] as const satisfies readonly GeneratedChildIosEntitlementProtocolEvent[];
export const GeneratedChildIosEntitlementBridgeStates = [
  'simulator-scaffold',
  'not-implemented',
] as const satisfies readonly GeneratedChildIosEntitlementBridgeState[];

export interface GeneratedChildIosEntitlementSurfaceProof {
  surface: GeneratedChildIosEntitlementSurfaceName;
  parentCapability: GeneratedChildIosEntitlementParentCapability;
  parentCapabilityStatus: GeneratedChildIosEntitlementParentCapabilityStatus;
  declarationState: GeneratedChildIosEntitlementDeclarationState;
  proofState: GeneratedChildIosEntitlementProofState;
  runtimeOwner: GeneratedChildIosEntitlementRuntimeOwner;
  proofRequirement: GeneratedChildIosEntitlementRequirement;
  claimBoundary: GeneratedChildIosEntitlementBoundary;
}

export interface GeneratedChildIosEntitlementPackageLifecycleProof {
  phase: GeneratedChildIosEntitlementPackagePhase;
  proofState: GeneratedChildIosEntitlementProofState;
  runtimeOwner: GeneratedChildIosEntitlementRuntimeOwner;
  proofRequirement: GeneratedChildIosEntitlementRequirement;
  claimBoundary: GeneratedChildIosEntitlementBoundary;
}

export interface GeneratedChildIosEntitlementProtocolBridgeProof {
  bundleId: GeneratedChildIosEntitlementBundleId;
  statusSurfaceClass: GeneratedChildIosEntitlementClassName;
  bridgeState: GeneratedChildIosEntitlementBridgeState;
  externalTransportState: GeneratedChildIosEntitlementBridgeState;
  commands: readonly GeneratedChildIosEntitlementProtocolCommand[];
  events: readonly GeneratedChildIosEntitlementProtocolEvent[];
  runtimeOwner: GeneratedChildIosEntitlementRuntimeOwner;
  proofRequirement: GeneratedChildIosEntitlementRequirement;
  claimBoundary: GeneratedChildIosEntitlementBoundary;
}

export interface GeneratedChildIosEntitlementClaimBoundaries {
  simulatorPackage: GeneratedChildIosEntitlementBoundary;
  launchAvailability: GeneratedChildIosEntitlementBoundary;
  familyControls: GeneratedChildIosEntitlementBoundary;
  deviceActivity: GeneratedChildIosEntitlementBoundary;
  screenTime: GeneratedChildIosEntitlementBoundary;
  networkExtension: GeneratedChildIosEntitlementBoundary;
  notifications: GeneratedChildIosEntitlementBoundary;
  backgroundExecution: GeneratedChildIosEntitlementBoundary;
  recoveryBehavior: GeneratedChildIosEntitlementBoundary;
  provisioningProfile: GeneratedChildIosEntitlementBoundary;
  supervision: GeneratedChildIosEntitlementBoundary;
  signingEntitlements: GeneratedChildIosEntitlementBoundary;
  testflight: GeneratedChildIosEntitlementBoundary;
  deviceProof: GeneratedChildIosEntitlementBoundary;
  capabilityOnlyState: GeneratedChildIosEntitlementBoundary;
  externalTransport: GeneratedChildIosEntitlementBoundary;
}

export interface GeneratedChildIosEntitlementCapabilityReadModelShape {
  schemaVersion: typeof ChildIosEntitlementCapabilityContractRuntime.SchemaVersion;
  bundleId: GeneratedChildIosEntitlementBundleId;
  statusSurfaceClass: GeneratedChildIosEntitlementClassName;
  protocolBridgeProof: GeneratedChildIosEntitlementProtocolBridgeProof;
  surfaceProofs: readonly GeneratedChildIosEntitlementSurfaceProof[];
  packageLifecycleProofs: readonly GeneratedChildIosEntitlementPackageLifecycleProof[];
  claimBoundaries: GeneratedChildIosEntitlementClaimBoundaries;
  updatedAt: GeneratedChildIosEntitlementTimestamp;
}

export const GeneratedChildIosEntitlementCapabilityReadModel = {
  schemaVersion: 'child-ios-entitlement-capability-proof',
  bundleId: 'ca.ocentra.child.agent',
  statusSurfaceClass: 'AgentStatusViewController',
  protocolBridgeProof: {
    bundleId: 'ca.ocentra.child.agent',
    statusSurfaceClass: 'AgentStatusViewController',
    bridgeState: 'simulator-scaffold',
    externalTransportState: 'not-implemented',
    commands: [
      'child.ios.entitlement.capability.snapshot.get',
      'child.ios.entitlement.package.proof.get',
      'child.ios.entitlement.manual-proof.get',
    ],
    events: [
      'child.ios.entitlement.capability.snapshot.reported',
      'child.ios.entitlement.package.proof.reported',
      'child.ios.entitlement.manual-proof.reported',
    ],
    runtimeOwner: 'ios-swift-scaffold',
    proofRequirement:
      'iOS simulator scaffold status surface names capability-only launch, recovery, entitlement, provisioning, and supervision states',
    claimBoundary:
      'status surface is capability-only; no hidden daemon, persistent background service, launch recovery, external child-agent transport, Apple entitlement, provisioning, or device proof is claimed',
  },
  surfaceProofs: [
    {
      surface: 'simulator-app-target',
      parentCapability: 'package-lifecycle',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'declared-in-project',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'ios-xcode-project',
      proofRequirement: 'simulator-app-target remains ci-mechanical-proof until Apple artifacts change it',
      claimBoundary: 'simulator-app-target remains ci-mechanical-proof until Apple artifacts change it',
    },
    {
      surface: 'bundle-identifier',
      parentCapability: 'package-lifecycle',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'declared-in-project',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'ios-xcode-project',
      proofRequirement: 'bundle-identifier remains ci-mechanical-proof until Apple artifacts change it',
      claimBoundary: 'bundle-identifier remains ci-mechanical-proof until Apple artifacts change it',
    },
    {
      surface: 'status-surface',
      parentCapability: 'typed-protocol-bridge',
      parentCapabilityStatus: 'scaffold',
      declarationState: 'scaffold-status-label',
      proofState: 'simulator-scaffold',
      runtimeOwner: 'ios-swift-scaffold',
      proofRequirement: 'status-surface remains simulator-scaffold until Apple artifacts change it',
      claimBoundary: 'status-surface remains simulator-scaffold until Apple artifacts change it',
    },
    {
      surface: 'family-controls-entitlement',
      parentCapability: 'family-controls-entitlement',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      proofState: 'entitlement-required',
      runtimeOwner: 'apple-entitlement',
      proofRequirement: 'family-controls-entitlement remains entitlement-required until Apple artifacts change it',
      claimBoundary: 'family-controls-entitlement remains entitlement-required until Apple artifacts change it',
    },
    {
      surface: 'device-activity-framework',
      parentCapability: 'device-activity',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      proofState: 'entitlement-required',
      runtimeOwner: 'apple-device-framework',
      proofRequirement: 'device-activity-framework remains entitlement-required until Apple artifacts change it',
      claimBoundary: 'device-activity-framework remains entitlement-required until Apple artifacts change it',
    },
    {
      surface: 'screen-time-api',
      parentCapability: 'screen-time-api',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      proofState: 'entitlement-required',
      runtimeOwner: 'apple-device-framework',
      proofRequirement: 'screen-time-api remains entitlement-required until Apple artifacts change it',
      claimBoundary: 'screen-time-api remains entitlement-required until Apple artifacts change it',
    },
    {
      surface: 'network-extension',
      parentCapability: 'network-extension',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      proofState: 'entitlement-required',
      runtimeOwner: 'apple-network-extension',
      proofRequirement: 'network-extension remains entitlement-required until Apple artifacts change it',
      claimBoundary: 'network-extension remains entitlement-required until Apple artifacts change it',
    },
    {
      surface: 'notifications-permission',
      parentCapability: 'notifications',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      proofState: 'manual-required',
      runtimeOwner: 'apple-notification-permission',
      proofRequirement: 'notifications-permission remains manual-required until Apple artifacts change it',
      claimBoundary: 'notifications-permission remains manual-required until Apple artifacts change it',
    },
    {
      surface: 'background-execution',
      parentCapability: 'background-execution',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-declared',
      proofState: 'manual-required',
      runtimeOwner: 'apple-background-mode',
      proofRequirement: 'background-execution remains manual-required until Apple artifacts change it',
      claimBoundary: 'background-execution remains manual-required until Apple artifacts change it',
    },
    {
      surface: 'provisioning-profile',
      parentCapability: 'signing-entitlements',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'signing-required',
      runtimeOwner: 'apple-signing',
      proofRequirement: 'provisioning-profile remains signing-required until Apple artifacts change it',
      claimBoundary: 'provisioning-profile remains signing-required until Apple artifacts change it',
    },
    {
      surface: 'supervision-state',
      parentCapability: 'package-lifecycle',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'device-proof-required',
      runtimeOwner: 'apple-device-proof',
      proofRequirement: 'supervision-state remains device-proof-required until Apple artifacts change it',
      claimBoundary: 'supervision-state remains device-proof-required until Apple artifacts change it',
    },
    {
      surface: 'signing-entitlements',
      parentCapability: 'signing-entitlements',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'signing-required',
      runtimeOwner: 'apple-signing',
      proofRequirement: 'signing-entitlements remains signing-required until Apple artifacts change it',
      claimBoundary: 'signing-entitlements remains signing-required until Apple artifacts change it',
    },
    {
      surface: 'testflight-distribution',
      parentCapability: 'testflight-distribution',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'device-proof-required',
      runtimeOwner: 'apple-testflight',
      proofRequirement: 'testflight-distribution remains device-proof-required until Apple artifacts change it',
      claimBoundary: 'testflight-distribution remains device-proof-required until Apple artifacts change it',
    },
    {
      surface: 'physical-device-proof',
      parentCapability: 'package-lifecycle',
      parentCapabilityStatus: 'manual-required',
      declarationState: 'not-applicable',
      proofState: 'device-proof-required',
      runtimeOwner: 'apple-device-proof',
      proofRequirement: 'physical-device-proof remains device-proof-required until Apple artifacts change it',
      claimBoundary: 'physical-device-proof remains device-proof-required until Apple artifacts change it',
    },
    {
      surface: 'app-store-distribution',
      parentCapability: 'store-distribution',
      parentCapabilityStatus: 'planned',
      declarationState: 'not-applicable',
      proofState: 'planned',
      runtimeOwner: 'app-store-connect',
      proofRequirement: 'app-store-distribution remains planned until Apple artifacts change it',
      claimBoundary: 'app-store-distribution remains planned until Apple artifacts change it',
    },
  ],
  packageLifecycleProofs: [
    {
      phase: 'xcode-project-target',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'ios-xcode-project',
      proofRequirement: 'xcode-project-target proof state is ci-mechanical-proof',
      claimBoundary:
        'xcode-project-target does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'bundle-identifier',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'ios-xcode-project',
      proofRequirement: 'bundle-identifier proof state is ci-mechanical-proof',
      claimBoundary:
        'bundle-identifier does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'simulator-build-script',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'ios-simulator-build-script',
      proofRequirement: 'simulator-build-script proof state is ci-mechanical-proof',
      claimBoundary:
        'simulator-build-script does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'status-view',
      proofState: 'simulator-scaffold',
      runtimeOwner: 'ios-swift-scaffold',
      proofRequirement: 'status-view proof state is simulator-scaffold',
      claimBoundary: 'status-view does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'info-plist',
      proofState: 'ci-mechanical-proof',
      runtimeOwner: 'ios-info-plist',
      proofRequirement: 'info-plist proof state is ci-mechanical-proof',
      claimBoundary: 'info-plist does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'simulator-build',
      proofState: 'manual-required',
      runtimeOwner: 'ios-simulator-build-script',
      proofRequirement: 'simulator-build proof state is manual-required',
      claimBoundary:
        'simulator-build does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'simulator-launch',
      proofState: 'manual-required',
      runtimeOwner: 'apple-simulator-host',
      proofRequirement: 'simulator-launch proof state is manual-required',
      claimBoundary:
        'simulator-launch does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'device-install',
      proofState: 'device-proof-required',
      runtimeOwner: 'apple-device-proof',
      proofRequirement: 'device-install proof state is device-proof-required',
      claimBoundary:
        'device-install does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'device-launch',
      proofState: 'device-proof-required',
      runtimeOwner: 'apple-device-proof',
      proofRequirement: 'device-launch proof state is device-proof-required',
      claimBoundary:
        'device-launch does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'testflight-install',
      proofState: 'device-proof-required',
      runtimeOwner: 'apple-testflight',
      proofRequirement: 'testflight-install proof state is device-proof-required',
      claimBoundary:
        'testflight-install does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'signing-profile',
      proofState: 'signing-required',
      runtimeOwner: 'apple-signing',
      proofRequirement: 'signing-profile proof state is signing-required',
      claimBoundary:
        'signing-profile does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'entitlement-review',
      proofState: 'entitlement-required',
      runtimeOwner: 'apple-entitlement',
      proofRequirement: 'entitlement-review proof state is entitlement-required',
      claimBoundary:
        'entitlement-review does not upgrade iOS child capability without entitlement signing or device evidence',
    },
    {
      phase: 'recovery-behavior',
      proofState: 'not-implemented',
      runtimeOwner: 'apple-background-mode',
      proofRequirement: 'recovery-behavior proof state is not-implemented',
      claimBoundary:
        'recovery-behavior does not upgrade iOS child capability without entitlement signing or device evidence',
    },
  ],
  claimBoundaries: {
    simulatorPackage: 'Xcode project target, bundle id, plist, status view, and package script are source proof only',
    launchAvailability:
      'simulator and physical-device launch availability remain manual-required or device-proof-required without Apple host or device artifacts',
    familyControls: 'Family Controls remains entitlement-required without Apple approval and device artifacts',
    deviceActivity: 'DeviceActivity remains entitlement-required without schedule and event artifacts',
    screenTime: 'Screen Time API remains entitlement-required without authorization and behavior artifacts',
    networkExtension: 'Network Extension remains entitlement-required without filtering artifacts',
    notifications: 'notification authorization and delivery remain manual-required',
    backgroundExecution: 'background execution remains manual-required without UIBackgroundModes and device proof',
    recoveryBehavior:
      'launch recovery remains not-implemented; no iOS daemon, relaunch, or persistent background recovery is claimed',
    provisioningProfile:
      'provisioning remains manual-required without Apple signing credentials, provisioning profile artifacts, and install evidence',
    supervision: 'supervision remains manual-required without supervised-device enrollment and device artifacts',
    signingEntitlements: 'signing and entitlements remain signing-required; simulator script disables signing',
    testflight: 'TestFlight and App Store distribution remain device-proof-required or planned',
    deviceProof: 'physical-device install and runtime behavior remain device-proof-required',
    capabilityOnlyState:
      'iOS child runtime remains capability-only; no hidden daemon or persistent background service is claimed',
    externalTransport: 'no external LAN or WebSocket iOS child-agent transport is claimed',
  },
  updatedAt: '2026-05-31T00:00:00.000Z',
} as const satisfies GeneratedChildIosEntitlementCapabilityReadModelShape;
