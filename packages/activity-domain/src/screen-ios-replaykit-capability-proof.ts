import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import { ScreenEvidenceReasonSchema } from './screen-evidence-primitives';
import { ScreenOptionalVisibilityPlatformProofRefSchema } from './screen-optional-visibility-mode-values';

export const ScreenIosReplayKitCapabilitySchemaVersion = 1;

const ScreenIosReplayKitDocRefSchema = withParser(
  Schema.String.pipe(Schema.minLength(1), Schema.brand('ScreenIosReplayKitDocRef'))
);
const ScreenIosReplayKitReasonSchema = withParser(ScreenEvidenceReasonSchema);
const OptionalScreenIosReplayKitProofRefSchema = Schema.Union(
  ScreenOptionalVisibilityPlatformProofRefSchema,
  Schema.Null
);
const RequiredFalse = Schema.Literal(false);

export const ScreenIosReplayKitModeSchema = withParser(
  Schema.Literal('notClaimed', 'inAppReplayKitSession', 'broadcastUploadExtension')
);

export const ScreenIosReplayKitCaptureStateSchema = withParser(Schema.Literal('notClaimed', 'manualRequired', 'ready'));

export const ScreenIosReplayKitProofStateSchema = withParser(
  Schema.Literal('sourceDocsVerified', 'physicalDeviceMissing', 'physicalDeviceVerified')
);

const ScreenIosReplayKitCapabilityRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenIosReplayKitCapabilitySchemaVersion),
  checkedAt: ActivityTimestampSchema,
  mode: ScreenIosReplayKitModeSchema,
  captureState: ScreenIosReplayKitCaptureStateSchema,
  proofState: ScreenIosReplayKitProofStateSchema,
  appleDocRefs: Schema.NonEmptyArray(ScreenIosReplayKitDocRefSchema),
  requiresExplicitUserStart: Schema.Boolean,
  requiresReplayKitUi: Schema.Boolean,
  requiresBroadcastUploadExtension: Schema.Boolean,
  arbitraryBackgroundOtherAppCaptureClaimed: RequiredFalse,
  rawFrameRemoteUploadAllowed: RequiredFalse,
  rawFrameRetentionDefault: RequiredFalse,
  physicalDeviceProofRef: OptionalScreenIosReplayKitProofRefSchema,
  deletionProofRef: OptionalScreenIosReplayKitProofRefSchema,
  productCaptureReady: Schema.Boolean,
  reason: ScreenEvidenceReasonSchema,
});

type ScreenIosReplayKitCapabilityRowInput = Infer<typeof ScreenIosReplayKitCapabilityRowBaseSchema>;

export const ScreenIosReplayKitCapabilityRowSchema = withParser(
  ScreenIosReplayKitCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenIosReplayKitCapabilityRowIsConsistent(value) ||
        'Expected iOS ReplayKit readiness to require explicit session/broadcast proof, physical device evidence, deletion proof, no arbitrary background capture, no remote raw upload, and no raw-retention default'
    )
  )
);

const ScreenIosReplayKitCapabilityProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenIosReplayKitCapabilitySchemaVersion),
  generatedAt: ActivityTimestampSchema,
  proofId: Schema.Literal('screen-ios-replaykit-capability-proof'),
  rows: Schema.NonEmptyArray(ScreenIosReplayKitCapabilityRowSchema),
  appleDocsVerifiedAt: ActivityTimestampSchema,
  productIosCaptureReady: Schema.Boolean,
  nonClaims: Schema.NonEmptyArray(ScreenEvidenceReasonSchema),
});

type ScreenIosReplayKitCapabilityProofInput = Infer<typeof ScreenIosReplayKitCapabilityProofBaseSchema>;

export const ScreenIosReplayKitCapabilityProofSchema = withParser(
  ScreenIosReplayKitCapabilityProofBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenIosReplayKitCapabilityProofIsConsistent(value) ||
        'Expected iOS ReplayKit proof to keep product readiness false until every row is physically proved'
    )
  )
);

export function screenIosReplayKitCapabilityRowIsConsistent(value: ScreenIosReplayKitCapabilityRowInput): boolean {
  if (value.arbitraryBackgroundOtherAppCaptureClaimed) {
    return false;
  }

  if (value.rawFrameRemoteUploadAllowed || value.rawFrameRetentionDefault) {
    return false;
  }

  if (value.mode === 'notClaimed') {
    return (
      value.captureState === 'notClaimed' &&
      value.proofState === 'sourceDocsVerified' &&
      value.physicalDeviceProofRef === null &&
      value.deletionProofRef === null &&
      !value.productCaptureReady
    );
  }

  if (value.mode === 'broadcastUploadExtension' && !value.requiresBroadcastUploadExtension) {
    return false;
  }

  if (value.mode === 'inAppReplayKitSession' && value.requiresBroadcastUploadExtension) {
    return false;
  }

  if (!value.requiresExplicitUserStart || !value.requiresReplayKitUi) {
    return false;
  }

  if (!value.productCaptureReady) {
    return value.captureState === 'manualRequired' && value.proofState === 'physicalDeviceMissing';
  }

  return (
    value.captureState === 'ready' &&
    value.proofState === 'physicalDeviceVerified' &&
    value.physicalDeviceProofRef !== null &&
    value.deletionProofRef !== null
  );
}

