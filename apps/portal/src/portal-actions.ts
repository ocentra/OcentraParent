import type {
  AgentCommandName,
  AgentEventName,
  AgentProtocolLogFields,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import type { ParentUiAction, ParentUiActionResult } from './generated/parent-ui-bridge';

export interface PortalRenderActions {
  reconnect(): void;
  selectCommandResult(resultEvent: AgentEventName): void;
  sendCommand(command: AgentCommandName, payload: AgentProtocolLogFields): Promise<ParentUiActionResult | null>;
  refreshRouteSnapshot?(): Promise<ParentUiActionResult | null>;
  requestLanPairingBrowserDiscoveryScan?(): Promise<ParentUiActionResult | null>;
  requestNetworkFlowReadModelRefresh?(): Promise<ParentUiActionResult | null>;
  requestTrackingRetentionSettingsWrite?(): Promise<ParentUiActionResult | null>;
  requestScreenSettingsGet?(payload: ParentUiAction['payload']): Promise<ParentUiActionResult | null>;
  requestScreenSettingsReplace?(payload: ParentUiAction['payload']): Promise<ParentUiActionResult | null>;
  requestAppGameAdapterDispatchExecute?(): Promise<ParentUiActionResult | null>;
  requestAppGameTimerParentPreferenceSetup?(
    payload: ParentUiAction['payload']
  ): Promise<ParentUiActionResult | null>;
}
