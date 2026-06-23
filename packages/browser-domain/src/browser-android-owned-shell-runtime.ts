import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserEvidenceSchemaVersion } from '@ocentra-parent/schema-domain/browser-schemas';

export const BrowserAndroidOwnedShellRuntimeStateSchema = withParser(
  Schema.Literal('physical-visible-owned-shell', 'emulator-browser-role-routing', 'manual-required')
);
export const BrowserAndroidOwnedShellRuntimeProofRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAndroidOwnedShellRuntimeProofRef')
);
export const BrowserAndroidOwnedShellRuntimeReasonSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAndroidOwnedShellRuntimeReason')
);

const BrowserAndroidOwnedShellProofDeviceSchema = Schema.Struct({
  serialKind: Schema.Literal('emulator', 'physical-or-network-adb-device'),
  proofLaunchedEmulator: Schema.Boolean,
  packageInstalled: Schema.Boolean,
  explicitActivityStartObserved: Schema.Boolean,
  explicitActivityResumedObserved: Schema.Boolean,
  explicitActivityFocusedObserved: Schema.Boolean,
  explicitLaunchObserved: Schema.Boolean,
  localProofPageObserved: Schema.Boolean,
  implicitViewIntentLaunchObserved: Schema.Boolean,
  uiTreeCaptured: Schema.Boolean,
  uiTreeRawPersisted: Schema.Boolean,
  screenshotCaptured: Schema.Boolean,
  screenshotPersisted: Schema.Boolean,
  exactUrlPolicyClaimed: Schema.Boolean,
  knownActiveTabProofClaimed: Schema.Boolean,
  deviceOwnerEnrollmentClaimed: Schema.Boolean,
  deviceOwnerPolicyMutationClaimed: Schema.Boolean,
  browserRoleAssignmentClaimed: Schema.Boolean,
  androidOwnedBrowserRoutingEnforcementClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

const BrowserAndroidOwnedShellProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    proofId: Schema.Literal('browser-platform-android-owned-shell-proof'),
    generatedAt: ActivityTimestampSchema,
    hostProofSummary: Schema.Struct({
      physicalDeviceProofObserved: Schema.Boolean,
      physicalDeviceInstallObserved: Schema.Boolean,
      physicalDeviceActivityStartObserved: Schema.Boolean,
      physicalDeviceExplicitLaunchObserved: Schema.Boolean,
      physicalDeviceScreenshotCaptured: Schema.Boolean,
      physicalDeviceUiTreeCaptured: Schema.Boolean,
      deviceOwnerProofLimitedToProofLaunchedEmulator: Schema.Boolean,
      deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator: Schema.Boolean,
      androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator: Schema.Boolean,
      exactUrlPolicyClaimed: Schema.Boolean,
      knownActiveTabProofClaimed: Schema.Boolean,
      deviceOwnerEnrollmentClaimed: Schema.Boolean,
      deviceOwnerPolicyMutationClaimed: Schema.Boolean,
      androidOwnedBrowserRoutingEnforcementClaimed: Schema.Boolean,
      browserRoleAssignmentClaimed: Schema.Boolean,
      vpnDnsBrowserProofClaimed: Schema.Boolean,
      usageStatsRouteProofClaimed: Schema.Boolean,
      accessibilityRouteProofClaimed: Schema.Boolean,
      enforcementClaimed: Schema.Boolean,
      physicalDeviceClaimBoundary: NonEmptyStringSchema,
      resultState: NonEmptyStringSchema,
    }),
    devices: Schema.Array(BrowserAndroidOwnedShellProofDeviceSchema),
  })
);

const BrowserAndroidOwnedShellRuntimeRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
  runtimeState: BrowserAndroidOwnedShellRuntimeStateSchema,
  observedAt: ActivityTimestampSchema,
  sourceProofRef: BrowserAndroidOwnedShellRuntimeProofRefSchema,
  reasonCode: BrowserAndroidOwnedShellRuntimeReasonSchema,
  physicalDeviceObserved: Schema.Boolean,
  physicalInstallObserved: Schema.Boolean,
  physicalExplicitLaunchObserved: Schema.Boolean,
  physicalVisibleScreenshotObserved: Schema.Boolean,
  physicalUiTreeObserved: Schema.Boolean,
  emulatorDeviceOwnerOnly: Schema.Boolean,
  emulatorBrowserRoleRoutingOnly: Schema.Boolean,
  exactUrlPolicyClaimed: Schema.Boolean,
  knownActiveTabProofClaimed: Schema.Boolean,
  physicalDeviceOwnerClaimed: Schema.Boolean,
  physicalBrowserRoleRoutingClaimed: Schema.Boolean,
  vpnDnsBrowserProofClaimed: Schema.Boolean,
  usageStatsRouteProofClaimed: Schema.Boolean,
  accessibilityRouteProofClaimed: Schema.Boolean,
  finalPolicyExecutionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserAndroidOwnedShellRuntimeRowCandidate = Infer<typeof BrowserAndroidOwnedShellRuntimeRowBaseSchema>;

export const BrowserAndroidOwnedShellRuntimeRowSchema = withParser(
  BrowserAndroidOwnedShellRuntimeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        androidOwnedShellRuntimeRowIsHonest(row) ||
        'Expected Android owned-shell runtime rows to preserve physical/current-browser no-claim boundaries'
    )
  )
);

export const BrowserAndroidOwnedShellRuntimeReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    sourceProofGeneratedAt: ActivityTimestampSchema,
    rows: Schema.Array(BrowserAndroidOwnedShellRuntimeRowSchema),
    physicalVisibleRows: Schema.Number,
    manualRequiredRows: Schema.Number,
    productClaimed: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (readModel) =>
        readModel.productClaimed === false ||
        'Expected Android owned-shell runtime read model to keep product completion unclaimed'
    )
  )
);

export type BrowserAndroidOwnedShellRuntimeState = Infer<typeof BrowserAndroidOwnedShellRuntimeStateSchema>;
export type BrowserAndroidOwnedShellRuntimeRow = Infer<typeof BrowserAndroidOwnedShellRuntimeRowSchema>;
export type BrowserAndroidOwnedShellRuntimeReadModel = Infer<typeof BrowserAndroidOwnedShellRuntimeReadModelSchema>;

export const BrowserAndroidOwnedShellRuntimeState = {
  PhysicalVisibleOwnedShell: BrowserAndroidOwnedShellRuntimeStateSchema.parse('physical-visible-owned-shell'),
  EmulatorBrowserRoleRouting: BrowserAndroidOwnedShellRuntimeStateSchema.parse('emulator-browser-role-routing'),
  ManualRequired: BrowserAndroidOwnedShellRuntimeStateSchema.parse('manual-required'),
} as const;

export function buildBrowserAndroidOwnedShellRuntimeReadModel(
  sourceProof: unknown
): BrowserAndroidOwnedShellRuntimeReadModel {
  const proof = BrowserAndroidOwnedShellProofSchema.parse(sourceProof);
  const rows = [physicalRuntimeRow(proof), emulatorRoutingRow(proof), manualRequiredRow(proof)].filter(
    (row): row is BrowserAndroidOwnedShellRuntimeRow => row !== null
  );

  return BrowserAndroidOwnedShellRuntimeReadModelSchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    generatedAt: proof.generatedAt,
    sourceProofGeneratedAt: proof.generatedAt,
    rows,
    physicalVisibleRows: rows.filter((row) => row.runtimeState === 'physical-visible-owned-shell').length,
    manualRequiredRows: rows.filter((row) => row.runtimeState === 'manual-required').length,
    productClaimed: false,
  });
}

