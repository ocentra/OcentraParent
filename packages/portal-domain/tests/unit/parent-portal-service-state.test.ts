import { describe, expect, it } from 'vitest';
import { PARENT_PORTAL_SERVICE_STATE, PortalRoute, resolveParentPortalServiceState } from '../../src/contracts';
import {
  parentPortalManageLaneForRoute,
  parentPortalRouteContext,
  type ParentPortalRow,
} from '../../src/parent-portal-data';
import { portalRouteFromHashPath } from '../../src/routes';

describe('portal service-backed parent portal state', () => {
  it('hydrates rows only from Rust-owned snapshot rows', () => {
    const snapshotRows = [snapshotRow('Local agent', 'LOCAL'), snapshotRow('Browser activity', 'Browser')];

    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [],
      snapshotRows,
    });

    expect(state.content.modes.parentOverview.rowSource).toBe('api');
    expect(state.content.modes.parentManage.rowSource).toBe('api');
    expect(state.connectionState).toBe(PARENT_PORTAL_SERVICE_STATE.Connection.Connected);
    expect(state.parentPortalRows).toEqual(snapshotRows);
    expect(state.parentPortalRows).not.toBe(snapshotRows);
    expect(state.serviceReachability).toBe('reachable');
    expect(state.serviceDegradationReason).toBeNull();
    expect(state.userEntry).toEqual(snapshotRows[0]);
  });

  it('exposes degraded reachability when Rust snapshot rows are absent', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [],
      snapshotRows: null,
    });

    expect(state.parentPortalRows).toEqual([]);
    expect(state.userEntry).toBeNull();
    expect(state.serviceReachability).toBe('degraded');
    expect(state.serviceDegradationReason).toBe(
      'Connected to the local service, but no Rust-owned route rows were supplied.'
    );
  });

  it('reports an unavailable bridge when the service is disconnected and no rows are present', () => {
    const state = resolveParentPortalServiceState({
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Disconnected,
      events: [],
      snapshotRows: null,
    });

    expect(state.serviceReachability).toBe('unavailable');
    expect(state.serviceDegradationReason).toBe('The local service bridge is unavailable.');
  });

  it('keeps manage-lane ownership in portal-domain route contracts', () => {
    expect(parentPortalManageLaneForRoute(PortalRoute.Browser)).toBe('childPolicy');
    expect(parentPortalManageLaneForRoute(PortalRoute.Notifications)).toBe('portal');
    expect(parentPortalManageLaneForRoute(PortalRoute.Subscription)).toBe('portal');
    expect(parentPortalManageLaneForRoute(PortalRoute.Devices)).toBe('deviceOps');
    expect(parentPortalManageLaneForRoute(PortalRoute.PolicyTracking)).toBe('childPolicy');
    expect(parentPortalManageLaneForRoute(PortalRoute.Overview)).toBeNull();
  });

  it('parses portal hash routes and hydrates normalized current-route lane context', () => {
    expect(portalRouteFromHashPath('#/browser?panel=inventory')).toBe(PortalRoute.Browser);
    expect(portalRouteFromHashPath('#/notification-channels?panel=alerts')).toBe(PortalRoute.NotificationChannels);
    expect(portalRouteFromHashPath('#/subscription')).toBe(PortalRoute.Subscription);
    expect(portalRouteFromHashPath('#/unknown')).toBeNull();

    expectPortalRouteContext(PortalRoute.Browser, 'managed-web', 'childPolicy');
    expectPortalRouteContext(PortalRoute.Notifications, 'notifications', 'portal');
    expectPortalRouteContext(PortalRoute.Subscription, 'subscription-plans', 'portal');
    expectPortalRouteContext(PortalRoute.Devices, 'lan-pairing', 'deviceOps');
    expectPortalRouteContext(PortalRoute.PolicyTracking, 'policy-tracking', 'childPolicy');
    expect(parentPortalRouteContext(PortalRoute.Overview).manageLane).toBeNull();
  });
});

function expectPortalRouteContext(route: PortalRoute, selectedControlId: string, manageLane: string) {
  expect(parentPortalRouteContext(route)).toMatchObject({
    selectedControlId,
    manageLane,
  });
}

function snapshotRow(label: string, primaryArea: string): ParentPortalRow {
  return {
    id: `row-${label.toLowerCase().replace(/[^a-z0-9]+/g, '-')}`,
    label,
    primaryArea,
    detail: `${label} detail`,
    tone: 'cyan',
    trend: 'ready',
    readyCount: 1,
    gapCount: 0,
    secondaryArea: null,
    updatedAtLabel: null,
    route: null,
    source: 'api',
  };
}
