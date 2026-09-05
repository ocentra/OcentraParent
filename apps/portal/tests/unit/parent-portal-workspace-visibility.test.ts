import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { parentPortalWorkspaceIsVisible } from '../../src/parent-portal-workspace-visibility';

describe('parent portal workspace visibility', () => {
  it('reserves the primary workspace for dedicated capability, Browser, Network, Tracking, and Screen surfaces', () => {
    expect([
      parentPortalWorkspaceIsVisible(ParentRoute.CapabilityStatus),
      parentPortalWorkspaceIsVisible(ParentRoute.Browser),
      parentPortalWorkspaceIsVisible(ParentRoute.BrowserSettings),
      parentPortalWorkspaceIsVisible(ParentRoute.NetworkActivity),
      parentPortalWorkspaceIsVisible(ParentRoute.PolicyApps),
      parentPortalWorkspaceIsVisible(ParentRoute.PolicyGames),
      parentPortalWorkspaceIsVisible(ParentRoute.PolicyScreen),
      parentPortalWorkspaceIsVisible(ParentRoute.PolicyRemoteScreen),
      parentPortalWorkspaceIsVisible(ParentRoute.RemoteAccess),
      parentPortalWorkspaceIsVisible(ParentRoute.PolicyTracking),
      parentPortalWorkspaceIsVisible(ParentRoute.ScreenAnalysis),
      parentPortalWorkspaceIsVisible(ParentRoute.SettingsRules),
    ]).toEqual([false, false, false, false, false, false, false, false, false, false, false, true]);
  });

  it('reserves policy authoring routes for their service-backed preview or unavailable surface', () => {
    expect([
      parentPortalWorkspaceIsVisible(ParentRoute.RuleManagement),
      parentPortalWorkspaceIsVisible(ParentRoute.Schedules),
      parentPortalWorkspaceIsVisible(ParentRoute.Approvals),
      parentPortalWorkspaceIsVisible(ParentRoute.Enforcement),
    ]).toEqual([false, false, false, false]);
  });

  it('reserves desktop distribution routes for their dedicated package status surface', () => {
    expect([
      parentPortalWorkspaceIsVisible(ParentRoute.PlatformsInstall),
      parentPortalWorkspaceIsVisible(ParentRoute.InstallUpdates),
    ]).toEqual([false, false]);
  });

  it('keeps unrelated product workspaces visible', () => {
    expect(parentPortalWorkspaceIsVisible(ParentRoute.Overview)).toBe(true);
  });
});
