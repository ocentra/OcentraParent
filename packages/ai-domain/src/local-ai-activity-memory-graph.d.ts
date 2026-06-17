import { type Infer, Schema } from '@ocentra-parent/schema-domain/effect';
export declare const LocalAiActivityMemoryGraphTraceSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    entryStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["candidate", "usable", "degraded", "stale", "rejected"]>>;
    sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
    sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
    sourceParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
        actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
            role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
        }>>;
        policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
        createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
    generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
    confidence: Schema.filter<typeof Schema.Number>;
    derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
    degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
}>>>;
export declare const LocalAiActivityMemoryTimeRangeSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    observedFrom: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    observedUntil: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
}>>>;
export declare const LocalAiActivityMemoryGraphNodeSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    graphId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphId">;
    nodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
    nodeKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["child-profile", "device", "browser-url", "domain", "video", "app", "game", "activity-session"]>>;
    label: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryLabel">;
    childProfile: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
        displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
    }>>, typeof Schema.Null]>;
    device: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
        childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
        platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
    }>>, typeof Schema.Null]>;
    trace: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
        entryStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["candidate", "usable", "degraded", "stale", "rejected"]>>;
        sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
            kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
            observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>>;
        sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
        sourceParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
            actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
            }>>;
            policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
            createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>>;
        generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        confidence: Schema.filter<typeof Schema.Number>;
        derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
        degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
    }>>>;
}>>;
export declare const LocalAiActivityMemoryGraphEdgeSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.filter<Schema.Struct<{
    graphId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphId">;
    edgeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphEdgeId">;
    edgeKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["visited", "watched", "played", "active-during", "performed-by-child", "derived-from-evidence"]>>;
    fromNodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
    toNodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
    observedFrom: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    observedUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
    durationMs: Schema.Union<[Schema.filter<Schema.filter<typeof Schema.Number>>, typeof Schema.Null]>;
    trace: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
        entryStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["candidate", "usable", "degraded", "stale", "rejected"]>>;
        sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
            kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
            observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>>;
        sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
        sourceParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
            actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
            }>>;
            policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
            createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>>;
        generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        confidence: Schema.filter<typeof Schema.Number>;
        derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
        degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
    }>>>;
}>>>>;
export declare const LocalAiActivityMemoryGraphQuerySchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    queryId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphQueryId">;
    queryKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["visited-urls", "played-games", "watched-videos", "activity-by-time-range", "explain-evidence"]>>;
    childProfile: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
        displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
    }>>, typeof Schema.Null]>;
    device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
        childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
        platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
    }>>;
    timeRange: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
        observedFrom: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        observedUntil: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    }>>>;
    asOf: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    limit: Schema.filter<Schema.filter<typeof Schema.Number>>;
}>>;
export declare const LocalAiActivityMemoryGraphReadInputSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    query: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        queryId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphQueryId">;
        queryKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["visited-urls", "played-games", "watched-videos", "activity-by-time-range", "explain-evidence"]>>;
        childProfile: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>, typeof Schema.Null]>;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        timeRange: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            observedFrom: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            observedUntil: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        }>>>;
        asOf: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        limit: Schema.filter<Schema.filter<typeof Schema.Number>>;
    }>>;
    nodes: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        graphId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphId">;
        nodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
        nodeKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["child-profile", "device", "browser-url", "domain", "video", "app", "game", "activity-session"]>>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryLabel">;
        childProfile: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>, typeof Schema.Null]>;
        device: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>, typeof Schema.Null]>;
        trace: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            entryStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["candidate", "usable", "degraded", "stale", "rejected"]>>;
            sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
                kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
                observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
            sourceParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
                actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                    role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
                }>>;
                policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
                createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
            confidence: Schema.filter<typeof Schema.Number>;
            derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
            degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        }>>>;
    }>>>;
    edges: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.filter<Schema.Struct<{
        graphId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphId">;
        edgeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphEdgeId">;
        edgeKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["visited", "watched", "played", "active-during", "performed-by-child", "derived-from-evidence"]>>;
        fromNodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
        toNodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
        observedFrom: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        observedUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        durationMs: Schema.Union<[Schema.filter<Schema.filter<typeof Schema.Number>>, typeof Schema.Null]>;
        trace: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            entryStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["candidate", "usable", "degraded", "stale", "rejected"]>>;
            sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
                kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
                observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
            sourceParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
                actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                    role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
                }>>;
                policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
                createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
            confidence: Schema.filter<typeof Schema.Number>;
            derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
            degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        }>>>;
    }>>>>>;
    selectedEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
    selectedPolicyVersions: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">>;
    selectedParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
        actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
            role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
        }>>;
        policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
        createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
}>>;
export declare const LocalAiActivityMemoryGraphReadResultSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.filter<Schema.filter<Schema.Struct<{
    query: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        queryId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphQueryId">;
        queryKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["visited-urls", "played-games", "watched-videos", "activity-by-time-range", "explain-evidence"]>>;
        childProfile: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>, typeof Schema.Null]>;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        timeRange: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            observedFrom: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            observedUntil: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        }>>>;
        asOf: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        limit: Schema.filter<Schema.filter<typeof Schema.Number>>;
    }>>;
    readAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    nodes: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        graphId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphId">;
        nodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
        nodeKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["child-profile", "device", "browser-url", "domain", "video", "app", "game", "activity-session"]>>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryLabel">;
        childProfile: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>, typeof Schema.Null]>;
        device: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>, typeof Schema.Null]>;
        trace: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            entryStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["candidate", "usable", "degraded", "stale", "rejected"]>>;
            sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
                kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
                observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
            sourceParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
                actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                    role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
                }>>;
                policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
                createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
            confidence: Schema.filter<typeof Schema.Number>;
            derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
            degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        }>>>;
    }>>>;
    edges: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.filter<Schema.Struct<{
        graphId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphId">;
        edgeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphEdgeId">;
        edgeKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["visited", "watched", "played", "active-during", "performed-by-child", "derived-from-evidence"]>>;
        fromNodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
        toNodeId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiActivityMemoryGraphNodeId">;
        observedFrom: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        observedUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        durationMs: Schema.Union<[Schema.filter<Schema.filter<typeof Schema.Number>>, typeof Schema.Null]>;
        trace: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            entryStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["candidate", "usable", "degraded", "stale", "rejected"]>>;
            sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
                kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
                observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            sourcePolicyVersion: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">, typeof Schema.Null]>;
            sourceParentActionReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actionReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActionReferenceId">;
                actor: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                    role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
                }>>;
                policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
                createdAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
            generatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
            confidence: Schema.filter<typeof Schema.Number>;
            derivedIndexVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiDerivedIndexVersion">;
            degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        }>>>;
    }>>>>>;
    returnedNodeCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    returnedEdgeCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    omittedEdgeCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
}>>>>>;
export type LocalAiActivityMemoryGraphTrace = Infer<typeof LocalAiActivityMemoryGraphTraceSchema>;
export type LocalAiActivityMemoryGraphNode = Infer<typeof LocalAiActivityMemoryGraphNodeSchema>;
export type LocalAiActivityMemoryGraphEdge = Infer<typeof LocalAiActivityMemoryGraphEdgeSchema>;
export type LocalAiActivityMemoryGraphReadInput = Infer<typeof LocalAiActivityMemoryGraphReadInputSchema>;
export type LocalAiActivityMemoryGraphReadResult = Infer<typeof LocalAiActivityMemoryGraphReadResultSchema>;
//# sourceMappingURL=local-ai-activity-memory-graph.d.ts.map