import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  ScreenOptionalVisibilityAuditRefSchema,
  ScreenOptionalVisibilityPlatformProofRefSchema,
} from './screen-optional-visibility-mode-values';

export const ScreenLiveViewRequiredFalseSchema = Schema.Literal(false);
export const ScreenLiveViewRequiredTrueSchema = Schema.Literal(true);
export const ScreenLiveViewOptionalProofRefSchema = Schema.Union(
  ScreenOptionalVisibilityPlatformProofRefSchema,
  Schema.Null
);
export const ScreenLiveViewOptionalAuditRefSchema = Schema.Union(
  ScreenOptionalVisibilityAuditRefSchema,
  Schema.Null
);

type ScreenLiveViewViewOnlySafetyShape = {
  readonly frameRetentionBehavior: string;
  readonly cacheRawFrames: boolean;
  readonly sessionRecordingAllowed: boolean;
  readonly remoteInputControlAllowed: boolean;
};

type ScreenLiveViewProofRefsShape = {
  readonly viewerAuditRef: unknown;
  readonly liveTransportProofRef: unknown;
};

type ScreenLiveViewDisabledCoreShape = ScreenLiveViewViewOnlySafetyShape &
  ScreenLiveViewProofRefsShape & {
    readonly transportMode: string;
    readonly sourceLabel: string;
    readonly custodyState: string;
    readonly productLiveViewReady: boolean;
  };

export function screenLiveViewDisablesFrameStorageAndRemoteControl(
  value: ScreenLiveViewViewOnlySafetyShape
): boolean {
  return (
    value.frameRetentionBehavior === 'noFrameRetention' &&
    value.cacheRawFrames === false &&
    value.sessionRecordingAllowed === false &&
    value.remoteInputControlAllowed === false
  );
}

export function screenLiveViewHasViewerAuditAndTransportProof(value: ScreenLiveViewProofRefsShape): boolean {
  return value.viewerAuditRef !== null && value.liveTransportProofRef !== null;
}

export function screenLiveViewDisabledCoreFieldsAreConsistent(value: ScreenLiveViewDisabledCoreShape): boolean {
  return (
    value.transportMode === 'none' &&
    value.sourceLabel === 'unavailable' &&
    value.custodyState === 'unavailable' &&
    value.viewerAuditRef === null &&
    value.liveTransportProofRef === null &&
    !value.productLiveViewReady &&
    screenLiveViewDisablesFrameStorageAndRemoteControl(value)
  );
}
