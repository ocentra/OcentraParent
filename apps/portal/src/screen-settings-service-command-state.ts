import { type ScreenAnalysisParentSetting } from '@ocentra-parent/schema-domain/screen-evidence-settings';
import {
  ScreenSettingsGetRequestSchema,
  ScreenSettingsReplaceRequestSchema,
  ScreenSettingsSchemaVersion,
  ScreenSettingsUpdateKindValue,
  ScreenSettingsUpdateRequestSchema,
  ScreenSettingsUpdateStatus,
  ScreenSettingsUpdateResponseSchema,
  type ScreenSettingsGetRequest,
  type ScreenSettingsReplaceRequest,
  type ScreenSettingsUpdateRequest,
  type ScreenSettingsUpdateResponse,
} from '@ocentra-parent/schema-domain/agent-screen-settings';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { type ScreenEvidenceSettingsUiProof } from '@ocentra-parent/schema-domain/screen-evidence-settings-ui-proof';
import { ParentUiActionKind, type ParentUiActionPayload } from '../generated/parent-ui-bridge';

export type ScreenSettingsServiceRequestId = ReturnType<typeof createScreenSettingsPortalRequestId>;
export type ScreenSettingsServiceBridgeAction =
  | typeof ParentUiActionKind.ScreenSettingsGetRequested
  | typeof ParentUiActionKind.ScreenSettingsReplaceRequested;

export type ScreenSettingsServiceCommandDraft = {
  readonly action: ScreenSettingsServiceBridgeAction;
  readonly payload: ParentUiActionPayload;
  readonly requestId: ScreenSettingsServiceRequestId;
};

export type ScreenSettingsServiceResponse = ScreenSettingsUpdateResponse | null;
const SCREEN_SETTINGS_REQUEST_ID_PREFIX = 'screen-settings-request-';

export function createScreenSettingsGetCommandDraft(sequence: number): ScreenSettingsServiceCommandDraft {
  const requestId = createScreenSettingsPortalRequestId(sequence);
  return {
    action: ParentUiActionKind.ScreenSettingsGetRequested,
    payload: createScreenSettingsCommandPayload(createScreenSettingsGetRequest(requestId)),
    requestId,
  };
}

export function createScreenSettingsReplaceCommandDraft(input: {
  readonly baseSettingVersion: number | null;
  readonly sequence: number;
  readonly setting: ScreenAnalysisParentSetting;
}): ScreenSettingsServiceCommandDraft {
  const requestId = createScreenSettingsPortalRequestId(input.sequence);
  return {
    action: ParentUiActionKind.ScreenSettingsReplaceRequested,
    payload: createScreenSettingsCommandPayload(
      createScreenSettingsReplaceRequest({
        requestId,
        baseSettingVersion: input.baseSettingVersion,
        setting: input.setting,
      })
    ),
    requestId,
  };
}

export function decodeScreenSettingsServiceResponseSnapshot(snapshot: unknown): ScreenSettingsServiceResponse {
  const parsed = ScreenSettingsUpdateResponseSchema.safeParse(snapshot);
  return parsed.success ? parsed.data : null;
}

export function matchingScreenSettingsServiceResponse(
  response: ScreenSettingsServiceResponse,
  requestId: ScreenSettingsServiceRequestId | null
): ScreenSettingsServiceResponse {
  if (response === null) {
    return null;
  }
  if (requestId === null || response.requestId === requestId) {
    return response;
  }
  return null;
}

export function screenSettingsBaseVersionForReplace(response: ScreenSettingsUpdateResponse | null): number | null {
  if (
    response?.kind !== ScreenSettingsUpdateKindValue.Replace ||
    response.status !== ScreenSettingsUpdateStatus.Accepted ||
    response.setting === null
  ) {
    return null;
  }
  return response.setting.settingVersion;
}

export function screenSettingsServiceStatusText(input: {
  readonly commandEnabled: boolean;
  readonly pendingRequestId: ScreenSettingsServiceRequestId | null;
  readonly proof: ScreenEvidenceSettingsUiProof;
  readonly response: ScreenSettingsUpdateResponse | null;
}) {
  if (!input.commandEnabled) {
    return input.proof.serviceDisconnectedStatus;
  }
  if (input.response?.status === ScreenSettingsUpdateStatus.Accepted) {
    return input.proof.serviceAcceptedStatus;
  }
  if (input.response?.status === ScreenSettingsUpdateStatus.Rejected) {
    return input.proof.serviceRejectedStatus;
  }
  if (input.pendingRequestId !== null) {
    return input.proof.servicePendingStatus;
  }
  return input.proof.serviceNoResponseStatus;
}

function createScreenSettingsPortalRequestId(sequence: number) {
  return `${SCREEN_SETTINGS_REQUEST_ID_PREFIX}${sequence}`;
}

function createScreenSettingsGetRequest(requestId: string): ScreenSettingsGetRequest {
  return ScreenSettingsGetRequestSchema.parse({
    schemaVersion: ScreenSettingsSchemaVersion,
    requestId,
    kind: 'get',
  });
}

function createScreenSettingsReplaceRequest(input: {
  readonly requestId: string;
  readonly baseSettingVersion: number | null;
  readonly setting: ScreenAnalysisParentSetting;
}): ScreenSettingsReplaceRequest {
  return ScreenSettingsReplaceRequestSchema.parse({
    schemaVersion: ScreenSettingsSchemaVersion,
    requestId: input.requestId,
    kind: 'replace',
    baseSettingVersion: input.baseSettingVersion,
    setting: input.setting,
  });
}

function createScreenSettingsCommandPayload(request: ScreenSettingsUpdateRequest): ParentUiActionPayload {
  const parsed = ScreenSettingsUpdateRequestSchema.safeParse(request);
  if (!parsed.success || parsed.data === undefined) {
    throw new Error('invalid screen settings request');
  }
  return {
    [AgentProtocolDefaults.Field.ScreenSettingsRequest]: JSON.stringify(parsed.data),
    [AgentProtocolDefaults.Field.ScreenSettingsUpdateKind]: parsed.data.kind,
  };
}
