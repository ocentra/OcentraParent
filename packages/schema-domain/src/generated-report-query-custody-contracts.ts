/* generated from crates/schema/src/report_query_custody.rs */

import { brandedNonEmptyStringSchema } from './effect';

export const ReportQueryCustodyContractRuntime = {
  SchemaVersion: 'report-query-custody-proof',
} as const;
export const GeneratedReportQueryCustodyMaxPageSize = 100 as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export const GeneratedParentAccountIdSchema = brandedNonEmptyStringSchema('ParentAccountId');
export const GeneratedFamilyIdSchema = brandedNonEmptyStringSchema('FamilyId');
export const GeneratedChildProfileIdSchema = brandedNonEmptyStringSchema('ChildProfileId');
export const GeneratedParentDeviceIdSchema = brandedNonEmptyStringSchema('ParentDeviceId');
export const GeneratedParentDeviceLabelSchema = brandedNonEmptyStringSchema('ParentDeviceLabel');
export const GeneratedParentActorIdSchema = brandedNonEmptyStringSchema('ParentActorId');
export const GeneratedParentPolicyVersionSchema = brandedNonEmptyStringSchema('ParentPolicyVersion');
export const GeneratedParentEvidenceReferenceIdSchema = brandedNonEmptyStringSchema('ParentEvidenceReferenceId');
export const GeneratedParentActionReferenceIdSchema = brandedNonEmptyStringSchema('ParentActionReferenceId');
export const GeneratedParentAuthorityReferenceIdSchema = brandedNonEmptyStringSchema('ParentAuthorityReferenceId');
export const GeneratedParentTimestampSchema = brandedNonEmptyStringSchema('ParentTimestamp');
export const GeneratedReportQueryCustodyRequestIdSchema = brandedNonEmptyStringSchema('ReportQueryCustodyRequestId');
export const GeneratedReportQueryCustodyQueryCursorSchema = brandedNonEmptyStringSchema(
  'ReportQueryCustodyQueryCursor'
);
export const GeneratedReportQueryCustodyCursorRefSchema = brandedNonEmptyStringSchema('ReportQueryCustodyCursorRef');
export const GeneratedReportQueryCustodySortKeySchema = brandedNonEmptyStringSchema('ReportQueryCustodySortKey');
export const GeneratedReportQueryCustodySourceRefSchema = brandedNonEmptyStringSchema('ReportQueryCustodySourceRef');
export const GeneratedReportQueryCustodyConflictRefSchema = brandedNonEmptyStringSchema(
  'ReportQueryCustodyConflictRef'
);
export const GeneratedReportQueryCustodyDeletedSourceRefSchema = brandedNonEmptyStringSchema(
  'ReportQueryCustodyDeletedSourceRef'
);

export type GeneratedParentAccountId = typeof GeneratedParentAccountIdSchema.Type;
export type GeneratedFamilyId = typeof GeneratedFamilyIdSchema.Type;
export type GeneratedChildProfileId = typeof GeneratedChildProfileIdSchema.Type;
export type GeneratedParentDeviceId = typeof GeneratedParentDeviceIdSchema.Type;
export type GeneratedParentDeviceLabel = typeof GeneratedParentDeviceLabelSchema.Type;
export type GeneratedParentActorId = typeof GeneratedParentActorIdSchema.Type;
export type GeneratedParentPolicyVersion = typeof GeneratedParentPolicyVersionSchema.Type;
export type GeneratedParentEvidenceReferenceId = typeof GeneratedParentEvidenceReferenceIdSchema.Type;
export type GeneratedParentActionReferenceId = typeof GeneratedParentActionReferenceIdSchema.Type;
export type GeneratedParentAuthorityReferenceId = typeof GeneratedParentAuthorityReferenceIdSchema.Type;
export type GeneratedParentTimestamp = typeof GeneratedParentTimestampSchema.Type;
export type GeneratedReportQueryCustodyRequestId = typeof GeneratedReportQueryCustodyRequestIdSchema.Type;
export type GeneratedReportQueryCustodyQueryCursor = typeof GeneratedReportQueryCustodyQueryCursorSchema.Type;
export type GeneratedReportQueryCustodyCursorRef = typeof GeneratedReportQueryCustodyCursorRefSchema.Type;
export type GeneratedReportQueryCustodySortKey = typeof GeneratedReportQueryCustodySortKeySchema.Type;
export type GeneratedReportQueryCustodySourceRef = typeof GeneratedReportQueryCustodySourceRefSchema.Type;
export type GeneratedReportQueryCustodyConflictRef = typeof GeneratedReportQueryCustodyConflictRefSchema.Type;
export type GeneratedReportQueryCustodyDeletedSourceRef = typeof GeneratedReportQueryCustodyDeletedSourceRefSchema.Type;

