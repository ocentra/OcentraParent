import {
  ParentScreenSettingsUpdateKind,
  parentScreenSettingsGetCommandDraft,
  parentScreenSettingsReplaceCommandDraft,
  type ParentScreenSettingsServiceBridgeAction,
  type ParentScreenSettingsServiceCommandDraft,
  type ParentScreenSettingsServiceRequestId,
} from '../generated/parent-ui-bridge';
import {
  ParentScreenSettingsUpdateStatus,
  decodeParentScreenSettingsUpdateResponse,
  type ParentScreenAnalysisParentSetting,
  type ParentScreenEvidenceSettingsUiProof,
  type ParentScreenSettingsUpdateResponse,
} from '../generated/parent-ui-screen-bridge';

export type ScreenSettingsServiceRequestId = ParentScreenSettingsServiceRequestId;
export type ScreenSettingsServiceBridgeAction = ParentScreenSettingsServiceBridgeAction;
export type ScreenSettingsServiceCommandDraft = ParentScreenSettingsServiceCommandDraft;

export type ScreenSettingsServiceResponse = ParentScreenSettingsUpdateResponse | null;

export function createScreenSettingsGetCommandDraft(sequence: number): ScreenSettingsServiceCommandDraft {
  return parentScreenSettingsGetCommandDraft(sequence);
}

export function createScreenSettingsReplaceCommandDraft(input: {
  readonly baseSettingVersion: number | null;
  readonly sequence: number;
  readonly setting: ParentScreenAnalysisParentSetting;
}): ScreenSettingsServiceCommandDraft {
  return parentScreenSettingsReplaceCommandDraft(input);
}

export function decodeScreenSettingsServiceResponseSnapshot(snapshot: unknown): ScreenSettingsServiceResponse {
  return decodeParentScreenSettingsUpdateResponse(snapshot);
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

export function screenSettingsBaseVersionForReplace(
  response: ParentScreenSettingsUpdateResponse | null
): number | null {
  if (
    response?.kind !== ParentScreenSettingsUpdateKind.Replace ||
    response.status !== ParentScreenSettingsUpdateStatus.Accepted ||
    response.setting === null
  ) {
    return null;
  }
  return response.setting.settingVersion;
}

export function screenSettingsServiceStatusText(input: {
  readonly commandEnabled: boolean;
  readonly pendingRequestId: ScreenSettingsServiceRequestId | null;
  readonly proof: ParentScreenEvidenceSettingsUiProof;
  readonly response: ParentScreenSettingsUpdateResponse | null;
}) {
  if (!input.commandEnabled) {
    return input.proof.serviceDisconnectedStatus;
  }
  if (input.response?.status === ParentScreenSettingsUpdateStatus.Accepted) {
    return input.proof.serviceAcceptedStatus;
  }
  if (input.response?.status === ParentScreenSettingsUpdateStatus.Rejected) {
    return input.proof.serviceRejectedStatus;
  }
  if (input.pendingRequestId !== null) {
    return input.proof.servicePendingStatus;
  }
  return input.proof.serviceNoResponseStatus;
}
