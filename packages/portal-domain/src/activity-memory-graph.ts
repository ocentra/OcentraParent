import { AgentProtocolDefaults, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
const NonNegativeCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
const EvidenceReferenceSchema = Schema.Struct({
  evidenceReferenceId: NonEmptyStringSchema,
  kind: NonEmptyStringSchema,
  observedAt: NonEmptyStringSchema,
});
const ParentActionReferenceSchema = Schema.Struct({
  actionReferenceId: NonEmptyStringSchema,
  actor: Schema.Struct({
    actorId: NonEmptyStringSchema,
    role: NonEmptyStringSchema,
  }),
  policyVersion: NonEmptyStringSchema,
  createdAt: NonEmptyStringSchema,
});
const DeviceReferenceSchema = Schema.Struct({
  deviceId: NonEmptyStringSchema,
  childProfileId: NullableTextSchema,
  label: NonEmptyStringSchema,
  platform: NonEmptyStringSchema,
});
const ChildProfileReferenceSchema = Schema.Struct({
  childProfileId: NonEmptyStringSchema,
  displayName: NonEmptyStringSchema,
});

const ActivityMemoryGraphTraceSchema = withParser(
  Schema.Struct({
    entryStatus: Schema.Literal('usable', 'degraded', 'stale', 'rejected'),
    sourceEvidenceReferences: Schema.Array(EvidenceReferenceSchema),
    sourcePolicyVersion: NullableTextSchema,
    sourceParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    generatedAt: NonEmptyStringSchema,
    expiresAt: NullableTextSchema,
    confidence: Schema.Number.pipe(Schema.nonNegative()),
    derivedIndexVersion: NonEmptyStringSchema,
    degradedReasons: Schema.Array(NonEmptyStringSchema),
  }).pipe(
    Schema.filter(
      (trace) =>
        trace.sourceEvidenceReferences.length > 0 ||
        trace.sourcePolicyVersion !== null ||
        trace.sourceParentActionReferences.length > 0 ||
        'Expected activity memory graph trace to cite source context'
    )
  )
);

const ActivityMemoryGraphNodeSchema = Schema.Struct({
  graphId: NonEmptyStringSchema,
  nodeId: NonEmptyStringSchema,
  nodeKind: Schema.Literal(
    'child-profile',
    'device',
    'browser-url',
    'domain',
    'video',
    'app',
    'game',
    'activity-session'
  ),
  label: NonEmptyStringSchema,
  childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
  device: Schema.Union(DeviceReferenceSchema, Schema.Null),
  trace: ActivityMemoryGraphTraceSchema,
});
const ActivityMemoryGraphEdgeSchema = Schema.Struct({
  graphId: NonEmptyStringSchema,
  edgeId: NonEmptyStringSchema,
  edgeKind: Schema.Literal(
    'visited',
    'watched',
    'played',
    'active-during',
    'performed-by-child',
    'derived-from-evidence'
  ),
  fromNodeId: NonEmptyStringSchema,
  toNodeId: NonEmptyStringSchema,
  observedFrom: NonEmptyStringSchema,
  observedUntil: NullableTextSchema,
  durationMs: Schema.Union(NonNegativeCountSchema, Schema.Null),
  trace: ActivityMemoryGraphTraceSchema,
});
const ActivityMemoryGraphReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: NonNegativeCountSchema,
    generatedAt: NonEmptyStringSchema,
    custody: NonEmptyStringSchema,
    capabilityStatus: NonEmptyStringSchema,
    query: Schema.Struct({
      queryId: NonEmptyStringSchema,
      queryKind: Schema.Literal(
        'visited-urls',
        'played-games',
        'watched-videos',
        'activity-by-time-range',
        'explain-evidence'
      ),
      childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
      device: DeviceReferenceSchema,
      timeRange: Schema.Struct({
        observedFrom: NonEmptyStringSchema,
        observedUntil: NonEmptyStringSchema,
      }),
      asOf: NonEmptyStringSchema,
      limit: NonNegativeCountSchema,
    }),
    readAt: NonEmptyStringSchema,
    nodes: Schema.Array(ActivityMemoryGraphNodeSchema),
    edges: Schema.Array(ActivityMemoryGraphEdgeSchema),
    returnedNodeCount: NonNegativeCountSchema,
    returnedEdgeCount: NonNegativeCountSchema,
    omittedEdgeCount: NonNegativeCountSchema,
    degradedReasons: Schema.Array(NonEmptyStringSchema),
  }).pipe(
    Schema.filter(
      (model) => model.returnedNodeCount === model.nodes.length || 'Expected memory graph node count match'
    ),
    Schema.filter((model) => model.returnedEdgeCount === model.edges.length || 'Expected memory graph edge count match')
  )
);

export type PortalActivityMemoryGraphReadModel = Infer<typeof ActivityMemoryGraphReadModelSchema>;
export type PortalActivityMemoryGraphNode = PortalActivityMemoryGraphReadModel['nodes'][number];
export type PortalActivityMemoryGraphEdge = PortalActivityMemoryGraphReadModel['edges'][number];
export type PortalActivityMemoryGraphNodeId = PortalActivityMemoryGraphNode['nodeId'];

export function parseActivityMemoryGraphReadModel(
  payload: AgentProtocolLogFields
): PortalActivityMemoryGraphReadModel | null {
  const digest = payload[AgentProtocolDefaults.Field.ActivityDigest];
  if (typeof digest !== 'string') {
    return null;
  }
  const decoded = parseDigest(digest);
  if (decoded === null) {
    return null;
  }
  const parsed = ActivityMemoryGraphReadModelSchema.safeParse(decoded);
  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function parseDigest(digest: string): unknown {
  try {
    return JSON.parse(digest) as unknown;
  } catch {
    return null;
  }
}

