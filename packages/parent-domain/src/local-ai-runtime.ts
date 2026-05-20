import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiCapabilityFlagSchema,
  LocalAiDegradedStateSchema,
  LocalAiModelIdSchema,
  LocalAiModelReferenceSchema,
  LocalAiModelLoadStateSchema,
  LocalAiPromptVersionSchema,
  LocalAiProviderIdSchema,
  LocalAiResourceClassSchema,
  LocalAiRuntimeReferenceIdSchema,
  LocalAiTimestampSchema,
  LocalAiUnavailableReasonSchema,
} from './local-ai-primitives';

export const LocalModelRuntimeStatusSchema = withParser(
  Schema.Struct({
    runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
    providerId: LocalAiProviderIdSchema,
    modelId: LocalAiModelIdSchema,
    modelReference: LocalAiModelReferenceSchema,
    loadState: LocalAiModelLoadStateSchema,
    capabilityFlags: Schema.Array(LocalAiCapabilityFlagSchema),
    resourceClass: LocalAiResourceClassSchema,
    degradedState: LocalAiDegradedStateSchema,
    lastCheckedAt: LocalAiTimestampSchema,
    unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
  })
);

export const LocalProviderCapabilitySchema = withParser(
  Schema.Struct({
    providerId: LocalAiProviderIdSchema,
    supportedTasks: Schema.Array(LocalAiCapabilityFlagSchema),
    resourceClass: LocalAiResourceClassSchema,
    privacyMode: Schema.Literal('local-only'),
    fallbackOrder: Schema.Number,
  })
);

export const LocalAiModelRequestMetadataSchema = withParser(
  Schema.Struct({
    providerId: LocalAiProviderIdSchema,
    modelId: LocalAiModelIdSchema,
    promptVersion: LocalAiPromptVersionSchema,
  })
);

export type LocalModelRuntimeStatus = Infer<typeof LocalModelRuntimeStatusSchema>;
export type LocalProviderCapability = Infer<typeof LocalProviderCapabilitySchema>;
export type LocalAiModelRequestMetadata = Infer<typeof LocalAiModelRequestMetadataSchema>;
