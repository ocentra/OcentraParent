import {
  type LocalAiActivityMemoryGraphEdge,
  type LocalAiActivityMemoryGraphNode,
  type LocalAiActivityMemoryGraphReadInput,
  type LocalAiActivityMemoryGraphReadResult,
  type LocalAiActivityMemoryGraphTrace,
  LocalAiActivityMemoryGraphReadInputSchema,
  LocalAiActivityMemoryGraphReadResultSchema,
} from './local-ai-activity-memory-graph';

function traceIsSelected(trace: LocalAiActivityMemoryGraphTrace, input: LocalAiActivityMemoryGraphReadInput): boolean {
  const selectedEvidenceIds = new Set(
    input.selectedEvidenceReferences.map((reference) => reference.evidenceReferenceId)
  );
  const selectedPolicyVersions = new Set(input.selectedPolicyVersions);
  const selectedActionIds = new Set(
    input.selectedParentActionReferences.map((reference) => reference.actionReferenceId)
  );
  return (
    trace.sourceEvidenceReferences.every((reference) => selectedEvidenceIds.has(reference.evidenceReferenceId)) &&
    (trace.sourcePolicyVersion === null || selectedPolicyVersions.has(trace.sourcePolicyVersion)) &&
    trace.sourceParentActionReferences.every((reference) => selectedActionIds.has(reference.actionReferenceId))
  );
}

function traceIsFresh(trace: LocalAiActivityMemoryGraphTrace, asOf: string): boolean {
  return trace.entryStatus === 'usable' && (trace.expiresAt === null || Date.parse(trace.expiresAt) > Date.parse(asOf));
}

function edgeMatchesQueryKind(
  edge: LocalAiActivityMemoryGraphEdge,
  queryKind: LocalAiActivityMemoryGraphReadInput['query']['queryKind']
): boolean {
  if (queryKind === 'visited-urls') {
    return edge.edgeKind === 'visited';
  }
  if (queryKind === 'played-games') {
    return edge.edgeKind === 'played';
  }
  if (queryKind === 'watched-videos') {
    return edge.edgeKind === 'watched';
  }
  return true;
}

function edgeOverlapsTimeRange(
  edge: LocalAiActivityMemoryGraphEdge,
  query: LocalAiActivityMemoryGraphReadInput['query']
): boolean {
  const edgeEnd = edge.observedUntil ?? edge.observedFrom;
  return (
    Date.parse(edge.observedFrom) <= Date.parse(query.timeRange.observedUntil) &&
    Date.parse(edgeEnd) >= Date.parse(query.timeRange.observedFrom)
  );
}

function selectUsableNodes(input: LocalAiActivityMemoryGraphReadInput): LocalAiActivityMemoryGraphNode[] {
  return input.nodes.filter((node) => traceIsFresh(node.trace, input.query.asOf) && traceIsSelected(node.trace, input));
}

function selectMatchingEdges(
  input: LocalAiActivityMemoryGraphReadInput,
  usableNodes: readonly LocalAiActivityMemoryGraphNode[]
): LocalAiActivityMemoryGraphEdge[] {
  const usableNodeIds = new Set(usableNodes.map((node) => node.nodeId));
  return input.edges.filter(
    (edge) =>
      traceIsFresh(edge.trace, input.query.asOf) &&
      traceIsSelected(edge.trace, input) &&
      edgeMatchesQueryKind(edge, input.query.queryKind) &&
      edgeOverlapsTimeRange(edge, input.query) &&
      usableNodeIds.has(edge.fromNodeId) &&
      usableNodeIds.has(edge.toNodeId)
  );
}

export function readLocalAiActivityMemoryGraph(input: unknown): LocalAiActivityMemoryGraphReadResult {
  const parsed = LocalAiActivityMemoryGraphReadInputSchema.parse(input);
  const usableNodes = selectUsableNodes(parsed);
  const matchingEdges = selectMatchingEdges(parsed, usableNodes);
  const edges = matchingEdges.slice(0, parsed.query.limit);
  const resultNodeIds = new Set(edges.flatMap((edge) => [edge.fromNodeId, edge.toNodeId]));
  const nodes = usableNodes.filter((node) => resultNodeIds.has(node.nodeId));
  return LocalAiActivityMemoryGraphReadResultSchema.parse({
    query: parsed.query,
    readAt: parsed.query.asOf,
    nodes,
    edges,
    returnedNodeCount: nodes.length,
    returnedEdgeCount: edges.length,
    omittedEdgeCount: parsed.edges.length - edges.length,
    degradedReasons: matchingEdges.length === parsed.edges.length ? [] : ['memory-ungrounded'],
  });
}
