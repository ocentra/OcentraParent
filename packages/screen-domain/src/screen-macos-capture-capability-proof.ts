import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { ScreenEvidenceReasonSchema } from './screen-evidence-primitives';
import { ScreenOptionalVisibilityPlatformProofRefSchema } from './screen-optional-visibility-mode-values';

export const ScreenMacosCaptureCapabilitySchemaVersion = 1;

const ScreenMacosCaptureDocRefSchema = withParser(
  Schema.String.pipe(Schema.minLength(1), Schema.brand('ScreenMacosCaptureDocRef'))
);
const ScreenMacosCaptureReasonSchema = withParser(ScreenEvidenceReasonSchema);
const OptionalScreenMacosCaptureProofRefSchema = Schema.Union(
  ScreenOptionalVisibilityPlatformProofRefSchema,
  Schema.Null
);
const RequiredFalse = Schema.Literal(false);

export const ScreenMacosCaptureModeSchema = withParser(
  Schema.Literal('screenCaptureKitDisplay', 'screenCaptureKitWindow', 'screenRecordingPermission', 'pppcMdmManaged')
);

export const ScreenMacosCaptureStateSchema = withParser(Schema.Literal('manualRequired', 'ready'));

export const ScreenMacosCaptureProofStateSchema = withParser(
  Schema.Literal('sourceDocsVerified', 'permissionProofMissing', 'liveSessionVerified')
);

const ScreenMacosCaptureCapabilityRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenMacosCaptureCapabilitySchemaVersion),
  checkedAt: ActivityTimestampSchema,
  mode: ScreenMacosCaptureModeSchema,
  captureState: ScreenMacosCaptureStateSchema,
  proofState: ScreenMacosCaptureProofStateSchema,
  appleDocRefs: Schema.NonEmptyArray(ScreenMacosCaptureDocRefSchema),
  requiresScreenRecordingPermission: Schema.Boolean,
  requiresUserVisibleCaptureIndicator: Schema.Boolean,
  requiresScreenCaptureKitContentFilter: Schema.Boolean,
  requiresPppcMdmReview: Schema.Boolean,
  silentBackgroundCaptureClaimed: RequiredFalse,
  rawFrameRemoteUploadAllowed: RequiredFalse,
  rawFrameRetentionDefault: RequiredFalse,
  liveSessionProofRef: OptionalScreenMacosCaptureProofRefSchema,
  permissionProofRef: OptionalScreenMacosCaptureProofRefSchema,
  deletionProofRef: OptionalScreenMacosCaptureProofRefSchema,
  productMacosCaptureReady: Schema.Boolean,
  reason: ScreenEvidenceReasonSchema,
});

type ScreenMacosCaptureCapabilityRowInput = Infer<typeof ScreenMacosCaptureCapabilityRowBaseSchema>;

export const ScreenMacosCaptureCapabilityRowSchema = withParser(
  ScreenMacosCaptureCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenMacosCaptureCapabilityRowIsConsistent(value) ||
        'Expected macOS capture readiness to require ScreenCaptureKit scope, Screen Recording permission, live macOS pixels, deletion proof, no silent background capture, no raw remote upload, and no raw-retention default'
    )
  )
);

const ScreenMacosCaptureCapabilityProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenMacosCaptureCapabilitySchemaVersion),
  generatedAt: ActivityTimestampSchema,
  proofId: Schema.Literal('screen-macos-capture-capability-proof'),
  rows: Schema.NonEmptyArray(ScreenMacosCaptureCapabilityRowSchema),
  appleDocsVerifiedAt: ActivityTimestampSchema,
  productMacosCaptureReady: Schema.Boolean,
  nonClaims: Schema.NonEmptyArray(ScreenEvidenceReasonSchema),
});

type ScreenMacosCaptureCapabilityProofInput = Infer<typeof ScreenMacosCaptureCapabilityProofBaseSchema>;

export const ScreenMacosCaptureCapabilityProofSchema = withParser(
  ScreenMacosCaptureCapabilityProofBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenMacosCaptureCapabilityProofIsConsistent(value) ||
        'Expected macOS product readiness to stay false until every live capture row has permission, session, and deletion proof'
    )
  )
);

export function screenMacosCaptureCapabilityRowIsConsistent(value: ScreenMacosCaptureCapabilityRowInput): boolean {
  if (!macosRawFrameClaimsAreSafe(value)) {
    return false;
  }

  if (!macosPppcRequirementsAreConsistent(value)) {
    return false;
  }

  if (!macosScreenCaptureKitRequirementsAreConsistent(value)) {
    return false;
  }

  if (!value.productMacosCaptureReady) {
    return macosManualRequiredRowIsConsistent(value);
  }

  return macosReadyRowIsConsistent(value);
}

function macosRawFrameClaimsAreSafe(value: ScreenMacosCaptureCapabilityRowInput): boolean {
  return [
    value.silentBackgroundCaptureClaimed,
    value.rawFrameRemoteUploadAllowed,
    value.rawFrameRetentionDefault,
  ].every((claimed) => claimed === false);
}

function macosPppcRequirementsAreConsistent(value: ScreenMacosCaptureCapabilityRowInput): boolean {
  if (value.mode === 'pppcMdmManaged') {
    return value.requiresPppcMdmReview;
  }

  return !value.requiresPppcMdmReview;
}

function macosScreenCaptureKitRequirementsAreConsistent(value: ScreenMacosCaptureCapabilityRowInput): boolean {
  if (value.mode !== 'screenCaptureKitDisplay' && value.mode !== 'screenCaptureKitWindow') {
    return true;
  }

  return value.requiresScreenRecordingPermission && value.requiresScreenCaptureKitContentFilter;
}

