import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { ScreenEvidenceReasonSchema } from './screen-evidence-primitives';
import { ScreenOptionalVisibilityPlatformProofRefSchema } from './screen-optional-visibility-mode-values';

export const ScreenLinuxCaptureCapabilitySchemaVersion = 1;

const ScreenLinuxCaptureDocRefSchema = withParser(
  Schema.String.pipe(Schema.minLength(1), Schema.brand('ScreenLinuxCaptureDocRef'))
);
const ScreenLinuxCaptureReasonSchema = withParser(ScreenEvidenceReasonSchema);
const OptionalScreenLinuxCaptureProofRefSchema = Schema.Union(
  ScreenOptionalVisibilityPlatformProofRefSchema,
  Schema.Null
);
const RequiredFalse = Schema.Literal(false);

export const ScreenLinuxCaptureModeSchema = withParser(
  Schema.Literal(
    'wslgX11SelectedWindow',
    'nativeX11SelectedWindow',
    'nativeX11RootDisplay',
    'waylandPortalPipeWire',
    'unsupportedCompositor'
  )
);

export const ScreenLinuxCaptureStateSchema = withParser(
  Schema.Literal('provedWslgSelectedWindow', 'manualRequired', 'ready', 'unsupported')
);

export const ScreenLinuxCaptureProofStateSchema = withParser(
  Schema.Literal('x11WslgVerified', 'sourceDocsVerified', 'nativeSessionMissing', 'nativeSessionVerified')
);

export const ScreenLinuxCompositorSchema = withParser(
  Schema.Literal(
    'wslgX11',
    'nativeX11',
    'gnomeWayland',
    'kdeWayland',
    'wlrootsWayland',
    'unknownWayland',
    'unsupported'
  )
);

const ScreenLinuxCaptureCapabilityRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLinuxCaptureCapabilitySchemaVersion),
  checkedAt: ActivityTimestampSchema,
  mode: ScreenLinuxCaptureModeSchema,
  captureState: ScreenLinuxCaptureStateSchema,
  proofState: ScreenLinuxCaptureProofStateSchema,
  compositor: ScreenLinuxCompositorSchema,
  linuxDocRefs: Schema.NonEmptyArray(ScreenLinuxCaptureDocRefSchema),
  x11CommandBackendRequired: Schema.Boolean,
  waylandPortalRequired: Schema.Boolean,
  pipeWireRequired: Schema.Boolean,
  userMediatedSelectionRequired: Schema.Boolean,
  rootDisplayClaimed: RequiredFalse,
  rawFrameRemoteUploadAllowed: RequiredFalse,
  rawFrameRetentionDefault: RequiredFalse,
  wslgProofRef: OptionalScreenLinuxCaptureProofRefSchema,
  nativeSessionProofRef: OptionalScreenLinuxCaptureProofRefSchema,
  deletionProofRef: OptionalScreenLinuxCaptureProofRefSchema,
  productLinuxCaptureReady: Schema.Boolean,
  reason: ScreenEvidenceReasonSchema,
});

type ScreenLinuxCaptureCapabilityRowInput = Infer<typeof ScreenLinuxCaptureCapabilityRowBaseSchema>;

export const ScreenLinuxCaptureCapabilityRowSchema = withParser(
  ScreenLinuxCaptureCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenLinuxCaptureCapabilityRowIsConsistent(value) ||
        'Expected Linux screen capture readiness to separate WSLg/X11 proof from native X11 root-display and Wayland/PipeWire product readiness'
    )
  )
);

const ScreenLinuxCaptureCapabilityProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLinuxCaptureCapabilitySchemaVersion),
  generatedAt: ActivityTimestampSchema,
  proofId: Schema.Literal('screen-linux-capture-capability-proof'),
  rows: Schema.NonEmptyArray(ScreenLinuxCaptureCapabilityRowSchema),
  linuxDocsVerifiedAt: ActivityTimestampSchema,
  wslgSelectedWindowCaptureProved: Schema.Boolean,
  productLinuxCaptureReady: Schema.Boolean,
  nonClaims: Schema.NonEmptyArray(ScreenEvidenceReasonSchema),
});

type ScreenLinuxCaptureCapabilityProofInput = Infer<typeof ScreenLinuxCaptureCapabilityProofBaseSchema>;

export const ScreenLinuxCaptureCapabilityProofSchema = withParser(
  ScreenLinuxCaptureCapabilityProofBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenLinuxCaptureCapabilityProofIsConsistent(value) ||
        'Expected Linux product readiness to stay false until native Linux rows have native session and deletion proof'
    )
  )
);

