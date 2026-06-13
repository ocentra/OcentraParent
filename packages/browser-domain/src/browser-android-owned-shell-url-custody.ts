import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { BrowserEvidenceSchemaVersion } from './browser-schemas';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';

export const BrowserAndroidOwnedShellUrlCustodyStateSchema = withParser(
  Schema.Literal('physical-owned-shell-request-url-ref', 'manual-required')
);
export const BrowserAndroidOwnedShellUrlCustodyProofRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAndroidOwnedShellUrlCustodyProofRef')
);
export const BrowserAndroidOwnedShellRequestedUrlRefSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAndroidOwnedShellRequestedUrlRef')
);
export const BrowserAndroidOwnedShellUrlCustodyReasonSchema = withParser(
  brandedNonEmptyStringSchema('BrowserAndroidOwnedShellUrlCustodyReason')
);

const BrowserAndroidOwnedShellUrlCustodyDeviceSchema = Schema.Struct({
  serialKind: Schema.Literal('emulator', 'physical-or-network-adb-device'),
  proofLaunchedEmulator: Schema.Boolean,
  packageInstalled: Schema.Boolean,
  explicitLaunchObserved: Schema.Boolean,
  localProofPageObserved: Schema.Boolean,
  uiTreeCaptured: Schema.Boolean,
  screenshotCaptured: Schema.Boolean,
  rawUrlPersisted: Schema.Boolean,
  exactUrlPolicyClaimed: Schema.Boolean,
  knownActiveTabProofClaimed: Schema.Boolean,
  deviceOwnerEnrollmentClaimed: Schema.Boolean,
  deviceOwnerPolicyMutationClaimed: Schema.Boolean,
  browserRoleAssignmentClaimed: Schema.Boolean,
  androidOwnedBrowserRoutingEnforcementClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

const BrowserAndroidOwnedShellUrlCustodySourceBoundarySchema = Schema.Struct({
  webViewDeclared: Schema.Boolean,
  browsableViewIntentDeclared: Schema.Boolean,
  deviceOwnerPolicyMutationDeclared: Schema.Boolean,
  accessibilityServiceDeclared: Schema.Boolean,
  vpnServiceDeclared: Schema.Boolean,
  usageStatsPermissionDeclared: Schema.Boolean,
});

const BrowserAndroidOwnedShellUrlCustodyProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    proofId: Schema.Literal('browser-platform-android-owned-shell-proof'),
    generatedAt: ActivityTimestampSchema,
    proofUrlRef: Schema.Union(NonEmptyStringSchema, Schema.Null),
    proofUrlPersisted: Schema.Boolean,
    hostProofSummary: Schema.Struct({
      physicalDeviceProofObserved: Schema.Boolean,
      physicalDeviceInstallObserved: Schema.Boolean,
      physicalDeviceExplicitLaunchObserved: Schema.Boolean,
      physicalDeviceScreenshotCaptured: Schema.Boolean,
      physicalDeviceUiTreeCaptured: Schema.Boolean,
      deviceOwnerProofLimitedToProofLaunchedEmulator: Schema.Boolean,
      deviceOwnerPolicyMutationLimitedToProofLaunchedEmulator: Schema.Boolean,
      androidBrowserRoleAssignmentLimitedToProofLaunchedEmulator: Schema.Boolean,
      exactUrlPolicyClaimed: Schema.Boolean,
      knownActiveTabProofClaimed: Schema.Boolean,
      vpnDnsBrowserProofClaimed: Schema.Boolean,
      usageStatsRouteProofClaimed: Schema.Boolean,
      accessibilityRouteProofClaimed: Schema.Boolean,
      enforcementClaimed: Schema.Boolean,
    }),
    devices: Schema.Array(BrowserAndroidOwnedShellUrlCustodyDeviceSchema),
    sourceBoundary: BrowserAndroidOwnedShellUrlCustodySourceBoundarySchema,
  })
);

const BrowserAndroidOwnedShellUrlCustodyRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
  custodyState: BrowserAndroidOwnedShellUrlCustodyStateSchema,
  observedAt: ActivityTimestampSchema,
  sourceProofRef: BrowserAndroidOwnedShellUrlCustodyProofRefSchema,
  requestedUrlRef: Schema.Union(BrowserAndroidOwnedShellRequestedUrlRefSchema, Schema.Null),
  reasonCode: BrowserAndroidOwnedShellUrlCustodyReasonSchema,
  physicalDeviceObserved: Schema.Boolean,
  physicalInstallObserved: Schema.Boolean,
  physicalExplicitLaunchObserved: Schema.Boolean,
  physicalVisibleScreenshotObserved: Schema.Boolean,
  physicalUiTreeObserved: Schema.Boolean,
  ownedShellViewIntentDeclared: Schema.Boolean,
  ownedShellWebViewDeclared: Schema.Boolean,
  localProofPageObserved: Schema.Boolean,
  rawUrlPersisted: Schema.Boolean,
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

