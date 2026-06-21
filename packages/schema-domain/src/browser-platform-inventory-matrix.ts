import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  BrowserCapabilityStatusSchema,
  BrowserChannelSchema,
  BrowserEvidenceSchemaVersion,
  BrowserFamilySchema,
} from './browser-schemas';
import {
  BrowserActiveTabCapabilitySchema,
  BrowserExactUrlCapabilitySchema,
  BrowserInventoryInstallStateSchema,
  BrowserInventoryReasonCodeSchema,
  BrowserManagedProfileStateSchema,
  BrowserManagementTierSchema,
  BrowserProductNameSchema,
  BrowserSupportTierSchema,
  BrowserUnmanagedFallbackCapabilitySchema,
} from './browser-inventory-schemas';

export const BrowserInventoryPlatformSchema = withParser(
  Schema.Literal('windows', 'macos', 'linux', 'android', 'ios', 'unknown')
);
export const BrowserInventoryPlatformProofStateSchema = withParser(
  Schema.Literal('host-observed', 'fixture-backed', 'manual-required', 'unsupported', 'not-claimed')
);
export const BrowserInventoryPlatformProofRequirementSchema = withParser(
  brandedNonEmptyStringSchema('BrowserInventoryPlatformProofRequirement')
);

const BrowserInventoryPlatformMatrixEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
  platform: BrowserInventoryPlatformSchema,
  browserFamily: BrowserFamilySchema,
  browserChannel: BrowserChannelSchema,
  productName: BrowserProductNameSchema,
  installState: BrowserInventoryInstallStateSchema,
  managementTier: BrowserManagementTierSchema,
  supportTier: BrowserSupportTierSchema,
  exactUrlCapability: BrowserExactUrlCapabilitySchema,
  activeTabCapability: BrowserActiveTabCapabilitySchema,
  managedProfileState: BrowserManagedProfileStateSchema,
  unmanagedFallbackCapability: BrowserUnmanagedFallbackCapabilitySchema,
  capabilityStatus: BrowserCapabilityStatusSchema,
  proofState: BrowserInventoryPlatformProofStateSchema,
  reasonCode: BrowserInventoryReasonCodeSchema,
  proofRequirement: Schema.Union(BrowserInventoryPlatformProofRequirementSchema, Schema.Null),
});

type BrowserInventoryPlatformMatrixEntryCandidate = Infer<typeof BrowserInventoryPlatformMatrixEntryBaseSchema>;

export const BrowserInventoryPlatformMatrixEntrySchema = withParser(
  BrowserInventoryPlatformMatrixEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        browserInventoryPlatformMatrixEntryIsHonest(entry) ||
        'Expected browser platform inventory matrix entries to preserve platform-specific no-claim boundaries'
    )
  )
);

export const BrowserInventoryPlatformMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    entries: Schema.Array(BrowserInventoryPlatformMatrixEntrySchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        new Set(matrix.entries.map((entry) => platformMatrixEntryKey(entry))).size === matrix.entries.length ||
        'Expected browser platform inventory matrix entries to be unique by platform, family, channel, and product'
    )
  )
);

export type BrowserInventoryPlatform = Infer<typeof BrowserInventoryPlatformSchema>;
export type BrowserInventoryPlatformProofState = Infer<typeof BrowserInventoryPlatformProofStateSchema>;
export type BrowserInventoryPlatformProofRequirement = Infer<typeof BrowserInventoryPlatformProofRequirementSchema>;
export type BrowserInventoryPlatformMatrixEntry = Infer<typeof BrowserInventoryPlatformMatrixEntrySchema>;
export type BrowserInventoryPlatformMatrix = Infer<typeof BrowserInventoryPlatformMatrixSchema>;

export const BrowserInventoryPlatform = {
  Windows: BrowserInventoryPlatformSchema.parse('windows'),
  Macos: BrowserInventoryPlatformSchema.parse('macos'),
  Linux: BrowserInventoryPlatformSchema.parse('linux'),
  Android: BrowserInventoryPlatformSchema.parse('android'),
  Ios: BrowserInventoryPlatformSchema.parse('ios'),
  Unknown: BrowserInventoryPlatformSchema.parse('unknown'),
} as const;

export const BrowserInventoryPlatformProofState = {
  HostObserved: BrowserInventoryPlatformProofStateSchema.parse('host-observed'),
  FixtureBacked: BrowserInventoryPlatformProofStateSchema.parse('fixture-backed'),
  ManualRequired: BrowserInventoryPlatformProofStateSchema.parse('manual-required'),
  Unsupported: BrowserInventoryPlatformProofStateSchema.parse('unsupported'),
  NotClaimed: BrowserInventoryPlatformProofStateSchema.parse('not-claimed'),
} as const;

