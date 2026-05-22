import { AgentProtocolDefaults, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyTextSchema = Schema.String.pipe(Schema.minLength(1));
const NonNegativeCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableTextSchema = Schema.Union(NonEmptyTextSchema, Schema.Null);
const EvidenceReferenceSchema = Schema.Struct({
  evidenceReferenceId: NonEmptyTextSchema,
  kind: NonEmptyTextSchema,
  observedAt: NonEmptyTextSchema,
});
const ParentActionReferenceSchema = Schema.Struct({
  actionReferenceId: NonEmptyTextSchema,
  actor: Schema.Struct({
    actorId: NonEmptyTextSchema,
    role: NonEmptyTextSchema,
  }),
  policyVersion: NonEmptyTextSchema,
  createdAt: NonEmptyTextSchema,
});
const DeviceReferenceSchema = Schema.Struct({
  deviceId: NonEmptyTextSchema,
  childProfileId: NullableTextSchema,
  label: NonEmptyTextSchema,
  platform: NonEmptyTextSchema,
});
const ChildProfileReferenceSchema = Schema.Struct({
  childProfileId: NonEmptyTextSchema,
  displayName: NonEmptyTextSchema,
});

const ActivityMemoryGraphTraceSchema = withParser(
  Schema.Struct({
    entryStatus: Schema.Literal('usable', 'degraded', 'stale', 'rejected'),
    sourceEvidenceReferences: Schema.Array(EvidenceReferenceSchema),
    sourcePolicyVersion: NullableTextSchema,
    sourceParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    generatedAt: NonEmptyTextSchema,
    expiresAt: NullableTextSchema,
    confidence: Schema.Number.pipe(Schema.nonNegative()),
    derivedIndexVersion: NonEmptyTextSchema,
    degradedReasons: Schema.Array(NonEmptyTextSchema),
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
  graphId: NonEmptyTextSchema,
  nodeId: NonEmptyTextSchema,
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
  label: NonEmptyTextSchema,
  childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
  device: Schema.Union(DeviceReferenceSchema, Schema.Null),
  trace: ActivityMemoryGraphTraceSchema,
});
const ActivityMemoryGraphEdgeSchema = Schema.Struct({
  graphId: NonEmptyTextSchema,
  edgeId: NonEmptyTextSchema,
  edgeKind: Schema.Literal(
    'visited',
    'watched',
    'played',
    'active-during',
    'performed-by-child',
    'derived-from-evidence'
  ),
  fromNodeId: NonEmptyTextSchema,
  toNodeId: NonEmptyTextSchema,
  observedFrom: NonEmptyTextSchema,
  observedUntil: NullableTextSchema,
  durationMs: Schema.Union(NonNegativeCountSchema, Schema.Null),
  trace: ActivityMemoryGraphTraceSchema,
});
const ActivityMemoryGraphReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: NonNegativeCountSchema,
    generatedAt: NonEmptyTextSchema,
    custody: NonEmptyTextSchema,
    capabilityStatus: NonEmptyTextSchema,
    query: Schema.Struct({
      queryId: NonEmptyTextSchema,
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
        observedFrom: NonEmptyTextSchema,
        observedUntil: NonEmptyTextSchema,
      }),
      asOf: NonEmptyTextSchema,
      limit: NonNegativeCountSchema,
    }),
    readAt: NonEmptyTextSchema,
    nodes: Schema.Array(ActivityMemoryGraphNodeSchema),
    edges: Schema.Array(ActivityMemoryGraphEdgeSchema),
    returnedNodeCount: NonNegativeCountSchema,
    returnedEdgeCount: NonNegativeCountSchema,
    omittedEdgeCount: NonNegativeCountSchema,
    degradedReasons: Schema.Array(NonEmptyTextSchema),
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
