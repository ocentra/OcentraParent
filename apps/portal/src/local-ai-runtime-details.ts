import { AgentProtocolDefaults, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/contracts';
import { appendDetail } from './detail-list';
import { eventStatus, payloadDetail } from './event-detail-values';

export function appendRuntimeDetails(metadata: HTMLDListElement, runtimeEvent: AgentEventEnvelope | null): void {
  appendDetail(metadata, PortalDetails.Status, eventStatus(runtimeEvent));
  appendDetail(
    metadata,
    PortalDetails.RuntimeReference,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId)
  );
  appendDetail(
    metadata,
    PortalDetails.Provider,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiProviderId)
  );
  appendDetail(metadata, PortalDetails.Model, payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiModelId));
  appendDetail(
    metadata,
    PortalDetails.PrivacyMode,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiPrivacyMode)
  );
  appendDetail(
    metadata,
    PortalDetails.AdapterBoundary,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiAdapterBoundary)
  );
  appendDetail(
    metadata,
    PortalDetails.ExecutionState,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiExecutionState)
  );
  appendDetail(
    metadata,
    PortalDetails.ProviderSource,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiProviderSource)
  );
  appendDetail(metadata, PortalDetails.LoadState, payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LoadState));
  appendDetail(
    metadata,
    PortalDetails.Capability,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiCapabilityFlags)
  );
  appendDetail(
    metadata,
    PortalDetails.ResourceClass,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiResourceClass)
  );
  appendDetail(
    metadata,
    PortalDetails.DegradedState,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiDegradedState)
  );
  appendDetail(metadata, PortalDetails.LastChecked, payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.CheckedAt));
  appendDetail(
    metadata,
    PortalDetails.Reason,
    payloadDetail(runtimeEvent, AgentProtocolDefaults.Field.LocalAiUnavailableReason)
  );
}
