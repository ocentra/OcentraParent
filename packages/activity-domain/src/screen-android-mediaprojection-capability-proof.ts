import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import { ScreenEvidenceReasonSchema } from './screen-evidence-primitives';
import { ScreenOptionalVisibilityPlatformProofRefSchema } from './screen-optional-visibility-mode-values';

export const ScreenAndroidMediaProjectionCapabilitySchemaVersion = 1;

const ScreenAndroidMediaProjectionDocRefSchema = withParser(
  Schema.String.pipe(Schema.minLength(1), Schema.brand('ScreenAndroidMediaProjectionDocRef'))
);
const ScreenAndroidMediaProjectionReasonSchema = withParser(ScreenEvidenceReasonSchema);
const OptionalScreenAndroidMediaProjectionProofRefSchema = Schema.Union(
  ScreenOptionalVisibilityPlatformProofRefSchema,
  Schema.Null
);
const RequiredFalse = Schema.Literal(false);

export const ScreenAndroidMediaProjectionModeSchema = withParser(
  Schema.Literal('emulatorMediaProjection', 'physicalDeviceMediaProjection', 'android14AppWindowSharing', 'notClaimed')
);

export const ScreenAndroidMediaProjectionCaptureStateSchema = withParser(
  Schema.Literal('provedEmulator', 'manualRequired', 'ready', 'notClaimed')
);

export const ScreenAndroidMediaProjectionProofStateSchema = withParser(
  Schema.Literal('sourceDocsVerified', 'emulatorVerified', 'physicalDeviceMissing', 'physicalDeviceVerified')
);

const ScreenAndroidMediaProjectionCapabilityRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenAndroidMediaProjectionCapabilitySchemaVersion),
  checkedAt: ActivityTimestampSchema,
  mode: ScreenAndroidMediaProjectionModeSchema,
  captureState: ScreenAndroidMediaProjectionCaptureStateSchema,
  proofState: ScreenAndroidMediaProjectionProofStateSchema,
  androidDocRefs: Schema.NonEmptyArray(ScreenAndroidMediaProjectionDocRefSchema),
  requiresUserConsentPerSession: Schema.Boolean,
  requiresForegroundServiceType: Schema.Boolean,
  requiresStopCallbackOnUserStop: Schema.Boolean,
  supportsAppWindowSelection: Schema.Boolean,
  silentBackgroundCaptureClaimed: RequiredFalse,
  rawFrameRemoteUploadAllowed: RequiredFalse,
  rawFrameRetentionDefault: RequiredFalse,
  emulatorProofRef: OptionalScreenAndroidMediaProjectionProofRefSchema,
  physicalDeviceProofRef: OptionalScreenAndroidMediaProjectionProofRefSchema,
  deletionProofRef: OptionalScreenAndroidMediaProjectionProofRefSchema,
  productAndroidCaptureReady: Schema.Boolean,
  reason: ScreenEvidenceReasonSchema,
});

type ScreenAndroidMediaProjectionCapabilityRowInput = Infer<typeof ScreenAndroidMediaProjectionCapabilityRowBaseSchema>;

export const ScreenAndroidMediaProjectionCapabilityRowSchema = withParser(
  ScreenAndroidMediaProjectionCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenAndroidMediaProjectionCapabilityRowIsConsistent(value) ||
        'Expected Android MediaProjection readiness to require per-session consent, foreground service type, physical-device proof, deletion proof, no silent background capture, no remote raw upload, and no raw-retention default'
    )
  )
);

const ScreenAndroidMediaProjectionCapabilityProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenAndroidMediaProjectionCapabilitySchemaVersion),
  generatedAt: ActivityTimestampSchema,
  proofId: Schema.Literal('screen-android-mediaprojection-capability-proof'),
  rows: Schema.NonEmptyArray(ScreenAndroidMediaProjectionCapabilityRowSchema),
  androidDocsVerifiedAt: ActivityTimestampSchema,
  emulatorCaptureProved: Schema.Boolean,
  productAndroidCaptureReady: Schema.Boolean,
  nonClaims: Schema.NonEmptyArray(ScreenEvidenceReasonSchema),
});

type ScreenAndroidMediaProjectionCapabilityProofInput = Infer<
  typeof ScreenAndroidMediaProjectionCapabilityProofBaseSchema
>;