const BrowserInventoryPlatformMatrixEntryHonestyChecks = [
  nonWindowsEntryDoesNotClaimExactUrl,
  nonWindowsEntryDoesNotClaimActiveTab,
  unsupportedEntryHasUnsupportedExactUrl,
  manualEntryHasProofRequirement,
  hostObservedOrFixtureBackedEntryIsWindowsOnly,
  iosEntryIsUnsupported,
] as const;

export const BrowserInventoryPlatformMatrix = BrowserInventoryPlatformMatrixSchema.parse({
  schemaVersion: BrowserEvidenceSchemaVersion,
  entries: [
    windowsCandidate('edge', 'Microsoft Edge', 'windows-managed-edge-candidate'),
    windowsCandidate('chrome', 'Google Chrome', 'windows-managed-chrome-candidate'),
    manualCandidate('macos', 'chrome', 'Google Chrome', 'macos-chrome-cdp-candidate-manual-required'),
    unsupportedEntry('macos', 'unknown', 'Safari', 'macos-safari-webkit-later-adapter'),
    linuxHostObservedCandidate('chrome', 'Google Chrome', 'linux-chrome-host-observed-launch-proof'),
    manualCandidate('linux', 'unknown-chromium', 'Chromium', 'linux-chromium-cdp-candidate-manual-required'),
    unsupportedEntry('linux', 'firefox', 'Mozilla Firefox', 'linux-firefox-bidi-later-adapter'),
    androidOwnedShellHostObservedCandidate('unknown-chromium', 'Android owned browser shell'),
    unsupportedEntry('android', 'chrome', 'Android Chrome', 'android-external-chrome-device-policy-required'),
    unsupportedEntry('android', 'firefox', 'Android Firefox', 'android-firefox-later-adapter'),
    unsupportedEntry('ios', 'unknown', 'iOS Safari', 'ios-safari-familycontrols-manual-required'),
    unsupportedEntry('ios', 'unknown-chromium', 'iOS browser app', 'ios-browser-app-webkit-policy-boundary'),
  ],
});

export const decodeBrowserInventoryPlatformMatrixEntry = Schema.decodeUnknownSync(
  BrowserInventoryPlatformMatrixEntrySchema
);
export const decodeBrowserInventoryPlatformMatrix = Schema.decodeUnknownSync(BrowserInventoryPlatformMatrixSchema);

function windowsCandidate(
  browserFamily: 'edge' | 'chrome',
  productName: string,
  reasonCode: string
): BrowserInventoryPlatformMatrixEntry {
  return BrowserInventoryPlatformMatrixEntrySchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    platform: 'windows',
    browserFamily,
    browserChannel: 'stable',
    productName,
    installState: 'installed',
    managementTier: 'managed',
    supportTier: 'candidate',
    exactUrlCapability: 'unavailable',
    activeTabCapability: 'unavailable',
    managedProfileState: 'missing',
    unmanagedFallbackCapability: 'os-block-manual-required',
    capabilityStatus: 'managed-profile-missing',
    proofState: 'host-observed',
    reasonCode,
    proofRequirement: 'host inventory proof exists; managed launch and bridge proof required before exact URL claims',
  });
}

function manualCandidate(
  platform: 'macos' | 'linux',
  browserFamily: 'chrome' | 'unknown-chromium',
  productName: string,
  reasonCode: string
): BrowserInventoryPlatformMatrixEntry {
  return BrowserInventoryPlatformMatrixEntrySchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    platform,
    browserFamily,
    browserChannel: 'stable',
    productName,
    installState: 'unknown',
    managementTier: 'manual-required',
    supportTier: 'candidate',
    exactUrlCapability: 'manual-required',
    activeTabCapability: 'manual-required',
    managedProfileState: 'manual-required',
    unmanagedFallbackCapability: 'unsupported',
    capabilityStatus: 'permission-limited',
    proofState: 'manual-required',
    reasonCode,
    proofRequirement: 'platform adapter, owned profile, and manual browser proof required',
  });
}

