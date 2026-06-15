import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ScreenVlmWorkerSchemaVersion } from './screen-vlm-worker';

export const ScreenVlmExecutionReadinessSchemaVersion = ScreenVlmWorkerSchemaVersion;
export const ScreenVlmExecutionReadinessProofTier = 'P3_CONTRACT_LOCAL_VLM_EXECUTION_READINESS';
export const ScreenVlmExecutionReadinessStatusSource = 'activity-domain-screen-vlm-execution-readiness';

const RequiredFalse = Schema.Literal(false);

export const ScreenVlmExecutionReadinessStateSchema = withParser(
  Schema.Literal('queued', 'ready', 'running', 'completed', 'degraded', 'manual-required', 'unavailable')
);

export const ScreenVlmExecutionReadinessNonClaimsSchema = withParser(
  Schema.Struct({
    liveModelExecutionClaimed: RequiredFalse,
    productionVlmQualityClaimed: RequiredFalse,
    portalRuntimeClaimed: RequiredFalse,
    enforcementClaimed: RequiredFalse,
    remoteAiUsed: RequiredFalse,
    rawImageRetained: RequiredFalse,
  })
);
