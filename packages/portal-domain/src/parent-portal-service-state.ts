import { PARENT_PORTAL_CONTENT, type ParentPortalContent, type ParentPortalRow } from './parent-portal-data';
import {
  generatedResolveParentPortalServiceReachability,
  type GeneratedParentPortalServiceConnectionState,
  type GeneratedParentPortalServiceDegradationReasonCode,
  type GeneratedParentPortalServiceReachability,
} from './portal-route-state.generated';
import PARENT_PORTAL_SERVICE_STATE, { PORTAL_HOST_BRIDGE_RUNTIME } from './parent-portal-service-state-constants';
import type { PortalRouteEventRecord } from './portal-contract-adapter';

export { PARENT_PORTAL_SERVICE_STATE, PORTAL_HOST_BRIDGE_RUNTIME };

type ParentPortalServiceReachability = GeneratedParentPortalServiceReachability;

export const SERVICE_BACKED_CONTENT: ParentPortalContent = {
  ...PARENT_PORTAL_CONTENT,
  uiCopy: {
    ...PARENT_PORTAL_CONTENT.uiCopy,
    detailSnapshotLines: [
      'Visible rows come from Rust-owned route snapshots through the parent bridge.',
      'When snapshot rows are absent, the UI leaves the surface unclaimed instead of rebuilding state in TypeScript.',
      'Reachability and degradation stay explicit instead of being inferred from the route surface.',
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

export type ParentPortalServiceConnectionState = GeneratedParentPortalServiceConnectionState;

export type ParentPortalServiceStateInput = {
  readonly connectionState: ParentPortalServiceConnectionState;
  readonly events: readonly PortalRouteEventRecord[];
  readonly snapshotRows?: readonly ParentPortalRow[] | null;
};

export type ParentPortalServiceState = {
  readonly connectionState: ParentPortalServiceConnectionState;
  readonly content: ParentPortalContent;
  readonly parentPortalRows: ParentPortalRow[];
  readonly serviceReachability: ParentPortalServiceReachability;
  readonly serviceDegradationReason: string | null;
  readonly userEntry: ParentPortalRow | null;
};

const SERVICE_DEGRADATION_REASON_TEXT: Readonly<Record<GeneratedParentPortalServiceDegradationReasonCode, string>> = {
  'missing-snapshot-rows': 'Connected to the local service, but no Rust-owned route rows were supplied.',
  connecting: 'The local service bridge is still connecting.',
  'stale-snapshot-rows': 'Stale route rows remain visible while the service is not connected.',
  'service-unavailable': 'The local service bridge is unavailable.',
};

export function resolveParentPortalServiceState(input: ParentPortalServiceStateInput): ParentPortalServiceState {
  const parentPortalRows =
    input.snapshotRows === null || input.snapshotRows === undefined ? [] : [...input.snapshotRows];
  const { serviceReachability, serviceDegradationReasonCode } = generatedResolveParentPortalServiceReachability(
    input.connectionState,
    parentPortalRows.length > 0
  );

  return {
    connectionState: input.connectionState,
    content: SERVICE_BACKED_CONTENT,
    parentPortalRows,
    serviceReachability,
    serviceDegradationReason:
      serviceDegradationReasonCode === null ? null : SERVICE_DEGRADATION_REASON_TEXT[serviceDegradationReasonCode],
    userEntry: parentPortalRows[0] ?? null,
  };
}
