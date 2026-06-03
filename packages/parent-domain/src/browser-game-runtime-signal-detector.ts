import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  BrowserGameRuntimeSignalConfidenceSchema,
  BrowserGameRuntimeSignalDetectionIdSchema,
  BrowserGameRuntimeSignalDetectorSchemaVersionSchema,
  BrowserGameRuntimeSignalEvidenceRefsSchema,
  BrowserGameRuntimeSignalFingerprintSchema,
  BrowserGameRuntimeSignalIdSchema,
  BrowserGameRuntimeSignalKindSchema,
  BrowserGameRuntimeSignalReasonCodesSchema,
  BrowserGameRuntimeSignalSourceKindSchema,
  BrowserGameRuntimeSignalStatusSchema,
} from './browser-game-runtime-signal-detector-values';

const BrowserGameRuntimeSignalRowBaseSchema = Schema.Struct({
  signalId: BrowserGameRuntimeSignalIdSchema,
  signalKind: BrowserGameRuntimeSignalKindSchema,
  signalFingerprint: BrowserGameRuntimeSignalFingerprintSchema,
  sourceKind: BrowserGameRuntimeSignalSourceKindSchema,
  sourceEvidenceRefs: BrowserGameRuntimeSignalEvidenceRefsSchema,
  confidence: BrowserGameRuntimeSignalConfidenceSchema,
  status: BrowserGameRuntimeSignalStatusSchema,
  reasonCodes: BrowserGameRuntimeSignalReasonCodesSchema,
  managedBrowserProofRequired: Schema.Boolean,
  childLaunchCandidate: Schema.Boolean,
  cloudSessionCandidate: Schema.Boolean,
  rawDomStored: Schema.Boolean,
  rawCanvasFrameStored: Schema.Boolean,
  rawStreamFrameStored: Schema.Boolean,
  rawAudioStored: Schema.Boolean,
  rawGamepadInputStored: Schema.Boolean,
  browserInstrumentationClaimed: Schema.Boolean,
  runtimeDetectionExecutedClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameRuntimeSignalRowCandidate = Infer<typeof BrowserGameRuntimeSignalRowBaseSchema>;

export const BrowserGameRuntimeSignalRowSchema = withParser(
  BrowserGameRuntimeSignalRowBaseSchema.pipe(
    Schema.filter(
      (signal) =>
        browserGameRuntimeSignalRowIsHonest(signal) ||
        'Expected browser-game runtime signal row to stay shape-only and non-executing'
    )
  )
);

const BrowserGameRuntimeSignalRowsSchema = Schema.Array(BrowserGameRuntimeSignalRowSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game runtime signal rows')
);

const BrowserGameRuntimeSignalDetectionBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameRuntimeSignalDetectorSchemaVersionSchema,
  detectionId: BrowserGameRuntimeSignalDetectionIdSchema,
  detectedAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGameRuntimeSignalEvidenceRefsSchema,
  signals: BrowserGameRuntimeSignalRowsSchema,
  confidence: BrowserGameRuntimeSignalConfidenceSchema,
  status: BrowserGameRuntimeSignalStatusSchema,
  rawDomStored: Schema.Boolean,
  rawCanvasFrameStored: Schema.Boolean,
  rawStreamFrameStored: Schema.Boolean,
  rawAudioStored: Schema.Boolean,
  rawGamepadInputStored: Schema.Boolean,
  browserInstrumentationClaimed: Schema.Boolean,
  runtimeDetectionExecutedClaimed: Schema.Boolean,
  aiClassificationClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameRuntimeSignalDetectionCandidate = Infer<typeof BrowserGameRuntimeSignalDetectionBaseSchema>;

export const BrowserGameRuntimeSignalDetectionSchema = withParser(
  BrowserGameRuntimeSignalDetectionBaseSchema.pipe(
    Schema.filter(
      (detection) =>
        browserGameRuntimeSignalDetectionIsHonest(detection) ||
        'Expected browser-game runtime signal detection to remain contract-only'
    )
  )
);

export const decodeBrowserGameRuntimeSignalDetection = Schema.decodeUnknownSync(
  BrowserGameRuntimeSignalDetectionSchema
);

export type BrowserGameRuntimeSignalDetection = Infer<typeof BrowserGameRuntimeSignalDetectionSchema>;
export type BrowserGameRuntimeSignalRow = Infer<typeof BrowserGameRuntimeSignalRowSchema>;

function browserGameRuntimeSignalRowIsHonest(signal: BrowserGameRuntimeSignalRowCandidate): boolean {
  if (browserGameRuntimeSignalRowClaimsRuntime(signal) || browserGameRuntimeSignalPurposeIsInconsistent(signal)) {
    return false;
  }
  if (signal.status === 'detected-shape') {
    return (
      signal.confidence !== 'unknown' &&
      signal.signalKind !== 'unknown' &&
      signal.sourceKind !== 'manual-review-ref' &&
      signal.sourceKind !== 'unavailable' &&
      signal.managedBrowserProofRequired
    );
  }
  if (signal.status === 'candidate-shape') {
    return signal.confidence !== 'high' && signal.signalKind !== 'unknown' && signal.sourceKind !== 'unavailable';
  }
  return (
    signal.confidence !== 'high' &&
    (signal.signalKind === 'unknown' ||
      signal.sourceKind === 'manual-review-ref' ||
      signal.sourceKind === 'unavailable' ||
      signal.reasonCodes.includes('manual-required') ||
      signal.reasonCodes.includes('unavailable'))
  );
}

function browserGameRuntimeSignalDetectionIsHonest(detection: BrowserGameRuntimeSignalDetectionCandidate): boolean {
  if (browserGameRuntimeSignalDetectionClaimsRuntime(detection)) {
    return false;
  }
  if (detection.status === 'detected-shape') {
    return (
      detection.confidence !== 'unknown' && detection.signals.every((signal) => signal.status === 'detected-shape')
    );
  }
  return detection.confidence !== 'high' && detection.signals.some((signal) => signal.status !== 'detected-shape');
}

function browserGameRuntimeSignalPurposeIsInconsistent(signal: BrowserGameRuntimeSignalRowCandidate): boolean {
  if (signal.cloudSessionCandidate && signal.signalKind !== 'cloud-streaming-shape') {
    return true;
  }
  if (signal.childLaunchCandidate && signal.signalKind === 'unknown') {
    return true;
  }
  return signal.status === 'detected-shape' && !signal.reasonCodes.includes('managed-browser-proof-required');
}

function browserGameRuntimeSignalRowClaimsRuntime(signal: BrowserGameRuntimeSignalRowCandidate): boolean {
  return (
    signal.rawDomStored ||
    signal.rawCanvasFrameStored ||
    signal.rawStreamFrameStored ||
    signal.rawAudioStored ||
    signal.rawGamepadInputStored ||
    signal.browserInstrumentationClaimed ||
    signal.runtimeDetectionExecutedClaimed ||
    signal.aiClassificationClaimed ||
    signal.policyDecisionClaimed ||
    signal.cloudFrameAnalysisClaimed ||
    signal.nativeGameControlClaimed ||
    signal.enforcementClaimed
  );
}

function browserGameRuntimeSignalDetectionClaimsRuntime(
  detection: BrowserGameRuntimeSignalDetectionCandidate
): boolean {
  return (
    detection.rawDomStored ||
    detection.rawCanvasFrameStored ||
    detection.rawStreamFrameStored ||
    detection.rawAudioStored ||
    detection.rawGamepadInputStored ||
    detection.browserInstrumentationClaimed ||
    detection.runtimeDetectionExecutedClaimed ||
    detection.aiClassificationClaimed ||
    detection.policyDecisionClaimed ||
    detection.cloudFrameAnalysisClaimed ||
    detection.nativeGameControlClaimed ||
    detection.enforcementClaimed
  );
}
