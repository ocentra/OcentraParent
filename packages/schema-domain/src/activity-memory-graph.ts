import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  withParser,
} from './effect';

const NonNegativeCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);

const ActivityMemoryGraphEvidenceReferenceSchema = Schema.Struct({
  evidenceReferenceId: NonEmptyStringSchema,
  kind: NonEmptyStringSchema,
  observedAt: NonEmptyStringSchema,
});

const ActivityMemoryGraphParentActionReferenceSchema = Schema.Struct({
  actionReferenceId: NonEmptyStringSchema,
  actor: Schema.Struct({
    actorId: NonEmptyStringSchema,
    role: NonEmptyStringSchema,
  }),
  policyVersion: NonEmptyStringSchema,
  createdAt: NonEmptyStringSchema,
});

const ActivityMemoryGraphDeviceReferenceSchema = Schema.Struct({
  deviceId: NonEmptyStringSchema,
  childProfileId: NullableTextSchema,
  label: NonEmptyStringSchema,
  platform: NonEmptyStringSchema,
});

const ActivityMemoryGraphChildProfileReferenceSchema = Schema.Struct({
  childProfileId: NonEmptyStringSchema,
  displayName: NonEmptyStringSchema,
});

export const ActivityMemoryGraphEntryStatusSchema = withParser(
  Schema.Literal('usable', 'degraded', 'stale', 'rejected')
);

export const ActivityMemoryGraphNodeKindSchema = withParser(
  Schema.Literal(
    'child-profile',
    'device',
    'browser-url',
    'domain',
    'video',
    'app',
    'game',
    'activity-session'
  )
);

export const ActivityMemoryGraphEdgeKindSchema = withParser(
  Schema.Literal(
    'visited',
    'watched',
    'played',
    'active-during',
    'performed-by-child',
    'derived-from-evidence'
  )
);

export const ActivityMemoryGraphQueryKindSchema = withParser(
  Schema.Literal(
    'visited-urls',
    'played-games',
    'watched-videos',
    'activity-by-time-range',
    'explain-evidence'
  )
);

export const ActivityMemoryGraphTraceSchema = withParser(
  Schema.Struct({
    entryStatus: ActivityMemoryGraphEntryStatusSchema,
    sourceEvidenceReferences: Schema.Array(ActivityMemoryGraphEvidenceReferenceSchema),
    sourcePolicyVersion: NullableTextSchema,
    sourceParentActionReferences: Schema.Array(ActivityMemoryGraphParentActionReferenceSchema),
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

export const ActivityMemoryGraphNodeSchema = withParser(
  Schema.Struct({
    graphId: NonEmptyStringSchema,
    nodeId: NonEmptyStringSchema,
    nodeKind: ActivityMemoryGraphNodeKindSchema,
    label: NonEmptyStringSchema,
    childProfile: Schema.Union(ActivityMemoryGraphChildProfileReferenceSchema, Schema.Null),
    device: Schema.Union(ActivityMemoryGraphDeviceReferenceSchema, Schema.Null),
    trace: ActivityMemoryGraphTraceSchema,
  })
);

export const ActivityMemoryGraphEdgeSchema = withParser(
  Schema.Struct({
    graphId: NonEmptyStringSchema,
    edgeId: NonEmptyStringSchema,
    edgeKind: ActivityMemoryGraphEdgeKindSchema,
    fromNodeId: NonEmptyStringSchema,
    toNodeId: NonEmptyStringSchema,
    observedFrom: NonEmptyStringSchema,
    observedUntil: NullableTextSchema,
    durationMs: Schema.Union(NonNegativeCountSchema, Schema.Null),
    trace: ActivityMemoryGraphTraceSchema,
  })
);

export const ActivityMemoryGraphReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: NonNegativeCountSchema,
    generatedAt: NonEmptyStringSchema,
    custody: NonEmptyStringSchema,
    capabilityStatus: NonEmptyStringSchema,
    query: Schema.Struct({
      queryId: NonEmptyStringSchema,
      queryKind: ActivityMemoryGraphQueryKindSchema,
      childProfile: Schema.Union(ActivityMemoryGraphChildProfileReferenceSchema, Schema.Null),
      device: ActivityMemoryGraphDeviceReferenceSchema,
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
    Schema.filter(
      (model) => model.returnedEdgeCount === model.edges.length || 'Expected memory graph edge count match'
    )
  )
);

export type ActivityMemoryGraphReadModel = Infer<typeof ActivityMemoryGraphReadModelSchema>;
export type ActivityMemoryGraphNode = ActivityMemoryGraphReadModel['nodes'][number];
export type ActivityMemoryGraphEdge = ActivityMemoryGraphReadModel['edges'][number];
export type ActivityMemoryGraphNodeId = ActivityMemoryGraphNode['nodeId'];

export function parseActivityMemoryGraphDigest(digest: string): ActivityMemoryGraphReadModel | null {
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
