/* generated from crates/parent-runtime-core/src/portal_route_state.rs */

export type GeneratedParentPortalPageMode = 'parentOverview' | 'parentManage' | 'parentGuide';
export type GeneratedParentPortalManageLane = 'portal' | 'childPolicy' | 'deviceOps';
export type GeneratedParentPortalServiceConnectionState = 'connected' | 'connecting' | 'disconnected' | 'error';
export type GeneratedParentPortalServiceReachability = 'reachable' | 'degraded' | 'unavailable';
export type GeneratedParentPortalServiceDegradationReasonCode =
  | 'missing-snapshot-rows'
  | 'connecting'
  | 'stale-snapshot-rows'
  | 'service-unavailable';

export type GeneratedParentPortalRouteStateRecord = {
  readonly pageMode: GeneratedParentPortalPageMode;
  readonly selectedControlId: string;
  readonly manageLane: GeneratedParentPortalManageLane | null;
};

export type GeneratedParentPortalServiceReachabilityState = {
  readonly serviceReachability: GeneratedParentPortalServiceReachability;
  readonly serviceDegradationReasonCode: GeneratedParentPortalServiceDegradationReasonCode | null;
};

const portalRouteStateRecords = {
  overview: { pageMode: 'parentOverview', selectedControlId: 'activity-store', manageLane: null },
  assistant: { pageMode: 'parentGuide', selectedControlId: 'ai-runtime', manageLane: null },
  start: { pageMode: 'parentOverview', selectedControlId: 'setup-overall', manageLane: null },
  activity: { pageMode: 'parentManage', selectedControlId: 'reports-settings', manageLane: 'childPolicy' },
  browser: { pageMode: 'parentManage', selectedControlId: 'managed-web', manageLane: 'childPolicy' },
  'browser-settings': { pageMode: 'parentManage', selectedControlId: 'browser-settings', manageLane: 'childPolicy' },
  policy: { pageMode: 'parentGuide', selectedControlId: 'rules-policy', manageLane: null },
  'policy-apps': { pageMode: 'parentManage', selectedControlId: 'policy-apps', manageLane: 'childPolicy' },
  'policy-games': { pageMode: 'parentManage', selectedControlId: 'policy-games', manageLane: 'childPolicy' },
  'policy-screen': { pageMode: 'parentManage', selectedControlId: 'screen-analysis', manageLane: 'childPolicy' },
  'policy-network': { pageMode: 'parentManage', selectedControlId: 'network-activity', manageLane: 'childPolicy' },
  'policy-tracking': { pageMode: 'parentManage', selectedControlId: 'policy-tracking', manageLane: 'childPolicy' },
  'policy-remote-screen': {
    pageMode: 'parentManage',
    selectedControlId: 'policy-remote-screen',
    manageLane: 'childPolicy',
  },
  'rule-management': { pageMode: 'parentManage', selectedControlId: 'rules-management', manageLane: 'childPolicy' },
  schedules: { pageMode: 'parentManage', selectedControlId: 'schedules-budgets', manageLane: 'childPolicy' },
  approvals: { pageMode: 'parentManage', selectedControlId: 'approvals', manageLane: 'childPolicy' },
  enforcement: { pageMode: 'parentManage', selectedControlId: 'enforcement-readiness', manageLane: 'childPolicy' },
  'privacy-design': { pageMode: 'parentGuide', selectedControlId: 'privacy-design', manageLane: null },
  memory: { pageMode: 'parentGuide', selectedControlId: 'memory-citations', manageLane: null },
  'memory-settings': { pageMode: 'parentManage', selectedControlId: 'memory-settings', manageLane: 'childPolicy' },
  'ai-guide': { pageMode: 'parentGuide', selectedControlId: 'local-ai-evidence', manageLane: null },
  'ai-runtime': { pageMode: 'parentManage', selectedControlId: 'ai-runtime', manageLane: 'childPolicy' },
  'api-providers': { pageMode: 'parentManage', selectedControlId: 'api-providers', manageLane: 'childPolicy' },
  'reports-guide': { pageMode: 'parentGuide', selectedControlId: 'reports-summaries', manageLane: null },
  'screen-analysis': { pageMode: 'parentManage', selectedControlId: 'screen-analysis', manageLane: 'childPolicy' },
  'app-game-sessions': { pageMode: 'parentManage', selectedControlId: 'app-game-sessions', manageLane: 'childPolicy' },
  'network-activity': { pageMode: 'parentManage', selectedControlId: 'network-activity', manageLane: 'childPolicy' },
  devices: { pageMode: 'parentManage', selectedControlId: 'lan-pairing', manageLane: 'deviceOps' },
  'lan-pairing': { pageMode: 'parentManage', selectedControlId: 'lan-pairing', manageLane: 'deviceOps' },
  'capability-status': { pageMode: 'parentManage', selectedControlId: 'capability-status', manageLane: 'deviceOps' },
  notifications: { pageMode: 'parentManage', selectedControlId: 'notifications', manageLane: 'portal' },
  'notification-channels': {
    pageMode: 'parentManage',
    selectedControlId: 'notification-channels',
    manageLane: 'portal',
  },
  'drive-connections': { pageMode: 'parentManage', selectedControlId: 'drive-exports', manageLane: 'portal' },
  'export-retention': { pageMode: 'parentManage', selectedControlId: 'export-retention', manageLane: 'portal' },
  'remote-access': { pageMode: 'parentManage', selectedControlId: 'remote-access', manageLane: 'deviceOps' },
  'report-compiler': { pageMode: 'parentManage', selectedControlId: 'report-compiler', manageLane: 'childPolicy' },
  'audit-history': { pageMode: 'parentManage', selectedControlId: 'audit-history', manageLane: 'portal' },
  subscription: { pageMode: 'parentManage', selectedControlId: 'subscription-plans', manageLane: 'portal' },
  entitlements: { pageMode: 'parentManage', selectedControlId: 'entitlements', manageLane: 'portal' },
  'platforms-install': { pageMode: 'parentManage', selectedControlId: 'platforms-install', manageLane: 'deviceOps' },
  'install-updates': { pageMode: 'parentManage', selectedControlId: 'install-updates', manageLane: 'deviceOps' },
  diagnostics: { pageMode: 'parentManage', selectedControlId: 'support-api-status', manageLane: 'portal' },
  'proof-panels': { pageMode: 'parentManage', selectedControlId: 'dev-proof-panels', manageLane: null },
  'settings-rules': { pageMode: 'parentManage', selectedControlId: 'family-settings', manageLane: 'portal' },
  'app-layout': { pageMode: 'parentManage', selectedControlId: 'app-layout', manageLane: null },
  'frame-tuner': { pageMode: 'parentManage', selectedControlId: 'app-layout', manageLane: null },
  commands: { pageMode: 'parentManage', selectedControlId: 'dev-commands', manageLane: null },
  events: { pageMode: 'parentManage', selectedControlId: 'dev-events', manageLane: null },
  logs: { pageMode: 'parentManage', selectedControlId: 'dev-logs', manageLane: null },
} as const satisfies Record<string, GeneratedParentPortalRouteStateRecord>;