export type GeneratedParentPlatform = 'windows' | 'linux' | 'macos' | 'android' | 'ios';
export type GeneratedParentActorRole = 'parent' | 'guardian' | 'system';
export type GeneratedParentEvidenceReferenceKind =
  | 'journal-event'
  | 'query-store-summary'
  | 'activity-event'
  | 'policy-decision'
  | 'local-ai-result';
export type GeneratedReportQueryCustodyState =
  | 'derivedFresh'
  | 'derivedStale'
  | 'partiallyRedacted'
  | 'deletedSource'
  | 'syncConflict'
  | 'cursorExpired'
  | 'rateLimited';
export type GeneratedReportQueryCustodySourceFreshness =
  | 'fresh'
  | 'stale'
  | 'deleted'
  | 'conflicted'
  | 'expired'
  | 'rate-limited';
export type GeneratedReportQueryCustodyBoundary = 'parent-owned-citations-only';
export type GeneratedReportQueryCustodyPayloadRedaction = 'fully-redacted' | 'partially-redacted';
export type GeneratedReportQueryCustodySourceDataClass =
  | 'sqlite-query-row'
  | 'notification-history'
  | 'audit-event'
  | 'generated-summary';
export type GeneratedReportQueryCustodyNonClaim =
  | 'no-second-truth-store'
  | 'no-portal-ui'
  | 'no-raw-child-evidence'
  | 'no-unbounded-pagination'
  | 'no-provider-routing'
  | 'no-ocentra-hosted-family-data-custody';
export type GeneratedReportQueryCustodyTombstoneState = 'not-required' | 'written';

