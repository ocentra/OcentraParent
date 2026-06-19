import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { LocalAiContextNonNegativeCountSchema, LocalAiContextReasonCodeSchema } from './local-ai-context-primitives';
import { LocalAiConfidenceSchema, LocalAiDerivedIndexVersionSchema, LocalAiTimestampSchema, } from './local-ai-primitives';
import { ParentPolicyVersionSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { ChildProfileReferenceSchema, ParentActionReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema, } from '@ocentra-parent/family-domain/references';
const LocalAiActivityMemoryGraphIdSchema = brandedNonEmptyStringSchema('LocalAiActivityMemoryGraphId');
const LocalAiActivityMemoryGraphNodeIdSchema = brandedNonEmptyStringSchema('LocalAiActivityMemoryGraphNodeId');
const LocalAiActivityMemoryGraphEdgeIdSchema = brandedNonEmptyStringSchema('LocalAiActivityMemoryGraphEdgeId');
const LocalAiActivityMemoryGraphQueryIdSchema = brandedNonEmptyStringSchema('LocalAiActivityMemoryGraphQueryId');
const LocalAiActivityMemoryLabelSchema = brandedNonEmptyStringSchema('LocalAiActivityMemoryLabel');
const LocalAiActivityMemoryDurationMsSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const LocalAiActivityMemoryGraphNodeKindSchema = withParser(Schema.Literal('child-profile', 'device', 'browser-url', 'domain', 'video', 'app', 'game', 'activity-session'));
const LocalAiActivityMemoryGraphEdgeKindSchema = withParser(Schema.Literal('visited', 'watched', 'played', 'active-during', 'performed-by-child', 'derived-from-evidence'));
const LocalAiActivityMemoryGraphQueryKindSchema = withParser(Schema.Literal('visited-urls', 'played-games', 'watched-videos', 'activity-by-time-range', 'explain-evidence'));
const LocalAiActivityMemoryGraphEntryStatusSchema = withParser(Schema.Literal('candidate', 'usable', 'degraded', 'stale', 'rejected'));
export const LocalAiActivityMemoryGraphTraceSchema = withParser(Schema.Struct({
    entryStatus: LocalAiActivityMemoryGraphEntryStatusSchema,
    sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    sourcePolicyVersion: Schema.Union(ParentPolicyVersionSchema, Schema.Null),
    sourceParentActionReferences: Schema.Array(ParentActionReferenceSchema),
    generatedAt: LocalAiTimestampSchema,
    expiresAt: Schema.Union(LocalAiTimestampSchema, Schema.Null),
    confidence: LocalAiConfidenceSchema,
    derivedIndexVersion: LocalAiDerivedIndexVersionSchema,
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
}).pipe(Schema.filter((trace) => trace.sourceEvidenceReferences.length > 0 ||
    trace.sourcePolicyVersion !== null ||
    trace.sourceParentActionReferences.length > 0 ||
    'Expected memory graph entry to cite source context')));
export const LocalAiActivityMemoryTimeRangeSchema = withParser(Schema.Struct({
    observedFrom: LocalAiTimestampSchema,
    observedUntil: LocalAiTimestampSchema,
}).pipe(Schema.filter((range) => Date.parse(range.observedUntil) >= Date.parse(range.observedFrom) ||
    'Expected memory graph time range to be ordered')));
export const LocalAiActivityMemoryGraphNodeSchema = withParser(Schema.Struct({
    graphId: LocalAiActivityMemoryGraphIdSchema,
    nodeId: LocalAiActivityMemoryGraphNodeIdSchema,
    nodeKind: LocalAiActivityMemoryGraphNodeKindSchema,
    label: LocalAiActivityMemoryLabelSchema,
    childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
    device: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    trace: LocalAiActivityMemoryGraphTraceSchema,
}));
export const LocalAiActivityMemoryGraphEdgeSchema = withParser(Schema.Struct({
    graphId: LocalAiActivityMemoryGraphIdSchema,
    edgeId: LocalAiActivityMemoryGraphEdgeIdSchema,
    edgeKind: LocalAiActivityMemoryGraphEdgeKindSchema,
    fromNodeId: LocalAiActivityMemoryGraphNodeIdSchema,
    toNodeId: LocalAiActivityMemoryGraphNodeIdSchema,
    observedFrom: LocalAiTimestampSchema,
    observedUntil: Schema.Union(LocalAiTimestampSchema, Schema.Null),
    durationMs: Schema.Union(LocalAiActivityMemoryDurationMsSchema, Schema.Null),
    trace: LocalAiActivityMemoryGraphTraceSchema,
}).pipe(Schema.filter((edge) => edge.fromNodeId !== edge.toNodeId || 'Expected memory graph edge endpoints to differ'), Schema.filter((edge) => edge.observedUntil === null ||
    Date.parse(edge.observedUntil) >= Date.parse(edge.observedFrom) ||
    'Expected memory graph edge time range to be ordered')));
export const LocalAiActivityMemoryGraphQuerySchema = withParser(Schema.Struct({
    queryId: LocalAiActivityMemoryGraphQueryIdSchema,
    queryKind: LocalAiActivityMemoryGraphQueryKindSchema,
    childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
    device: ParentDeviceReferenceSchema,
    timeRange: LocalAiActivityMemoryTimeRangeSchema,
    asOf: LocalAiTimestampSchema,
    limit: LocalAiContextNonNegativeCountSchema,
}));
export const LocalAiActivityMemoryGraphReadInputSchema = withParser(Schema.Struct({
    query: LocalAiActivityMemoryGraphQuerySchema,
    nodes: Schema.Array(LocalAiActivityMemoryGraphNodeSchema),
    edges: Schema.Array(LocalAiActivityMemoryGraphEdgeSchema),
    selectedEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    selectedPolicyVersions: Schema.Array(ParentPolicyVersionSchema),
    selectedParentActionReferences: Schema.Array(ParentActionReferenceSchema),
}));
export const LocalAiActivityMemoryGraphReadResultSchema = withParser(Schema.Struct({
    query: LocalAiActivityMemoryGraphQuerySchema,
    readAt: LocalAiTimestampSchema,
    nodes: Schema.Array(LocalAiActivityMemoryGraphNodeSchema),
    edges: Schema.Array(LocalAiActivityMemoryGraphEdgeSchema),
    returnedNodeCount: LocalAiContextNonNegativeCountSchema,
    returnedEdgeCount: LocalAiContextNonNegativeCountSchema,
    omittedEdgeCount: LocalAiContextNonNegativeCountSchema,
    degradedReasons: Schema.Array(LocalAiContextReasonCodeSchema),
}).pipe(Schema.filter((result) => result.returnedNodeCount === result.nodes.length || 'Expected returned node count match'), Schema.filter((result) => result.returnedEdgeCount === result.edges.length || 'Expected returned edge count match'), Schema.filter((result) => result.edges.every((edge) => edgeReferencesReturnedNodes(edge, result.nodes)))));
function edgeReferencesReturnedNodes(edge, nodes) {
    const nodeIds = new Set(nodes.map((node) => node.nodeId));
    return nodeIds.has(edge.fromNodeId) && nodeIds.has(edge.toNodeId);
}
//# sourceMappingURL=local-ai-activity-memory-graph.js.map