function linuxHostObservedCandidate(
  browserFamily: 'chrome',
  productName: string,
  reasonCode: string
): BrowserInventoryPlatformMatrixEntry {
  return BrowserInventoryPlatformMatrixEntrySchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    platform: 'linux',
    browserFamily,
    browserChannel: 'stable',
    productName,
    installState: 'installed',
    managementTier: 'manual-required',
    supportTier: 'candidate',
    exactUrlCapability: 'manual-required',
    activeTabCapability: 'manual-required',
    managedProfileState: 'manual-required',
    unmanagedFallbackCapability: 'unsupported',
    capabilityStatus: 'permission-limited',
    proofState: 'host-observed',
    reasonCode,
    proofRequirement:
      'Linux WSL browser install and launch proof exists; desktop adapter and managed profile proof required before exact URL claims',
  });
}

function androidOwnedShellHostObservedCandidate(
  browserFamily: 'unknown-chromium',
  productName: string
): BrowserInventoryPlatformMatrixEntry {
  return BrowserInventoryPlatformMatrixEntrySchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    platform: 'android',
    browserFamily,
    browserChannel: 'unknown',
    productName,
    installState: 'installed',
    managementTier: 'owned-shell',
    supportTier: 'candidate',
    exactUrlCapability: 'manual-required',
    activeTabCapability: 'manual-required',
    managedProfileState: 'manual-required',
    unmanagedFallbackCapability: 'unsupported',
    capabilityStatus: 'permission-limited',
    proofState: 'host-observed',
    reasonCode: 'android-owned-browser-shell-browser-role-routing-proof',
    proofRequirement:
      'Android owned browser shell build/install/launch, Device Owner policy mutation, and browser-role implicit routing proof exist; exact URL, active tab, physical device, and enforcement proof still required',
  });
}

function unsupportedEntry(
  platform: 'macos' | 'linux' | 'android' | 'ios',
  browserFamily: 'chrome' | 'firefox' | 'unknown-chromium' | 'unknown',
  productName: string,
  reasonCode: string
): BrowserInventoryPlatformMatrixEntry {
  return BrowserInventoryPlatformMatrixEntrySchema.parse({
    schemaVersion: BrowserEvidenceSchemaVersion,
    platform,
    browserFamily,
    browserChannel: 'unknown',
    productName,
    installState: 'unknown',
    managementTier: 'unsupported',
    supportTier: 'unsupported',
    exactUrlCapability: 'unsupported',
    activeTabCapability: 'unsupported',
    managedProfileState: 'not-applicable',
    unmanagedFallbackCapability: 'unsupported',
    capabilityStatus: 'unsupported-browser',
    proofState: 'unsupported',
    reasonCode,
    proofRequirement: 'later platform-specific adapter required before product claim',
  });
}

function browserInventoryPlatformMatrixEntryIsHonest(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return BrowserInventoryPlatformMatrixEntryHonestyChecks.every((check) => check(entry));
}

function nonWindowsEntryDoesNotClaimExactUrl(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return entry.platform === 'windows' || entry.exactUrlCapability !== 'managed-exact-url-available';
}

function nonWindowsEntryDoesNotClaimActiveTab(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return entry.platform === 'windows' || entry.activeTabCapability !== 'known-active-supported';
}

function unsupportedEntryHasUnsupportedExactUrl(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return entry.supportTier !== 'unsupported' || entry.exactUrlCapability === 'unsupported';
}

function manualEntryHasProofRequirement(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return entry.proofState !== 'manual-required' || entry.proofRequirement !== null;
}

function linuxHostObservedEntryStaysManual(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return (
    entry.platform === 'linux' &&
    entry.proofState === 'host-observed' &&
    entry.exactUrlCapability === 'manual-required' &&
    entry.activeTabCapability === 'manual-required' &&
    entry.managementTier === 'manual-required'
  );
}

function androidHostObservedEntryStaysOwnedShellManual(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return (
    entry.platform === 'android' &&
    entry.proofState === 'host-observed' &&
    entry.exactUrlCapability === 'manual-required' &&
    entry.activeTabCapability === 'manual-required' &&
    entry.managementTier === 'owned-shell'
  );
}

function hostObservedOrFixtureBackedEntryIsWindowsOnly(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  if (entry.proofState !== 'host-observed' && entry.proofState !== 'fixture-backed') {
    return true;
  }

  return (
    entry.platform === 'windows' ||
    linuxHostObservedEntryStaysManual(entry) ||
    androidHostObservedEntryStaysOwnedShellManual(entry)
  );
}

function iosEntryIsUnsupported(entry: BrowserInventoryPlatformMatrixEntryCandidate): boolean {
  return entry.platform !== 'ios' || entry.managementTier === 'unsupported';
}

function platformMatrixEntryKey(entry: BrowserInventoryPlatformMatrixEntryCandidate): string {
  return [entry.platform, entry.browserFamily, entry.browserChannel, entry.productName].join('|');
}