export const ScreenAndroidMediaProjectionCapabilityProofSchema = withParser(
  ScreenAndroidMediaProjectionCapabilityProofBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenAndroidMediaProjectionCapabilityProofIsConsistent(value) ||
        'Expected Android MediaProjection proof to keep product readiness false until physical-device rows are proved'
    )
  )
);

export function screenAndroidMediaProjectionCapabilityRowIsConsistent(
  value: ScreenAndroidMediaProjectionCapabilityRowInput
): boolean {
  if (!androidRawFrameClaimsAreSafe(value)) {
    return false;
  }

  if (value.mode === 'notClaimed') {
    return androidNotClaimedRowIsConsistent(value);
  }

  if (!androidConsentRequirementsArePresent(value)) {
    return false;
  }

  if (value.mode === 'emulatorMediaProjection') {
    return androidEmulatorRowIsConsistent(value);
  }

  if (!androidAppWindowSelectionIsConsistent(value)) {
    return false;
  }

  if (!value.productAndroidCaptureReady) {
    return androidManualRequiredRowIsConsistent(value);
  }

  return androidReadyRowIsConsistent(value);
}

function androidRawFrameClaimsAreSafe(value: ScreenAndroidMediaProjectionCapabilityRowInput): boolean {
  return [
    value.silentBackgroundCaptureClaimed,
    value.rawFrameRemoteUploadAllowed,
    value.rawFrameRetentionDefault,
  ].every((claimed) => claimed === false);
}

function androidNotClaimedRowIsConsistent(value: ScreenAndroidMediaProjectionCapabilityRowInput): boolean {
  return (
    value.captureState === 'notClaimed' &&
    value.proofState === 'sourceDocsVerified' &&
    value.emulatorProofRef === null &&
    value.physicalDeviceProofRef === null &&
    value.deletionProofRef === null &&
    !value.productAndroidCaptureReady
  );
}

function androidConsentRequirementsArePresent(value: ScreenAndroidMediaProjectionCapabilityRowInput): boolean {
  return [
    value.requiresUserConsentPerSession,
    value.requiresForegroundServiceType,
    value.requiresStopCallbackOnUserStop,
  ].every((required) => required === true);
}

function androidEmulatorRowIsConsistent(value: ScreenAndroidMediaProjectionCapabilityRowInput): boolean {
  return (
    value.captureState === 'provedEmulator' &&
    value.proofState === 'emulatorVerified' &&
    value.emulatorProofRef !== null &&
    value.deletionProofRef !== null &&
    value.physicalDeviceProofRef === null &&
    !value.productAndroidCaptureReady
  );
}

function androidAppWindowSelectionIsConsistent(value: ScreenAndroidMediaProjectionCapabilityRowInput): boolean {
  return value.mode !== 'android14AppWindowSharing' || value.supportsAppWindowSelection;
}

function androidManualRequiredRowIsConsistent(value: ScreenAndroidMediaProjectionCapabilityRowInput): boolean {
  return value.captureState === 'manualRequired' && value.proofState === 'physicalDeviceMissing';
}

function androidReadyRowIsConsistent(value: ScreenAndroidMediaProjectionCapabilityRowInput): boolean {
  return (
    value.captureState === 'ready' &&
    value.proofState === 'physicalDeviceVerified' &&
    value.physicalDeviceProofRef !== null &&
    value.deletionProofRef !== null
  );
}

export function screenAndroidMediaProjectionCapabilityProofIsConsistent(
  value: ScreenAndroidMediaProjectionCapabilityProofInput
): boolean {
  const readyRows = value.rows.filter(
    (row: ScreenAndroidMediaProjectionCapabilityRowInput) =>
      row.mode === 'physicalDeviceMediaProjection' || row.mode === 'android14AppWindowSharing'
  );
  const physicalRowsReady = readyRows.length > 0 && readyRows.every((row) => row.productAndroidCaptureReady);
  return value.productAndroidCaptureReady === physicalRowsReady;
}

