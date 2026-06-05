import { type Infer } from '@ocentra-parent/schema-domain/effect';
import type { ScreenLocalModelProviderKindSchema } from './screen-evidence-states';
import {
  ScreenLocalAiResourceCapsSchema,
  ScreenLocalAiResourceDecisionSchema,
  ScreenLocalAiResourceMaxLocalImagePixels,
  ScreenLocalAiResourceMaxOcrSnippetCharacters,
  ScreenLocalAiResourceProofSchema,
  ScreenLocalAiResourceSchedulerSchemaVersion,
  type ScreenLocalAiResourceDecision,
  type ScreenLocalAiResourceJobKind,
  type ScreenLocalAiResourceJobState,
  type ScreenLocalAiResourcePriority,
  type ScreenLocalAiResourceProof,
  type ScreenLocalAiResourceWeight,
} from './screen-evidence-resource-scheduler';

const DeviceRef = 'windows-child-device';
const ObservedAt = '2026-06-05T01:06:00.000Z';
const RuntimeRef = 'screen-local-ai-runtime-singleton';
const VisionModelId = 'screen-local-vlm-safety-model';
const OcrModelId = 'screen-local-ocr-engine';
const Caps = ScreenLocalAiResourceCapsSchema.parse({
  schemaVersion: ScreenLocalAiResourceSchedulerSchemaVersion,
  maxImagePixels: ScreenLocalAiResourceMaxLocalImagePixels,
  ocrSnippetCharLimit: ScreenLocalAiResourceMaxOcrSnippetCharacters,
  timeoutMs: 30000,
  oneHeavyJobPerDevice: true,
  localOnly: true,
  remoteAiAllowed: false,
  rawImageRetained: false,
});

export const screenLocalAiResourceSchedulerDecisions: ScreenLocalAiResourceDecision[] = [
  decision('screen-policy-vlm-running', 'vlmPolicyDecisionSupport', 'localVision', 'policyBlocking', 'heavy', 'running'),
  decision('screen-foreground-ocr-complete', 'ocrText', 'localOcr', 'foregroundTrigger', 'light', 'complete'),
  decision('screen-cadence-vlm-queued', 'vlmVisualClassifier', 'localVision', 'cadenceSummary', 'heavy', 'queued', 1),
  decision('screen-background-vlm-queued', 'vlmVisualClassifier', 'localVision', 'backgroundSummary', 'heavy', 'queued', 2),
  decision('screen-foreground-vlm-timeout', 'vlmVisualClassifier', 'localVision', 'foregroundTrigger', 'heavy', 'timedOut'),
  decision('screen-protected-surface-skipped', 'ocrText', 'unavailable', 'foregroundTrigger', 'light', 'skipped'),
  decision(
    'screen-structured-context-complete',
    'deterministicStructuredContext',
    'deterministicRules',
    'foregroundTrigger',
    'none',
    'complete'
  ),
];

export const screenLocalAiResourceSchedulerProof: ScreenLocalAiResourceProof = ScreenLocalAiResourceProofSchema.parse({
  schemaVersion: ScreenLocalAiResourceSchedulerSchemaVersion,
  generatedAt: ObservedAt,
  deviceRef: DeviceRef,
  queueSnapshot: {
    schemaVersion: ScreenLocalAiResourceSchedulerSchemaVersion,
    generatedAt: ObservedAt,
    deviceRef: DeviceRef,
    queueState: 'queued',
    activeHeavyJobCount: 1,
    maxActiveHeavyJobs: 1,
    pendingPolicyBlockingCount: 0,
    pendingBackgroundCount: 2,
    currentHeavyRuntimeRef: RuntimeRef,
  },
  admissionOrder: [
    'screen-policy-vlm-running',
    'screen-foreground-ocr-complete',
    'screen-cadence-vlm-queued',
    'screen-background-vlm-queued',
  ],
  decisions: screenLocalAiResourceSchedulerDecisions,
});

export function screenLocalAiResourceSchedulerProofSummary(
  decisions: readonly ScreenLocalAiResourceDecision[] = screenLocalAiResourceSchedulerDecisions
) {
  return {
    totalJobs: decisions.length,
    heavyJobs: decisions.filter((decisionRow) => decisionRow.resourceWeight === 'heavy').length,
    queuedJobs: decisions.filter((decisionRow) => decisionRow.jobState === 'queued').length,
    timedOutJobs: decisions.filter((decisionRow) => decisionRow.jobState === 'timedOut').length,
    skippedOrDegradedJobs: decisions.filter((decisionRow) => ['skipped', 'degraded'].includes(decisionRow.jobState))
      .length,
    policyBlockingJobs: decisions.filter((decisionRow) => decisionRow.priority === 'policyBlocking').length,
    remoteAiAllowed: false,
    rawImageRetained: false,
  };
}

function decision(
  queueJobId: string,
  jobKind: ScreenLocalAiResourceJobKind,
  providerKind: Infer<typeof ScreenLocalModelProviderKindSchema>,
  priority: ScreenLocalAiResourcePriority,
  resourceWeight: ScreenLocalAiResourceWeight,
  jobState: ScreenLocalAiResourceJobState,
  queuePosition: number | null = null
): ScreenLocalAiResourceDecision {
  return ScreenLocalAiResourceDecisionSchema.parse({
    schemaVersion: ScreenLocalAiResourceSchedulerSchemaVersion,
    queueJobId,
    deviceRef: DeviceRef,
    observedAt: ObservedAt,
    jobKind,
    providerKind,
    priority,
    resourceWeight,
    jobState,
    modelRuntimeRef: resourceWeight === 'none' || providerKind === 'unavailable' ? null : RuntimeRef,
    modelId:
      resourceWeight === 'none' || providerKind === 'unavailable'
        ? null
        : providerKind === 'localOcr'
          ? OcrModelId
          : VisionModelId,
    queuePosition,
    caps: Caps,
    duplicateRuntimeBlocked: resourceWeight === 'heavy',
    degradedReason: degradedReasonFor(jobState),
    policyInputEligible: jobState === 'complete' && providerKind !== 'unavailable',
    remoteAiAllowed: false,
    rawImageRetained: false,
  });
}

function degradedReasonFor(jobState: ScreenLocalAiResourceJobState) {
  if (jobState === 'timedOut') {
    return 'screen local AI job timed out before policy handoff';
  }
  if (jobState === 'skipped') {
    return 'protected surface or permission boundary skipped local screen AI';
  }
  if (jobState === 'degraded' || jobState === 'unavailable') {
    return 'local screen AI resource unavailable or degraded';
  }
  return null;
}
