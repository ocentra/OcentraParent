/* generated from crates/schema/src/report_query_custody.rs */

export const ReportQueryCustodyContractRuntime = {
  SchemaVersion: 'report-query-custody-proof',
} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export type GeneratedParentAccountId = string;
export type GeneratedFamilyId = string;
export type GeneratedChildProfileId = string;
export type GeneratedParentDeviceId = string;
export type GeneratedParentDeviceLabel = string;
export type GeneratedParentActorId = string;
export type GeneratedParentPolicyVersion = string;
export type GeneratedParentEvidenceReferenceId = string;
export type GeneratedParentActionReferenceId = string;
export type GeneratedParentTimestamp = string;
export type GeneratedReportQueryCustodyRequestId = string;
export type GeneratedReportQueryCustodyQueryCursor = string;
export type GeneratedReportQueryCustodyCursorRef = string;
export type GeneratedReportQueryCustodySortKey = string;
export type GeneratedReportQueryCustodySourceRef = string;
export type GeneratedReportQueryCustodyConflictRef = string;
export type GeneratedReportQueryCustodyDeletedSourceRef = string;

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

export const GeneratedParentPlatforms = ['windows', 'linux', 'macos', 'android', 'ios'] as const satisfies readonly GeneratedParentPlatform[];
export const GeneratedParentActorRoles = ['parent', 'guardian', 'system'] as const satisfies readonly GeneratedParentActorRole[];
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
  actorId: GeneratedParentActorId;
  role: GeneratedParentActorRole;
}

export interface GeneratedParentAccountReference {
  parentAccountId: GeneratedParentAccountId;
}

export interface GeneratedFamilyReference {
  familyId: GeneratedFamilyId;
}

export interface GeneratedParentDeviceReference {
  deviceId: GeneratedParentDeviceId;
  childProfileId: GeneratedChildProfileId | null;
  label: GeneratedParentDeviceLabel;
  platform: GeneratedParentPlatform;
}

export interface GeneratedParentEvidenceReference {
  evidenceReferenceId: GeneratedParentEvidenceReferenceId;
  kind: GeneratedParentEvidenceReferenceKind;
  observedAt: GeneratedParentTimestamp;
}

export interface GeneratedParentActionReference {
  actionReferenceId: GeneratedParentActionReferenceId;
  actor: GeneratedParentActorReference;
  policyVersion: GeneratedParentPolicyVersion;
  createdAt: GeneratedParentTimestamp;
}

export interface GeneratedReportQueryCustodyRequest {
  schemaVersion: typeof ReportQueryCustodyContractRuntime.SchemaVersion;
  requestId: GeneratedReportQueryCustodyRequestId;
  family: GeneratedFamilyReference;
  account: GeneratedParentAccountReference;
  device: GeneratedParentDeviceReference;
  parentAction: GeneratedParentActionReference;
  requestedCursor: GeneratedReportQueryCustodyQueryCursor;
  pageSize: number;
  requestedDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  allowedSourceDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  sourceCitationRefs: readonly GeneratedParentEvidenceReference[];
  assistantCitationRefs: readonly GeneratedParentEvidenceReference[];
  notificationPayloadBoundary: GeneratedReportQueryCustodyBoundary;
  parentAuthorized: boolean;
  parentOwnedSourceRequired: boolean;
  rawChildEvidenceRequested: boolean;
}

export interface GeneratedReportQueryCustodyRow {
  rowId: GeneratedReportQueryCustodySourceRef;
  requestId: GeneratedReportQueryCustodyRequestId;
  state: GeneratedReportQueryCustodyState;
  sourceFreshness: GeneratedReportQueryCustodySourceFreshness;
  sourceDataClass: GeneratedReportQueryCustodySourceDataClass;
  cursorRef: GeneratedReportQueryCustodyCursorRef;
  sourceCursorRef: GeneratedReportQueryCustodyCursorRef;
  nextCursorRef: GeneratedReportQueryCustodyCursorRef | null;
  pageIndex: number;
  pageSize: number;
  stableSortKey: GeneratedReportQueryCustodySortKey;
  requestedDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  allowedSourceDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  sourceCitationRefs: readonly GeneratedParentEvidenceReference[];
  assistantCitationRefs: readonly GeneratedParentEvidenceReference[];
  notificationPayloadBoundary: GeneratedReportQueryCustodyBoundary;
  payloadRedactionState: GeneratedReportQueryCustodyPayloadRedaction;
  tombstoneState: GeneratedReportQueryCustodyTombstoneState;
  deletedSourceRef: GeneratedReportQueryCustodyDeletedSourceRef | null;
  deletedSourceAt: GeneratedParentTimestamp | null;
  conflictRef: GeneratedReportQueryCustodyConflictRef | null;
  cursorExpiredAt: GeneratedParentTimestamp | null;
  rateLimitedUntilAt: GeneratedParentTimestamp | null;
  parentAuthorized: boolean;
  parentOwnedSourceRequired: boolean;
  rawChildEvidenceIncluded: boolean;
  reportCacheMutated: boolean;
  secondTruthStoreClaimed: boolean;
  claimSafe: boolean;
}