export function screenAndroidMediaProjectionCapabilityProof(generatedAt: string) {
  return ScreenAndroidMediaProjectionCapabilityProofSchema.parse({
    schemaVersion: ScreenAndroidMediaProjectionCapabilitySchemaVersion,
    generatedAt,
    proofId: 'screen-android-mediaprojection-capability-proof',
    androidDocsVerifiedAt: generatedAt,
    emulatorCaptureProved: true,
    productAndroidCaptureReady: false,
    rows: [
      emulatorProofRow(generatedAt),
      physicalManualRequiredRow(generatedAt, 'physicalDeviceMediaProjection'),
      physicalManualRequiredRow(generatedAt, 'android14AppWindowSharing', { supportsAppWindowSelection: true }),
      notClaimedSilentBackgroundRow(generatedAt),
    ],
    nonClaims: [
      'This proof does not claim Android physical-device parity.',
      'This proof does not claim silent Android background capture.',
      'This proof does not enable raw remote upload or raw-retention-by-default.',
      'This proof defines stop-callback-on-user-stop behavior but does not claim physical-device runtime execution.',
    ],
  });
}

function emulatorProofRow(checkedAt: string) {
  return ScreenAndroidMediaProjectionCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'emulatorMediaProjection'),
    captureState: 'provedEmulator',
    proofState: 'emulatorVerified',
    emulatorProofRef: 'output/screen-plan-proof/android-mediaprojection/proof-summary.json',
    deletionProofRef: 'output/screen-plan-proof/android-mediaprojection/03-android-capture-proof.json',
    reason: ScreenAndroidMediaProjectionReasonSchema.parse(
      'Android emulator MediaProjection proof captured pixels with explicit consent and raw temp deletion.'
    ),
  });
}

function physicalManualRequiredRow(
  checkedAt: string,
  mode: Exclude<ScreenAndroidMediaProjectionMode, 'emulatorMediaProjection' | 'notClaimed'>,
  overrides: Partial<ScreenAndroidMediaProjectionCapabilityRowInput> = {}
) {
  return ScreenAndroidMediaProjectionCapabilityRowSchema.parse({
    ...baseRow(checkedAt, mode),
    ...overrides,
  });
}

function notClaimedSilentBackgroundRow(checkedAt: string) {
  return ScreenAndroidMediaProjectionCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'notClaimed'),
    captureState: 'notClaimed',
    proofState: 'sourceDocsVerified',
    requiresUserConsentPerSession: false,
    requiresForegroundServiceType: false,
    requiresStopCallbackOnUserStop: false,
    reason: ScreenAndroidMediaProjectionReasonSchema.parse(
      'Silent Android background screen capture is not a MediaProjection product claim and remains blocked before proof.'
    ),
  });
}

function baseRow(checkedAt: string, mode: ScreenAndroidMediaProjectionMode) {
  return {
    schemaVersion: ScreenAndroidMediaProjectionCapabilitySchemaVersion,
    checkedAt,
    mode,
    captureState: 'manualRequired',
    proofState: 'physicalDeviceMissing',
    androidDocRefs: [
      ScreenAndroidMediaProjectionDocRefSchema.parse('android-developer-media-projection'),
      ScreenAndroidMediaProjectionDocRefSchema.parse('android-developer-app-screen-sharing'),
    ],
    requiresUserConsentPerSession: true,
    requiresForegroundServiceType: true,
    requiresStopCallbackOnUserStop: true,
    supportsAppWindowSelection: false,
    silentBackgroundCaptureClaimed: false,
    rawFrameRemoteUploadAllowed: false,
    rawFrameRetentionDefault: false,
    emulatorProofRef: null,
    physicalDeviceProofRef: null,
    deletionProofRef: null,
    productAndroidCaptureReady: false,
    reason: ScreenAndroidMediaProjectionReasonSchema.parse(
      'Android MediaProjection physical-device capture requires explicit proof before product readiness.'
    ),
  };
}

export type ScreenAndroidMediaProjectionMode = Infer<typeof ScreenAndroidMediaProjectionModeSchema>;
export type ScreenAndroidMediaProjectionCaptureState = Infer<typeof ScreenAndroidMediaProjectionCaptureStateSchema>;
export type ScreenAndroidMediaProjectionProofState = Infer<typeof ScreenAndroidMediaProjectionProofStateSchema>;
export type ScreenAndroidMediaProjectionCapabilityRow = Infer<typeof ScreenAndroidMediaProjectionCapabilityRowSchema>;
export type ScreenAndroidMediaProjectionCapabilityProof = Infer<
  typeof ScreenAndroidMediaProjectionCapabilityProofSchema
>;
