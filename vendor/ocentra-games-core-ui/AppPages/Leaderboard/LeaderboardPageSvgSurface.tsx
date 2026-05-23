// @ts-nocheck
import {
  useEffect,
  useId,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent,
  type MouseEvent,
  type ReactElement,
  type ReactNode,
  type WheelEvent,
} from 'react';
import { avatarImageUrls, defaultAvatarImageUrl } from '@ocentra/app-assets/avatars';
import { getPlaceholderImageUrl, placeholderImageCount } from '@ocentra/app-assets/placeholders';
import {
  bannerAIBenchmarkingImageUrl,
  bannerGlobalLeaderboardImageUrl,
  bannerPlayYourWayImageUrl,
} from '@ocentra/app-assets/banners';
import { cardGameBackCardImageUrl } from '@ocentra/app-assets/cardgame';
import {
  shopPageCardBackImageUrl,
  shopPageDecksImageUrl,
  shopPageEliteCrownImageUrl,
  shopPageEventBundleImageUrl,
  shopPageInviteFriendImageUrl,
  shopPagePlayersAccessImageUrl,
  shopPageProfileFrameImageUrl,
  shopPagePrivateTableImageUrl,
  shopPageProfileFrameImageUrls,
  shopPagePublicTableImageUrl,
  shopPageRoomChatImageUrl,
  shopPageSeasonPassImageUrl,
  shopPageTableThemesImageUrl,
  shopPageWeeklyCupImageUrl,
} from '@ocentra/app-assets/shop-page';
import {
  normalizeLeaderboardPageSvgControls,
  type LeaderboardPageSvgControls,
} from './LeaderboardPageSvgSurfaceControls';
import {
  normalizeLeaderboardPageContent,
  type LeaderboardGameOption,
  type LeaderboardGuideNote,
  type LeaderboardGuideTopic,
  type LeaderboardIconName,
  type LeaderboardNavGroup,
  type LeaderboardNavItem,
  type LeaderboardPageContentData,
  type LeaderboardQuickGame,
  type LeaderboardTabDetail,
  type LeaderboardTabId,
  type LeaderboardTone,
  type PartialLeaderboardPageContentData,
} from './LeaderboardPageSvgContent';
import {
  createGoldenFrameVariantConfig,
  createGoldenFrameFrameOnlySvgDataUri,
} from './LeaderboardGoldenFrameForeignObject';
import {
  getPictureViewerAnchoredFrame,
  getPictureViewerFrameGroupTransform,
  getPictureViewerFrameTransform,
  normalizePictureViewerFrameControls,
  pictureViewerDarkenHex,
  pictureViewerFrameSegmentThickness,
  pictureViewerFrameSegments,
  type PictureViewerFrameControls,
  type PictureViewerFrameSegment,
} from '../../Common/PictureViewerFrame/PictureViewerFrameControls';
import './LeaderboardPageSvgSurface.css';

type IconProps = {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  color?: string;
  strokeWidth?: number;
};

type IconComponent = (props: IconProps) => ReactElement;
type Tone = LeaderboardTone;
type DetailMode = 'player' | 'game' | 'season';
type LeaderboardFocusSection = 'podium' | 'table';
type LeaderboardGameBrowserView = 'grid' | 'list';
type LeaderboardGameBrowserSort = 'rank' | 'az';
type LeaderboardTopCardItem =
  | {
      kind: 'leader';
      key: string;
      row: DisplayRow;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    }
  | {
      kind: 'game';
      key: string;
      game: TopGame | QuickGame;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    }
  | {
      kind: 'guide';
      key: string;
      topic: LeaderboardGuideTopic;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    };
type TournamentAd = {
  id: string;
  label: string;
  title: string;
  dateRange: string;
  detail: string;
  imageUrl: string;
  tone: Tone;
};

export type LeaderboardPageMode = 'leaderboard' | 'gameLeaderboard' | 'aiBenchmarkLeaderboard';

export type LeaderboardPageRow = {
  user_id: string;
  rank: number;
  score: number;
  wins?: number;
  losses?: number;
  bestGame?: string;
  trend?: string;
  tone?: Tone;
};

type LeaderboardPageSvgSurfaceProps = {
  pageMode: LeaderboardPageMode;
  gameType: number;
  seasonId: string;
  lastUpdated: string;
  leaderboardEntries: LeaderboardPageRow[];
  userEntry: LeaderboardPageRow | null;
  nearbyAbove: LeaderboardPageRow[];
  nearbyBelow: LeaderboardPageRow[];
  gameId?: string;
  loading?: boolean;
  error?: string | null;
  controls?: Partial<LeaderboardPageSvgControls> | null;
  content?: PartialLeaderboardPageContentData | null;
  initialNavLabel?: string;
  initialSelectedGameId?: string;
  onRefreshLeaderboard: (gameType: number) => void;
  onMatchmaking: () => void;
  onNavigate?: (routePath: string) => void;
};

type NavItem = Omit<LeaderboardNavItem, 'icon'> & {
  icon: IconComponent;
  imageUrl: string;
};

type NavGroup = LeaderboardNavGroup & {
  items: NavItem[];
};

type TabDetail = LeaderboardTabDetail;

type DisplayRow = {
  id: string;
  rank: number;
  player: string;
  rating: string;
  games: string;
  wins: string;
  winRate: string;
  bestGame: string;
  trend: string;
  tone: Tone;
};

type LeaderboardTableVariant = 'players' | 'games' | 'ai' | 'events' | 'social' | 'creators';

type TopGame = LeaderboardGameOption;

type QuickGame = Omit<LeaderboardQuickGame, 'icon'> & {
  icon: IconComponent;
};

type SelectableGame = LeaderboardGameOption | LeaderboardQuickGame;

type GameCategorySummary = {
  id: string;
  label: string;
  detail: string;
  count: number;
  tone: Tone;
  sampleGame: QuickGame;
  subcategories: GameSubcategorySummary[];
};

type GameSubcategorySummary = {
  id: string;
  label: string;
  count: number;
  tone: Tone;
  sampleGame: QuickGame;
};

const LEADERBOARD_RESPONSIVE_MIN_LEFT_W = 210;
const LEADERBOARD_RESPONSIVE_MIN_RIGHT_W = 250;
const LEADERBOARD_RESPONSIVE_MIN_MAIN_W = 560;
const LEADERBOARD_RESPONSIVE_COMPACT_SURFACE_W = 1600;
const LEADERBOARD_RESPONSIVE_MAX_CANVAS_W = 8192;
const LEADERBOARD_RESPONSIVE_MAX_CANVAS_H = 2800;
const LEADERBOARD_TOP_CAROUSEL_MAX_VISIBLE = 5;
const LEADERBOARD_SIDE_HANDLE_W = 15;
const LEADERBOARD_SIDE_HANDLE_OVERLAP = 1;
const LEADERBOARD_CATEGORY_LABELS = [
  'Browser',
  'Policy',
  'Activity',
  'Privacy',
  'Memory',
  'AI',
  'Devices',
  'Support',
] as const;

const leaderboardTableColumnLabels: Record<
  LeaderboardTableVariant,
  [string, string, string, string, string, string, string, string, string]
> = {
  players: ['#', 'CONTROL', 'READINESS', 'CHECKS', 'READY', 'STATE', 'AREA', 'SIGNALS', 'NEXT'],
  games: ['#', 'CONTROL', 'READINESS', 'CHECKS', 'READY', 'STATE', 'AREA', 'SIGNALS', 'NEXT'],
  ai: ['#', 'LOCAL AI', 'READINESS', 'RUNS', 'READY', 'STATE', 'BOUNDARY', 'SIGNALS', 'NEXT'],
  events: ['#', 'ROUTINE', 'READINESS', 'CHECKS', 'READY', 'STATE', 'DEVICE', 'SIGNALS', 'NEXT'],
  social: ['#', 'SUPPORT', 'READINESS', 'CHECKS', 'READY', 'STATE', 'CHANNEL', 'SIGNALS', 'NEXT'],
  creators: ['#', 'OWNER', 'READINESS', 'CHECKS', 'READY', 'STATE', 'SURFACE', 'SIGNALS', 'NEXT'],
};

function clampValue(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function wrapIndex(value: number, length: number): number {
  if (length <= 0) {
    return 0;
  }
  return ((value % length) + length) % length;
}

function fitSingleLineTextSize(text: string, width: number, min: number, max: number, factor = 0.56): number {
  if (!text) return max;
  return clampValue(width / Math.max(1, text.length * factor), min, max);
}

function truncateTextForWidth(text: string, width: number, fontSize: number, factor = 0.56): string {
  const maxChars = Math.max(1, Math.floor(width / Math.max(1, fontSize * factor)));
  if (text.length <= maxChars) return text;
  if (maxChars <= 3) return text.slice(0, maxChars);
  return `${text.slice(0, maxChars - 3).trimEnd()}...`;
}

function compactGameStatLabel(value: string): string {
  const text = value.trim().replace(/\s+/g, ' ');
  if (!text) return '';
  if (/^\d+(?:\.\d+)?\s*[kmb]$/i.test(text) || /^[+-]?\d+(?:\.\d+)?%$/.test(text)) return text;
  const range = text.match(/\b\d+\s*[-–]\s*\d+\b/);
  if (range) return range[0].replace(/\s+/g, '');
  const number = text.match(/\b\d+\b/);
  return number ? number[0] : text;
}

function compactGameGrowthLabel(value: string): string {
  const text = value.trim().replace(/\s+/g, ' ');
  if (!text) return '';
  if (/^[+-]?\d+(?:\.\d+)?%$/.test(text)) return text;
  const minuteRange = text.match(/\b(\d+\s*[-–]\s*\d+)\s*(?:min|mins|minute|minutes)\b/i);
  if (minuteRange) return `${minuteRange[1].replace(/\s+/g, '')}m`;
  const minutes = text.match(/\b(\d+)\s*(?:min|mins|minute|minutes)\b/i);
  if (minutes) return `${minutes[1]}m`;
  return compactGameStatLabel(text);
}

function tournamentAdsForSeason(season: LeaderboardPageContentData['season']): TournamentAd[] {
  const prizePool = season.stats.find((stat) => assetKey(stat.label).includes('prize'))?.value ?? '2.35M';
  const endsIn = season.stats.find((stat) => assetKey(stat.label).includes('ends'))?.value ?? 'LIVE NOW';
  return [
    {
      id: 'season-track',
      label: season.label,
      title: season.title,
      dateRange: season.dateRange,
      detail: 'GLOBAL SEASON HUB',
      imageUrl: shopPageSeasonPassImageUrl,
      tone: 'gold',
    },
    {
      id: 'weekly-cup',
      label: 'WEEKLY CUP',
      title: 'LIVE BRACKETS',
      dateRange: `ENDS ${endsIn}`,
      detail: 'TOURNAMENT LEADERS',
      imageUrl: shopPageWeeklyCupImageUrl,
      tone: 'cyan',
    },
    {
      id: 'arena-events',
      label: 'ARENA EVENTS',
      title: 'PRIZE PATHS',
      dateRange: `${prizePool} POOL`,
      detail: 'EVENT ROTATION',
      imageUrl: shopPageEventBundleImageUrl,
      tone: 'purple',
    },
    {
      id: 'elite-series',
      label: 'ELITE SERIES',
      title: 'TOP 100 PUSH',
      dateRange: 'WEEKEND LADDER',
      detail: 'CHAMPION TRACK',
      imageUrl: shopPageEliteCrownImageUrl,
      tone: 'gold',
    },
  ];
}

function minimumLeaderboardCanvasWidth(cfg: LeaderboardPageSvgControls): number {
  return Math.max(
    cfg.canvas.width,
    Math.ceil(
      cfg.layout.outerPad * 2 +
        LEADERBOARD_RESPONSIVE_MIN_LEFT_W +
        LEADERBOARD_RESPONSIVE_MIN_RIGHT_W +
        LEADERBOARD_RESPONSIVE_MIN_MAIN_W +
        cfg.layout.gap * 2
    )
  );
}

function leaderboardCanvasWidthForSurface(
  cfg: LeaderboardPageSvgControls,
  surfaceSize: { width: number; height: number }
): number {
  if (surfaceSize.width <= 0 || surfaceSize.height <= 0) return cfg.canvas.width;
  const minimumWidth = minimumLeaderboardCanvasWidth(cfg);
  const ratioWidth = Math.round(cfg.canvas.height * (surfaceSize.width / surfaceSize.height));
  return Math.max(minimumWidth, Math.min(LEADERBOARD_RESPONSIVE_MAX_CANVAS_W, ratioWidth));
}

function compactLeaderboardCanvasWidth(cfg: LeaderboardPageSvgControls): number {
  return Math.ceil(cfg.layout.outerPad * 2 + cfg.layout.leftW + cfg.layout.gap + LEADERBOARD_RESPONSIVE_MIN_MAIN_W);
}

function leaderboardCanvasSizeForSurface(
  cfg: LeaderboardPageSvgControls,
  surfaceSize: { width: number; height: number }
): { width: number; height: number } {
  if (surfaceSize.width <= 0 || surfaceSize.height <= 0) {
    return cfg.canvas;
  }

  if (surfaceSize.width >= LEADERBOARD_RESPONSIVE_COMPACT_SURFACE_W) {
    return {
      width: leaderboardCanvasWidthForSurface(cfg, surfaceSize),
      height: cfg.canvas.height,
    };
  }

  const compactWidth = compactLeaderboardCanvasWidth(cfg);
  const aspectWidth = Math.round(cfg.canvas.height * (surfaceSize.width / surfaceSize.height));
  const width = clampValue(aspectWidth, compactWidth, cfg.canvas.width);
  const aspectHeight = Math.round(width * (surfaceSize.height / surfaceSize.width));

  return {
    width,
    height: clampValue(aspectHeight, cfg.canvas.height, LEADERBOARD_RESPONSIVE_MAX_CANVAS_H),
  };
}

function responsiveLeaderboardColumnWidths(
  canvasWidth: number,
  cfg: LeaderboardPageSvgControls
): { leftW: number; mainW: number; rightW: number } {
  const availableW = canvasWidth - cfg.layout.outerPad * 2 - cfg.layout.gap;
  const leftW = clampValue(cfg.layout.leftW, LEADERBOARD_RESPONSIVE_MIN_LEFT_W, cfg.layout.leftW);
  const mainW = Math.max(LEADERBOARD_RESPONSIVE_MIN_MAIN_W, availableW - leftW);
  return { leftW, mainW, rightW: 0 };
}

function IconSvg({
  x = 0,
  y = 0,
  width = 24,
  height = 24,
  color = 'currentColor',
  strokeWidth = 2,
  children,
}: IconProps & { children: ReactNode }) {
  return (
    <svg
      x={x}
      y={y}
      width={width}
      height={height}
      viewBox="0 0 24 24"
      fill="none"
      stroke={color}
      strokeWidth={strokeWidth}
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      {children}
    </svg>
  );
}

const Activity = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 3 12 H 7 L 10 4 L 14 20 L 17 12 H 21" />
  </IconSvg>
);
const Bot = (props: IconProps) => (
  <IconSvg {...props}>
    <rect x="5" y="8" width="14" height="10" rx="3" />
    <path d="M 12 8 V 4" />
    <circle cx="9" cy="13" r="1.2" />
    <circle cx="15" cy="13" r="1.2" />
    <path d="M 9 17 H 15" />
  </IconSvg>
);
const CalendarDays = (props: IconProps) => (
  <IconSvg {...props}>
    <rect x="4" y="5" width="16" height="15" rx="2" />
    <path d="M 8 3 V 7 M 16 3 V 7 M 4 10 H 20 M 8 14 H 9 M 12 14 H 13 M 16 14 H 17 M 8 17 H 9 M 12 17 H 13" />
  </IconSvg>
);
const ChevronRight = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 9 5 L 16 12 L 9 19" />
  </IconSvg>
);
const CircleDot = (props: IconProps) => (
  <IconSvg {...props}>
    <circle cx="12" cy="12" r="8" />
    <circle cx="12" cy="12" r="2" fill={props.color ?? 'currentColor'} stroke="none" />
  </IconSvg>
);
const Coins = (props: IconProps) => (
  <IconSvg {...props}>
    <ellipse cx="12" cy="6" rx="7" ry="3" />
    <path d="M 5 6 V 12 C 5 13.7 8.1 15 12 15 C 15.9 15 19 13.7 19 12 V 6" />
    <path d="M 5 12 V 18 C 5 19.7 8.1 21 12 21 C 15.9 21 19 19.7 19 18 V 12" />
  </IconSvg>
);
const Crown = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 4 18 H 20 L 18 8 L 14 12 L 12 5 L 10 12 L 6 8 Z" />
    <path d="M 7 21 H 17" />
  </IconSvg>
);
const Gamepad2 = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 7 9 H 17 C 20 9 22 12 21 16 L 20.5 18 C 20 20 18 20.5 16.5 18.8 L 14.8 17 H 9.2 L 7.5 18.8 C 6 20.5 4 20 3.5 18 L 3 16 C 2 12 4 9 7 9 Z" />
    <path d="M 8 12 V 16 M 6 14 H 10" />
    <circle cx="16" cy="13" r="1" />
    <circle cx="18" cy="16" r="1" />
  </IconSvg>
);
const Gift = (props: IconProps) => (
  <IconSvg {...props}>
    <rect x="4" y="10" width="16" height="10" rx="1" />
    <path d="M 4 14 H 20 M 12 10 V 20 M 12 10 C 9 8 8 6 9.5 5 C 11 4 12 7 12 10 Z M 12 10 C 15 8 16 6 14.5 5 C 13 4 12 7 12 10 Z" />
  </IconSvg>
);
const Grid3X3 = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 4 4 H 9 V 9 H 4 Z M 15 4 H 20 V 9 H 15 Z M 4 15 H 9 V 20 H 4 Z M 15 15 H 20 V 20 H 15 Z M 10 10 H 14 V 14 H 10 Z" />
  </IconSvg>
);
const Home = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 4 11 L 12 4 L 20 11" />
    <path d="M 6 10 V 20 H 18 V 10" />
    <path d="M 10 20 V 14 H 14 V 20" />
  </IconSvg>
);
const Medal = (props: IconProps) => (
  <IconSvg {...props}>
    <circle cx="12" cy="14" r="5" />
    <path d="M 8 3 L 12 9 L 16 3 M 9 3 H 15" />
    <path d="M 12 12 L 13 14 L 15 14.3 L 13.5 15.8 L 13.8 18 L 12 17 L 10.2 18 L 10.5 15.8 L 9 14.3 L 11 14 Z" />
  </IconSvg>
);
const Shield = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 12 3 L 19 6 V 11 C 19 16 16 19.5 12 21 C 8 19.5 5 16 5 11 V 6 Z" />
  </IconSvg>
);
const Swords = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 5 19 L 19 5 M 14 5 H 19 V 10 M 4 14 L 10 20 M 6 20 L 10 16" />
    <path d="M 19 19 L 5 5 M 10 5 H 5 V 10 M 20 14 L 14 20 M 18 20 L 14 16" />
  </IconSvg>
);
const Trophy = (props: IconProps) => (
  <IconSvg {...props}>
    <path d="M 8 4 H 16 V 10 C 16 13 14 15 12 15 C 10 15 8 13 8 10 Z" />
    <path d="M 8 6 H 4 V 8 C 4 10.5 6 12 8 12 M 16 6 H 20 V 8 C 20 10.5 18 12 16 12 M 12 15 V 20 M 8 20 H 16" />
  </IconSvg>
);
const Users = (props: IconProps) => (
  <IconSvg {...props}>
    <circle cx="9" cy="8" r="3" />
    <circle cx="17" cy="9" r="2.5" />
    <path d="M 3 20 C 3.8 16.5 5.8 14.5 9 14.5 C 12.2 14.5 14.2 16.5 15 20" />
    <path d="M 14.5 15 C 17.5 15.2 19.5 16.8 21 20" />
  </IconSvg>
);
const Clock = (props: IconProps) => (
  <IconSvg {...props}>
    <circle cx="12" cy="12" r="8" />
    <path d="M 12 7 V 12 L 15.5 14.2" />
  </IconSvg>
);

const iconByName: Record<LeaderboardIconName, IconComponent> = {
  activity: Activity,
  bot: Bot,
  calendar: CalendarDays,
  circle: CircleDot,
  coins: Coins,
  crown: Crown,
  gamepad: Gamepad2,
  gift: Gift,
  grid: Grid3X3,
  home: Home,
  medal: Medal,
  shield: Shield,
  swords: Swords,
  trophy: Trophy,
  users: Users,
};

function iconForName(icon: LeaderboardIconName): IconComponent {
  return iconByName[icon] ?? Trophy;
}

function toneColor(tone: Tone, cfg: LeaderboardPageSvgControls): string {
  return cfg.colors[tone];
}

function colorAlpha(color: string, alphaHex: string): string {
  return color.startsWith('#') ? `${color}${alphaHex}` : color;
}

