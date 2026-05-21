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

export const LocalAiAdapterProbeStateSchema = withParser(
  Schema.Literal('probe-unavailable', 'probe-ready', 'probe-failed')
);

export const LocalAiProviderConfigurationStateSchema = withParser(
  Schema.Literal('local-provider-unconfigured', 'local-provider-configured', 'local-provider-config-invalid')
);

export const LocalAiAdapterReadinessStateSchema = withParser(
  Schema.Literal('adapter-not-ready', 'adapter-ready', 'adapter-readiness-invalid')
);

const LocalModelRuntimeStatusBaseSchema = Schema.Struct({
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
});

const LocalProviderAdapterProbeBaseSchema = Schema.Struct({
  providerId: LocalAiProviderIdSchema,
  privacyMode: LocalAiProviderPrivacyModeSchema,
  adapterBoundary: LocalAiAdapterBoundarySchema,
  executionState: LocalAiExecutionStateSchema,
  providerSource: LocalAiProviderSourceSchema,
  probeState: LocalAiAdapterProbeStateSchema,
  configurationState: LocalAiProviderConfigurationStateSchema,
  readinessState: LocalAiAdapterReadinessStateSchema,
  executionAllowed: Schema.Boolean,
  lastCheckedAt: LocalAiTimestampSchema,
  unavailableReason: Schema.Union(LocalAiUnavailableReasonSchema, Schema.Null),
});

type LocalProviderAdapterProbeCandidate = Infer<typeof LocalProviderAdapterProbeBaseSchema>;

export const LocalModelRuntimeStatusSchema = withParser(LocalModelRuntimeStatusBaseSchema);

export const LocalProviderAdapterProbeSchema = withParser(
  LocalProviderAdapterProbeBaseSchema.pipe(
    Schema.filter(
      (probe) =>
        localProviderAdapterProbeReadinessIsConsistent(probe) ||
        'Expected local provider adapter readiness to match execution permission'
    )
  )
);

function localProviderAdapterProbeReadinessIsConsistent(probe: LocalProviderAdapterProbeCandidate): boolean {
  if (probe.readinessState === 'adapter-not-ready') {
    return probe.executionAllowed === false;
  }

  if (probe.readinessState === 'adapter-ready') {
    return (
      probe.executionAllowed === true &&
      probe.probeState === 'probe-ready' &&
      probe.configurationState === 'local-provider-configured' &&
      probe.adapterBoundary === 'local-adapter-ready' &&
      probe.executionState === 'dry-run-ready' &&
      probe.providerSource !== 'unavailable' &&
      probe.unavailableReason === null
    );
  }

  return (
    probe.executionAllowed === false &&
    (probe.probeState === 'probe-failed' || probe.configurationState === 'local-provider-config-invalid')
  );
}

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
export type LocalProviderAdapterProbe = Infer<typeof LocalProviderAdapterProbeSchema>;
export type LocalProviderCapability = Infer<typeof LocalProviderCapabilitySchema>;
export type LocalAiModelRequestMetadata = Infer<typeof LocalAiModelRequestMetadataSchema>;
export type LocalAiProviderPrivacyMode = Infer<typeof LocalAiProviderPrivacyModeSchema>;
export type LocalAiAdapterBoundary = Infer<typeof LocalAiAdapterBoundarySchema>;
export type LocalAiExecutionState = Infer<typeof LocalAiExecutionStateSchema>;
export type LocalAiProviderSource = Infer<typeof LocalAiProviderSourceSchema>;
export type LocalAiAdapterProbeState = Infer<typeof LocalAiAdapterProbeStateSchema>;
export type LocalAiProviderConfigurationState = Infer<typeof LocalAiProviderConfigurationStateSchema>;
export type LocalAiAdapterReadinessState = Infer<typeof LocalAiAdapterReadinessStateSchema>;
