import { type Infer, Schema, withParser } from './effect';
import { ActivityDeviceIdSchema, ActivityTimestampSchema } from './evidence-primitives';
import {
  ScreenEvidenceModelIdSchema,
  ScreenEvidenceModelRuntimeRefSchema,
  ScreenEvidenceQueueJobIdSchema,
  ScreenEvidenceReasonSchema,
} from './screen-evidence-primitives';
import { ScreenLocalModelProviderKindSchema } from './screen-evidence-states';

export const ScreenLocalAiResourceSchedulerSchemaVersion = 1;

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const PositiveInteger = Schema.Number.pipe(Schema.int(), Schema.positive());
const NonNegativeInteger = Schema.Number.pipe(Schema.int(), Schema.nonNegative());
export const ScreenLocalAiResourceMaxLocalImagePixels = 2073600;
export const ScreenLocalAiResourceMaxOcrSnippetCharacters = 240;

export const ScreenLocalAiResourceJobKindSchema = withParser(
  Schema.Literal(
    'ocrText',
    'vlmVisualClassifier',
    'vlmPolicyDecisionSupport',
    'deterministicStructuredContext'
  )
);

export const ScreenLocalAiResourcePrioritySchema = withParser(
  Schema.Literal(
    'policyBlocking',
    'foregroundTrigger',
    'cadenceSummary',
    'backgroundSummary'
  )
);

export const ScreenLocalAiResourceWeightSchema = withParser(
  Schema.Literal('none', 'light', 'heavy')
);

export const ScreenLocalAiResourceJobStateSchema = withParser(
  Schema.Literal(
    'running',
    'queued',
    'skipped',
    'degraded',
    'timedOut',
    'unavailable',
    'complete'
  )
);

export const ScreenLocalAiResourceQueueStateSchema = withParser(
  Schema.Literal('running', 'queued', 'degraded', 'unavailable', 'idle')
);

export const ScreenLocalAiResourceCapsSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenLocalAiResourceSchedulerSchemaVersion),
    maxImagePixels: PositiveInteger,
    ocrSnippetCharLimit: PositiveInteger,
    timeoutMs: PositiveInteger,
    oneHeavyJobPerDevice: RequiredTrue,
    localOnly: RequiredTrue,
    remoteAiAllowed: RequiredFalse,
    rawImageRetained: RequiredFalse,
  }).pipe(
    Schema.filter(
      (caps) =>
        caps.maxImagePixels <= ScreenLocalAiResourceMaxLocalImagePixels ||
        'Expected screen AI image pixels to stay locally bounded'
    ),
    Schema.filter(
      (caps) =>
        caps.ocrSnippetCharLimit <= ScreenLocalAiResourceMaxOcrSnippetCharacters ||
        'Expected screen OCR snippets to stay within the local redaction boundary'
    )
  )
);

const ScreenLocalAiResourceDecisionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLocalAiResourceSchedulerSchemaVersion),
  queueJobId: ScreenEvidenceQueueJobIdSchema,
  deviceRef: ActivityDeviceIdSchema,
  observedAt: ActivityTimestampSchema,
  jobKind: ScreenLocalAiResourceJobKindSchema,
  providerKind: ScreenLocalModelProviderKindSchema,
  priority: ScreenLocalAiResourcePrioritySchema,
  resourceWeight: ScreenLocalAiResourceWeightSchema,
  jobState: ScreenLocalAiResourceJobStateSchema,
  modelRuntimeRef: Schema.Union(ScreenEvidenceModelRuntimeRefSchema, Schema.Null),
  modelId: Schema.Union(ScreenEvidenceModelIdSchema, Schema.Null),
  queuePosition: Schema.Union(NonNegativeInteger, Schema.Null),
  caps: ScreenLocalAiResourceCapsSchema,
  duplicateRuntimeBlocked: Schema.Boolean,
  degradedReason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
  policyInputEligible: Schema.Boolean,
  remoteAiAllowed: RequiredFalse,
  rawImageRetained: RequiredFalse,
});

type ScreenLocalAiResourceDecisionCandidate = Infer<
  typeof ScreenLocalAiResourceDecisionBaseSchema
>;

export const ScreenLocalAiResourceDecisionSchema = withParser(
  ScreenLocalAiResourceDecisionBaseSchema.pipe(
    Schema.filter(
      (decision) =>
        screenLocalAiResourceDecisionIsConsistent(decision) ||
        'Expected screen local AI resource decisions to match queue, caps, runtime, and custody state'
    )
  )
);

export const ScreenLocalAiResourceQueueSnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenLocalAiResourceSchedulerSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    deviceRef: ActivityDeviceIdSchema,
    queueState: ScreenLocalAiResourceQueueStateSchema,
    activeHeavyJobCount: NonNegativeInteger,
    maxActiveHeavyJobs: Schema.Literal(1),
    pendingPolicyBlockingCount: NonNegativeInteger,
    pendingBackgroundCount: NonNegativeInteger,
    currentHeavyRuntimeRef: Schema.Union(ScreenEvidenceModelRuntimeRefSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (snapshot) =>
        snapshot.activeHeavyJobCount <= snapshot.maxActiveHeavyJobs ||
        'Expected only one heavy local AI screen job per device'
    )
  )
);

const ScreenLocalAiResourceProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLocalAiResourceSchedulerSchemaVersion),
  generatedAt: ActivityTimestampSchema,
  deviceRef: ActivityDeviceIdSchema,
  queueSnapshot: ScreenLocalAiResourceQueueSnapshotSchema,
  admissionOrder: Schema.Array(ScreenEvidenceQueueJobIdSchema).pipe(
    Schema.filter(
      (order) => order.length >= 4 || 'Expected resource proof to include policy and background ordering'
    )
  ),
  decisions: Schema.Array(ScreenLocalAiResourceDecisionSchema).pipe(
    Schema.filter(
      (decisions) => decisions.length >= 6 || 'Expected resource proof to cover all scheduler states'
    )
  ),
});

type ScreenLocalAiResourceProofCandidate = Infer<typeof ScreenLocalAiResourceProofBaseSchema>;

export const ScreenLocalAiResourceProofSchema = withParser(
  ScreenLocalAiResourceProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        screenLocalAiResourceProofIsComplete(proof) ||
        'Expected screen local AI scheduler proof to cover priority, singleton heavy lane, caps, and degraded states'
    )
  )
);

export type ScreenLocalAiResourceJobKind = Infer<typeof ScreenLocalAiResourceJobKindSchema>;
export type ScreenLocalAiResourcePriority = Infer<typeof ScreenLocalAiResourcePrioritySchema>;
export type ScreenLocalAiResourceWeight = Infer<typeof ScreenLocalAiResourceWeightSchema>;
export type ScreenLocalAiResourceJobState = Infer<typeof ScreenLocalAiResourceJobStateSchema>;
export type ScreenLocalAiResourceDecision = Infer<typeof ScreenLocalAiResourceDecisionSchema>;
export type ScreenLocalAiResourceProof = Infer<typeof ScreenLocalAiResourceProofSchema>;

export const decodeScreenLocalAiResourceProof = Schema.decodeUnknownSync(
  ScreenLocalAiResourceProofSchema
);
export const decodeScreenLocalAiResourceDecision = Schema.decodeUnknownSync(
  ScreenLocalAiResourceDecisionSchema
);

function screenLocalAiResourceDecisionIsConsistent(
  decisionRow: ScreenLocalAiResourceDecisionCandidate
): boolean {
  return (
    decisionKeepsLocalCustody(decisionRow) &&
    resourceWeightMatchesRuntime(decisionRow) &&
    jobStateMatchesScheduler(decisionRow)
  );
}

function decisionKeepsLocalCustody(
  decisionRow: ScreenLocalAiResourceDecisionCandidate
): boolean {
  return !decisionRow.remoteAiAllowed && !decisionRow.rawImageRetained;
}

function resourceWeightMatchesRuntime(
  decisionRow: ScreenLocalAiResourceDecisionCandidate
): boolean {
  if (decisionRow.resourceWeight === 'none') {
    return (
      decisionRow.providerKind === 'deterministicRules' &&
      decisionRow.queuePosition === null
    );
  }
  return decisionRow.resourceWeight !== 'heavy' || decisionRow.duplicateRuntimeBlocked;
}

function jobStateMatchesScheduler(
  decisionRow: ScreenLocalAiResourceDecisionCandidate
): boolean {
  if (decisionRow.resourceWeight === 'none') {
    return (
      decisionRow.jobState === 'complete' &&
      decisionRow.modelRuntimeRef === null &&
      decisionRow.modelId === null
    );
  }
  if (decisionRow.jobState === 'queued') {
    return (
      decisionRow.queuePosition !== null &&
      decisionRow.duplicateRuntimeBlocked &&
      hasLocalRuntime(decisionRow)
    );
  }
  if (['skipped', 'timedOut', 'unavailable', 'degraded'].includes(decisionRow.jobState)) {
    return decisionRow.degradedReason !== null && !decisionRow.policyInputEligible;
  }
  return decisionRow.queuePosition === null && hasLocalRuntime(decisionRow);
}

function hasLocalRuntime(decisionRow: ScreenLocalAiResourceDecisionCandidate): boolean {
  return decisionRow.modelRuntimeRef !== null && decisionRow.modelId !== null;
}

function screenLocalAiResourceProofIsComplete(
  proof: ScreenLocalAiResourceProofCandidate
): boolean {
  const decisions = proof.decisions;
  const priorityIndex = proof.admissionOrder.findIndex(
    (queueJobId) => queueJobId === 'screen-policy-vlm-running'
  );
  const backgroundIndex = proof.admissionOrder.findIndex(
    (queueJobId) => queueJobId === 'screen-background-vlm-queued'
  );

  return (
    proof.queueSnapshot.activeHeavyJobCount === 1 &&
    priorityIndex !== -1 &&
    backgroundIndex !== -1 &&
    priorityIndex < backgroundIndex &&
    decisions.some(
      (decisionRow) =>
        decisionRow.priority === 'policyBlocking' && decisionRow.jobState === 'running'
    ) &&
    decisions.some(
      (decisionRow) => decisionRow.jobState === 'queued' && decisionRow.queuePosition === 1
    ) &&
    decisions.some((decisionRow) => decisionRow.jobState === 'timedOut') &&
    decisions.some((decisionRow) => decisionRow.jobState === 'skipped') &&
    decisions.every(
      (decisionRow) => !decisionRow.remoteAiAllowed && !decisionRow.rawImageRetained
    )
  );
}
