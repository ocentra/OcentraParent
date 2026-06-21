import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { ScreenEvidenceParentSettingRefSchema, ScreenEvidenceReasonSchema } from './screen-evidence-primitives';
import {
  ScreenLiveViewPlatformPermissionGateSchema,
  type ScreenLiveViewPlatformPermissionGate,
} from './screen-live-view-platform-permission';
import {
  ScreenLiveViewOptInSettingSchema,
  ScreenRawScreenshotRetentionOptInSettingSchema,
} from './screen-optional-visibility-mode';
import { ScreenOptionalVisibilityPlatformProofRefSchema } from './screen-optional-visibility-mode-values';

export const ScreenOptionalVisibilityCapabilityStatusSchemaVersion = 1;

const OptionalRuntimeProofRefSchema = Schema.Union(ScreenOptionalVisibilityPlatformProofRefSchema, Schema.Null);
const OptionalRawRetentionSettingSchema = Schema.Union(ScreenRawScreenshotRetentionOptInSettingSchema, Schema.Null);
const OptionalLiveViewSettingSchema = Schema.Union(ScreenLiveViewOptInSettingSchema, Schema.Null);
const OptionalLiveViewPermissionGateSchema = Schema.Union(ScreenLiveViewPlatformPermissionGateSchema, Schema.Null);

export const ScreenOptionalVisibilityCapabilityKindSchema = withParser(
  Schema.Literal('rawScreenshotRetention', 'liveView')
);
export const ScreenOptionalVisibilityReadinessStateSchema = withParser(
  Schema.Literal('disabled', 'manualRequired', 'blocked', 'ready')
);

const ScreenOptionalVisibilityCapabilityStatusBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenOptionalVisibilityCapabilityStatusSchemaVersion),
  checkedAt: ActivityTimestampSchema,
  capabilityKind: ScreenOptionalVisibilityCapabilityKindSchema,
  parentSettingRef: ScreenEvidenceParentSettingRefSchema,
  readinessState: ScreenOptionalVisibilityReadinessStateSchema,
  rawRetentionSetting: OptionalRawRetentionSettingSchema,
  liveViewSetting: OptionalLiveViewSettingSchema,
  liveViewPermissionGate: OptionalLiveViewPermissionGateSchema,
  runtimeProofRef: OptionalRuntimeProofRefSchema,
  deletionProofRef: OptionalRuntimeProofRefSchema,
  transportProofRef: OptionalRuntimeProofRefSchema,
  childDisclosureReady: Schema.Boolean,
  childDeviceCapabilityReady: Schema.Boolean,
  productModeReady: Schema.Boolean,
  rawFramesRetained: Schema.Literal(false),
  rawRemoteUploadAllowed: Schema.Literal(false),
  remoteInputAllowed: Schema.Literal(false),
  reason: ScreenEvidenceReasonSchema,
});

type ScreenOptionalVisibilityCapabilityStatusBase = Infer<typeof ScreenOptionalVisibilityCapabilityStatusBaseSchema>;

export const ScreenOptionalVisibilityCapabilityStatusSchema = withParser(
  ScreenOptionalVisibilityCapabilityStatusBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenOptionalVisibilityCapabilityStatusIsConsistent(value) ||
        'Expected optional screen visibility capability status to match parent opt-in, child capability, runtime proof, deletion proof, and live-view permission proof before readiness'
    )
  )
);

function screenOptionalVisibilityCapabilityStatusIsConsistent(
  value: ScreenOptionalVisibilityCapabilityStatusBase
): boolean {
  if (!statusKeepsRawCustody(value)) {
    return false;
  }
  return value.capabilityKind === 'rawScreenshotRetention'
    ? rawRetentionCapabilityStatusIsConsistent(value)
    : liveViewCapabilityStatusIsConsistent(value);
}

function rawRetentionCapabilityStatusIsConsistent(value: ScreenOptionalVisibilityCapabilityStatusBase): boolean {
  if (value.rawRetentionSetting === null || value.liveViewSetting !== null || value.liveViewPermissionGate !== null) {
    return false;
  }
  if (value.rawRetentionSetting.mode === 'disabled') {
    return disabledCapabilityStatusIsConsistent(value);
  }
  const hasRuntimeProof = value.runtimeProofRef !== null && value.deletionProofRef !== null;
  return (
    value.readinessState === (hasRuntimeProof ? 'ready' : 'manualRequired') &&
    readinessBooleansMatch(value, hasRuntimeProof)
  );
}

function liveViewCapabilityStatusIsConsistent(value: ScreenOptionalVisibilityCapabilityStatusBase): boolean {
  if (value.liveViewSetting === null || value.rawRetentionSetting !== null || value.deletionProofRef !== null) {
    return false;
  }
  if (value.liveViewSetting.liveViewMode === 'disabled') {
    return disabledCapabilityStatusIsConsistent(value);
  }
  if (value.liveViewPermissionGate === null) {
    return value.readinessState === 'blocked' && readinessBooleansMatch(value, false);
  }
  const ready = liveViewGateAllowsProductMode(value.liveViewPermissionGate);
  return (
    value.readinessState === (ready ? 'ready' : 'blocked') &&
    value.transportProofRef === value.liveViewPermissionGate.liveTransportProofRef &&
    readinessBooleansMatch(value, ready)
  );
}

function disabledCapabilityStatusIsConsistent(value: ScreenOptionalVisibilityCapabilityStatusBase): boolean {
  return (
    value.readinessState === 'disabled' &&
    value.runtimeProofRef === null &&
    value.deletionProofRef === null &&
    value.transportProofRef === null &&
    !value.childDisclosureReady &&
    !value.childDeviceCapabilityReady &&
    !value.productModeReady
  );
}

function readinessBooleansMatch(value: ScreenOptionalVisibilityCapabilityStatusBase, ready: boolean): boolean {
  return (
    value.childDisclosureReady === ready &&
    value.childDeviceCapabilityReady === ready &&
    value.productModeReady === ready
  );
}

function statusKeepsRawCustody(value: ScreenOptionalVisibilityCapabilityStatusBase): boolean {
  return !value.rawFramesRetained && !value.rawRemoteUploadAllowed && !value.remoteInputAllowed;
}

function liveViewGateAllowsProductMode(value: ScreenLiveViewPlatformPermissionGate): boolean {
  return value.productLiveViewReady && value.liveTransportProofRef !== null;
}

export type ScreenOptionalVisibilityCapabilityKind = Infer<typeof ScreenOptionalVisibilityCapabilityKindSchema>;
export type ScreenOptionalVisibilityReadinessState = Infer<typeof ScreenOptionalVisibilityReadinessStateSchema>;
export type ScreenOptionalVisibilityCapabilityStatus = Infer<typeof ScreenOptionalVisibilityCapabilityStatusSchema>;
