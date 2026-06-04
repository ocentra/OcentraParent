import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityDeviceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  BrowserCapabilityStatusSchema,
  BrowserChannelSchema,
  BrowserCustodyLabelSchema,
  BrowserEvidenceSchemaVersion,
  BrowserFamilySchema,
  BrowserProfileIdSchema,
  BrowserQueryVisibilityLabelSchema,
  BrowserVersionSchema,
} from './browser-schemas';

const NonEmptyBrowserInventoryText = Schema.String.pipe(Schema.minLength(1));

export const BrowserInventoryInstallStateSchema = withParser(
  Schema.Literal('installed', 'not-installed', 'candidate-running', 'packaged', 'portable', 'unknown')
);
export const BrowserInventoryRunningStateSchema = withParser(
  Schema.Literal('not-running', 'running-managed', 'running-unmanaged', 'running-unknown', 'unknown')
);
export const BrowserManagementTierSchema = withParser(
  Schema.Literal(
    'managed',
    'owned-shell',
    'managed-profile-extension',
    'unmanaged',
    'unsupported',
    'manual-required',
    'unknown'
  )
);
export const BrowserSupportTierSchema = withParser(
  Schema.Literal(
    'managed-url-tab',
    'managed-target-list',
    'candidate',
    'unmanaged-process-only',
    'unsupported',
    'manual-required',
    'unknown'
  )
);
export const BrowserExactUrlCapabilitySchema = withParser(
  Schema.Literal(
    'managed-exact-url-available',
    'managed-target-list-only',
    'manual-required',
    'not-claimed',
    'unsupported',
    'unavailable'
  )
);
export const BrowserActiveTabCapabilitySchema = withParser(
  Schema.Literal(
    'known-active-supported',
    'target-list-only',
    'manual-required',
    'not-claimed',
    'unsupported',
    'unavailable'
  )
);
export const BrowserManagedProfileStateSchema = withParser(
  Schema.Literal('ready', 'missing', 'repair-required', 'not-applicable', 'manual-required', 'unavailable')
);
export const BrowserUnmanagedFallbackCapabilitySchema = withParser(
  Schema.Literal(
    'report-only',
    'warn-child',
    'terminate-process',
    'relaunch-managed',
    'os-block-manual-required',
    'unsupported',
    'unavailable'
  )
);

export const BrowserExecutablePathRefSchema = withParser(
  NonEmptyBrowserInventoryText.pipe(Schema.brand('BrowserExecutablePathRef'))
);
export const BrowserFileHashRefSchema = withParser(
  NonEmptyBrowserInventoryText.pipe(Schema.brand('BrowserFileHashRef'))
);
export const BrowserInventoryReasonCodeSchema = withParser(
  NonEmptyBrowserInventoryText.pipe(Schema.brand('BrowserInventoryReasonCode'))
);
export const BrowserInventoryRowIdSchema = withParser(
  NonEmptyBrowserInventoryText.pipe(Schema.brand('BrowserInventoryRowId'))
);
export const BrowserProductNameSchema = withParser(
  NonEmptyBrowserInventoryText.pipe(Schema.brand('BrowserProductName'))
);
export const BrowserPublisherSignatureRefSchema = withParser(
  NonEmptyBrowserInventoryText.pipe(Schema.brand('BrowserPublisherSignatureRef'))
);

const BrowserInventoryRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
  inventoryRowId: BrowserInventoryRowIdSchema,
  scannedAt: ActivityTimestampSchema,
  deviceId: ActivityDeviceIdSchema,
  browserFamily: BrowserFamilySchema,
  browserChannel: BrowserChannelSchema,
  productName: BrowserProductNameSchema,
  browserVersion: Schema.Union(BrowserVersionSchema, Schema.Null),
  installState: BrowserInventoryInstallStateSchema,
  runningState: BrowserInventoryRunningStateSchema,
  managementTier: BrowserManagementTierSchema,
  supportTier: BrowserSupportTierSchema,
  exactUrlCapability: BrowserExactUrlCapabilitySchema,
  activeTabCapability: BrowserActiveTabCapabilitySchema,
  managedProfileState: BrowserManagedProfileStateSchema,
  unmanagedFallbackCapability: BrowserUnmanagedFallbackCapabilitySchema,
  executablePathRef: Schema.Union(BrowserExecutablePathRefSchema, Schema.Null),
  publisherSignatureRef: Schema.Union(BrowserPublisherSignatureRefSchema, Schema.Null),
  fileHashRef: Schema.Union(BrowserFileHashRefSchema, Schema.Null),
  profileId: Schema.Union(BrowserProfileIdSchema, Schema.Null),
  processId: Schema.Union(Schema.Number, Schema.Null),
  capabilityStatus: BrowserCapabilityStatusSchema,
  reasonCode: BrowserInventoryReasonCodeSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  queryVisibility: BrowserQueryVisibilityLabelSchema,
});

export const BrowserInventoryRowSchema = withParser(
  BrowserInventoryRowBaseSchema.pipe(
    Schema.filter((row) => browserInventoryRowIsConsistent(row) || 'Inconsistent browser inventory claim boundary')
  )
);

export const BrowserInventoryReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    limit: Schema.Number,
    returned: Schema.Number,
    latestObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    capabilityStatus: Schema.Union(BrowserCapabilityStatusSchema, Schema.Null),
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
    rows: Schema.Array(BrowserInventoryRowSchema),
  })
);

export type BrowserInventoryInstallState = Infer<typeof BrowserInventoryInstallStateSchema>;
export type BrowserInventoryRunningState = Infer<typeof BrowserInventoryRunningStateSchema>;
export type BrowserManagementTier = Infer<typeof BrowserManagementTierSchema>;
export type BrowserSupportTier = Infer<typeof BrowserSupportTierSchema>;
export type BrowserExactUrlCapability = Infer<typeof BrowserExactUrlCapabilitySchema>;
export type BrowserActiveTabCapability = Infer<typeof BrowserActiveTabCapabilitySchema>;
export type BrowserManagedProfileState = Infer<typeof BrowserManagedProfileStateSchema>;
export type BrowserUnmanagedFallbackCapability = Infer<typeof BrowserUnmanagedFallbackCapabilitySchema>;
export type BrowserFileHashRef = Infer<typeof BrowserFileHashRefSchema>;
export type BrowserPublisherSignatureRef = Infer<typeof BrowserPublisherSignatureRefSchema>;
export type BrowserInventoryRow = Infer<typeof BrowserInventoryRowSchema>;
export type BrowserInventoryReadModel = Infer<typeof BrowserInventoryReadModelSchema>;

function browserInventoryRowIsConsistent(row: Infer<typeof BrowserInventoryRowBaseSchema>): boolean {
  const exactUrlManaged =
    row.exactUrlCapability === 'managed-exact-url-available' || row.exactUrlCapability === 'managed-target-list-only';
  const managedBoundary =
    row.managementTier === 'managed' ||
    row.managementTier === 'owned-shell' ||
    row.managementTier === 'managed-profile-extension';
  if (exactUrlManaged && !managedBoundary) {
    return false;
  }

  if (
    row.activeTabCapability === 'known-active-supported' &&
    row.exactUrlCapability !== 'managed-exact-url-available'
  ) {
    return false;
  }

  if (row.managementTier === 'unmanaged' && row.exactUrlCapability !== 'not-claimed') {
    return false;
  }

  if (row.managementTier === 'unsupported' && row.supportTier !== 'unsupported') {
    return false;
  }

  return true;
}
