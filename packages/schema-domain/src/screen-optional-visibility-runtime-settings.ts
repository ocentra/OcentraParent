import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { ActivityTimestampSchema } from './evidence-primitives';
import { ScreenEvidenceReasonSchema, ScreenEvidenceSettingVersionSchema } from './screen-evidence-primitives';
import {
  ScreenLiveViewOptInSettingSchema,
  ScreenRawScreenshotRetentionOptInSettingSchema,
  type ScreenLiveViewOptInSetting,
  type ScreenRawScreenshotRetentionOptInSetting,
} from './screen-optional-visibility-mode';

export const ScreenOptionalVisibilityRuntimeSettingsSchemaVersion = 1;

const RequiredFalse = Schema.Literal(false);
const OptionalRuntimeReasonSchema = Schema.Union(ScreenEvidenceReasonSchema, Schema.Null);
type OptionalRuntimeReason = Infer<typeof OptionalRuntimeReasonSchema>;

export const ScreenOptionalVisibilityRuntimeRequestKindSchema = withParser(
  Schema.Literal('replaceRawRetention', 'replaceLiveView', 'disableAll')
);

export const ScreenOptionalVisibilityRuntimeUpdateStatusSchema = withParser(Schema.Literal('accepted', 'rejected'));

export const ScreenOptionalVisibilityRuntimeRejectionReasonSchema = withParser(
  Schema.Literal('invalid-setting', 'stale-revision', 'mode-conflict')
);

type ScreenOptionalVisibilityRuntimeSettingsStateShape = {
  readonly schemaVersion: typeof ScreenOptionalVisibilityRuntimeSettingsSchemaVersion;
  readonly revision: number;
  readonly updatedAt: string;
  readonly rawRetentionSetting: ScreenRawScreenshotRetentionOptInSetting;
  readonly liveViewSetting: ScreenLiveViewOptInSetting;
  readonly rawScreenshotRemoteUploadEnabled: false;
  readonly productLiveViewReady: false;
  readonly reason: OptionalRuntimeReason;
};

type ScreenOptionalVisibilityRuntimeUpdateRequestShape = {
  readonly schemaVersion: typeof ScreenOptionalVisibilityRuntimeSettingsSchemaVersion;
  readonly requestId: string;
  readonly kind: ScreenOptionalVisibilityRuntimeRequestKind;
  readonly baseRevision: number | null;
  readonly rawRetentionSetting: ScreenRawScreenshotRetentionOptInSetting | null;
  readonly liveViewSetting: ScreenLiveViewOptInSetting | null;
  readonly changedAt: string;
  readonly reason: OptionalRuntimeReason;
};

type ScreenOptionalVisibilityRuntimeUpdateResponseShape = {
  readonly schemaVersion: typeof ScreenOptionalVisibilityRuntimeSettingsSchemaVersion;
  readonly requestId: string;
  readonly status: ScreenOptionalVisibilityRuntimeUpdateStatus;
  readonly state: ScreenOptionalVisibilityRuntimeSettingsState | null;
  readonly rejectionReason: ScreenOptionalVisibilityRuntimeRejectionReason | null;
  readonly message: string;
};

export const ScreenOptionalVisibilityRuntimeSettingsStateSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOptionalVisibilityRuntimeSettingsSchemaVersion),
    revision: ScreenEvidenceSettingVersionSchema,
    updatedAt: ActivityTimestampSchema,
    rawRetentionSetting: ScreenRawScreenshotRetentionOptInSettingSchema,
    liveViewSetting: ScreenLiveViewOptInSettingSchema,
    rawScreenshotRemoteUploadEnabled: RequiredFalse,
    productLiveViewReady: RequiredFalse,
    reason: OptionalRuntimeReasonSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        optionalVisibilityRuntimeStateIsConsistent(value) ||
        'Expected optional visibility runtime settings to avoid mixed raw-retention/live-view custody and product-live-view readiness'
    )
  )
);

export const ScreenOptionalVisibilityRuntimeUpdateRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOptionalVisibilityRuntimeSettingsSchemaVersion),
    requestId: NonEmptyStringSchema,
    kind: ScreenOptionalVisibilityRuntimeRequestKindSchema,
    baseRevision: Schema.Union(ScreenEvidenceSettingVersionSchema, Schema.Null),
    rawRetentionSetting: Schema.Union(ScreenRawScreenshotRetentionOptInSettingSchema, Schema.Null),
    liveViewSetting: Schema.Union(ScreenLiveViewOptInSettingSchema, Schema.Null),
    changedAt: ActivityTimestampSchema,
    reason: OptionalRuntimeReasonSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        optionalVisibilityRuntimeRequestShapeIsConsistent(value) ||
        'Expected optional visibility runtime update request to carry only the setting required by its kind'
    )
  )
);

