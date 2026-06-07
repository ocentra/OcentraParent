import { type ScreenAnalysisParentSetting } from '@ocentra-parent/activity-domain/screen-evidence-settings';
import {
  AgentCommand,
  type AgentCommandName,
  type AgentEventEnvelope,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  createScreenSettingsCommandPayload,
  createScreenSettingsGetRequest,
  createScreenSettingsPortalRequestId,
  createScreenSettingsReplaceRequest,
  parseScreenSettingsUpdateEvent,
  ScreenSettingsUpdateKindValue,
  ScreenSettingsUpdateStatus,
  type ScreenSettingsUpdateResponse,
} from '@ocentra-parent/agent-protocol-domain/screen-settings-adapter';
import { type ScreenEvidenceSettingsUiProof } from '@ocentra-parent/activity-domain/screen-evidence';

export type ScreenSettingsServiceCommandDraft = {
  readonly command: AgentCommandName;
  readonly payload: AgentProtocolLogFields;
  readonly requestId: string;
};
export type ScreenSettingsServiceRequestId = ScreenSettingsServiceCommandDraft['requestId'];

export function createScreenSettingsGetCommandDraft(sequence: number): ScreenSettingsServiceCommandDraft {
  const requestId = createScreenSettingsPortalRequestId(sequence);
  return {
    command: AgentCommand.ScreenSettingsGet,
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
    command: AgentCommand.ScreenSettingsReplace,
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

export function latestScreenSettingsServiceResponse(
  events: readonly AgentEventEnvelope[],
  requestId: ScreenSettingsServiceRequestId | null
): ScreenSettingsUpdateResponse | null {
  for (const event of events) {
    const parsed = parseScreenSettingsUpdateEvent(event);
    if (!parsed.ok) {
      continue;
    }
    if (requestId === null || parsed.value.requestId === requestId) {
      return parsed.value;
    }
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
}): string {
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
