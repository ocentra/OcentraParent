/* generated from crates/schema/src/browser_generated_values_ts.rs */

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivityEventIdSchema,
  ActivitySourceIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserUnmanagedDetectionConfidenceSchema,
  BrowserUnmanagedDetectionReasonSchema,
  BrowserUnmanagedExecutablePathRefSchema,
  BrowserUnmanagedProcessHashRefSchema,
  BrowserUnmanagedProcessKindSchema,
  BrowserUnmanagedProcessNameSchema,
  BrowserUnmanagedSignatureRefSchema,
} from './generated-browser-unmanaged-process-schemas';
import { BrowserTargetIdSchema } from './generated-browser-target-schemas';

export const BrowserEvidenceSchemaVersion = 1;
const BrowserUrlText = NonEmptyStringSchema.pipe(
  Schema.filter((value) => browserUrlIsValid(value) || 'Expected an absolute browser URL')
);
const BrowserRedactedRefText = NonEmptyStringSchema.pipe(
  Schema.filter((value) => browserRedactedRefIsSafe(value) || 'Expected a redacted browser reference')
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

const BrowserManagedProfileLifecycleStateSchema = withParser(
  Schema.Literal(
    'ready',
    'missing',
    'repair-required',
    'deleted',
    'unsafe-default-profile',
    'unowned-profile',
    'unavailable'
  )
);
export const BrowserBridgeKindSchema = withParser(Schema.Literal('chromium-devtools-protocol'));
export const BrowserActiveTabStateSchema = withParser(Schema.Literal('known-active', 'known-inactive', 'unknown'));
export const BrowserActiveProofSourceSchema = withParser(
  Schema.Literal(
    'target-list-only',
    'cdp-focus-activation',
    'managed-extension-event',
    'foreground-correlation',
    'owned-shell-event'
  )
);
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
export const BrowserAdapterIdSchema = withParser(brandedNonEmptyStringSchema('BrowserAdapterId'));
export const BrowserBridgeEndpointRefSchema = withParser(brandedNonEmptyStringSchema('BrowserBridgeEndpointRef'));
export const BrowserDegradedReasonSchema = withParser(brandedNonEmptyStringSchema('BrowserDegradedReason'));
export const BrowserDomainSchema = withParser(brandedNonEmptyStringSchema('BrowserDomain'));
export const BrowserManagedSessionIdSchema = withParser(brandedNonEmptyStringSchema('BrowserManagedSessionId'));
export const BrowserOriginSchema = withParser(brandedNonEmptyStringSchema('BrowserOrigin'));
export const BrowserPageTitleSchema = withParser(brandedNonEmptyStringSchema('BrowserPageTitle'));
export const BrowserProfileIdSchema = withParser(brandedNonEmptyStringSchema('BrowserProfileId'));
export const BrowserProfilePathRefSchema = withParser(
  BrowserRedactedRefText.pipe(Schema.brand('BrowserProfilePathRef'))
);
const BrowserProfileRootRefSchema = withParser(BrowserRedactedRefText.pipe(Schema.brand('BrowserProfileRootRef')));
const BrowserProfileScopeIdSchema = withParser(brandedNonEmptyStringSchema('BrowserProfileScopeId'));
const BrowserPolicyRevisionSchema = withParser(brandedNonEmptyStringSchema('BrowserPolicyRevision'));
export const BrowserTabIdSchema = withParser(brandedNonEmptyStringSchema('BrowserTabId'));
export const BrowserUrlSchema = withParser(BrowserUrlText.pipe(Schema.brand('BrowserUrl')));
export const BrowserVersionSchema = withParser(brandedNonEmptyStringSchema('BrowserVersion'));
export const BrowserWindowIdSchema = withParser(brandedNonEmptyStringSchema('BrowserWindowId'));

const BrowserUnmanagedProcessEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
  browserEvidenceId: ActivityEvidenceIdSchema,
  observedAt: ActivityTimestampSchema,
  sourceId: ActivitySourceIdSchema,
  deviceId: ActivityDeviceIdSchema,
  processId: Schema.Number,
  processName: BrowserUnmanagedProcessNameSchema,
  executablePathRef: Schema.Union(BrowserUnmanagedExecutablePathRefSchema, Schema.Null),
  signatureRef: Schema.Union(BrowserUnmanagedSignatureRefSchema, Schema.Null),
  processHashRef: Schema.Union(BrowserUnmanagedProcessHashRefSchema, Schema.Null),
  browserFamily: BrowserFamilySchema,
  browserChannel: BrowserChannelSchema,
  processKind: BrowserUnmanagedProcessKindSchema,
  detectionConfidence: BrowserUnmanagedDetectionConfidenceSchema,
  detectionReason: BrowserUnmanagedDetectionReasonSchema,
  capabilityStatus: BrowserCapabilityStatusSchema,
  custodyLabel: BrowserCustodyLabelSchema,
  queryVisibility: BrowserQueryVisibilityLabelSchema,
});

