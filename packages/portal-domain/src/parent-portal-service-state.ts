import type { AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { ParentPortalContent, ParentPortalRow } from './parent-portal-data';
import { PARENT_PORTAL_SERVICE_STATE, SERVICE_BACKED_CONTENT } from './parent-portal-service-state-constants';
import { parentPortalServiceRows } from './parent-portal-service-state-rows';

export { PARENT_PORTAL_SERVICE_STATE };

export type ParentPortalServiceConnectionState = 'connected' | 'connecting' | 'disconnected' | 'error';

export type ParentPortalServiceStateInput = {
  readonly connectionState: ParentPortalServiceConnectionState;
  readonly events: readonly AgentEventEnvelope[];
};

export type ParentPortalServiceState = {
  readonly content: ParentPortalContent;
  readonly parentPortalRows: ParentPortalRow[];
  readonly userEntry: ParentPortalRow | null;
};

export function resolveParentPortalServiceState(input: ParentPortalServiceStateInput): ParentPortalServiceState {
  const parentPortalRows = parentPortalServiceRows(input);

  return {
    content: SERVICE_BACKED_CONTENT,
    parentPortalRows,
    userEntry: parentPortalRows[0] ?? null,
  };
}
