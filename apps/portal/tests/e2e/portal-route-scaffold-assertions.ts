import { type Page } from '@playwright/test';
import { PARENT_PORTAL_NAV_LABELS } from '@ocentra-parent/portal-domain/parent-portal-nav';
import {
  assertAssistantEntryAvailable,
  assertDuplicateLabelSidePanelRoutes,
  assertFrameTunerRoute,
  assertManageTargetSelectorSemantics,
  assertPolicyGuideDeepLinks,
  assertSidePanelFoldouts,
  assertSupportContactRoute,
} from './portal-route-scaffold-common';
import { assertLanRouteSurface } from './portal-route-scaffold-lan';
import { assertProductRouteSurface } from './portal-route-scaffold-product';

const productRoutes = [
  { path: '/#/start', nav: 'START HERE', title: 'START HERE', kind: 'guideDashboard' },
  { path: '/#/overview', nav: 'OVERVIEW', title: 'Current device state', kind: 'control' },
  { path: '/#/assistant', nav: 'AI ASSISTANT', title: 'Ask MIA about', kind: 'assistant' },
  { path: '/#/activity', nav: 'ACTIVITY', title: 'REPORTS CONTROL DETAIL', kind: 'activityManage' },
  { path: '/#/browser', nav: 'WEB', title: 'MANAGED WEB CONTROL DETAIL', kind: 'control' },
  {
    path: '/#/browser-settings',
    nav: PARENT_PORTAL_NAV_LABELS.Browser,
    title: 'BROWSER SETUP CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/policy', nav: 'RULES', title: 'Rules', kind: 'guide' },
  {
    path: '/#/rule-management',
    nav: PARENT_PORTAL_NAV_LABELS.RuleSet,
    title: 'RULES CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/schedules', nav: 'SCHEDULES', title: 'SCHEDULES CONTROL DETAIL', kind: 'manage' },
  { path: '/#/approvals', nav: 'APPROVALS', title: 'APPROVALS CONTROL DETAIL', kind: 'manage' },
  { path: '/#/enforcement', nav: 'ENFORCE', title: 'ENFORCEMENT CONTROL DETAIL', kind: 'manage' },
  { path: '/#/privacy-design', nav: 'PRIVATE', title: 'PRIVACY AND DATA GUIDE', kind: 'guide' },
  { path: '/#/memory', nav: 'MEMORY', title: 'CITED MEMORY GUIDE', kind: 'guide' },
  {
    path: '/#/memory-settings',
    nav: PARENT_PORTAL_NAV_LABELS.MemorySet,
    title: 'MEMORY SETUP CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/ai-guide', nav: 'AI', title: 'AI AND EVIDENCE GUIDE', kind: 'guide' },
  { path: '/#/ai-runtime', nav: 'AI SETUP', title: 'AI SETUP CONTROL DETAIL', kind: 'manage' },
  { path: '/#/api-providers', nav: 'API KEYS', title: 'API PROVIDERS CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/reports-guide',
    nav: PARENT_PORTAL_NAV_LABELS.ReportsGuide,
    title: 'REPORTS AND SUMMARIES GUIDE',
    kind: 'guide',
  },
  {
    path: '/#/screen-analysis',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'SCREEN ANALYSIS CONTROL DETAIL',
    kind: 'activityManage',
  },
  {
    path: '/#/app-game-sessions',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'APP USE CONTROL DETAIL',
    kind: 'activityManage',
  },
  {
    path: '/#/network-activity',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'NETWORK ACTIVITY CONTROL DETAIL',
    kind: 'activityManage',
  },
  {
    path: '/#/devices',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'lanPairing',
  },
  {
    path: '/#/lan-pairing',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'lanPairing',
  },
  {
    path: '/#/capability-status',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'lanPairing',
  },
  { path: '/#/notifications', nav: 'ALERTS', title: 'ALERTS CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/notification-channels',
    nav: 'CHANNELS',
    title: 'NOTIFICATION CHANNELS CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/drive-connections', nav: 'DRIVES', title: 'DRIVES CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/export-retention',
    nav: PARENT_PORTAL_NAV_LABELS.Export,
    title: 'EXPORT DELETE RETENTION CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/remote-access',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/report-compiler',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'REPORT COMPILER CONTROL DETAIL',
    kind: 'activityManage',
  },
  { path: '/#/audit-history', nav: 'AUDIT', title: 'AUDIT HISTORY CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/subscription',
    nav: PARENT_PORTAL_NAV_LABELS.Plan,
    title: 'SUBSCRIPTION CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/entitlements',
    nav: PARENT_PORTAL_NAV_LABELS.Access,
    title: 'ENTITLEMENTS CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/platforms-install',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/install-updates',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/diagnostics', nav: 'SUPPORT', title: 'SUPPORT CONTROL DETAIL', kind: 'manage' },
  { path: '/#/settings-rules', nav: 'SETTINGS', title: 'FAMILY SETTINGS CONTROL DETAIL', kind: 'manage' },
] as const;

const lanRelevantScaffoldPaths = new Set([
  '/#/activity',
  '/#/browser',
  '/#/browser-settings',
  '/#/network-activity',
  '/#/devices',
  '/#/lan-pairing',
  '/#/capability-status',
]);

export async function assertRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    await assertProductRouteSurface(page, route.path, route.nav, route.title, route.kind);
  }
  await assertSidePanelFoldouts(page);
  await assertDuplicateLabelSidePanelRoutes(page);
  await assertPolicyGuideDeepLinks(page);
  await assertManageTargetSelectorSemantics(page);
  await assertSupportContactRoute(page);
  await assertAssistantEntryAvailable(page);
  await assertFrameTunerRoute(page);
}

export async function assertLanRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    if (!lanRelevantScaffoldPaths.has(route.path)) {
      continue;
    }
    await assertProductRouteSurface(page, route.path, route.nav, route.title, route.kind);
  }
  await assertLanRouteSurface(page);
}