export function screenLinuxCaptureCapabilityRowIsConsistent(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  if (!linuxRawFrameClaimsAreSafe(value)) {
    return false;
  }

  const modeConsistency = linuxModeSpecificConsistency(value);
  if (modeConsistency !== null) {
    return modeConsistency;
  }

  if (!value.productLinuxCaptureReady) {
    return linuxManualRequiredRowIsConsistent(value);
  }

  return linuxReadyRowIsConsistent(value);
}

function linuxRawFrameClaimsAreSafe(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  return [value.rootDisplayClaimed, value.rawFrameRemoteUploadAllowed, value.rawFrameRetentionDefault].every(
    (claimed) => claimed === false
  );
}

function linuxModeSpecificConsistency(value: ScreenLinuxCaptureCapabilityRowInput): boolean | null {
  if (value.mode === 'wslgX11SelectedWindow') {
    return linuxWslgRowIsConsistent(value);
  }

  if (value.mode === 'unsupportedCompositor') {
    return linuxUnsupportedRowIsConsistent(value);
  }

  if (value.mode === 'waylandPortalPipeWire') {
    return linuxWaylandRequirementsArePresent(value) ? null : false;
  }

  if (value.mode === 'nativeX11SelectedWindow' || value.mode === 'nativeX11RootDisplay') {
    return linuxNativeX11RequirementsArePresent(value) ? null : false;
  }

  return null;
}

function linuxWslgRowIsConsistent(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  return (
    value.captureState === 'provedWslgSelectedWindow' &&
    value.proofState === 'x11WslgVerified' &&
    value.compositor === 'wslgX11' &&
    value.x11CommandBackendRequired &&
    !value.waylandPortalRequired &&
    !value.pipeWireRequired &&
    value.wslgProofRef !== null &&
    value.deletionProofRef !== null &&
    value.nativeSessionProofRef === null &&
    !value.productLinuxCaptureReady
  );
}

function linuxUnsupportedRowIsConsistent(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  return (
    value.captureState === 'unsupported' &&
    value.proofState === 'sourceDocsVerified' &&
    value.compositor === 'unsupported' &&
    value.wslgProofRef === null &&
    value.nativeSessionProofRef === null &&
    value.deletionProofRef === null &&
    !value.productLinuxCaptureReady
  );
}

function linuxWaylandRequirementsArePresent(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  return [value.waylandPortalRequired, value.pipeWireRequired, value.userMediatedSelectionRequired].every(
    (required) => required === true
  );
}

function linuxNativeX11RequirementsArePresent(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  return value.x11CommandBackendRequired && !value.waylandPortalRequired && !value.pipeWireRequired;
}

function linuxManualRequiredRowIsConsistent(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  return (
    value.captureState === 'manualRequired' &&
    value.proofState === 'nativeSessionMissing' &&
    value.nativeSessionProofRef === null
  );
}

function linuxReadyRowIsConsistent(value: ScreenLinuxCaptureCapabilityRowInput): boolean {
  return (
    value.captureState === 'ready' &&
    value.proofState === 'nativeSessionVerified' &&
    value.nativeSessionProofRef !== null &&
    value.deletionProofRef !== null
  );
}

export function screenLinuxCaptureCapabilityProofIsConsistent(value: ScreenLinuxCaptureCapabilityProofInput): boolean {
  const nativeRows = value.rows.filter(
    (row: ScreenLinuxCaptureCapabilityRowInput) =>
      row.mode === 'nativeX11SelectedWindow' ||
      row.mode === 'nativeX11RootDisplay' ||
      row.mode === 'waylandPortalPipeWire'
  );
  const nativeRowsReady = nativeRows.length > 0 && nativeRows.every((row) => row.productLinuxCaptureReady);
  return value.productLinuxCaptureReady === nativeRowsReady;
}

export function screenLinuxCaptureCapabilityProof(generatedAt: string) {
  return ScreenLinuxCaptureCapabilityProofSchema.parse({
    schemaVersion: ScreenLinuxCaptureCapabilitySchemaVersion,
    generatedAt,
    proofId: 'screen-linux-capture-capability-proof',
    linuxDocsVerifiedAt: generatedAt,
    wslgSelectedWindowCaptureProved: true,
    productLinuxCaptureReady: false,
    rows: [
      wslgSelectedWindowRow(generatedAt),
      nativeX11ManualRequiredRow(generatedAt, 'nativeX11SelectedWindow'),
      nativeX11ManualRequiredRow(generatedAt, 'nativeX11RootDisplay'),
      waylandManualRequiredRow(generatedAt, 'gnomeWayland'),
      waylandManualRequiredRow(generatedAt, 'kdeWayland'),
      waylandManualRequiredRow(generatedAt, 'wlrootsWayland'),
      unsupportedCompositorRow(generatedAt),
    ],
    nonClaims: [
      'This proof does not claim native Linux root-display capture.',
      'This proof does not claim native Wayland/PipeWire portal capture execution.',
      'This proof does not enable raw remote upload or raw-retention-by-default.',
    ],
  });
}