function assetKey(value?: string): string {
  return (value ?? '')
    .trim()
    .toLowerCase()
    .replace(/&/g, 'and')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

function hashString(value: string): number {
  return Array.from(value).reduce((hash, char) => (hash * 31 + char.charCodeAt(0)) >>> 0, 0);
}

function playerAvatarImageUrl(player: string): string {
  const imageIndex = avatarImageUrls.length > 0 ? hashString(player) % avatarImageUrls.length : 0;
  return avatarImageUrls[imageIndex] ?? defaultAvatarImageUrl;
}

function leaderboardGameImageUrl(value?: string): string | null {
  const key = assetKey(value);
  if (!key) return null;
  if (key.includes('ai-benchmark') || key.includes('model')) return bannerAIBenchmarkingImageUrl;
  if (key.includes('leaderboard') || key.includes('quick-access') || key.includes('hub'))
    return bannerGlobalLeaderboardImageUrl;
  if (key.includes('all-games') || key.includes('catalog')) return bannerPlayYourWayImageUrl;
  if (key.includes('season')) return shopPageSeasonPassImageUrl;
  if (key.includes('tournament') || key.includes('event')) return shopPageWeeklyCupImageUrl;
  if (key.includes('three-card-brag') || key.includes('3-card-brag') || key.includes('teen-patti'))
    return shopPageCardBackImageUrl;
  if (key.includes('spades') || key.includes('call-break')) return shopPageDecksImageUrl;
  if (key.includes('poker')) return shopPageTableThemesImageUrl;
  if (key.includes('blackjack')) return shopPagePrivateTableImageUrl;
  if (key.includes('claim')) return shopPagePublicTableImageUrl;
  if (key.includes('rummy')) return cardGameBackCardImageUrl;
  return null;
}

function leaderboardExplorerGameImageUrl(game: Pick<TopGame | QuickGame, 'id' | 'name'>): string {
  const key = game.id || game.name;
  const imageIndex = placeholderImageCount > 0 ? hashString(key) % placeholderImageCount : 0;
  return getPlaceholderImageUrl(imageIndex);
}

function leaderboardCategoryImageUrl(category: GameCategorySummary): string | null {
  return category.count > 0 ? leaderboardExplorerGameImageUrl(category.sampleGame) : null;
}

function leaderboardBadgeImageUrl(tone: Tone, index: number): string {
  if (tone === 'gold') return shopPageEliteCrownImageUrl;
  const imageIndex =
    shopPageProfileFrameImageUrls.length > 0
      ? hashString(`${tone}-${index}`) % shopPageProfileFrameImageUrls.length
      : 0;
  return shopPageProfileFrameImageUrls[imageIndex] ?? shopPageEliteCrownImageUrl;
}

function navItemImageUrl(item: LeaderboardNavItem): string {
  const key = assetKey(item.label);
  if (key.includes('overview')) return bannerGlobalLeaderboardImageUrl;
  if (key.includes('overall') || key.includes('global') || key.includes('player')) return shopPagePlayersAccessImageUrl;
  if (key.includes('ai')) return bannerAIBenchmarkingImageUrl;
  if (key.includes('category')) return bannerPlayYourWayImageUrl;
  if (key.includes('per-game') || key.includes('game')) return shopPageDecksImageUrl;
  if (key.includes('tournament')) return shopPageWeeklyCupImageUrl;
  if (key.includes('friend')) return shopPageInviteFriendImageUrl;
  if (key.includes('guild')) return shopPageRoomChatImageUrl;
  if (key.includes('creator')) return shopPageProfileFrameImageUrl;
  return shopPageEventBundleImageUrl;
}

function cutRectPath(x: number, y: number, w: number, h: number, cut: number) {
  const c = Math.min(cut, w / 2, h / 2);
  return [
    `M ${x + c} ${y}`,
    `H ${x + w - c}`,
    `L ${x + w} ${y + c}`,
    `V ${y + h - c}`,
    `L ${x + w - c} ${y + h}`,
    `H ${x + c}`,
    `L ${x} ${y + h - c}`,
    `V ${y + c}`,
    'Z',
  ].join(' ');
}

function topCutRectPath(x: number, y: number, w: number, h: number, cut: number) {
  const c = Math.min(cut, w / 2, h / 2);
  return [`M ${x + c} ${y}`, `H ${x + w - c}`, `L ${x + w} ${y + c}`, `V ${y + h}`, `H ${x}`, `V ${y + c}`, 'Z'].join(
    ' '
  );
}

function bottomCutRectPath(x: number, y: number, w: number, h: number, cut: number) {
  const c = Math.min(cut, w / 2, h / 2);
  return [
    `M ${x} ${y}`,
    `H ${x + w}`,
    `V ${y + h - c}`,
    `L ${x + w - c} ${y + h}`,
    `H ${x + c}`,
    `L ${x} ${y + h - c}`,
    'Z',
  ].join(' ');
}

function hexPath(cx: number, cy: number, radius: number) {
  return (
    Array.from({ length: 6 }, (_, index) => {
      const angle = Math.PI / 6 + (index * Math.PI) / 3;
      const x = cx + Math.cos(angle) * radius;
      const y = cy + Math.sin(angle) * radius;
      return `${index === 0 ? 'M' : 'L'} ${x.toFixed(1)} ${y.toFixed(1)}`;
    }).join(' ') + ' Z'
  );
}

type LeaderboardRect = { x: number; y: number; w: number; h: number };

function leaderboardFrameRects(
  x: number,
  y: number,
  w: number,
  h: number,
  footerH = 38,
  headerH = 48
): { body: LeaderboardRect; footer: LeaderboardRect; headerH: number; footerH: number } {
  const inset = 18;
  return {
    body: {
      x: x + inset,
      y: y + headerH + 10,
      w: Math.max(1, w - inset * 2),
      h: Math.max(1, h - headerH - footerH - 18),
    },
    footer: {
      x: x + inset,
      y: y + h - footerH,
      w: Math.max(1, w - inset * 2),
      h: footerH,
    },
    headerH,
    footerH,
  };
}

function formatRouteScope(value?: string): string {
  if (!value) return 'All games';
  return value
    .split(/[/:_-]/)
    .filter(Boolean)
    .slice(0, 3)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function formatUpdatedLabel(value: string): string {
  if (!value) return 'Live';
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value.length > 14 ? `${value.slice(0, 11)}...` : value;
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function initialTabForPageMode(pageMode: LeaderboardPageMode, content: LeaderboardPageContentData): LeaderboardTabId {
  return content.modes[pageMode]?.defaultTab ?? 'overall';
}

function initialNavLabelForTab(navItems: NavItem[], tab: LeaderboardTabId): string {
  return navItems.find((item) => item.tabId === tab)?.label ?? navItems[0]?.label ?? '';
}

function groupedLeaderboardNavItems(navGroups: LeaderboardNavGroup[], navItems: NavItem[]): NavGroup[] {
  const groupIds = new Set(navGroups.map((group) => group.id));
  const groups = navGroups.map((group) => ({
    ...group,
    items: navItems.filter((item) => item.groupId === group.id),
  }));
  const ungroupedItems = navItems.filter((item) => !item.groupId || !groupIds.has(item.groupId));
  if (ungroupedItems.length === 0) return groups.filter((group) => group.items.length > 0);
  return [
    ...groups.filter((group) => group.items.length > 0),
    {
      id: 'menu',
      label: 'MENU',
      detail: 'Navigation',
      items: ungroupedItems,
    },
  ];
}

function navGroupIdForNavLabel(navGroups: NavGroup[], navLabel: string): string {
  return navGroups.find((group) => group.items.some((item) => item.label === navLabel))?.id ?? navGroups[0]?.id ?? '';
}

function initialOpenNavGroupIds(navGroups: NavGroup[], navLabel: string): Record<string, boolean> {
  const activeGroupId = navGroupIdForNavLabel(navGroups, navLabel);
  return Object.fromEntries(
    navGroups.map((group, index) => [group.id, group.id === activeGroupId || (!activeGroupId && index === 0)])
  );
}

function ensureOpenNavGroupIds(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  navLabel: string
): Record<string, boolean> {
  const activeGroupId = navGroupIdForNavLabel(navGroups, navLabel);
  return Object.fromEntries(
    navGroups.map((group) => [group.id, Boolean(current[group.id]) || group.id === activeGroupId])
  );
}

function toggleOpenNavGroupId(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  groupId: string
): Record<string, boolean> {
  return Object.fromEntries(
    navGroups.map((group) => [group.id, group.id === groupId ? !current[group.id] : Boolean(current[group.id])])
  );
}

function normalizeSelectionId(value?: string): string {
  return (value ?? '').trim().toLowerCase();
}

function findSelectedGame(content: LeaderboardPageContentData, selectedGameId: string): SelectableGame | undefined {
  const normalizedId = normalizeSelectionId(selectedGameId);
  return [...content.topGames, ...content.quickGames.filter(isLeaderboardGameEntry)].find(
    (game) => normalizeSelectionId(game.id) === normalizedId
  );
}

function initialGameIdForPageMode(
  pageMode: LeaderboardPageMode,
  content: LeaderboardPageContentData,
  gameId?: string
): string {
  const routeId = normalizeSelectionId(gameId);
  const contentGames = content.quickGames.filter(isLeaderboardGameEntry);
  const routeGame = [...content.topGames, ...contentGames].find((game) => normalizeSelectionId(game.id) === routeId);
  if (routeGame) return routeGame.id;
  return content.modes[pageMode]?.selectedGameId ?? content.topGames[0]?.id ?? contentGames[0]?.id ?? '';
}

function isLeaderboardGameEntry(game: Pick<LeaderboardQuickGame, 'id' | 'name' | 'routePath'>): boolean {
  const id = normalizeSelectionId(game.id);
  const name = normalizeSelectionId(game.name);
  if (id === 'leaderboard-hub' || id === 'quick-access' || id === 'all-games') return false;
  if (name === 'quick-access' || name === 'all-games') return false;
  if (game.routePath === '/leaderboard' || game.routePath === '/games/card-games') return false;
  return true;
}

function isHashRoutePath(routePath?: string): routePath is string {
  return typeof routePath === 'string' && routePath.startsWith('#/');
}

function rowSourceForPageMode(
  content: LeaderboardPageContentData,
  pageMode: LeaderboardPageMode,
  leaderboardRows: LeaderboardPageRow[]
): LeaderboardPageRow[] {
  const source = content.modes[pageMode]?.rowSource ?? 'api';
  if (source === 'aiBenchmarkRows') return content.aiBenchmarkRows;
  if (source === 'fallbackRows') return content.fallbackRows;
  return leaderboardRows.length > 0 ? leaderboardRows : content.fallbackRows;
}

function toDisplayRows(
  rows: LeaderboardPageRow[],
  pageMode: LeaderboardPageMode,
  selectedGameName: string,
  gameId?: string
): DisplayRow[] {
  const bestGame = pageMode === 'gameLeaderboard' ? selectedGameName || formatRouteScope(gameId) : 'Mixed';
  return rows.map((row, index) => {
    const wins = row.wins ?? Math.max(0, Math.round(row.score * 0.32));
    const losses = row.losses ?? Math.max(1, Math.round(wins * 0.62));
    const winRate = wins + losses > 0 ? `${Math.round((wins / (wins + losses)) * 1000) / 10}%` : '-';
    const tones: Tone[] = ['purple', 'red', 'cyan', 'gold', 'purple', 'red', 'cyan'];
    return {
      id: row.user_id || `player-${row.rank}`,
      rank: row.rank,
      player: row.user_id || `Player ${row.rank}`,
      rating: row.score.toLocaleString(),
      games: (wins + losses).toLocaleString(),
      wins: wins.toLocaleString(),
      winRate,
      bestGame: row.bestGame ?? bestGame,
      trend: row.trend ?? (index % 3 === 0 ? '+2' : index % 3 === 1 ? '+1' : '-'),
      tone: row.tone ?? tones[index % tones.length],
    };
  });
}

function tableVariantForContext(activeNavLabel: string, activeTab: LeaderboardTabId): LeaderboardTableVariant {
  const key = assetKey(activeNavLabel);
  if (key.includes('ai') || activeTab === 'aiBenchmarks') return 'ai';
  if (key.includes('friend') || key.includes('guild') || activeTab === 'friends') return 'social';
  if (key.includes('per-game') || key.includes('per-category') || activeTab === 'perGame') return 'games';
  if (key.includes('tournament') || key.includes('season') || key.includes('reward') || activeTab === 'tournaments')
    return 'events';
  return 'players';
}

function tableTitleForVariant(
  variant: LeaderboardTableVariant,
  activeNavLabel: string,
  selectedGameName: string
): string {
  const key = assetKey(activeNavLabel);
  if (key.includes('overview') || key.includes('today')) return 'TODAY CONTROL SNAPSHOT';
  if (variant === 'games') return `${selectedGameName.toUpperCase()} CONTROL DETAIL`;
  if (variant === 'ai') return 'LOCAL AI AND MEMORY READINESS';
  if (variant === 'events') return 'DEVICE ROUTINE AND APPROVALS';
  if (variant === 'social') return 'SUPPORT EXPORTS AND DRIVE CONNECTIONS';
  if (variant === 'creators') return 'PARENT OWNERSHIP DETAIL';
  return 'PARENT CONTROL DETAIL';
}

function tableBestLabelForVariant(
  row: DisplayRow,
  variant: LeaderboardTableVariant,
  activeNavLabel: string,
  selectedGameName: string
): string {
  const key = assetKey(activeNavLabel);
  if (variant === 'games') return selectedGameName;
  if (variant === 'ai') return row.bestGame === 'Mixed' ? 'Local AI' : row.bestGame;
  if (variant === 'events') return key.includes('settings') ? 'Family defaults' : 'Device routine';
  if (variant === 'social') return key.includes('drive') ? 'Drive export' : 'Support';
  return row.bestGame;
}

function tableParticipantLabel(row: DisplayRow, variant: LeaderboardTableVariant): string {
  if (variant === 'ai') return row.player.replace(/-/g, ' ');
  return row.player;
}

function leaderboardScopeKey(value?: string): string {
  return (value ?? '').toLowerCase().replace(/[^a-z0-9]/g, '');
}

function rowsForGameScope(rows: DisplayRow[], selectedGameName: string, selectedGameId: string): DisplayRow[] {
  const gameKeys = new Set(
    [
      leaderboardScopeKey(selectedGameName),
      leaderboardScopeKey(selectedGameId),
      leaderboardScopeKey(selectedGameName.replace(/^three card/i, '3 card')),
      leaderboardScopeKey(selectedGameName.replace(/^3 card/i, 'three card')),
    ].filter(Boolean)
  );
  const matched = rows.filter((row) => gameKeys.has(leaderboardScopeKey(row.bestGame)));
  const source = matched.length > 0 ? matched : rows.slice(0, Math.min(10, rows.length));
  return source.map((row, index) => ({
    ...row,
    rank: index + 1,
    bestGame: selectedGameName || row.bestGame,
  }));
}

function rowsForCategoryScope(rows: DisplayRow[], selectedCategoryLabel: string): DisplayRow[] {
  const categoryKey = assetKey(selectedCategoryLabel);
  const matched = rows.filter(
    (row) => assetKey(gameCategoryLabel({ id: row.bestGame, name: row.bestGame })) === categoryKey
  );
  const source = matched.length > 0 ? matched : rows.slice(0, Math.min(10, rows.length));
  return source.map((row, index) => ({
    ...row,
    rank: index + 1,
    bestGame: selectedCategoryLabel || row.bestGame,
  }));
}

function leaderTopCard(row: DisplayRow): LeaderboardTopCardItem {
  return {
    kind: 'leader',
    key: `leader:${row.id}`,
    row,
    title: row.player,
    subtitle: `Rank ${row.rank} / ${row.bestGame}`,
    value: row.rating,
    detail: `${row.winRate} ready`,
    tone: row.tone,
  };
}

function titleCaseGameName(value: string): string {
  return value
    .toLowerCase()
    .split(' ')
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function gameCategoryLabel(game: Pick<TopGame | QuickGame, 'id' | 'name'> & { category?: string }): string {
  if (game.category) return game.category;
  const key = assetKey(`${game.id} ${game.name}`);
  if (key.includes('browser') || key.includes('web')) return 'Browser';
  if (key.includes('policy') || key.includes('rule')) return 'Policy';
  if (key.includes('activity') || key.includes('evidence')) return 'Activity';
  if (key.includes('privacy') || key.includes('private')) return 'Privacy';
  if (key.includes('memory')) return 'Memory';
  if (key.includes('ai')) return 'AI';
  if (key.includes('device') || key.includes('notification') || key.includes('setting')) return 'Devices';
  if (key.includes('support') || key.includes('export') || key.includes('drive')) return 'Support';
  return 'Controls';
}

function taxonomyLeafLabel(value: string): string {
  const parts = value
    .split('/')
    .map((part) => part.trim())
    .filter(Boolean);
  return parts[parts.length - 1] ?? value;
}

function gameSubcategoryLabel(game: { detail?: string; subcategory?: string | null }): string {
  const value = game.subcategory?.trim() || game.detail?.trim();
  return value ? taxonomyLeafLabel(value) : 'General';
}

function gameTopCard(game: TopGame | QuickGame, statsGame?: TopGame, index = 0): LeaderboardTopCardItem {
  const title = game.name === game.name.toUpperCase() ? titleCaseGameName(game.name) : game.name;
  const value = 'matches' in game ? game.matches : (statsGame?.matches ?? game.detail);
  const detail = 'growth' in game ? game.growth : (statsGame?.growth ?? game.subcategory ?? 'View ladder');
  return {
    kind: 'game',
    key: `game:${normalizeSelectionId(game.id)}:${assetKey(game.name)}:${index}`,
    game,
    title,
    subtitle: gameCategoryLabel(game),
    value,
    detail,
    tone: game.tone,
  };
}

function guideTopCard(topic: LeaderboardGuideTopic): LeaderboardTopCardItem {
  return {
    kind: 'guide',
    key: `guide:${normalizeSelectionId(topic.id)}`,
    topic,
    title: topic.title,
    subtitle: topic.subtitle,
    value: topic.category,
    detail: topic.detail,
    tone: topic.tone,
  };
}

function buildGameCategorySummaries(games: QuickGame[]): GameCategorySummary[] {
  const summaries = new Map<string, GameCategorySummary>();
  const subcategoryMaps = new Map<string, Map<string, GameSubcategorySummary>>();
  const fallbackGame = games[0];
  const tones: Tone[] = ['gold', 'cyan', 'purple', 'red', 'muted'];
  for (const game of games) {
    const label = gameCategoryLabel(game);
    const id = assetKey(label);
    const subcategoryLabel = gameSubcategoryLabel(game);
    const subcategoryId = assetKey(subcategoryLabel);
    const existing = summaries.get(id);
    if (existing) {
      existing.count += 1;
      const subcategories = subcategoryMaps.get(id) ?? new Map<string, GameSubcategorySummary>();
      const existingSubcategory = subcategories.get(subcategoryId);
      if (existingSubcategory) {
        existingSubcategory.count += 1;
      } else {
        subcategories.set(subcategoryId, {
          id: subcategoryId,
          label: subcategoryLabel,
          count: 1,
          tone: game.tone,
          sampleGame: game,
        });
      }
      subcategoryMaps.set(id, subcategories);
      existing.subcategories = Array.from(subcategories.values()).sort(
        (a, b) => b.count - a.count || a.label.localeCompare(b.label)
      );
      continue;
    }
    const firstSubcategory: GameSubcategorySummary = {
      id: subcategoryId,
      label: subcategoryLabel,
      count: 1,
      tone: game.tone,
      sampleGame: game,
    };
    subcategoryMaps.set(id, new Map([[subcategoryId, firstSubcategory]]));
    summaries.set(id, {
      id,
      label,
      detail: subcategoryLabel,
      count: 1,
      tone: game.tone,
      sampleGame: game,
      subcategories: [firstSubcategory],
    });
  }
  LEADERBOARD_CATEGORY_LABELS.forEach((label, index) => {
    const id = assetKey(label);
    if (summaries.has(id) || !fallbackGame) return;
    summaries.set(id, {
      id,
      label,
      detail: 'Catalog scope',
      count: 0,
      tone: tones[index % tones.length],
      sampleGame: fallbackGame,
      subcategories: [],
    });
  });
  return Array.from(summaries.values()).sort((a, b) => {
    if (a.count !== b.count) return b.count - a.count;
    const aKnown = LEADERBOARD_CATEGORY_LABELS.indexOf(a.label as (typeof LEADERBOARD_CATEGORY_LABELS)[number]);
    const bKnown = LEADERBOARD_CATEGORY_LABELS.indexOf(b.label as (typeof LEADERBOARD_CATEGORY_LABELS)[number]);
    if (aKnown >= 0 && bKnown >= 0) return aKnown - bKnown;
    if (aKnown >= 0) return -1;
    if (bKnown >= 0) return 1;
    return a.label.localeCompare(b.label);
  });
}

function detailForNav(activeNavLabel: string, detail: TabDetail): TabDetail {
  const key = assetKey(activeNavLabel);
  if (!key.includes('overview') && !key.includes('today')) return detail;
  return {
    ...detail,
    eyebrow: 'Family command',
    title: 'Today',
    summary: 'Snapshot of child-device connection, evidence readiness, browser controls, and setup gaps.',
    primary: 'Current device state',
    secondary: 'Controls, evidence, privacy, and setup',
    action: 'Review today',
    tone: 'cyan',
  };
}

function tableColumnPositions(x: number, w: number) {
  return {
    rank: x + 22,
    rankCenter: x + 31,
    avatarCenter: x + 79,
    participant: x + 112,
    score: x + w * 0.21,
    marker: x + w * 0.265,
    games: x + w * 0.32,
    wins: x + w * 0.42,
    rate: x + w * 0.5,
    best: x + w * 0.58,
    bestText: x + w * 0.61,
    badges: x + w * 0.7,
    trend: x + w - 48,
    trendCenter: x + w - 23,
  };
}

function SurfacePanel({
  x,
  y,
  w,
  h,
  tone = 'cyan',
  frame = 'default',
  selected = false,
  disabled = false,
  frameCornerThicknessScale = 1,
  onClick,
  ariaLabel,
  children,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  tone?: Tone;
  frame?: 'default' | 'deckSide';
  selected?: boolean;
  disabled?: boolean;
  frameCornerThicknessScale?: number;
  onClick?: () => void;
  ariaLabel?: string;
  children?: ReactNode;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const interactive = Boolean(onClick) && !disabled;
  const active = selected || hovered;
  const fill = selected ? cfg.colors.selectedFill : hovered ? `${color}20` : cfg.colors.panelFill;
  const handleClick = (event: MouseEvent<SVGGElement>) => {
    if (!interactive) return;
    event.stopPropagation();
    onClick?.();
  };
  const handleKeyDown = (event: KeyboardEvent<SVGGElement>) => {
    if (!interactive || (event.key !== 'Enter' && event.key !== ' ')) return;
    event.preventDefault();
    event.stopPropagation();
    onClick?.();
  };
  return (
    <g
      className={interactive ? 'leaderboard-page-svg-clickable' : undefined}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      onMouseEnter={() => interactive && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      role={interactive ? 'button' : undefined}
      tabIndex={interactive ? 0 : undefined}
      aria-label={ariaLabel}
      aria-disabled={disabled || undefined}
    >
      {frame === 'deckSide' ? (
        <LeaderboardSidePanelFrame
          x={x}
          y={y}
          w={w}
          h={h}
          tone={tone}
          active={active}
          cornerThicknessScale={frameCornerThicknessScale}
          cfg={cfg}
        />
      ) : (
        <>
          {hovered && !selected ? (
            <path
              d={cutRectPath(
                x - cfg.chrome.hoverPad,
                y - cfg.chrome.hoverPad,
                w + cfg.chrome.hoverPad * 2,
                h + cfg.chrome.hoverPad * 2,
                cfg.chrome.panelCut
              )}
              fill="none"
              stroke={color}
              strokeWidth={2.2}
              opacity={cfg.chrome.glowOpacity}
              filter="url(#leaderboardGlow)"
            />
          ) : null}
          <path
            d={cutRectPath(x, y, w, h, active ? cfg.chrome.panelCut + 2 : cfg.chrome.panelCut)}
            fill={fill}
            stroke={color}
            strokeWidth={active ? cfg.chrome.panelStrokeWidth + 0.5 : cfg.chrome.panelStrokeWidth}
            opacity={disabled ? 0.48 : 0.97}
          />
          <path
            d={cutRectPath(
              x + cfg.chrome.panelInnerInset,
              y + cfg.chrome.panelInnerInset,
              w - cfg.chrome.panelInnerInset * 2,
              h - cfg.chrome.panelInnerInset * 2,
              Math.max(4, cfg.chrome.panelCut - 4)
            )}
            fill="none"
            stroke={color}
            strokeWidth={0.7}
            opacity={active ? 0.52 : 0.25}
          />
          {selected ? (
            <path
              d={cutRectPath(x, y, w, h, cfg.chrome.panelCut + 2)}
              fill="none"
              stroke={color}
              strokeWidth={5}
              opacity={0.16}
              filter="url(#leaderboardGlow)"
            />
          ) : null}
        </>
      )}
      {children}
    </g>
  );
}

function LeaderboardFrameSideHandle({
  x,
  y,
  side,
  disabled = false,
  height = 132,
  width = 22,
  onClick,
  cfg,
}: {
  x: number;
  y: number;
  side: 'left' | 'right';
  disabled?: boolean;
  height?: number;
  width?: number;
  onClick: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const handleW = width;
  const handleH = height;
  const color = hovered && !disabled ? '#23ff98' : cfg.colors.cyan;
  const compact = handleH <= 64;
  const visualH = compact ? 40 : handleH;
  const visualY = y + (handleH - visualH) / 2;
  const tipInset = Math.max(5, Math.min(7, handleW * 0.32));
  const arrowInset = compact ? 4.2 : Math.max(5, Math.min(6, handleW * 0.32));
  const arrowHalfH = compact ? 4.8 : Math.max(7, Math.min(10, handleH * 0.18));
  const bodyPath =
    side === 'left'
      ? `M ${x + tipInset} ${visualY} H ${x + handleW} V ${visualY + visualH} H ${x + tipInset} L ${x} ${visualY + visualH - tipInset} V ${visualY + tipInset} Z`
      : `M ${x} ${visualY} H ${x + handleW - tipInset} L ${x + handleW} ${visualY + tipInset} V ${visualY + visualH - tipInset} L ${x + handleW - tipInset} ${visualY + visualH} H ${x} Z`;
  const arrowTipX = side === 'left' ? x + tipInset : x + handleW - tipInset;
  const arrowBackX = side === 'left' ? x + tipInset + arrowInset : x + handleW - tipInset - arrowInset;
  const arrowY = visualY + visualH / 2;
  const arrowPath = `M ${arrowTipX} ${arrowY} L ${arrowBackX} ${arrowY - arrowHalfH} V ${arrowY + arrowHalfH} Z`;
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={disabled ? -1 : 0}
      aria-disabled={disabled}
      aria-label={side === 'left' ? 'Previous leaderboard carousel page' : 'Next leaderboard carousel page'}
      opacity={disabled ? 0.72 : 1}
      onClick={(event) => {
        event.stopPropagation();
        if (!disabled) onClick();
      }}
      onKeyDown={(event) => {
        if (disabled || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {hovered && !disabled ? (
        <path d={bodyPath} fill={color} opacity={0.24} filter="url(#leaderboardGreenGlow)" pointerEvents="none" />
      ) : null}
      <path
        d={bodyPath}
        fill={hovered && !disabled ? 'rgba(13, 89, 48, 0.9)' : 'rgba(8, 47, 75, 0.9)'}
        stroke={color}
        strokeWidth={hovered && !disabled ? 2 : 1.35}
      />
      <path
        d={bodyPath}
        fill="url(#leaderboardFrameShine)"
        opacity={hovered && !disabled ? 0.95 : 0.76}
        pointerEvents="none"
      />
      <path
        d={arrowPath}
        fill="#001522"
        opacity={0.8}
        transform={`translate(${side === 'left' ? 1 : -1} 0)`}
        pointerEvents="none"
      />
      <path d={arrowPath} fill={hovered && !disabled ? '#c9ffd8' : '#ecfbff'} pointerEvents="none" />
      <rect x={x - 6} y={y - 6} width={handleW + 12} height={handleH + 12} fill="transparent" />
    </g>
  );
}

function LeaderboardFrameDots({
  x,
  y,
  page,
  pageCount,
  onPageChange,
  cfg,
}: {
  x: number;
  y: number;
  page: number;
  pageCount: number;
  onPageChange: (page: number) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const visibleCount = Math.min(pageCount, 7);
  const start =
    pageCount <= visibleCount ? 0 : clampValue(page - Math.floor(visibleCount / 2), 0, pageCount - visibleCount);
  const slots = Array.from({ length: visibleCount }, (_, index) => start + index);
  const inactiveW = 15;
  const activeW = 34;
  const gap = 8;
  const totalW =
    slots.reduce((sum, slot) => sum + (slot === page ? activeW : inactiveW), 0) + Math.max(0, slots.length - 1) * gap;
  let cursor = x - totalW / 2;
  return (
    <g>
      {slots.map((slot) => {
        const active = slot === page;
        const dotW = active ? activeW : inactiveW;
        const dotX = cursor;
        cursor += dotW + gap;
        return (
          <g
            key={slot}
            className="leaderboard-page-svg-clickable"
            role="button"
            tabIndex={0}
            aria-label={`Open carousel page ${slot + 1}`}
            onClick={(event) => {
              event.stopPropagation();
              onPageChange(slot);
            }}
            onKeyDown={(event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              event.stopPropagation();
              onPageChange(slot);
            }}
          >
            <rect x={dotX - 5} y={y - 10} width={dotW + 10} height={22} fill="transparent" />
            <rect
              x={dotX}
              y={y - 4}
              width={dotW}
              height={8}
              rx={4}
              fill={active ? 'url(#leaderboardFooterActivePill)' : 'rgba(100, 216, 255, 0.08)'}
              stroke={active ? '#ffe187' : cfg.colors.cyan}
              strokeWidth={active ? 1.5 : 1.1}
              strokeOpacity={active ? 0.95 : 0.58}
              filter={active ? 'url(#leaderboardGlow)' : undefined}
            />
          </g>
        );
      })}
    </g>
  );
}

function LeaderboardHeaderAction({
  x,
  y,
  w,
  h,
  label,
  tone = 'cyan',
  active = false,
  onClick,
  ariaLabel,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  tone?: Tone;
  active?: boolean;
  onClick: () => void;
  ariaLabel?: string;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const lit = active || hovered;
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={ariaLabel ?? label}
      onClick={(event) => {
        event.stopPropagation();
        onClick();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {lit ? (
        <rect
          x={x - 3}
          y={y - 3}
          width={w + 6}
          height={h + 6}
          fill="none"
          stroke={color}
          strokeWidth={1.4}
          opacity={0.24}
          filter="url(#leaderboardGlow)"
        />
      ) : null}
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={3}
        fill={lit ? `${color}24` : 'rgba(4, 18, 31, 0.88)'}
        stroke={color}
        strokeWidth={lit ? 1.35 : 0.9}
        strokeOpacity={lit ? 0.95 : 0.64}
      />
      <rect
        x={x + 2}
        y={y + 2}
        width={w - 4}
        height={Math.max(1, h * 0.36)}
        rx={2}
        fill="#ffffff"
        opacity={lit ? 0.12 : 0.06}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={fitSingleLineTextSize(label, w - 16, 8.5, 11.5, 0.58)}
        fontWeight={900}
        fill={cfg.colors.bodyText}
      >
        {label}
      </text>
    </g>
  );
}

function LeaderboardBrowserToolbarToggle({
  x,
  y,
  w,
  h,
  label,
  active,
  tone,
  onClick,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  active: boolean;
  tone: Tone;
  onClick: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const lit = active || hovered;
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={label}
      aria-pressed={active}
      onClick={(event) => {
        event.stopPropagation();
        onClick();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={8}
        fill={lit ? colorAlpha(color, active ? '58' : '30') : 'rgba(7, 16, 38, 0.92)'}
        stroke={lit ? '#9ceeff' : '#526b9d'}
        strokeWidth={lit ? 1.45 : 0.9}
        filter={active ? 'url(#leaderboardGlow)' : undefined}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={fitSingleLineTextSize(label, w - 16, 9, 11.5, 0.58)}
        fontWeight={920}
        fill={cfg.colors.bodyText}
      >
        {label}
      </text>
    </g>
  );
}

function LeaderboardGameBrowserToolbar({
  x,
  y,
  search,
  onSearchChange,
  view,
  onViewChange,
  sort,
  onSortChange,
  cfg,
}: {
  x: number;
  y: number;
  search: string;
  onSearchChange: (value: string) => void;
  view: LeaderboardGameBrowserView;
  onViewChange: (value: LeaderboardGameBrowserView) => void;
  sort: LeaderboardGameBrowserSort;
  onSortChange: (value: LeaderboardGameBrowserSort) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const searchW = 238;
  const h = 28;
  return (
    <g>
      <foreignObject x={x} y={y} width={searchW} height={h}>
        <div
          style={{
            width: '100%',
            height: '100%',
            boxSizing: 'border-box',
            border: '1px solid rgba(156,238,255,0.62)',
            borderRadius: '9px',
            background: 'rgba(7,16,38,0.94)',
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '0 10px',
            boxShadow: '0 0 12px rgba(66,232,255,0.12)',
          }}
          onClick={(event) => event.stopPropagation()}
        >
          <span
            style={{
              width: '8px',
              height: '8px',
              borderRadius: '999px',
              background: cfg.colors.cyan,
              opacity: 0.78,
              flex: '0 0 auto',
            }}
          />
          <input
            aria-label="Search parent controls"
            value={search}
            placeholder="Search controls..."
            onChange={(event) => onSearchChange(event.currentTarget.value)}
            style={{
              width: '100%',
              minWidth: 0,
              border: 0,
              outline: 0,
              background: 'transparent',
              color: cfg.colors.bodyText,
              font: '700 12px Inter, system-ui, sans-serif',
            }}
          />
          <span style={{ color: cfg.colors.cyan, font: '900 13px Inter, system-ui, sans-serif' }}>Q</span>
        </div>
      </foreignObject>
      <LeaderboardBrowserToolbarToggle
        x={x + searchW + 12}
        y={y}
        w={70}
        h={h}
        label="GRID"
        active={view === 'grid'}
        tone="purple"
        onClick={() => onViewChange('grid')}
        cfg={cfg}
      />
      <LeaderboardBrowserToolbarToggle
        x={x + searchW + 90}
        y={y}
        w={68}
        h={h}
        label="LIST"
        active={view === 'list'}
        tone="cyan"
        onClick={() => onViewChange('list')}
        cfg={cfg}
      />
      <LeaderboardBrowserToolbarToggle
        x={x + searchW + 172}
        y={y}
        w={76}
        h={h}
        label={sort === 'az' ? 'A-Z' : 'RANK'}
        active={sort === 'az'}
        tone="muted"
        onClick={() => onSortChange(sort === 'az' ? 'rank' : 'az')}
        cfg={cfg}
      />
    </g>
  );
}

function LeaderboardSectionFrame({
  x,
  y,
  w,
  h,
  title,
  subtitle,
  count,
  tone = 'cyan',
  headerRight,
  footer,
  footerH,
  headerH,
  innerStrokeOpacity = 0.6,
  bodyStrokeOpacity = 0.72,
  bodyFill = 'rgba(7, 30, 48, 0.38)',
  footerLineOpacity = 0.42,
  showSideHandles = false,
  sideDisabled = false,
  onPrevious,
  onNext,
  onWheel,
  selected = false,
  onSelect,
  ariaLabel,
  cfg,
  children,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  title: string;
  subtitle?: string;
  count?: string;
  tone?: Tone;
  headerRight?: ReactNode;
  footer?: (rect: LeaderboardRect) => ReactNode;
  footerH?: number;
  headerH?: number;
  innerStrokeOpacity?: number;
  bodyStrokeOpacity?: number;
  bodyFill?: string;
  footerLineOpacity?: number;
  showSideHandles?: boolean;
  sideDisabled?: boolean;
  onPrevious?: () => void;
  onNext?: () => void;
  onWheel?: (event: WheelEvent<SVGGElement>) => void;
  selected?: boolean;
  onSelect?: () => void;
  ariaLabel?: string;
  cfg: LeaderboardPageSvgControls;
  children: (rect: LeaderboardRect) => ReactNode;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const { body, footer: footerRect, headerH: resolvedHeaderH } = leaderboardFrameRects(x, y, w, h, footerH, headerH);
  const cut = Math.max(6, Math.min(cfg.chrome.panelCut, 12));
  const interactive = Boolean(onSelect);
  const active = selected || hovered;
  const countW = count ? 58 : 0;
  const titleX = x + 22 + countW;
  const titleW = Math.max(190, Math.min(w * 0.42, title.length * 11 + 68));
  const titleY = y + 11;
  const titleH = 31;
  const countPath = count ? cutRectPath(x + 22, titleY - 3, countW, titleH + 3, 5) : '';
  const titlePath = cutRectPath(titleX, titleY, titleW, titleH, 5);
  const sideHandleW = LEADERBOARD_SIDE_HANDLE_W;
  const sideHandleH = Math.max(72, Math.min(128, body.h - 28));
  const sideHandleY = body.y + Math.max(12, (body.h - sideHandleH) / 2);
  const leftHandleX = x - sideHandleW + LEADERBOARD_SIDE_HANDLE_OVERLAP;
  const rightHandleX = x + w - LEADERBOARD_SIDE_HANDLE_OVERLAP;
  const handleSelect = (event: MouseEvent<SVGGElement>) => {
    if (!interactive) return;
    event.stopPropagation();
    onSelect?.();
  };
  const handleKeyDown = (event: KeyboardEvent<SVGGElement>) => {
    if (!interactive || (event.key !== 'Enter' && event.key !== ' ')) return;
    event.preventDefault();
    event.stopPropagation();
    onSelect?.();
  };
  return (
    <g
      className={interactive ? 'leaderboard-page-svg-clickable' : undefined}
      role={interactive ? 'button' : undefined}
      tabIndex={interactive ? 0 : undefined}
      aria-label={ariaLabel}
      aria-pressed={interactive ? selected : undefined}
      onClick={handleSelect}
      onKeyDown={handleKeyDown}
      onWheel={onWheel}
      onMouseEnter={() => interactive && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {interactive ? (
        <rect x={x - 8} y={y - 8} width={w + 16} height={h + 16} fill="transparent" pointerEvents="all" />
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, cut)}
        fill="none"
        stroke={color}
        strokeWidth={active ? 5.2 : 3.1}
        opacity={active ? 0.35 : 0.24}
        filter="url(#leaderboardGlow)"
        pointerEvents="none"
      />
      {active ? (
        <path
          d={cutRectPath(x - 4, y - 4, w + 8, h + 8, cut + 2)}
          fill="none"
          stroke={color}
          strokeWidth={2}
          opacity={selected ? 0.42 : 0.26}
          filter="url(#leaderboardGlow)"
          pointerEvents="none"
        />
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, cut)}
        fill="url(#leaderboardFrameFill)"
        stroke={color}
        strokeWidth={active ? 2 : 1.5}
      />
      <path
        d={cutRectPath(x + 6, y + 6, w - 12, h - 12, Math.max(4, cut - 4))}
        fill="url(#leaderboardFrameGlass)"
        stroke={cfg.colors.cyan}
        strokeWidth={active ? 1.2 : 0.85}
        strokeOpacity={selected ? Math.max(innerStrokeOpacity, 0.46) : innerStrokeOpacity}
      />
      <path
        d={cutRectPath(x + 8, y + 8, w - 16, Math.min(62, resolvedHeaderH + 10), Math.max(4, cut - 5))}
        fill="url(#leaderboardFrameShine)"
        opacity={active ? 0.56 : 0.42}
        pointerEvents="none"
      />
      {count ? (
        <>
          <path
            d={countPath}
            fill={colorAlpha(color, 'd8')}
            stroke="#8df5ff"
            strokeWidth={1.15}
            filter="url(#leaderboardGlow)"
          />
          <text
            x={x + 22 + countW / 2}
            y={titleY + 20}
            textAnchor="middle"
            fontSize={15}
            fontWeight={950}
            fill="#02121a"
            stroke="#d9fbff"
            strokeWidth={0.6}
            paintOrder="stroke fill"
          >
            {count}
          </text>
        </>
      ) : null}
      <path
        d={titlePath}
        fill={tone === 'purple' ? 'rgba(50, 27, 92, 0.78)' : 'rgba(8, 43, 62, 0.74)'}
        stroke={color}
        strokeWidth={1.1}
      />
      <path
        d={`M ${titleX + 12} ${titleY + 6} H ${titleX + titleW - 12}`}
        stroke="#ffffff"
        strokeWidth={1.1}
        opacity={0.13}
      />
      <text x={titleX + 18} y={titleY + 21} fontSize={13.5} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      <line
        x1={titleX + titleW + 12}
        y1={y + resolvedHeaderH - 8}
        x2={x + w - 20}
        y2={y + resolvedHeaderH - 8}
        stroke={cfg.colors.cyan}
        strokeWidth={1.1}
        opacity={0.38}
      />
      {subtitle && !count ? (
        <text x={x + 24} y={y + resolvedHeaderH - 12} fontSize={10.2} fontWeight={820} fill={cfg.colors.mutedText}>
          {subtitle}
        </text>
      ) : null}
      {headerRight}
      <path
        d={cutRectPath(body.x, body.y, body.w, body.h, 7)}
        fill={bodyFill}
        stroke={color}
        strokeWidth={0.9}
        strokeOpacity={bodyStrokeOpacity}
      />
      {children(body)}
      {footerLineOpacity > 0 ? (
        <line
          x1={footerRect.x + 12}
          y1={footerRect.y + 3}
          x2={footerRect.x + footerRect.w - 12}
          y2={footerRect.y + 3}
          stroke={cfg.colors.cyan}
          strokeWidth={1.1}
          opacity={footerLineOpacity}
        />
      ) : null}
      {footer?.(footerRect)}
      {showSideHandles && onPrevious ? (
        <LeaderboardFrameSideHandle
          x={leftHandleX}
          y={sideHandleY}
          side="left"
          height={sideHandleH}
          width={sideHandleW}
          disabled={sideDisabled}
          onClick={onPrevious}
          cfg={cfg}
        />
      ) : null}
      {showSideHandles && onNext ? (
        <LeaderboardFrameSideHandle
          x={rightHandleX}
          y={sideHandleY}
          side="right"
          height={sideHandleH}
          width={sideHandleW}
          disabled={sideDisabled}
          onClick={onNext}
          cfg={cfg}
        />
      ) : null}
    </g>
  );
}

function ArtworkSlot({
  x,
  y,
  w,
  h,
  label,
  imageUrl = null,
  tone = 'cyan',
  compact = false,
  shape = 'rect',
  imageFit = 'meet',
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  imageUrl?: string | null;
  tone?: Tone;
  compact?: boolean;
  shape?: 'rect' | 'hex' | 'circle';
  imageFit?: 'meet' | 'slice';
  cfg: LeaderboardPageSvgControls;
}) {
  const rawId = useId();
  const color = toneColor(tone, cfg);
  const cx = x + w / 2;
  const cy = y + h / 2;
  const inset = Math.min(8, w * 0.18, h * 0.18);
  const radius = Math.min(w, h) / 2 - 1;
  const clipId = `leaderboard-art-${rawId.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const primaryText = compact ? 'MISS' : 'MISSING';
  const primaryFontSize = compact ? Math.min(7, Math.max(4.5, w * 0.24)) : Math.min(11, Math.max(7.5, w * 0.09));
  const secondaryFontSize = Math.min(8.5, Math.max(6.5, w * 0.06));
  const fill = imageUrl ? 'rgba(3, 13, 24, 0.82)' : 'rgba(48, 12, 23, 0.72)';
  const renderShape = (shapeFill: string, shapeStroke: string, dash = '') => {
    if (shape === 'circle') {
      return (
        <circle
          cx={cx}
          cy={cy}
          r={radius}
          fill={shapeFill}
          stroke={shapeStroke}
          strokeWidth={1.1}
          strokeDasharray={dash}
          opacity={0.95}
        />
      );
    }
    if (shape === 'hex') {
      return (
        <path
          d={hexPath(cx, cy, radius)}
          fill={shapeFill}
          stroke={shapeStroke}
          strokeWidth={1.1}
          strokeDasharray={dash}
          opacity={0.95}
        />
      );
    }
    return (
      <path
        d={cutRectPath(x, y, w, h, 4)}
        fill={shapeFill}
        stroke={shapeStroke}
        strokeWidth={1.1}
        strokeDasharray={dash}
        opacity={0.95}
      />
    );
  };
  return (
    <g pointerEvents="none">
      {renderShape(fill, color, imageUrl ? '' : '4 3')}
      {imageUrl ? (
        <>
          <clipPath id={clipId}>
            {shape === 'circle' ? (
              <circle cx={cx} cy={cy} r={radius - 1} />
            ) : shape === 'hex' ? (
              <path d={hexPath(cx, cy, radius - 1)} />
            ) : (
              <path d={cutRectPath(x + 1, y + 1, w - 2, h - 2, 4)} />
            )}
          </clipPath>
          <image
            href={imageUrl}
            x={x + 2}
            y={y + 2}
            width={w - 4}
            height={h - 4}
            preserveAspectRatio={`xMidYMid ${imageFit}`}
            clipPath={`url(#${clipId})`}
            opacity={0.98}
          />
          {renderShape('none', color)}
        </>
      ) : (
        <>
          <title>{`Missing ${label} image`}</title>
          <line
            x1={x + inset}
            y1={y + inset}
            x2={x + w - inset}
            y2={y + h - inset}
            stroke={color}
            strokeOpacity={0.55}
            strokeWidth={0.8}
          />
          <line
            x1={x + w - inset}
            y1={y + inset}
            x2={x + inset}
            y2={y + h - inset}
            stroke={color}
            strokeOpacity={0.55}
            strokeWidth={0.8}
          />
          <text
            x={cx}
            y={compact ? cy + primaryFontSize * 0.35 : cy - 1}
            textAnchor="middle"
            fontSize={primaryFontSize}
            fontWeight={950}
            fill={color}
          >
            {primaryText}
          </text>
          {compact ? null : (
            <text
              x={cx}
              y={cy + 13}
              textAnchor="middle"
              fontSize={secondaryFontSize}
              fontWeight={900}
              fill={cfg.colors.mutedText}
            >
              {label}
            </text>
          )}
        </>
      )}
    </g>
  );
}

function IconBadge({
  x,
  y,
  icon: Icon,
  tone = 'cyan',
  size = 38,
  rank,
  cfg,
}: {
  x: number;
  y: number;
  icon: IconComponent;
  tone?: Tone;
  size?: number;
  rank?: number;
  cfg: LeaderboardPageSvgControls;
}) {
  const color = toneColor(tone, cfg);
  return (
    <g>
      <path
        d={hexPath(x + size / 2, y + size / 2, size / 2)}
        fill="#071a2a"
        stroke={color}
        strokeWidth={1.4}
        filter="url(#leaderboardGlow)"
      />
      {rank ? (
        <text x={x + size / 2} y={y + size / 2 + 7} textAnchor="middle" fontSize={18} fontWeight={900} fill={color}>
          {rank}
        </text>
      ) : (
        <Icon x={x + 9} y={y + 9} width={size - 18} height={size - 18} color={color} strokeWidth={2.2} />
      )}
    </g>
  );
}

function PlayerAvatarSlot({
  cx,
  cy,
  r,
  player,
  tone = 'cyan',
  cfg,
}: {
  cx: number;
  cy: number;
  r: number;
  player: string;
  tone?: Tone;
  cfg: LeaderboardPageSvgControls;
}) {
  return (
    <ArtworkSlot
      x={cx - r}
      y={cy - r}
      w={r * 2}
      h={r * 2}
      label="PLAYER"
      imageUrl={playerAvatarImageUrl(player)}
      tone={tone}
      compact={r < 20}
      shape="circle"
      cfg={cfg}
    />
  );
}

function GameArtBadge({
  x,
  y,
  size,
  gameId,
  tone = 'cyan',
  cfg,
}: {
  x: number;
  y: number;
  size: number;
  gameId?: string;
  tone?: Tone;
  cfg: LeaderboardPageSvgControls;
}) {
  return (
    <ArtworkSlot
      x={x}
      y={y}
      w={size}
      h={size}
      label="GAME ART"
      imageUrl={leaderboardGameImageUrl(gameId)}
      tone={tone}
      compact={size < 44}
      shape="hex"
      cfg={cfg}
    />
  );
}

function BadgeSet({
  x,
  y,
  tone = 'cyan',
  cfg,
}: {
  x: number;
  y: number;
  tone?: Tone;
  cfg: LeaderboardPageSvgControls;
}) {
  return (
    <g>
      {[0, 1, 2].map((index) => (
        <ArtworkSlot
          key={index}
          x={x - 8 + index * 21}
          y={y - 8}
          w={16}
          h={16}
          label="BADGE"
          imageUrl={leaderboardBadgeImageUrl(tone, index)}
          tone={tone}
          compact
          shape="hex"
          cfg={cfg}
        />
      ))}
    </g>
  );
}

function MetricCard({
  label,
  value,
  icon: Icon,
  tone,
  x,
  y,
  w,
  cfg,
}: {
  label: string;
  value: string | number;
  icon: IconComponent;
  tone: Tone;
  x: number;
  y: number;
  w: number;
  cfg: LeaderboardPageSvgControls;
}) {
  const color = toneColor(tone, cfg);
  const text = String(value);
  const cardH = cfg.layout.headerH;
  const iconBox = Math.max(38, Math.min(46, cardH - 28));
  const iconX = x + 19;
  const iconY = y + (cardH - iconBox) / 2;
  const dividerX = iconX + iconBox + 14;
  const textX = dividerX + 18;
  const textW = Math.max(52, x + w - textX - 18);
  const labelSize = fitSingleLineTextSize(label, textW, 8.2, 10.2, 0.58);
  const valueFontSize = fitSingleLineTextSize(text, textW, 14, text.length > 10 ? 18 : 23, 0.58);
  const labelY = y + 25;
  const splitY = y + 34;
  const valueY = y + 58;
  const bottomLineY = y + cardH - 16;
  return (
    <SurfacePanel x={x} y={y} w={w} h={cardH} tone={tone} cfg={cfg}>
      <path
        d={cutRectPath(iconX, iconY, iconBox, iconBox, 8)}
        fill={colorAlpha(color, '18')}
        stroke={color}
        strokeWidth={1.05}
        strokeOpacity={0.72}
      />
      <path d={cutRectPath(iconX + 4, iconY + 4, iconBox - 8, iconBox * 0.44, 5)} fill="#ffffff" opacity={0.08} />
      <Icon
        x={iconX + iconBox / 2 - 13}
        y={iconY + iconBox / 2 - 13}
        width={26}
        height={26}
        color={color}
        strokeWidth={2.35}
      />
      <line
        x1={dividerX}
        y1={y + 16}
        x2={dividerX}
        y2={y + cardH - 16}
        stroke={color}
        strokeWidth={1.05}
        opacity={0.68}
      />
      <line
        x1={dividerX + 5}
        y1={y + 16}
        x2={dividerX + 5}
        y2={y + cardH - 16}
        stroke="#ffffff"
        strokeWidth={0.7}
        opacity={0.08}
      />
      <text x={textX} y={labelY} fontSize={labelSize} fontWeight={850} fill={cfg.colors.mutedText}>
        {label}
      </text>
      <line x1={textX} y1={splitY} x2={textX + textW} y2={splitY} stroke={color} strokeWidth={0.85} opacity={0.35} />
      <text
        x={textX}
        y={valueY}
        fontSize={valueFontSize}
        fontWeight={950}
        fill={tone === 'gold' ? '#ffde78' : '#effaff'}
      >
        {text}
      </text>
      <line
        x1={textX}
        y1={bottomLineY}
        x2={textX + Math.min(textW, Math.max(42, text.length * valueFontSize * 0.48))}
        y2={bottomLineY}
        stroke={color}
        strokeWidth={1.1}
        opacity={0.22}
      />
    </SurfacePanel>
  );
}

function NavRow({
  item,
  active,
  x,
  w,
  y,
  rowH,
  iconSize,
  onSelect,
  cfg,
}: {
  item: NavItem;
  active: boolean;
  x: number;
  w: number;
  y: number;
  rowH: number;
  iconSize: number;
  onSelect: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = active ? cfg.colors.bodyText : '#d8eaff';
  const rowX = x + 14;
  const rowW = w - 28;
  const lit = active || hovered;
  const accent = active ? cfg.colors.cyan : cfg.colors.cyan;
  const slotX = x + 22;
  const slotY = y + (rowH - iconSize) / 2;
  const slotW = Math.max(30, iconSize - 6);
  const textX = x + 72;
  const labelW = Math.max(48, x + w - 46 - textX);
  const labelSize = fitSingleLineTextSize(item.label, labelW, 9.8, 13.4, 0.58);
  const detailSize = fitSingleLineTextSize(item.detail, labelW, 8.4, 10.6, 0.58);
  const arrowTop = y + 7;
  const arrowBottom = y + rowH - 7;
  const arrowMid = y + rowH / 2;
  return (
    <g
      className="leaderboard-page-svg-clickable"
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      role="button"
      tabIndex={0}
      aria-label={`Open ${item.label}`}
    >
      <rect x={rowX - 6} y={y - 4} width={rowW + 28} height={rowH + 8} fill="transparent" pointerEvents="all" />
      {lit ? (
        <>
          <path
            d={cutRectPath(rowX - 3, y - 3, rowW + 6, rowH + 6, 11)}
            fill="none"
            stroke={accent}
            strokeWidth={active ? 2.2 : 2}
            opacity={active ? 0.42 : 0.34}
            filter="url(#leaderboardGlow)"
            pointerEvents="none"
          />
          <path
            d={`M ${rowX + rowW - 8} ${arrowTop} L ${rowX + rowW + 18} ${arrowMid} L ${rowX + rowW - 8} ${arrowBottom} Z`}
            fill={accent}
            opacity={active ? 0.82 : 0.62}
            filter="url(#leaderboardGlow)"
            pointerEvents="none"
          />
        </>
      ) : null}
      <path
        d={cutRectPath(rowX, y, rowW, rowH, 8)}
        fill={lit ? 'url(#leaderboardActiveBlue)' : 'transparent'}
        fillOpacity={active ? 0.92 : hovered ? 0.78 : 1}
        stroke={lit ? accent : 'transparent'}
        strokeWidth={lit ? 1.6 : 0}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX + 3, y + 3, rowW - 6, rowH - 6, 6)}
          fill="none"
          stroke={accent}
          strokeWidth={1}
          opacity={active ? 0.68 : 0.5}
          pointerEvents="none"
        />
      ) : null}
      <path
        d={cutRectPath(slotX, slotY + 3, slotW, slotW, 5)}
        fill="rgba(7, 22, 37, 0.42)"
        stroke={accent}
        strokeWidth={1.1}
        strokeOpacity={lit ? 0.86 : 0.62}
        pointerEvents="none"
      />
      <path
        d={cutRectPath(slotX + 4, slotY + 7, slotW - 8, slotW - 8, 4)}
        fill="none"
        stroke={accent}
        strokeWidth={0.8}
        strokeOpacity={lit ? 0.54 : 0.32}
        pointerEvents="none"
      />
      <text x={textX} y={y + rowH * 0.43} fontSize={labelSize} fontWeight={900} fill={color} pointerEvents="none">
        {item.label}
      </text>
      <text
        x={textX}
        y={y + rowH * 0.79}
        fontSize={detailSize}
        fontWeight={720}
        fill={cfg.colors.mutedText}
        pointerEvents="none"
      >
        {item.detail}
      </text>
      {lit ? <ChevronRight x={x + w - 34} y={y + 15} width={16} height={16} color={cfg.colors.bodyText} /> : null}
    </g>
  );
}

function NavGroupHeader({
  group,
  open,
  x,
  w,
  y,
  h,
  onToggle,
  cfg,
}: {
  group: NavGroup;
  open: boolean;
  x: number;
  w: number;
  y: number;
  h: number;
  onToggle: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const rowX = x + 14;
  const rowW = w - 28;
  const lit = open || hovered;
  const labelW = rowW - 58;
  const labelSize = fitSingleLineTextSize(group.label, labelW, 9.2, 12.2, 0.58);
  const detailSize = fitSingleLineTextSize(group.detail, labelW, 7.2, 8.8, 0.58);
  const chevronX = rowX + rowW - 26;
  const chevronY = y + h / 2;
  return (
    <g
      className="leaderboard-page-svg-clickable"
      onClick={(event) => {
        event.stopPropagation();
        onToggle();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onToggle();
      }}
      role="button"
      tabIndex={0}
      aria-label={`${open ? 'Collapse' : 'Expand'} ${group.label}`}
      aria-expanded={open}
    >
      <rect x={rowX - 6} y={y - 4} width={rowW + 12} height={h + 8} fill="transparent" pointerEvents="all" />
      <path
        d={cutRectPath(rowX, y, rowW, h, 7)}
        fill={lit ? 'url(#leaderboardPanelBlue)' : 'rgba(8, 28, 45, 0.3)'}
        stroke={cfg.colors.cyan}
        strokeWidth={lit ? 1.2 : 0.8}
        opacity={lit ? 0.92 : 0.72}
        pointerEvents="none"
      />
      <line
        x1={rowX + 10}
        y1={y + h - 4}
        x2={rowX + rowW - 12}
        y2={y + h - 4}
        stroke={cfg.colors.cyan}
        strokeWidth={0.8}
        opacity={lit ? 0.48 : 0.28}
        pointerEvents="none"
      />
      <text
        x={rowX + 10}
        y={y + h * 0.44}
        fontSize={labelSize}
        fontWeight={900}
        fill={cfg.colors.bodyText}
        pointerEvents="none"
      >
        {group.label}
      </text>
      <text
        x={rowX + 10}
        y={y + h * 0.76}
        fontSize={detailSize}
        fontWeight={680}
        fill={cfg.colors.mutedText}
        pointerEvents="none"
      >
        {group.detail}
      </text>
      <path
        d={
          open
            ? `M ${chevronX - 5} ${chevronY - 2} L ${chevronX} ${chevronY + 4} L ${chevronX + 5} ${chevronY - 2}`
            : `M ${chevronX - 2} ${chevronY - 5} L ${chevronX + 4} ${chevronY} L ${chevronX - 2} ${chevronY + 5}`
        }
        fill="none"
        stroke={cfg.colors.bodyText}
        strokeWidth={1.7}
        strokeLinecap="round"
        strokeLinejoin="round"
        opacity={lit ? 0.96 : 0.72}
        pointerEvents="none"
      />
    </g>
  );
}

function ChatDockPanel({
  x,
  y,
  w,
  h,
  active,
  onOpen,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  active: boolean;
  onOpen: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const lit = active || hovered;
  const color = active ? cfg.colors.gold : cfg.colors.cyan;
  const labelSize = fitSingleLineTextSize('CHAT', w - 92, 13, 18, 0.58);
  return (
    <SurfacePanel
      x={x}
      y={y}
      w={w}
      h={h}
      tone={active ? 'gold' : 'cyan'}
      frame="deckSide"
      frameCornerThicknessScale={0.5}
      cfg={cfg}
    >
      <g
        className="leaderboard-page-svg-clickable"
        role="button"
        tabIndex={0}
        aria-label="Open parent chat"
        aria-pressed={active}
        onClick={(event) => {
          event.stopPropagation();
          onOpen();
        }}
        onKeyDown={(event) => {
          if (event.key !== 'Enter' && event.key !== ' ') return;
          event.preventDefault();
          event.stopPropagation();
          onOpen();
        }}
        onMouseEnter={() => setHovered(true)}
        onMouseLeave={() => setHovered(false)}
      >
        {lit ? (
          <>
            <path
              d={cutRectPath(x + 13, y + 11, w - 26, h - 22, 10)}
              fill="none"
              stroke={color}
              strokeWidth={active ? 2.2 : 1.8}
              opacity={active ? 0.52 : 0.38}
              filter={active ? 'url(#leaderboardGoldGlow)' : 'url(#leaderboardGlow)'}
            />
            <path
              d={`M ${x + w - 19} ${y + 18} L ${x + w + 11} ${y + h / 2} L ${x + w - 19} ${y + h - 18} Z`}
              fill={color}
              opacity={active ? 0.82 : 0.58}
              filter={active ? 'url(#leaderboardGoldGlow)' : 'url(#leaderboardGlow)'}
            />
          </>
        ) : null}
        <path
          d={cutRectPath(x + 17, y + 15, w - 34, h - 30, 9)}
          fill={lit ? colorAlpha(color, active ? '2f' : '20') : 'rgba(5, 19, 32, 0.18)'}
          stroke={lit ? color : cfg.colors.cyan}
          strokeWidth={lit ? 1.4 : 0.8}
          strokeOpacity={lit ? 0.92 : 0.5}
          pointerEvents="all"
        />
        <path
          d={cutRectPath(x + 29, y + 24, 43, h - 48, 7)}
          fill={colorAlpha(color, lit ? '22' : '12')}
          stroke={color}
          strokeWidth={1}
          strokeOpacity={lit ? 0.82 : 0.5}
        />
        <path
          d={`M ${x + 42} ${y + h / 2 - 7} H ${x + 60} M ${x + 42} ${y + h / 2} H ${x + 56} M ${x + 42} ${y + h / 2 + 7} H ${x + 62}`}
          stroke={color}
          strokeWidth={2}
          strokeLinecap="round"
          opacity={lit ? 0.95 : 0.72}
        />
        <text x={x + 84} y={y + h / 2 - 4} fontSize={labelSize} fontWeight={950} fill={cfg.colors.bodyText}>
          CHAT
        </text>
        <text x={x + 84} y={y + h / 2 + 15} fontSize={9.2} fontWeight={760} fill={cfg.colors.mutedText}>
          Parent assistant
        </text>
      </g>
    </SurfacePanel>
  );
}

function NavPanel({
  activeNavLabel,
  navGroups,
  openGroupIds,
  onNavGroupToggle,
  onNavItemSelect,
  onChatOpen,
  cfg,
}: {
  activeNavLabel: string;
  navGroups: NavGroup[];
  openGroupIds: Record<string, boolean>;
  onNavGroupToggle: (groupId: string) => void;
  onNavItemSelect: (item: NavItem) => void;
  onChatOpen: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const { outerPad, leftW, topY, bottomH } = cfg.layout;
  const navItems = navGroups.flatMap((group) => group.items);
  const rawNavClipId = useId();
  const navClipId = `leaderboard-nav-clip-${rawNavClipId.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const [navScroll, setNavScroll] = useState(0);
  const rowH = 48;
  const rowStep = navItems.length > 8 ? 49 : 52;
  const groupH = 34;
  const groupGap = 6;
  const iconSize = 48;
  const dockH = Math.max(62, Math.round(bottomH * 1.1));
  const seasonY = cfg.canvas.height - dockH;
  const seasonH = dockH;
  const sideTopY = Math.max(0, topY - 14);
  const rowTop = sideTopY + 28;
  const sidePanelGap = 8;
  const navH = Math.max(300, seasonY - sideTopY - sidePanelGap);
  const navViewportY = rowTop;
  const navViewportH = Math.max(72, sideTopY + navH - rowTop - 22);
  const navContentH = navGroups.reduce(
    (height, group) => height + groupH + (openGroupIds[group.id] ? group.items.length * rowStep : 0) + groupGap,
    0
  );
  const maxNavScroll = Math.max(0, navContentH - navViewportH);
  const safeNavScroll = clampValue(navScroll, 0, maxNavScroll);
  useEffect(() => {
    if (safeNavScroll !== navScroll) setNavScroll(safeNavScroll);
  }, [navScroll, safeNavScroll]);
  const handleNavWheel = (event: WheelEvent<SVGGElement>) => {
    if (maxNavScroll <= 0) return;
    event.stopPropagation();
    event.preventDefault();
    setNavScroll((value) => clampValue(value + event.deltaY * 0.72, 0, maxNavScroll));
  };
  const thumbH = maxNavScroll > 0 ? clampValue((navViewportH / navContentH) * navViewportH, 46, navViewportH) : 0;
  const thumbY =
    maxNavScroll > 0
      ? navViewportY + (safeNavScroll / maxNavScroll) * Math.max(0, navViewportH - thumbH)
      : navViewportY;
  let cursorY = rowTop;
  return (
    <g>
      <SurfacePanel x={outerPad} y={sideTopY} w={leftW} h={navH} tone="cyan" frame="deckSide" cfg={cfg}>
        <defs>
          <clipPath id={navClipId}>
            <rect x={outerPad - 8} y={navViewportY - 2} width={leftW + 46} height={navViewportH + 4} />
          </clipPath>
        </defs>
        <g onWheel={handleNavWheel}>
          <rect
            x={outerPad + 12}
            y={navViewportY - 2}
            width={leftW - 24}
            height={navViewportH + 4}
            fill="transparent"
            pointerEvents={maxNavScroll > 0 ? 'all' : 'none'}
          />
          <g clipPath={`url(#${navClipId})`}>
            <g transform={`translate(0 ${-safeNavScroll})`}>
              {navGroups.map((group) => {
                const groupY = cursorY;
                cursorY += groupH;
                const open = Boolean(openGroupIds[group.id]);
                const rows = open
                  ? group.items.map((item) => {
                      const itemY = cursorY;
                      cursorY += rowStep;
                      return (
                        <NavRow
                          key={item.label}
                          item={item}
                          active={item.label === activeNavLabel}
                          x={outerPad}
                          w={leftW}
                          y={itemY}
                          rowH={rowH}
                          iconSize={iconSize}
                          onSelect={() => onNavItemSelect(item)}
                          cfg={cfg}
                        />
                      );
                    })
                  : null;
                cursorY += groupGap;
                return (
                  <g key={group.id}>
                    <NavGroupHeader
                      group={group}
                      open={open}
                      x={outerPad}
                      w={leftW}
                      y={groupY}
                      h={groupH}
                      onToggle={() => onNavGroupToggle(group.id)}
                      cfg={cfg}
                    />
                    {rows}
                  </g>
                );
              })}
            </g>
          </g>
        </g>
        {maxNavScroll > 0 ? (
          <g pointerEvents="none">
            <path
              d={`M ${outerPad + leftW - 10} ${navViewportY + 8} V ${navViewportY + navViewportH - 8}`}
              stroke={cfg.colors.cyan}
              strokeWidth={1.4}
              strokeLinecap="round"
              opacity={0.28}
            />
            <path
              d={`M ${outerPad + leftW - 10} ${thumbY} V ${thumbY + thumbH}`}
              stroke={cfg.colors.cyan}
              strokeWidth={3.4}
              strokeLinecap="round"
              opacity={0.82}
              filter="url(#leaderboardGlow)"
            />
          </g>
        ) : null}
      </SurfacePanel>
      <ChatDockPanel
        x={outerPad}
        y={seasonY}
        w={leftW}
        h={seasonH}
        active={activeNavLabel === 'CHAT'}
        onOpen={onChatOpen}
        cfg={cfg}
      />
    </g>
  );
}

function LeaderboardTable({
  rows,
  x,
  y,
  w,
  selectedPlayerId,
  selectedGameName,
  activeNavLabel,
  variant,
  onPlayerSelect,
  cfg,
}: {
  rows: DisplayRow[];
  x: number;
  y: number;
  w: number;
  selectedPlayerId: string;
  selectedGameName: string;
  activeNavLabel: string;
  variant: LeaderboardTableVariant;
  onPlayerSelect: (playerId: string) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const p = tableColumnPositions(x, w);
  const labels = leaderboardTableColumnLabels[variant];
  const columns = [
    { label: labels[0], x: p.rank },
    { label: labels[1], x: p.participant },
    { label: labels[2], x: p.score },
    { label: labels[3], x: p.games },
    { label: labels[4], x: p.wins },
    { label: labels[5], x: p.rate },
    { label: labels[6], x: p.best },
    { label: labels[7], x: p.badges },
    { label: labels[8], x: p.trend },
  ];
  return (
    <g>
      <path d={bottomCutRectPath(x, y, w, 38, 9)} fill="#061626" stroke={cfg.colors.panelStroke} strokeWidth={0.9} />
      {columns.map((column) => (
        <text
          key={column.label}
          x={column.x}
          y={y + 24}
          fontSize={fitSingleLineTextSize(column.label, Math.max(54, w * 0.1), 7.4, 10, 0.62)}
          fontWeight={760}
          fill="#bcd3e7"
        >
          {column.label}
        </text>
      ))}
      {rows.map((row, index) => (
        <TableRow
          key={`${row.rank}-${row.id}`}
          row={row}
          x={x}
          y={y + 42 + index * (cfg.chrome.rowHeight + cfg.chrome.rowGap)}
          w={w}
          selected={row.id === selectedPlayerId}
          selectedGameName={selectedGameName}
          activeNavLabel={activeNavLabel}
          variant={variant}
          onSelect={() => onPlayerSelect(row.id)}
          cfg={cfg}
        />
      ))}
    </g>
  );
}

function LeaderboardTableScrollRail({
  x,
  y,
  h,
  page,
  maxPage,
  cfg,
}: {
  x: number;
  y: number;
  h: number;
  page: number;
  maxPage: number;
  cfg: LeaderboardPageSvgControls;
}) {
  if (maxPage <= 1) {
    return null;
  }
  const trackH = Math.max(40, h);
  const trackW = 5;
  const thumbW = 7;
  const thumbH = Math.max(24, trackH / maxPage);
  const progress = maxPage <= 1 ? 0 : (page - 1) / (maxPage - 1);
  const thumbY = y + (trackH - thumbH) * progress;
  return (
    <g pointerEvents="none" opacity={0.9}>
      <rect
        x={x + 1}
        y={y + 8}
        width={trackW}
        height={trackH - 16}
        rx={2.5}
        fill="rgba(5, 22, 36, 0.62)"
        stroke={cfg.colors.cyan}
        strokeWidth={0.55}
        strokeOpacity={0.34}
      />
      <path
        d={cutRectPath(x, thumbY, thumbW, thumbH, 3)}
        fill="url(#leaderboardFooterActivePill)"
        stroke="#ffe187"
        strokeWidth={0.75}
        strokeOpacity={0.9}
        filter="url(#leaderboardGoldGlow)"
      />
      <rect
        x={x + 2.8}
        y={thumbY + 5}
        width={1.4}
        height={Math.max(8, thumbH - 10)}
        rx={0.7}
        fill="#fff6b8"
        opacity={0.5}
      />
    </g>
  );
}

function GameLeaderStrip({
  rows,
  x,
  y,
  w,
  h,
  selectedPlayerId,
  onPlayerSelect,
  cfg,
}: {
  rows: DisplayRow[];
  x: number;
  y: number;
  w: number;
  h: number;
  selectedPlayerId: string;
  onPlayerSelect: (playerId: string) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  if (rows.length === 0) return null;
  const items = rows.slice(0, 3).map(leaderTopCard);
  return (
    <LeaderboardTopCarousel
      x={x}
      y={y}
      w={w}
      h={h}
      items={items}
      page={0}
      selectedKey={`leader:${selectedPlayerId}`}
      onSelect={(item) => {
        if (item.kind === 'leader') onPlayerSelect(item.row.id);
      }}
      minCardW={230}
      cfg={cfg}
    />
  );
}

function ParentPortalDetailPanel({
  x,
  y,
  w,
  h,
  activeNavLabel,
  detail,
  rows,
  selectedGameName,
  guideTopic,
  guidePage,
  onGuidePageChange,
  quickPanelMode,
  onQuickPanelModeChange,
  onGuideNoteSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  activeNavLabel: string;
  detail: TabDetail;
  rows: DisplayRow[];
  selectedGameName: string;
  guideTopic?: LeaderboardGuideTopic | null;
  guidePage: number;
  onGuidePageChange: (page: number) => void;
  quickPanelMode: 'read' | 'action';
  onQuickPanelModeChange: (mode: 'read' | 'action') => void;
  onGuideNoteSelect: (note: LeaderboardGuideNote) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  if (guideTopic) {
    return (
      <GuideTopicDetailPanel
        x={x}
        y={y}
        w={w}
        h={h}
        topic={guideTopic}
        page={guidePage}
        onPageChange={onGuidePageChange}
        quickPanelMode={quickPanelMode}
        onQuickPanelModeChange={onQuickPanelModeChange}
        onNoteSelect={onGuideNoteSelect}
        cfg={cfg}
      />
    );
  }
  const color = toneColor(detail.tone, cfg);
  const cardGap = 12;
  const usableH = Math.max(120, h);
  const title = activeNavLabel || detail.title;
  const bodyLines = wrapCardText(detail.summary, w - 40, 12, 2);
  const featureCards = [
    {
      label: 'WHAT PARENTS CONTROL',
      value: detail.primary,
      body: detail.secondary,
      tone: detail.tone,
    },
    {
      label: 'CURRENT AREA',
      value: selectedGameName,
      body: 'Open this area per child device, then wire real service state as each adapter lands.',
      tone: 'cyan',
    },
    {
      label: 'DATA CUSTODY',
      value: 'LOCAL FIRST',
      body: 'No cloud sharing by default. Drive exports and support bundles are parent opt-in.',
      tone: 'gold',
    },
    ...rows.slice(0, 3).map((row) => ({
      label: row.bestGame.toUpperCase(),
      value: row.player,
      body: `${row.trend} / ${row.winRate}`,
      tone: row.tone,
    })),
  ];
  const visibleCards = featureCards.slice(0, usableH < 300 ? 3 : 6);
  const columnCount = w > 1220 ? 3 : w > 760 ? 2 : 1;
  const rowCount = Math.max(1, Math.ceil(visibleCards.length / columnCount));
  const headerH = bodyLines.length > 1 ? 78 : 62;
  const cardW = (w - cardGap * Math.max(0, columnCount - 1)) / columnCount;
  const cardH = clampValue((usableH - headerH - cardGap * Math.max(0, rowCount - 1)) / rowCount, 74, 118);
  const titleSize = fitSingleLineTextSize(title, w - 40, 16, 26, 0.58);
  return (
    <g>
      <text x={x} y={y + 24} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      <path d={`M ${x} ${y + 39} H ${x + w}`} stroke={color} strokeWidth={1.1} opacity={0.5} />
      {bodyLines.map((line, index) => (
        <text
          key={`${line}:${index}`}
          x={x}
          y={y + 58 + index * 17}
          fontSize={12}
          fontWeight={760}
          fill={cfg.colors.mutedText}
        >
          {line}
        </text>
      ))}
      {visibleCards.map((card, index) => {
        const col = index % columnCount;
        const row = Math.floor(index / columnCount);
        const cardX = x + col * (cardW + cardGap);
        const cardY = y + headerH + row * (cardH + cardGap);
        const cardColor = toneColor(card.tone, cfg);
        const valueSize = fitSingleLineTextSize(card.value, cardW - 34, 12, 17, 0.58);
        const cardBodyLines = wrapCardText(card.body, cardW - 34, 10.5, cardH > 92 ? 2 : 1);
        return (
          <SurfacePanel
            key={`${card.label}:${index}`}
            x={cardX}
            y={cardY}
            w={cardW}
            h={cardH}
            tone={card.tone}
            cfg={cfg}
          >
            <text x={cardX + 16} y={cardY + 25} fontSize={9.8} fontWeight={900} fill={cardColor}>
              {card.label}
            </text>
            <text x={cardX + 16} y={cardY + 49} fontSize={valueSize} fontWeight={950} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(card.value, cardW - 34, valueSize, 0.58)}
            </text>
            {cardBodyLines.map((line, lineIndex) => (
              <text
                key={`${line}:${lineIndex}`}
                x={cardX + 16}
                y={cardY + 70 + lineIndex * 15}
                fontSize={10.5}
                fontWeight={720}
                fill={cfg.colors.mutedText}
              >
                {line}
              </text>
            ))}
          </SurfacePanel>
        );
      })}
    </g>
  );
}

type ParentChatAction = {
  readonly label: string;
  readonly detail: string;
  readonly routePath: string;
  readonly tone: Tone;
};

function ParentChatActionCard({
  x,
  y,
  w,
  h,
  action,
  onOpen,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  action: ParentChatAction;
  onOpen: (routePath: string) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(action.tone, cfg);
  const labelSize = fitSingleLineTextSize(action.label, w - 32, 10, 13.5, 0.58);
  const detailLines = wrapCardText(action.detail, w - 32, 10, h > 62 ? 2 : 1);
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Open ${action.label}`}
      onClick={(event) => {
        event.stopPropagation();
        onOpen(action.routePath);
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onOpen(action.routePath);
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      {hovered ? (
        <>
          <path
            d={cutRectPath(x - 4, y - 4, w + 8, h + 8, 10)}
            fill="none"
            stroke={color}
            strokeWidth={1.7}
            opacity={0.38}
            filter="url(#leaderboardGlow)"
          />
          <path
            d={`M ${x + w - 11} ${y + 14} L ${x + w + 8} ${y + h / 2} L ${x + w - 11} ${y + h - 14} Z`}
            fill={color}
            opacity={0.64}
            filter="url(#leaderboardGlow)"
          />
        </>
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={hovered ? colorAlpha(color, '32') : colorAlpha(color, '18')}
        stroke={hovered ? color : cfg.colors.panelStroke}
        strokeWidth={hovered ? 1.18 : 0.78}
        strokeOpacity={hovered ? 0.92 : 0.64}
      />
      <path
        d={cutRectPath(x + 9, y + 10, 26, h - 20, 6)}
        fill={colorAlpha(color, hovered ? '38' : '20')}
        stroke={color}
        strokeWidth={0.8}
        strokeOpacity={0.82}
      />
      <text x={x + 22} y={y + h / 2 + 4} textAnchor="middle" fontSize={12} fontWeight={950} fill={color}>
        +
      </text>
      <text x={x + 45} y={y + 23} fontSize={labelSize} fontWeight={950} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(action.label.toUpperCase(), w - 66, labelSize, 0.58)}
      </text>
      {detailLines.map((line, index) => (
        <text
          key={`${action.label}:detail:${index}`}
          x={x + 45}
          y={y + 42 + index * 13}
          fontSize={10}
          fontWeight={720}
          fill={cfg.colors.mutedText}
        >
          {line}
        </text>
      ))}
    </g>
  );
}

function ParentChatPanel({
  x,
  y,
  w,
  h,
  onNavigate,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  onNavigate?: (routePath: string) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const compact = w < 860;
  const gap = compact ? 10 : 14;
  const sideW = compact ? w : clampValue(w * 0.28, 250, 360);
  const chatW = compact ? w : w - sideW - gap;
  const chatH = compact ? Math.max(220, h * 0.58) : h;
  const sideX = compact ? x : x + chatW + gap;
  const sideY = compact ? y + chatH + gap : y;
  const sideH = compact ? Math.max(150, h - chatH - gap) : h;
  const primaryColor = cfg.colors.cyan;
  const messageW = chatW - 42;
  const actions: ParentChatAction[] = [
    {
      label: 'Today report',
      detail: 'Open stored activity and summary evidence.',
      routePath: '#/activity',
      tone: 'cyan',
    },
    {
      label: 'Browser state',
      detail: 'Supported browsers, unmanaged risk, web evidence.',
      routePath: '#/browser',
      tone: 'gold',
    },
    { label: 'Rules', detail: 'House rules, allow, ask, explain, block.', routePath: '#/policy', tone: 'red' },
    { label: 'AI setup', detail: 'Local AI, API providers, model state.', routePath: '#/ai-runtime', tone: 'purple' },
    {
      label: 'Drives',
      detail: 'Connect parent-owned exports and custody.',
      routePath: '#/drive-connections',
      tone: 'gold',
    },
    {
      label: 'Support/API',
      detail: 'Diagnostics, route status, support bundles.',
      routePath: '#/diagnostics',
      tone: 'cyan',
    },
  ];
  const visibleActions = actions.slice(0, compact ? 4 : 6);
  const actionGap = 9;
  const actionH = Math.max(
    54,
    Math.min(74, (sideH - 86 - actionGap * (visibleActions.length - 1)) / visibleActions.length)
  );
  const promptLines = wrapCardText(
    'Ask about activity, browser evidence, rules, reports, AI setup, subscriptions, alerts, or drive custody. Answers stay tied to local evidence and citations.',
    messageW - 34,
    12,
    compact ? 3 : 4
  );
  const assistantLines = wrapCardText(
    'I can explain what happened, where the evidence came from, what risk remains, and which parent control page can change it.',
    messageW - 34,
    12,
    compact ? 3 : 4
  );
  const onOpen = (routePath: string) => {
    onNavigate?.(routePath);
  };
  return (
    <g>
      <SurfacePanel x={x} y={y} w={chatW} h={chatH} tone="cyan" cfg={cfg}>
        <text x={x + 18} y={y + 30} fontSize={19} fontWeight={950} fill={cfg.colors.bodyText}>
          PARENT ASSISTANT
        </text>
        <text x={x + 18} y={y + 52} fontSize={11.5} fontWeight={780} fill={cfg.colors.mutedText}>
          Local evidence chat for family controls and reports
        </text>
        <path d={`M ${x + 18} ${y + 68} H ${x + chatW - 18}`} stroke={primaryColor} strokeWidth={0.9} opacity={0.48} />
        <path
          d={cutRectPath(x + 18, y + 88, messageW, Math.max(86, promptLines.length * 17 + 44), 10)}
          fill="rgba(3, 12, 22, 0.82)"
          stroke={primaryColor}
          strokeWidth={0.9}
          strokeOpacity={0.74}
        />
        <text x={x + 35} y={y + 114} fontSize={10} fontWeight={950} fill={primaryColor}>
          WHAT YOU CAN ASK
        </text>
        {promptLines.map((line, index) => (
          <text
            key={`chat-prompt:${index}`}
            x={x + 35}
            y={y + 139 + index * 17}
            fontSize={12}
            fontWeight={720}
            fill={cfg.colors.mutedText}
          >
            {line}
          </text>
        ))}
        <path
          d={cutRectPath(x + 18, y + 194, messageW, Math.max(92, assistantLines.length * 17 + 50), 10)}
          fill={colorAlpha(cfg.colors.gold, '12')}
          stroke={cfg.colors.gold}
          strokeWidth={0.9}
          strokeOpacity={0.72}
        />
        <text x={x + 35} y={y + 221} fontSize={10} fontWeight={950} fill={cfg.colors.gold}>
          ANSWER STYLE
        </text>
        {assistantLines.map((line, index) => (
          <text
            key={`chat-answer:${index}`}
            x={x + 35}
            y={y + 246 + index * 17}
            fontSize={12}
            fontWeight={720}
            fill={cfg.colors.bodyText}
          >
            {line}
          </text>
        ))}
        <path
          d={cutRectPath(x + 18, y + chatH - 62, chatW - 36, 42, 9)}
          fill="rgba(5, 18, 31, 0.9)"
          stroke={primaryColor}
          strokeWidth={0.9}
          strokeOpacity={0.72}
        />
        <text x={x + 35} y={y + chatH - 36} fontSize={12} fontWeight={760} fill={cfg.colors.mutedText}>
          Ask about web, apps, rules, AI, reports, alerts, subscriptions...
        </text>
        <path
          d={cutRectPath(x + chatW - 126, y + chatH - 54, 88, 26, 7)}
          fill={colorAlpha(primaryColor, '22')}
          stroke={primaryColor}
          strokeWidth={0.8}
        />
        <text
          x={x + chatW - 82}
          y={y + chatH - 37}
          textAnchor="middle"
          fontSize={10.5}
          fontWeight={950}
          fill={primaryColor}
        >
          LOCAL
        </text>
      </SurfacePanel>
      <SurfacePanel x={sideX} y={sideY} w={sideW} h={sideH} tone="gold" cfg={cfg}>
        <text x={sideX + 16} y={sideY + 27} fontSize={13.5} fontWeight={950} fill={cfg.colors.bodyText}>
          QUICK ACTION
        </text>
        <text x={sideX + 16} y={sideY + 47} fontSize={10.5} fontWeight={760} fill={cfg.colors.mutedText}>
          Jump to the control page
        </text>
        <path
          d={`M ${sideX + 16} ${sideY + 61} H ${sideX + sideW - 16}`}
          stroke={cfg.colors.gold}
          strokeWidth={0.85}
          opacity={0.5}
        />
        {visibleActions.map((action, index) => (
          <ParentChatActionCard
            key={action.label}
            x={sideX + 14}
            y={sideY + 78 + index * (actionH + actionGap)}
            w={sideW - 28}
            h={actionH}
            action={action}
            onOpen={onOpen}
            cfg={cfg}
          />
        ))}
      </SurfacePanel>
    </g>
  );
}

function GuideQuickTab({
  x,
  y,
  w,
  h,
  label,
  active,
  tone,
  onClick,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  active: boolean;
  tone: Tone;
  onClick: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const lit = active || hovered;
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Show ${label}`}
      aria-pressed={active}
      onClick={(event) => {
        event.stopPropagation();
        onClick();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <ClickableCardHoverChrome x={x} y={y} w={w} h={h} color={color} active={active} hovered={hovered} arrow={false} />
      <path
        d={cutRectPath(x, y, w, h, 7)}
        fill={lit ? colorAlpha(color, active ? '42' : '24') : 'rgba(5, 19, 32, 0.86)'}
        stroke={lit ? color : cfg.colors.panelStroke}
        strokeWidth={active ? 1.25 : 0.85}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={fitSingleLineTextSize(label, w - 12, 8.8, 10.8, 0.58)}
        fontWeight={950}
        fill={lit ? cfg.colors.bodyText : cfg.colors.mutedText}
      >
        {label}
      </text>
    </g>
  );
}

function GuideNoteCard({
  x,
  y,
  w,
  h,
  note,
  mode,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  note: LeaderboardGuideNote;
  mode: 'read' | 'action';
  onSelect: (note: LeaderboardGuideNote) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const noteColor = toneColor(note.tone, cfg);
  const actionable =
    mode === 'action' ||
    Boolean(note.targetRoutePath || note.targetNavLabel || note.targetTopicId || typeof note.targetPage === 'number');
  const lit = actionable && (hovered || mode === 'action');
  const labelSize = fitSingleLineTextSize(note.label, w - 20, 9.5, 12.5, 0.58);
  const noteLines = wrapCardText(note.body, w - 24, 10.2, h > 66 ? 3 : 2);
  return (
    <g
      className={actionable ? 'leaderboard-page-svg-clickable' : undefined}
      role={actionable ? 'button' : undefined}
      tabIndex={actionable ? 0 : undefined}
      aria-label={actionable ? `Open ${note.label}` : undefined}
      onClick={(event) => {
        if (!actionable) return;
        event.stopPropagation();
        onSelect(note);
      }}
      onKeyDown={(event) => {
        if (!actionable || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onSelect(note);
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      {lit ? (
        <>
          <path
            d={cutRectPath(x - 4, y - 4, w + 8, h + 8, 10)}
            fill="none"
            stroke={noteColor}
            strokeWidth={hovered ? 1.7 : 1.2}
            opacity={hovered ? 0.42 : 0.28}
            filter="url(#leaderboardGlow)"
          />
          {hovered ? (
            <path
              d={`M ${x + w - 10} ${y + 15} L ${x + w + 8} ${y + h / 2} L ${x + w - 10} ${y + h - 15} Z`}
              fill={noteColor}
              opacity={0.64}
              filter="url(#leaderboardGlow)"
            />
          ) : null}
        </>
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={lit ? colorAlpha(noteColor, hovered ? '2f' : '24') : colorAlpha(noteColor, '16')}
        stroke={lit ? noteColor : cfg.colors.panelStroke}
        strokeWidth={lit ? 1.16 : 0.78}
        strokeOpacity={lit ? 0.9 : 0.62}
      />
      <text x={x + 12} y={y + 19} fontSize={labelSize} fontWeight={950} fill={lit ? cfg.colors.bodyText : noteColor}>
        {truncateTextForWidth(note.label.toUpperCase(), w - (actionable ? 72 : 24), labelSize, 0.58)}
      </text>
      {actionable ? (
        <text x={x + w - 12} y={y + 19} textAnchor="end" fontSize={8.4} fontWeight={950} fill="#fff3b2">
          OPEN
        </text>
      ) : null}
      {noteLines.map((line, lineIndex) => (
        <text
          key={`${note.label}:note:${lineIndex}`}
          x={x + 12}
          y={y + 38 + lineIndex * 13}
          fontSize={10.2}
          fontWeight={720}
          fill={cfg.colors.mutedText}
        >
          {line}
        </text>
      ))}
    </g>
  );
}

function GuideTopicDetailPanel({
  x,
  y,
  w,
  h,
  topic,
  page,
  onPageChange,
  quickPanelMode,
  onQuickPanelModeChange,
  onNoteSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  topic: LeaderboardGuideTopic;
  page: number;
  onPageChange: (page: number) => void;
  quickPanelMode: 'read' | 'action';
  onQuickPanelModeChange: (mode: 'read' | 'action') => void;
  onNoteSelect: (note: LeaderboardGuideNote) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const color = toneColor(topic.tone, cfg);
  const pageCount = Math.max(1, topic.pages.length);
  const safePage = clampValue(page, 0, pageCount - 1);
  const currentPage = topic.pages[safePage] ??
    topic.pages[0] ?? {
      eyebrow: topic.category,
      title: topic.title,
      body: topic.detail,
      steps: [topic.subtitle],
    };
  const compact = w < 760;
  const gap = compact ? 10 : 14;
  const sideW = compact ? w : clampValue(w * 0.25, 235, 330);
  const mainW = compact ? w : Math.max(280, w - sideW - gap);
  const mainH = compact ? Math.max(210, h * 0.62) : h;
  const sideX = compact ? x : x + mainW + gap;
  const sideY = compact ? y + mainH + gap : y;
  const sideH = compact ? Math.max(160, h - mainH - gap) : h;
  const titleSize = fitSingleLineTextSize(topic.title, mainW - 36, 17, 25, 0.58);
  const subtitleLines = wrapCardText(topic.subtitle, mainW - 36, 12, 2);
  const bodyLines = wrapCardText(currentPage.body, mainW - 44, 12.3, compact ? 4 : 5);
  const stepStartY = y + 168 + bodyLines.length * 17;
  const stepGap = compact ? 42 : 46;
  const maxSteps = Math.max(2, Math.min(currentPage.steps.length, Math.floor((y + mainH - 44 - stepStartY) / stepGap)));
  const visibleSteps = currentPage.steps.slice(0, maxSteps);
  const quickNotes = quickPanelMode === 'action' ? topic.actions : topic.tips;
  const noteGap = 9;
  const noteHeaderH = 58;
  const noteH =
    quickNotes.length > 0 ? Math.max(54, Math.min(82, (sideH - noteHeaderH - 28) / quickNotes.length - 1)) : 56;
  const visibleNotes = quickNotes.slice(0, Math.max(1, Math.floor((sideH - noteHeaderH - 14) / (noteH + noteGap))));
  return (
    <g>
      <SurfacePanel x={x} y={y} w={mainW} h={mainH} tone={topic.tone} cfg={cfg}>
        <text x={x + 18} y={y + 28} fontSize={9.8} fontWeight={950} fill={color}>
          {currentPage.eyebrow}
        </text>
        <text x={x + 18} y={y + 58} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
          {truncateTextForWidth(topic.title, mainW - 36, titleSize, 0.58)}
        </text>
        {subtitleLines.map((line, index) => (
          <text
            key={`${topic.id}:subtitle:${index}`}
            x={x + 18}
            y={y + 79 + index * 16}
            fontSize={12}
            fontWeight={760}
            fill={cfg.colors.mutedText}
          >
            {line}
          </text>
        ))}
        <path d={`M ${x + 18} ${y + 104} H ${x + mainW - 18}`} stroke={color} strokeWidth={0.9} opacity={0.45} />
        <text x={x + 18} y={y + 127} fontSize={15} fontWeight={950} fill={cfg.colors.bodyText}>
          {truncateTextForWidth(currentPage.title, mainW - 36, 15, 0.58)}
        </text>
        {bodyLines.map((line, index) => (
          <text
            key={`${topic.id}:body:${index}`}
            x={x + 18}
            y={y + 151 + index * 17}
            fontSize={12.3}
            fontWeight={720}
            fill={cfg.colors.mutedText}
          >
            {line}
          </text>
        ))}
        {visibleSteps.map((step, index) => {
          const stepY = stepStartY + index * stepGap;
          const stepLines = wrapCardText(step, mainW - 76, 11.5, 2);
          return (
            <g key={`${topic.id}:step:${index}`}>
              <path
                d={cutRectPath(x + 18, stepY - 14, 32, 28, 7)}
                fill={colorAlpha(color, '2a')}
                stroke={color}
                strokeWidth={0.85}
              />
              <text x={x + 34} y={stepY + 5} textAnchor="middle" fontSize={11} fontWeight={950} fill={color}>
                {index + 1}
              </text>
              {stepLines.map((line, lineIndex) => (
                <text
                  key={`${topic.id}:step:${index}:${lineIndex}`}
                  x={x + 62}
                  y={stepY + lineIndex * 15}
                  fontSize={11.5}
                  fontWeight={760}
                  fill={lineIndex === 0 ? cfg.colors.bodyText : cfg.colors.mutedText}
                >
                  {line}
                </text>
              ))}
            </g>
          );
        })}
        {pageCount > 1 ? (
          <g>
            {topic.pages.map((item, index) => {
              const pillW = 30;
              const pillX = x + mainW - 18 - (pageCount - index) * (pillW + 6);
              const selected = index === safePage;
              return (
                <g
                  key={`${topic.id}:page:${item.title}`}
                  className="leaderboard-page-svg-clickable"
                  role="button"
                  tabIndex={0}
                  aria-label={`Show guide page ${index + 1}`}
                  aria-pressed={selected}
                  onClick={(event) => {
                    event.stopPropagation();
                    onPageChange(index);
                  }}
                  onKeyDown={(event) => {
                    if (event.key !== 'Enter' && event.key !== ' ') return;
                    event.preventDefault();
                    event.stopPropagation();
                    onPageChange(index);
                  }}
                >
                  <path
                    d={cutRectPath(pillX, y + mainH - 30, pillW, 18, 5)}
                    fill={selected ? colorAlpha(color, '44') : 'rgba(3, 12, 22, 0.82)'}
                    stroke={selected ? '#ffe187' : color}
                    strokeWidth={selected ? 1.2 : 0.75}
                  />
                  <text
                    x={pillX + pillW / 2}
                    y={y + mainH - 17}
                    textAnchor="middle"
                    fontSize={9}
                    fontWeight={950}
                    fill={selected ? '#fff3b2' : cfg.colors.mutedText}
                  >
                    {index + 1}
                  </text>
                </g>
              );
            })}
          </g>
        ) : null}
      </SurfacePanel>
      <SurfacePanel x={sideX} y={sideY} w={sideW} h={sideH} tone={topic.tone} cfg={cfg}>
        <GuideQuickTab
          x={sideX + 14}
          y={sideY + 14}
          w={(sideW - 34) / 2}
          h={28}
          label="QUICK READ"
          active={quickPanelMode === 'read'}
          tone={topic.tone}
          onClick={() => onQuickPanelModeChange('read')}
          cfg={cfg}
        />
        <GuideQuickTab
          x={sideX + 20 + (sideW - 34) / 2}
          y={sideY + 14}
          w={(sideW - 34) / 2}
          h={28}
          label="QUICK ACTION"
          active={quickPanelMode === 'action'}
          tone="gold"
          onClick={() => onQuickPanelModeChange('action')}
          cfg={cfg}
        />
        <path
          d={`M ${sideX + 16} ${sideY + 50} H ${sideX + sideW - 16}`}
          stroke={color}
          strokeWidth={0.85}
          opacity={0.42}
        />
        {visibleNotes.map((note, index) => (
          <GuideNoteCard
            key={`${topic.id}:note:${quickPanelMode}:${note.label}:${index}`}
            x={sideX + 14}
            y={sideY + noteHeaderH + index * (noteH + noteGap)}
            w={sideW - 28}
            h={noteH}
            note={note}
            mode={quickPanelMode}
            onSelect={onNoteSelect}
            cfg={cfg}
          />
        ))}
      </SurfacePanel>
    </g>
  );
}

function GuideOverviewDashboard({
  x,
  y,
  w,
  h,
  topics,
  selectedTopicId,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  topics: LeaderboardGuideTopic[];
  selectedTopicId: string;
  onSelect: (topic: LeaderboardGuideTopic) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const setupTopic = topics.find((topic) => normalizeSelectionId(topic.id) === 'setup-overall') ?? topics[0];
  const cards = topics.filter((topic) => topic !== setupTopic);
  const columns = w > 1220 ? 4 : w > 900 ? 3 : w > 340 ? 2 : 1;
  const denseTopicMap = columns === 1 || cards.length > 10;
  const introSplit = w > 720;
  const introH = h > 390 ? (denseTopicMap ? 78 : 92) : denseTopicMap ? 58 : 70;
  const gap = denseTopicMap ? 7 : 12;
  const rows = Math.max(1, Math.ceil(cards.length / columns));
  const cardW = (w - gap * Math.max(0, columns - 1)) / columns;
  const rawCardH = (h - introH - gap * Math.max(0, rows - 1)) / rows;
  const cardH = clampValue(rawCardH, columns === 1 ? 34 : denseTopicMap ? 58 : 84, denseTopicMap ? 104 : 130);
  const setupColor = setupTopic ? toneColor(setupTopic.tone, cfg) : cfg.colors.cyan;
  const introTitleSize = fitSingleLineTextSize(setupTopic?.title ?? '', introSplit ? w * 0.42 : w - 36, 15, 20, 0.58);
  const [hoveredTopicId, setHoveredTopicId] = useState<string | null>(null);
  return (
    <g>
      {setupTopic ? (
        <g
          className="leaderboard-page-svg-clickable"
          role="button"
          tabIndex={0}
          aria-label={`Open ${setupTopic.title}`}
          onClick={(event) => {
            event.stopPropagation();
            onSelect(setupTopic);
          }}
          onKeyDown={(event) => {
            if (event.key !== 'Enter' && event.key !== ' ') return;
            event.preventDefault();
            event.stopPropagation();
            onSelect(setupTopic);
          }}
          onMouseEnter={() => setHoveredTopicId(setupTopic.id)}
          onMouseLeave={() => setHoveredTopicId(null)}
          onFocus={() => setHoveredTopicId(setupTopic.id)}
          onBlur={() => setHoveredTopicId(null)}
        >
          <ClickableCardHoverChrome
            x={x}
            y={y}
            w={w}
            h={introH - 10}
            color={setupColor}
            active={normalizeSelectionId(setupTopic.id) === normalizeSelectionId(selectedTopicId)}
            hovered={hoveredTopicId === setupTopic.id}
          />
          <path
            d={cutRectPath(x, y, w, introH - 10, 12)}
            fill={colorAlpha(setupColor, '1f')}
            stroke={setupColor}
            strokeWidth={1.1}
            strokeOpacity={0.72}
            filter="url(#leaderboardGlow)"
          />
          <text x={x + 18} y={y + 28} fontSize={10} fontWeight={950} fill={setupColor}>
            FIRST SETUP
          </text>
          <text
            x={x + 18}
            y={y + (introH < 68 ? 50 : 56)}
            fontSize={introTitleSize}
            fontWeight={950}
            fill={cfg.colors.bodyText}
          >
            {truncateTextForWidth(setupTopic.title, introSplit ? w * 0.42 : w - 36, introTitleSize, 0.58)}
          </text>
          {introSplit ? (
            <>
              <text
                x={x + Math.max(300, w * 0.38)}
                y={y + 35}
                fontSize={12.4}
                fontWeight={760}
                fill={cfg.colors.mutedText}
              >
                {truncateTextForWidth(setupTopic.subtitle, Math.max(220, w * 0.5), 12.4, 0.58)}
              </text>
              <text
                x={x + Math.max(300, w * 0.38)}
                y={y + 58}
                fontSize={11.6}
                fontWeight={720}
                fill={cfg.colors.mutedText}
              >
                {truncateTextForWidth(setupTopic.detail, Math.max(220, w * 0.5), 11.6, 0.58)}
              </text>
            </>
          ) : null}
        </g>
      ) : null}
      {cards.map((topic, index) => {
        const col = index % columns;
        const row = Math.floor(index / columns);
        const cardX = x + col * (cardW + gap);
        const cardY = y + introH + row * (cardH + gap);
        const color = toneColor(topic.tone, cfg);
        const selected = normalizeSelectionId(topic.id) === normalizeSelectionId(selectedTopicId);
        const compactCard = cardH < 70 || cardW < 220;
        const denseCard = cardH < 58;
        const badgeY = denseCard ? cardY + 5 : cardY + 14;
        const badgeH = denseCard ? 24 : 28;
        const titleSize = fitSingleLineTextSize(
          topic.title,
          cardW - 62,
          compactCard ? 9.8 : 12,
          compactCard ? 13.5 : 16,
          0.58
        );
        const subtitleLines = compactCard ? [] : wrapCardText(topic.subtitle, cardW - 34, 10.6, cardH > 105 ? 2 : 1);
        const detailSize = compactCard ? 8.8 : 9.6;
        const titleY = denseCard ? cardY + cardH / 2 + 4 : cardY + (compactCard ? 31 : 32);
        return (
          <g
            key={topic.id}
            className="leaderboard-page-svg-clickable"
            role="button"
            tabIndex={0}
            aria-label={`Open ${topic.title}`}
            aria-pressed={selected}
            onClick={(event) => {
              event.stopPropagation();
              onSelect(topic);
            }}
            onKeyDown={(event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              event.stopPropagation();
              onSelect(topic);
            }}
            onMouseEnter={() => setHoveredTopicId(topic.id)}
            onMouseLeave={() => setHoveredTopicId(null)}
            onFocus={() => setHoveredTopicId(topic.id)}
            onBlur={() => setHoveredTopicId(null)}
          >
            <ClickableCardHoverChrome
              x={cardX}
              y={cardY}
              w={cardW}
              h={cardH}
              color={color}
              active={selected}
              hovered={hoveredTopicId === topic.id}
            />
            <path
              d={cutRectPath(cardX, cardY, cardW, cardH, 10)}
              fill={selected ? colorAlpha(color, '2e') : 'rgba(5, 18, 31, 0.86)'}
              stroke={selected ? '#ffe187' : color}
              strokeWidth={selected ? 1.7 : 0.95}
              strokeOpacity={selected ? 0.92 : 0.62}
              filter={selected ? 'url(#leaderboardGoldGlow)' : undefined}
            />
            <path
              d={cutRectPath(cardX + 12, badgeY, 34, badgeH, 7)}
              fill={colorAlpha(color, '2a')}
              stroke={color}
              strokeWidth={0.8}
            />
            <text
              x={cardX + 29}
              y={badgeY + badgeH / 2 + 4}
              textAnchor="middle"
              fontSize={denseCard ? 10 : 11}
              fontWeight={950}
              fill={color}
            >
              {topic.rank}
            </text>
            <text x={cardX + 56} y={titleY} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(topic.title, cardW - 70, titleSize, 0.58)}
            </text>
            {subtitleLines.map((line, lineIndex) => (
              <text
                key={`${topic.id}:dashboard-subtitle:${lineIndex}`}
                x={cardX + 16}
                y={cardY + 62 + lineIndex * 15}
                fontSize={10.6}
                fontWeight={720}
                fill={cfg.colors.mutedText}
              >
                {line}
              </text>
            ))}
            {!compactCard ? (
              <path
                d={`M ${cardX + 16} ${cardY + cardH - 25} H ${cardX + cardW - 16}`}
                stroke={color}
                strokeWidth={0.8}
                opacity={0.36}
              />
            ) : null}
            {!denseCard ? (
              <text
                x={cardX + 16}
                y={cardY + cardH - (compactCard ? 8 : 10)}
                fontSize={detailSize}
                fontWeight={900}
                fill={color}
              >
                {truncateTextForWidth(topic.detail.toUpperCase(), cardW - 32, detailSize, 0.58)}
              </text>
            ) : null}
          </g>
        );
      })}
    </g>
  );
}

function wrapCardText(text: string, width: number, fontSize: number, maxLines: number): string[] {
  const words = text.trim().split(/\s+/).filter(Boolean);
  const lines: string[] = [];
  let current = '';
  for (const word of words) {
    const next = current ? `${current} ${word}` : word;
    if (next.length * fontSize * 0.55 <= width || !current) {
      current = next;
      continue;
    }
    lines.push(current);
    current = word;
    if (lines.length >= maxLines) break;
  }
  if (current && lines.length < maxLines) lines.push(current);
  return lines.map((line, index) =>
    index === maxLines - 1 ? truncateTextForWidth(line, width, fontSize, 0.55) : line
  );
}

function TableRow({
  row,
  x,
  y,
  w,
  selected,
  selectedGameName,
  activeNavLabel,
  variant,
  onSelect,
  cfg,
}: {
  row: DisplayRow;
  x: number;
  y: number;
  w: number;
  selected: boolean;
  selectedGameName: string;
  activeNavLabel: string;
  variant: LeaderboardTableVariant;
  onSelect: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(row.tone, cfg);
  const isUp = row.trend.startsWith('+');
  const isDown = row.trend.startsWith('-');
  const lit = selected || hovered;
  const p = tableColumnPositions(x, w);
  const participant = tableParticipantLabel(row, variant);
  const bestLabel = tableBestLabelForVariant(row, variant, activeNavLabel, selectedGameName);
  const participantW = Math.max(56, p.score - p.participant - 12);
  const bestW = Math.max(52, p.badges - p.bestText - 18);
  const participantSize = fitSingleLineTextSize(participant, participantW, 8.4, 13, 0.58);
  const bestSize = fitSingleLineTextSize(bestLabel, bestW, 8.2, 12, 0.58);
  return (
    <g
      className="leaderboard-page-svg-clickable"
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      role="button"
      tabIndex={0}
      aria-label={`Select ${row.player}`}
    >
      {hovered && !selected ? (
        <rect
          x={x - 3}
          y={y - 3}
          width={w + 6}
          height={cfg.chrome.rowHeight + 6}
          rx={6}
          fill="none"
          stroke={color}
          strokeWidth={1.3}
          opacity={0.25}
          filter="url(#leaderboardGlow)"
        />
      ) : null}
      <rect
        x={x}
        y={y}
        width={w}
        height={cfg.chrome.rowHeight}
        rx={4}
        fill={selected ? 'rgba(38, 49, 132, 0.8)' : hovered ? `${color}18` : row.rank % 2 === 0 ? '#071a2b' : '#051422'}
        stroke={lit ? color : '#123f62'}
        strokeWidth={lit ? 1.2 : 0.7}
      />
      <text x={p.rankCenter} y={y + 23} textAnchor="middle" fontSize={13} fontWeight={760} fill={cfg.colors.bodyText}>
        {row.rank}
      </text>
      <PlayerAvatarSlot
        cx={p.avatarCenter + cfg.chrome.avatarRadius}
        cy={y + cfg.chrome.rowHeight / 2}
        r={cfg.chrome.avatarRadius}
        player={row.player}
        tone={row.tone}
        cfg={cfg}
      />
      <text x={p.participant} y={y + 23} fontSize={participantSize} fontWeight={700} fill={cfg.colors.bodyText}>
        {participant}
      </text>
      <text x={p.score} y={y + 23} fontSize={13} fontWeight={760} fill={cfg.colors.bodyText}>
        {row.rating}
      </text>
      <CircleDot x={p.marker} y={y + 12} width={10} height={10} color={color} strokeWidth={2.3} />
      <text x={p.games} y={y + 23} fontSize={13} fontWeight={680} fill="#e5f7ff">
        {row.games}
      </text>
      <text x={p.wins} y={y + 23} fontSize={13} fontWeight={680} fill="#e5f7ff">
        {row.wins}
      </text>
      <text x={p.rate} y={y + 23} fontSize={13} fontWeight={680} fill="#e5f7ff">
        {row.winRate}
      </text>
      <GameArtBadge x={p.best} y={y + 7} size={24} gameId={bestLabel} tone={row.tone} cfg={cfg} />
      <text x={p.bestText} y={y + 23} fontSize={bestSize} fontWeight={690} fill={cfg.colors.bodyText}>
        {bestLabel}
      </text>
      <BadgeSet x={p.badges + 18} y={y + 17} tone={row.tone} cfg={cfg} />
      <text
        x={p.trendCenter}
        y={y + 23}
        textAnchor="middle"
        fontSize={13}
        fontWeight={800}
        fill={isUp ? '#57ff9a' : isDown ? '#ff4c60' : '#e8f4ff'}
      >
        {row.trend}
      </text>
    </g>
  );
}

const LEADERBOARD_TOP_CARD_MIN_W = 300;
const LEADERBOARD_GAME_CARD_MIN_W = 245;

function leaderFrameTone(row: DisplayRow): 'gold' | 'silver' | 'bronze' | 'blue' | 'red' {
  return 'blue';
}

function LeaderboardPictureViewerFrameLines({
  frame,
  segments,
  filterId,
}: {
  frame: PictureViewerFrameControls;
  segments: PictureViewerFrameSegment[];
  filterId: string;
}) {
  return (
    <g opacity={frame.opacity ?? 1} pointerEvents="none">
      {frame.glowEnabled ? (
        <g filter={`url(#${filterId})`} opacity={frame.glowOpacity}>
          {segments.map((segment) => (
            <path
              key={`deck-preview-frame-glow-${segment.id}`}
              d={segment.d}
              fill="none"
              stroke={frame.glowColor}
              strokeWidth={pictureViewerFrameSegmentThickness(frame, segment) + frame.glowWidthBoost}
              strokeLinecap={frame.lineCap}
              strokeLinejoin="round"
              vectorEffect="non-scaling-stroke"
            />
          ))}
        </g>
      ) : null}
      {frame.outlineEnabled ? (
        <g opacity={frame.outlineOpacity}>
          {segments.map((segment) => (
            <path
              key={`deck-preview-frame-outline-${segment.id}`}
              d={segment.d}
              fill="none"
              stroke={pictureViewerDarkenHex(frame.color, 0.48)}
              strokeWidth={pictureViewerFrameSegmentThickness(frame, segment) + frame.outlineWidthBoost}
              strokeLinecap={frame.lineCap}
              strokeLinejoin="round"
              vectorEffect="non-scaling-stroke"
            />
          ))}
        </g>
      ) : null}
      {segments.map((segment) => (
        <path
          key={`deck-preview-frame-${segment.id}`}
          d={segment.d}
          fill="none"
          stroke={frame.color}
          strokeWidth={pictureViewerFrameSegmentThickness(frame, segment)}
          strokeLinecap={frame.lineCap}
          strokeLinejoin="round"
          vectorEffect="non-scaling-stroke"
        />
      ))}
    </g>
  );
}

const SIDE_PANEL_CORNER_SEGMENT_IDS = [
  'leftTopCorner',
  'topLeftRunStart',
  'topRightRunEnd',
  'rightTopCorner',
  'rightSideRunStart',
  'rightSideRunEnd',
  'rightBottomCorner',
  'bottomRightRunStart',
  'bottomLeftRunEnd',
  'leftBottomCorner',
  'leftSideRunStart',
  'leftSideRunEnd',
] as const;

function scaleSidePanelCornerSegments(frame: PictureViewerFrameControls, scale: number): PictureViewerFrameControls {
  if (scale === 1) {
    return frame;
  }
  const segmentThicknesses = { ...frame.segmentThicknesses };
  for (const segmentId of SIDE_PANEL_CORNER_SEGMENT_IDS) {
    const value = segmentThicknesses[segmentId];
    if (typeof value === 'number') {
      segmentThicknesses[segmentId] = Math.max(0.5, value * scale);
    }
  }
  return {
    ...frame,
    segmentThicknesses,
  };
}

function LeaderboardSidePanelFrame({
  x,
  y,
  w,
  h,
  tone,
  active,
  cornerThicknessScale = 1,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  tone: Tone;
  active: boolean;
  cornerThicknessScale?: number;
  cfg: LeaderboardPageSvgControls;
}) {
  const color = toneColor(tone, cfg);
  const tall = h > w * 1.08;
  const baseViewBoxW = tall ? 1200 : 1600;
  const frameScale = Math.max(0.01, w / baseViewBoxW);
  const minViewBoxH = tall ? 1200 : 420;
  const viewBox = useMemo(
    () => ({ w: baseViewBoxW, h: Math.max(minViewBoxH, h / frameScale) }),
    [baseViewBoxW, frameScale, h, minViewBoxH]
  );
  const frameControls = useMemo(() => {
    const base = normalizePictureViewerFrameControls({
      orientation: tall ? 'portrait' : 'landscape',
      viewBox,
      frameGroup: { inset: 2, offsetX: 0, offsetY: 0 },
      outerAnchor: { sideInset: 24, topInset: 24, bottomInset: 24 },
      innerAnchor: { sideInset: 54, topInset: 58, bottomInset: 58 },
    });
    return {
      ...base,
      navArrows: {
        ...base.navArrows,
        enabled: false,
      },
      outerFrame: scaleSidePanelCornerSegments(
        {
          ...base.outerFrame,
          color,
          glowColor: color,
          glowEnabled: true,
          glowOpacity: active ? 0.34 : 0.2,
          glowBlur: active ? 18 : 11,
          glowWidthBoost: active ? 7 : 4,
          outlineOpacity: 0.78,
          outlineWidthBoost: 2,
          topRise: 0,
          cornerCut: tall ? 86 : 62,
          topStepWidth: tall ? 420 : 520,
          topStepInset: 0,
          bottomTabWidth: tall ? 420 : 520,
          bottomTabDepth: 0,
          bottomTabInset: 0,
          bottomTabDirection: 'down' as const,
          opacity: active ? 0.98 : 0.82,
        },
        cornerThicknessScale
      ),
      innerFrame: scaleSidePanelCornerSegments(
        {
          ...base.innerFrame,
          color,
          glowColor: color,
          glowEnabled: active,
          glowOpacity: active ? 0.2 : 0,
          glowBlur: 8,
          glowWidthBoost: 4,
          outlineOpacity: 0.62,
          outlineWidthBoost: 1.4,
          topRise: 0,
          cornerCut: tall ? 64 : 42,
          bottomTabDepth: 0,
          bottomTabWidth: tall ? 340 : 420,
          opacity: active ? 0.88 : 0.58,
        },
        cornerThicknessScale
      ),
    };
  }, [active, color, cornerThicknessScale, tall, viewBox]);
  const rawId = useId().replace(/[^a-zA-Z0-9_-]/g, '');
  const outerFrame = useMemo(
    () => getPictureViewerAnchoredFrame(frameControls, 'outerFrame', 'outerAnchor'),
    [frameControls]
  );
  const innerFrame = useMemo(
    () => getPictureViewerAnchoredFrame(frameControls, 'innerFrame', 'innerAnchor'),
    [frameControls]
  );
  const outerSegments = useMemo(() => pictureViewerFrameSegments(outerFrame), [outerFrame]);
  const innerSegments = useMemo(() => pictureViewerFrameSegments(innerFrame), [innerFrame]);
  const outerGlowId = `leaderboardSidePanelFrameOuterGlow-${rawId}`;
  const innerGlowId = `leaderboardSidePanelFrameInnerGlow-${rawId}`;

  return (
    <g transform={`translate(${x} ${y}) scale(${frameScale})`} opacity={active ? 1 : 0.86} pointerEvents="none">
      <defs>
        <filter id={outerGlowId} x="-40%" y="-40%" width="180%" height="180%">
          <feGaussianBlur stdDeviation={outerFrame.glowBlur} />
        </filter>
        <filter id={innerGlowId} x="-40%" y="-40%" width="180%" height="180%">
          <feGaussianBlur stdDeviation={innerFrame.glowBlur} />
        </filter>
      </defs>
      <g transform={getPictureViewerFrameGroupTransform(frameControls)}>
        <g transform={getPictureViewerFrameTransform(frameControls)}>
          <LeaderboardPictureViewerFrameLines frame={outerFrame} segments={outerSegments} filterId={outerGlowId} />
          <LeaderboardPictureViewerFrameLines frame={innerFrame} segments={innerSegments} filterId={innerGlowId} />
        </g>
      </g>
    </g>
  );
}

function LeaderboardTopCarouselCard({
  item,
  x,
  y,
  w,
  h,
  selected,
  onSelect,
  onHoverChange,
  gameHoverAnchor = 'center',
  cfg,
}: {
  item: LeaderboardTopCardItem;
  x: number;
  y: number;
  w: number;
  h: number;
  selected: boolean;
  onSelect: () => void;
  onHoverChange?: (item: LeaderboardTopCardItem | null) => void;
  gameHoverAnchor?: 'center' | 'up';
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const rawAvatarClipId = useId();
  const rawGameClipId = useId();
  const avatarClipId = `leader-avatar-${rawAvatarClipId.replace(/:/g, '')}`;
  const gameClipId = `leader-game-card-${rawGameClipId.replace(/:/g, '')}`;
  const color = toneColor(item.tone, cfg);
  const active = selected || hovered;
  const leaderFrameHref = useMemo(() => {
    if (item.kind !== 'leader') return '';
    const frameConfig = createGoldenFrameVariantConfig({
      rank: String(item.row.rank),
      name: item.row.player,
      statName: 'Global',
      statValue: item.row.rating,
      tone: leaderFrameTone(item.row),
    });
    return createGoldenFrameFrameOnlySvgDataUri(frameConfig);
  }, [item]);
  const compactGameCard = item.kind === 'game' && !hovered;
  const gameW = item.kind === 'game' ? (hovered ? Math.min(w + 28, w * 1.08) : w) : w;
  const gameH = item.kind === 'game' ? (hovered ? clampValue(h * 1.82, 86, 112) : clampValue(h * 0.62, 38, 44)) : h;
  const gameX = x + (w - gameW) / 2;
  const gameY = item.kind === 'game' && hovered && gameHoverAnchor === 'up' ? y + h - gameH - 4 : y + (h - gameH) / 2;
  const gamePad = Math.max(10, Math.min(14, gameW * 0.045));
  const gameBannerX = gameX + gamePad;
  const gameBannerY = gameY + 18;
  const gameBannerW = gameW - gamePad * 2;
  const gameBannerH = Math.max(78, Math.min(gameH * 0.68, gameH - 58));
  const gameBodyY = gameBannerY + gameBannerH + 18;
  const gameImageUrl = item.kind === 'game' ? leaderboardExplorerGameImageUrl(item.game) : null;
  const gameCategoryText = item.kind === 'game' ? gameSubcategoryLabel(item.game) : '';
  const gameTitleW = Math.max(58, gameW - gamePad * 2 - 4);
  const gameTitleSize = fitSingleLineTextSize(item.title, gameTitleW, 13.5, 20, 0.56);
  const gameTitleBaseline = Math.min(gameY + gameH - 16, gameBodyY + gameTitleSize * 0.42);
  const gameCategorySize = fitSingleLineTextSize(gameCategoryText, gameBannerW - 20, 7.5, 10.5, 0.56);
  const gameTitleText = truncateTextForWidth(item.title, gameTitleW, gameTitleSize, 0.56);
  const gameCategoryDisplayText = truncateTextForWidth(gameCategoryText, gameBannerW - 34, gameCategorySize, 0.56);
  const compactGameImageSize = Math.min(31, Math.max(23, gameH - 16));
  const compactGameImageX = gameX + 10;
  const compactGameImageY = gameY + (gameH - compactGameImageSize) / 2;
  const compactGameChipValue = item.kind === 'game' ? compactGameStatLabel(item.value) : '';
  const compactGameChipText = /\d|%/.test(compactGameChipValue) ? compactGameChipValue : '';
  const compactGameChipW = compactGameChipText ? Math.max(38, Math.min(54, compactGameChipText.length * 6.2 + 20)) : 0;
  const compactGameTitleX = compactGameImageX + compactGameImageSize + 14;
  const compactGameTitleW = Math.max(46, gameX + gameW - compactGameTitleX - compactGameChipW - 18);
  const compactGameTitleSize = fitSingleLineTextSize(item.title, compactGameTitleW, 11.8, 14.2, 0.58);
  const leaderFrameScale = Math.min(w / 1536, h / 864);
  const leaderFrameW = 1536 * leaderFrameScale;
  const leaderFrameH = 864 * leaderFrameScale;
  const leaderFrameX = x + (w - leaderFrameW) / 2;
  const leaderFrameY = y + (h - leaderFrameH) / 2;
  const leaderHoverScale = hovered ? 1.075 : selected ? 1.012 : 1;
  const leaderScaleCx = x + w / 2;
  const leaderScaleCy = y + h / 2;
  const scaleFromCenter = (value: number, center: number) => center + (value - center) * leaderHoverScale;
  const leaderDrawFrameX = scaleFromCenter(leaderFrameX, leaderScaleCx);
  const leaderDrawFrameY = scaleFromCenter(leaderFrameY, leaderScaleCy);
  const leaderDrawFrameW = leaderFrameW * leaderHoverScale;
  const leaderDrawFrameH = leaderFrameH * leaderHoverScale;
  const leaderHoverBoxX = leaderDrawFrameX + leaderDrawFrameW * 0.035;
  const leaderHoverBoxY = leaderDrawFrameY + leaderDrawFrameH * 0.13;
  const leaderHoverBoxW = leaderDrawFrameW * 0.93;
  const leaderHoverBoxH = leaderDrawFrameH * 0.72;
  const leaderAvatarR = 206 * leaderFrameScale;
  const leaderAvatarSize = leaderAvatarR * 2 * 1.22;
  const leaderAvatarCx = leaderFrameX + 768 * leaderFrameScale;
  const leaderAvatarCy = leaderFrameY + 312 * leaderFrameScale;
  const leaderAvatarX = leaderAvatarCx - leaderAvatarSize / 2;
  const leaderAvatarY = leaderAvatarCy - leaderAvatarSize / 2 + 10 * leaderFrameScale;
  const leaderDrawAvatarR = leaderAvatarR * leaderHoverScale;
  const leaderDrawAvatarCx = scaleFromCenter(leaderAvatarCx, leaderScaleCx);
  const leaderDrawAvatarCy = scaleFromCenter(leaderAvatarCy, leaderScaleCy);
  const leaderDrawAvatarSize = leaderAvatarSize * leaderHoverScale;
  const leaderDrawAvatarX = scaleFromCenter(leaderAvatarX, leaderScaleCx);
  const leaderDrawAvatarY = scaleFromCenter(leaderAvatarY, leaderScaleCy);
  const leaderAvatarUrl = item.kind === 'leader' ? playerAvatarImageUrl(item.row.player) : '';
  const guidePad = Math.max(12, Math.min(18, w * 0.045));
  const guideRankW = 42;
  const guideTitleX = x + guidePad + guideRankW + 10;
  const guideTitleW = Math.max(60, w - guidePad * 2 - guideRankW - 12);
  const guideTitleSize = fitSingleLineTextSize(item.title, guideTitleW, 13, 18, 0.56);
  const guideSubtitleLines = wrapCardText(item.subtitle, guideTitleW, 10.6, 2);
  const guideDetailLines = wrapCardText(item.detail, w - guidePad * 2, 10.2, h > 102 ? 2 : 1);
  const leaderHoverStrokeWidth = hovered ? 2.4 : selected ? 1.35 : 1.5;
  const leaderHoverOuterOpacity = hovered ? 0.72 : selected ? 0.34 : 0.38;
  const leaderHoverInnerOpacity = hovered ? 0.82 : selected ? 0.48 : 0.52;
  const leaderHoverFill = hovered
    ? colorAlpha(color, '2c')
    : selected
      ? 'rgba(255, 210, 59, 0.10)'
      : colorAlpha(color, '16');
  const hitX = item.kind === 'game' ? Math.min(x - 4, gameX - 8) : x - 4;
  const hitY = item.kind === 'game' ? Math.min(y - 4, gameY - 8) : y - 4;
  const hitW = item.kind === 'game' ? Math.max(x + w + 4, gameX + gameW + 8) - hitX : w + 8;
  const hitH = item.kind === 'game' ? Math.max(y + h + 4, gameY + gameH + 8) - hitY : h + 8;
  const showHoverState = () => {
    setHovered(true);
    if (item.kind === 'game') onHoverChange?.(item);
  };
  const clearHoverState = () => {
    setHovered(false);
    if (item.kind === 'game') onHoverChange?.(null);
  };
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={
        item.kind === 'game'
          ? `Show ${item.title} controls`
          : item.kind === 'guide'
            ? `Show ${item.title} guide`
            : `Show ${item.title} control row`
      }
      aria-pressed={selected}
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={showHoverState}
      onMouseOver={showHoverState}
      onMouseMove={showHoverState}
      onPointerEnter={showHoverState}
      onPointerMove={showHoverState}
      onMouseLeave={clearHoverState}
      onPointerLeave={clearHoverState}
    >
      <rect x={hitX} y={hitY} width={hitW} height={hitH} fill="transparent" pointerEvents="all" />
      {item.kind === 'game' && active && !compactGameCard ? (
        <path
          d={cutRectPath(x, y, w, h, 14)}
          fill={colorAlpha(color, selected ? '24' : '12')}
          stroke={selected ? '#ffe187' : color}
          strokeWidth={selected ? 2.1 : 1.4}
          opacity={selected ? 0.82 : 0.5}
          filter={selected ? 'url(#leaderboardGoldGlow)' : 'url(#leaderboardGlow)'}
          pointerEvents="none"
        />
      ) : null}
      {item.kind === 'leader' ? (
        <>
          {active ? (
            <>
              <path
                d={cutRectPath(
                  leaderHoverBoxX - 8,
                  leaderHoverBoxY - 8,
                  leaderHoverBoxW + 16,
                  leaderHoverBoxH + 16,
                  13
                )}
                fill="none"
                stroke={selected ? '#ffe187' : color}
                strokeWidth={leaderHoverStrokeWidth}
                opacity={leaderHoverOuterOpacity}
                filter={selected ? 'url(#leaderboardGoldGlow)' : 'url(#leaderboardGlow)'}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(leaderHoverBoxX, leaderHoverBoxY, leaderHoverBoxW, leaderHoverBoxH, 10)}
                fill={leaderHoverFill}
                stroke={selected ? '#ffe187' : color}
                strokeWidth={hovered ? 1.6 : 1.1}
                strokeOpacity={leaderHoverInnerOpacity}
                pointerEvents="none"
              />
            </>
          ) : null}
          <image
            href={leaderFrameHref}
            xlinkHref={leaderFrameHref}
            x={leaderDrawFrameX}
            y={leaderDrawFrameY}
            width={leaderDrawFrameW}
            height={leaderDrawFrameH}
            preserveAspectRatio="xMidYMid meet"
            filter={active ? (selected ? 'url(#leaderboardGoldGlow)' : 'url(#leaderboardGlow)') : undefined}
            pointerEvents="none"
          />
          <text
            x={leaderDrawFrameX + leaderDrawFrameW * 0.13}
            y={leaderDrawFrameY + leaderDrawFrameH * 0.36}
            fontSize={Math.max(12, leaderDrawFrameH * 0.1)}
            fontWeight={950}
            fill={cfg.colors.bodyText}
            pointerEvents="none"
          >
            {item.title}
          </text>
          <text
            x={leaderDrawFrameX + leaderDrawFrameW * 0.13}
            y={leaderDrawFrameY + leaderDrawFrameH * 0.52}
            fontSize={Math.max(8, leaderDrawFrameH * 0.064)}
            fontWeight={820}
            fill={cfg.colors.mutedText}
            pointerEvents="none"
          >
            {item.row.bestGame}
          </text>
          <text
            x={leaderDrawFrameX + leaderDrawFrameW * 0.72}
            y={leaderDrawFrameY + leaderDrawFrameH * 0.42}
            textAnchor="middle"
            fontSize={Math.max(11, leaderDrawFrameH * 0.082)}
            fontWeight={950}
            fill={color}
            pointerEvents="none"
          >
            {item.row.trend}
          </text>
          <text
            x={leaderDrawFrameX + leaderDrawFrameW * 0.72}
            y={leaderDrawFrameY + leaderDrawFrameH * 0.56}
            textAnchor="middle"
            fontSize={Math.max(8, leaderDrawFrameH * 0.056)}
            fontWeight={760}
            fill={cfg.colors.mutedText}
            pointerEvents="none"
          >
            {item.detail}
          </text>
        </>
      ) : item.kind === 'guide' ? (
        <>
          <path
            d={cutRectPath(x, y, w, h, 12)}
            fill={active ? colorAlpha(color, selected ? '2e' : '20') : 'rgba(5, 17, 30, 0.88)'}
            stroke={selected ? '#ffe187' : color}
            strokeWidth={selected ? 2 : hovered ? 1.55 : 1.05}
            strokeOpacity={selected ? 0.92 : hovered ? 0.8 : 0.56}
            filter={selected ? 'url(#leaderboardGoldGlow)' : hovered ? 'url(#leaderboardGlow)' : undefined}
            pointerEvents="none"
          />
          <path
            d={cutRectPath(x + 5, y + 5, w - 10, h - 10, 10)}
            fill="url(#leaderboardFrameGlass)"
            stroke={color}
            strokeWidth={0.75}
            strokeOpacity={active ? 0.48 : 0.26}
            pointerEvents="none"
          />
          <path
            d={bottomCutRectPath(x + w * 0.34, y - 5, w * 0.32, 12, 5)}
            fill={color}
            fillOpacity={active ? 0.42 : 0.24}
            stroke={selected ? '#ffe187' : color}
            strokeWidth={1}
            filter="url(#leaderboardGlow)"
            pointerEvents="none"
          />
          <path
            d={cutRectPath(x + guidePad, y + 17, guideRankW, 30, 7)}
            fill={colorAlpha(color, selected ? '44' : '28')}
            stroke={selected ? '#ffe187' : color}
            strokeWidth={selected ? 1.2 : 0.8}
            pointerEvents="none"
          />
          <text
            x={x + guidePad + guideRankW / 2}
            y={y + 37}
            textAnchor="middle"
            fontSize={13}
            fontWeight={950}
            fill={selected ? '#fff3b2' : color}
            pointerEvents="none"
          >
            {item.topic.rank}
          </text>
          <text
            x={guideTitleX}
            y={y + 32}
            fontSize={guideTitleSize}
            fontWeight={950}
            fill={cfg.colors.bodyText}
            pointerEvents="none"
          >
            {truncateTextForWidth(item.title, guideTitleW, guideTitleSize, 0.56)}
          </text>
          {guideSubtitleLines.map((line, index) => (
            <text
              key={`${item.key}:subtitle:${index}`}
              x={guideTitleX}
              y={y + 51 + index * 14}
              fontSize={10.5}
              fontWeight={760}
              fill={cfg.colors.mutedText}
              pointerEvents="none"
            >
              {line}
            </text>
          ))}
          <path
            d={`M ${x + guidePad} ${y + h - 42} H ${x + w - guidePad}`}
            stroke={color}
            strokeWidth={0.8}
            opacity={active ? 0.5 : 0.28}
            pointerEvents="none"
          />
          {guideDetailLines.map((line, index) => (
            <text
              key={`${item.key}:detail:${index}`}
              x={x + guidePad}
              y={y + h - 25 + index * 13}
              fontSize={10.2}
              fontWeight={800}
              fill={index === 0 ? color : cfg.colors.mutedText}
              pointerEvents="none"
            >
              {line}
            </text>
          ))}
        </>
      ) : (
        <>
          {compactGameCard ? (
            <>
              <path
                d={cutRectPath(gameX, gameY, gameW, gameH, 8)}
                fill={selected ? colorAlpha(color, '30') : 'rgba(4, 16, 28, 0.82)'}
                stroke={selected ? '#ffe187' : color}
                strokeWidth={selected ? 1.65 : 0.95}
                strokeOpacity={selected ? 0.96 : 0.7}
                filter={selected ? 'url(#leaderboardGoldGlow)' : undefined}
                pointerEvents="none"
              />
              <ArtworkSlot
                x={compactGameImageX}
                y={compactGameImageY}
                w={compactGameImageSize}
                h={compactGameImageSize}
                label={`${item.title} image`}
                imageUrl={gameImageUrl}
                tone={item.tone}
                compact
                shape="rect"
                imageFit="slice"
                cfg={cfg}
              />
              <line
                x1={compactGameImageX + compactGameImageSize + 7}
                y1={gameY + 8}
                x2={compactGameImageX + compactGameImageSize + 7}
                y2={gameY + gameH - 8}
                stroke={color}
                strokeWidth={0.85}
                opacity={selected ? 0.64 : 0.42}
              />
              <text
                x={compactGameTitleX}
                y={gameY + gameH / 2 + compactGameTitleSize * 0.34}
                fontSize={compactGameTitleSize}
                fontWeight={950}
                fill={cfg.colors.bodyText}
              >
                {truncateTextForWidth(item.title, compactGameTitleW, compactGameTitleSize, 0.58)}
              </text>
              {compactGameChipText ? (
                <>
                  <path
                    d={cutRectPath(
                      gameX + gameW - compactGameChipW - 12,
                      gameY + (gameH - 24) / 2,
                      compactGameChipW,
                      24,
                      5
                    )}
                    fill={colorAlpha(color, selected ? '34' : '18')}
                    stroke={color}
                    strokeWidth={0.8}
                    strokeOpacity={0.72}
                    pointerEvents="none"
                  />
                  <text
                    x={gameX + gameW - 12 - compactGameChipW / 2}
                    y={gameY + gameH / 2 + 4}
                    textAnchor="middle"
                    fontSize={10.2}
                    fontWeight={950}
                    fill={color}
                  >
                    {compactGameChipText}
                  </text>
                </>
              ) : null}
            </>
          ) : (
            <>
              {active ? (
                <path
                  d={cutRectPath(gameX - 5, gameY - 5, gameW + 10, gameH + 10, 16)}
                  fill="none"
                  stroke={selected ? '#ffe187' : color}
                  strokeWidth={hovered ? 2.3 : 1.6}
                  opacity={hovered ? 0.48 : 0.34}
                  filter={selected ? 'url(#leaderboardGoldGlow)' : 'url(#leaderboardGlow)'}
                  pointerEvents="none"
                />
              ) : null}
              <path
                d={cutRectPath(gameX, gameY, gameW, gameH, 15)}
                fill={active ? colorAlpha(color, selected ? '24' : '18') : 'rgba(6, 18, 31, 0.95)'}
                stroke={active ? (selected ? '#ffe187' : color) : cfg.colors.panelStroke}
                strokeWidth={selected ? 2 : hovered ? 1.65 : 1.05}
                strokeOpacity={active ? 0.94 : 0.68}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(gameX + 4, gameY + 4, gameW - 8, gameH - 8, 13)}
                fill="url(#leaderboardFrameGlass)"
                stroke={color}
                strokeWidth={active ? 1.1 : 0.7}
                opacity={active ? 0.56 : 0.36}
                pointerEvents="none"
              />
              <path
                d={bottomCutRectPath(gameX + gameW * 0.32, gameY - 6, gameW * 0.36, 13, 5)}
                fill="#8ceeff"
                fillOpacity={0.34}
                stroke="#9eefff"
                strokeWidth={1.1}
                filter="url(#leaderboardGlow)"
                pointerEvents="none"
              />
              <path
                d={bottomCutRectPath(gameX + gameW * 0.34, gameY - 3, gameW * 0.32, 7, 3)}
                fill="#d9fbff"
                fillOpacity={0.48}
                pointerEvents="none"
              />
              <defs>
                <clipPath id={gameClipId}>
                  <path d={cutRectPath(gameBannerX, gameBannerY, gameBannerW, gameBannerH, 11)} />
                </clipPath>
              </defs>
              {gameImageUrl ? (
                <image
                  href={gameImageUrl}
                  x={gameBannerX}
                  y={gameBannerY}
                  width={gameBannerW}
                  height={gameBannerH}
                  preserveAspectRatio="xMidYMid slice"
                  clipPath={`url(#${gameClipId})`}
                  opacity={0.96}
                  pointerEvents="none"
                />
              ) : (
                <ArtworkSlot
                  x={gameBannerX}
                  y={gameBannerY}
                  w={gameBannerW}
                  h={gameBannerH}
                  label={`${item.title} image`}
                  imageUrl={null}
                  tone={item.tone}
                  shape="rect"
                  cfg={cfg}
                />
              )}
              <path
                d={cutRectPath(gameBannerX, gameBannerY, gameBannerW, gameBannerH, 11)}
                fill="url(#leaderboardGameCardBannerShade)"
                clipPath={`url(#${gameClipId})`}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(gameBannerX, gameBannerY, gameBannerW, gameBannerH, 11)}
                fill="none"
                stroke={color}
                strokeWidth={active ? 1.2 : 0.85}
                strokeOpacity={active ? 0.86 : 0.56}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(
                  gameBannerX + 8,
                  gameBannerY + 7,
                  Math.min(gameBannerW - 16, gameCategoryDisplayText.length * 5.8 + 28),
                  20,
                  6
                )}
                fill="rgba(4, 11, 24, 0.72)"
                stroke="rgba(255,255,255,0.16)"
                strokeWidth={0.7}
                pointerEvents="none"
              />
              <text
                x={gameBannerX + 18}
                y={gameBannerY + 21.5}
                fontSize={gameCategorySize}
                fontWeight={900}
                fill="#e6f9ff"
              >
                {gameCategoryDisplayText}
              </text>
              <text
                x={gameX + gamePad + 2}
                y={gameTitleBaseline}
                fontSize={gameTitleSize}
                fontWeight={950}
                fill={cfg.colors.bodyText}
              >
                {gameTitleText}
              </text>
            </>
          )}
        </>
      )}
    </g>
  );
}

function LeaderboardTopCarousel({
  x,
  y,
  w,
  h,
  items,
  page,
  selectedKey,
  onSelect,
  onHoverChange,
  minCardW = LEADERBOARD_TOP_CARD_MIN_W,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  items: LeaderboardTopCardItem[];
  page: number;
  selectedKey: string;
  onSelect: (item: LeaderboardTopCardItem) => void;
  onHoverChange?: (item: LeaderboardTopCardItem | null) => void;
  minCardW?: number;
  cfg: LeaderboardPageSvgControls;
}) {
  const allLeaderItems = items.length > 0 && items.every((item) => item.kind === 'leader');
  const gap = allLeaderItems ? 10 : 16;
  const visibleCount = Math.max(
    1,
    Math.min(items.length || 1, LEADERBOARD_TOP_CAROUSEL_MAX_VISIBLE, Math.floor((w + gap) / (minCardW + gap)))
  );
  const pageCount = Math.max(1, Math.ceil(items.length / visibleCount));
  const safePage = wrapIndex(page, pageCount);
  const visibleItems = items.slice(safePage * visibleCount, safePage * visibleCount + visibleCount);
  const stretchedCardW =
    visibleItems.length > 0 ? (w - gap * Math.max(0, visibleItems.length - 1)) / visibleItems.length : w;
  const leaderMaxCardW = visibleItems.length <= 3 ? 370 : 340;
  const cardW = allLeaderItems ? Math.min(stretchedCardW, leaderMaxCardW) : stretchedCardW;
  const trackW = visibleItems.length > 0 ? visibleItems.length * cardW + Math.max(0, visibleItems.length - 1) * gap : 0;
  const startX = x + Math.max(0, (w - trackW) / 2);
  return (
    <g>
      {visibleItems.map((item, index) => {
        const selected = item.key === selectedKey || item.key.startsWith(`${selectedKey}:`);
        return (
          <LeaderboardTopCarouselCard
            key={item.key}
            item={item}
            x={startX + index * (cardW + gap)}
            y={y}
            w={cardW}
            h={h}
            selected={selected}
            onSelect={() => onSelect(item)}
            onHoverChange={onHoverChange}
            cfg={cfg}
          />
        );
      })}
      {items.length === 0 ? (
        <path
          d={cutRectPath(x, y, w, h, 8)}
          fill="rgba(6, 20, 34, 0.7)"
          stroke={cfg.colors.cyan}
          strokeWidth={0.8}
          strokeOpacity={0.5}
        />
      ) : null}
    </g>
  );
}

function LeaderboardGameList({
  x,
  y,
  w,
  h,
  items,
  page,
  visibleCount,
  selectedKey,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  items: LeaderboardTopCardItem[];
  page: number;
  visibleCount: number;
  selectedKey: string;
  onSelect: (item: LeaderboardTopCardItem) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const pageCount = Math.max(1, Math.ceil(items.length / Math.max(1, visibleCount)));
  const safePage = wrapIndex(page, pageCount);
  const visibleItems = items.slice(safePage * visibleCount, safePage * visibleCount + visibleCount);
  const gap = 6;
  const rowH =
    visibleItems.length > 0 ? Math.min(46, (h - gap * Math.max(0, visibleItems.length - 1)) / visibleItems.length) : h;
  const startY = y + Math.max(0, (h - (rowH * visibleItems.length + gap * Math.max(0, visibleItems.length - 1))) / 2);
  return (
    <g>
      {visibleItems.map((item, index) => {
        const rowY = startY + index * (rowH + gap);
        const selected = item.key === selectedKey || item.key.startsWith(`${selectedKey}:`);
        const color = toneColor(item.tone, cfg);
        const game = item.kind === 'game' ? item.game : null;
        const titleW = Math.max(50, w - 238);
        const titleSize = fitSingleLineTextSize(item.title, titleW, 10.5, 13.2, 0.56);
        return (
          <g
            key={item.key}
            className="leaderboard-page-svg-clickable"
            role="button"
            tabIndex={0}
            aria-label={item.kind === 'game' ? `Show ${item.title} controls` : `Show ${item.title}`}
            aria-pressed={selected}
            onClick={(event) => {
              event.stopPropagation();
              onSelect(item);
            }}
            onKeyDown={(event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              event.stopPropagation();
              onSelect(item);
            }}
          >
            <path
              d={cutRectPath(x, rowY, w, rowH, 8)}
              fill={selected ? colorAlpha(color, '2e') : 'rgba(5, 19, 32, 0.88)'}
              stroke={selected ? '#ffe187' : color}
              strokeWidth={selected ? 1.35 : 0.8}
              strokeOpacity={selected ? 0.92 : 0.62}
              filter={selected ? 'url(#leaderboardGlow)' : undefined}
              pointerEvents="none"
            />
            {game ? (
              <ArtworkSlot
                x={x + 12}
                y={rowY + 6}
                w={rowH - 12}
                h={rowH - 12}
                label="GAME"
                imageUrl={leaderboardGameImageUrl(game.id)}
                tone={item.tone}
                compact
                shape="rect"
                cfg={cfg}
              />
            ) : null}
            <line
              x1={x + 58}
              y1={rowY + 8}
              x2={x + 58}
              y2={rowY + rowH - 8}
              stroke={color}
              strokeWidth={0.85}
              opacity={0.45}
            />
            <text
              x={x + 72}
              y={rowY + rowH / 2 + titleSize * 0.34}
              fontSize={titleSize}
              fontWeight={950}
              fill={cfg.colors.bodyText}
            >
              {item.title}
            </text>
            <text
              x={x + w - 154}
              y={rowY + rowH / 2 + 3.5}
              fontSize={10.5}
              fontWeight={900}
              fill={cfg.colors.mutedText}
            >
              {item.subtitle}
            </text>
            <text
              x={x + w - 58}
              y={rowY + rowH / 2 + 4.5}
              textAnchor="end"
              fontSize={12.5}
              fontWeight={950}
              fill={color}
            >
              {item.value}
            </text>
            <path
              d={`M ${x + w - 26} ${rowY + rowH / 2 - 5} L ${x + w - 18} ${rowY + rowH / 2} L ${x + w - 26} ${rowY + rowH / 2 + 5}`}
              fill="none"
              stroke={color}
              strokeWidth={1.4}
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </g>
        );
      })}
      {items.length === 0 ? (
        <path
          d={cutRectPath(x, y, w, h, 8)}
          fill="rgba(6, 20, 34, 0.7)"
          stroke={cfg.colors.cyan}
          strokeWidth={0.8}
          strokeOpacity={0.5}
        />
      ) : null}
    </g>
  );
}

function GameCategoryCard({
  category,
  x,
  y,
  w,
  h,
  selected,
  onSelect,
  cfg,
}: {
  category: GameCategorySummary;
  x: number;
  y: number;
  w: number;
  h: number;
  selected: boolean;
  onSelect: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const active = selected || hovered;
  const color = toneColor(category.tone, cfg);
  const cx = x + w / 2;
  const cy = y + h / 2;
  const imageSize = Math.min(31, Math.max(23, h - 16));
  const imageX = x + 10;
  const imageY = y + (h - imageSize) / 2;
  const titleX = imageX + imageSize + 14;
  const countText = category.count > 0 ? String(category.count) : '-';
  const countW = 39;
  const titleW = Math.max(48, x + w - titleX - countW - 18);
  const titleSize = fitSingleLineTextSize(category.label, titleW, 11.8, 14.2, 0.58);
  const countX = x + w - countW - 12;
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Show ${category.label} controls`}
      aria-pressed={selected}
      transform={hovered ? `translate(${cx} ${cy}) scale(1.018) translate(${-cx} ${-cy})` : undefined}
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <rect x={x - 3} y={y - 3} width={w + 6} height={h + 6} fill="transparent" pointerEvents="all" />
      <ClickableCardHoverChrome x={x} y={y} w={w} h={h} color={color} active={selected} hovered={hovered} />
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '30') : hovered ? colorAlpha(color, '22') : 'rgba(4, 16, 28, 0.78)'}
        stroke={selected ? '#ffe187' : color}
        strokeWidth={selected ? 1.65 : hovered ? 1.35 : 0.95}
        strokeOpacity={active ? 0.96 : 0.68}
        filter={active ? (selected ? 'url(#leaderboardGoldGlow)' : 'url(#leaderboardGlow)') : undefined}
        pointerEvents="none"
      />
      <ArtworkSlot
        x={imageX}
        y={imageY}
        w={imageSize}
        h={imageSize}
        label={`${category.label} artwork`}
        imageUrl={leaderboardCategoryImageUrl(category)}
        tone={category.tone}
        compact
        shape="rect"
        imageFit="slice"
        cfg={cfg}
      />
      <line
        x1={imageX + imageSize + 7}
        y1={y + 8}
        x2={imageX + imageSize + 7}
        y2={y + h - 8}
        stroke={color}
        strokeWidth={0.85}
        opacity={active ? 0.64 : 0.42}
      />
      <text
        x={titleX}
        y={y + h / 2 + titleSize * 0.34}
        fontSize={titleSize}
        fontWeight={950}
        fill={cfg.colors.bodyText}
      >
        {category.label}
      </text>
      <path
        d={cutRectPath(countX, y + (h - 24) / 2, countW, 24, 5)}
        fill={colorAlpha(color, selected ? '34' : '18')}
        stroke={color}
        strokeWidth={0.8}
        strokeOpacity={0.72}
        pointerEvents="none"
      />
      <text x={countX + countW / 2} y={y + h / 2 + 4} textAnchor="middle" fontSize={10.2} fontWeight={950} fill={color}>
        {countText}
      </text>
    </g>
  );
}

function GameCategoryGrid({
  x,
  y,
  w,
  h,
  categories,
  selectedCategoryId,
  page,
  pageCount,
  handleLeftX,
  handleRightX,
  onPageChange,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  categories: GameCategorySummary[];
  selectedCategoryId: string;
  page: number;
  pageCount: number;
  handleLeftX?: number;
  handleRightX?: number;
  onPageChange: (page: number) => void;
  onSelect: (category: GameCategorySummary) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  if (categories.length === 0) return null;
  const gap = 10;
  const handleW = LEADERBOARD_SIDE_HANDLE_W;
  const handleH = Math.max(40, h - 10);
  const handleY = y + (h - handleH) / 2;
  const trackX = x;
  const trackW = Math.max(1, w);
  const cardW = (trackW - gap * Math.max(0, categories.length - 1)) / categories.length;
  return (
    <g>
      <LeaderboardFrameSideHandle
        x={handleLeftX ?? x}
        y={handleY}
        side="left"
        height={handleH}
        width={handleW}
        disabled={pageCount <= 1}
        onClick={() => onPageChange(wrapIndex(page - 1, pageCount))}
        cfg={cfg}
      />
      <LeaderboardFrameSideHandle
        x={handleRightX ?? x + w - handleW}
        y={handleY}
        side="right"
        height={handleH}
        width={handleW}
        disabled={pageCount <= 1}
        onClick={() => onPageChange(wrapIndex(page + 1, pageCount))}
        cfg={cfg}
      />
      {categories.map((category, index) => {
        const cardX = trackX + index * (cardW + gap);
        return (
          <GameCategoryCard
            key={category.id}
            category={category}
            x={cardX}
            y={y}
            w={cardW}
            h={h}
            selected={category.id === selectedCategoryId}
            onSelect={() => onSelect(category)}
            cfg={cfg}
          />
        );
      })}
    </g>
  );
}

function GameSubcategoryGrid({
  x,
  y,
  w,
  h,
  category,
  selectedSubcategoryId,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  category: GameCategorySummary;
  selectedSubcategoryId: string | null;
  onSelect: (subcategory: GameSubcategorySummary) => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const subcategories = category.subcategories;
  if (subcategories.length === 0) return null;
  const categoryColor = toneColor(category.tone, cfg);
  return (
    <foreignObject x={x} y={y} width={w} height={h}>
      <div
        className="leaderboard-subcategory-grid"
        style={
          {
            '--leaderboard-subcategory-accent': categoryColor,
            '--leaderboard-subcategory-text': cfg.colors.bodyText,
            '--leaderboard-subcategory-muted': cfg.colors.mutedText,
          } as CSSProperties
        }
        onClick={(event) => event.stopPropagation()}
        onWheel={(event) => event.stopPropagation()}
      >
        {subcategories.map((subcategory) => {
          const selected = selectedSubcategoryId === subcategory.id;
          const color = toneColor(subcategory.tone, cfg);
          const style = {
            '--leaderboard-subcategory-color': color,
            '--leaderboard-subcategory-fill': selected ? colorAlpha(color, '30') : 'rgba(5, 19, 32, 0.9)',
            '--leaderboard-subcategory-border': selected ? '#ffe187' : color,
          } as CSSProperties;
          return (
            <button
              key={subcategory.id}
              type="button"
              className={`leaderboard-subcategory-grid__item${selected ? ' leaderboard-subcategory-grid__item--selected' : ''}`}
              style={style}
              aria-label={`Filter ${category.label} by ${subcategory.label}`}
              aria-pressed={selected}
              onClick={(event) => {
                event.stopPropagation();
                onSelect(subcategory);
              }}
            >
              <span className="leaderboard-subcategory-grid__dot" />
              <span className="leaderboard-subcategory-grid__divider" />
              <span className="leaderboard-subcategory-grid__label">{subcategory.label}</span>
              <span className="leaderboard-subcategory-grid__count">{subcategory.count}</span>
            </button>
          );
        })}
      </div>
    </foreignObject>
  );
}

function PagerControl({
  x,
  y,
  w,
  h,
  tone = 'cyan',
  selected = false,
  disabled = false,
  ariaLabel,
  onClick,
  children,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  tone?: Tone;
  selected?: boolean;
  disabled?: boolean;
  ariaLabel: string;
  onClick: () => void;
  children: (active: boolean, color: string) => ReactNode;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const active = selected || hovered;
  const fill = selected ? cfg.colors.selectedFill : active ? colorAlpha(color, '26') : 'rgba(5, 21, 35, 0.92)';
  const handleClick = (event: MouseEvent<SVGGElement>) => {
    event.stopPropagation();
    if (!disabled) onClick();
  };
  const handleKeyDown = (event: KeyboardEvent<SVGGElement>) => {
    if (disabled || (event.key !== 'Enter' && event.key !== ' ')) return;
    event.preventDefault();
    event.stopPropagation();
    onClick();
  };
  return (
    <g
      className={!disabled ? 'leaderboard-page-svg-clickable' : undefined}
      role="button"
      tabIndex={disabled ? -1 : 0}
      aria-label={ariaLabel}
      aria-disabled={disabled || undefined}
      opacity={disabled ? 0.46 : 1}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      onMouseEnter={() => !disabled && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {active && !selected ? (
        <rect
          x={x - 3}
          y={y - 3}
          width={w + 6}
          height={h + 6}
          rx={3}
          fill="none"
          stroke={color}
          strokeWidth={1.6}
          opacity={0.24}
          filter="url(#leaderboardGlow)"
        />
      ) : null}
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={3}
        fill={fill}
        stroke={selected ? cfg.colors.purple : color}
        strokeWidth={selected ? 1.6 : 1.05}
        strokeOpacity={active ? 0.96 : 0.72}
      />
      <rect
        x={x + 2}
        y={y + 2}
        width={w - 4}
        height={Math.max(1, h * 0.36)}
        rx={2}
        fill="#ffffff"
        opacity={active ? 0.13 : 0.06}
      />
      {children(active, color)}
    </g>
  );
}

function ClickableCardHoverChrome({
  x,
  y,
  w,
  h,
  color,
  active,
  hovered,
  arrow = true,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  color: string;
  active: boolean;
  hovered: boolean;
  arrow?: boolean;
}) {
  if (!active && !hovered) return null;
  return (
    <>
      <path
        d={cutRectPath(x - 4, y - 4, w + 8, h + 8, 10)}
        fill="none"
        stroke={color}
        strokeWidth={active ? 1.9 : 1.45}
        opacity={active ? 0.42 : 0.32}
        filter="url(#leaderboardGlow)"
        pointerEvents="none"
      />
      {arrow ? (
        <path
          d={`M ${x + w - 10} ${y + 14} L ${x + w + 10} ${y + h / 2} L ${x + w - 10} ${y + h - 14} Z`}
          fill={color}
          opacity={active ? 0.72 : 0.56}
          filter="url(#leaderboardGlow)"
          pointerEvents="none"
        />
      ) : null}
    </>
  );
}

function PagerArrow({
  x,
  y,
  direction,
  active,
  color,
}: {
  x: number;
  y: number;
  direction: 'left' | 'right';
  active: boolean;
  color: string;
}) {
  const tipX = direction === 'left' ? x : x + 10;
  const backX = direction === 'left' ? x + 10 : x;
  return (
    <path
      d={`M ${tipX} ${y + 8} L ${backX} ${y} V ${y + 16} Z`}
      fill={active ? '#c9ffd8' : '#ecfbff'}
      stroke={active ? '#e7ffe8' : color}
      strokeWidth={0.6}
      filter={active ? 'url(#leaderboardGreenGlow)' : undefined}
      pointerEvents="none"
    />
  );
}

function Pagination({
  x,
  y,
  w,
  page,
  maxPage,
  rowsPerPage,
  pageLabel,
  showLabel,
  onPageChange,
  onRowsPerPageChange,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  page: number;
  maxPage: number;
  rowsPerPage: number;
  pageLabel: string;
  showLabel: string;
  onPageChange: (page: number) => void;
  onRowsPerPageChange: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const centerX = x + w / 2;
  const firstX = centerX - 170;
  const controlH = 22;
  const arrowW = 30;
  const pageW = 26;
  const pageStep = 34;
  const pagesX = firstX + 42;
  const nextX = pagesX + pageStep * Math.min(5, maxPage) + 4;
  const visiblePages = Array.from({ length: Math.min(5, maxPage) }, (_, index) => {
    const start = clampValue(page <= 3 ? 1 : page >= maxPage - 2 ? maxPage - 4 : page - 2, 1, Math.max(1, maxPage - 4));
    return start + index;
  });
  return (
    <g>
      <PagerControl
        x={firstX}
        y={y}
        w={arrowW}
        h={controlH}
        tone="cyan"
        disabled={page <= 1}
        onClick={() => onPageChange(Math.max(1, page - 1))}
        ariaLabel="Previous leaderboard page"
        cfg={cfg}
      >
        {(active, color) => <PagerArrow x={firstX + 10} y={y + 3} direction="left" active={active} color={color} />}
      </PagerControl>
      {visiblePages.map((slotPage, index) => (
        <PagerControl
          key={slotPage}
          x={pagesX + index * pageStep}
          y={y}
          w={pageW}
          h={controlH}
          tone={slotPage === page ? 'purple' : 'cyan'}
          selected={slotPage === page}
          onClick={() => onPageChange(slotPage)}
          ariaLabel={`Open leaderboard page ${slotPage}`}
          cfg={cfg}
        >
          {() => (
            <text
              x={pagesX + pageW / 2 + index * pageStep}
              y={y + 15.5}
              textAnchor="middle"
              fontSize={slotPage >= 100 ? 9.2 : 11.8}
              fontWeight={900}
              fill={cfg.colors.bodyText}
            >
              {slotPage}
            </text>
          )}
        </PagerControl>
      ))}
      <PagerControl
        x={nextX}
        y={y}
        w={arrowW}
        h={controlH}
        tone="cyan"
        disabled={page >= maxPage}
        onClick={() => onPageChange(Math.min(maxPage, page + 1))}
        ariaLabel="Next leaderboard page"
        cfg={cfg}
      >
        {(active, color) => <PagerArrow x={nextX + 10} y={y + 3} direction="right" active={active} color={color} />}
      </PagerControl>
      <text x={nextX + 42} y={y + 15.5} fontSize={9.8} fontWeight={850} fill={cfg.colors.mutedText}>
        {pageLabel} {page}/{maxPage}
      </text>
      <text x={x + w - 118} y={y + 16} fontSize={10.6} fontWeight={800} fill="#d7efff">
        {showLabel}
      </text>
      <PagerControl
        x={x + w - 72}
        y={y - 1}
        w={52}
        h={24}
        tone="cyan"
        onClick={onRowsPerPageChange}
        ariaLabel="Change rows shown"
        cfg={cfg}
      >
        {(active, color) => (
          <>
            <text
              x={x + w - 52}
              y={y + 16.5}
              textAnchor="middle"
              fontSize={11.8}
              fontWeight={900}
              fill={cfg.colors.bodyText}
            >
              {rowsPerPage}
            </text>
            <PagerArrow x={x + w - 36} y={y + 4} direction="right" active={active} color={color} />
          </>
        )}
      </PagerControl>
    </g>
  );
}

function MainBoard({
  activeNavLabel,
  activeNavGroupId,
  activeTab,
  rows,
  tabs,
  tabDetails,
  topGames,
  quickGames,
  guideTopics,
  season,
  uiCopy,
  selectedGameId,
  selectedPlayerId,
  selectedPlayer,
  selectedGameName,
  page,
  rowsPerPage,
  detailMode,
  onTabChange,
  onPlayerSelect,
  onPageChange,
  onRowsPerPageChange,
  onDetailOpen,
  onDetailClose,
  onGameSelect,
  onNavigate,
  onSelectNavLabel,
  onRefresh,
  onMatchmaking,
  cfg,
  mainX,
  mainW,
  mainY,
  mainH,
  rightX,
  rightW,
}: {
  activeNavLabel: string;
  activeNavGroupId: string;
  activeTab: LeaderboardTabId;
  rows: DisplayRow[];
  tabs: LeaderboardPageContentData['tabs'];
  tabDetails: LeaderboardPageContentData['tabDetails'];
  topGames: TopGame[];
  quickGames: QuickGame[];
  guideTopics: LeaderboardPageContentData['guideTopics'];
  season: LeaderboardPageContentData['season'];
  uiCopy: LeaderboardPageContentData['uiCopy'];
  selectedGameId: string;
  selectedPlayerId: string;
  selectedPlayer: DisplayRow;
  selectedGameName: string;
  page: number;
  rowsPerPage: number;
  detailMode: DetailMode | null;
  onTabChange: (tab: LeaderboardTabId) => void;
  onPlayerSelect: (playerId: string) => void;
  onPageChange: (page: number) => void;
  onRowsPerPageChange: () => void;
  onDetailOpen: (mode: DetailMode) => void;
  onDetailClose: () => void;
  onGameSelect: (gameId: string) => void;
  onNavigate?: (routePath: string) => void;
  onSelectNavLabel: (navLabel: string) => void;
  onRefresh: () => void;
  onMatchmaking: () => void;
  cfg: LeaderboardPageSvgControls;
  mainX: number;
  mainW: number;
  mainY: number;
  mainH: number;
  rightX: number;
  rightW: number;
}) {
  const activeTabConfig = tabs.find((tab) => tab.id === activeTab) ?? tabs[0];
  const rawTableClipId = useId();
  const tableClipId = `leaderboard-table-clip-${rawTableClipId.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const detail = detailForNav(activeNavLabel, tabDetails[activeTab]);
  const rankingTitle = activeTab === 'overall' ? 'PARENT CONTROL SNAPSHOT' : activeTabConfig.title;
  const activeNavKey = assetKey(activeNavLabel);
  const chatMode = activeNavKey.includes('chat');
  const guideOverviewMode = activeNavKey.includes('start-here');
  const guideEligible = !chatMode && activeNavGroupId === 'guide';
  const guideTopicPool = useMemo(
    () =>
      guideEligible
        ? guideOverviewMode
          ? guideTopics
          : guideTopics.filter((topic) => assetKey(topic.navLabel) === activeNavKey)
        : [],
    [activeNavKey, guideEligible, guideOverviewMode, guideTopics]
  );
  const guideMode = guideTopicPool.length > 0;
  const [selectedGuideTopicId, setSelectedGuideTopicId] = useState(() => guideTopicPool[0]?.id ?? '');
  const [guidePage, setGuidePage] = useState(0);
  const [guideQuickPanelMode, setGuideQuickPanelMode] = useState<'read' | 'action'>('read');
  const [guideDashboardDrilldown, setGuideDashboardDrilldown] = useState(false);
  const selectedGuideTopic =
    guideTopicPool.find((topic) => normalizeSelectionId(topic.id) === normalizeSelectionId(selectedGuideTopicId)) ??
    guideTopicPool[0] ??
    null;
  const guideDashboardMode = guideOverviewMode && !guideDashboardDrilldown;
  useEffect(() => {
    const firstTopic = guideTopicPool[0];
    if (!firstTopic) return;
    const selectedStillVisible = guideTopicPool.some(
      (topic) => normalizeSelectionId(topic.id) === normalizeSelectionId(selectedGuideTopicId)
    );
    if (selectedStillVisible) return;
    setSelectedGuideTopicId(firstTopic.id);
    setGuidePage(0);
  }, [guideTopicPool, selectedGuideTopicId]);
  useEffect(() => {
    setGuideDashboardDrilldown(false);
    setGuidePage(0);
    setGuideQuickPanelMode('read');
  }, [activeNavKey]);
  useEffect(() => {
    setGuideQuickPanelMode('read');
  }, [selectedGuideTopicId]);
  const guideTopicById = useMemo(
    () => new Map(guideTopics.map((topic) => [normalizeSelectionId(topic.id), topic])),
    [guideTopics]
  );
  const handleGuideNoteSelect = (note: LeaderboardGuideNote) => {
    if (normalizeSelectionId(note.targetNavLabel ?? '') === 'chat') {
      onTabChange('aiBenchmarks');
      onSelectNavLabel('CHAT');
      setGuideDashboardDrilldown(false);
      return;
    }
    if (isHashRoutePath(note.targetRoutePath)) {
      if (note.targetNavLabel) onSelectNavLabel(note.targetNavLabel);
      onNavigate?.(note.targetRoutePath);
      return;
    }
    if (note.targetTopicId) {
      const targetTopic = guideTopicById.get(normalizeSelectionId(note.targetTopicId));
      if (targetTopic) {
        onSelectNavLabel(note.targetNavLabel ?? targetTopic.navLabel);
        setSelectedGuideTopicId(targetTopic.id);
        setGuidePage(typeof note.targetPage === 'number' ? note.targetPage : 0);
        setGuideDashboardDrilldown(true);
        setGuideQuickPanelMode('read');
        return;
      }
    }
    if (note.targetNavLabel) {
      onSelectNavLabel(note.targetNavLabel);
    }
    if (typeof note.targetPage === 'number') {
      setGuidePage(clampValue(note.targetPage, 0, Math.max(0, (selectedGuideTopic?.pages.length ?? 1) - 1)));
      setGuideDashboardDrilldown(true);
    }
  };
  const focusedSectionTitle = chatMode
    ? 'PARENT CHAT'
    : activeNavKey.includes('overview')
      ? 'PARENT OVERVIEW'
      : activeNavKey.includes('today')
        ? 'TODAY'
        : activeNavKey.includes('global') || activeNavKey.includes('overall')
          ? rankingTitle
          : guideDashboardMode
            ? 'START HERE'
            : guideMode
              ? 'GUIDE TOPICS'
              : activeNavLabel || rankingTitle;
  const tableVariant = tableVariantForContext(activeNavLabel, activeTab);
  const baseTableTitle = tableTitleForVariant(tableVariant, activeNavLabel, selectedGameName);
  const isOverviewContext = activeNavKey.includes('overview') || activeNavKey.includes('today');
  const sortedRows = useMemo(() => [...rows].sort((a, b) => a.rank - b.rank), [rows]);
  const gameScopedRows = useMemo(
    () => rowsForGameScope(sortedRows, selectedGameName, selectedGameId),
    [selectedGameId, selectedGameName, sortedRows]
  );
  const gameCategories = useMemo(() => buildGameCategorySummaries(quickGames), [quickGames]);
  const selectedQuickGame = useMemo(
    () =>
      quickGames.find((game) => normalizeSelectionId(game.id) === normalizeSelectionId(selectedGameId)) ??
      quickGames[0],
    [quickGames, selectedGameId]
  );
  const [selectedCategoryIdOverride, setSelectedCategoryIdOverride] = useState<string | null>(null);
  const [selectedSubcategoryIdOverride, setSelectedSubcategoryIdOverride] = useState<string | null>(null);
  const [expandedCategoryId, setExpandedCategoryId] = useState<string | null>(null);
  const [gameBrowserSearch, setGameBrowserSearch] = useState('');
  const [gameBrowserView, setGameBrowserView] = useState<LeaderboardGameBrowserView>('grid');
  const [gameBrowserSort, setGameBrowserSort] = useState<LeaderboardGameBrowserSort>('rank');
  const [hoveredTopGameKey, setHoveredTopGameKey] = useState<string | null>(null);
  const selectedGameCategoryId =
    selectedCategoryIdOverride ??
    (selectedQuickGame ? assetKey(gameCategoryLabel(selectedQuickGame)) : (gameCategories[0]?.id ?? ''));
  const selectedCategory =
    gameCategories.find((category) => category.id === selectedGameCategoryId) ?? gameCategories[0] ?? null;
  const selectedCategoryLabel =
    selectedCategory?.label ?? (selectedQuickGame ? gameCategoryLabel(selectedQuickGame) : 'Card Games');
  const selectedSubcategoryId = selectedCategory?.subcategories.some(
    (subcategory) => subcategory.id === selectedSubcategoryIdOverride
  )
    ? selectedSubcategoryIdOverride
    : null;
  const perCategoryMode = activeNavKey.includes('category');
  const aiBrowserMode = tableVariant === 'ai' && (activeNavKey.includes('game') || activeNavKey.includes('category'));
  const categoryBrowserMode = perCategoryMode || activeNavKey.includes('ai-by-category');
  const gameBrowserMode = !guideMode && (tableVariant === 'games' || aiBrowserMode || activeNavGroupId === 'manage');
  const expandedGameCategory =
    gameBrowserMode && selectedCategory && expandedCategoryId === selectedCategory.id ? selectedCategory : null;
  const selectedTableScopeName = categoryBrowserMode ? selectedCategoryLabel : selectedGameName;
  const tableTitle = chatMode
    ? 'PARENT ASSISTANT CHAT'
    : guideMode && selectedGuideTopic
      ? `${selectedGuideTopic.title.toUpperCase()} GUIDE`
      : activeNavKey.includes('ai-by-game')
        ? `${selectedGameName.toUpperCase()} AI BENCHMARKS`
        : activeNavKey.includes('ai-by-category')
          ? `${selectedCategoryLabel.toUpperCase()} AI BENCHMARKS`
          : perCategoryMode
            ? `${selectedCategoryLabel.toUpperCase()} CATEGORY LADDER`
            : baseTableTitle;
  const filteredCategoryGames = quickGames.filter(
    (game) => assetKey(gameCategoryLabel(game)) === selectedGameCategoryId
  );
  const filteredSubcategoryGames = selectedSubcategoryId
    ? filteredCategoryGames.filter((game) => assetKey(gameSubcategoryLabel(game)) === selectedSubcategoryId)
    : filteredCategoryGames;
  const searchedCategoryGames = gameBrowserSearch.trim()
    ? filteredSubcategoryGames.filter((game) =>
        `${game.name} ${game.detail} ${game.category ?? ''} ${game.subcategory ?? ''}`
          .toLowerCase()
          .includes(gameBrowserSearch.trim().toLowerCase())
      )
    : filteredSubcategoryGames;
  const sortedCategoryGames = [...searchedCategoryGames].sort((a, b) =>
    gameBrowserSort === 'az'
      ? a.name.localeCompare(b.name)
      : (a.gameType ?? Number.MAX_SAFE_INTEGER) - (b.gameType ?? Number.MAX_SAFE_INTEGER) ||
        a.name.localeCompare(b.name)
  );
  const gameBrowserPoolBase = gameBrowserMode ? sortedCategoryGames : quickGames;
  const gameBrowserPool =
    activeNavGroupId === 'manage'
      ? [
          ...gameBrowserPoolBase.filter(
            (game) => normalizeSelectionId(game.id) === normalizeSelectionId(selectedGameId)
          ),
          ...gameBrowserPoolBase.filter(
            (game) => normalizeSelectionId(game.id) !== normalizeSelectionId(selectedGameId)
          ),
        ]
      : gameBrowserPoolBase;
  const topGamesById = new Map(topGames.map((game) => [normalizeSelectionId(game.id), game]));
  const topItems: LeaderboardTopCardItem[] = guideMode
    ? guideTopicPool.map(guideTopCard)
    : gameBrowserMode
      ? gameBrowserPool.map((game, index) => gameTopCard(game, topGamesById.get(normalizeSelectionId(game.id)), index))
      : sortedRows.slice(0, isOverviewContext ? 3 : 10).map(leaderTopCard);
  const selectedTopKey =
    guideMode && selectedGuideTopic
      ? `guide:${normalizeSelectionId(selectedGuideTopic.id)}`
      : gameBrowserMode
        ? `game:${normalizeSelectionId(selectedGameId)}`
        : `leader:${selectedPlayerId}`;
  const focusContextKey = `${activeNavLabel}:${activeTab}`;
  const [focusState, setFocusState] = useState<{ contextKey: string; section: LeaderboardFocusSection }>(() => ({
    contextKey: focusContextKey,
    section: 'podium',
  }));
  const focusedSection = focusState.contextKey === focusContextKey ? focusState.section : 'podium';
  const setFocusedSection = (section: LeaderboardFocusSection) =>
    setFocusState({ contextKey: focusContextKey, section });
  const tableFocused = focusedSection === 'table';
  const sectionGap = Math.max(8, Math.min(cfg.layout.gap, 14));
  const expandedTopPanelH = Math.max(276, Math.min(mainH - 210, clampValue(mainH * 0.46, 276, 334)));
  const hoverTopPanelH = Math.max(242, Math.min(mainH - 210, clampValue(mainH * 0.4, 242, 292)));
  const compactTopPanelH = Math.max(178, Math.min(mainH - 250, clampValue(mainH * 0.29, 178, 214)));
  const topPanelH = chatMode
    ? 0
    : tableFocused
      ? 0
      : guideDashboardMode
        ? mainH
        : guideMode
          ? clampValue(mainH * 0.24, 164, 220)
          : gameBrowserMode
            ? hoveredTopGameKey
              ? Math.max(hoverTopPanelH, expandedGameCategory ? expandedTopPanelH : 0)
              : expandedGameCategory
                ? expandedTopPanelH
                : compactTopPanelH
            : clampValue(mainH * 0.39, 260, 294);
  const bottomPanelY = chatMode || tableFocused ? mainY : mainY + topPanelH + sectionGap;
  const bottomPanelH = chatMode || tableFocused ? mainH : mainH - topPanelH - sectionGap;
  const showGameLeaderStrip = gameBrowserMode && !tableFocused && !hoveredTopGameKey;
  const gameLeaderStripH = showGameLeaderStrip ? clampValue(bottomPanelH * 0.38, 128, 154) : 0;
  const selectorHandleGutter = LEADERBOARD_SIDE_HANDLE_W;
  const selectorX = mainX + selectorHandleGutter;
  const selectorW = Math.max(320, mainW - selectorHandleGutter * 2);
  const rowHandleReserve = (LEADERBOARD_SIDE_HANDLE_W + 8) * 2;
  const selectorInnerW = Math.max(1, selectorW - 36);
  const categoryTrackW = Math.max(1, selectorInnerW - rowHandleReserve);
  const categoryVisibleCount = Math.max(
    1,
    Math.min(gameCategories.length || 1, Math.floor((categoryTrackW + 8) / 178), 6)
  );
  const [categoryPage, setCategoryPage] = useState(0);
  const categoryPageCount = Math.max(1, Math.ceil(gameCategories.length / categoryVisibleCount));
  const safeCategoryPage = wrapIndex(categoryPage, categoryPageCount);
  const visibleGameCategories = gameCategories.slice(
    safeCategoryPage * categoryVisibleCount,
    safeCategoryPage * categoryVisibleCount + categoryVisibleCount
  );
  const tableFrame = leaderboardFrameRects(mainX, bottomPanelY, mainW, bottomPanelH);
  const tableX = tableFrame.body.x + 10;
  const tableTopInset = showGameLeaderStrip ? 10 : 0;
  const tableY = showGameLeaderStrip ? tableFrame.body.y + tableTopInset + gameLeaderStripH + 8 : bottomPanelY + 47;
  const tableW = tableFrame.body.w - 32;
  const rowStep = cfg.chrome.rowHeight + cfg.chrome.rowGap;
  const visibleRowCapacity = Math.max(1, Math.floor((tableFrame.body.y + tableFrame.body.h - tableY - 42) / rowStep));
  const effectiveRowsPerPage = Math.max(1, Math.min(rowsPerPage, visibleRowCapacity));
  const categoryScopedRows = rowsForCategoryScope(sortedRows, selectedCategoryLabel);
  const aiGameScopedRows = rowsForGameScope(sortedRows, selectedGameName, selectedGameId);
  const tableRows =
    tableVariant === 'games'
      ? categoryBrowserMode
        ? categoryScopedRows
        : gameScopedRows
      : tableVariant === 'ai' && activeNavKey.includes('ai-by-category')
        ? categoryScopedRows
        : tableVariant === 'ai' && activeNavKey.includes('ai-by-game')
          ? aiGameScopedRows
          : isOverviewContext
            ? [
                ...sortedRows.filter((row) => row.id === selectedPlayerId && row.rank <= 3),
                ...sortedRows.filter((row) => row.rank > 3),
              ]
            : sortedRows;
  const maxPage = Math.max(1, Math.ceil(tableRows.length / effectiveRowsPerPage));
  const safePage = clampValue(page, 1, maxPage);
  const pagedRows = tableRows.slice((safePage - 1) * effectiveRowsPerPage, safePage * effectiveRowsPerPage);
  const gameLeaderRows = showGameLeaderStrip ? tableRows.slice(0, 3) : [];
  const handleTableWheel = (event: WheelEvent<SVGGElement>) => {
    const delta = Math.abs(event.deltaX) > Math.abs(event.deltaY) ? event.deltaX : event.deltaY;
    if (Math.abs(delta) < 8) return;
    event.preventDefault();
    event.stopPropagation();
    if (maxPage > 1) {
      onPageChange(clampValue(safePage + (delta > 0 ? 1 : -1), 1, maxPage));
    }
  };
  const handleTablePlayerSelect = (playerId: string) => {
    setFocusedSection('table');
    onPlayerSelect(playerId);
  };
  const [podiumPage, setPodiumPage] = useState(0);
  const topCardGap = 10;
  const topCardMinW = guideMode ? 250 : gameBrowserMode ? LEADERBOARD_GAME_CARD_MIN_W : LEADERBOARD_TOP_CARD_MIN_W;
  const topCarouselAvailableW = gameBrowserMode
    ? Math.max(1, selectorInnerW - rowHandleReserve)
    : Math.max(1, selectorW - 56);
  const topCardVisibleCount = Math.max(
    1,
    Math.min(
      topItems.length || 1,
      LEADERBOARD_TOP_CAROUSEL_MAX_VISIBLE,
      Math.floor((topCarouselAvailableW + topCardGap) / (topCardMinW + topCardGap))
    )
  );
  const podiumPageCount = Math.max(1, Math.ceil(topItems.length / topCardVisibleCount));
  const safePodiumPage = wrapIndex(podiumPage, podiumPageCount);
  const shiftPodiumPage = (delta: number) => setPodiumPage((value) => wrapIndex(value + delta, podiumPageCount));
  const framePage = safePodiumPage;
  const framePageCount = podiumPageCount;
  const shiftFramePage = (delta: number) => {
    shiftPodiumPage(delta);
  };
  const routeGameCategoryToBottom = (category: GameCategorySummary) => {
    setFocusedSection('podium');
    setSelectedCategoryIdOverride(category.id);
    setSelectedSubcategoryIdOverride(null);
    setExpandedCategoryId((value) => (value === category.id ? null : category.id));
    setPodiumPage(0);
    const nextRows =
      category.count > 0
        ? rowsForGameScope(sortedRows, category.sampleGame.name, category.sampleGame.id)
        : rowsForCategoryScope(sortedRows, category.label);
    if (nextRows[0]) onPlayerSelect(nextRows[0].id);
    if (category.count > 0) onGameSelect(category.sampleGame.id);
    onPageChange(1);
  };
  const routeGameSubcategoryToBottom = (subcategory: GameSubcategorySummary) => {
    setFocusedSection('podium');
    setSelectedSubcategoryIdOverride(subcategory.id);
    setExpandedCategoryId(null);
    setPodiumPage(0);
    const nextRows = rowsForGameScope(sortedRows, subcategory.sampleGame.name, subcategory.sampleGame.id);
    if (nextRows[0]) onPlayerSelect(nextRows[0].id);
    onGameSelect(subcategory.sampleGame.id);
    onPageChange(1);
  };
  const routeTopItemToBottom = (item: LeaderboardTopCardItem) => {
    setFocusedSection('podium');
    if (item.kind === 'guide') {
      setSelectedGuideTopicId(item.topic.id);
      setGuidePage(0);
      onPageChange(1);
      return;
    }
    if (item.kind === 'game') {
      const nextCategoryId = assetKey(gameCategoryLabel(item.game));
      setSelectedCategoryIdOverride(nextCategoryId);
      setSelectedSubcategoryIdOverride(assetKey(gameSubcategoryLabel(item.game)));
      setExpandedCategoryId(null);
      const nextCategoryIndex = gameCategories.findIndex((category) => category.id === nextCategoryId);
      if (nextCategoryIndex >= 0) {
        setCategoryPage(Math.floor(nextCategoryIndex / categoryVisibleCount));
      }
      const nextRows = rowsForGameScope(sortedRows, item.game.name, item.game.id);
      if (nextRows[0]) onPlayerSelect(nextRows[0].id);
      onGameSelect(item.game.id);
      onPageChange(1);
      return;
    }
    onPlayerSelect(item.row.id);
    const targetIndex = tableRows.findIndex((row) => row.id === item.row.id);
    if (targetIndex >= 0) {
      onPageChange(Math.floor(targetIndex / effectiveRowsPerPage) + 1);
    }
  };
  const handlePodiumWheel = (event: WheelEvent<SVGGElement>) => {
    const delta = Math.abs(event.deltaX) > Math.abs(event.deltaY) ? event.deltaX : event.deltaY;
    if (framePageCount <= 1 || Math.abs(delta) < 8) return;
    event.preventDefault();
    event.stopPropagation();
    shiftFramePage(delta > 0 ? 1 : -1);
  };
  const headerButtonY = mainY + 13;
  const tableHeaderButtonY = bottomPanelY + 13;
  const tableHeaderAction = tableFocused ? (
    <LeaderboardHeaderAction
      x={mainX + mainW - 118}
      y={tableHeaderButtonY + 2}
      w={98}
      h={25}
      tone="gold"
      active
      label="PODIUM"
      onClick={() => setFocusedSection('podium')}
      ariaLabel="Show podium section"
      cfg={cfg}
    />
  ) : guideOverviewMode && guideDashboardDrilldown ? (
    <LeaderboardHeaderAction
      x={mainX + mainW - 118}
      y={tableHeaderButtonY + 2}
      w={98}
      h={25}
      tone="cyan"
      active
      label="MAP"
      onClick={() => setGuideDashboardDrilldown(false)}
      ariaLabel="Show guide setup map"
      cfg={cfg}
    />
  ) : null;
  return (
    <g>
      {!tableFocused && !chatMode ? (
        <LeaderboardSectionFrame
          x={selectorX}
          y={mainY}
          w={selectorW}
          h={topPanelH}
          title={focusedSectionTitle}
          subtitle={
            guideDashboardMode
              ? 'Set up parent app, child devices, controls, privacy, alerts, and storage'
              : guideMode && selectedGuideTopic
                ? `${selectedGuideTopic.subtitle} / ${selectedGuideTopic.detail}`
                : `${detail.eyebrow} / ${detail.primary}`
          }
          count="1"
          tone="cyan"
          headerH={gameBrowserMode ? 48 : 40}
          footerH={guideDashboardMode ? 18 : gameBrowserMode ? 22 : 30}
          innerStrokeOpacity={gameBrowserMode ? 0.24 : undefined}
          bodyStrokeOpacity={gameBrowserMode ? 0 : undefined}
          bodyFill={gameBrowserMode ? 'transparent' : undefined}
          footerLineOpacity={gameBrowserMode ? 0 : undefined}
          headerRight={null}
          showSideHandles={!gameBrowserMode && !guideDashboardMode}
          sideDisabled={guideDashboardMode || framePageCount <= 1}
          onPrevious={() => shiftFramePage(-1)}
          onNext={() => shiftFramePage(1)}
          selected={focusedSection === 'podium'}
          onSelect={() => setFocusedSection('podium')}
          ariaLabel="Focus parent control selector"
          footer={(footerRect) => (
            <>
              {guideDashboardMode ? null : (
                <LeaderboardFrameDots
                  x={footerRect.x + footerRect.w / 2}
                  y={footerRect.y + (gameBrowserMode ? 14 : 18)}
                  page={framePage}
                  pageCount={framePageCount}
                  onPageChange={setPodiumPage}
                  cfg={cfg}
                />
              )}
              <text
                x={footerRect.x + footerRect.w - 22}
                y={footerRect.y + (gameBrowserMode ? 18 : 23)}
                textAnchor="end"
                fontSize={gameBrowserMode ? 9.5 : 10}
                fontWeight={900}
                fill={cfg.colors.mutedText}
              >
                {guideDashboardMode
                  ? `GUIDES ${guideTopicPool.length}`
                  : `${guideMode ? 'TOPICS' : gameBrowserMode ? 'AREAS' : isOverviewContext ? 'READY' : 'ITEMS'} ${
                      framePage + 1
                    }/${framePageCount}`}
              </text>
            </>
          )}
          cfg={cfg}
        >
          {(body) => {
            const contentX = body.x + 2;
            const contentY = body.y + (gameBrowserMode ? 2 : -2);
            const contentW = body.w - 4;
            const contentH = body.h + (gameBrowserMode ? -2 : 4);
            const expandedCategory = expandedGameCategory;
            const categoryH = gameBrowserMode
              ? clampValue(
                  contentH * (expandedCategory ? 0.17 : 0.28),
                  expandedCategory ? 34 : 28,
                  expandedCategory ? 42 : 34
                )
              : 0;
            const subcategoryH = expandedCategory ? clampValue(contentH * 0.32, 82, 110) : 0;
            const categoryGap = gameBrowserMode ? 4 : 0;
            const subcategoryGap = expandedCategory ? 4 : 0;
            const subcategoryY = contentY + categoryH + categoryGap;
            const carouselY = subcategoryY + subcategoryH + subcategoryGap;
            const carouselH = expandedCategory
              ? Math.max(64, contentH - categoryH - categoryGap - subcategoryH - subcategoryGap)
              : Math.max(48, contentH - categoryH - categoryGap);
            const rowHandleW = LEADERBOARD_SIDE_HANDLE_W;
            const rowHandleLeftX = selectorX - rowHandleW + LEADERBOARD_SIDE_HANDLE_OVERLAP;
            const rowHandleRightX = selectorX + selectorW - LEADERBOARD_SIDE_HANDLE_OVERLAP;
            const carouselTrackX = contentX;
            const carouselTrackW = contentW;
            const carouselHandleH = gameBrowserMode ? Math.max(44, Math.min(92, carouselH - 8)) : 0;
            const carouselHandleY = carouselY + Math.max(0, (carouselH - carouselHandleH) / 2);
            return (
              <g onWheel={handlePodiumWheel}>
                <rect x={contentX - 4} y={contentY - 4} width={contentW + 8} height={contentH + 8} fill="transparent" />
                {guideDashboardMode ? (
                  <GuideOverviewDashboard
                    x={contentX + 10}
                    y={contentY + 10}
                    w={contentW - 20}
                    h={contentH - 18}
                    topics={guideTopicPool}
                    selectedTopicId={selectedGuideTopicId}
                    onSelect={(topic) => {
                      setSelectedGuideTopicId(topic.id);
                      setGuidePage(0);
                      setGuideDashboardDrilldown(true);
                    }}
                    cfg={cfg}
                  />
                ) : gameBrowserMode ? (
                  <GameCategoryGrid
                    x={contentX}
                    y={contentY}
                    w={contentW}
                    h={categoryH}
                    categories={visibleGameCategories}
                    selectedCategoryId={selectedGameCategoryId}
                    page={safeCategoryPage}
                    pageCount={categoryPageCount}
                    handleLeftX={rowHandleLeftX}
                    handleRightX={rowHandleRightX}
                    onPageChange={setCategoryPage}
                    onSelect={routeGameCategoryToBottom}
                    cfg={cfg}
                  />
                ) : null}
                {!guideDashboardMode && expandedCategory ? (
                  <GameSubcategoryGrid
                    x={carouselTrackX}
                    y={subcategoryY}
                    w={carouselTrackW}
                    h={subcategoryH}
                    category={expandedCategory}
                    selectedSubcategoryId={selectedSubcategoryId}
                    onSelect={routeGameSubcategoryToBottom}
                    cfg={cfg}
                  />
                ) : null}
                {!guideDashboardMode && gameBrowserMode ? (
                  <>
                    <LeaderboardFrameSideHandle
                      x={rowHandleLeftX}
                      y={carouselHandleY}
                      side="left"
                      height={carouselHandleH}
                      width={rowHandleW}
                      disabled={podiumPageCount <= 1}
                      onClick={() => shiftPodiumPage(-1)}
                      cfg={cfg}
                    />
                    <LeaderboardFrameSideHandle
                      x={rowHandleRightX}
                      y={carouselHandleY}
                      side="right"
                      height={carouselHandleH}
                      width={rowHandleW}
                      disabled={podiumPageCount <= 1}
                      onClick={() => shiftPodiumPage(1)}
                      cfg={cfg}
                    />
                  </>
                ) : null}
                {!guideDashboardMode && gameBrowserMode && gameBrowserView === 'list' ? (
                  <LeaderboardGameList
                    x={carouselTrackX}
                    y={carouselY}
                    w={carouselTrackW}
                    h={carouselH}
                    items={topItems}
                    page={safePodiumPage}
                    visibleCount={topCardVisibleCount}
                    selectedKey={selectedTopKey}
                    onSelect={routeTopItemToBottom}
                    cfg={cfg}
                  />
                ) : !guideDashboardMode ? (
                  <LeaderboardTopCarousel
                    x={carouselTrackX}
                    y={gameBrowserMode ? carouselY : contentY}
                    w={carouselTrackW}
                    h={gameBrowserMode ? carouselH : contentH}
                    items={topItems}
                    page={safePodiumPage}
                    selectedKey={selectedTopKey}
                    onSelect={routeTopItemToBottom}
                    onHoverChange={(item) => setHoveredTopGameKey(item?.kind === 'game' ? item.key : null)}
                    minCardW={topCardMinW}
                    cfg={cfg}
                  />
                ) : null}
              </g>
            );
          }}
        </LeaderboardSectionFrame>
      ) : null}
      {!guideDashboardMode ? (
        <LeaderboardSectionFrame
          x={mainX}
          y={bottomPanelY}
          w={mainW}
          h={bottomPanelH}
          title={tableTitle}
          count="2"
          tone="cyan"
          headerRight={tableHeaderAction}
          bodyStrokeOpacity={0}
          bodyFill="transparent"
          selected={tableFocused}
          onSelect={() => setFocusedSection('table')}
          ariaLabel={tableFocused ? 'Expanded parent detail panel' : 'Expand parent detail panel'}
          cfg={cfg}
        >
          {(body) =>
            chatMode ? (
              <ParentChatPanel
                x={body.x + 18}
                y={body.y + 18}
                w={body.w - 36}
                h={body.h - 36}
                onNavigate={onNavigate}
                cfg={cfg}
              />
            ) : (
              <ParentPortalDetailPanel
                x={body.x + 18}
                y={body.y + 18}
                w={body.w - 36}
                h={body.h - 36}
                activeNavLabel={activeNavLabel}
                detail={detail}
                rows={tableRows}
                selectedGameName={selectedGameName}
                guideTopic={selectedGuideTopic}
                guidePage={guidePage}
                onGuidePageChange={setGuidePage}
                quickPanelMode={guideQuickPanelMode}
                onQuickPanelModeChange={setGuideQuickPanelMode}
                onGuideNoteSelect={handleGuideNoteSelect}
                cfg={cfg}
              />
            )
          }
        </LeaderboardSectionFrame>
      ) : null}
      {detailMode ? (
        <DetailOverlay
          x={mainX + 18}
          y={mainY + 50}
          w={mainW - 36}
          h={mainH - 100}
          mode={detailMode}
          activeTab={activeTab}
          detail={detail}
          season={season}
          topGames={topGames}
          uiCopy={uiCopy}
          selectedPlayer={selectedPlayer}
          selectedGameName={selectedGameName}
          onTabChange={onTabChange}
          onClose={onDetailClose}
          cfg={cfg}
        />
      ) : null}
    </g>
  );
}

function DetailOverlay({
  x,
  y,
  w,
  h,
  mode,
  activeTab,
  detail,
  season,
  topGames,
  uiCopy,
  selectedPlayer,
  selectedGameName,
  onTabChange,
  onClose,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  mode: DetailMode;
  activeTab: LeaderboardTabId;
  detail: TabDetail;
  season: LeaderboardPageContentData['season'];
  topGames: TopGame[];
  uiCopy: LeaderboardPageContentData['uiCopy'];
  selectedPlayer: DisplayRow;
  selectedGameName: string;
  onTabChange: (tab: LeaderboardTabId) => void;
  onClose: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const tone: Tone = mode === 'season' ? 'gold' : mode === 'game' ? 'purple' : selectedPlayer.tone;
  const color = toneColor(tone, cfg);
  const selectedGame = topGames.find((game) => game.name === selectedGameName || game.id === selectedGameName);
  const title =
    mode === 'season'
      ? season.detailTitle
      : mode === 'game'
        ? `${selectedGameName.toUpperCase()} DRILLDOWN`
        : `${selectedPlayer.player.toUpperCase()} PROFILE`;
  const subtitle =
    mode === 'season'
      ? season.detailSubtitle
      : mode === 'game'
        ? `Per-game ranking from ${detail.eyebrow}`
        : `${detail.eyebrow} / ${selectedGameName}`;
  const stats =
    mode === 'season'
      ? season.stats.map((stat) => [stat.label, stat.value] as [string, string])
      : mode === 'game'
        ? [
            ['MATCHES', selectedGame?.matches ?? 'N/A'],
            ['GROWTH', selectedGame?.growth ?? 'N/A'],
            ['LEADERS', '100'],
          ]
        : [
            ['RATING', selectedPlayer.rating],
            ['WIN RATE', selectedPlayer.winRate],
            ['TREND', selectedPlayer.trend],
          ];
  const primaryTab: LeaderboardTabId = mode === 'season' ? 'tournaments' : mode === 'game' ? 'perGame' : activeTab;
  const primaryLabel = mode === 'season' ? 'OPEN EVENTS' : mode === 'game' ? 'OPEN GAME' : 'KEEP SELECTED';
  const artworkImageUrl =
    mode === 'season'
      ? shopPageSeasonPassImageUrl
      : mode === 'game'
        ? leaderboardGameImageUrl(selectedGame?.id ?? selectedGameName)
        : playerAvatarImageUrl(selectedPlayer.player);
  return (
    <g role="dialog" aria-label={title}>
      <path
        d={cutRectPath(x - 8, y - 8, w + 16, h + 16, 18)}
        fill="rgba(1, 5, 12, 0.72)"
        stroke={color}
        strokeWidth={1.2}
        opacity={0.98}
      />
      <path
        d={cutRectPath(x, y, w, h, 16)}
        fill="rgba(5, 17, 30, 0.97)"
        stroke={color}
        strokeWidth={1.4}
        filter="url(#leaderboardGlow)"
      />
      <ArtworkSlot
        x={x + 22}
        y={y + 24}
        w={92}
        h={82}
        label={mode === 'season' ? 'SEASON' : mode === 'game' ? 'GAME ART' : 'PLAYER'}
        imageUrl={artworkImageUrl}
        tone={tone}
        shape={mode === 'player' ? 'circle' : 'hex'}
        cfg={cfg}
      />
      <text x={x + 136} y={y + 39} fontSize={22} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      <text x={x + 136} y={y + 64} fontSize={12} fontWeight={800} fill="#a9c3da">
        {subtitle}
      </text>
      <text x={x + 136} y={y + 91} fontSize={11} fontWeight={900} fill={color}>
        {detail.primary}
      </text>
      <text x={x + 136} y={y + 112} fontSize={10.5} fontWeight={760} fill="#d8eaff">
        {detail.summary}
      </text>
      <LeaderboardHeaderAction
        x={x + w - 208}
        y={y + 26}
        w={126}
        h={30}
        tone={tone}
        active
        label={primaryLabel}
        onClick={() => {
          onTabChange(primaryTab);
          onClose();
        }}
        ariaLabel={primaryLabel}
        cfg={cfg}
      />
      <LeaderboardHeaderAction
        x={x + w - 68}
        y={y + 26}
        w={44}
        h={30}
        tone="muted"
        label="X"
        onClick={onClose}
        ariaLabel="Close leaderboard detail"
        cfg={cfg}
      />
      {stats.map(([label, value], index) => {
        const cardW = (w - 62) / 3;
        const cardX = x + 22 + index * (cardW + 9);
        return (
          <SurfacePanel key={label} x={cardX} y={y + 136} w={cardW} h={72} tone={index === 1 ? tone : 'cyan'} cfg={cfg}>
            <text x={cardX + 18} y={y + 162} fontSize={10} fontWeight={900} fill={cfg.colors.mutedText}>
              {label}
            </text>
            <text x={cardX + 18} y={y + 190} fontSize={22} fontWeight={950} fill={cfg.colors.bodyText}>
              {value}
            </text>
          </SurfacePanel>
        );
      })}
      <path
        d={cutRectPath(x + 22, y + 230, w - 44, h - 256, 12)}
        fill="rgba(7, 22, 37, 0.72)"
        stroke={color}
        strokeWidth={0.8}
        strokeOpacity={0.72}
      />
      <text x={x + 44} y={y + 266} fontSize={12} fontWeight={950} fill={cfg.colors.bodyText}>
        {uiCopy.detailSnapshotTitle}
      </text>
      {uiCopy.detailSnapshotLines.slice(0, 2).map((line, index) => (
        <text key={line} x={x + 44} y={y + 291 + index * 27} fontSize={11} fontWeight={760} fill="#b9d2e7">
          {line}
        </text>
      ))}
    </g>
  );
}

function TopGameRow({
  game,
  x,
  y,
  w,
  selected,
  onSelect,
  cfg,
}: {
  game: TopGame;
  x: number;
  y: number;
  w: number;
  selected: boolean;
  onSelect: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const rawClipId = useId();
  const clipId = `leaderboard-top-game-row-${rawClipId.replace(/:/g, '')}`;
  const lit = selected || hovered;
  const color = toneColor(game.tone, cfg);
  const rowH = 24;
  const matchesLabel = compactGameStatLabel(game.matches);
  const growthLabel = compactGameGrowthLabel(game.growth);
  const gameNameW = Math.max(48, w - 164);
  const gameNameSize = fitSingleLineTextSize(game.name, gameNameW, 8.5, 12, 0.58);
  const matchesSize = fitSingleLineTextSize(matchesLabel, 42, 8.4, 10.8, 0.56);
  const growthSize = fitSingleLineTextSize(growthLabel, 56, 8.2, 10.2, 0.56);
  return (
    <g
      className="leaderboard-page-svg-clickable"
      onClick={(event) => {
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      role="button"
      tabIndex={0}
      aria-label={`Select ${game.name}`}
    >
      <defs>
        <clipPath id={clipId}>
          <path d={cutRectPath(x, y, w, rowH, 6)} />
        </clipPath>
      </defs>
      <path
        d={cutRectPath(x, y, w, rowH, 6)}
        fill={lit ? (selected ? 'rgba(123, 92, 255, 0.22)' : `${color}18`) : 'rgba(6, 18, 31, 0.62)'}
        stroke={lit ? color : '#123f62'}
        strokeWidth={lit ? 1.15 : 0.65}
      />
      {hovered && !selected ? (
        <path
          d={cutRectPath(x - 2, y - 2, w + 4, rowH + 4, 8)}
          fill="none"
          stroke={color}
          strokeWidth={1.5}
          opacity={0.26}
          filter="url(#leaderboardGlow)"
        />
      ) : null}
      <g clipPath={`url(#${clipId})`}>
        <IconBadge x={x + 6} y={y + 1} icon={Trophy} tone={game.tone} size={22} rank={game.rank} cfg={cfg} />
        <GameArtBadge x={x + 34} y={y + 2} size={20} gameId={game.id} tone={game.tone} cfg={cfg} />
        <text x={x + 59} y={y + 16} fontSize={gameNameSize} fontWeight={720} fill={cfg.colors.bodyText}>
          {truncateTextForWidth(game.name, gameNameW, gameNameSize, 0.58)}
        </text>
        <text
          x={x + w - 78}
          y={y + 16}
          textAnchor="end"
          fontSize={matchesSize}
          fontWeight={700}
          fill={cfg.colors.bodyText}
        >
          {matchesLabel}
        </text>
        <text x={x + w - 25} y={y + 16} textAnchor="end" fontSize={growthSize} fontWeight={760} fill="#42ff83">
          {growthLabel}
        </text>
      </g>
      {lit ? <ChevronRight x={x + w - 18} y={y + 5} width={13} height={13} color={color} /> : null}
    </g>
  );
}

function SelectionSummary({
  x,
  y,
  w,
  label,
  player,
  selectedGame,
  detail,
  onOpen,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  label: string;
  player: DisplayRow;
  selectedGame: string;
  detail: TabDetail;
  onOpen: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(player.tone, cfg);
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={`Open ${player.player} detail`}
      onClick={(event) => {
        event.stopPropagation();
        onOpen();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onOpen();
      }}
    >
      {hovered ? (
        <rect
          x={x - 4}
          y={y - 4}
          width={w + 8}
          height={66}
          rx={7}
          fill="none"
          stroke={color}
          strokeWidth={1.5}
          opacity={0.28}
          filter="url(#leaderboardGlow)"
        />
      ) : null}
      <rect
        x={x}
        y={y}
        width={w}
        height={58}
        rx={5}
        fill={hovered ? `${color}18` : 'rgba(7, 22, 37, 0.9)'}
        stroke={color}
        strokeWidth={hovered ? 1.2 : 0.8}
        strokeOpacity={0.72}
      />
      <PlayerAvatarSlot cx={x + 21} cy={y + 19} r={13} player={player.player} tone={player.tone} cfg={cfg} />
      <text x={x + 44} y={y + 15} fontSize={9.5} fontWeight={900} fill="#8ddff2">
        {label}
      </text>
      <text x={x + 44} y={y + 31} fontSize={12} fontWeight={950} fill={cfg.colors.bodyText}>
        {player.player}
      </text>
      <text x={x + w - 14} y={y + 31} textAnchor="end" fontSize={12} fontWeight={950} fill={color}>
        {player.rating}
      </text>
      <text x={x + 44} y={y + 48} fontSize={9.5} fontWeight={800} fill={cfg.colors.mutedText}>
        {detail.eyebrow} / {selectedGame}
      </text>
      <ChevronRight x={x + w - 28} y={y + 40} width={13} height={13} color={toneColor(detail.tone, cfg)} />
    </g>
  );
}

function RightRailCard({
  x,
  y,
  w,
  h,
  tone,
  title,
  subtitle,
  onClick,
  ariaLabel,
  cfg,
  children,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  tone: Tone;
  title: string;
  subtitle: string;
  onClick: () => void;
  ariaLabel: string;
  cfg: LeaderboardPageSvgControls;
  children: (bounds: { x: number; y: number; w: number; h: number; color: string }) => ReactNode;
}) {
  const [hovered, setHovered] = useState(false);
  const color = toneColor(tone, cfg);
  const headerX = x + 14;
  const headerY = y + 12;
  const headerW = w - 28;
  const headerH = 39;
  const bodyX = x + 14;
  const bodyY = y + 59;
  const bodyW = w - 28;
  const bodyH = h - 73;
  const titleSize = fitSingleLineTextSize(title, headerW - 18, 11.5, 15, 0.58);
  const arrowTop = y + h / 2 - 42;
  const arrowBottom = y + h / 2 + 42;
  const arrowMid = y + h / 2;
  const handleKeyDown = (event: KeyboardEvent<SVGGElement>) => {
    if (event.key !== 'Enter' && event.key !== ' ') return;
    event.preventDefault();
    event.stopPropagation();
    onClick();
  };
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={ariaLabel}
      onClick={(event) => {
        event.stopPropagation();
        onClick();
      }}
      onKeyDown={handleKeyDown}
      onMouseEnter={() => setHovered(true)}
      onMouseOver={() => setHovered(true)}
      onMouseMove={() => setHovered(true)}
      onPointerEnter={() => setHovered(true)}
      onPointerMove={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onPointerLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      {hovered ? (
        <>
          <path
            d={`M ${x + 3} ${arrowTop} L ${x - 12} ${arrowMid} L ${x + 3} ${arrowBottom} Z`}
            fill={color}
            opacity={0.82}
            filter="url(#leaderboardGlow)"
          />
        </>
      ) : null}
      <LeaderboardSidePanelFrame x={x} y={y} w={w} h={h} tone={tone} active={hovered} cfg={cfg} />
      <path
        d={cutRectPath(headerX, headerY, headerW, headerH, 8)}
        fill={hovered ? colorAlpha(color, '3f') : colorAlpha(color, '18')}
        stroke={color}
        strokeWidth={hovered ? 1.25 : 0.9}
        strokeOpacity={hovered ? 0.9 : 0.78}
      />
      <text x={headerX + 13} y={headerY + 17} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      <text x={headerX + 13} y={headerY + 32} fontSize={8.8} fontWeight={850} fill={cfg.colors.mutedText}>
        {subtitle}
      </text>
      <path
        d={cutRectPath(bodyX, bodyY, bodyW, bodyH, 8)}
        fill={hovered ? colorAlpha(color, '18') : 'rgba(4, 15, 27, 0.68)'}
        stroke={color}
        strokeWidth={hovered ? 1.15 : 0.7}
        strokeOpacity={hovered ? 0.82 : 0.42}
      />
      {hovered ? (
        <g pointerEvents="none">
          <path
            d={cutRectPath(bodyX + 5, bodyY + 5, bodyW - 10, bodyH - 10, 6)}
            fill="none"
            stroke={color}
            strokeWidth={1.2}
            opacity={0.36}
            filter="url(#leaderboardGlow)"
          />
          <line
            x1={bodyX + 8}
            y1={bodyY + 14}
            x2={bodyX + 8}
            y2={bodyY + bodyH - 14}
            stroke={color}
            strokeWidth={1.15}
            opacity={0.78}
          />
          <line
            x1={bodyX + bodyW - 8}
            y1={bodyY + 14}
            x2={bodyX + bodyW - 8}
            y2={bodyY + bodyH - 14}
            stroke={color}
            strokeWidth={1.15}
            opacity={0.78}
          />
        </g>
      ) : null}
      {children({ x: bodyX, y: bodyY, w: bodyW, h: bodyH, color })}
    </g>
  );
}

function DistributionChart({
  x,
  y,
  totalLabel,
  totalValue,
  cfg,
}: {
  x: number;
  y: number;
  totalLabel: string;
  totalValue: string;
  cfg: LeaderboardPageSvgControls;
}) {
  const segments = [
    { start: -140, end: -74, color: cfg.colors.red },
    { start: -72, end: -26, color: cfg.colors.purple },
    { start: -24, end: 42, color: cfg.colors.cyan },
    { start: 44, end: 126, color: '#4ed77c' },
    { start: 128, end: 218, color: cfg.colors.gold },
  ];
  return (
    <g>
      {segments.map((segment) => (
        <DonutSegment
          key={`${segment.start}-${segment.end}`}
          cx={x}
          cy={y}
          rOuter={62}
          rInner={40}
          start={segment.start}
          end={segment.end}
          color={segment.color}
        />
      ))}
      <circle cx={x} cy={y} r={37} fill="#061626" stroke={cfg.colors.cyan} strokeWidth={0.9} />
      <text x={x} y={y - 8} textAnchor="middle" fontSize={9} fontWeight={900} fill="#a9bed5">
        {totalLabel}
      </text>
      <text x={x} y={y + 18} textAnchor="middle" fontSize={21} fontWeight={950} fill={cfg.colors.bodyText}>
        {totalValue}
      </text>
    </g>
  );
}

function polar(cx: number, cy: number, r: number, angle: number) {
  const rad = ((angle - 90) * Math.PI) / 180;
  return { x: cx + r * Math.cos(rad), y: cy + r * Math.sin(rad) };
}

function DonutSegment({
  cx,
  cy,
  rOuter,
  rInner,
  start,
  end,
  color,
}: {
  cx: number;
  cy: number;
  rOuter: number;
  rInner: number;
  start: number;
  end: number;
  color: string;
}) {
  const outerStart = polar(cx, cy, rOuter, end);
  const outerEnd = polar(cx, cy, rOuter, start);
  const innerStart = polar(cx, cy, rInner, start);
  const innerEnd = polar(cx, cy, rInner, end);
  const largeArc = end - start <= 180 ? 0 : 1;
  const d = [
    `M ${outerStart.x} ${outerStart.y}`,
    `A ${rOuter} ${rOuter} 0 ${largeArc} 0 ${outerEnd.x} ${outerEnd.y}`,
    `L ${innerStart.x} ${innerStart.y}`,
    `A ${rInner} ${rInner} 0 ${largeArc} 1 ${innerEnd.x} ${innerEnd.y}`,
    'Z',
  ].join(' ');
  return <path d={d} fill={color} opacity={0.88} stroke="#07121f" strokeWidth={1} />;
}

function RightRail({
  activeTab,
  tabDetails,
  topGames,
  quickGames,
  distributionLabels,
  uiCopy,
  selectedPlayer,
  selectedGameId,
  totalPlayersValue,
  onTabChange,
  onGameSelect,
  onDetailOpen,
  cfg,
  rightX,
  rightW,
  mainY,
  mainH,
}: {
  activeTab: LeaderboardTabId;
  tabDetails: LeaderboardPageContentData['tabDetails'];
  topGames: TopGame[];
  quickGames: QuickGame[];
  distributionLabels: string[];
  season: LeaderboardPageContentData['season'];
  uiCopy: LeaderboardPageContentData['uiCopy'];
  selectedPlayer: DisplayRow;
  selectedGameId: string;
  totalPlayersValue: string;
  onTabChange: (tab: LeaderboardTabId) => void;
  onGameSelect: (gameId: string) => void;
  onDetailOpen: (mode: DetailMode) => void;
  cfg: LeaderboardPageSvgControls;
  rightX: number;
  rightW: number;
  mainY: number;
  mainH: number;
}) {
  const selectedGame =
    topGames.find((game) => game.id === selectedGameId) ?? quickGames.find((game) => game.id === selectedGameId);
  const detail = tabDetails[activeTab];
  const topH = 206;
  const distributionH = 228;
  const distributionY = mainY + topH + cfg.layout.gap;
  const feedY = distributionY + distributionH + cfg.layout.gap;
  const feedH = Math.max(168, mainY + mainH - feedY);
  const distributionStats = distributionLabels.slice(0, 5).map((label, index) => {
    const match = label.match(/^(.*)\s+([0-9.]+%)$/);
    return {
      label: match?.[1] ?? label,
      value: match?.[2] ?? '',
      tone: (['cyan', 'purple', 'gold', 'red', 'muted'] as Tone[])[index] ?? 'cyan',
    };
  });
  const openGames = () => {
    onTabChange('perGame');
    onDetailOpen('game');
  };
  const openDistribution = () => {
    onTabChange('overall');
    onDetailOpen('season');
  };
  return (
    <g>
      <RightRailCard
        x={rightX}
        y={mainY}
        w={rightW}
        h={topH}
        tone="cyan"
        title={uiCopy.topGamesTitle}
        subtitle={`${detail.eyebrow} / ${selectedGame?.name ?? selectedGameId}`}
        onClick={openGames}
        ariaLabel="Open all parent controls"
        cfg={cfg}
      >
        {(bounds) => (
          <g>
            {topGames.slice(0, 5).map((game, index) => (
              <TopGameRow
                key={`${game.id}:${index}`}
                game={game}
                x={bounds.x + 8}
                y={bounds.y + 8 + index * 25}
                w={bounds.w - 16}
                selected={game.id === selectedGameId}
                onSelect={() => onGameSelect(game.id)}
                cfg={cfg}
              />
            ))}
          </g>
        )}
      </RightRailCard>
      <RightRailCard
        x={rightX}
        y={distributionY}
        w={rightW}
        h={distributionH}
        tone="cyan"
        title={uiCopy.distributionTitle}
        subtitle={`${uiCopy.distributionCenterLabel} / ${totalPlayersValue}`}
        onClick={openDistribution}
        ariaLabel="Open data custody detail"
        cfg={cfg}
      >
        {(bounds) => (
          <g>
            <DistributionChart
              x={bounds.x + 78}
              y={bounds.y + 84}
              totalLabel={uiCopy.distributionCenterLabel}
              totalValue={totalPlayersValue}
              cfg={cfg}
            />
            {distributionStats.map((stat, index) => {
              const statColor = toneColor(stat.tone, cfg);
              const statY = bounds.y + 30 + index * 25;
              return (
                <g key={`${stat.label}-${stat.value}`}>
                  <rect
                    x={bounds.x + 154}
                    y={statY - 12}
                    width={bounds.w - 168}
                    height={18}
                    rx={5}
                    fill={`${statColor}12`}
                    stroke={statColor}
                    strokeOpacity={0.36}
                  />
                  <rect x={bounds.x + 162} y={statY - 5} width={8} height={8} fill={statColor} />
                  <text x={bounds.x + 178} y={statY + 3} fontSize={9.4} fontWeight={900} fill={cfg.colors.bodyText}>
                    {stat.label}
                  </text>
                  <text
                    x={bounds.x + bounds.w - 18}
                    y={statY + 3}
                    textAnchor="end"
                    fontSize={9.4}
                    fontWeight={950}
                    fill={cfg.colors.bodyText}
                  >
                    {stat.value}
                  </text>
                </g>
              );
            })}
          </g>
        )}
      </RightRailCard>
      <RightRailCard
        x={rightX}
        y={feedY}
        w={rightW}
        h={feedH}
        tone="cyan"
        title={uiCopy.feedTitle}
        subtitle={`${selectedGame?.name ?? 'Leaderboard'} / ${detail.primary}`}
        onClick={() => onDetailOpen('player')}
        ariaLabel="Open live device feed"
        cfg={cfg}
      >
        {(bounds) => (
          <g>
            {[
              `${selectedPlayer.player} hit ${selectedPlayer.rating} rating`,
              `${selectedGame?.name ?? 'Leaderboard'} updated ${detail.primary.toLowerCase()}`,
            ].map((line, index) => {
              const eventY = bounds.y + 12 + index * 40;
              return (
                <g key={line}>
                  <path
                    d={cutRectPath(bounds.x + 8, eventY, bounds.w - 16, 32, 6)}
                    fill={index === 0 ? 'rgba(64, 255, 139, 0.08)' : 'rgba(65, 210, 255, 0.08)'}
                    stroke={index === 0 ? '#40ff8b' : cfg.colors.cyan}
                    strokeWidth={0.7}
                    strokeOpacity={0.44}
                  />
                  {index === 0 ? (
                    <PlayerAvatarSlot
                      cx={bounds.x + 27}
                      cy={eventY + 16}
                      r={10}
                      player={selectedPlayer.player}
                      tone={selectedPlayer.tone}
                      cfg={cfg}
                    />
                  ) : (
                    <GameArtBadge
                      x={bounds.x + 17}
                      y={eventY + 6}
                      size={20}
                      gameId={selectedGame?.id ?? selectedGame?.name}
                      tone={detail.tone}
                      cfg={cfg}
                    />
                  )}
                  <text x={bounds.x + 48} y={eventY + 14} fontSize={10.2} fontWeight={900} fill={cfg.colors.bodyText}>
                    {line}
                  </text>
                  <text x={bounds.x + 48} y={eventY + 27} fontSize={8.8} fontWeight={760} fill={cfg.colors.mutedText}>
                    {index === 0 ? 'Live device movement' : 'Control scope changed'}
                  </text>
                </g>
              );
            })}
            <SelectionSummary
              x={bounds.x + 8}
              y={bounds.y + bounds.h - 66}
              w={bounds.w - 16}
              label={uiCopy.selectedPlayerLabel}
              player={selectedPlayer}
              selectedGame={selectedGame?.name ?? selectedGameId}
              detail={detail}
              onOpen={() => onDetailOpen('player')}
              cfg={cfg}
            />
          </g>
        )}
      </RightRailCard>
    </g>
  );
}

function QuickAccessHubCard({
  x,
  y,
  w,
  h,
  onOpen,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  onOpen: () => void;
  cfg: LeaderboardPageSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const titleSize = fitSingleLineTextSize('QUICK ACCESS', w - 76, 10, 13.5, 0.58);
  const path = topCutRectPath(x, y, w, h, 12);
  return (
    <g
      className="leaderboard-page-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label="Open leaderboard hub"
      onClick={(event) => {
        event.stopPropagation();
        onOpen();
      }}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        event.stopPropagation();
        onOpen();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      {hovered ? (
        <path
          d={topCutRectPath(x - 4, y - 4, w + 8, h + 8, 14)}
          fill="none"
          stroke={cfg.colors.gold}
          strokeWidth={2.1}
          opacity={0.34}
          filter="url(#leaderboardGoldGlow)"
          pointerEvents="none"
        />
      ) : null}
      <path
        d={path}
        fill={hovered ? colorAlpha(cfg.colors.gold, '24') : 'url(#leaderboardFrameFill)'}
        stroke={cfg.colors.gold}
        strokeWidth={hovered ? 1.8 : 1.3}
        pointerEvents="none"
      />
      <path
        d={topCutRectPath(x + 4, y + 4, w - 8, Math.min(28, h * 0.46), 8)}
        fill="url(#leaderboardFrameShine)"
        opacity={hovered ? 0.6 : 0.38}
        pointerEvents="none"
      />
      <ArtworkSlot
        x={x + 14}
        y={y + 13}
        w={48}
        h={h - 26}
        label="HUB"
        imageUrl={bannerGlobalLeaderboardImageUrl}
        tone="gold"
        shape="hex"
        cfg={cfg}
      />
      <text x={x + 76} y={y + 30} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
        QUICK ACCESS
      </text>
      <text x={x + 76} y={y + 52} fontSize={9.5} fontWeight={800} fill="#b5cde3">
        Leaderboard hub
      </text>
      <ChevronRight x={x + w - 30} y={y + h / 2 - 9} width={18} height={18} color={cfg.colors.gold} />
    </g>
  );
}

function QuickAccessRail({
  quickGames,
  selectedGameId,
  onGameSelect,
  cfg,
  x,
  y,
  w,
  h,
}: {
  quickGames: QuickGame[];
  selectedGameId: string;
  onGameSelect: (gameId: string) => void;
  cfg: LeaderboardPageSvgControls;
  x: number;
  y: number;
  w: number;
  h: number;
}) {
  const [offset, setOffset] = useState(0);
  const [dragStartX, setDragStartX] = useState<number | null>(null);
  const [slideState, setSlideState] = useState({ direction: 0, nonce: 0 });
  if (quickGames.length === 0) return null;
  const gap = 16;
  const handleW = LEADERBOARD_SIDE_HANDLE_W;
  const handleOverlap = LEADERBOARD_SIDE_HANDLE_OVERLAP;
  const leftHandleX = x;
  const rightHandleX = x + w - handleW;
  const trackX = leftHandleX + handleW - handleOverlap;
  const trackW = Math.max(1, rightHandleX + handleOverlap - trackX);
  const trackPad = 7;
  const totalCount = quickGames.length;
  const cardAreaX = trackX + trackPad;
  const cardAreaW = Math.max(1, trackW - trackPad * 2);
  const visibleCount = Math.max(
    1,
    Math.min(totalCount, Math.floor((cardAreaW + gap) / (LEADERBOARD_GAME_CARD_MIN_W + gap)))
  );
  const cardW = (cardAreaW - gap * Math.max(0, visibleCount - 1)) / visibleCount;
  const shiftRail = (direction: number) => {
    setSlideState((current) => ({ direction, nonce: current.nonce + 1 }));
    setOffset((current) => (current + direction + totalCount) % totalCount);
  };
  const handleWheel = (event: WheelEvent<SVGGElement>) => {
    event.preventDefault();
    const direction = event.deltaY + event.deltaX > 0 ? 1 : -1;
    shiftRail(direction);
  };
  const handleMouseDown = (event: MouseEvent<SVGGElement>) => {
    setDragStartX(event.clientX);
  };
  const handleMouseMove = (event: MouseEvent<SVGGElement>) => {
    if (dragStartX === null) return;
    const delta = event.clientX - dragStartX;
    if (Math.abs(delta) < 44) return;
    shiftRail(delta < 0 ? 1 : -1);
    setDragStartX(event.clientX);
  };
  const orderedGames = Array.from({ length: visibleCount }, (_, index) => quickGames[(index + offset) % totalCount]);
  const pageCount = Math.max(1, Math.ceil(totalCount / visibleCount));
  const currentPage = wrapIndex(Math.floor(offset / visibleCount), pageCount);
  const goToPage = (pageIndex: number) => {
    const nextOffset = wrapIndex(pageIndex * visibleCount, totalCount);
    setSlideState((current) => ({ direction: nextOffset >= offset ? 1 : -1, nonce: current.nonce + 1 }));
    setOffset(nextOffset);
  };
  const handleH = Math.max(38, h - 28);
  const handleY = y + Math.max(4, (h - handleH) / 2);
  const cardY = y + 5;
  const cardH = Math.max(50, h - 17);
  const trackDistance = Math.min(trackW * 0.16, 130);
  const fromX = slideState.direction > 0 ? trackDistance : slideState.direction < 0 ? -trackDistance : 0;
  const selectedKey = `game:${normalizeSelectionId(selectedGameId)}`;
  return (
    <g>
      <LeaderboardFrameSideHandle
        x={leftHandleX}
        y={handleY}
        side="left"
        height={handleH}
        width={handleW}
        disabled={totalCount <= visibleCount}
        onClick={() => shiftRail(-1)}
        cfg={cfg}
      />
      <rect x={trackX} y={y} width={trackW} height={h} fill="transparent" pointerEvents="all" />
      <g
        key={slideState.nonce}
        onWheel={handleWheel}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={() => setDragStartX(null)}
        onMouseLeave={() => setDragStartX(null)}
        opacity={1}
      >
        {slideState.direction !== 0 ? (
          <>
            <animateTransform
              attributeName="transform"
              type="translate"
              from={`${fromX} 0`}
              to="0 0"
              dur="190ms"
              calcMode="spline"
              keySplines="0.2 0.8 0.2 1"
            />
            <animate
              attributeName="opacity"
              from="0.42"
              to="1"
              dur="190ms"
              calcMode="spline"
              keySplines="0.2 0.8 0.2 1"
            />
          </>
        ) : null}
        {orderedGames.map((game, index) => {
          const cardX = cardAreaX + index * (cardW + gap);
          const item = gameTopCard(game, undefined, offset + index);
          return (
            <LeaderboardTopCarouselCard
              key={item.key}
              item={item}
              x={cardX}
              y={cardY}
              w={cardW}
              h={cardH}
              selected={item.key.startsWith(`${selectedKey}:`)}
              onSelect={() => onGameSelect(game.id)}
              gameHoverAnchor="up"
              cfg={cfg}
            />
          );
        })}
      </g>
      <LeaderboardFrameDots
        x={trackX + trackW / 2}
        y={y + h - 8}
        page={currentPage}
        pageCount={pageCount}
        onPageChange={goToPage}
        cfg={cfg}
      />
      <LeaderboardFrameSideHandle
        x={rightHandleX}
        y={handleY}
        side="right"
        height={handleH}
        width={handleW}
        disabled={totalCount <= visibleCount}
        onClick={() => shiftRail(1)}
        cfg={cfg}
      />
    </g>
  );
}

function Defs() {
  return (
    <defs>
      <linearGradient id="leaderboardActiveBlue" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#39dfff" stopOpacity="0.74" />
        <stop offset="48%" stopColor="#266aa6" stopOpacity="0.78" />
        <stop offset="100%" stopColor="#273184" stopOpacity="0.9" />
      </linearGradient>
      <radialGradient id="leaderboardHoverShine" cx="18%" cy="0%" r="125%">
        <stop offset="0%" stopColor="#ffffff" stopOpacity="0.16" />
        <stop offset="35%" stopColor="#42e8ff" stopOpacity="0.1" />
        <stop offset="100%" stopColor="#42e8ff" stopOpacity="0" />
      </radialGradient>
      <linearGradient id="leaderboardFrameFill" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#08243a" stopOpacity="0.94" />
        <stop offset="48%" stopColor="#041624" stopOpacity="0.96" />
        <stop offset="100%" stopColor="#061d31" stopOpacity="0.92" />
      </linearGradient>
      <linearGradient id="leaderboardFrameGlass" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#123f59" stopOpacity="0.32" />
        <stop offset="52%" stopColor="#061525" stopOpacity="0.24" />
        <stop offset="100%" stopColor="#0b2445" stopOpacity="0.28" />
      </linearGradient>
      <linearGradient id="leaderboardFrameShine" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" stopColor="#ffffff" stopOpacity="0.14" />
        <stop offset="42%" stopColor="#42e8ff" stopOpacity="0.07" />
        <stop offset="100%" stopColor="#42e8ff" stopOpacity="0" />
      </linearGradient>
      <linearGradient id="leaderboardGameCardBannerShade" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" stopColor="#07111f" stopOpacity="0.16" />
        <stop offset="52%" stopColor="#06101f" stopOpacity="0.46" />
        <stop offset="100%" stopColor="#061525" stopOpacity="0.92" />
      </linearGradient>
      <linearGradient id="leaderboardFrameCountGold" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" stopColor="#fff7c7" />
        <stop offset="22%" stopColor="#ffd85c" />
        <stop offset="56%" stopColor="#d89010" />
        <stop offset="100%" stopColor="#4a2a00" />
      </linearGradient>
      <linearGradient id="leaderboardFooterActivePill" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#ffe187" />
        <stop offset="100%" stopColor="#b98214" />
      </linearGradient>
      <filter id="leaderboardGlow" x="-35%" y="-35%" width="170%" height="170%">
        <feGaussianBlur stdDeviation={3.5} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
      <filter id="leaderboardGreenGlow" x="-60%" y="-60%" width="220%" height="220%">
        <feGaussianBlur stdDeviation={5} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
      <filter id="leaderboardGoldGlow" x="-60%" y="-60%" width="220%" height="220%">
        <feGaussianBlur stdDeviation={4} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
    </defs>
  );
}

export function LeaderboardPageSvgSurface({
  pageMode,
  gameType,
  seasonId,
  lastUpdated,
  leaderboardEntries,
  userEntry,
  nearbyAbove,
  nearbyBelow,
  gameId,
  loading = false,
  error = null,
  controls,
  content,
  initialNavLabel,
  initialSelectedGameId,
  onRefreshLeaderboard,
  onMatchmaking,
  onNavigate,
}: LeaderboardPageSvgSurfaceProps) {
  const mainRef = useRef<HTMLElement | null>(null);
  const baseCfg = useMemo(() => normalizeLeaderboardPageSvgControls(controls), [controls]);
  const pageContent = useMemo(() => normalizeLeaderboardPageContent(content), [content]);
  const tabs = pageContent.tabs;
  const tabDetails = pageContent.tabDetails;
  const navItems = useMemo<NavItem[]>(
    () =>
      pageContent.navItems.map((item) => ({
        ...item,
        icon: iconForName(item.icon),
        imageUrl: navItemImageUrl(item),
      })),
    [pageContent.navItems]
  );
  const navGroups = useMemo(
    () => groupedLeaderboardNavItems(pageContent.navGroups, navItems),
    [navItems, pageContent.navGroups]
  );
  const topGames = pageContent.topGames;
  const quickGames = useMemo<QuickGame[]>(() => {
    const gameEntries = pageContent.quickGames.filter(isLeaderboardGameEntry).map((game) => ({
      ...game,
      icon: iconForName(game.icon),
    }));
    if (gameEntries.length > 0) return gameEntries;
    return pageContent.topGames.map((game) => ({
      id: game.id,
      name: game.name,
      detail: typeof game.gameType === 'number' ? `Game type ${game.gameType}` : 'View leaderboard',
      icon: Crown,
      tone: game.tone,
      category: game.category,
      subcategory: game.subcategory,
      gameType: game.gameType,
      routePath: game.routePath,
    }));
  }, [pageContent.quickGames, pageContent.topGames]);
  const [surfaceSize, setSurfaceSize] = useState({ width: 0, height: 0 });
  const canvasSize = useMemo(
    () => leaderboardCanvasSizeForSurface(baseCfg, surfaceSize),
    [baseCfg, surfaceSize.height, surfaceSize.width]
  );
  const columns = useMemo(
    () => responsiveLeaderboardColumnWidths(canvasSize.width, baseCfg),
    [baseCfg, canvasSize.width]
  );
  const cfg = useMemo<LeaderboardPageSvgControls>(
    () => ({
      ...baseCfg,
      canvas: {
        ...baseCfg.canvas,
        width: canvasSize.width,
        height: canvasSize.height,
      },
      layout: {
        ...baseCfg.layout,
        leftW: columns.leftW,
        rightW: columns.rightW,
      },
    }),
    [baseCfg, canvasSize.height, canvasSize.width, columns.leftW, columns.rightW]
  );
  const pageModeTab = initialTabForPageMode(pageMode, pageContent);
  const initialNavItem = navItems.find((item) => item.label === initialNavLabel);
  const initialTab = initialNavItem?.tabId ?? pageModeTab;
  const [activeTab, setActiveTab] = useState<LeaderboardTabId>(initialTab);
  const [activeNavLabel, setActiveNavLabel] = useState(
    () => initialNavItem?.label ?? initialNavLabelForTab(navItems, pageModeTab)
  );
  const [openNavGroupIds, setOpenNavGroupIds] = useState(() =>
    initialOpenNavGroupIds(navGroups, initialNavItem?.label ?? initialNavLabelForTab(navItems, pageModeTab))
  );
  const [selectedGameId, setSelectedGameId] = useState(
    initialSelectedGameId ?? initialGameIdForPageMode(pageMode, pageContent, gameId)
  );
  const selectedGame = findSelectedGame(pageContent, selectedGameId);
  const selectedGameName = selectedGame?.name ?? formatRouteScope(gameId);
  const baseSourceRows = useMemo(
    () => rowSourceForPageMode(pageContent, pageMode, leaderboardEntries),
    [leaderboardEntries, pageContent, pageMode]
  );
  const sourceRows = activeTab === 'aiBenchmarks' ? pageContent.aiBenchmarkRows : baseSourceRows;
  const rows = useMemo(
    () => toDisplayRows(sourceRows, pageMode, selectedGameName, gameId),
    [gameId, pageMode, selectedGameName, sourceRows]
  );
  const userDisplayRow = useMemo(
    () =>
      userEntry && pageMode !== 'aiBenchmarkLeaderboard'
        ? toDisplayRows([userEntry], pageMode, selectedGameName, gameId)[0]
        : null,
    [gameId, pageMode, selectedGameName, userEntry]
  );
  const [selectedPlayerId, setSelectedPlayerId] = useState(userDisplayRow?.id ?? rows[0]?.id ?? '');
  const [page, setPage] = useState(1);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [detailMode, setDetailMode] = useState<DetailMode | null>(null);

  useEffect(() => {
    const nextNavItem = navItems.find((item) => item.label === initialNavLabel);
    const nextTab = nextNavItem?.tabId ?? pageModeTab;
    const nextNavLabel = nextNavItem?.label ?? initialNavLabelForTab(navItems, nextTab);
    setActiveTab(nextTab);
    setActiveNavLabel(nextNavLabel);
    setOpenNavGroupIds((current) => ensureOpenNavGroupIds(current, navGroups, nextNavLabel));
    setSelectedGameId(initialSelectedGameId ?? initialGameIdForPageMode(pageMode, pageContent, gameId));
    setDetailMode(null);
    setPage(1);
  }, [gameId, initialNavLabel, initialSelectedGameId, navGroups, navItems, pageContent, pageMode, pageModeTab]);

  useEffect(() => {
    const target = mainRef.current;
    if (!target || typeof ResizeObserver === 'undefined') return undefined;
    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;
      const { width, height } = entry.contentRect;
      setSurfaceSize((current) => {
        const nextWidth = Math.round(width);
        const nextHeight = Math.round(height);
        return current.width === nextWidth && current.height === nextHeight
          ? current
          : { width: nextWidth, height: nextHeight };
      });
    });
    observer.observe(target);
    return () => observer.disconnect();
  }, []);

  const selectedPlayer =
    rows.find((row) => row.id === selectedPlayerId) ??
    (activeTab === 'aiBenchmarks' ? null : userDisplayRow) ??
    rows[0] ??
    toDisplayRows(pageContent.fallbackRows, pageMode, selectedGameName, gameId)[0];
  const leftX = cfg.layout.outerPad;
  const sideMainGap = Math.max(1, Math.round(cfg.layout.gap * 0.5));
  const mainX = leftX + cfg.layout.leftW + sideMainGap;
  const rightX = cfg.canvas.width - cfg.layout.outerPad - cfg.layout.rightW;
  const boardY = cfg.layout.topY;
  const mainW = rightX - mainX;
  const mainH = cfg.canvas.height - boardY - 6;
  const activeNavGroupId = navGroupIdForNavLabel(navGroups, activeNavLabel);
  const activateNavLabel = (navLabel: string) => {
    setActiveNavLabel(navLabel);
    setOpenNavGroupIds((current) => ensureOpenNavGroupIds(current, navGroups, navLabel));
  };
  const changeTab = (tab: LeaderboardTabId) => {
    const nextNavLabel = initialNavLabelForTab(navItems, tab);
    setActiveTab(tab);
    activateNavLabel(nextNavLabel);
    setDetailMode(null);
    setPage(1);
  };
  const selectNavItem = (item: NavItem) => {
    setActiveTab(item.tabId);
    activateNavLabel(item.label);
    setDetailMode(null);
    setPage(1);
    if (isHashRoutePath(item.routePath)) {
      onNavigate?.(item.routePath);
    }
  };
  const toggleNavGroup = (groupId: string) => {
    setOpenNavGroupIds((current) => toggleOpenNavGroupId(current, navGroups, groupId));
  };
  const selectQuickAccessHub = () => {
    const nextNavLabel = initialNavLabelForTab(navItems, 'overall');
    setActiveTab('overall');
    activateNavLabel(nextNavLabel);
    setDetailMode(null);
    setPage(1);
  };
  const selectChat = () => {
    setActiveTab('aiBenchmarks');
    activateNavLabel('CHAT');
    setDetailMode(null);
    setPage(1);
  };
  const selectPlayer = (playerId: string) => {
    setSelectedPlayerId(playerId);
    setDetailMode(null);
  };
  const selectGame = (gameIdValue: string) => {
    setSelectedGameId(gameIdValue);
    const game = findSelectedGame(pageContent, gameIdValue);
    const tab =
      game?.routePath === '#/ai-runtime' || assetKey(game?.category ?? '').includes('ai')
        ? 'aiBenchmarks'
        : game?.routePath === '#/overview'
          ? 'overall'
          : 'perGame';
    const nextNavLabel = initialNavLabelForTab(navItems, tab);
    setActiveTab(tab);
    activateNavLabel(nextNavLabel);
    setDetailMode(null);
    setPage(1);
    if (pageMode === 'gameLeaderboard' && typeof game?.gameType === 'number') {
      onRefreshLeaderboard(game.gameType);
    }
    if (isHashRoutePath(game?.routePath)) {
      onNavigate?.(game.routePath);
    }
  };
  const cycleRowsPerPage = () => {
    setRowsPerPage((current) => (current === 10 ? 25 : current === 25 ? 50 : 10));
  };
  return (
    <main ref={mainRef} className="leaderboard-page-svg-main">
      <svg
        viewBox={`0 0 ${cfg.canvas.width} ${cfg.canvas.height}`}
        className="leaderboard-page-svg-surface"
        role="img"
        aria-label="Ocentra parent dashboard"
        preserveAspectRatio="xMidYMin meet"
      >
        <Defs />
        <NavPanel
          activeNavLabel={activeNavLabel}
          navGroups={navGroups}
          openGroupIds={openNavGroupIds}
          onNavGroupToggle={toggleNavGroup}
          onNavItemSelect={selectNavItem}
          onChatOpen={selectChat}
          cfg={cfg}
        />
        <MainBoard
          activeNavLabel={activeNavLabel}
          activeNavGroupId={activeNavGroupId}
          activeTab={activeTab}
          rows={rows}
          tabs={tabs}
          tabDetails={tabDetails}
          topGames={topGames}
          quickGames={quickGames}
          guideTopics={pageContent.guideTopics}
          season={pageContent.season}
          uiCopy={pageContent.uiCopy}
          selectedGameId={selectedGameId}
          selectedPlayerId={selectedPlayer.id}
          selectedPlayer={selectedPlayer}
          selectedGameName={selectedGameName}
          page={page}
          rowsPerPage={rowsPerPage}
          detailMode={detailMode}
          onTabChange={changeTab}
          onPlayerSelect={selectPlayer}
          onPageChange={setPage}
          onRowsPerPageChange={cycleRowsPerPage}
          onDetailOpen={setDetailMode}
          onDetailClose={() => setDetailMode(null)}
          onGameSelect={selectGame}
          onNavigate={onNavigate}
          onSelectNavLabel={(navLabel) => {
            const item = navItems.find((entry) => entry.label === navLabel);
            if (item) {
              selectNavItem(item);
              return;
            }
            activateNavLabel(navLabel);
          }}
          onRefresh={() => onRefreshLeaderboard(gameType)}
          onMatchmaking={onMatchmaking}
          cfg={cfg}
          mainX={mainX}
          mainW={mainW}
          mainY={boardY}
          mainH={mainH}
          rightX={rightX}
          rightW={cfg.layout.rightW}
        />
        {loading || error ? (
          <g role={loading ? 'status' : 'alert'}>
            <rect
              x={mainX + 28}
              y={boardY + 120}
              width={mainW - 56}
              height={82}
              rx={6}
              fill="rgba(3, 7, 18, 0.82)"
              stroke={error ? cfg.colors.red : cfg.colors.cyan}
              strokeWidth={1.2}
            />
            <text
              x={mainX + mainW / 2}
              y={boardY + 154}
              textAnchor="middle"
              fontSize={16}
              fontWeight={950}
              fill={cfg.colors.bodyText}
            >
              {loading ? pageContent.uiCopy.loadingTitle : pageContent.uiCopy.errorTitle}
            </text>
            <text
              x={mainX + mainW / 2}
              y={boardY + 180}
              textAnchor="middle"
              fontSize={11}
              fontWeight={760}
              fill={cfg.colors.mutedText}
            >
              {error ?? pageContent.uiCopy.loadingBody}
            </text>
          </g>
        ) : null}
      </svg>
    </main>
  );
}
