import type { ParentLeaderboardCopyTone } from './parent-leaderboard-copy-data';

export type ParentLeaderboardCopyGuidePage = {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly steps: readonly string[];
};

export type ParentLeaderboardCopyGuideNote = {
  readonly label: string;
  readonly body: string;
  readonly tone: ParentLeaderboardCopyTone;
  readonly targetPage?: number;
  readonly targetTopicId?: string;
  readonly targetNavLabel?: string;
  readonly targetRoutePath?: string;
};

export type ParentLeaderboardCopyGuideTopic = {
  readonly id: string;
  readonly navLabel: string;
  readonly rank: number;
  readonly title: string;
  readonly subtitle: string;
  readonly detail: string;
  readonly tone: ParentLeaderboardCopyTone;
  readonly category: string;
  readonly subcategory: string;
  readonly pages: readonly ParentLeaderboardCopyGuidePage[];
  readonly tips: readonly ParentLeaderboardCopyGuideNote[];
  readonly actions: readonly ParentLeaderboardCopyGuideNote[];
};
