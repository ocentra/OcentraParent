import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivityEventIdSchema,
  ActivitySourceIdSchema,
  ActivityTimestampSchema,
} from './primitives';

export const BrowserEvidenceSchemaVersion = 1;

const NonEmptyBrowserText = Schema.String.pipe(Schema.minLength(1));
const BrowserUrlText = NonEmptyBrowserText.pipe(
  Schema.filter((value) => browserUrlIsValid(value) || 'Expected an absolute browser URL')
);

export const BrowserFamilySchema = withParser(
  Schema.Literal('edge', 'chrome', 'brave', 'firefox', 'opera', 'unknown-chromium', 'unknown')
);

export const BrowserChannelSchema = withParser(Schema.Literal('stable', 'beta', 'dev', 'canary', 'unknown'));

export const BrowserCapabilityStatusSchema = withParser(
  Schema.Literal(
    'available',
    'tab-list-only',
    'unsupported-browser',
    'unmanaged-browser',
    'managed-profile-missing',
    'bridge-missing',
    'permission-limited',
    'stale',
    'adapter-error',
    'disabled-by-parent'
  )
);

export const BrowserManagedStateSchema = withParser(
  Schema.Literal(
    'not-installed',
    'installed-unsupported',
    'installed-supported',
    'managed-profile-ready',
    'launch-pending',
    'running-managed',
    'bridge-connected',
    'bridge-disconnected',
    'permission-required',
    'stopped',
    'error'
  )
);

export const BrowserBridgeKindSchema = withParser(Schema.Literal('chromium-devtools-protocol'));
export const BrowserActiveTabStateSchema = withParser(Schema.Literal('known-active', 'known-inactive', 'unknown'));
export const BrowserCustodyLabelSchema = withParser(
  Schema.Literal(
    'child-device-local',
    'local-network-child-agent',
    'parent-cache',
    'parent-owned-export',
    'unavailable'
  )
);
export const BrowserQueryVisibilityLabelSchema = withParser(
  Schema.Literal('live-local', 'live-lan', 'parent-cache', 'parent-owned-export', 'unavailable')
);

export const BrowserAdapterIdSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserAdapterId')));
export const BrowserBridgeEndpointRefSchema = withParser(
  NonEmptyBrowserText.pipe(Schema.brand('BrowserBridgeEndpointRef'))
);
export const BrowserDegradedReasonSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserDegradedReason')));
export const BrowserDomainSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserDomain')));
export const BrowserManagedSessionIdSchema = withParser(
  NonEmptyBrowserText.pipe(Schema.brand('BrowserManagedSessionId'))
);
export const BrowserOriginSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserOrigin')));
export const BrowserPageTitleSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserPageTitle')));
export const BrowserProfileIdSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserProfileId')));
export const BrowserProfilePathRefSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserProfilePathRef')));
export const BrowserTabIdSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserTabId')));
export const BrowserTargetIdSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserTargetId')));
export const BrowserUrlSchema = withParser(BrowserUrlText.pipe(Schema.brand('BrowserUrl')));
export const BrowserVersionSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserVersion')));
export const BrowserWindowIdSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserWindowId')));

export const BrowserTabEvidenceSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    browserEvidenceId: ActivityEvidenceIdSchema,
    observedAt: ActivityTimestampSchema,
    freshUntil: ActivityTimestampSchema,
    sourceId: ActivitySourceIdSchema,
    adapterId: BrowserAdapterIdSchema,
    deviceId: ActivityDeviceIdSchema,
    browserFamily: BrowserFamilySchema,
    browserChannel: BrowserChannelSchema,
    managedBrowserSessionId: BrowserManagedSessionIdSchema,
    profileId: BrowserProfileIdSchema,
    processId: Schema.Number,
    windowId: Schema.Union(BrowserWindowIdSchema, Schema.Null),
    tabId: Schema.Union(BrowserTabIdSchema, Schema.Null),
    targetId: Schema.Union(BrowserTargetIdSchema, Schema.Null),
    activeState: BrowserActiveTabStateSchema,
    url: BrowserUrlSchema,
    origin: BrowserOriginSchema,
    domain: BrowserDomainSchema,
    title: Schema.Union(BrowserPageTitleSchema, Schema.Null),
    capabilityStatus: BrowserCapabilityStatusSchema,
    degradedReason: Schema.Union(BrowserDegradedReasonSchema, Schema.Null),
    staleAt: ActivityTimestampSchema,
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
  })
);

export const BrowserEvidenceRecentSummarySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    returned: Schema.Number,
    latestEventId: Schema.Union(ActivityEventIdSchema, Schema.Null),
    latestObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    browserEvidenceId: Schema.Union(ActivityEvidenceIdSchema, Schema.Null),
    sourceId: Schema.Union(ActivitySourceIdSchema, Schema.Null),
    adapterId: Schema.Union(BrowserAdapterIdSchema, Schema.Null),
    managedBrowserSessionId: Schema.Union(BrowserManagedSessionIdSchema, Schema.Null),
    browserFamily: Schema.Union(BrowserFamilySchema, Schema.Null),
    activeState: Schema.Union(BrowserActiveTabStateSchema, Schema.Null),
    url: Schema.Union(BrowserUrlSchema, Schema.Null),
    origin: Schema.Union(BrowserOriginSchema, Schema.Null),
    domain: Schema.Union(BrowserDomainSchema, Schema.Null),
    title: Schema.Union(BrowserPageTitleSchema, Schema.Null),
    capabilityStatus: Schema.Union(BrowserCapabilityStatusSchema, Schema.Null),
    custodyLabel: Schema.Union(BrowserCustodyLabelSchema, Schema.Null),
  })
);

export const BrowserManagedSessionStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    checkedAt: ActivityTimestampSchema,
    managedBrowserSessionId: Schema.Union(BrowserManagedSessionIdSchema, Schema.Null),
    browserFamily: Schema.Union(BrowserFamilySchema, Schema.Null),
    browserChannel: Schema.Union(BrowserChannelSchema, Schema.Null),
    browserVersion: Schema.Union(BrowserVersionSchema, Schema.Null),
    profileId: Schema.Union(BrowserProfileIdSchema, Schema.Null),
    profilePathRef: Schema.Union(BrowserProfilePathRefSchema, Schema.Null),
    processId: Schema.Union(Schema.Number, Schema.Null),
    bridgeKind: Schema.Union(BrowserBridgeKindSchema, Schema.Null),
    bridgeEndpointRef: Schema.Union(BrowserBridgeEndpointRefSchema, Schema.Null),
    managedState: BrowserManagedStateSchema,
    capabilityStatus: BrowserCapabilityStatusSchema,
    degradedReason: Schema.Union(BrowserDegradedReasonSchema, Schema.Null),
    startedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
  })
);

export type BrowserFamily = Infer<typeof BrowserFamilySchema>;
export type BrowserChannel = Infer<typeof BrowserChannelSchema>;
export type BrowserCapabilityStatus = Infer<typeof BrowserCapabilityStatusSchema>;
export type BrowserManagedState = Infer<typeof BrowserManagedStateSchema>;
export type BrowserBridgeKind = Infer<typeof BrowserBridgeKindSchema>;
export type BrowserActiveTabState = Infer<typeof BrowserActiveTabStateSchema>;
export type BrowserCustodyLabel = Infer<typeof BrowserCustodyLabelSchema>;
export type BrowserQueryVisibilityLabel = Infer<typeof BrowserQueryVisibilityLabelSchema>;
export type BrowserTabEvidence = Infer<typeof BrowserTabEvidenceSchema>;
export type BrowserEvidenceRecentSummary = Infer<typeof BrowserEvidenceRecentSummarySchema>;
export type BrowserManagedSessionStatus = Infer<typeof BrowserManagedSessionStatusSchema>;

export const decodeBrowserUrl = Schema.decodeUnknownSync(BrowserUrlSchema);

function browserUrlIsValid(value: string): boolean {
  const separatorIndex = value.indexOf('://');
  if (separatorIndex <= 0) {
    return false;
  }
  const authority = value.slice(separatorIndex + 3).split('/')[0] ?? '';
  return authority.length > 0;
}