export const GeneratedParentPlatforms = [
  'windows',
  'linux',
  'macos',
  'android',
  'ios',
] as const satisfies readonly GeneratedParentPlatform[];
export const GeneratedParentActorRoles = [
  'parent',
  'guardian',
  'system',
] as const satisfies readonly GeneratedParentActorRole[];
export const GeneratedParentEvidenceReferenceKinds = [
  'journal-event',
  'query-store-summary',
  'activity-event',
  'policy-decision',
  'local-ai-result',
] as const satisfies readonly GeneratedParentEvidenceReferenceKind[];
export const GeneratedReportQueryCustodyStates = [
  'derivedFresh',
  'derivedStale',
  'partiallyRedacted',
  'deletedSource',
  'syncConflict',
  'cursorExpired',
  'rateLimited',
] as const satisfies readonly GeneratedReportQueryCustodyState[];
export const GeneratedReportQueryCustodySourceFreshnessStates = [
  'fresh',
  'stale',
  'deleted',
  'conflicted',
  'expired',
  'rate-limited',
] as const satisfies readonly GeneratedReportQueryCustodySourceFreshness[];
export const GeneratedReportQueryCustodyBoundaries = [
  'parent-owned-citations-only',
] as const satisfies readonly GeneratedReportQueryCustodyBoundary[];
export const GeneratedReportQueryCustodyPayloadRedactionStates = [
  'fully-redacted',
  'partially-redacted',
] as const satisfies readonly GeneratedReportQueryCustodyPayloadRedaction[];
export const GeneratedReportQueryCustodySourceDataClasses = [
  'sqlite-query-row',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const satisfies readonly GeneratedReportQueryCustodySourceDataClass[];
export const GeneratedReportQueryCustodyNonClaims = [
  'no-second-truth-store',
  'no-portal-ui',
  'no-raw-child-evidence',
  'no-unbounded-pagination',
  'no-provider-routing',
  'no-ocentra-hosted-family-data-custody',
] as const satisfies readonly GeneratedReportQueryCustodyNonClaim[];
export const GeneratedReportQueryCustodyTombstoneStates = [
  'not-required',
  'written',
] as const satisfies readonly GeneratedReportQueryCustodyTombstoneState[];

export interface GeneratedParentActorReference {
  readonly actorId: GeneratedParentActorId;
  readonly role: GeneratedParentActorRole;
}

export interface GeneratedParentAccountReference {
  readonly parentAccountId: GeneratedParentAccountId;
}

export interface GeneratedFamilyReference {
  readonly familyId: GeneratedFamilyId;
}

export interface GeneratedParentDeviceReference {
  readonly deviceId: GeneratedParentDeviceId;
  readonly childProfileId: GeneratedChildProfileId | null;
  readonly label: GeneratedParentDeviceLabel;
  readonly platform: GeneratedParentPlatform;
}

export interface GeneratedReportQueryCustodyParentAuthorityReference {
  readonly authorityReferenceId: GeneratedParentAuthorityReferenceId;
  readonly familyId: GeneratedFamilyId;
  readonly parentAccountId: GeneratedParentAccountId;
  readonly deviceId: GeneratedParentDeviceId;
  readonly childProfileId: GeneratedChildProfileId | null;
  readonly authorityGeneration: number;
}

export interface GeneratedParentEvidenceReference {
  readonly evidenceReferenceId: GeneratedParentEvidenceReferenceId;
  readonly kind: GeneratedParentEvidenceReferenceKind;
  readonly observedAt: GeneratedParentTimestamp;
  readonly familyId: GeneratedFamilyId;
  readonly childProfileId: GeneratedChildProfileId | null;
  readonly sourceDataClass: GeneratedReportQueryCustodySourceDataClass;
  readonly sourceReference: GeneratedReportQueryCustodySourceRef;
}

export interface GeneratedParentActionReference {
  readonly actionReferenceId: GeneratedParentActionReferenceId;
  readonly actor: GeneratedParentActorReference;
  readonly policyVersion: GeneratedParentPolicyVersion;
  readonly createdAt: GeneratedParentTimestamp;
}

export interface GeneratedReportQueryCustodyRequest {
  readonly schemaVersion: typeof ReportQueryCustodyContractRuntime.SchemaVersion;
  readonly requestId: GeneratedReportQueryCustodyRequestId;
  readonly family: GeneratedFamilyReference;
  readonly account: GeneratedParentAccountReference;
  readonly device: GeneratedParentDeviceReference;
  readonly parentAction: GeneratedParentActionReference;
  readonly requestedCursor: GeneratedReportQueryCustodyQueryCursor;
  readonly pageSize: number;
  readonly requestedDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  readonly allowedSourceDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  readonly sourceCitationRefs: readonly GeneratedParentEvidenceReference[];
  readonly assistantCitationRefs: readonly GeneratedParentEvidenceReference[];
  readonly notificationPayloadBoundary: GeneratedReportQueryCustodyBoundary;
  readonly parentAuthority: GeneratedReportQueryCustodyParentAuthorityReference;
  readonly rawChildEvidenceRequested: boolean;
}

export interface GeneratedReportQueryCustodyRow {
  readonly rowId: GeneratedReportQueryCustodySourceRef;
  readonly requestId: GeneratedReportQueryCustodyRequestId;
  readonly state: GeneratedReportQueryCustodyState;
  readonly sourceFreshness: GeneratedReportQueryCustodySourceFreshness;
  readonly sourceDataClass: GeneratedReportQueryCustodySourceDataClass;
  readonly cursorRef: GeneratedReportQueryCustodyCursorRef;
  readonly sourceCursorRef: GeneratedReportQueryCustodyCursorRef;
  readonly nextCursorRef: GeneratedReportQueryCustodyCursorRef | null;
  readonly pageIndex: number;
  readonly pageSize: number;
  readonly stableSortKey: GeneratedReportQueryCustodySortKey;
  readonly requestedDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  readonly allowedSourceDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  readonly sourceCitationRefs: readonly GeneratedParentEvidenceReference[];
  readonly assistantCitationRefs: readonly GeneratedParentEvidenceReference[];
  readonly notificationPayloadBoundary: GeneratedReportQueryCustodyBoundary;
  readonly payloadRedactionState: GeneratedReportQueryCustodyPayloadRedaction;
  readonly tombstoneState: GeneratedReportQueryCustodyTombstoneState;
  readonly deletedSourceRef: GeneratedReportQueryCustodyDeletedSourceRef | null;
  readonly deletedSourceAt: GeneratedParentTimestamp | null;
  readonly conflictRef: GeneratedReportQueryCustodyConflictRef | null;
  readonly cursorExpiredAt: GeneratedParentTimestamp | null;
  readonly rateLimitedUntilAt: GeneratedParentTimestamp | null;
  readonly parentAuthority: GeneratedReportQueryCustodyParentAuthorityReference;
  readonly rawChildEvidenceIncluded: boolean;
  readonly reportCacheMutated: boolean;
  readonly secondTruthStoreClaimed: boolean;
  readonly claimSafe: boolean;
}

export interface GeneratedReportQueryCustodyContractProof {
  readonly schemaVersion: typeof ReportQueryCustodyContractRuntime.SchemaVersion;
  readonly contractVersion: GeneratedParentContractSchemaVersion;
  readonly request: GeneratedReportQueryCustodyRequest;
  readonly rows: readonly GeneratedReportQueryCustodyRow[];
  readonly nonClaims: readonly GeneratedReportQueryCustodyNonClaim[];
  readonly reportRuntimeClaimed: boolean;
  readonly portalUiClaimed: boolean;
  readonly providerRoutingClaimed: boolean;
  readonly ocentraHostedFamilyDataCustodyClaimed: boolean;
  readonly secondTruthStoreClaimed: boolean;
  readonly rawChildEvidenceClaimed: boolean;
  readonly updatedAt: GeneratedParentTimestamp;
}

export const GeneratedReportQueryCustodyKnownGaps = [
  'No uncontrolled second truth store is claimed for report or query custody.',
  'No portal rendering, provider routing, or raw child evidence handling is claimed.',
  'Pagination is modeled as stable derived state over governed evidence, not a second report store.',
  'Delete, tombstone, stale, conflict, and rate-limit outcomes stay explicit and claim-safe.',
  'Assistant and report citations stay inside query-store-summary evidence refs only.',
] as const;

export const GeneratedReportQueryCustodyContractProof = {
  schemaVersion: 'report-query-custody-proof',
  contractVersion: 'v0.6',
  request: {
    schemaVersion: 'report-query-custody-proof',
    requestId: 'report-query-custody-request-proof-1',
    family: {
      familyId: 'family-report-query-custody-proof-1',
    },
    account: {
      parentAccountId: 'parent-account-report-query-custody-proof-1',
    },
    device: {
      deviceId: 'windows-parent-device-report-query-custody-proof-1',
      childProfileId: null,
      label: 'Windows parent device report query custody proof',
      platform: 'windows',
    },
    parentAction: {
      actionReferenceId: 'parent-action-report-query-custody-proof-1',
      actor: {
        actorId: 'parent-report-query-custody-proof-1',
        role: 'parent',
      },
      policyVersion: 'report-query-custody-proof-v1',
      createdAt: '2026-06-28T15:55:00.000Z',
    },
    requestedCursor: 'report-query-custody-cursor-proof-1',
    pageSize: 25,
    requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
    allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
    sourceCitationRefs: [
      {
        evidenceReferenceId: 'report-query-custody-evidence-1',
        kind: 'query-store-summary',
        observedAt: '2026-06-28T15:55:00.000Z',
        familyId: 'family-report-query-custody-proof-1',
        childProfileId: null,
        sourceDataClass: 'sqlite-query-row',
        sourceReference: 'report-query-custody-source-1',
      },
      {
        evidenceReferenceId: 'report-query-custody-evidence-2',
        kind: 'query-store-summary',
        observedAt: '2026-06-28T15:55:00.000Z',
        familyId: 'family-report-query-custody-proof-1',
        childProfileId: null,
        sourceDataClass: 'notification-history',
        sourceReference: 'report-query-custody-source-2',
      },
    ],
    assistantCitationRefs: [
      {
        evidenceReferenceId: 'report-query-custody-evidence-1',
        kind: 'query-store-summary',
        observedAt: '2026-06-28T15:55:00.000Z',
        familyId: 'family-report-query-custody-proof-1',
        childProfileId: null,
        sourceDataClass: 'sqlite-query-row',
        sourceReference: 'report-query-custody-source-1',
      },
      {
        evidenceReferenceId: 'report-query-custody-evidence-2',
        kind: 'query-store-summary',
        observedAt: '2026-06-28T15:55:00.000Z',
        familyId: 'family-report-query-custody-proof-1',
        childProfileId: null,
        sourceDataClass: 'notification-history',
        sourceReference: 'report-query-custody-source-2',
      },
    ],
    notificationPayloadBoundary: 'parent-owned-citations-only',
    parentAuthority: {
      authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
      familyId: 'family-report-query-custody-proof-1',
      parentAccountId: 'parent-account-report-query-custody-proof-1',
      deviceId: 'windows-parent-device-report-query-custody-proof-1',
      childProfileId: null,
      authorityGeneration: 1,
    },
    rawChildEvidenceRequested: false,
  },
  rows: [
    {
      rowId: 'report-query-custody-row-derivedFresh',
      requestId: 'report-query-custody-request-proof-1',
      state: 'derivedFresh',
      sourceFreshness: 'fresh',
      sourceDataClass: 'sqlite-query-row',
      cursorRef: 'report-query-custody-cursor-proof-1',
      sourceCursorRef: 'report-query-custody-source-cursor-proof-1',
      nextCursorRef: 'derived-fresh-next-cursor',
      pageIndex: 1,
      pageSize: 25,
      stableSortKey: 'report-query-custody-stable-sort-key-01',
      requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      sourceCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      assistantCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      notificationPayloadBoundary: 'parent-owned-citations-only',
      payloadRedactionState: 'fully-redacted',
      tombstoneState: 'not-required',
      deletedSourceRef: null,
      deletedSourceAt: null,
      conflictRef: null,
      cursorExpiredAt: null,
      rateLimitedUntilAt: null,
      parentAuthority: {
        authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
        familyId: 'family-report-query-custody-proof-1',
        parentAccountId: 'parent-account-report-query-custody-proof-1',
        deviceId: 'windows-parent-device-report-query-custody-proof-1',
        childProfileId: null,
        authorityGeneration: 1,
      },
      rawChildEvidenceIncluded: false,
      reportCacheMutated: false,
      secondTruthStoreClaimed: false,
      claimSafe: true,
    },
    {
      rowId: 'report-query-custody-row-derivedStale',
      requestId: 'report-query-custody-request-proof-1',
      state: 'derivedStale',
      sourceFreshness: 'stale',
      sourceDataClass: 'generated-summary',
      cursorRef: 'derived-fresh-next-cursor',
      sourceCursorRef: 'report-query-custody-source-cursor-proof-1',
      nextCursorRef: 'derived-stale-next-cursor',
      pageIndex: 2,
      pageSize: 25,
      stableSortKey: 'report-query-custody-stable-sort-key-02',
      requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      sourceCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      assistantCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      notificationPayloadBoundary: 'parent-owned-citations-only',
      payloadRedactionState: 'fully-redacted',
      tombstoneState: 'not-required',
      deletedSourceRef: null,
      deletedSourceAt: null,
      conflictRef: null,
      cursorExpiredAt: null,
      rateLimitedUntilAt: null,
      parentAuthority: {
        authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
        familyId: 'family-report-query-custody-proof-1',
        parentAccountId: 'parent-account-report-query-custody-proof-1',
        deviceId: 'windows-parent-device-report-query-custody-proof-1',
        childProfileId: null,
        authorityGeneration: 1,
      },
      rawChildEvidenceIncluded: false,
      reportCacheMutated: false,
      secondTruthStoreClaimed: false,
      claimSafe: true,
    },
    {
      rowId: 'report-query-custody-row-partiallyRedacted',
      requestId: 'report-query-custody-request-proof-1',
      state: 'partiallyRedacted',
      sourceFreshness: 'stale',
      sourceDataClass: 'notification-history',
      cursorRef: 'derived-stale-next-cursor',
      sourceCursorRef: 'report-query-custody-source-cursor-proof-1',
      nextCursorRef: 'partially-redacted-next-cursor',
      pageIndex: 3,
      pageSize: 25,
      stableSortKey: 'report-query-custody-stable-sort-key-03',
      requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      sourceCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      assistantCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      notificationPayloadBoundary: 'parent-owned-citations-only',
      payloadRedactionState: 'partially-redacted',
      tombstoneState: 'not-required',
      deletedSourceRef: null,
      deletedSourceAt: null,
      conflictRef: null,
      cursorExpiredAt: null,
      rateLimitedUntilAt: null,
      parentAuthority: {
        authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
        familyId: 'family-report-query-custody-proof-1',
        parentAccountId: 'parent-account-report-query-custody-proof-1',
        deviceId: 'windows-parent-device-report-query-custody-proof-1',
        childProfileId: null,
        authorityGeneration: 1,
      },
      rawChildEvidenceIncluded: false,
      reportCacheMutated: false,
      secondTruthStoreClaimed: false,
      claimSafe: true,
    },
    {
      rowId: 'report-query-custody-row-deletedSource',
      requestId: 'report-query-custody-request-proof-1',
      state: 'deletedSource',
      sourceFreshness: 'deleted',
      sourceDataClass: 'audit-event',
      cursorRef: 'partially-redacted-next-cursor',
      sourceCursorRef: 'report-query-custody-source-cursor-proof-1',
      nextCursorRef: null,
      pageIndex: 4,
      pageSize: 25,
      stableSortKey: 'report-query-custody-stable-sort-key-04',
      requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      sourceCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      assistantCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      notificationPayloadBoundary: 'parent-owned-citations-only',
      payloadRedactionState: 'fully-redacted',
      tombstoneState: 'written',
      deletedSourceRef: 'deleted-source-ref-1',
      deletedSourceAt: '2026-06-28T15:57:00.000Z',
      conflictRef: null,
      cursorExpiredAt: null,
      rateLimitedUntilAt: null,
      parentAuthority: {
        authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
        familyId: 'family-report-query-custody-proof-1',
        parentAccountId: 'parent-account-report-query-custody-proof-1',
        deviceId: 'windows-parent-device-report-query-custody-proof-1',
        childProfileId: null,
        authorityGeneration: 1,
      },
      rawChildEvidenceIncluded: false,
      reportCacheMutated: false,
      secondTruthStoreClaimed: false,
      claimSafe: true,
    },
    {
      rowId: 'report-query-custody-row-syncConflict',
      requestId: 'report-query-custody-request-proof-1',
      state: 'syncConflict',
      sourceFreshness: 'conflicted',
      sourceDataClass: 'sqlite-query-row',
      cursorRef: 'report-query-custody-cursor-syncConflict',
      sourceCursorRef: 'report-query-custody-source-cursor-proof-1',
      nextCursorRef: 'sync-conflict-next-cursor',
      pageIndex: 5,
      pageSize: 25,
      stableSortKey: 'report-query-custody-stable-sort-key-05',
      requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      sourceCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      assistantCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      notificationPayloadBoundary: 'parent-owned-citations-only',
      payloadRedactionState: 'fully-redacted',
      tombstoneState: 'not-required',
      deletedSourceRef: null,
      deletedSourceAt: null,
      conflictRef: 'conflict-ref-1',
      cursorExpiredAt: null,
      rateLimitedUntilAt: null,
      parentAuthority: {
        authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
        familyId: 'family-report-query-custody-proof-1',
        parentAccountId: 'parent-account-report-query-custody-proof-1',
        deviceId: 'windows-parent-device-report-query-custody-proof-1',
        childProfileId: null,
        authorityGeneration: 1,
      },
      rawChildEvidenceIncluded: false,
      reportCacheMutated: false,
      secondTruthStoreClaimed: false,
      claimSafe: true,
    },
    {
      rowId: 'report-query-custody-row-cursorExpired',
      requestId: 'report-query-custody-request-proof-1',
      state: 'cursorExpired',
      sourceFreshness: 'expired',
      sourceDataClass: 'generated-summary',
      cursorRef: 'sync-conflict-next-cursor',
      sourceCursorRef: 'report-query-custody-source-cursor-proof-1',
      nextCursorRef: null,
      pageIndex: 6,
      pageSize: 25,
      stableSortKey: 'report-query-custody-stable-sort-key-06',
      requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      sourceCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      assistantCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      notificationPayloadBoundary: 'parent-owned-citations-only',
      payloadRedactionState: 'fully-redacted',
      tombstoneState: 'not-required',
      deletedSourceRef: null,
      deletedSourceAt: null,
      conflictRef: null,
      cursorExpiredAt: '2026-06-28T15:59:00.000Z',
      rateLimitedUntilAt: null,
      parentAuthority: {
        authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
        familyId: 'family-report-query-custody-proof-1',
        parentAccountId: 'parent-account-report-query-custody-proof-1',
        deviceId: 'windows-parent-device-report-query-custody-proof-1',
        childProfileId: null,
        authorityGeneration: 1,
      },
      rawChildEvidenceIncluded: false,
      reportCacheMutated: false,
      secondTruthStoreClaimed: false,
      claimSafe: true,
    },
    {
      rowId: 'report-query-custody-row-rateLimited',
      requestId: 'report-query-custody-request-proof-1',
      state: 'rateLimited',
      sourceFreshness: 'rate-limited',
      sourceDataClass: 'notification-history',
      cursorRef: 'report-query-custody-cursor-rateLimited',
      sourceCursorRef: 'report-query-custody-source-cursor-proof-1',
      nextCursorRef: null,
      pageIndex: 7,
      pageSize: 25,
      stableSortKey: 'report-query-custody-stable-sort-key-07',
      requestedDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      allowedSourceDataClasses: ['sqlite-query-row', 'notification-history', 'audit-event', 'generated-summary'],
      sourceCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      assistantCitationRefs: [
        {
          evidenceReferenceId: 'report-query-custody-evidence-1',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'sqlite-query-row',
          sourceReference: 'report-query-custody-source-1',
        },
        {
          evidenceReferenceId: 'report-query-custody-evidence-2',
          kind: 'query-store-summary',
          observedAt: '2026-06-28T15:55:00.000Z',
          familyId: 'family-report-query-custody-proof-1',
          childProfileId: null,
          sourceDataClass: 'notification-history',
          sourceReference: 'report-query-custody-source-2',
        },
      ],
      notificationPayloadBoundary: 'parent-owned-citations-only',
      payloadRedactionState: 'fully-redacted',
      tombstoneState: 'not-required',
      deletedSourceRef: null,
      deletedSourceAt: null,
      conflictRef: null,
      cursorExpiredAt: null,
      rateLimitedUntilAt: '2026-06-28T16:05:00.000Z',
      parentAuthority: {
        authorityReferenceId: 'parent-authority-report-query-custody-proof-1',
        familyId: 'family-report-query-custody-proof-1',
        parentAccountId: 'parent-account-report-query-custody-proof-1',
        deviceId: 'windows-parent-device-report-query-custody-proof-1',
        childProfileId: null,
        authorityGeneration: 1,
      },
      rawChildEvidenceIncluded: false,
      reportCacheMutated: false,
      secondTruthStoreClaimed: false,
      claimSafe: true,
    },
  ],
  nonClaims: [
    'no-second-truth-store',
    'no-portal-ui',
    'no-raw-child-evidence',
    'no-unbounded-pagination',
    'no-provider-routing',
    'no-ocentra-hosted-family-data-custody',
  ],
  reportRuntimeClaimed: false,
  portalUiClaimed: false,
  providerRoutingClaimed: false,
  ocentraHostedFamilyDataCustodyClaimed: false,
  secondTruthStoreClaimed: false,
  rawChildEvidenceClaimed: false,
  updatedAt: '2026-06-28T15:55:00.000Z',
} as const;