export const BrowserUnmanagedProcessEvidenceSchema = withParser(
  BrowserUnmanagedProcessEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserUnmanagedProcessEvidenceIsConsistent(value) || 'Expected unmanaged browser evidence to be process-only'
    )
  )
);

export const BrowserUnmanagedProcessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    limit: Schema.Number,
    returned: Schema.Number,
    latestObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    capabilityStatus: Schema.Union(BrowserCapabilityStatusSchema, Schema.Null),
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
    rows: Schema.Array(BrowserUnmanagedProcessEvidenceSchema),
  })
);

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
    activeProofSource: BrowserActiveProofSourceSchema,
    url: BrowserUrlSchema,
    origin: BrowserOriginSchema,
    domain: BrowserDomainSchema,
    title: Schema.Union(BrowserPageTitleSchema, Schema.Null),
    capabilityStatus: BrowserCapabilityStatusSchema,
    degradedReason: Schema.Union(BrowserDegradedReasonSchema, Schema.Null),
    staleAt: ActivityTimestampSchema,
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        (browserEvidenceUrlFieldsAreConsistent(value) && browserEvidenceActiveProofIsConsistent(value)) ||
        'Expected browser evidence URL fields and active-tab proof source to match'
    )
  )
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
    activeProofSource: Schema.Union(BrowserActiveProofSourceSchema, Schema.Null),
    url: Schema.Union(BrowserUrlSchema, Schema.Null),
    origin: Schema.Union(BrowserOriginSchema, Schema.Null),
    domain: Schema.Union(BrowserDomainSchema, Schema.Null),
    title: Schema.Union(BrowserPageTitleSchema, Schema.Null),
    capabilityStatus: Schema.Union(BrowserCapabilityStatusSchema, Schema.Null),
    custodyLabel: Schema.Union(BrowserCustodyLabelSchema, Schema.Null),
  })
);

