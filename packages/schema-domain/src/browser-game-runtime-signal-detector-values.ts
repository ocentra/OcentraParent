import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const BrowserGameRuntimeSignalDetectorSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-runtime-signal-detector-contract')
);

export const BrowserGameRuntimeSignalDetectionIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameRuntimeSignalDetectionId')
);

export const BrowserGameRuntimeSignalIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameRuntimeSignalId')
);

export const BrowserGameRuntimeSignalFingerprintSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameRuntimeSignalFingerprint')
);

export const BrowserGameRuntimeSignalKindSchema = withParser(
  Schema.Literal(
    'canvas-present-shape',
    'webgl-present-shape',
    'gamepad-api-shape',
    'fullscreen-request-shape',
    'pointer-lock-shape',
    'audio-context-shape',
    'animation-loop-shape',
    'iframe-game-surface-shape',
    'cloud-streaming-shape',
    'unknown'
  )
);

export const BrowserGameRuntimeSignalSourceKindSchema = withParser(
  Schema.Literal(
    'managed-browser-signal-ref',
    'tab-evidence-ref',
    'url-shape-ref',
    'metadata-shape-ref',
    'manual-review-ref',
    'unavailable'
  )
);

export const BrowserGameRuntimeSignalStatusSchema = withParser(
  Schema.Literal('detected-shape', 'candidate-shape', 'manual-required', 'unavailable')
);

export const BrowserGameRuntimeSignalConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameRuntimeSignalReasonCodeSchema = withParser(
  Schema.Literal(
    'runtime-shape-present',
    'canvas-shape-present',
    'webgl-shape-present',
    'gamepad-shape-present',
    'fullscreen-shape-present',
    'pointer-lock-shape-present',
    'audio-shape-present',
    'animation-loop-shape-present',
    'iframe-surface-shape-present',
    'cloud-streaming-shape-present',
    'managed-browser-proof-required',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameRuntimeSignalEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game runtime signal evidence refs')
);

export const BrowserGameRuntimeSignalReasonCodesSchema = Schema.Array(BrowserGameRuntimeSignalReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game runtime signal reason codes')
);

export type BrowserGameRuntimeSignalConfidence = Infer<typeof BrowserGameRuntimeSignalConfidenceSchema>;
export type BrowserGameRuntimeSignalKind = Infer<typeof BrowserGameRuntimeSignalKindSchema>;
export type BrowserGameRuntimeSignalReasonCode = Infer<typeof BrowserGameRuntimeSignalReasonCodeSchema>;
export type BrowserGameRuntimeSignalSourceKind = Infer<typeof BrowserGameRuntimeSignalSourceKindSchema>;
export type BrowserGameRuntimeSignalStatus = Infer<typeof BrowserGameRuntimeSignalStatusSchema>;