export const ScreenOptionalVisibilityRuntimeUpdateResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenOptionalVisibilityRuntimeSettingsSchemaVersion),
    requestId: NonEmptyStringSchema,
    status: ScreenOptionalVisibilityRuntimeUpdateStatusSchema,
    state: Schema.Union(ScreenOptionalVisibilityRuntimeSettingsStateSchema, Schema.Null),
    rejectionReason: Schema.Union(ScreenOptionalVisibilityRuntimeRejectionReasonSchema, Schema.Null),
    message: NonEmptyStringSchema,
  }).pipe(
    Schema.filter(
      (value) =>
        optionalVisibilityRuntimeResponseIsConsistent(value) ||
        'Expected accepted optional visibility runtime responses to include state and rejected responses to include a reason'
    )
  )
);

export function createDisabledScreenOptionalVisibilityRuntimeSettingsState(input: {
  readonly updatedAt: string;
  readonly rawRetentionSetting: ScreenRawScreenshotRetentionOptInSetting;
  readonly liveViewSetting: ScreenLiveViewOptInSetting;
  readonly reason: string;
}): ScreenOptionalVisibilityRuntimeSettingsState {
  return ScreenOptionalVisibilityRuntimeSettingsStateSchema.parse({
    schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
    revision: 1,
    updatedAt: input.updatedAt,
    rawRetentionSetting: input.rawRetentionSetting,
    liveViewSetting: input.liveViewSetting,
    rawScreenshotRemoteUploadEnabled: false,
    productLiveViewReady: false,
    reason: input.reason,
  });
}

export function applyScreenOptionalVisibilityRuntimeSettingsRequest(
  state: ScreenOptionalVisibilityRuntimeSettingsState,
  request: ScreenOptionalVisibilityRuntimeUpdateRequest
): ScreenOptionalVisibilityRuntimeUpdateResponse {
  if (request.baseRevision !== null && request.baseRevision !== state.revision) {
    return rejectedRuntimeResponse(request.requestId, 'stale-revision');
  }

  const nextState = nextOptionalVisibilityRuntimeState(state, request);
  if (nextState === null) {
    return rejectedRuntimeResponse(request.requestId, 'invalid-setting');
  }
  const parsed = ScreenOptionalVisibilityRuntimeSettingsStateSchema.safeParse(nextState);
  if (!parsed.success || parsed.data === undefined) {
    return rejectedRuntimeResponse(request.requestId, 'mode-conflict');
  }

  return ScreenOptionalVisibilityRuntimeUpdateResponseSchema.parse({
    schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
    requestId: request.requestId,
    status: 'accepted',
    state: parsed.data,
    rejectionReason: null,
    message: 'optional visibility runtime settings accepted',
  });
}

function nextOptionalVisibilityRuntimeState(
  state: ScreenOptionalVisibilityRuntimeSettingsState,
  request: ScreenOptionalVisibilityRuntimeUpdateRequest
): ScreenOptionalVisibilityRuntimeSettingsStateShape | null {
  if (request.kind === 'replaceRawRetention' && request.rawRetentionSetting !== null) {
    return runtimeStateFromSettings(
      state,
      request.changedAt,
      request.rawRetentionSetting,
      state.liveViewSetting,
      request.reason
    );
  }
  if (request.kind === 'replaceLiveView' && request.liveViewSetting !== null) {
    return runtimeStateFromSettings(
      state,
      request.changedAt,
      state.rawRetentionSetting,
      request.liveViewSetting,
      request.reason
    );
  }
  if (request.kind === 'disableAll') {
    return runtimeStateFromSettings(
      state,
      request.changedAt,
      disabledRawRetention(state),
      disabledLiveView(state),
      request.reason
    );
  }

  return null;
}

function runtimeStateFromSettings(
  state: ScreenOptionalVisibilityRuntimeSettingsState,
  updatedAt: string,
  rawRetentionSetting: ScreenRawScreenshotRetentionOptInSetting,
  liveViewSetting: ScreenLiveViewOptInSetting,
  reason: OptionalRuntimeReason
): ScreenOptionalVisibilityRuntimeSettingsStateShape {
  return {
    schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
    revision: state.revision + 1,
    updatedAt,
    rawRetentionSetting,
    liveViewSetting,
    rawScreenshotRemoteUploadEnabled: false,
    productLiveViewReady: false,
    reason,
  };
}

