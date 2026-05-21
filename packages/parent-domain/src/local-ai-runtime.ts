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

export const LocalAiProviderPrivacyModeSchema = withParser(Schema.Literal('local-only'));

export const LocalAiAdapterBoundarySchema = withParser(
  Schema.Literal('status-only', 'local-adapter-unavailable', 'local-adapter-ready')
);

export const LocalAiExecutionStateSchema = withParser(Schema.Literal('disabled', 'dry-run-ready', 'running', 'failed'));

export const LocalAiProviderSourceSchema = withParser(
  Schema.Literal('unavailable', 'local-config', 'local-model-cache', 'os-capability-probe')
);

export const LocalModelRuntimeStatusSchema = withParser(
  Schema.Struct({
    runtimeReferenceId: LocalAiRuntimeReferenceIdSchema,
    providerId: LocalAiProviderIdSchema,
    modelId: LocalAiModelIdSchema,
    modelReference: LocalAiModelReferenceSchema,
    privacyMode: LocalAiProviderPrivacyModeSchema,
    adapterBoundary: LocalAiAdapterBoundarySchema,
    executionState: LocalAiExecutionStateSchema,
    providerSource: LocalAiProviderSourceSchema,
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
    privacyMode: LocalAiProviderPrivacyModeSchema,
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
export type LocalAiProviderPrivacyMode = Infer<typeof LocalAiProviderPrivacyModeSchema>;
export type LocalAiAdapterBoundary = Infer<typeof LocalAiAdapterBoundarySchema>;
export type LocalAiExecutionState = Infer<typeof LocalAiExecutionStateSchema>;
export type LocalAiProviderSource = Infer<typeof LocalAiProviderSourceSchema>;
