import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserAiConfidenceSchema,
  BrowserAiUncertaintyReasonSchema,
  BrowserParentRuleRefSchema,
  BrowserPolicyVersionRefSchema,
} from './browser-ai-analysis-values';
import {
  BrowserAiKnowledgeGraphEdgeKindSchema,
  BrowserAiKnowledgeGraphEdgeRefSchema,
  BrowserAiKnowledgeGraphIdSchema,
  BrowserAiKnowledgeGraphNodeKindSchema,
  type BrowserAiKnowledgeGraphNodeRef,
  BrowserAiKnowledgeGraphNodeRefSchema,
  BrowserAiKnowledgeGraphSnapshotIdSchema,
  BrowserAiKnowledgeGraphSourceKindSchema,
  BrowserAiKnowledgeGraphUseSchema,
  BrowserAiKnowledgeGraphVersionRefSchema,
  BrowserKnowledgeGraphRefSchema,
} from './browser-ai-knowledge-graph-values';

export {
  BrowserAiKnowledgeGraphEdgeKindSchema,
  BrowserAiKnowledgeGraphNodeKindSchema,
  BrowserAiKnowledgeGraphSourceKindSchema,
  BrowserAiKnowledgeGraphUseSchema,
  BrowserKnowledgeGraphRefSchema,
};

const EvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one knowledge graph evidence id')
);
const KnowledgeGraphUseListSchema = Schema.Array(BrowserAiKnowledgeGraphUseSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one knowledge graph use')
);
const ParentRuleRefsSchema = Schema.Array(BrowserParentRuleRefSchema);
const OptionalPolicyVersionRefSchema = Schema.Union(BrowserPolicyVersionRefSchema, Schema.Null);

export const BrowserAiKnowledgeGraphSchemaVersion = 1;

const BrowserAiKnowledgeGraphNodeBaseSchema = Schema.Struct({
  graphRef: BrowserKnowledgeGraphRefSchema,
  nodeRef: BrowserAiKnowledgeGraphNodeRefSchema,
  nodeKind: BrowserAiKnowledgeGraphNodeKindSchema,
  sourceKind: BrowserAiKnowledgeGraphSourceKindSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  parentRuleRefs: ParentRuleRefsSchema,
  confidence: BrowserAiConfidenceSchema,
  uncertaintyReasons: Schema.Array(BrowserAiUncertaintyReasonSchema),
  stale: Schema.Boolean,
  parentApproved: Schema.Boolean,
  canDrivePolicyInput: Schema.Boolean,
  rawContentStored: Schema.Boolean,
  platformLabelUsedAsAuthority: Schema.Boolean,
  directPolicyAuthorityClaimed: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});
export const BrowserAiKnowledgeGraphNodeSchema = withParser(
  BrowserAiKnowledgeGraphNodeBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiKnowledgeGraphNodeIsConsistent(value) ||
        'Expected knowledge graph node to be evidence-backed, fresh, and non-authoritative'
    )
  )
);

const BrowserAiKnowledgeGraphEdgeBaseSchema = Schema.Struct({
  edgeRef: BrowserAiKnowledgeGraphEdgeRefSchema,
  edgeKind: BrowserAiKnowledgeGraphEdgeKindSchema,
  fromNodeRef: BrowserAiKnowledgeGraphNodeRefSchema,
  toNodeRef: BrowserAiKnowledgeGraphNodeRefSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  confidence: BrowserAiConfidenceSchema,
  uncertaintyReasons: Schema.Array(BrowserAiUncertaintyReasonSchema),
  stale: Schema.Boolean,
  canDrivePolicyInput: Schema.Boolean,
  directPolicyAuthorityClaimed: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});
export const BrowserAiKnowledgeGraphEdgeSchema = withParser(
  BrowserAiKnowledgeGraphEdgeBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiKnowledgeGraphEdgeIsConsistent(value) ||
        'Expected knowledge graph edge to be evidence-backed, fresh, and non-authoritative'
    )
  )
);

const BrowserAiKnowledgeGraphReferenceBundleBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiKnowledgeGraphSchemaVersion),
  graphId: BrowserAiKnowledgeGraphIdSchema,
  graphRef: BrowserKnowledgeGraphRefSchema,
  snapshotId: BrowserAiKnowledgeGraphSnapshotIdSchema,
  graphVersionRef: BrowserAiKnowledgeGraphVersionRefSchema,
  capturedAt: ActivityTimestampSchema,
  policyVersionRef: OptionalPolicyVersionRefSchema,
  uses: KnowledgeGraphUseListSchema,
  nodes: Schema.Array(BrowserAiKnowledgeGraphNodeSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected at least one knowledge graph node')
  ),
  edges: Schema.Array(BrowserAiKnowledgeGraphEdgeSchema),
  retentionBounded: Schema.Boolean,
  rawContentStored: Schema.Boolean,
  canDrivePolicyInput: Schema.Boolean,
  directPolicyAuthorityClaimed: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});
