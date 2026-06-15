import {
  PARENT_PORTAL_CONTROL_GUIDES,
  PARENT_PORTAL_GUIDE_QUERY,
  PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES,
  PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS,
} from './parent-portal-guide-controls';
import { PARENT_PORTAL_INSIGHT_GUIDES } from './parent-portal-guide-insight';
import { PARENT_PORTAL_OPERATION_GUIDES } from './parent-portal-guide-operations';
import { PARENT_PORTAL_PRIVACY_GUIDES } from './parent-portal-guide-privacy';
import { PARENT_PORTAL_START_GUIDES } from './parent-portal-guide-start';
import { PARENT_PORTAL_API_GUIDES } from './parent-portal-guide-api';
import type { ParentPortalGuideNote, ParentPortalGuidePage, ParentPortalGuideTopic } from './parent-portal-guide-types';

export type { ParentPortalGuideNote, ParentPortalGuidePage, ParentPortalGuideTopic };
export { PARENT_PORTAL_GUIDE_QUERY, PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES, PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS };

export const PARENT_PORTAL_GUIDE_TOPICS: readonly ParentPortalGuideTopic[] = [
  ...PARENT_PORTAL_START_GUIDES,
  ...PARENT_PORTAL_CONTROL_GUIDES,
  ...PARENT_PORTAL_INSIGHT_GUIDES,
  ...PARENT_PORTAL_API_GUIDES,
  ...PARENT_PORTAL_PRIVACY_GUIDES,
  ...PARENT_PORTAL_OPERATION_GUIDES,
] as const;