type BrowserAndroidOwnedShellUrlCustodyRowCandidate = Infer<typeof BrowserAndroidOwnedShellUrlCustodyRowBaseSchema>;

export const BrowserAndroidOwnedShellUrlCustodyRowSchema = withParser(
  BrowserAndroidOwnedShellUrlCustodyRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        androidOwnedShellUrlCustodyRowIsHonest(row) ||
        'Expected Android owned-shell URL custody rows to require physical owned-shell evidence and preserve no-claim boundaries'
    )
  )
);

export const BrowserAndroidOwnedShellUrlCustodyReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    sourceProofGeneratedAt: ActivityTimestampSchema,
    rows: Schema.Array(BrowserAndroidOwnedShellUrlCustodyRowSchema),
    physicalRequestedUrlRefRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    manualRequiredRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    exactActiveTabClaimed: Schema.Literal(false),
    policyExecutionClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    productClaimed: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (readModel) =>
        readModel.physicalRequestedUrlRefRows ===
          readModel.rows.filter((row) => row.custodyState === 'physical-owned-shell-request-url-ref').length &&
        readModel.manualRequiredRows === readModel.rows.filter((row) => row.custodyState === 'manual-required').length
    )
  )
);

export type BrowserAndroidOwnedShellUrlCustodyState = Infer<typeof BrowserAndroidOwnedShellUrlCustodyStateSchema>;
export type BrowserAndroidOwnedShellUrlCustodyRow = Infer<typeof BrowserAndroidOwnedShellUrlCustodyRowSchema>;
export type BrowserAndroidOwnedShellUrlCustodyReadModel = Infer<
  typeof BrowserAndroidOwnedShellUrlCustodyReadModelSchema
>;

export const BrowserAndroidOwnedShellUrlCustodyState = {
  PhysicalOwnedShellRequestUrlRef: BrowserAndroidOwnedShellUrlCustodyStateSchema.parse(
    'physical-owned-shell-request-url-ref'
  ),
  ManualRequired: BrowserAndroidOwnedShellUrlCustodyStateSchema.parse('manual-required'),
} as const;

export function buildBrowserAndroidOwnedShellUrlCustodyReadModel(
  sourceProof: unknown
): BrowserAndroidOwnedShellUrlCustodyReadModel {
  const proof = BrowserAndroidOwnedShellUrlCustodyProofSchema.parse(sourceProof);
  const rows = [physicalRequestedUrlRefRow(proof), manualRequiredRow(proof)].filter(
    (row): row is BrowserAndroidOwnedShellUrlCustodyRow => row !== null
  );

  return BrowserAndroidOwnedShellUrlCustodyReadModelSchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    generatedAt: proof.generatedAt,
    sourceProofGeneratedAt: proof.generatedAt,
    rows,
    physicalRequestedUrlRefRows: rows.filter((row) => row.custodyState === 'physical-owned-shell-request-url-ref')
      .length,
    manualRequiredRows: rows.filter((row) => row.custodyState === 'manual-required').length,
    exactActiveTabClaimed: false,
    policyExecutionClaimed: false,
    enforcementClaimed: false,
    productClaimed: false,
  });
}

function physicalRequestedUrlRefRow(
  proof: Infer<typeof BrowserAndroidOwnedShellUrlCustodyProofSchema>
): BrowserAndroidOwnedShellUrlCustodyRow | null {
  const physicalDevice = proof.devices.find((device) => device.serialKind === 'physical-or-network-adb-device');
  if (physicalDevice === undefined || proof.proofUrlRef === null) {
    return null;
  }
  if (!physicalDeviceHasUrlCustodyProof(proof, physicalDevice)) {
    return null;
  }

  return BrowserAndroidOwnedShellUrlCustodyRowSchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    custodyState: 'physical-owned-shell-request-url-ref',
    observedAt: proof.generatedAt,
    sourceProofRef: 'browser-platform-android-owned-shell-proof',
    requestedUrlRef: proof.proofUrlRef,
    reasonCode: 'physical-owned-shell-view-intent-url-ref-custody',
    physicalDeviceObserved: proof.hostProofSummary.physicalDeviceProofObserved,
    physicalInstallObserved: physicalDevice.packageInstalled,
    physicalExplicitLaunchObserved: physicalDevice.explicitLaunchObserved,
    physicalVisibleScreenshotObserved: physicalDevice.screenshotCaptured,
    physicalUiTreeObserved: physicalDevice.uiTreeCaptured,
    ownedShellViewIntentDeclared: proof.sourceBoundary.browsableViewIntentDeclared,
    ownedShellWebViewDeclared: proof.sourceBoundary.webViewDeclared,
    localProofPageObserved: physicalDevice.localProofPageObserved,
    rawUrlPersisted: physicalDevice.rawUrlPersisted || proof.proofUrlPersisted,
    exactUrlPolicyClaimed: physicalDevice.exactUrlPolicyClaimed || proof.hostProofSummary.exactUrlPolicyClaimed,
    knownActiveTabProofClaimed:
      physicalDevice.knownActiveTabProofClaimed || proof.hostProofSummary.knownActiveTabProofClaimed,
    physicalDeviceOwnerClaimed:
      physicalDevice.deviceOwnerEnrollmentClaimed || physicalDevice.deviceOwnerPolicyMutationClaimed,
    physicalBrowserRoleRoutingClaimed:
      physicalDevice.browserRoleAssignmentClaimed || physicalDevice.androidOwnedBrowserRoutingEnforcementClaimed,
    vpnDnsBrowserProofClaimed: proof.hostProofSummary.vpnDnsBrowserProofClaimed,
    usageStatsRouteProofClaimed: proof.hostProofSummary.usageStatsRouteProofClaimed,
    accessibilityRouteProofClaimed: proof.hostProofSummary.accessibilityRouteProofClaimed,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: physicalDevice.enforcementClaimed || proof.hostProofSummary.enforcementClaimed,
  });
}