function physicalRuntimeRow(
  proof: Infer<typeof BrowserAndroidOwnedShellProofSchema>
): BrowserAndroidOwnedShellRuntimeRow | null {
  const physicalDevice = proof.devices.find((device) => device.serialKind === 'physical-or-network-adb-device');
  if (physicalDevice === undefined) {
    return null;
  }

  return BrowserAndroidOwnedShellRuntimeRowSchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    runtimeState:
      physicalDevice.packageInstalled &&
      physicalDevice.explicitLaunchObserved &&
      physicalDevice.screenshotCaptured &&
      physicalDevice.uiTreeCaptured
        ? 'physical-visible-owned-shell'
        : 'manual-required',
    observedAt: proof.generatedAt,
    sourceProofRef: 'browser-platform-android-owned-shell-proof',
    reasonCode: 'physical-android-owned-shell-visible-runtime-proof',
    physicalDeviceObserved: proof.hostProofSummary.physicalDeviceProofObserved,
    physicalInstallObserved: physicalDevice.packageInstalled,
    physicalExplicitLaunchObserved: physicalDevice.explicitLaunchObserved,
    physicalVisibleScreenshotObserved: physicalDevice.screenshotCaptured,
    physicalUiTreeObserved: physicalDevice.uiTreeCaptured,
    emulatorDeviceOwnerOnly: proof.hostProofSummary.deviceOwnerProofLimitedToProofLaunchedEmulator,
    emulatorBrowserRoleRoutingOnly: proof.hostProofSummary.androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator,
    exactUrlPolicyClaimed: physicalDevice.exactUrlPolicyClaimed,
    knownActiveTabProofClaimed: physicalDevice.knownActiveTabProofClaimed,
    physicalDeviceOwnerClaimed:
      physicalDevice.deviceOwnerEnrollmentClaimed || physicalDevice.deviceOwnerPolicyMutationClaimed,
    physicalBrowserRoleRoutingClaimed:
      physicalDevice.browserRoleAssignmentClaimed || physicalDevice.androidOwnedBrowserRoutingEnforcementClaimed,
    vpnDnsBrowserProofClaimed: proof.hostProofSummary.vpnDnsBrowserProofClaimed,
    usageStatsRouteProofClaimed: proof.hostProofSummary.usageStatsRouteProofClaimed,
    accessibilityRouteProofClaimed: proof.hostProofSummary.accessibilityRouteProofClaimed,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: physicalDevice.enforcementClaimed,
  });
}

function emulatorRoutingRow(
  proof: Infer<typeof BrowserAndroidOwnedShellProofSchema>
): BrowserAndroidOwnedShellRuntimeRow | null {
  const emulatorDevice = proof.devices.find(
    (device) => device.serialKind === 'emulator' && device.proofLaunchedEmulator === true
  );
  if (emulatorDevice === undefined || !emulatorDevice.implicitViewIntentLaunchObserved) {
    return null;
  }

  return BrowserAndroidOwnedShellRuntimeRowSchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    runtimeState: 'emulator-browser-role-routing',
    observedAt: proof.generatedAt,
    sourceProofRef: 'browser-platform-android-owned-shell-proof',
    reasonCode: 'emulator-browser-role-routing-proof-not-physical-default-browser',
    physicalDeviceObserved: false,
    physicalInstallObserved: false,
    physicalExplicitLaunchObserved: false,
    physicalVisibleScreenshotObserved: false,
    physicalUiTreeObserved: false,
    emulatorDeviceOwnerOnly: proof.hostProofSummary.deviceOwnerProofLimitedToProofLaunchedEmulator,
    emulatorBrowserRoleRoutingOnly: proof.hostProofSummary.androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator,
    exactUrlPolicyClaimed: emulatorDevice.exactUrlPolicyClaimed,
    knownActiveTabProofClaimed: emulatorDevice.knownActiveTabProofClaimed,
    physicalDeviceOwnerClaimed: false,
    physicalBrowserRoleRoutingClaimed: false,
    vpnDnsBrowserProofClaimed: proof.hostProofSummary.vpnDnsBrowserProofClaimed,
    usageStatsRouteProofClaimed: proof.hostProofSummary.usageStatsRouteProofClaimed,
    accessibilityRouteProofClaimed: proof.hostProofSummary.accessibilityRouteProofClaimed,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: emulatorDevice.enforcementClaimed,
  });
}