const portalRouteStateRecordKeys = Object.keys(portalRouteStateRecords);
const portalRouteStateRecordKeySet = new Set<string>(portalRouteStateRecordKeys);

export function generatedPortalRouteFromHashPath(routeHash: string): string | null {
  const normalizedHash = routeHash.replace(/^#\/?/u, '');
  const route = normalizedHash.split('?')[0] ?? '';
  return portalRouteStateRecordKeySet.has(route) ? route : null;
}

export function generatedParentPortalRouteState(route: string): GeneratedParentPortalRouteStateRecord | null {
  return Object.prototype.hasOwnProperty.call(portalRouteStateRecords, route)
    ? portalRouteStateRecords[route as keyof typeof portalRouteStateRecords]
    : null;
}

export function generatedParentPortalManageLaneForRoute(route: string): GeneratedParentPortalManageLane | null {
  return generatedParentPortalRouteState(route)?.manageLane ?? null;
}

export function generatedResolveParentPortalServiceReachability(
  connectionState: GeneratedParentPortalServiceConnectionState,
  hasSnapshotRows: boolean
): GeneratedParentPortalServiceReachabilityState {
  if (connectionState === 'connected') {
    return hasSnapshotRows
      ? { serviceReachability: 'reachable', serviceDegradationReasonCode: null }
      : {
          serviceReachability: 'degraded',
          serviceDegradationReasonCode: 'missing-snapshot-rows',
        };
  }

  if (connectionState === 'connecting') {
    return {
      serviceReachability: 'degraded',
      serviceDegradationReasonCode: 'connecting',
    };
  }

  if (hasSnapshotRows) {
    return {
      serviceReachability: 'degraded',
      serviceDegradationReasonCode: 'stale-snapshot-rows',
    };
  }

  return {
    serviceReachability: 'unavailable',
    serviceDegradationReasonCode: 'service-unavailable',
  };
}
