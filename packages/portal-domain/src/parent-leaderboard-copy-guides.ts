export type {
  ParentLeaderboardCopyGuideNote,
  ParentLeaderboardCopyGuidePage,
  ParentLeaderboardCopyGuideTopic,
} from './parent-leaderboard-copy-guide-types';
import { PARENT_LEADERBOARD_COPY_CONTROL_GUIDES } from './parent-leaderboard-copy-guide-controls';
import { PARENT_LEADERBOARD_COPY_INSIGHT_GUIDES } from './parent-leaderboard-copy-guide-insight';
import { PARENT_LEADERBOARD_COPY_OPERATION_GUIDES } from './parent-leaderboard-copy-guide-operations';
import { PARENT_LEADERBOARD_COPY_PRIVACY_GUIDES } from './parent-leaderboard-copy-guide-privacy';
import { PARENT_LEADERBOARD_COPY_START_GUIDES } from './parent-leaderboard-copy-guide-start';
import { PARENT_LEADERBOARD_COPY_API_GUIDES } from './parent-leaderboard-copy-guide-api';
import type { ParentLeaderboardCopyGuideTopic } from './parent-leaderboard-copy-guide-types';

export const PARENT_LEADERBOARD_COPY_GUIDE_TOPICS: readonly ParentLeaderboardCopyGuideTopic[] = [
  ...PARENT_LEADERBOARD_COPY_START_GUIDES,
  ...PARENT_LEADERBOARD_COPY_CONTROL_GUIDES,
  ...PARENT_LEADERBOARD_COPY_INSIGHT_GUIDES,
  ...PARENT_LEADERBOARD_COPY_API_GUIDES,
  ...PARENT_LEADERBOARD_COPY_PRIVACY_GUIDES,
  ...PARENT_LEADERBOARD_COPY_OPERATION_GUIDES,
] as const;
