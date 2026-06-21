import type { AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { PARENT_PORTAL_CONTENT, type ParentPortalContent, type ParentPortalRow } from './parent-portal-data';
import PARENT_PORTAL_SERVICE_STATE from './parent-portal-service-state-constants';
import { parentPortalServiceRows } from './parent-portal-service-state-rows';

export { PARENT_PORTAL_SERVICE_STATE };

export const SERVICE_BACKED_CONTENT: ParentPortalContent = {
  ...PARENT_PORTAL_CONTENT,
  uiCopy: {
    ...PARENT_PORTAL_CONTENT.uiCopy,
    detailSnapshotLines: [
      'Visible rows use real service events first, then honest manual-required or unavailable gaps.',
      'State labels stay explicit: paired, pending, observer-only, controller, degraded, or backend-not-connected.',
    ],
  },
  modes: {
    ...PARENT_PORTAL_CONTENT.modes,
    parentOverview: {
      ...PARENT_PORTAL_CONTENT.modes.parentOverview,
      rowSource: PARENT_PORTAL_SERVICE_STATE.RowSource.Api,
    },
    parentManage: {
      ...PARENT_PORTAL_CONTENT.modes.parentManage,
      rowSource: PARENT_PORTAL_SERVICE_STATE.RowSource.Api,
    },
  },
};

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
