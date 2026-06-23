import { resolveLiveActivityState as resolvePortalDomainLiveActivityState } from '@ocentra-parent/portal-domain/live-activity-state';
import type {
  PortalBrowserRuntimeEventChainEntry as PortalDomainPortalBrowserRuntimeEventChainEntry,
  PortalBrowserRuntimeEventChainStream as PortalDomainPortalBrowserRuntimeEventChainStream,
  PortalLiveActivityState as PortalDomainPortalLiveActivityState,
  PortalNetworkRuntimeEventChainStream as PortalDomainPortalNetworkRuntimeEventChainStream,
} from '@ocentra-parent/portal-domain/live-activity-state';

import type { AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';

export type PortalBrowserRuntimeEventChainEntry = PortalDomainPortalBrowserRuntimeEventChainEntry;
export type PortalBrowserRuntimeEventChainStream = PortalDomainPortalBrowserRuntimeEventChainStream;
export type PortalLiveActivityState = PortalDomainPortalLiveActivityState;
export type PortalNetworkRuntimeEventChainStream = PortalDomainPortalNetworkRuntimeEventChainStream;

export function resolveLiveActivityState(events: readonly AgentEventEnvelope[]): PortalLiveActivityState {
  return resolvePortalDomainLiveActivityState(events);
}
