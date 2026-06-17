import { type Infer, Schema } from '@ocentra-parent/schema-domain/effect';
export declare const LocalAiPhysicalDeviceIdSchema: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiPhysicalDeviceId">;
export declare const LocalAiProviderSingletonScopeSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["physical-device"]>>;
export declare const LocalAiProviderSchedulerLifecycleSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["idle", "running", "queued", "degraded", "unavailable"]>>;
export declare const LocalAiProviderSchedulerJobClassSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["child-safety", "parent-assistant", "parent-report"]>>;
export declare const LocalAiProviderSchedulerJobStatusSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["accepted", "running", "queued", "degraded", "unavailable", "complete"]>>;
export declare const LocalAiProviderSchedulerQueueSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
    childSafetyQueued: Schema.filter<Schema.filter<typeof Schema.Number>>;
    parentAssistantQueued: Schema.filter<Schema.filter<typeof Schema.Number>>;
    parentReportQueued: Schema.filter<Schema.filter<typeof Schema.Number>>;
}>>;
export declare const LocalAiProviderSchedulerStatusSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    physicalDeviceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiPhysicalDeviceId">;
    singletonScope: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["physical-device"]>>;
    providerId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiProviderId">;
    runtimeReferenceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiRuntimeReferenceId">;
    modelId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiModelId">;
    modelReference: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiModelReference">;
    resourceClass: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["cpu", "gpu", "npu", "remote-unavailable"]>>;
    lifecycleState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["idle", "running", "queued", "degraded", "unavailable"]>>;
    currentJobClass: Schema.Union<[import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["child-safety", "parent-assistant", "parent-report"]>>, typeof Schema.Null]>;
    queue: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Struct<{
        childSafetyQueued: Schema.filter<Schema.filter<typeof Schema.Number>>;
        parentAssistantQueued: Schema.filter<Schema.filter<typeof Schema.Number>>;
        parentReportQueued: Schema.filter<Schema.filter<typeof Schema.Number>>;
    }>>;
    duplicateRuntimeBlocked: typeof Schema.Boolean;
    degradedState: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["none", "provider-unavailable", "model-load-failed", "overloaded", "invalid-output"]>>;
    unavailableReason: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiUnavailableReason">, typeof Schema.Null]>;
    lastCheckedAt: Schema.brand<Schema.filter<typeof Schema.String>, "AiTimestamp">;
}>>>;
export declare const LocalAiProviderSchedulerDecisionSchema: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.filter<Schema.Struct<{
    physicalDeviceId: Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiPhysicalDeviceId">;
    jobClass: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["child-safety", "parent-assistant", "parent-report"]>>;
    jobStatus: import("@ocentra-parent/schema-domain/effect").ParsedSchema<Schema.Literal<["accepted", "running", "queued", "degraded", "unavailable", "complete"]>>;
    selectedRuntimeReferenceId: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiRuntimeReferenceId">, typeof Schema.Null]>;
    queuePosition: Schema.Union<[Schema.filter<Schema.filter<typeof Schema.Number>>, typeof Schema.Null]>;
    unavailableReason: Schema.Union<[Schema.brand<Schema.filter<typeof Schema.String>, "LocalAiUnavailableReason">, typeof Schema.Null]>;
    duplicateRuntimeBlocked: typeof Schema.Boolean;
}>>>;
export type LocalAiPhysicalDeviceId = typeof LocalAiPhysicalDeviceIdSchema.Type;
export type LocalAiProviderSingletonScope = Infer<typeof LocalAiProviderSingletonScopeSchema>;
export type LocalAiProviderSchedulerLifecycle = Infer<typeof LocalAiProviderSchedulerLifecycleSchema>;
export type LocalAiProviderSchedulerJobClass = Infer<typeof LocalAiProviderSchedulerJobClassSchema>;
export type LocalAiProviderSchedulerJobStatus = Infer<typeof LocalAiProviderSchedulerJobStatusSchema>;
export type LocalAiProviderSchedulerStatus = Infer<typeof LocalAiProviderSchedulerStatusSchema>;
export type LocalAiProviderSchedulerDecision = Infer<typeof LocalAiProviderSchedulerDecisionSchema>;
//# sourceMappingURL=local-ai-provider-scheduler.d.ts.map