import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_BROWSER_CONTROL_GUIDE } from './parent-portal-guide-controls-browser';
import { PARENT_PORTAL_ENFORCEMENT_CONTROL_GUIDE } from './parent-portal-guide-controls-enforcement';
import { PARENT_PORTAL_MONITORING_EVIDENCE_GUIDE } from './parent-portal-guide-controls-monitoring';
import { PARENT_PORTAL_RULES_POLICY_GUIDE } from './parent-portal-guide-controls-rules';

export const PARENT_PORTAL_GUIDE_QUERY = {
  Topic: 'guideTopic',
  Page: 'guidePage',
} as const;

export const PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS = {
  Overview: 'rules-policy',
  Browser: 'browser-policy-guide',
  Apps: 'apps-policy-guide',
  Games: 'games-policy-guide',
  ScreenNetwork: 'screen-network-policy-guide',
  Tracking: 'tracking-policy-guide',
  Enforcement: 'enforcement-control',
} as const;

export const PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES = {
  Rules: 0,
  Schedule: 1,
  Budget: 2,
  Approvals: 3,
  Audit: 4,
} as const;

export const PARENT_PORTAL_CONTROL_GUIDES: readonly ParentPortalGuideTopic[] = [
  PARENT_PORTAL_BROWSER_CONTROL_GUIDE,
  PARENT_PORTAL_MONITORING_EVIDENCE_GUIDE,
  PARENT_PORTAL_RULES_POLICY_GUIDE,
  PARENT_PORTAL_ENFORCEMENT_CONTROL_GUIDE,
];