export const BrowserAiKnowledgeGraphReferenceBundleSchema = withParser(
  BrowserAiKnowledgeGraphReferenceBundleBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiKnowledgeGraphReferenceBundleIsConsistent(value) ||
        'Expected knowledge graph bundle to be bounded, evidence-backed, and non-enforcing'
    )
  )
);

export const decodeBrowserAiKnowledgeGraphNode = Schema.decodeUnknownSync(BrowserAiKnowledgeGraphNodeSchema);
export const decodeBrowserAiKnowledgeGraphEdge = Schema.decodeUnknownSync(BrowserAiKnowledgeGraphEdgeSchema);
export const decodeBrowserAiKnowledgeGraphReferenceBundle = Schema.decodeUnknownSync(
  BrowserAiKnowledgeGraphReferenceBundleSchema
);

export type BrowserAiKnowledgeGraphNode = Infer<typeof BrowserAiKnowledgeGraphNodeSchema>;
export type BrowserAiKnowledgeGraphEdge = Infer<typeof BrowserAiKnowledgeGraphEdgeSchema>;
export type BrowserAiKnowledgeGraphReferenceBundle = Infer<typeof BrowserAiKnowledgeGraphReferenceBundleSchema>;

function browserAiKnowledgeGraphNodeIsConsistent(value: Infer<typeof BrowserAiKnowledgeGraphNodeBaseSchema>) {
  if (knowledgeGraphAuthorityCreepClaimed(value)) {
    return false;
  }
  if (lowConfidenceStateIsHidden(value.confidence, value.uncertaintyReasons)) {
    return false;
  }
  if (value.sourceKind === 'platform-label' && value.canDrivePolicyInput) {
    return false;
  }
  if (value.sourceKind === 'external-taxonomy' && value.canDrivePolicyInput && !value.parentApproved) {
    return false;
  }
  return !value.stale || !value.canDrivePolicyInput;
}

function browserAiKnowledgeGraphEdgeIsConsistent(value: Infer<typeof BrowserAiKnowledgeGraphEdgeBaseSchema>) {
  if (value.directPolicyAuthorityClaimed || value.directEnforcementClaimed) {
    return false;
  }
  if (lowConfidenceStateIsHidden(value.confidence, value.uncertaintyReasons)) {
    return false;
  }
  return !value.stale || !value.canDrivePolicyInput;
}

function browserAiKnowledgeGraphReferenceBundleIsConsistent(
  value: Infer<typeof BrowserAiKnowledgeGraphReferenceBundleBaseSchema>
) {
  if (!value.retentionBounded || value.rawContentStored || bundleAuthorityCreepClaimed(value)) {
    return false;
  }
  if (!nodeRefsAreUnique(value.nodes) || !edgesReferenceKnownNodes(value.nodes, value.edges)) {
    return false;
  }
  if (!value.canDrivePolicyInput) {
    return true;
  }
  return policyCandidateBundleCanDrivePolicyInput(value);
}

function policyCandidateBundleCanDrivePolicyInput(
  value: Infer<typeof BrowserAiKnowledgeGraphReferenceBundleBaseSchema>
) {
  return (
    value.policyVersionRef !== null &&
    value.uses.includes('policy-candidate-support') &&
    value.nodes.every((node) => node.canDrivePolicyInput) &&
    value.edges.every((edge) => edge.canDrivePolicyInput)
  );
}

function knowledgeGraphAuthorityCreepClaimed(value: Infer<typeof BrowserAiKnowledgeGraphNodeBaseSchema>) {
  return (
    value.rawContentStored ||
    value.platformLabelUsedAsAuthority ||
    value.directPolicyAuthorityClaimed ||
    value.directEnforcementClaimed
  );
}

function bundleAuthorityCreepClaimed(value: Infer<typeof BrowserAiKnowledgeGraphReferenceBundleBaseSchema>) {
  return value.directPolicyAuthorityClaimed || value.directEnforcementClaimed;
}

function lowConfidenceStateIsHidden(
  confidence: Infer<typeof BrowserAiConfidenceSchema>,
  uncertaintyReasons: ReadonlyArray<Infer<typeof BrowserAiUncertaintyReasonSchema>>
) {
  return (confidence === 'low' || confidence === 'unknown') && uncertaintyReasons.length === 0;
}

function nodeRefsAreUnique(nodes: ReadonlyArray<Infer<typeof BrowserAiKnowledgeGraphNodeBaseSchema>>) {
  const refs = new Set<BrowserAiKnowledgeGraphNodeRef>();
  for (const node of nodes) {
    if (refs.has(node.nodeRef)) {
      return false;
    }
    refs.add(node.nodeRef);
  }
  return true;
}

function edgesReferenceKnownNodes(
  nodes: ReadonlyArray<Infer<typeof BrowserAiKnowledgeGraphNodeBaseSchema>>,
  edges: ReadonlyArray<Infer<typeof BrowserAiKnowledgeGraphEdgeBaseSchema>>
) {
  const refs = new Set(nodes.map((node) => node.nodeRef));
  return edges.every((edge) => refs.has(edge.fromNodeRef) && refs.has(edge.toNodeRef));
}
