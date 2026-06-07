import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import { ScreenEvidenceCustodyStateSchema } from './screen-evidence-states';
import {
  ScreenLiveViewModeSchema,
  ScreenLiveViewTransportModeSchema,
  ScreenOptionalVisibilityAuditRefSchema,
  ScreenOptionalVisibilityPlatformProofRefSchema,
  ScreenOptionalVisibilityPlatformProofStateSchema,
  ScreenOptionalVisibilityRetentionBehaviorSchema,
  ScreenOptionalVisibilitySourceLabelSchema,
} from './screen-optional-visibility-mode-values';

export const ScreenLiveViewPermissionGateSchemaVersion = 1;

const NonEmptyLiveViewText = Schema.String.pipe(Schema.minLength(1));
const RequiredFalse = Schema.Literal(false);
const OptionalLiveViewProofRefSchema = Schema.Union(ScreenOptionalVisibilityPlatformProofRefSchema, Schema.Null);
const OptionalLiveViewAuditRefSchema = Schema.Union(ScreenOptionalVisibilityAuditRefSchema, Schema.Null);

export const ScreenLiveViewPlatformKindSchema = withParser(
  Schema.Literal('windows', 'macos', 'linux-x11', 'linux-wayland', 'android-mediaprojection', 'ios-replaykit')
);

export const ScreenLiveViewPermissionEvidenceKindSchema = withParser(
  Schema.Literal('live-view-permission', 'screen-capture-only', 'missing')
);

const ScreenLiveViewPlatformPermissionGateBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLiveViewPermissionGateSchemaVersion),
  checkedAt: ActivityTimestampSchema,
  platform: ScreenLiveViewPlatformKindSchema,
  liveViewMode: ScreenLiveViewModeSchema,
  transportMode: ScreenLiveViewTransportModeSchema,
  permissionEvidenceKind: ScreenLiveViewPermissionEvidenceKindSchema,
  platformProofState: ScreenOptionalVisibilityPlatformProofStateSchema,
  platformProofRef: OptionalLiveViewProofRefSchema,
  viewerAuditRef: OptionalLiveViewAuditRefSchema,
  sourceLabel: ScreenOptionalVisibilitySourceLabelSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  frameRetentionBehavior: ScreenOptionalVisibilityRetentionBehaviorSchema,
  liveTransportProofRef: OptionalLiveViewProofRefSchema,
  explicitViewerDisclosure: Schema.Boolean,
  cacheRawFrames: RequiredFalse,
  sessionRecordingAllowed: RequiredFalse,
  remoteInputControlAllowed: RequiredFalse,
  productLiveViewReady: Schema.Boolean,
  reason: NonEmptyLiveViewText,
});

type ScreenLiveViewPlatformPermissionGateInput = Infer<typeof ScreenLiveViewPlatformPermissionGateBaseSchema>;

export const ScreenLiveViewPlatformPermissionGateSchema = withParser(
  ScreenLiveViewPlatformPermissionGateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenLiveViewPermissionGateIsConsistent(value) ||
        'Expected live view permission gate to require live-view permission evidence, transport proof, viewer audit, no frame retention, and no remote input before readiness'
    )
  )
);

export function screenLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  if (value.liveViewMode === 'disabled') {
    return disabledLiveViewPermissionGateIsConsistent(value);
  }

  return enabledLiveViewPermissionGateIsConsistent(value);
}

function disabledLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  return (
    value.transportMode === 'none' &&
    value.permissionEvidenceKind === 'missing' &&
    value.platformProofState === 'notRequired' &&
    value.platformProofRef === null &&
    value.viewerAuditRef === null &&
    value.sourceLabel === 'unavailable' &&
    value.custodyState === 'unavailable' &&
    value.frameRetentionBehavior === 'noFrameRetention' &&
    value.liveTransportProofRef === null &&
    !value.explicitViewerDisclosure &&
    !value.productLiveViewReady
  );
}

function enabledLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  if (
    value.permissionEvidenceKind !== 'live-view-permission' ||
    value.platformProofState !== 'operatorVerified' ||
    value.platformProofRef === null ||
    value.viewerAuditRef === null ||
    value.frameRetentionBehavior !== 'noFrameRetention' ||
    value.liveTransportProofRef === null ||
    !value.explicitViewerDisclosure
  ) {
    return !value.productLiveViewReady;
  }

  if (value.liveViewMode === 'lanOnlyView') {
    return (
      value.transportMode === 'lanMutualAuth' &&
      value.sourceLabel === 'liveView' &&
      value.custodyState === 'live-lan-child-agent' &&
      value.productLiveViewReady
    );
  }

  return (
    value.transportMode === 'relayEndToEndEncrypted' &&
    value.sourceLabel === 'relay' &&
    value.custodyState === 'ocentra-hosted-non-activity' &&
    value.productLiveViewReady
  );
}

export type ScreenLiveViewPlatformKind = Infer<typeof ScreenLiveViewPlatformKindSchema>;
export type ScreenLiveViewPermissionEvidenceKind = Infer<typeof ScreenLiveViewPermissionEvidenceKindSchema>;
export type ScreenLiveViewPlatformPermissionGate = Infer<typeof ScreenLiveViewPlatformPermissionGateSchema>;
