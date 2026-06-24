import { type ScreenAnalysisParentSetting } from '@ocentra-parent/schema-domain/screen-evidence-settings';
import {
  AgentEvent,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
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
import { type ScreenEvidenceSettingsUiProof } from '@ocentra-parent/schema-domain/screen-evidence-settings-ui-proof';
import type { ParentUiAction } from './generated/parent-ui-bridge';

export type ScreenSettingsServiceRequestId = ReturnType<typeof createScreenSettingsPortalRequestId>;
export type ScreenSettingsServiceBridgeAction =
  | 'screen-settings-get-requested'
  | 'screen-settings-replace-requested';

export type ScreenSettingsServiceCommandDraft = {
  readonly action: ScreenSettingsServiceBridgeAction;
  readonly payload: ParentUiAction['payload'];
  readonly requestId: ScreenSettingsServiceRequestId;
};

const SCREEN_SETTINGS_REQUEST_ID_PREFIX = 'screen-settings-request-';

export function createScreenSettingsGetCommandDraft(sequence: number): ScreenSettingsServiceCommandDraft {
  const requestId = createScreenSettingsPortalRequestId(sequence);
  return {
    action: 'screen-settings-get-requested',
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
    action: 'screen-settings-replace-requested',
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
    const response = parseScreenSettingsUpdateEvent(event);
    if (response === null) {
      continue;
    }
    if (requestId === null || response.requestId === requestId) {
      return response;
    }
  }
  return null;
}

export function decodeScreenSettingsServiceResponseSnapshot(snapshot: unknown): ScreenSettingsUpdateResponse | null {
  const parsed = ScreenSettingsUpdateResponseSchema.safeParse(snapshot);
  return parsed.success ? parsed.data : null;
}

export function matchingScreenSettingsServiceResponse(
  response: ScreenSettingsUpdateResponse | null,
  requestId: ScreenSettingsServiceRequestId | null
): ScreenSettingsUpdateResponse | null {
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

function createScreenSettingsPortalRequestId(sequence: number): string {
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

function createScreenSettingsCommandPayload(request: ScreenSettingsUpdateRequest): ParentUiAction['payload'] {
  const parsed = ScreenSettingsUpdateRequestSchema.safeParse(request);
  if (!parsed.success || parsed.data === undefined) {
    throw new Error('invalid screen settings request');
  }
  return {
    screenSettingsRequest: JSON.stringify(parsed.data),
    screenSettingsUpdateKind: parsed.data.kind,
  };
}

function parseScreenSettingsUpdateEvent(event: AgentEventEnvelope): ScreenSettingsUpdateResponse | null {
  if (
    event.event !== AgentEvent.ScreenSettingsReported &&
    event.event !== AgentEvent.ScreenSettingsReplaceAccepted &&
    event.event !== AgentEvent.ScreenSettingsReplaceRejected
  ) {
    return null;
  }

  const raw = event.payload['screenSettingsResponse'];
  if (typeof raw !== 'string') {
    return null;
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return null;
  }

  const parsed = ScreenSettingsUpdateResponseSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return null;
  }

  return parsed.data;
}