function rejectedRuntimeResponse(
  requestId: string,
  rejectionReason: ScreenOptionalVisibilityRuntimeRejectionReason
): ScreenOptionalVisibilityRuntimeUpdateResponse {
  return ScreenOptionalVisibilityRuntimeUpdateResponseSchema.parse({
    schemaVersion: ScreenOptionalVisibilityRuntimeSettingsSchemaVersion,
    requestId,
    status: 'rejected',
    state: null,
    rejectionReason,
    message: 'optional visibility runtime settings rejected',
  });
}

function optionalVisibilityRuntimeStateIsConsistent(value: ScreenOptionalVisibilityRuntimeSettingsStateShape): boolean {
  return (
    value.rawScreenshotRemoteUploadEnabled === false &&
    value.productLiveViewReady === false &&
    (value.rawRetentionSetting.mode === 'disabled' || value.liveViewSetting.liveViewMode === 'disabled')
  );
}

function optionalVisibilityRuntimeRequestShapeIsConsistent(
  value: ScreenOptionalVisibilityRuntimeUpdateRequestShape
): boolean {
  if (value.kind === 'replaceRawRetention') {
    return value.rawRetentionSetting !== null && value.liveViewSetting === null;
  }
  if (value.kind === 'replaceLiveView') {
    return value.rawRetentionSetting === null && value.liveViewSetting !== null;
  }

  return value.rawRetentionSetting === null && value.liveViewSetting === null;
}

function optionalVisibilityRuntimeResponseIsConsistent(
  value: ScreenOptionalVisibilityRuntimeUpdateResponseShape
): boolean {
  if (value.status === 'accepted') {
    return value.state !== null && value.rejectionReason === null;
  }

  return value.state === null && value.rejectionReason !== null;
}

function disabledRawRetention(
  state: ScreenOptionalVisibilityRuntimeSettingsState
): ScreenRawScreenshotRetentionOptInSetting {
  return ScreenRawScreenshotRetentionOptInSettingSchema.parse({
    ...state.rawRetentionSetting,
    mode: 'disabled',
    explicitParentApproval: false,
    approvalRef: null,
    disclosureState: 'notRequired',
    auditRef: null,
    ttlSeconds: null,
    custodyState: 'unavailable',
    exportRef: null,
    sourceLabel: 'unavailable',
    retentionBehavior: 'noRawRetention',
    deleteAfterTtl: false,
    deleteProofRequired: false,
    rawScreenshotRemoteUploadEnabled: false,
    reason: 'raw retention disabled by optional visibility runtime update',
  });
}

function disabledLiveView(state: ScreenOptionalVisibilityRuntimeSettingsState): ScreenLiveViewOptInSetting {
  return ScreenLiveViewOptInSettingSchema.parse({
    ...state.liveViewSetting,
    liveViewMode: 'disabled',
    transportMode: 'none',
    explicitParentApproval: false,
    approvalRef: null,
    disclosureState: 'notRequired',
    viewerAuditRef: null,
    platformProofState: 'notRequired',
    platformProofRef: null,
    custodyState: 'unavailable',
    sourceLabel: 'unavailable',
    frameRetentionBehavior: 'noFrameRetention',
    cacheRawFrames: false,
    sessionRecordingAllowed: false,
    remoteInputControlAllowed: false,
    reason: 'live view disabled by optional visibility runtime update',
  });
}

export type ScreenOptionalVisibilityRuntimeRequestKind = Infer<typeof ScreenOptionalVisibilityRuntimeRequestKindSchema>;
export type ScreenOptionalVisibilityRuntimeUpdateStatus = Infer<
  typeof ScreenOptionalVisibilityRuntimeUpdateStatusSchema
>;
export type ScreenOptionalVisibilityRuntimeRejectionReason = Infer<
  typeof ScreenOptionalVisibilityRuntimeRejectionReasonSchema
>;
export type ScreenOptionalVisibilityRuntimeSettingsState = Infer<
  typeof ScreenOptionalVisibilityRuntimeSettingsStateSchema
>;
export type ScreenOptionalVisibilityRuntimeUpdateRequest = Infer<
  typeof ScreenOptionalVisibilityRuntimeUpdateRequestSchema
>;
export type ScreenOptionalVisibilityRuntimeUpdateResponse = Infer<
  typeof ScreenOptionalVisibilityRuntimeUpdateResponseSchema
>;
