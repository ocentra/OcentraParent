import {
  resolveLiveActivityState as resolvePortalDomainLiveActivityState,
  type PortalBrowserRuntimeEventChainEntry as PortalDomainPortalBrowserRuntimeEventChainEntry,
  type PortalBrowserRuntimeEventChainStream as PortalDomainPortalBrowserRuntimeEventChainStream,
  type PortalLiveActivityState as PortalDomainPortalLiveActivityState,
  type PortalNetworkRuntimeEventChainStream as PortalDomainPortalNetworkRuntimeEventChainStream,
} from '@ocentra-parent/portal-domain/live-activity-state';

import { AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';

export type PortalBrowserRuntimeEventChainEntry = PortalDomainPortalBrowserRuntimeEventChainEntry;
export type PortalBrowserRuntimeEventChainStream = PortalDomainPortalBrowserRuntimeEventChainStream;
export type PortalLiveActivityState = PortalDomainPortalLiveActivityState;
export type PortalNetworkRuntimeEventChainStream = PortalDomainPortalNetworkRuntimeEventChainStream;

export function resolveLiveActivityState(events: readonly AgentEventEnvelope[]): PortalLiveActivityState {
  return resolvePortalDomainLiveActivityState(events);
}