function wslgSelectedWindowRow(checkedAt: string) {
  return ScreenLinuxCaptureCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'wslgX11SelectedWindow', 'wslgX11'),
    captureState: 'provedWslgSelectedWindow',
    proofState: 'x11WslgVerified',
    x11CommandBackendRequired: true,
    wslgProofRef: 'output/screen-plan-proof/linux-wslg/proof-summary.json',
    deletionProofRef: 'output/screen-plan-proof/linux-wslg/proof-summary.json',
    reason: ScreenLinuxCaptureReasonSchema.parse(
      'Existing WSLg/X11 selected-window proof captured pixels and deleted raw image custody artifacts.'
    ),
  });
}

function nativeX11ManualRequiredRow(
  checkedAt: string,
  mode: Extract<ScreenLinuxCaptureMode, 'nativeX11SelectedWindow' | 'nativeX11RootDisplay'>
) {
  return ScreenLinuxCaptureCapabilityRowSchema.parse({
    ...baseRow(checkedAt, mode, 'nativeX11'),
    x11CommandBackendRequired: true,
    reason: ScreenLinuxCaptureReasonSchema.parse(
      'Native Linux X11 capture requires a native-session proof artifact before product readiness.'
    ),
  });
}

function waylandManualRequiredRow(
  checkedAt: string,
  compositor: Extract<ScreenLinuxCompositor, 'gnomeWayland' | 'kdeWayland' | 'wlrootsWayland'>
) {
  return ScreenLinuxCaptureCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'waylandPortalPipeWire', compositor),
    waylandPortalRequired: true,
    pipeWireRequired: true,
    userMediatedSelectionRequired: true,
    reason: ScreenLinuxCaptureReasonSchema.parse(
      'Native Wayland capture requires XDG Desktop Portal ScreenCast/PipeWire session proof before product readiness.'
    ),
  });
}

function unsupportedCompositorRow(checkedAt: string) {
  return ScreenLinuxCaptureCapabilityRowSchema.parse({
    ...baseRow(checkedAt, 'unsupportedCompositor', 'unsupported'),
    captureState: 'unsupported',
    proofState: 'sourceDocsVerified',
    reason: ScreenLinuxCaptureReasonSchema.parse(
      'Unsupported or unknown Linux compositor states stay manual-required instead of being treated as capture-ready.'
    ),
  });
}

function baseRow(checkedAt: string, mode: ScreenLinuxCaptureMode, compositor: ScreenLinuxCompositor) {
  return {
    schemaVersion: ScreenLinuxCaptureCapabilitySchemaVersion,
    checkedAt,
    mode,
    captureState: 'manualRequired',
    proofState: 'nativeSessionMissing',
    compositor,
    linuxDocRefs: [
      ScreenLinuxCaptureDocRefSchema.parse('xdg-desktop-portal-screencast'),
      ScreenLinuxCaptureDocRefSchema.parse('imagemagick-import-x-server'),
    ],
    x11CommandBackendRequired: false,
    waylandPortalRequired: false,
    pipeWireRequired: false,
    userMediatedSelectionRequired: false,
    rootDisplayClaimed: false,
    rawFrameRemoteUploadAllowed: false,
    rawFrameRetentionDefault: false,
    wslgProofRef: null,
    nativeSessionProofRef: null,
    deletionProofRef: null,
    productLinuxCaptureReady: false,
    reason: ScreenLinuxCaptureReasonSchema.parse(
      'Linux capture readiness requires native proof before product readiness.'
    ),
  };
}

export type ScreenLinuxCaptureMode = Infer<typeof ScreenLinuxCaptureModeSchema>;
export type ScreenLinuxCaptureState = Infer<typeof ScreenLinuxCaptureStateSchema>;
export type ScreenLinuxCaptureProofState = Infer<typeof ScreenLinuxCaptureProofStateSchema>;
export type ScreenLinuxCompositor = Infer<typeof ScreenLinuxCompositorSchema>;
export type ScreenLinuxCaptureCapabilityRow = Infer<typeof ScreenLinuxCaptureCapabilityRowSchema>;
export type ScreenLinuxCaptureCapabilityProof = Infer<typeof ScreenLinuxCaptureCapabilityProofSchema>;
