import { PARENT_PORTAL_CONTENT, type ParentPortalContent } from './parent-portal-data';

export const PARENT_PORTAL_SERVICE_STATE = {
  Empty: '',
  RowSource: {
    Api: 'api',
  },
  Connection: {
    Connected: 'connected',
    Connecting: 'connecting',
    Error: 'error',
  },
  Label: {
    LocalAgent: 'Local agent',
    LanDiscovery: 'LAN discovery',
    DevicePairing: 'Device pairing',
    BrowserActivity: 'Browser activity',
    ActivityReports: 'Activity reports',
    NetworkTracking: 'Network tracking',
  },
  Area: {
    Service: 'Service',
    Runtime: 'Runtime',
    Lan: 'LAN',
    CurrentDevice: 'Current device',
    Browser: 'Browser',
    Activity: 'Activity',
    Network: 'Network',
  },
  Trend: {
    NotReported: 'not-reported',
    Offline: 'offline',
    Reported: 'reported',
    Unavailable: 'unavailable',
    ManualRequired: 'manual-required',
    PermissionRequired: 'permission-required',
    ScaffoldOnly: 'scaffold-only',
  },
} as const;

export const SERVICE_BACKED_CONTENT: ParentPortalContent = {
  ...PARENT_PORTAL_CONTENT,
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