function macosManualRequiredRowIsConsistent(value: ScreenMacosCaptureCapabilityRowInput): boolean {
  return (
    value.captureState === 'manualRequired' &&
    value.proofState !== 'liveSessionVerified' &&
    value.liveSessionProofRef === null &&
    value.deletionProofRef === null
  );
}

function macosReadyRowIsConsistent(value: ScreenMacosCaptureCapabilityRowInput): boolean {
  return (
    value.captureState === 'ready' &&
    value.proofState === 'liveSessionVerified' &&
    value.liveSessionProofRef !== null &&
    value.permissionProofRef !== null &&
    value.deletionProofRef !== null
  );
}

export function screenMacosCaptureCapabilityProofIsConsistent(value: ScreenMacosCaptureCapabilityProofInput): boolean {
  const liveRows = value.rows.filter(
    (row: ScreenMacosCaptureCapabilityRowInput) =>
      row.mode === 'screenCaptureKitDisplay' || row.mode === 'screenCaptureKitWindow'
  );
  const liveRowsReady = liveRows.length > 0 && liveRows.every((row) => row.productMacosCaptureReady);
  return value.productMacosCaptureReady === liveRowsReady;
}

export function screenMacosCaptureCapabilityProof(generatedAt: string) {
  return ScreenMacosCaptureCapabilityProofSchema.parse({
    schemaVersion: ScreenMacosCaptureCapabilitySchemaVersion,
    generatedAt,
    proofId: 'screen-macos-capture-capability-proof',
    appleDocsVerifiedAt: generatedAt,
    productMacosCaptureReady: false,
    rows: [
      screenCaptureKitManualRequiredRow(generatedAt, 'screenCaptureKitDisplay'),
      screenCaptureKitManualRequiredRow(generatedAt, 'screenCaptureKitWindow'),
      permissionManualRequiredRow(generatedAt),
      pppcMdmManualRequiredRow(generatedAt),
    ],
    nonClaims: [
      'This proof does not claim live macOS ScreenCaptureKit execution.',
      'This proof does not claim Screen Recording permission has been granted on a real Mac.',
      'This proof does not enable silent background capture, raw remote upload, or raw-retention-by-default.',
    ],
  });
}

function screenCaptureKitManualRequiredRow(
  checkedAt: string,
  mode: Extract<ScreenMacosCaptureMode, 'screenCaptureKitDisplay' | 'screenCaptureKitWindow'>
) {
  return ScreenMacosCaptureCapabilityRowSchema.parse({
    ...baseRow(checkedAt, mode),
    requiresScreenCaptureKitContentFilter: true,
    reason: ScreenMacosCaptureReasonSchema.parse(
      'macOS ScreenCaptureKit display/window capture requires live macOS session proof, Screen Recording permission proof, and deletion proof before product readiness.'
    ),
  });
}

function permissionManualRequiredRow(checkedAt: string) {
  return ScreenMacosCaptureCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'screenRecordingPermission'),
    proofState: 'permissionProofMissing',
    reason: ScreenMacosCaptureReasonSchema.parse(
      'macOS Screen Recording permission remains manual-required until a real Mac permission artifact is recorded.'
    ),
  });
}

function pppcMdmManualRequiredRow(checkedAt: string) {
  return ScreenMacosCaptureCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'pppcMdmManaged'),
    requiresScreenRecordingPermission: false,
    requiresUserVisibleCaptureIndicator: false,
    requiresPppcMdmReview: true,
    proofState: 'permissionProofMissing',
    reason: ScreenMacosCaptureReasonSchema.parse(
      'Managed macOS PPPC/MDM screen-recording authorization remains a manual enterprise deployment gate.'
    ),
  });
}

function baseRow(checkedAt: string, mode: ScreenMacosCaptureMode) {
  return {
    schemaVersion: ScreenMacosCaptureCapabilitySchemaVersion,
    checkedAt,
    mode,
    captureState: 'manualRequired',
    proofState: 'sourceDocsVerified',
    appleDocRefs: [
      ScreenMacosCaptureDocRefSchema.parse('apple-developer-screencapturekit'),
      ScreenMacosCaptureDocRefSchema.parse('apple-developer-screencapturekit-content-filter'),
      ScreenMacosCaptureDocRefSchema.parse('apple-support-screen-recording-privacy'),
    ],
    requiresScreenRecordingPermission: true,
    requiresUserVisibleCaptureIndicator: true,
    requiresScreenCaptureKitContentFilter: false,
    requiresPppcMdmReview: false,
    silentBackgroundCaptureClaimed: false,
    rawFrameRemoteUploadAllowed: false,
    rawFrameRetentionDefault: false,
    liveSessionProofRef: null,
    permissionProofRef: null,
    deletionProofRef: null,
    productMacosCaptureReady: false,
    reason: ScreenMacosCaptureReasonSchema.parse(
      'macOS capture readiness requires live macOS proof before product readiness.'
    ),
  };
}

export type ScreenMacosCaptureMode = Infer<typeof ScreenMacosCaptureModeSchema>;
export type ScreenMacosCaptureState = Infer<typeof ScreenMacosCaptureStateSchema>;
export type ScreenMacosCaptureProofState = Infer<typeof ScreenMacosCaptureProofStateSchema>;
export type ScreenMacosCaptureCapabilityRow = Infer<typeof ScreenMacosCaptureCapabilityRowSchema>;
export type ScreenMacosCaptureCapabilityProof = Infer<typeof ScreenMacosCaptureCapabilityProofSchema>;
