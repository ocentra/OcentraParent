import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import {
  ActivityCaptureCapabilityStatusSchema,
  ActivityDomainAttributionStatusSchema,
  ActivityNetworkProtocolSchema,
  ActivityNetworkTcpStateSchema,
  ActivityProcessAttributionStatusSchema,
} from './capture';
import { ActivityObserverSchema } from './kinds';
import { ActivityEventIdSchema, ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { ActivityQuerySchemaVersion } from './query';

export * from './network-contracts';

const NetworkNonEmptyText = Schema.String.pipe(Schema.minLength(1));
const NetworkPortNumber = Schema.Number.pipe(Schema.int(), Schema.between(0, 65535));
const NetworkNonNegativeNumber = Schema.Number.pipe(Schema.nonNegative());
const NetworkNonNegativeInteger = NetworkNonNegativeNumber.pipe(Schema.int());

export const ActivityNetworkEndpointAddressSchema = withParser(
  NetworkNonEmptyText.pipe(Schema.brand('ActivityNetworkEndpointAddress'))
);

export const ActivityNetworkDomainNameSchema = withParser(
  NetworkNonEmptyText.pipe(Schema.brand('ActivityNetworkDomainName'))
);

export const ActivityNetworkProcessNameSchema = withParser(
  NetworkNonEmptyText.pipe(Schema.brand('ActivityNetworkProcessName'))
);

export const ActivityNetworkAdapterIdSchema = withParser(
  NetworkNonEmptyText.pipe(Schema.brand('ActivityNetworkAdapterId'))
);

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
  })
);

export const ActivityNetworkFlowReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityQuerySchemaVersion),
    generatedAt: ActivityTimestampSchema,
    custody: ActivityNetworkCustodyStateSchema,
    limit: NetworkNonNegativeInteger,
    returned: NetworkNonNegativeInteger,
    capabilityStatus: ActivityCaptureCapabilityStatusSchema,
    rows: Schema.Array(ActivityNetworkFlowObservationSchema),
  })
);

export const ActivityNetworkFlowRollupSchema = withParser(
  Schema.Struct({
    key: NetworkNonEmptyText,
    label: NetworkNonEmptyText,
    connectionCount: NetworkNonNegativeNumber,
    bytesSent: Schema.Union(NetworkNonNegativeNumber, Schema.Null),
    bytesReceived: Schema.Union(NetworkNonNegativeNumber, Schema.Null),
    evidenceIds: Schema.Array(ActivityEvidenceIdSchema),
  })
);

export const ActivityNetworkFlowIndicatorSchema = withParser(
  Schema.Struct({
    kind: ActivityNetworkFlowIndicatorKindSchema,
    label: NetworkNonEmptyText,
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
export type ActivityNetworkEndpoint = Infer<typeof ActivityNetworkEndpointSchema>;
export type ActivityNetworkFlowCounters = Infer<typeof ActivityNetworkFlowCountersSchema>;
export type ActivityNetworkFlowObservation = Infer<typeof ActivityNetworkFlowObservationSchema>;
export type ActivityNetworkFlowReadModel = Infer<typeof ActivityNetworkFlowReadModelSchema>;
export type ActivityNetworkFlowRollup = Infer<typeof ActivityNetworkFlowRollupSchema>;
export type ActivityNetworkFlowIndicator = Infer<typeof ActivityNetworkFlowIndicatorSchema>;
export type ActivityNetworkFlowDigest = Infer<typeof ActivityNetworkFlowDigestSchema>;