export interface GeneratedReportQueryCustodyContractProof {
  schemaVersion: typeof ReportQueryCustodyContractRuntime.SchemaVersion;
  contractVersion: GeneratedParentContractSchemaVersion;
  request: GeneratedReportQueryCustodyRequest;
  rows: readonly GeneratedReportQueryCustodyRow[];
  nonClaims: readonly GeneratedReportQueryCustodyNonClaim[];
  reportRuntimeClaimed: boolean;
  portalUiClaimed: boolean;
  providerRoutingClaimed: boolean;
  ocentraHostedFamilyDataCustodyClaimed: boolean;
  secondTruthStoreClaimed: boolean;
  rawChildEvidenceClaimed: boolean;
  updatedAt: GeneratedParentTimestamp;
}

export const GeneratedReportQueryCustodyKnownGaps = [
  "No uncontrolled second truth store is claimed for report or query custody.",
  "No portal rendering, provider routing, or raw child evidence handling is claimed.",
  "Pagination is modeled as stable derived state over governed evidence, not a second report store.",
  "Delete, tombstone, stale, conflict, and rate-limit outcomes stay explicit and claim-safe.",
  "Assistant and report citations stay inside query-store-summary evidence refs only.",
] as const;

export const GeneratedReportQueryCustodyContractProof = {
  "schemaVersion": "report-query-custody-proof",
  "contractVersion": "v0.6",
  "request": {
    "schemaVersion": "report-query-custody-proof",
    "requestId": "report-query-custody-request-proof-1",
    "family": {
      "familyId": "family-report-query-custody-proof-1"
    },
    "account": {
      "parentAccountId": "parent-account-report-query-custody-proof-1"
    },
    "device": {
      "deviceId": "windows-parent-device-report-query-custody-proof-1",
      "childProfileId": null,
      "label": "Windows parent device report query custody proof",
      "platform": "windows"
    },
    "parentAction": {
      "actionReferenceId": "parent-action-report-query-custody-proof-1",
      "actor": {
        "actorId": "parent-report-query-custody-proof-1",
        "role": "parent"
      },
      "policyVersion": "report-query-custody-proof-v1",
      "createdAt": "2026-06-28T15:55:00.000Z"
    },
    "requestedCursor": "report-query-custody-cursor-proof-1",
    "pageSize": 25,
    "requestedDataClasses": [
      "sqlite-query-row",
      "notification-history",
      "audit-event",
      "generated-summary"
    ],
    "allowedSourceDataClasses": [
      "sqlite-query-row",
      "notification-history",
      "audit-event",
      "generated-summary"
    ],
    "sourceCitationRefs": [
      {
        "evidenceReferenceId": "report-query-custody-evidence-1",
        "kind": "query-store-summary",
        "observedAt": "2026-06-28T15:55:00.000Z"
      },
      {
        "evidenceReferenceId": "report-query-custody-evidence-2",
        "kind": "query-store-summary",
        "observedAt": "2026-06-28T15:55:00.000Z"
      }
    ],
    "assistantCitationRefs": [
      {
        "evidenceReferenceId": "report-query-custody-evidence-1",
        "kind": "query-store-summary",
        "observedAt": "2026-06-28T15:55:00.000Z"
      },
      {
        "evidenceReferenceId": "report-query-custody-evidence-2",
        "kind": "query-store-summary",
        "observedAt": "2026-06-28T15:55:00.000Z"
      }
    ],
    "notificationPayloadBoundary": "parent-owned-citations-only",
    "parentAuthorized": true,
    "parentOwnedSourceRequired": true,
    "rawChildEvidenceRequested": false
  },
  "rows": [
    {
      "rowId": "report-query-custody-row-derivedFresh",
      "requestId": "report-query-custody-request-proof-1",
      "state": "derivedFresh",
      "sourceFreshness": "fresh",
      "sourceDataClass": "sqlite-query-row",
      "cursorRef": "report-query-custody-cursor-derivedFresh",
      "sourceCursorRef": "report-query-custody-source-cursor-proof-1",
      "nextCursorRef": "derived-fresh-next-cursor",
      "pageIndex": 1,
      "pageSize": 25,
      "stableSortKey": "report-query-custody-stable-sort-key",
      "requestedDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "allowedSourceDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "sourceCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "assistantCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "notificationPayloadBoundary": "parent-owned-citations-only",
      "payloadRedactionState": "fully-redacted",
      "tombstoneState": "not-required",
      "deletedSourceRef": null,
      "deletedSourceAt": null,
      "conflictRef": null,
      "cursorExpiredAt": null,
      "rateLimitedUntilAt": null,
      "parentAuthorized": true,
      "parentOwnedSourceRequired": true,
      "rawChildEvidenceIncluded": false,
      "reportCacheMutated": false,
      "secondTruthStoreClaimed": false,
      "claimSafe": true
    },
    {
      "rowId": "report-query-custody-row-derivedStale",
      "requestId": "report-query-custody-request-proof-1",
      "state": "derivedStale",
      "sourceFreshness": "stale",
      "sourceDataClass": "generated-summary",
      "cursorRef": "report-query-custody-cursor-derivedStale",
      "sourceCursorRef": "report-query-custody-source-cursor-proof-1",
      "nextCursorRef": "derived-stale-next-cursor",
      "pageIndex": 2,
      "pageSize": 25,
      "stableSortKey": "report-query-custody-stable-sort-key",
      "requestedDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "allowedSourceDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "sourceCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "assistantCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "notificationPayloadBoundary": "parent-owned-citations-only",
      "payloadRedactionState": "fully-redacted",
      "tombstoneState": "not-required",
      "deletedSourceRef": null,
      "deletedSourceAt": null,
      "conflictRef": null,
      "cursorExpiredAt": null,
      "rateLimitedUntilAt": null,
      "parentAuthorized": true,
      "parentOwnedSourceRequired": true,
      "rawChildEvidenceIncluded": false,
      "reportCacheMutated": false,
      "secondTruthStoreClaimed": false,
      "claimSafe": true
    },
    {
      "rowId": "report-query-custody-row-partiallyRedacted",
      "requestId": "report-query-custody-request-proof-1",
      "state": "partiallyRedacted",
      "sourceFreshness": "stale",
      "sourceDataClass": "notification-history",
      "cursorRef": "report-query-custody-cursor-partiallyRedacted",
      "sourceCursorRef": "report-query-custody-source-cursor-proof-1",
      "nextCursorRef": "partially-redacted-next-cursor",
      "pageIndex": 3,
      "pageSize": 25,
      "stableSortKey": "report-query-custody-stable-sort-key",
      "requestedDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "allowedSourceDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "sourceCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "assistantCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "notificationPayloadBoundary": "parent-owned-citations-only",
      "payloadRedactionState": "partially-redacted",
      "tombstoneState": "not-required",
      "deletedSourceRef": null,
      "deletedSourceAt": null,
      "conflictRef": null,
      "cursorExpiredAt": null,
      "rateLimitedUntilAt": null,
      "parentAuthorized": true,
      "parentOwnedSourceRequired": true,
      "rawChildEvidenceIncluded": false,
      "reportCacheMutated": false,
      "secondTruthStoreClaimed": false,
      "claimSafe": true
    },
    {
      "rowId": "report-query-custody-row-deletedSource",
      "requestId": "report-query-custody-request-proof-1",
      "state": "deletedSource",
      "sourceFreshness": "deleted",
      "sourceDataClass": "audit-event",
      "cursorRef": "report-query-custody-cursor-deletedSource",
      "sourceCursorRef": "report-query-custody-source-cursor-proof-1",
      "nextCursorRef": null,
      "pageIndex": 4,
      "pageSize": 25,
      "stableSortKey": "report-query-custody-stable-sort-key",
      "requestedDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "allowedSourceDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "sourceCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "assistantCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "notificationPayloadBoundary": "parent-owned-citations-only",
      "payloadRedactionState": "fully-redacted",
      "tombstoneState": "written",
      "deletedSourceRef": "deleted-source-ref-1",
      "deletedSourceAt": "2026-06-28T15:57:00.000Z",
      "conflictRef": null,
      "cursorExpiredAt": null,
      "rateLimitedUntilAt": null,
      "parentAuthorized": true,
      "parentOwnedSourceRequired": true,
      "rawChildEvidenceIncluded": false,
      "reportCacheMutated": false,
      "secondTruthStoreClaimed": false,
      "claimSafe": true
    },
    {
      "rowId": "report-query-custody-row-syncConflict",
      "requestId": "report-query-custody-request-proof-1",
      "state": "syncConflict",
      "sourceFreshness": "conflicted",
      "sourceDataClass": "sqlite-query-row",
      "cursorRef": "report-query-custody-cursor-syncConflict",
      "sourceCursorRef": "report-query-custody-source-cursor-proof-1",
      "nextCursorRef": "sync-conflict-next-cursor",
      "pageIndex": 5,
      "pageSize": 25,
      "stableSortKey": "report-query-custody-stable-sort-key",
      "requestedDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "allowedSourceDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "sourceCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "assistantCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "notificationPayloadBoundary": "parent-owned-citations-only",
      "payloadRedactionState": "fully-redacted",
      "tombstoneState": "not-required",
      "deletedSourceRef": null,
      "deletedSourceAt": null,
      "conflictRef": "conflict-ref-1",
      "cursorExpiredAt": null,
      "rateLimitedUntilAt": null,
      "parentAuthorized": true,
      "parentOwnedSourceRequired": true,
      "rawChildEvidenceIncluded": false,
      "reportCacheMutated": false,
      "secondTruthStoreClaimed": false,
      "claimSafe": true
    },
    {
      "rowId": "report-query-custody-row-cursorExpired",
      "requestId": "report-query-custody-request-proof-1",
      "state": "cursorExpired",
      "sourceFreshness": "expired",
      "sourceDataClass": "generated-summary",
      "cursorRef": "report-query-custody-cursor-cursorExpired",
      "sourceCursorRef": "report-query-custody-source-cursor-proof-1",
      "nextCursorRef": null,
      "pageIndex": 6,
      "pageSize": 25,
      "stableSortKey": "report-query-custody-stable-sort-key",
      "requestedDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "allowedSourceDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "sourceCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "assistantCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "notificationPayloadBoundary": "parent-owned-citations-only",
      "payloadRedactionState": "fully-redacted",
      "tombstoneState": "not-required",
      "deletedSourceRef": null,
      "deletedSourceAt": null,
      "conflictRef": null,
      "cursorExpiredAt": "2026-06-28T15:59:00.000Z",
      "rateLimitedUntilAt": null,
      "parentAuthorized": true,
      "parentOwnedSourceRequired": true,
      "rawChildEvidenceIncluded": false,
      "reportCacheMutated": false,
      "secondTruthStoreClaimed": false,
      "claimSafe": true
    },
    {
      "rowId": "report-query-custody-row-rateLimited",
      "requestId": "report-query-custody-request-proof-1",
      "state": "rateLimited",
      "sourceFreshness": "rate-limited",
      "sourceDataClass": "notification-history",
      "cursorRef": "report-query-custody-cursor-rateLimited",
      "sourceCursorRef": "report-query-custody-source-cursor-proof-1",
      "nextCursorRef": null,
      "pageIndex": 7,
      "pageSize": 25,
      "stableSortKey": "report-query-custody-stable-sort-key",
      "requestedDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "allowedSourceDataClasses": [
        "sqlite-query-row",
        "notification-history",
        "audit-event",
        "generated-summary"
      ],
      "sourceCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "assistantCitationRefs": [
        {
          "evidenceReferenceId": "report-query-custody-evidence-1",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        },
        {
          "evidenceReferenceId": "report-query-custody-evidence-2",
          "kind": "query-store-summary",
          "observedAt": "2026-06-28T15:55:00.000Z"
        }
      ],
      "notificationPayloadBoundary": "parent-owned-citations-only",
      "payloadRedactionState": "fully-redacted",
      "tombstoneState": "not-required",
      "deletedSourceRef": null,
      "deletedSourceAt": null,
      "conflictRef": null,
      "cursorExpiredAt": null,
      "rateLimitedUntilAt": "2026-06-28T16:05:00.000Z",
      "parentAuthorized": true,
      "parentOwnedSourceRequired": true,
      "rawChildEvidenceIncluded": false,
      "reportCacheMutated": false,
      "secondTruthStoreClaimed": false,
      "claimSafe": true
    }
  ],
  "nonClaims": [
    "no-second-truth-store",
    "no-portal-ui",
    "no-raw-child-evidence",
    "no-unbounded-pagination",
    "no-provider-routing",
    "no-ocentra-hosted-family-data-custody"
  ],
  "reportRuntimeClaimed": false,
  "portalUiClaimed": false,
  "providerRoutingClaimed": false,
  "ocentraHostedFamilyDataCustodyClaimed": false,
  "secondTruthStoreClaimed": false,
  "rawChildEvidenceClaimed": false,
  "updatedAt": "2026-06-28T15:55:00.000Z"
} as const satisfies GeneratedReportQueryCustodyContractProof;
