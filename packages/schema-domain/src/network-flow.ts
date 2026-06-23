import { type Infer, brandedNonEmptyStringSchema, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  ActivityCaptureCapabilityStatusSchema,
  ActivityDomainAttributionStatusSchema,
  ActivityNetworkProtocolSchema,
  ActivityNetworkTcpStateSchema,
  ActivityProcessAttributionStatusSchema,
} from './activity-capture';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityObserverSchema } from './evidence-kinds';
import { ActivityEventIdSchema, ActivityEvidenceIdSchema, ActivityTimestampSchema } from './evidence-primitives';

export const ActivityQuerySchemaVersion = 1;

const NetworkPortNumber = Schema.Number.pipe(Schema.int(), Schema.between(0, 65535));
const NetworkNonNegativeNumber = Schema.Number.pipe(Schema.nonNegative());
const NetworkNonNegativeInteger = NetworkNonNegativeNumber.pipe(Schema.int());

export const ActivityNetworkEndpointAddressSchema = withParser(
  brandedNonEmptyStringSchema('ActivityNetworkEndpointAddress')
);

export const ActivityNetworkDomainNameSchema = withParser(brandedNonEmptyStringSchema('ActivityNetworkDomainName'));

export const ActivityNetworkProcessNameSchema = withParser(brandedNonEmptyStringSchema('ActivityNetworkProcessName'));

export const ActivityNetworkAdapterIdSchema = withParser(brandedNonEmptyStringSchema('ActivityNetworkAdapterId'));

export const ActivityNetworkCustodyStateSchema = withParser(
  Schema.Literal(
    'live-local-child-agent',
    'live-lan-child-agent',
    'child-device-journal',
    'child-device-query-store',
    'parent-device-cache',
    'parent-owned-export',
    'ocentra-hosted-non-activity',
    'unavailable'
  )
);

export const ActivityNetworkFlowIndicatorKindSchema = withParser(
  Schema.Literal(
    'new-destination',
    'high-volume',
    'vpn-proxy-tunnel',
    'repeated-failure',
    'unusual-unknown-process',
    'adapter-unavailable',
    'encrypted-content-unavailable'
  )
);

export const ActivityNetworkFlowRowVisibilitySchema = withParser(Schema.Literal('active', 'tombstone'));

export const ActivityNetworkEndpointSchema = withParser(
  Schema.Struct({
    ip: Schema.Union(ActivityNetworkEndpointAddressSchema, Schema.Null),
    port: Schema.Union(NetworkPortNumber, Schema.Null),
  })
);

export const ActivityNetworkFlowCountersSchema = withParser(
  Schema.Struct({
    connectionCount: NetworkNonNegativeNumber,
    bytesSent: Schema.Union(NetworkNonNegativeNumber, Schema.Null),
    bytesReceived: Schema.Union(NetworkNonNegativeNumber, Schema.Null),
    firstSeenAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    lastSeenAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  })
);

export const ActivityNetworkFlowObservationSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    eventId: ActivityEventIdSchema,
    observedAt: ActivityTimestampSchema,
    observer: ActivityObserverSchema,
    capabilityStatus: ActivityCaptureCapabilityStatusSchema,
    adapterId: ActivityNetworkAdapterIdSchema,
    protocol: Schema.Union(ActivityNetworkProtocolSchema, Schema.Null),
    tcpState: Schema.Union(ActivityNetworkTcpStateSchema, Schema.Null),
    localEndpoint: ActivityNetworkEndpointSchema,
    destinationEndpoint: ActivityNetworkEndpointSchema,
    destinationDomain: Schema.Union(ActivityNetworkDomainNameSchema, Schema.Null),
    domainAttributionStatus: ActivityDomainAttributionStatusSchema,
    processAttributionStatus: ActivityProcessAttributionStatusSchema,
    processId: Schema.Union(NetworkNonNegativeInteger, Schema.Null),
    processName: Schema.Union(ActivityNetworkProcessNameSchema, Schema.Null),
    counters: ActivityNetworkFlowCountersSchema,
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  }).pipe(
    Schema.filter(
      (observation) =>
        observation.evidence.length > 0 || 'Network flow observations must include at least one evidence reference'
    )
  )
);

