import type { ParentPortalTone } from './parent-portal-data';

export type ParentPortalGuidePage = {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly steps: readonly string[];
};

export type ParentPortalGuideNote = {
  readonly label: string;
  readonly body: string;
  readonly tone: ParentPortalTone;
  readonly targetPage?: number;
  readonly targetTopicId?: string;
  readonly targetNavLabel?: string;
  readonly targetRoutePath?: string;
};

export type ParentPortalGuideTopic = {
  readonly id: string;
  readonly navLabel: string;
  readonly rank: number;
  readonly title: string;
  readonly subtitle: string;
  readonly detail: string;
  readonly tone: ParentPortalTone;
  readonly category: string;
  readonly subcategory: string;
  readonly pages: readonly ParentPortalGuidePage[];
  readonly tips: readonly ParentPortalGuideNote[];
  readonly actions: readonly ParentPortalGuideNote[];
};
