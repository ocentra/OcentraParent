import type {
  ParentAgentCommandName as AgentCommandName,
  ParentAgentEventName as AgentEventName,
  ParentAgentProtocolPayload as AgentProtocolLogFields,
  ParentUiActionPayload,
  ParentUiActionResult,
} from '../generated/parent-ui-bridge';

export interface PortalRenderActions {
  reconnect(): void;
  selectCommandResult(resultEvent: AgentEventName): void;
  sendCommand(command: AgentCommandName, payload: AgentProtocolLogFields): Promise<ParentUiActionResult | null>;
  refreshRouteSnapshot?(): Promise<ParentUiActionResult | null>;
  requestLanPairingBrowserDiscoveryScan?(): Promise<ParentUiActionResult | null>;
  requestNetworkFlowReadModelRefresh?(): Promise<ParentUiActionResult | null>;
  requestTrackingRetentionSettingsWrite?(): Promise<ParentUiActionResult | null>;
  requestPolicyRequestAssistantPreviewConfirm?(payload: ParentUiActionPayload): Promise<ParentUiActionResult | null>;
  requestPolicyRequestParentResolution?(payload: ParentUiActionPayload): Promise<ParentUiActionResult | null>;
  requestScreenSettingsGet?(payload: ParentUiActionPayload): Promise<ParentUiActionResult | null>;
  requestScreenSettingsReplace?(payload: ParentUiActionPayload): Promise<ParentUiActionResult | null>;
  requestAppGameAdapterDispatchExecute?(): Promise<ParentUiActionResult | null>;
  requestAppGameTimerParentPreferenceSetup?(payload: ParentUiActionPayload): Promise<ParentUiActionResult | null>;
}