function physicalDeviceHasUrlCustodyProof(
  proof: Infer<typeof BrowserAndroidOwnedShellUrlCustodyProofSchema>,
  physicalDevice: Infer<typeof BrowserAndroidOwnedShellUrlCustodyDeviceSchema>
): boolean {
  return (
    proof.hostProofSummary.physicalDeviceProofObserved &&
    physicalDevice.packageInstalled &&
    physicalDevice.explicitLaunchObserved &&
    physicalDevice.screenshotCaptured &&
    physicalDevice.uiTreeCaptured &&
    proof.sourceBoundary.browsableViewIntentDeclared &&
    proof.sourceBoundary.webViewDeclared &&
    physicalDevice.localProofPageObserved
  );
}

function manualRequiredRow(
  proof: Infer<typeof BrowserAndroidOwnedShellUrlCustodyProofSchema>
): BrowserAndroidOwnedShellUrlCustodyRow {
  return BrowserAndroidOwnedShellUrlCustodyRowSchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    custodyState: 'manual-required',
    observedAt: proof.generatedAt,
    sourceProofRef: 'browser-platform-android-owned-shell-proof',
    requestedUrlRef: null,
    reasonCode: 'android-active-tab-policy-execution-and-enforcement-proof-required',
    physicalDeviceObserved: proof.hostProofSummary.physicalDeviceProofObserved,
    physicalInstallObserved: proof.hostProofSummary.physicalDeviceInstallObserved,
    physicalExplicitLaunchObserved: proof.hostProofSummary.physicalDeviceExplicitLaunchObserved,
    physicalVisibleScreenshotObserved: proof.hostProofSummary.physicalDeviceScreenshotCaptured,
    physicalUiTreeObserved: proof.hostProofSummary.physicalDeviceUiTreeCaptured,
    ownedShellViewIntentDeclared: proof.sourceBoundary.browsableViewIntentDeclared,
    ownedShellWebViewDeclared: proof.sourceBoundary.webViewDeclared,
    localProofPageObserved: false,
    rawUrlPersisted: proof.proofUrlPersisted,
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

function androidOwnedShellUrlCustodyRowIsHonest(row: BrowserAndroidOwnedShellUrlCustodyRowCandidate): boolean {
  return (
    row.rawUrlPersisted === false &&
    row.exactUrlPolicyClaimed === false &&
    row.knownActiveTabProofClaimed === false &&
    row.physicalDeviceOwnerClaimed === false &&
    row.physicalBrowserRoleRoutingClaimed === false &&
    row.vpnDnsBrowserProofClaimed === false &&
    row.usageStatsRouteProofClaimed === false &&
    row.accessibilityRouteProofClaimed === false &&
    row.finalPolicyExecutionClaimed === false &&
    row.enforcementClaimed === false &&
    physicalUrlRowsNeedOwnedShellProof(row)
  );
}

function physicalUrlRowsNeedOwnedShellProof(row: BrowserAndroidOwnedShellUrlCustodyRowCandidate): boolean {
  return (
    row.custodyState !== 'physical-owned-shell-request-url-ref' ||
    (row.requestedUrlRef !== null &&
      row.physicalDeviceObserved &&
      row.physicalInstallObserved &&
      row.physicalExplicitLaunchObserved &&
      row.physicalVisibleScreenshotObserved &&
      row.physicalUiTreeObserved &&
      row.ownedShellViewIntentDeclared &&
      row.ownedShellWebViewDeclared &&
      row.localProofPageObserved)
  );
}