export const BrowserEvidenceReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(BrowserEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    limit: Schema.Number,
    returned: Schema.Number,
    latestEventId: Schema.Union(ActivityEventIdSchema, Schema.Null),
    latestObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    capabilityStatus: Schema.Union(BrowserCapabilityStatusSchema, Schema.Null),
    custodyLabel: BrowserCustodyLabelSchema,
    queryVisibility: BrowserQueryVisibilityLabelSchema,
    rows: Schema.Array(BrowserTabEvidenceSchema),
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
    profileRootRef: Schema.Union(BrowserProfileRootRefSchema, Schema.Null),
    profileScopeId: Schema.Union(BrowserProfileScopeIdSchema, Schema.Null),
    profileLifecycleState: Schema.Union(BrowserManagedProfileLifecycleStateSchema, Schema.Null),
    policyRevision: Schema.Union(BrowserPolicyRevisionSchema, Schema.Null),
    processId: Schema.Union(Schema.Number, Schema.Null),
    bridgeKind: Schema.Union(BrowserBridgeKindSchema, Schema.Null),
    bridgeEndpointRef: Schema.Union(BrowserBridgeEndpointRefSchema, Schema.Null),
    unmanagedProcessName: Schema.optionalWith(Schema.Union(BrowserUnmanagedProcessNameSchema, Schema.Null), {
      default: () => null,
    }),
    unmanagedExecutablePathRef: Schema.optionalWith(
      Schema.Union(BrowserUnmanagedExecutablePathRefSchema, Schema.Null),
      {
        default: () => null,
      }
    ),
    unmanagedSignatureRef: Schema.optionalWith(Schema.Union(BrowserUnmanagedSignatureRefSchema, Schema.Null), {
      default: () => null,
    }),
    unmanagedProcessHashRef: Schema.optionalWith(Schema.Union(BrowserUnmanagedProcessHashRefSchema, Schema.Null), {
      default: () => null,
    }),
    unmanagedProcessKind: Schema.optionalWith(Schema.Union(BrowserUnmanagedProcessKindSchema, Schema.Null), {
      default: () => null,
    }),
    unmanagedDetectionConfidence: Schema.optionalWith(
      Schema.Union(BrowserUnmanagedDetectionConfidenceSchema, Schema.Null),
      {
        default: () => null,
      }
    ),
    unmanagedDetectionReason: Schema.optionalWith(Schema.Union(BrowserUnmanagedDetectionReasonSchema, Schema.Null), {
      default: () => null,
    }),
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
export type BrowserActiveProofSource = Infer<typeof BrowserActiveProofSourceSchema>;
export type BrowserCustodyLabel = Infer<typeof BrowserCustodyLabelSchema>;
export type BrowserQueryVisibilityLabel = Infer<typeof BrowserQueryVisibilityLabelSchema>;
export type BrowserUnmanagedProcessEvidence = Infer<typeof BrowserUnmanagedProcessEvidenceSchema>;
export type BrowserUnmanagedProcessReadModel = Infer<typeof BrowserUnmanagedProcessReadModelSchema>;
export type BrowserTabEvidence = Infer<typeof BrowserTabEvidenceSchema>;
export type BrowserEvidenceRecentSummary = Infer<typeof BrowserEvidenceRecentSummarySchema>;
export type BrowserEvidenceReadModel = Infer<typeof BrowserEvidenceReadModelSchema>;
export type BrowserManagedSessionStatus = Infer<typeof BrowserManagedSessionStatusSchema>;

export const decodeBrowserUrl = Schema.decodeUnknownSync(BrowserUrlSchema);

function browserUrlIsValid(value: string): boolean {
  return browserUrlParts(value) !== null;
}

function browserEvidenceUrlFieldsAreConsistent(value: {
  readonly url: string;
  readonly origin: string;
  readonly domain: string;
}): boolean {
  const parts = browserUrlParts(value.url);
  return parts !== null && value.url === parts.url && value.origin === parts.origin && value.domain === parts.domain;
}

function browserEvidenceActiveProofIsConsistent(value: {
  readonly activeState: string;
  readonly activeProofSource: string;
}): boolean {
  if (value.activeProofSource === 'target-list-only') {
    return value.activeState === 'unknown';
  }
  return true;
}

function browserUnmanagedProcessEvidenceIsConsistent(value: {
  readonly capabilityStatus: string;
  readonly processKind: string;
  readonly detectionReason: string;
}): boolean {
  if (value.capabilityStatus !== 'unmanaged-browser' && value.capabilityStatus !== 'unsupported-browser') {
    return false;
  }
  if (value.processKind === 'supported-browser') {
    return value.detectionReason === 'supported-browser-outside-managed-session';
  }
  if (value.processKind === 'unsupported-browser') {
    return value.detectionReason === 'unsupported-browser-process';
  }
  return true;
}

function browserUrlParts(
  value: string
): { readonly url: string; readonly origin: string; readonly domain: string } | null {
  const separatorIndex = value.indexOf('://');
  if (separatorIndex <= 0) {
    return null;
  }
  const scheme = value.slice(0, separatorIndex).toLowerCase();
  const remainder = value.slice(separatorIndex + 3);
  const authority = remainder.split('/')[0] ?? '';
  if (authority.length === 0) {
    return null;
  }
  const authorityParts = authority.split('@');
  const authorityWithoutCredentials = authorityParts[authorityParts.length - 1] ?? '';
  const normalized = normalizedAuthority(authorityWithoutCredentials);
  if (normalized === null) {
    return null;
  }
  const suffix = remainder.slice(authority.length);
  const origin = `${scheme}://${normalized.authority}`;
  return {
    url: `${origin}${suffix}`,
    origin,
    domain: normalized.domain,
  };
}

function normalizedAuthority(value: string): { readonly authority: string; readonly domain: string } | null {
  const [host, port] = splitHostAndPort(value);
  const domain = normalizedHost(host);
  if (domain === null) {
    return null;
  }
  return {
    authority: port === null ? domain : `${domain}:${port}`,
    domain,
  };
}

function splitHostAndPort(value: string): readonly [string, string | null] {
  const separatorCount = value.split(':').length - 1;
  if (separatorCount === 1) {
    const separatorIndex = value.lastIndexOf(':');
    const host = value.slice(0, separatorIndex);
    const port = value.slice(separatorIndex + 1);
    if (host.length > 0 && /^[0-9]+$/.test(port)) {
      return [host, port];
    }
  }
  return [value, null];
}

function normalizedHost(value: string): string | null {
  const normalized = value.replace(/\.+$/, '').toLowerCase();
  if (normalized.length === 0 || normalized.includes('/')) {
    return null;
  }
  return normalized;
}

function browserRedactedRefIsSafe(value: string): boolean {
  return !value.includes('/') && !value.includes('\\') && !value.includes(':');
}