export function screenIosReplayKitCapabilityProofIsConsistent(value: ScreenIosReplayKitCapabilityProofInput): boolean {
  const everyRowReady = value.rows.every((row: ScreenIosReplayKitCapabilityRowInput) => row.productCaptureReady);
  return value.productIosCaptureReady === everyRowReady;
}

export function screenIosReplayKitCapabilityProof(generatedAt: string) {
  return ScreenIosReplayKitCapabilityProofSchema.parse({
    schemaVersion: ScreenIosReplayKitCapabilitySchemaVersion,
    generatedAt,
    proofId: 'screen-ios-replaykit-capability-proof',
    appleDocsVerifiedAt: generatedAt,
    productIosCaptureReady: false,
    rows: [
      sourceDocManualRequiredRow(generatedAt, 'inAppReplayKitSession', {
        reason: ScreenIosReplayKitReasonSchema.parse(
          'ReplayKit in-app capture remains manual-required until a real iOS device session captures pixels and proves deletion.'
        ),
      }),
      sourceDocManualRequiredRow(generatedAt, 'broadcastUploadExtension', {
        requiresBroadcastUploadExtension: true,
        reason: ScreenIosReplayKitReasonSchema.parse(
          'ReplayKit broadcast upload extension remains manual-required until a real extension processes frames and proves deletion.'
        ),
      }),
      notClaimedBackgroundCaptureRow(generatedAt),
    ],
    nonClaims: [
      'This proof does not claim iOS arbitrary other-app background capture.',
      'This proof does not claim physical iOS ReplayKit execution, live iOS pixels, or iOS deletion proof.',
      'This proof keeps raw frame remote upload and raw-retention-by-default disabled.',
    ],
  });
}

function sourceDocManualRequiredRow(
  checkedAt: string,
  mode: Exclude<ScreenIosReplayKitMode, 'notClaimed'>,
  overrides: Partial<ScreenIosReplayKitCapabilityRow>
) {
  return ScreenIosReplayKitCapabilityRowSchema.parse({
    ...baseRow(checkedAt, mode),
    ...overrides,
  });
}

function notClaimedBackgroundCaptureRow(checkedAt: string) {
  return ScreenIosReplayKitCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'notClaimed'),
    captureState: 'notClaimed',
    proofState: 'sourceDocsVerified',
    requiresExplicitUserStart: false,
    requiresReplayKitUi: false,
    reason:
      'Arbitrary silent background capture of other iOS apps is not a ReplayKit product claim and remains blocked before proof.',
  });
}

function baseRow(checkedAt: string, mode: ScreenIosReplayKitMode) {
  return {
    schemaVersion: ScreenIosReplayKitCapabilitySchemaVersion,
    checkedAt,
    mode,
    captureState: 'manualRequired',
    proofState: 'physicalDeviceMissing',
    appleDocRefs: [
      ScreenIosReplayKitDocRefSchema.parse('apple-developer-replaykit'),
      ScreenIosReplayKitDocRefSchema.parse('apple-developer-rpscreenrecorder'),
      ScreenIosReplayKitDocRefSchema.parse('apple-developer-rpbroadcastsamplehandler'),
    ],
    requiresExplicitUserStart: true,
    requiresReplayKitUi: true,
    requiresBroadcastUploadExtension: false,
    arbitraryBackgroundOtherAppCaptureClaimed: false,
    rawFrameRemoteUploadAllowed: false,
    rawFrameRetentionDefault: false,
    physicalDeviceProofRef: null,
    deletionProofRef: null,
    productCaptureReady: false,
    reason: 'iOS ReplayKit capture requires physical device proof before product readiness.',
  };
}

export type ScreenIosReplayKitMode = Infer<typeof ScreenIosReplayKitModeSchema>;
export type ScreenIosReplayKitCaptureState = Infer<typeof ScreenIosReplayKitCaptureStateSchema>;
export type ScreenIosReplayKitProofState = Infer<typeof ScreenIosReplayKitProofStateSchema>;
export type ScreenIosReplayKitCapabilityRow = Infer<typeof ScreenIosReplayKitCapabilityRowSchema>;
export type ScreenIosReplayKitCapabilityProof = Infer<typeof ScreenIosReplayKitCapabilityProofSchema>;