function manualRequiredRow(
  proof: Infer<typeof BrowserAndroidOwnedShellProofSchema>
): BrowserAndroidOwnedShellRuntimeRow {
  return BrowserAndroidOwnedShellRuntimeRowSchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    runtimeState: 'manual-required',
    observedAt: proof.generatedAt,
    sourceProofRef: 'browser-platform-android-owned-shell-proof',
    reasonCode: 'physical-device-owner-browser-role-exact-url-active-tab-and-enforcement-proof-required',
    physicalDeviceObserved: proof.hostProofSummary.physicalDeviceProofObserved,
    physicalInstallObserved: proof.hostProofSummary.physicalDeviceInstallObserved,
    physicalExplicitLaunchObserved: proof.hostProofSummary.physicalDeviceExplicitLaunchObserved,
    physicalVisibleScreenshotObserved: proof.hostProofSummary.physicalDeviceScreenshotCaptured,
    physicalUiTreeObserved: proof.hostProofSummary.physicalDeviceUiTreeCaptured,
    emulatorDeviceOwnerOnly: proof.hostProofSummary.deviceOwnerProofLimitedToProofLaunchedEmulator,
    emulatorBrowserRoleRoutingOnly: proof.hostProofSummary.androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator,
    exactUrlPolicyClaimed: proof.hostProofSummary.exactUrlPolicyClaimed,
    knownActiveTabProofClaimed: proof.hostProofSummary.knownActiveTabProofClaimed,
    physicalDeviceOwnerClaimed: false,
    physicalBrowserRoleRoutingClaimed: false,
    vpnDnsBrowserProofClaimed: proof.hostProofSummary.vpnDnsBrowserProofClaimed,
    usageStatsRouteProofClaimed: proof.hostProofSummary.usageStatsRouteProofClaimed,
    accessibilityRouteProofClaimed: proof.hostProofSummary.accessibilityRouteProofClaimed,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: proof.hostProofSummary.enforcementClaimed,
  });
}

function androidOwnedShellRuntimeRowIsHonest(row: BrowserAndroidOwnedShellRuntimeRowCandidate): boolean {
  return (
    row.exactUrlPolicyClaimed === false &&
    row.knownActiveTabProofClaimed === false &&
    row.physicalDeviceOwnerClaimed === false &&
    row.physicalBrowserRoleRoutingClaimed === false &&
    row.vpnDnsBrowserProofClaimed === false &&
    row.usageStatsRouteProofClaimed === false &&
    row.accessibilityRouteProofClaimed === false &&
    row.finalPolicyExecutionClaimed === false &&
    row.enforcementClaimed === false &&
    physicalRuntimeRowsNeedVisiblePhysicalProof(row) &&
    emulatorRoutingRowsStayEmulatorScoped(row)
  );
}

function physicalRuntimeRowsNeedVisiblePhysicalProof(row: BrowserAndroidOwnedShellRuntimeRowCandidate): boolean {
  return (
    row.runtimeState !== 'physical-visible-owned-shell' ||
    (row.physicalDeviceObserved &&
      row.physicalInstallObserved &&
      row.physicalExplicitLaunchObserved &&
      row.physicalVisibleScreenshotObserved &&
      row.physicalUiTreeObserved)
  );
}

function emulatorRoutingRowsStayEmulatorScoped(row: BrowserAndroidOwnedShellRuntimeRowCandidate): boolean {
  return (
    row.runtimeState !== 'emulator-browser-role-routing' ||
    (!row.physicalDeviceObserved &&
      !row.physicalInstallObserved &&
      !row.physicalExplicitLaunchObserved &&
      row.emulatorDeviceOwnerOnly &&
      row.emulatorBrowserRoleRoutingOnly)
  );
}
