import { type Infer, Schema } from '@ocentra-parent/schema-domain/effect';
import { LocalAiConfidenceKindSchema, LocalAiContextBuildStateSchema, LocalAiContextCapabilityStatusSchema, LocalAiContextNonNegativeCountSchema, LocalAiContextReasonCodeSchema, LocalAiEvidenceAdapterIdSchema, LocalAiEvidenceContextIdSchema, LocalAiEvidenceContextKindSchema, LocalAiEvidenceContextRefIdSchema, LocalAiEvidenceContextSummarySchema, LocalAiEvidenceCustodySchema, LocalAiEvidenceRetentionStateSchema, LocalAiEvidenceSourceIdSchema, LocalAiParentRuleContextRefIdSchema, LocalAiRejectedFieldSchema, LocalAiRequestedEvaluationKindSchema } from './context-primitives';
import type { LocalAiConfidenceKind, LocalAiContextBuildState, LocalAiContextCapabilityStatus, LocalAiContextReasonCode, LocalAiEvidenceAdapterId, LocalAiEvidenceContextId, LocalAiEvidenceContextKind, LocalAiEvidenceContextRefId, LocalAiEvidenceContextSummary, LocalAiEvidenceCustody, LocalAiEvidenceRetentionState, LocalAiEvidenceSourceId, LocalAiParentRuleContextRefId, LocalAiRejectedField, LocalAiRequestedEvaluationKind } from './context-primitives';
export { LocalAiConfidenceKindSchema, LocalAiContextBuildStateSchema, LocalAiContextCapabilityStatusSchema, LocalAiContextNonNegativeCountSchema, LocalAiContextReasonCodeSchema, LocalAiEvidenceAdapterIdSchema, LocalAiEvidenceContextIdSchema, LocalAiEvidenceContextKindSchema, LocalAiEvidenceContextRefIdSchema, LocalAiEvidenceContextSummarySchema, LocalAiEvidenceCustodySchema, LocalAiEvidenceRetentionStateSchema, LocalAiEvidenceSourceIdSchema, LocalAiParentRuleContextRefIdSchema, LocalAiRejectedFieldSchema, LocalAiRequestedEvaluationKindSchema };
export type { LocalAiConfidenceKind, LocalAiContextBuildState, LocalAiContextCapabilityStatus, LocalAiContextReasonCode, LocalAiEvidenceAdapterId, LocalAiEvidenceContextId, LocalAiEvidenceContextKind, LocalAiEvidenceContextRefId, LocalAiEvidenceContextSummary, LocalAiEvidenceCustody, LocalAiEvidenceRetentionState, LocalAiEvidenceSourceId, LocalAiParentRuleContextRefId, LocalAiRejectedField, LocalAiRequestedEvaluationKind };
export declare const LocalAiEvidenceContextSourceRefSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    evidenceRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">;
    evidence: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>;
    evidenceKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["browser", "app-game", "network-flow", "screen-summary", "policy-decision", "parent-action", "recent-activity"]>>;
    sourceSchemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
    observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    ingestedAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
    freshUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
    sourceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceSourceId">;
    adapterId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceAdapterId">;
    device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
        childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
        platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
    }>>;
    childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
        displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
    }>>;
    custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
    retentionState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["local", "temporary", "deleted-source", "export-copy", "parent-owned-copy", "unavailable"]>>;
    confidence: Schema.Union<[Schema.filter<typeof Schema.Number>, typeof Schema.Null]>;
    confidenceKind: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["observation", "correlation", "classifier", "model", "memory-match", "graph-edge", "rule-match"]>>, typeof Schema.Null]>;
    capabilityStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["available", "unsupported", "permission-limited", "stale", "degraded", "adapter-error", "disabled-by-parent", "unavailable"]>>;
    degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
    unknownReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
    sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
}>>>;
export declare const LocalAiParentRuleContextRefSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    parentRuleRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiParentRuleContextRefId">;
    policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
    family: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        familyId: Schema.brand<Schema.filter<typeof Schema.String>, "FamilyId">;
    }>>;
    childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
        displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
    }>>;
    device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
        childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
        platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
    }>>;
    rule: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        ruleId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyRuleId">;
        target: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            targetId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetId">;
            targetType: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["app", "process", "window", "domain", "site", "category", "video", "channel", "activity-type", "device"]>>;
            targetValue: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetValue">;
        }>>;
        action: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["allow", "warn", "block", "time-limit", "ask-parent", "unknown"]>>;
        scheduleId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyScheduleId">, typeof Schema.Null]>;
        priority: typeof Schema.Number;
        reasonCode: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyReasonCode">;
        createdBy: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
            role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
        }>>;
        enabled: typeof Schema.Boolean;
        effectiveFrom: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
        effectiveUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
    }>>;
    targetEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
    custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
    updatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
}>>>;
export declare const LocalAiEvidenceContextBuildRequestSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    schemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
    requestId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvaluationRequestId">;
    requestedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
    childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
        displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
    }>>;
    device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
        childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
        platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
    }>>;
    requestedEvaluationKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["page", "url", "video", "app", "game", "domain", "network-digest", "screen-summary", "recent-activity", "mixed-context"]>>;
    requiredEvidenceKinds: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["browser", "app-game", "network-flow", "screen-summary", "policy-decision", "parent-action", "recent-activity"]>>>;
    parentRuleContextReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
        parentRuleRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiParentRuleContextRefId">;
        policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
        family: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            familyId: Schema.brand<Schema.filter<typeof Schema.String>, "FamilyId">;
        }>>;
        childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        rule: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            ruleId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyRuleId">;
            target: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                targetId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetId">;
                targetType: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["app", "process", "window", "domain", "site", "category", "video", "channel", "activity-type", "device"]>>;
                targetValue: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetValue">;
            }>>;
            action: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["allow", "warn", "block", "time-limit", "ask-parent", "unknown"]>>;
            scheduleId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyScheduleId">, typeof Schema.Null]>;
            priority: typeof Schema.Number;
            reasonCode: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyReasonCode">;
            createdBy: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
            }>>;
            enabled: typeof Schema.Boolean;
            effectiveFrom: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
            effectiveUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
        }>>;
        targetEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
        custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
        updatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
    }>>>>;
    modelTaskRequirements: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["classification", "summarization", "embedding", "safety-decision", "chat-completion"]>>>;
    allowedCustody: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>>;
    promptVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiPromptVersion">;
}>>;
export declare const LocalAiEvidenceContextValidationSummarySchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    evidenceReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    sourceEvidenceReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    runtimeReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    memoryReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    graphReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    parentRuleReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    ungroundedParentRuleReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    forbiddenCustodyReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    unallowedCustodyReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
}>>;
export declare const LocalAiEvidenceContextSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    schemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
    contextId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextId">;
    requestId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvaluationRequestId">;
    childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
        displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
    }>>;
    device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
        childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
        label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
        platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
    }>>;
    evidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
        evidenceRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">;
        evidence: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
            kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
            observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>;
        evidenceKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["browser", "app-game", "network-flow", "screen-summary", "policy-decision", "parent-action", "recent-activity"]>>;
        sourceSchemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        ingestedAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        freshUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        sourceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceSourceId">;
        adapterId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceAdapterId">;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>;
        custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
        retentionState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["local", "temporary", "deleted-source", "export-copy", "parent-owned-copy", "unavailable"]>>;
        confidence: Schema.Union<[Schema.filter<typeof Schema.Number>, typeof Schema.Null]>;
        confidenceKind: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["observation", "correlation", "classifier", "model", "memory-match", "graph-edge", "rule-match"]>>, typeof Schema.Null]>;
        capabilityStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["available", "unsupported", "permission-limited", "stale", "degraded", "adapter-error", "disabled-by-parent", "unavailable"]>>;
        degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        unknownReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
            kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
            observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>>;
    }>>>>;
    browserEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
    appGameEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
    networkFlowEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
    screenSummaryRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
    parentRuleReferences: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "PolicyRuleId">>;
    parentRuleContextReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
        parentRuleRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiParentRuleContextRefId">;
        policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
        family: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            familyId: Schema.brand<Schema.filter<typeof Schema.String>, "FamilyId">;
        }>>;
        childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        rule: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            ruleId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyRuleId">;
            target: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                targetId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetId">;
                targetType: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["app", "process", "window", "domain", "site", "category", "video", "channel", "activity-type", "device"]>>;
                targetValue: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetValue">;
            }>>;
            action: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["allow", "warn", "block", "time-limit", "ask-parent", "unknown"]>>;
            scheduleId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyScheduleId">, typeof Schema.Null]>;
            priority: typeof Schema.Number;
            reasonCode: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyReasonCode">;
            createdBy: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
            }>>;
            enabled: typeof Schema.Boolean;
            effectiveFrom: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
            effectiveUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
        }>>;
        targetEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
        custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
        updatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
    }>>>>;
    recentActivitySummaryRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
    memoryReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
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
    }>>>>;
    graphReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
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
    }>>>>;
    localModelRuntimeRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiRuntimeReferenceId">>;
    promptVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiPromptVersion">;
    custodyLabels: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>>;
    degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
    unknownReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
    validationSummary: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        sourceEvidenceReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        runtimeReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        memoryReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        graphReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        parentRuleReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        ungroundedParentRuleReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        forbiddenCustodyReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        unallowedCustodyReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
    }>>;
}>>;
export declare const LocalAiEvidenceContextBuildResultSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    schemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
    requestId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvaluationRequestId">;
    state: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["ready", "partial", "insufficient", "unavailable", "rejected"]>>;
    context: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        schemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
        contextId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextId">;
        requestId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvaluationRequestId">;
        childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        evidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            evidenceRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">;
            evidence: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
                kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
                observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>;
            evidenceKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["browser", "app-game", "network-flow", "screen-summary", "policy-decision", "parent-action", "recent-activity"]>>;
            sourceSchemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
            observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            ingestedAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
            freshUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
            sourceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceSourceId">;
            adapterId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceAdapterId">;
            device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
                childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
                label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
                platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
            }>>;
            childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
                displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
            }>>;
            custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
            retentionState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["local", "temporary", "deleted-source", "export-copy", "parent-owned-copy", "unavailable"]>>;
            confidence: Schema.Union<[Schema.filter<typeof Schema.Number>, typeof Schema.Null]>;
            confidenceKind: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["observation", "correlation", "classifier", "model", "memory-match", "graph-edge", "rule-match"]>>, typeof Schema.Null]>;
            capabilityStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["available", "unsupported", "permission-limited", "stale", "degraded", "adapter-error", "disabled-by-parent", "unavailable"]>>;
            degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
            unknownReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
            sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
                kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
                observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
            }>>>;
        }>>>>;
        browserEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
        appGameEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
        networkFlowEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
        screenSummaryRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
        parentRuleReferences: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "PolicyRuleId">>;
        parentRuleContextReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            parentRuleRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiParentRuleContextRefId">;
            policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
            family: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                familyId: Schema.brand<Schema.filter<typeof Schema.String>, "FamilyId">;
            }>>;
            childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
                displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
            }>>;
            device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
                childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
                label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
                platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
            }>>;
            rule: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                ruleId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyRuleId">;
                target: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    targetId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetId">;
                    targetType: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["app", "process", "window", "domain", "site", "category", "video", "channel", "activity-type", "device"]>>;
                    targetValue: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetValue">;
                }>>;
                action: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["allow", "warn", "block", "time-limit", "ask-parent", "unknown"]>>;
                scheduleId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyScheduleId">, typeof Schema.Null]>;
                priority: typeof Schema.Number;
                reasonCode: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyReasonCode">;
                createdBy: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                    role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
                }>>;
                enabled: typeof Schema.Boolean;
                effectiveFrom: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
                effectiveUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
            }>>;
            targetEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
            custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
            updatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        }>>>>;
        recentActivitySummaryRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
        memoryReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
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
        }>>>>;
        graphReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
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
        }>>>>;
        localModelRuntimeRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiRuntimeReferenceId">>;
        promptVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiPromptVersion">;
        custodyLabels: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>>;
        degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        unknownReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        validationSummary: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            evidenceReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            sourceEvidenceReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            runtimeReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            memoryReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            graphReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            parentRuleReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            ungroundedParentRuleReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            forbiddenCustodyReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
            unallowedCustodyReferenceCount: Schema.filter<Schema.filter<typeof Schema.Number>>;
        }>>;
    }>>, typeof Schema.Null]>;
    rejectedFields: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiRejectedField">>;
    missingEvidenceKinds: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["browser", "app-game", "network-flow", "screen-summary", "policy-decision", "parent-action", "recent-activity"]>>>;
    degradedSourceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
    custodyBoundarySummary: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextSummary">;
    validationGateSummary: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextSummary">;
    auditEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
        kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
    }>>>;
}>>;
export declare const LocalAiStoredEvidenceContextBuildInputSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    contextId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextId">;
    request: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        schemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
        requestId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvaluationRequestId">;
        requestedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        requestedEvaluationKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["page", "url", "video", "app", "game", "domain", "network-digest", "screen-summary", "recent-activity", "mixed-context"]>>;
        requiredEvidenceKinds: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["browser", "app-game", "network-flow", "screen-summary", "policy-decision", "parent-action", "recent-activity"]>>>;
        parentRuleContextReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
            parentRuleRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiParentRuleContextRefId">;
            policyVersion: Schema.brand<Schema.filter<typeof Schema.String>, "ParentPolicyVersion">;
            family: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                familyId: Schema.brand<Schema.filter<typeof Schema.String>, "FamilyId">;
            }>>;
            childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
                displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
            }>>;
            device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
                childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
                label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
                platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
            }>>;
            rule: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                ruleId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyRuleId">;
                target: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    targetId: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetId">;
                    targetType: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["app", "process", "window", "domain", "site", "category", "video", "channel", "activity-type", "device"]>>;
                    targetValue: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTargetValue">;
                }>>;
                action: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["allow", "warn", "block", "time-limit", "ask-parent", "unknown"]>>;
                scheduleId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyScheduleId">, typeof Schema.Null]>;
                priority: typeof Schema.Number;
                reasonCode: Schema.brand<Schema.filter<typeof Schema.String>, "PolicyReasonCode">;
                createdBy: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
                    actorId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentActorId">;
                    role: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["parent", "guardian", "system"]>>;
                }>>;
                enabled: typeof Schema.Boolean;
                effectiveFrom: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
                effectiveUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "PolicyTimestamp">, typeof Schema.Null]>;
            }>>;
            targetEvidenceRefs: Schema.Array$<Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">>;
            custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
            updatedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
            expiresAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        }>>>>;
        modelTaskRequirements: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["classification", "summarization", "embedding", "safety-decision", "chat-completion"]>>>;
        allowedCustody: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>>;
        promptVersion: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiPromptVersion">;
    }>>;
    evidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
        evidenceRefId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceContextRefId">;
        evidence: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
            kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
            observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>;
        evidenceKind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["browser", "app-game", "network-flow", "screen-summary", "policy-decision", "parent-action", "recent-activity"]>>;
        sourceSchemaVersion: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["v0.6"]>>;
        observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        ingestedAt: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        freshUntil: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">, typeof Schema.Null]>;
        sourceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceSourceId">;
        adapterId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiEvidenceAdapterId">;
        device: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            deviceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceId">;
            childProfileId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">, typeof Schema.Null]>;
            label: Schema.brand<Schema.filter<typeof Schema.String>, "ParentDeviceLabel">;
            platform: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["windows", "linux", "macos", "android", "ios"]>>;
        }>>;
        childProfile: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            childProfileId: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileId">;
            displayName: Schema.brand<Schema.filter<typeof Schema.String>, "ChildProfileDisplayName">;
        }>>;
        custody: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["live-local-child-agent", "live-lan-child-agent", "child-device-journal", "child-device-query-store", "parent-device-cache", "parent-owned-export", "ocentra-hosted-non-activity", "unavailable"]>>;
        retentionState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["local", "temporary", "deleted-source", "export-copy", "parent-owned-copy", "unavailable"]>>;
        confidence: Schema.Union<[Schema.filter<typeof Schema.Number>, typeof Schema.Null]>;
        confidenceKind: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["observation", "correlation", "classifier", "model", "memory-match", "graph-edge", "rule-match"]>>, typeof Schema.Null]>;
        capabilityStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["available", "unsupported", "permission-limited", "stale", "degraded", "adapter-error", "disabled-by-parent", "unavailable"]>>;
        degradedReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        unknownReasons: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["missing-evidence", "stale-evidence", "source-conflict", "unsupported-source", "permission-limited", "adapter-error", "capability-disabled-by-parent", "custody-unavailable", "forbidden-remote-source", "invalid-confidence", "invalid-ai-output", "model-unavailable", "model-overloaded", "model-output-unparseable", "memory-ungrounded", "graph-ungrounded", "parent-rule-missing", "parent-rule-conflict", "schedule-unresolved", "protected-surface", "screen-image-deleted", "screen-deletion-unconfirmed", "network-encrypted-content-unavailable", "browser-active-tab-unknown", "app-duration-incomplete"]>>>;
        sourceEvidenceReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
            evidenceReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "ParentEvidenceReferenceId">;
            kind: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["journal-event", "query-store-summary", "activity-event", "policy-decision", "local-ai-result"]>>;
            observedAt: Schema.brand<Schema.filter<typeof Schema.String>, "ParentTimestamp">;
        }>>>;
    }>>>>;
    runtimeReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        runtimeReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiRuntimeReferenceId">;
        providerId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiProviderId">;
        modelId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiModelId">;
        modelReference: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiModelReference">;
        privacyMode: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["local-only"]>>;
        adapterBoundary: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["status-only", "local-adapter-unavailable", "local-adapter-ready"]>>;
        executionState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["disabled", "dry-run-ready", "running", "failed"]>>;
        providerSource: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["unavailable", "local-config", "local-model-cache", "os-capability-probe"]>>;
        loadState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["unavailable", "loading", "loaded", "degraded", "failed"]>>;
        capabilityFlags: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["classification", "summarization", "embedding", "safety-decision", "chat-completion"]>>>;
        resourceClass: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["cpu", "gpu", "npu", "remote-unavailable"]>>;
        degradedState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["none", "provider-unavailable", "model-load-failed", "overloaded", "invalid-output"]>>;
        lastCheckedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
        unavailableReason: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiUnavailableReason">, typeof Schema.Null]>;
    }>>>;
    memoryReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
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
    }>>>>;
    graphReferences: Schema.Array$<import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
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
    }>>>>;
}>>;
export type LocalAiEvidenceContextSourceRef = Infer<typeof LocalAiEvidenceContextSourceRefSchema>;
export type LocalAiParentRuleContextRef = Infer<typeof LocalAiParentRuleContextRefSchema>;
export type LocalAiEvidenceContextBuildRequest = Infer<typeof LocalAiEvidenceContextBuildRequestSchema>;
export type LocalAiEvidenceContextValidationSummary = Infer<typeof LocalAiEvidenceContextValidationSummarySchema>;
export type LocalAiEvidenceContext = Infer<typeof LocalAiEvidenceContextSchema>;
export type LocalAiEvidenceContextBuildResult = Infer<typeof LocalAiEvidenceContextBuildResultSchema>;
export type LocalAiStoredEvidenceContextBuildInput = Infer<typeof LocalAiStoredEvidenceContextBuildInputSchema>;
//# sourceMappingURL=local-ai-context.d.ts.map
