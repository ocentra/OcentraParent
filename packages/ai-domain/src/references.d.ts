import { type Infer, Schema } from '@ocentra-parent/schema-domain/effect';
export declare const LocalAiObservationReferenceSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    contextKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["app", "process", "window", "url", "page", "video", "domain", "network", "recent-activity"]>>;
    evidence: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>;
}>>;
export declare const LocalAiMemoryReferenceSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    memoryReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiMemoryReferenceId">;
    kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["evidence-memory", "recent-activity", "policy-memory", "semantic-memory"]>>;
    sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
    sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
    generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    confidence: Schema.filter<typeof Schema.Number>;
    derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
}>>>;
export declare const LocalAiGraphReferenceSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    graphReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiGraphReferenceId">;
    kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["graph-entity", "graph-edge"]>>;
    sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
    sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
    generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    confidence: Schema.filter<typeof Schema.Number>;
    derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
}>>>;
export type LocalAiObservationReference = Infer<typeof LocalAiObservationReferenceSchema>;
export type LocalAiMemoryReference = Infer<typeof LocalAiMemoryReferenceSchema>;
export type LocalAiGraphReference = Infer<typeof LocalAiGraphReferenceSchema>;
//# sourceMappingURL=references.d.ts.map