export const ActivityNetworkFlowReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    generatedAt: ActivityTimestampSchema,
    custody: ActivityNetworkCustodyStateSchema,
    limit: NetworkNonNegativeInteger,
    returned: NetworkNonNegativeInteger,
    activeRows: NetworkNonNegativeInteger,
    tombstoneRows: NetworkNonNegativeInteger,
    exportableRows: NetworkNonNegativeInteger,
    capabilityStatus: ActivityCaptureCapabilityStatusSchema,
    latestEventId: Schema.Union(ActivityEventIdSchema, Schema.Null),
    latestObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    latestTombstoneEventId: Schema.Union(ActivityEventIdSchema, Schema.Null),
    latestTombstoneObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    deletedEvidenceReferenceIds: Schema.Array(ActivityEvidenceIdSchema),
    rows: Schema.Array(ActivityNetworkFlowObservationSchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        (networkFlowCountsMatch(readModel) && networkFlowDeletionStateMatches(readModel)) ||
        'Network flow read-model counts must match visible rows and deletion/export state'
    )
  )
);

export const ActivityNetworkFlowRollupSchema = withParser(
  Schema.Struct({
    key: NonEmptyStringSchema,
    label: NonEmptyStringSchema,
    connectionCount: NetworkNonNegativeNumber,
    bytesSent: Schema.Union(NetworkNonNegativeNumber, Schema.Null),
    bytesReceived: Schema.Union(NetworkNonNegativeNumber, Schema.Null),
    evidenceIds: Schema.Array(ActivityEvidenceIdSchema),
  })
);

export const ActivityNetworkFlowIndicatorSchema = withParser(
  Schema.Struct({
    kind: ActivityNetworkFlowIndicatorKindSchema,
    label: NonEmptyStringSchema,
    observedAt: ActivityTimestampSchema,
    evidenceIds: Schema.Array(ActivityEvidenceIdSchema),
  })
);

export const ActivityNetworkFlowDigestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    generatedAt: ActivityTimestampSchema,
    custody: ActivityNetworkCustodyStateSchema,
    evidence: Schema.Array(ActivityEvidenceRefSchema),
    topProcesses: Schema.Array(ActivityNetworkFlowRollupSchema),
    topDestinations: Schema.Array(ActivityNetworkFlowRollupSchema),
    unusualIndicators: Schema.Array(ActivityNetworkFlowIndicatorSchema),
  })
);

export type ActivityNetworkEndpointAddress = Infer<typeof ActivityNetworkEndpointAddressSchema>;
export type ActivityNetworkDomainName = Infer<typeof ActivityNetworkDomainNameSchema>;
export type ActivityNetworkProcessName = Infer<typeof ActivityNetworkProcessNameSchema>;
export type ActivityNetworkAdapterId = Infer<typeof ActivityNetworkAdapterIdSchema>;
export type ActivityNetworkCustodyState = Infer<typeof ActivityNetworkCustodyStateSchema>;
export type ActivityNetworkFlowIndicatorKind = Infer<typeof ActivityNetworkFlowIndicatorKindSchema>;
export type ActivityNetworkFlowRowVisibility = Infer<typeof ActivityNetworkFlowRowVisibilitySchema>;
export type ActivityNetworkEndpoint = Infer<typeof ActivityNetworkEndpointSchema>;
export type ActivityNetworkFlowCounters = Infer<typeof ActivityNetworkFlowCountersSchema>;
export type ActivityNetworkFlowObservation = Infer<typeof ActivityNetworkFlowObservationSchema>;
export type ActivityNetworkFlowReadModel = Infer<typeof ActivityNetworkFlowReadModelSchema>;
export type ActivityNetworkFlowRollup = Infer<typeof ActivityNetworkFlowRollupSchema>;
export type ActivityNetworkFlowIndicator = Infer<typeof ActivityNetworkFlowIndicatorSchema>;
export type ActivityNetworkFlowDigest = Infer<typeof ActivityNetworkFlowDigestSchema>;

interface ActivityNetworkFlowReadModelCounts {
  readonly returned: number;
  readonly activeRows: number;
  readonly tombstoneRows: number;
  readonly exportableRows: number;
  readonly latestTombstoneEventId: string | null;
  readonly latestTombstoneObservedAt: string | null;
  readonly deletedEvidenceReferenceIds: readonly string[];
  readonly rows: readonly unknown[];
}

function networkFlowCountsMatch(readModel: ActivityNetworkFlowReadModelCounts): boolean {
  return (
    readModel.returned === readModel.rows.length &&
    readModel.activeRows === readModel.rows.length &&
    readModel.exportableRows <= readModel.activeRows
  );
}

function networkFlowDeletionStateMatches(readModel: ActivityNetworkFlowReadModelCounts): boolean {
  return (
    (readModel.tombstoneRows === 0 && readModel.deletedEvidenceReferenceIds.length === 0) ||
    (readModel.tombstoneRows > 0 &&
      readModel.rows.length === 0 &&
      readModel.exportableRows === 0 &&
      readModel.latestTombstoneEventId !== null &&
      readModel.latestTombstoneObservedAt !== null &&
      readModel.deletedEvidenceReferenceIds.length > 0)
  );
}
