import {
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type DragEvent,
  type KeyboardEvent as ReactKeyboardEvent,
  type MouseEvent as ReactMouseEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactElement,
  type ReactNode,
} from 'react';
import { createPortal } from 'react-dom';
import './weekly-scheduler-scratch.css';

type WeeklySchedulerActionId = 'ask' | 'allow' | 'block' | 'limit' | 'observe';
type WeeklySchedulerDay = 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat' | 'sun';
type WeeklySchedulerMenuId = 'day' | 'preset' | `tone-${WeeklySchedulerActionId}`;
type WeeklySchedulerMode = 'daily' | 'weekly';
type WeeklySchedulerPolicyArea = 'apps' | 'browser' | 'games';
type WeeklySchedulerPresetNoun = 'app' | 'game';
type WeeklySchedulerPresetScope = 'selected-day' | 'week';
type WeeklySchedulerPresetId =
  | 'allow-all'
  | 'ask-all'
  | 'balanced'
  | 'limit-all'
  | 'limit-balanced'
  | 'night-block'
  | 'observe-only'
  | 'relaxed'
  | 'strict'
  | `profile-${string}`;
type WeeklySchedulerControlId = string;
type WeeklySchedulerControlGroupId = string;
type WeeklySchedulerTimeEditorEdge = 'end' | 'start';
type WeeklySchedulerTimeEditorPart = 'hour' | 'minute';

type WeeklySchedulerTimeEditorDraft = {
  readonly endMinute: number;
  readonly startMinute: number;
};

type WeeklySchedulerTimeEditorTarget =
  | { readonly blockId: string; readonly kind: 'block' }
  | { readonly blockId: string; readonly kind: 'subclip'; readonly subClipId: string };

type WeeklySchedulerControlChoice = {
  readonly id: WeeklySchedulerControlId;
  readonly label: string;
};

type WeeklySchedulerControlGroup = {
  readonly id: WeeklySchedulerControlGroupId;
  readonly label: string;
  readonly choices: readonly WeeklySchedulerControlChoice[];
};

type WeeklySchedulerControlGroupsByTone = Record<WeeklySchedulerActionId, readonly WeeklySchedulerControlGroup[]>;

type WeeklySchedulerPresetControls = {
  readonly allowAll: readonly WeeklySchedulerControlId[];
  readonly askAll: readonly WeeklySchedulerControlId[];
  readonly blockAll: readonly WeeklySchedulerControlId[];
  readonly limitAll: readonly WeeklySchedulerControlId[];
  readonly observeAll: readonly WeeklySchedulerControlId[];
};

type WeeklySchedulerProfile = {
  readonly controlGroupsByTone: WeeklySchedulerControlGroupsByTone;
  readonly customPresetStorageKey: string;
  readonly defaultActiveGroupByTone: Record<WeeklySchedulerActionId, WeeklySchedulerControlGroupId>;
  readonly defaultPresetId: WeeklySchedulerPresetId;
  readonly defaultToneControls: Record<WeeklySchedulerActionId, readonly WeeklySchedulerControlId[]>;
  readonly id: WeeklySchedulerPolicyArea;
  readonly presets: readonly WeeklySchedulerPreset[];
};

type WeeklySchedulerSubClip = {
  readonly tone?: WeeklySchedulerActionId;
  readonly controlId: WeeklySchedulerControlId;
  readonly disabled?: boolean;
  readonly endHour: number;
  readonly id: string;
  readonly label: string;
  readonly startHour: number;
};

type WeeklySchedulerSubClipTrack = {
  readonly clips: readonly WeeklySchedulerSubClip[];
  readonly controlId: WeeklySchedulerControlId;
  readonly disabled: boolean;
  readonly label: string;
};

type WeeklySchedulerNavigatorState = {
  readonly endHour: number;
  readonly startHour: number;
};

type WeeklySchedulerFullscreenBounds = {
  readonly bottom: number;
  readonly top: number;
};

type WeeklySchedulerPresetSegment = {
  readonly controls: readonly WeeklySchedulerControlId[];
  readonly days?: readonly WeeklySchedulerDay[];
  readonly endHour: number;
  readonly label: string;
  readonly startHour: number;
  readonly tone: WeeklySchedulerActionId;
};

type WeeklySchedulerSubClipWindow = {
  readonly endHour: number;
  readonly startHour: number;
  readonly tone: WeeklySchedulerActionId;
};

type WeeklySchedulerPreset = {
  readonly blocks?: readonly WeeklySchedulerBlock[];
  readonly description: string;
  readonly id: WeeklySchedulerPresetId;
  readonly label: string;
  readonly segments: readonly WeeklySchedulerPresetSegment[];
};

export type WeeklySchedulerEdlControl = {
  readonly actionId: WeeklySchedulerActionId;
  readonly actionLabel: string;
  readonly controlId: WeeklySchedulerControlId;
  readonly disabled: boolean;
  readonly endMinute: number;
  readonly label: string;
  readonly mode: 'ganged' | 'independent';
  readonly startMinute: number;
};

export type WeeklySchedulerEdlItem = {
  readonly actionId: WeeklySchedulerActionId;
  readonly actionLabel: string;
  readonly controls: readonly WeeklySchedulerEdlControl[];
  readonly endMinute: number;
  readonly id: string;
  readonly label: string;
  readonly source: 'explicit' | 'implicit';
  readonly startMinute: number;
  readonly trackId: WeeklySchedulerDay;
  readonly trackLabel: string;
};

export type WeeklySchedulerEdl = {
  readonly actions: readonly {
    readonly id: WeeklySchedulerActionId;
    readonly label: string;
    readonly groups: readonly WeeklySchedulerControlGroup[];
  }[];
  readonly fallbackActionId: WeeklySchedulerActionId;
  readonly horizon: {
    readonly endMinute: number;
    readonly startMinute: number;
  };
  readonly items: readonly WeeklySchedulerEdlItem[];
  readonly snapMinutes: number;
  readonly tracks: typeof SCHEDULER_DAYS;
  readonly version: 'weekly-scheduler-edl/v1';
};

export type WeeklySchedulerBlock = {
  readonly id: string;
  readonly day: WeeklySchedulerDay;
  readonly endHour: number;
  readonly label: string;
  readonly startHour: number;
  readonly subClips?: readonly WeeklySchedulerSubClip[];
  readonly tone: WeeklySchedulerActionId;
};

type WeeklySchedulerScratchPageProps = {
  readonly blocks?: readonly WeeklySchedulerBlock[];
  readonly embedded?: boolean;
  readonly onScheduleChange?: (schedule: WeeklySchedulerEdl) => void;
  readonly policyArea?: WeeklySchedulerPolicyArea;
};

const SCHEDULER_DAYS: readonly { readonly id: WeeklySchedulerDay; readonly label: string }[] = [
  { id: 'mon', label: 'Mon' },
  { id: 'tue', label: 'Tue' },
  { id: 'wed', label: 'Wed' },
  { id: 'thu', label: 'Thu' },
  { id: 'fri', label: 'Fri' },
  { id: 'sat', label: 'Sat' },
  { id: 'sun', label: 'Sun' },
];
const SCHEDULER_DAY_IDS = SCHEDULER_DAYS.map((day) => day.id);
const SCHEDULER_DEFAULT_DAY: WeeklySchedulerDay = SCHEDULER_DAYS[0]?.id ?? 'mon';

const SCHEDULER_HOURS = Array.from({ length: 24 }, (_, hour) => hour);
const SCHEDULER_BASE_ROW_HEIGHT = 74;
const SCHEDULER_DAY_LABEL_WIDTH = 72;
const SCHEDULER_MIN_HOUR_WIDTH = 24;
const SCHEDULER_FIT_MIN_HOUR_WIDTH = 1;
const SCHEDULER_MIN_BLOCK_DURATION = 0.25;
const SCHEDULER_SNAP_MINUTES = 5;
const SCHEDULER_SNAP_HOURS = SCHEDULER_SNAP_MINUTES / 60;
const SCHEDULER_TINY_GAP_HOURS = 0.25;
const SCHEDULER_NAVIGATOR_MIN_HOURS = 4;
const SCHEDULER_TIME_LABEL_MIN_WIDTH = 72;
const SCHEDULER_PARENT_TIME_LABEL_MIN_WIDTH = 260;
const SCHEDULER_QA_MIN_HEIGHT = 844;
const SCHEDULER_QA_WIDTH_PARAM = 'qa-width';
const SCHEDULER_MENU_IDLE_CLOSE_MS = 2600;
const SCHEDULER_MENU_CLOSE_ANIMATION_MS = 170;

type WeeklySchedulerCssVars = CSSProperties & Record<string, string>;

const SCHEDULER_CLASS_NAMES = {
  block: 'weekly-scheduler-block',
  blockGrip: 'weekly-scheduler-block-grip',
  blockMain: 'weekly-scheduler-block-main',
  blockResize: 'weekly-scheduler-block-resize',
  blockTimeEditor: 'weekly-scheduler-block-time-editor',
  legendItem: 'weekly-scheduler-legend-item',
  modeToggle: 'weekly-scheduler-mode-toggle',
  navigatorRail: 'weekly-scheduler-navigator-rail',
  presetControl: 'weekly-scheduler-preset-control',
  subClip: 'weekly-scheduler-subclip',
  subClipGrip: 'weekly-scheduler-subclip-grip',
} as const;

const SCHEDULER_DOCUMENT_ATTRIBUTES = {
  fullscreen: 'data-weekly-scheduler-fullscreen',
} as const;

const SCHEDULER_DOM_SELECTORS = {
  button: 'button',
} as const;

const SCHEDULER_DRAG_FORMATS = {
  actionId: 'application/x-weekly-scheduler-action-id',
  plainText: 'text/plain',
} as const;

const SCHEDULER_CONTROL_MODES = {
  ganged: 'ganged',
  independent: 'independent',
} as const;

const SCHEDULER_EDL_SOURCES = {
  explicit: 'explicit',
  implicit: 'implicit',
} as const;

const SCHEDULER_RESULT = {
  fallbackActionId: 'observe',
  implicitObserveControlId: 'observe.implicit',
  version: 'weekly-scheduler-edl/v1',
} as const;

const DEFAULT_BLOCKS: readonly WeeklySchedulerBlock[] = [];
const DEFAULT_PRESET_ID: WeeklySchedulerPresetId = 'balanced';
const SCHEDULER_CUSTOM_PRESET_STORAGE_KEY = 'weekly-scheduler-custom-presets-v1';
const SCHEDULER_WEEKDAY_IDS: readonly WeeklySchedulerDay[] = ['mon', 'tue', 'wed', 'thu', 'fri'];
const SCHEDULER_WEEKEND_IDS: readonly WeeklySchedulerDay[] = ['sat', 'sun'];

const SCHEDULER_COPY = {
  browserActivity: 'Browser activity',
  daily: 'Daily',
  implicitObserve: 'Implicit observe',
  modeLabel: 'Schedule view',
  nothing: 'Nothing',
  observe: 'Observe',
  quickPreset: 'Quick preset',
  weekly: 'Weekly',
} as const;

const SCHEDULER_LEGENDS: readonly { readonly label: string; readonly tone: WeeklySchedulerActionId }[] = [
  { label: 'Block', tone: 'block' },
  { label: 'Limit', tone: 'limit' },
  { label: 'Ask', tone: 'ask' },
  { label: 'Allow', tone: 'allow' },
  { label: 'Observe', tone: 'observe' },
];

const SCHEDULER_CONTROL_GROUPS_BY_TONE: WeeklySchedulerControlGroupsByTone = {
  block: [
    {
      id: 'block-common',
      label: 'Common',
      choices: [
        { id: 'block.all', label: 'All' },
        { id: 'block.social-media', label: 'Social media' },
        { id: 'block.browser-games', label: 'Browser games' },
        { id: 'block.search', label: 'Search' },
        { id: 'block.video', label: 'YouTube/video' },
        { id: 'block.downloads', label: 'Downloads' },
      ],
    },
    {
      id: 'block-bypass',
      label: 'Bypass',
      choices: [
        { id: 'block.unmanaged-browsers', label: 'Unmanaged browsers' },
        { id: 'block.private-incognito', label: 'Private/incognito' },
        { id: 'block.tor-proxy-vpn', label: 'Tor/proxy/VPN' },
        { id: 'block.browser-installs', label: 'Browser installs' },
      ],
    },
  ],
  limit: [
    {
      id: 'limit-content',
      label: 'Content',
      choices: [
        { id: 'limit.social-media', label: 'Social media' },
        { id: 'limit.browser-games', label: 'Browser games' },
        { id: 'limit.video', label: 'YouTube/video' },
        { id: 'limit.search', label: 'Search' },
      ],
    },
    {
      id: 'limit-time',
      label: 'Time',
      choices: [
        { id: 'limit.total-browser-time', label: 'Total browser time' },
        { id: 'limit.managed-browser-time', label: 'Managed browser time' },
        { id: 'limit.unmanaged-browser-time', label: 'Unmanaged browser time' },
        { id: 'limit.downloads', label: 'Downloads' },
      ],
    },
  ],
  ask: [
    {
      id: 'ask-requests',
      label: 'Requests',
      choices: [
        { id: 'ask.new-sites', label: 'New/unknown sites' },
        { id: 'ask.blocked-sites', label: 'Blocked sites' },
        { id: 'ask.downloads', label: 'Downloads' },
        { id: 'ask.time-extension', label: 'Time extension' },
      ],
    },
    {
      id: 'ask-bypass',
      label: 'Bypass',
      choices: [
        { id: 'ask.unmanaged-browsers', label: 'Unmanaged browsers' },
        { id: 'ask.private-incognito', label: 'Private/incognito' },
        { id: 'ask.browser-installs', label: 'Browser install/setup' },
        { id: 'ask.extension-profile', label: 'Extension/profile changes' },
      ],
    },
  ],
  allow: [
    {
      id: 'allow-social',
      label: 'Social',
      choices: [
        { id: 'allow.facebook', label: 'Facebook' },
        { id: 'allow.instagram', label: 'Instagram' },
        { id: 'allow.tiktok', label: 'TikTok' },
        { id: 'allow.snapchat', label: 'Snapchat' },
        { id: 'allow.x-twitter', label: 'X/Twitter' },
        { id: 'allow.discord', label: 'Discord' },
        { id: 'allow.reddit', label: 'Reddit' },
      ],
    },
    {
      id: 'allow-video',
      label: 'Video',
      choices: [
        { id: 'allow.youtube', label: 'YouTube' },
        { id: 'allow.twitch', label: 'Twitch' },
        { id: 'allow.netflix', label: 'Netflix' },
        { id: 'allow.streaming-sites', label: 'Streaming sites' },
      ],
    },
    {
      id: 'allow-games',
      label: 'Games',
      choices: [
        { id: 'allow.roblox', label: 'Roblox' },
        { id: 'allow.browser-games', label: 'Browser games' },
        { id: 'allow.game-wikis', label: 'Game wikis' },
        { id: 'allow.game-downloads', label: 'Game downloads' },
      ],
    },
    {
      id: 'allow-web',
      label: 'Web',
      choices: [
        { id: 'allow.search', label: 'Search' },
        { id: 'allow.school-domains', label: 'School domains' },
        { id: 'allow.news-reference', label: 'News/reference' },
        { id: 'allow.downloads', label: 'Downloads' },
      ],
    },
  ],
  observe: [
    {
      id: 'observe-reports',
      label: 'Reports',
      choices: [
        { id: 'observe.all', label: 'All' },
        { id: 'observe.social-media', label: 'Social media' },
        { id: 'observe.search', label: 'Search' },
        { id: 'observe.video', label: 'YouTube/video' },
        { id: 'observe.downloads', label: 'Downloads' },
        { id: 'observe.unmanaged-browsers', label: 'Unmanaged browsers' },
      ],
    },
    {
      id: 'observe-privacy',
      label: 'Privacy',
      choices: [
        { id: 'observe.browser-time', label: 'Browser/app time' },
        { id: 'observe.domain-summary', label: 'Domain summary' },
        { id: 'observe.exact-on-event', label: 'Exact URL on block/ask' },
      ],
    },
  ],
};

const SCHEDULER_APP_CONTROL_GROUPS_BY_TONE: WeeklySchedulerControlGroupsByTone = {
  block: [
    {
      id: 'block-apps',
      label: 'Apps',
      choices: [
        { id: 'block.all', label: 'All' },
        { id: 'block.social-chat-apps', label: 'Social/chat apps' },
        { id: 'block.short-video-apps', label: 'Short video apps' },
        { id: 'block.entertainment-apps', label: 'Entertainment apps' },
        { id: 'block.new-unknown-apps', label: 'New/unknown apps' },
        { id: 'block.app-installers', label: 'App installers' },
      ],
    },
    {
      id: 'block-app-risk',
      label: 'Risk',
      choices: [
        { id: 'block.unverified-publisher', label: 'Unverified publisher' },
        { id: 'block.admin-elevated-apps', label: 'Admin/elevated apps' },
        { id: 'block.background-only-apps', label: 'Background-only apps' },
      ],
    },
  ],
  limit: [
    {
      id: 'limit-app-category',
      label: 'Category',
      choices: [
        { id: 'limit.social-chat-apps', label: 'Social/chat apps' },
        { id: 'limit.video-streaming-apps', label: 'Video/streaming apps' },
        { id: 'limit.entertainment-apps', label: 'Entertainment apps' },
        { id: 'limit.productivity-apps', label: 'Productivity apps' },
      ],
    },
    {
      id: 'limit-app-time',
      label: 'Time',
      choices: [
        { id: 'limit.total-app-time', label: 'Total app time' },
        { id: 'limit.social-app-time', label: 'Social app time' },
        { id: 'limit.entertainment-time', label: 'Entertainment time' },
        { id: 'limit.background-time', label: 'Background time' },
      ],
    },
  ],
  ask: [
    {
      id: 'ask-app-requests',
      label: 'Requests',
      choices: [
        { id: 'ask.new-unknown-app', label: 'New/unknown app' },
        { id: 'ask.blocked-app-open', label: 'Blocked app open' },
        { id: 'ask.extra-app-time', label: 'Extra app time' },
        { id: 'ask.app-install-update', label: 'Install/update' },
      ],
    },
    {
      id: 'ask-app-risk',
      label: 'Risk',
      choices: [
        { id: 'ask.unverified-publisher', label: 'Unverified publisher' },
        { id: 'ask.suspicious-app', label: 'Suspicious app' },
        { id: 'ask.admin-elevation', label: 'Admin elevation' },
      ],
    },
  ],
  allow: [
    {
      id: 'allow-app-safe',
      label: 'Safe',
      choices: [
        { id: 'allow.school-apps', label: 'School apps' },
        { id: 'allow.productivity-apps', label: 'Productivity apps' },
        { id: 'allow.creative-apps', label: 'Creative apps' },
        { id: 'allow.family-communication', label: 'Family communication' },
      ],
    },
    {
      id: 'allow-app-named',
      label: 'Named',
      choices: [
        { id: 'allow.whatsapp', label: 'WhatsApp' },
        { id: 'allow.discord', label: 'Discord' },
        { id: 'allow.spotify', label: 'Spotify' },
        { id: 'allow.zoom-meet', label: 'Zoom/Meet' },
      ],
    },
  ],
  observe: [
    {
      id: 'observe-app-reports',
      label: 'Reports',
      choices: [
        { id: 'observe.all', label: 'All' },
        { id: 'observe.installed-apps', label: 'Installed apps' },
        { id: 'observe.foreground-time', label: 'Foreground time' },
        { id: 'observe.background-time', label: 'Background time' },
        { id: 'observe.new-app-discovery', label: 'New app discovery' },
      ],
    },
    {
      id: 'observe-app-privacy',
      label: 'Privacy',
      choices: [
        { id: 'observe.session-summary', label: 'Session summary' },
        { id: 'observe.app-category-summary', label: 'Category summary' },
        { id: 'observe.exact-app-on-ask', label: 'Exact app on ask/block' },
      ],
    },
  ],
};

const SCHEDULER_GAME_CONTROL_GROUPS_BY_TONE: WeeklySchedulerControlGroupsByTone = {
  block: [
    {
      id: 'block-games',
      label: 'Games',
      choices: [
        { id: 'block.all', label: 'All' },
        { id: 'block.native-games', label: 'Native games' },
        { id: 'block.browser-games', label: 'Browser games' },
        { id: 'block.cloud-games', label: 'Cloud games' },
        { id: 'block.launchers', label: 'Launchers' },
        { id: 'block.voice-chat', label: 'Voice/chat' },
      ],
    },
    {
      id: 'block-game-risk',
      label: 'Risk',
      choices: [
        { id: 'block.unknown-games', label: 'Unknown games' },
        { id: 'block.multiplayer-games', label: 'Multiplayer games' },
        { id: 'block.game-purchases', label: 'Game purchases' },
      ],
    },
  ],
  limit: [
    {
      id: 'limit-game-kind',
      label: 'Kind',
      choices: [
        { id: 'limit.native-games', label: 'Native games' },
        { id: 'limit.browser-games', label: 'Browser games' },
        { id: 'limit.cloud-games', label: 'Cloud games' },
        { id: 'limit.multiplayer-games', label: 'Multiplayer games' },
      ],
    },
    {
      id: 'limit-game-time',
      label: 'Time',
      choices: [
        { id: 'limit.total-game-time', label: 'Total game time' },
        { id: 'limit.weekday-budget', label: 'Weekday budget' },
        { id: 'limit.weekend-budget', label: 'Weekend budget' },
        { id: 'limit.session-length', label: 'Session length' },
      ],
    },
  ],
  ask: [
    {
      id: 'ask-game-requests',
      label: 'Requests',
      choices: [
        { id: 'ask.new-unknown-game', label: 'New/unknown game' },
        { id: 'ask.blocked-game-open', label: 'Blocked game open' },
        { id: 'ask.extra-game-time', label: 'Extra game time' },
        { id: 'ask.online-multiplayer', label: 'Online multiplayer' },
      ],
    },
    {
      id: 'ask-game-risk',
      label: 'Risk',
      choices: [
        { id: 'ask.voice-chat', label: 'Voice/chat' },
        { id: 'ask.game-purchase', label: 'Game purchase' },
        { id: 'ask.launcher-install', label: 'Launcher install' },
      ],
    },
  ],
  allow: [
    {
      id: 'allow-game-safe',
      label: 'Safe',
      choices: [
        { id: 'allow.approved-games', label: 'Approved games' },
        { id: 'allow.educational-games', label: 'Educational games' },
        { id: 'allow.local-offline-games', label: 'Local/offline games' },
        { id: 'allow.family-multiplayer', label: 'Family multiplayer' },
      ],
    },
    {
      id: 'allow-game-launchers',
      label: 'Launchers',
      choices: [
        { id: 'allow.steam', label: 'Steam' },
        { id: 'allow.minecraft', label: 'Minecraft' },
        { id: 'allow.roblox', label: 'Roblox' },
        { id: 'allow.epic-games', label: 'Epic Games' },
      ],
    },
  ],
  observe: [
    {
      id: 'observe-game-reports',
      label: 'Reports',
      choices: [
        { id: 'observe.all', label: 'All' },
        { id: 'observe.game-sessions', label: 'Game sessions' },
        { id: 'observe.launcher-activity', label: 'Launcher activity' },
        { id: 'observe.multiplayer-risk', label: 'Multiplayer risk' },
        { id: 'observe.playtime-summary', label: 'Playtime summary' },
      ],
    },
    {
      id: 'observe-game-privacy',
      label: 'Privacy',
      choices: [
        { id: 'observe.title-summary', label: 'Title summary' },
        { id: 'observe.category-summary', label: 'Category summary' },
        { id: 'observe.exact-game-on-ask', label: 'Exact game on ask/block' },
      ],
    },
  ],
};

const DEFAULT_ACTIVE_GROUP_BY_TONE: Record<WeeklySchedulerActionId, WeeklySchedulerControlGroupId> = {
  block: 'block-common',
  limit: 'limit-content',
  ask: 'ask-requests',
  allow: 'allow-social',
  observe: 'observe-reports',
};

const APP_ACTIVE_GROUP_BY_TONE: Record<WeeklySchedulerActionId, WeeklySchedulerControlGroupId> = {
  block: 'block-apps',
  limit: 'limit-app-category',
  ask: 'ask-app-requests',
  allow: 'allow-app-safe',
  observe: 'observe-app-reports',
};

const GAME_ACTIVE_GROUP_BY_TONE: Record<WeeklySchedulerActionId, WeeklySchedulerControlGroupId> = {
  block: 'block-games',
  limit: 'limit-game-kind',
  ask: 'ask-game-requests',
  allow: 'allow-game-safe',
  observe: 'observe-game-reports',
};

const DEFAULT_TONE_CONTROLS: Record<WeeklySchedulerActionId, readonly WeeklySchedulerControlId[]> = {
  block: ['block.all'],
  limit: ['limit.social-media', 'limit.browser-games', 'limit.video'],
  ask: ['ask.downloads', 'ask.unmanaged-browsers'],
  allow: ['allow.youtube', 'allow.search'],
  observe: ['observe.all'],
};

const APP_TONE_CONTROLS: Record<WeeklySchedulerActionId, readonly WeeklySchedulerControlId[]> = {
  block: ['block.all'],
  limit: ['limit.social-chat-apps', 'limit.video-streaming-apps', 'limit.total-app-time'],
  ask: ['ask.new-unknown-app', 'ask.extra-app-time'],
  allow: ['allow.school-apps', 'allow.productivity-apps'],
  observe: ['observe.all'],
};

const GAME_TONE_CONTROLS: Record<WeeklySchedulerActionId, readonly WeeklySchedulerControlId[]> = {
  block: ['block.all'],
  limit: ['limit.native-games', 'limit.browser-games', 'limit.total-game-time'],
  ask: ['ask.new-unknown-game', 'ask.extra-game-time'],
  allow: ['allow.approved-games', 'allow.educational-games'],
  observe: ['observe.all'],
};

const SCHEDULER_PRESET_CONTROLS = {
  allowAll: [
    'allow.facebook',
    'allow.instagram',
    'allow.tiktok',
    'allow.snapchat',
    'allow.x-twitter',
    'allow.discord',
    'allow.reddit',
    'allow.youtube',
    'allow.twitch',
    'allow.netflix',
    'allow.streaming-sites',
    'allow.roblox',
    'allow.browser-games',
    'allow.game-wikis',
    'allow.game-downloads',
    'allow.search',
    'allow.school-domains',
    'allow.news-reference',
    'allow.downloads',
  ],
  askAll: [
    'ask.new-sites',
    'ask.blocked-sites',
    'ask.downloads',
    'ask.time-extension',
    'ask.unmanaged-browsers',
    'ask.private-incognito',
    'ask.browser-installs',
    'ask.extension-profile',
  ],
  blockAll: ['block.all'],
  limitAll: [
    'limit.social-media',
    'limit.browser-games',
    'limit.video',
    'limit.search',
    'limit.total-browser-time',
    'limit.managed-browser-time',
    'limit.unmanaged-browser-time',
    'limit.downloads',
  ],
  observeAll: ['observe.all'],
} as const satisfies Record<string, readonly WeeklySchedulerControlId[]>;

const SCHEDULER_LIMIT_ALL_BLOCKS = SCHEDULER_DAY_IDS.map((day) =>
  createLimitPresetBlock(
    'limit-all',
    day,
    'All day limit',
    SCHEDULER_PRESET_CONTROLS.limitAll,
    SCHEDULER_CONTROL_GROUPS_BY_TONE,
    () => [{ endHour: 24, startHour: 0, tone: 'limit' }]
  )
);

const SCHEDULER_LIMIT_BALANCED_BLOCKS = SCHEDULER_DAY_IDS.map((day) =>
  createLimitPresetBlock(
    'limit-balanced',
    day,
    'Balanced limit',
    SCHEDULER_PRESET_CONTROLS.limitAll,
    SCHEDULER_CONTROL_GROUPS_BY_TONE,
    (controlId, controlIndex) => limitBalancedWindowsForControl(day, controlIndex)
  )
);

const SCHEDULER_PRESETS: readonly WeeklySchedulerPreset[] = [
  {
    description: 'Implicit observe for every track.',
    id: 'observe-only',
    label: 'Observe only',
    segments: [],
  },
  {
    description: 'Block midnight to morning and late night.',
    id: 'night-block',
    label: 'Night block',
    segments: [
      { controls: SCHEDULER_PRESET_CONTROLS.blockAll, endHour: 8, label: 'Night block', startHour: 0, tone: 'block' },
      { controls: SCHEDULER_PRESET_CONTROLS.blockAll, endHour: 24, label: 'Late block', startHour: 22, tone: 'block' },
    ],
  },
  {
    description: 'Light guardrails with a long allowed window.',
    id: 'relaxed',
    label: 'Relaxed',
    segments: [
      { controls: SCHEDULER_PRESET_CONTROLS.blockAll, endHour: 7, label: 'Night block', startHour: 0, tone: 'block' },
      {
        controls: SCHEDULER_PRESET_CONTROLS.allowAll,
        endHour: 21,
        label: 'Allowed window',
        startHour: 7,
        tone: 'allow',
      },
      { controls: SCHEDULER_PRESET_CONTROLS.askAll, endHour: 23, label: 'Ask parent', startHour: 21, tone: 'ask' },
      {
        controls: SCHEDULER_PRESET_CONTROLS.blockAll,
        endHour: 24,
        label: 'Bedtime block',
        startHour: 23,
        tone: 'block',
      },
    ],
  },
  {
    description: 'Balanced browser controls across school, evening, and bedtime.',
    id: 'balanced',
    label: 'Medium',
    segments: [
      { controls: SCHEDULER_PRESET_CONTROLS.blockAll, endHour: 7.5, label: 'Night block', startHour: 0, tone: 'block' },
      {
        controls: SCHEDULER_PRESET_CONTROLS.limitAll,
        endHour: 16,
        label: 'School limits',
        startHour: 7.5,
        tone: 'limit',
      },
      {
        controls: SCHEDULER_PRESET_CONTROLS.allowAll,
        endHour: 19,
        label: 'Allowed window',
        startHour: 16,
        tone: 'allow',
      },
      { controls: SCHEDULER_PRESET_CONTROLS.askAll, endHour: 21.5, label: 'Ask parent', startHour: 19, tone: 'ask' },
      {
        controls: SCHEDULER_PRESET_CONTROLS.blockAll,
        endHour: 24,
        label: 'Bedtime block',
        startHour: 21.5,
        tone: 'block',
      },
    ],
  },
  {
    description: 'Mostly controlled, small allowed gap, early bedtime.',
    id: 'strict',
    label: 'Strict',
    segments: [
      { controls: SCHEDULER_PRESET_CONTROLS.blockAll, endHour: 8, label: 'Night block', startHour: 0, tone: 'block' },
      {
        controls: SCHEDULER_PRESET_CONTROLS.limitAll,
        endHour: 18,
        label: 'Limited browser',
        startHour: 8,
        tone: 'limit',
      },
      { controls: SCHEDULER_PRESET_CONTROLS.askAll, endHour: 20, label: 'Ask parent', startHour: 18, tone: 'ask' },
      {
        controls: SCHEDULER_PRESET_CONTROLS.allowAll,
        endHour: 20.5,
        label: 'Short allow',
        startHour: 20,
        tone: 'allow',
      },
      {
        controls: SCHEDULER_PRESET_CONTROLS.blockAll,
        endHour: 24,
        label: 'Bedtime block',
        startHour: 20.5,
        tone: 'block',
      },
    ],
  },
  {
    blocks: SCHEDULER_LIMIT_ALL_BLOCKS,
    description: 'One all-day limit clip with every limit track capped.',
    id: 'limit-all',
    label: 'Limit',
    segments: [],
  },
  {
    blocks: SCHEDULER_LIMIT_BALANCED_BLOCKS,
    description: 'All-day limit clips with ask, allow, and capped examples inside each limit track.',
    id: 'limit-balanced',
    label: 'Limit balanced',
    segments: [],
  },
  {
    description: 'One all-day ask-parent clip.',
    id: 'ask-all',
    label: 'Ask all',
    segments: [
      { controls: SCHEDULER_PRESET_CONTROLS.askAll, endHour: 24, label: 'Ask parent', startHour: 0, tone: 'ask' },
    ],
  },
  {
    description: 'One all-day allow clip.',
    id: 'allow-all',
    label: 'Allow all',
    segments: [
      { controls: SCHEDULER_PRESET_CONTROLS.allowAll, endHour: 24, label: 'All allowed', startHour: 0, tone: 'allow' },
    ],
  },
];

const SCHEDULER_APP_PRESET_CONTROLS = presetControlsFromGroups(SCHEDULER_APP_CONTROL_GROUPS_BY_TONE);
const SCHEDULER_GAME_PRESET_CONTROLS = presetControlsFromGroups(SCHEDULER_GAME_CONTROL_GROUPS_BY_TONE);
const SCHEDULER_APP_PRESETS = createSchedulerPresets(
  SCHEDULER_APP_PRESET_CONTROLS,
  SCHEDULER_APP_CONTROL_GROUPS_BY_TONE,
  'app'
);
const SCHEDULER_GAME_PRESETS = createSchedulerPresets(
  SCHEDULER_GAME_PRESET_CONTROLS,
  SCHEDULER_GAME_CONTROL_GROUPS_BY_TONE,
  'game'
);

const SCHEDULER_PROFILE_BY_POLICY_AREA: Record<WeeklySchedulerPolicyArea, WeeklySchedulerProfile> = {
  apps: {
    controlGroupsByTone: SCHEDULER_APP_CONTROL_GROUPS_BY_TONE,
    customPresetStorageKey: 'weekly-scheduler-app-custom-presets-v1',
    defaultActiveGroupByTone: APP_ACTIVE_GROUP_BY_TONE,
    defaultPresetId: DEFAULT_PRESET_ID,
    defaultToneControls: APP_TONE_CONTROLS,
    id: 'apps',
    presets: SCHEDULER_APP_PRESETS,
  },
  browser: {
    controlGroupsByTone: SCHEDULER_CONTROL_GROUPS_BY_TONE,
    customPresetStorageKey: SCHEDULER_CUSTOM_PRESET_STORAGE_KEY,
    defaultActiveGroupByTone: DEFAULT_ACTIVE_GROUP_BY_TONE,
    defaultPresetId: DEFAULT_PRESET_ID,
    defaultToneControls: DEFAULT_TONE_CONTROLS,
    id: 'browser',
    presets: SCHEDULER_PRESETS,
  },
  games: {
    controlGroupsByTone: SCHEDULER_GAME_CONTROL_GROUPS_BY_TONE,
    customPresetStorageKey: 'weekly-scheduler-game-custom-presets-v1',
    defaultActiveGroupByTone: GAME_ACTIVE_GROUP_BY_TONE,
    defaultPresetId: DEFAULT_PRESET_ID,
    defaultToneControls: GAME_TONE_CONTROLS,
    id: 'games',
    presets: SCHEDULER_GAME_PRESETS,
  },
};

export function WeeklySchedulerScratchPage({
  blocks = DEFAULT_BLOCKS,
  embedded = false,
  onScheduleChange,
  policyArea = 'browser',
}: WeeklySchedulerScratchPageProps): ReactElement {
  const profile = schedulerProfileForPolicyArea(policyArea);
  const [savedPresets, setSavedPresets] = useState<readonly WeeklySchedulerPreset[]>(() =>
    savedPresetsFromStorage(profile.customPresetStorageKey)
  );
  const allPresets: readonly WeeklySchedulerPreset[] = useMemo(
    () => [...profile.presets, ...savedPresets],
    [profile.presets, savedPresets]
  );
  const initialBlocks =
    blocks.length > 0 ? blocks : blocksForPreset(profile.defaultPresetId, allPresets, profile.controlGroupsByTone);
  const [mode, setMode] = useState<WeeklySchedulerMode>('weekly');
  const [selectedDays, setSelectedDays] = useState<readonly WeeklySchedulerDay[]>(SCHEDULER_DAY_IDS);
  const [presetScope, setPresetScope] = useState<WeeklySchedulerPresetScope>('week');
  const [selectedPresetDay, setSelectedPresetDay] = useState<WeeklySchedulerDay>(SCHEDULER_DEFAULT_DAY);
  const [dayMenuOpen, setDayMenuOpen] = useState(false);
  const [presetMenuOpen, setPresetMenuOpen] = useState(false);
  const [closingMenu, setClosingMenu] = useState<WeeklySchedulerMenuId | null>(null);
  const [activePresetId, setActivePresetId] = useState<WeeklySchedulerPresetId | null>(
    blocks.length > 0 ? null : profile.defaultPresetId
  );
  const [activeTone, setActiveTone] = useState<WeeklySchedulerActionId>('allow');
  const [openTone, setOpenTone] = useState<WeeklySchedulerActionId | null>(null);
  const [activeGroupByTone, setActiveGroupByTone] = useState<
    Record<WeeklySchedulerActionId, WeeklySchedulerControlGroupId>
  >(profile.defaultActiveGroupByTone);
  const [toneControls, setToneControls] = useState<
    Record<WeeklySchedulerActionId, readonly WeeklySchedulerControlId[]>
  >(profile.defaultToneControls);
  const [placedBlocks, setPlacedBlocks] = useState<readonly WeeklySchedulerBlock[]>(initialBlocks);
  const [navigatorWindow, setNavigatorWindow] = useState<WeeklySchedulerNavigatorState>({
    endHour: 24,
    startHour: 0,
  });
  const fitRows = true;
  const [viewportWidth, setViewportWidth] = useState(0);
  const [dragTone, setDragTone] = useState<WeeklySchedulerActionId | null>(null);
  const [dragPreview, setDragPreview] = useState<{
    readonly day: WeeklySchedulerDay;
    readonly startHour: number;
    readonly tone: WeeklySchedulerActionId;
    readonly valid: boolean;
  } | null>(null);
  const [subClipDragPreview, setSubClipDragPreview] = useState<{
    readonly blockId: string;
    readonly controlId: WeeklySchedulerControlId;
    readonly startHour: number;
    readonly tone: WeeklySchedulerActionId;
  } | null>(null);
  const [dayDropTarget, setDayDropTarget] = useState<WeeklySchedulerDay | null>(null);
  const [selectedBlockId, setSelectedBlockId] = useState<string | null>(null);
  const [timeEditorTarget, setTimeEditorTarget] = useState<WeeklySchedulerTimeEditorTarget | null>(null);
  const [timeEditorDraft, setTimeEditorDraft] = useState<WeeklySchedulerTimeEditorDraft>(() =>
    timeEditorDraftFromRange(0, SCHEDULER_MIN_BLOCK_DURATION)
  );
  const [movingBlockId, setMovingBlockId] = useState<string | null>(null);
  const [subtracksHidden, setSubtracksHidden] = useState(false);
  const [fullscreen, setFullscreen] = useState(false);
  const [soloBlockId, setSoloBlockId] = useState<string | null>(null);
  const [collapsedBlockIds, setCollapsedBlockIds] = useState<readonly string[]>(() =>
    collapsedBlockIdsForBlocks(initialBlocks)
  );
  const [collapsedSubClipIds, setCollapsedSubClipIds] = useState<readonly string[]>(() =>
    collapsedSubClipIdsForBlocks(initialBlocks)
  );
  const [collapsedDayIds, setCollapsedDayIds] = useState<readonly WeeklySchedulerDay[]>([]);
  const hoveredMenuRef = useRef<WeeklySchedulerMenuId | null>(null);
  const menuCloseTimerRef = useRef<number | null>(null);
  const menuClosingTimerRef = useRef<number | null>(null);
  const lastClipTapRef = useRef<{
    readonly blockId: string;
    readonly time: number;
  } | null>(null);
  const dropHandledRef = useRef(false);
  const viewportRef = useRef<HTMLDivElement | null>(null);
  const fullscreenAnchorRef = useRef<HTMLElement | null>(null);
  const [fullscreenBounds, setFullscreenBounds] = useState<WeeklySchedulerFullscreenBounds | null>(null);
  const navigatorDragStateRef = useRef<{
    readonly edge: 'move' | 'start' | 'end';
    readonly originEndHour: number;
    readonly originStartHour: number;
    readonly pointerId: number;
    readonly railLeft: number;
    readonly railWidth: number;
    readonly x: number;
  } | null>(null);
  const resizeStateRef = useRef<{
    readonly blockId: string;
    readonly edge: 'start' | 'end';
    readonly hourWidth: number;
    readonly originEndHour: number;
    readonly originStartHour: number;
    readonly pointerId: number;
    readonly x: number;
  } | null>(null);
  const subClipEditStateRef = useRef<{
    readonly blockEndHour: number;
    readonly blockId: string;
    readonly blockStartHour: number;
    readonly edge: 'move' | 'start' | 'end';
    readonly hourWidth: number;
    readonly originEndHour: number;
    readonly originStartHour: number;
    readonly pointerId: number;
    readonly subClipId: string;
    readonly x: number;
  } | null>(null);
  const moveStateRef = useRef<{
    readonly blockId: string;
    readonly duration: number;
    readonly hourWidth: number;
    readonly originDay: WeeklySchedulerDay;
    readonly originStartHour: number;
    readonly pointerId: number;
    readonly x: number;
  } | null>(null);

  useEffect(() => {
    const nextSavedPresets = savedPresetsFromStorage(profile.customPresetStorageKey);
    const nextAllPresets = [...profile.presets, ...nextSavedPresets];
    const nextBlocks =
      blocks.length > 0
        ? blocks
        : blocksForPreset(profile.defaultPresetId, nextAllPresets, profile.controlGroupsByTone);
    setSavedPresets(nextSavedPresets);
    setActivePresetId(blocks.length > 0 ? null : profile.defaultPresetId);
    setActiveGroupByTone(profile.defaultActiveGroupByTone);
    setToneControls(profile.defaultToneControls);
    setPlacedBlocks(nextBlocks);
    setSelectedBlockId(null);
    setSoloBlockId(null);
    setCollapsedBlockIds(collapsedBlockIdsForBlocks(nextBlocks));
    setCollapsedSubClipIds(collapsedSubClipIdsForBlocks(nextBlocks));
    setCollapsedDayIds([]);
  }, [blocks, profile]);

  const scheduleBlocks = blocks.length > 0 ? blocks : placedBlocks;
  const soloBlock = soloBlockId === null ? null : (scheduleBlocks.find((block) => block.id === soloBlockId) ?? null);
  const visibleDays =
    soloBlock !== null
      ? SCHEDULER_DAYS.filter((day) => day.id === soloBlock.day)
      : mode === 'weekly'
        ? SCHEDULER_DAYS
        : SCHEDULER_DAYS.filter((day) => selectedDays.includes(day.id));
  const allVisibleDaysCollapsed =
    visibleDays.length > 0 && visibleDays.every((day) => collapsedDayIds.includes(day.id));
  const navigatorWindowDuration = navigatorWindow.endHour - navigatorWindow.startHour;
  const minimumHourWidth = navigatorWindowDuration >= 24 ? SCHEDULER_FIT_MIN_HOUR_WIDTH : SCHEDULER_MIN_HOUR_WIDTH;
  const hourWidth = Math.max(
    minimumHourWidth,
    Math.floor((Math.max(0, viewportWidth) - SCHEDULER_DAY_LABEL_WIDTH) / navigatorWindowDuration)
  );
  const compactTimelineControlsHidden = hourWidth < 16;
  const clipControlsHidden = soloBlock !== null ? false : compactTimelineControlsHidden;
  const timelineTranslateX = -navigatorWindow.startHour * hourWidth;
  const selectedBlock = scheduleBlocks.find((block) => block.id === selectedBlockId) ?? null;
  const activePreset = activePresetId === null ? null : presetById(activePresetId, allPresets);
  const scheduleEdl = scheduleBlocksToEdl(scheduleBlocks, profile.controlGroupsByTone);
  const scheduleEdlJson = JSON.stringify(scheduleEdl);
  const selectedClipFitWindow =
    selectedBlock === null ? null : timelineWindowForClip(selectedBlock.startHour, selectedBlock.endHour);
  const selectedClipIsFit =
    selectedClipFitWindow !== null &&
    Math.abs(navigatorWindow.startHour - selectedClipFitWindow.startHour) < SCHEDULER_SNAP_HOURS &&
    Math.abs(navigatorWindow.endHour - selectedClipFitWindow.endHour) < SCHEDULER_SNAP_HOURS;
  const openMenuId: WeeklySchedulerMenuId | null = presetMenuOpen
    ? 'preset'
    : dayMenuOpen
      ? 'day'
      : openTone === null
        ? null
        : toneMenuId(openTone);
  const qaWidth = qaWidthFromLocation();
  const qaPageStyle =
    qaWidth === null
      ? undefined
      : ({
          maxWidth: `${qaWidth}px`,
          minHeight: `${SCHEDULER_QA_MIN_HEIGHT}px`,
          minWidth: `${qaWidth}px`,
          width: `${qaWidth}px`,
        } satisfies CSSProperties);
  const embeddedPageStyle =
    embedded && qaPageStyle === undefined
      ? ({
          height: '100%',
          minHeight: '0',
          width: '100%',
        } satisfies CSSProperties)
      : undefined;
  const fullscreenStyle =
    embedded && fullscreen && fullscreenBounds !== null
      ? ({
          '--weekly-scheduler-fullscreen-bottom': `${fullscreenBounds.bottom}px`,
          '--weekly-scheduler-fullscreen-top': `${fullscreenBounds.top}px`,
        } satisfies WeeklySchedulerCssVars)
      : undefined;
  const pageStyle =
    qaPageStyle ??
    (embeddedPageStyle === undefined && fullscreenStyle === undefined
      ? undefined
      : ({
          ...(fullscreen ? undefined : embeddedPageStyle),
          ...fullscreenStyle,
        } satisfies CSSProperties));
  const schedulerStyle: WeeklySchedulerCssVars = {
    '--scheduler-day-count': `${visibleDays.length}`,
    '--scheduler-grid-height': `${SCHEDULER_BASE_ROW_HEIGHT * visibleDays.length}px`,
    '--scheduler-hour-width': `${hourWidth}px`,
    '--scheduler-hours-width': `${hourWidth * SCHEDULER_HOURS.length}px`,
    '--scheduler-timeline-offset-x': `${timelineTranslateX}px`,
    '--scheduler-row-height': `${SCHEDULER_BASE_ROW_HEIGHT}px`,
    '--scheduler-subclip-count': `${selectedBlock === null ? 1 : subClipsForBlock(selectedBlock).length}`,
  };

  const setTimelineWindow = (nextWindow: WeeklySchedulerNavigatorState) => {
    setNavigatorWindow(normalizeNavigatorWindow(nextWindow));
  };

  const markScheduleCustom = () => setActivePresetId(null);

  const blockLabelForControls = (selectedControls: readonly WeeklySchedulerControlId[]) =>
    blockLabel(selectedControls, profile.controlGroupsByTone);

  const createScheduledBlock = (
    id: string,
    day: WeeklySchedulerDay,
    startHour: number,
    tone: WeeklySchedulerActionId,
    label: string,
    selectedControls: readonly WeeklySchedulerControlId[] = []
  ) => createBlock(id, day, startHour, tone, label, selectedControls, profile.controlGroupsByTone);

  const createScheduledBlockWithRange = (
    id: string,
    day: WeeklySchedulerDay,
    startHour: number,
    endHour: number,
    tone: WeeklySchedulerActionId,
    label: string,
    selectedControls: readonly WeeklySchedulerControlId[] = []
  ) => createBlockWithRange(id, day, startHour, endHour, tone, label, selectedControls, profile.controlGroupsByTone);

  const clearMenuCloseTimer = () => {
    if (menuCloseTimerRef.current !== null) {
      window.clearTimeout(menuCloseTimerRef.current);
      menuCloseTimerRef.current = null;
    }
  };

  const clearMenuClosingTimer = () => {
    if (menuClosingTimerRef.current !== null) {
      window.clearTimeout(menuClosingTimerRef.current);
      menuClosingTimerRef.current = null;
    }
  };

  const finishMenuCloseAnimation = (menuId: WeeklySchedulerMenuId) => {
    clearMenuClosingTimer();
    menuClosingTimerRef.current = window.setTimeout(() => {
      setClosingMenu((current) => (current === menuId ? null : current));
      menuClosingTimerRef.current = null;
    }, SCHEDULER_MENU_CLOSE_ANIMATION_MS);
  };

  const closeMenuAnimated = (menuId: WeeklySchedulerMenuId) => {
    clearMenuCloseTimer();
    if (hoveredMenuRef.current === menuId) {
      hoveredMenuRef.current = null;
    }
    setClosingMenu(menuId);
    if (menuId === 'preset') {
      setPresetMenuOpen(false);
    } else if (menuId === 'day') {
      setDayMenuOpen(false);
    } else {
      const tone = toneFromMenuId(menuId);
      if (tone !== null) {
        setOpenTone((current) => (current === tone ? null : current));
      }
    }
    finishMenuCloseAnimation(menuId);
  };

  const scheduleMenuClose = (menuId: WeeklySchedulerMenuId) => {
    clearMenuCloseTimer();
    menuCloseTimerRef.current = window.setTimeout(() => {
      if (hoveredMenuRef.current !== menuId) {
        closeMenuAnimated(menuId);
      }
    }, SCHEDULER_MENU_IDLE_CLOSE_MS);
  };

  const handleMenuMouseEnter = (menuId: WeeklySchedulerMenuId) => {
    hoveredMenuRef.current = menuId;
    clearMenuCloseTimer();
  };

  const handleMenuMouseLeave = (menuId: WeeklySchedulerMenuId) => {
    if (hoveredMenuRef.current === menuId) {
      hoveredMenuRef.current = null;
    }
    scheduleMenuClose(menuId);
  };

  const openPresetMenu = () => {
    clearMenuCloseTimer();
    hoveredMenuRef.current = null;
    setClosingMenu(null);
    setPresetMenuOpen(true);
    setOpenTone(null);
    setDayMenuOpen(false);
  };

  const openDayMenu = () => {
    clearMenuCloseTimer();
    hoveredMenuRef.current = null;
    setClosingMenu(null);
    setDayMenuOpen(true);
    setOpenTone(null);
    setPresetMenuOpen(false);
  };

  const openToneMenu = (tone: WeeklySchedulerActionId) => {
    clearMenuCloseTimer();
    hoveredMenuRef.current = null;
    setClosingMenu(null);
    setOpenTone(tone);
    setDayMenuOpen(false);
    setPresetMenuOpen(false);
  };

  const togglePresetMenu = () => {
    if (presetMenuOpen) {
      closeMenuAnimated('preset');
      return;
    }
    openPresetMenu();
  };

  const toggleDayMenu = () => {
    if (dayMenuOpen) {
      closeMenuAnimated('day');
      return;
    }
    openDayMenu();
  };

  const toggleToneMenu = (tone: WeeklySchedulerActionId) => {
    if (openTone === tone) {
      closeMenuAnimated(toneMenuId(tone));
      return;
    }
    openToneMenu(tone);
  };

  const applyPreset = (presetId: WeeklySchedulerPresetId) => {
    const targetDays = presetScope === 'week' ? SCHEDULER_DAY_IDS : [selectedPresetDay];
    const presetBlocks = blocksForPresetDays(presetId, targetDays, allPresets, profile.controlGroupsByTone);
    const nextBlocks =
      presetScope === 'week' ? presetBlocks : replaceBlocksForDays(scheduleBlocks, targetDays, presetBlocks);
    setActivePresetId(presetScope === 'week' ? presetId : null);
    setPlacedBlocks(nextBlocks);
    setSelectedBlockId(null);
    setSoloBlockId(null);
    setCollapsedBlockIds(collapsedBlockIdsForBlocks(nextBlocks));
    setCollapsedSubClipIds(collapsedSubClipIdsForBlocks(nextBlocks));
    setCollapsedDayIds((current) => (presetScope === 'week' ? [] : current.filter((day) => !targetDays.includes(day))));
    if (presetScope === 'week') {
      setMode('weekly');
      setSelectedDays(SCHEDULER_DAY_IDS);
    } else {
      setSelectedPresetDay(targetDays[0] ?? SCHEDULER_DEFAULT_DAY);
    }
    setDayMenuOpen(false);
    setOpenTone(null);
    closeMenuAnimated('preset');
    if (presetScope === 'week') {
      setTimelineWindow({ endHour: 24, startHour: 0 });
    }
  };

  const saveCurrentProfile = () => {
    const nextPreset: WeeklySchedulerPreset = {
      blocks: scheduleBlocks.map(cloneBlock),
      description: 'Saved from current schedule.',
      id: `profile-${Date.now()}`,
      label: `Profile ${savedPresets.length + 1}`,
      segments: [],
    };
    setSavedPresets((current) => {
      const nextPresets = [...current, nextPreset];
      savePresetsToStorage(profile.customPresetStorageKey, nextPresets);
      return nextPresets;
    });
    setActivePresetId(nextPreset.id);
    closeMenuAnimated('preset');
  };

  const handleFitTimelineWindow = () => {
    if (selectedBlock === null || selectedClipIsFit) {
      setTimelineWindow({ endHour: 24, startHour: 0 });
      return;
    }
    setTimelineWindow(timelineWindowForClip(selectedBlock.startHour, selectedBlock.endHour));
  };

  const enterClipSolo = (block: WeeklySchedulerBlock) => {
    setSoloBlockId(block.id);
    setSelectedBlockId(block.id);
    setMode('daily');
    setSelectedDays([block.day]);
    setDayMenuOpen(false);
    setOpenTone(null);
    setPresetMenuOpen(false);
    setSubtracksHidden(false);
    setCollapsedDayIds((current) => current.filter((day) => day !== block.day));
    setCollapsedBlockIds((current) => current.filter((blockId) => blockId !== block.id));
    const expandedSubTrackIds = new Set(collapsedSubClipIdsForBlock(block));
    setCollapsedSubClipIds((current) => current.filter((subClipId) => !expandedSubTrackIds.has(subClipId)));
    setTimelineWindow(timelineWindowForClip(block.startHour, block.endHour));
  };

  const exitClipSolo = () => {
    setSoloBlockId(null);
    setSelectedBlockId(null);
    setMode('weekly');
    setSelectedDays(SCHEDULER_DAY_IDS);
    setCollapsedDayIds([]);
    setCollapsedBlockIds(collapsedBlockIdsForBlocks(scheduleBlocks));
    setCollapsedSubClipIds(collapsedSubClipIdsForBlocks(scheduleBlocks));
    setTimelineWindow({ endHour: 24, startHour: 0 });
  };

  const handleBlockSolo = (block: WeeklySchedulerBlock, event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    if (soloBlockId === block.id) {
      exitClipSolo();
      return;
    }
    enterClipSolo(block);
  };

  const handleBlockDelete = (block: WeeklySchedulerBlock, event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    markScheduleCustom();
    setPlacedBlocks((current) => current.filter((item) => item.id !== block.id));
    setSelectedBlockId((current) => (current === block.id ? null : current));
    setTimeEditorTarget((current) => (current?.blockId === block.id ? null : current));
    setCollapsedBlockIds((current) => current.filter((blockId) => blockId !== block.id));
    setCollapsedSubClipIds((current) => current.filter((subClipId) => !subClipId.startsWith(`${block.id}-`)));
    if (soloBlockId === block.id) {
      setSoloBlockId(null);
      setMode('weekly');
      setSelectedDays(SCHEDULER_DAY_IDS);
      setTimelineWindow({ endHour: 24, startHour: 0 });
    }
  };

  const handleBlockClick = (block: WeeklySchedulerBlock, event: ReactMouseEvent<HTMLDivElement>) => {
    event.stopPropagation();
    const lastTap = lastClipTapRef.current;
    const now = window.performance.now();
    if (lastTap !== null && lastTap.blockId === block.id && now - lastTap.time < 340) {
      lastClipTapRef.current = null;
      enterClipSolo(block);
      return;
    }
    lastClipTapRef.current = { blockId: block.id, time: now };
    setSelectedBlockId(block.id);
  };

  const placeBlock = (day: WeeklySchedulerDay, startHour: number, tone: WeeklySchedulerActionId) => {
    markScheduleCustom();
    const id = blockIdForPlacement(day, startHour, tone);
    const label = blockLabelForControls(toneControls[tone]);
    const nextBlock = createScheduledBlock(id, day, startHour, tone, label, toneControls[tone]);
    const nextBlocks = mergeClipIntoBlocks(scheduleBlocks, nextBlock);
    if (nextBlocks === null) {
      return;
    }
    setActiveTone(tone);
    const selectedId = selectedIdForMergedClip(nextBlocks, nextBlock);
    const selectedNextBlock = nextBlocks.find((block) => block.id === selectedId);
    setSoloBlockId(null);
    setSelectedBlockId(selectedId);
    if (selectedNextBlock !== undefined) {
      const nextCollapsedSubClipIds = collapsedSubClipIdsForBlock(selectedNextBlock);
      setCollapsedBlockIds((current) => Array.from(new Set([...current, selectedNextBlock.id])));
      setCollapsedSubClipIds((current) => Array.from(new Set([...current, ...nextCollapsedSubClipIds])));
    }
    setPlacedBlocks((current) => mergeClipIntoBlocks(current, nextBlock) ?? current);
  };

  const selectPresetDay = (day: WeeklySchedulerDay) => {
    setPresetScope('selected-day');
    setSelectedPresetDay(day);
    setSelectedBlockId(null);
  };

  const applyToneToWholeDay = (day: WeeklySchedulerDay, tone: WeeklySchedulerActionId) => {
    markScheduleCustom();
    setActiveTone(tone);
    setPresetScope('selected-day');
    setSelectedPresetDay(day);
    const nextDayBlocks =
      tone === 'observe'
        ? []
        : [
            createScheduledBlockWithRange(
              blockIdForPlacement(day, 0, tone),
              day,
              0,
              24,
              tone,
              blockLabelForControls(toneControls[tone]),
              toneControls[tone]
            ),
          ];
    const nextBlocks = replaceBlocksForDays(scheduleBlocks, [day], nextDayBlocks);
    setPlacedBlocks(nextBlocks);
    setSelectedBlockId(nextDayBlocks[0]?.id ?? null);
    setSoloBlockId(null);
    setCollapsedDayIds((current) => current.filter((item) => item !== day));
    setCollapsedBlockIds((current) => current.filter((blockId) => nextBlocks.some((block) => block.id === blockId)));
    setCollapsedSubClipIds(collapsedSubClipIdsForBlocks(nextBlocks));
  };

  const handleLegendDragStart = (tone: WeeklySchedulerActionId, event: DragEvent<HTMLElement>) => {
    dropHandledRef.current = false;
    setActiveTone(tone);
    setDragTone(tone);
    event.dataTransfer.effectAllowed = 'copy';
    event.dataTransfer.setData(SCHEDULER_DRAG_FORMATS.actionId, tone);
    event.dataTransfer.setData(SCHEDULER_DRAG_FORMATS.plainText, tone);
  };

  const handleLegendDragEnd = (tone: WeeklySchedulerActionId, event: DragEvent<HTMLElement>) => {
    if (!dropHandledRef.current) {
      placeBlockFromPoint(event.clientX, event.clientY, tone);
    }
    dropHandledRef.current = false;
    setDragTone(null);
    setDragPreview(null);
    setSubClipDragPreview(null);
  };

  const placeBlockFromPoint = (clientX: number, clientY: number, tone: WeeklySchedulerActionId) => {
    if (clientX <= 0 && clientY <= 0) {
      return;
    }
    const elements = document.elementsFromPoint(clientX, clientY);
    const blockElement = elements
      .map((element) => element.closest(classSelector(SCHEDULER_CLASS_NAMES.block)))
      .find((element): element is HTMLElement => element instanceof HTMLElement);
    if (blockElement !== undefined) {
      const block = scheduleBlocks.find((item) => item.id === blockElement.dataset['blockId']);
      if (block !== undefined) {
        const startHour = adjacentDropStartHour(block, clientX, blockElement);
        if (
          canInsertClip(
            scheduleBlocks,
            createScheduledBlock(
              'preview',
              block.day,
              startHour,
              tone,
              blockLabelForControls(toneControls[tone]),
              toneControls[tone]
            )
          )
        ) {
          placeBlock(block.day, startHour, tone);
        }
      }
      return;
    }
    const trackElement = elements.find(
      (element): element is HTMLElement =>
        element instanceof HTMLElement && element.dataset['timelineDay'] !== undefined
    );
    const trackDay = trackElement?.dataset['timelineDay'];
    if (trackElement === undefined || trackDay === undefined || !isSchedulerDay(trackDay)) {
      return;
    }
    const startHour = timelineHourFromClientX(trackElement, clientX, hourWidth);
    if (
      canInsertClip(
        scheduleBlocks,
        createScheduledBlock(
          'preview',
          trackDay,
          startHour,
          tone,
          blockLabelForControls(toneControls[tone]),
          toneControls[tone]
        )
      )
    ) {
      placeBlock(trackDay, startHour, tone);
    }
  };

  const handleTrackDragOver = (event: DragEvent<HTMLDivElement>, day: WeeklySchedulerDay) => {
    event.preventDefault();
    const startHour = timelineHourFromClientX(event.currentTarget, event.clientX, hourWidth);
    const tone = dragTone ?? activeTone;
    const valid = canInsertClip(
      scheduleBlocks,
      createScheduledBlock(
        'preview',
        day,
        startHour,
        tone,
        blockLabelForControls(toneControls[tone]),
        toneControls[tone]
      )
    );
    event.dataTransfer.dropEffect = valid ? 'copy' : 'none';
    setDragPreview({
      day,
      startHour,
      tone,
      valid,
    });
  };

  const handleTrackDragLeave = (event: DragEvent<HTMLDivElement>) => {
    if (event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget)) {
      return;
    }
    setDragPreview(null);
  };

  const handleTrackDrop = (event: DragEvent<HTMLDivElement>, day: WeeklySchedulerDay) => {
    dropHandledRef.current = true;
    const tone = droppedTone(event) ?? activeTone;
    const startHour = timelineHourFromClientX(event.currentTarget, event.clientX, hourWidth);
    setDragTone(null);
    setDragPreview(null);
    setSubClipDragPreview(null);
    if (
      canInsertClip(
        scheduleBlocks,
        createScheduledBlock(
          'preview',
          day,
          startHour,
          tone,
          blockLabelForControls(toneControls[tone]),
          toneControls[tone]
        )
      )
    ) {
      placeBlock(day, startHour, tone);
    }
  };

  const handleDayApplyDragOver = (event: DragEvent<HTMLDivElement>, day: WeeklySchedulerDay) => {
    event.preventDefault();
    event.stopPropagation();
    event.dataTransfer.dropEffect = 'copy';
    setDayDropTarget(day);
    setDragPreview(null);
  };

  const handleDayApplyDragLeave = (event: DragEvent<HTMLDivElement>) => {
    if (event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget)) {
      return;
    }
    setDayDropTarget(null);
  };

  const handleDayApplyDrop = (event: DragEvent<HTMLDivElement>, day: WeeklySchedulerDay) => {
    dropHandledRef.current = true;
    event.preventDefault();
    event.stopPropagation();
    const tone = droppedTone(event) ?? activeTone;
    setDragTone(null);
    setDragPreview(null);
    setSubClipDragPreview(null);
    setDayDropTarget(null);
    applyToneToWholeDay(day, tone);
  };

  const handleClipDragOver = (event: DragEvent<HTMLDivElement>, block: WeeklySchedulerBlock) => {
    event.preventDefault();
    event.stopPropagation();
    const tone = dragTone ?? activeTone;
    const startHour = adjacentDropStartHour(block, event.clientX, event.currentTarget);
    const valid = canInsertClip(
      scheduleBlocks,
      createScheduledBlock(
        'preview',
        block.day,
        startHour,
        tone,
        blockLabelForControls(toneControls[tone]),
        toneControls[tone]
      )
    );
    event.dataTransfer.dropEffect = valid ? 'copy' : 'none';
    setDragPreview({
      day: block.day,
      startHour,
      tone,
      valid,
    });
  };

  const handleClipDrop = (event: DragEvent<HTMLDivElement>, block: WeeklySchedulerBlock) => {
    dropHandledRef.current = true;
    const tone = droppedTone(event) ?? activeTone;
    const startHour = adjacentDropStartHour(block, event.clientX, event.currentTarget);
    event.stopPropagation();
    setDragTone(null);
    setDragPreview(null);
    setSubClipDragPreview(null);
    if (
      canInsertClip(
        scheduleBlocks,
        createScheduledBlock(
          'preview',
          block.day,
          startHour,
          tone,
          blockLabelForControls(toneControls[tone]),
          toneControls[tone]
        )
      )
    ) {
      placeBlock(block.day, startHour, tone);
    }
  };

  const openBlockTimeEditor = (block: WeeklySchedulerBlock, event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    setSelectedBlockId(block.id);
    setTimeEditorTarget({ blockId: block.id, kind: 'block' });
    setTimeEditorDraft(timeEditorDraftFromRange(block.startHour, block.endHour));
  };

  const openSubClipTimeEditor = (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
    event.stopPropagation();
    if (subClip.disabled === true) {
      return;
    }
    setSelectedBlockId(block.id);
    setTimeEditorTarget({ blockId: block.id, kind: 'subclip', subClipId: subClip.id });
    setTimeEditorDraft(timeEditorDraftFromRange(subClip.startHour, subClip.endHour));
  };

  const updateTimeEditorDraft = (
    edge: WeeklySchedulerTimeEditorEdge,
    part: WeeklySchedulerTimeEditorPart,
    value: string
  ) => {
    setTimeEditorDraft((current) => timeEditorDraftWithPart(current, edge, part, value));
  };

  const applyTimeEditorDraft = () => {
    if (timeEditorTarget === null) {
      return;
    }
    const { endHour, startHour } = timeEditorRangeFromDraft(timeEditorDraft);
    const activeBlock = scheduleBlocks.find((block) => block.id === timeEditorTarget.blockId);
    if (activeBlock === undefined) {
      setTimeEditorTarget(null);
      return;
    }
    if (timeEditorTarget.kind === 'subclip') {
      const activeSubClip = subClipsForBlock(activeBlock).find((subClip) => subClip.id === timeEditorTarget.subClipId);
      if (activeSubClip === undefined) {
        setTimeEditorTarget(null);
        return;
      }
      const nextSubClip = subClipWithEditedTimeRange(activeBlock, activeSubClip, startHour, endHour);
      const nextBlock = {
        ...activeBlock,
        subClips: subClipsForBlock(activeBlock).map((subClip) =>
          subClip.id === activeSubClip.id ? nextSubClip : subClip
        ),
      };
      markScheduleCustom();
      setPlacedBlocks(scheduleBlocks.map((block) => (block.id === activeBlock.id ? nextBlock : block)));
      setSelectedBlockId(activeBlock.id);
      setTimeEditorTarget(null);
      setTimeEditorDraft(timeEditorDraftFromRange(nextSubClip.startHour, nextSubClip.endHour));
      return;
    }
    const nextBlock = blockWithEditedTimeRange(scheduleBlocks, activeBlock, startHour, endHour);
    const nextBlocks = mergeCompatibleBlocks(
      scheduleBlocks.map((block) => (block.id === activeBlock.id ? nextBlock : block))
    );
    const nextSelectedBlockId = selectedIdForMergedClip(nextBlocks, nextBlock);
    markScheduleCustom();
    setPlacedBlocks(nextBlocks);
    setSelectedBlockId(nextSelectedBlockId);
    setTimeEditorTarget(null);
    const selectedNextBlock = nextBlocks.find((block) => block.id === nextSelectedBlockId) ?? nextBlock;
    setTimeEditorDraft(timeEditorDraftFromRange(selectedNextBlock.startHour, selectedNextBlock.endHour));
  };

  const handleResizePointerDown = (
    block: WeeklySchedulerBlock,
    edge: 'start' | 'end',
    event: ReactPointerEvent<HTMLElement>
  ) => {
    event.preventDefault();
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    markScheduleCustom();
    setSelectedBlockId(block.id);
    resizeStateRef.current = {
      blockId: block.id,
      edge,
      hourWidth,
      originEndHour: block.endHour,
      originStartHour: block.startHour,
      pointerId: event.pointerId,
      x: event.clientX,
    };
  };

  const handleResizePointerMove = (event: ReactPointerEvent<HTMLElement>) => {
    const resizeState = resizeStateRef.current;
    if (resizeState === null || resizeState.pointerId !== event.pointerId) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const hourDelta = snapHour((event.clientX - resizeState.x) / resizeState.hourWidth, -24, 24);
    setPlacedBlocks((current) =>
      mergeCompatibleBlocks(
        resizeBlocksWithBoundaryTransfer(
          current,
          resizeState.blockId,
          resizeState.edge,
          resizeState.edge === 'start' ? resizeState.originStartHour + hourDelta : resizeState.originEndHour + hourDelta
        )
      )
    );
  };

  const handleResizePointerEnd = (event: ReactPointerEvent<HTMLElement>) => {
    const resizeState = resizeStateRef.current;
    if (resizeState === null || resizeState.pointerId !== event.pointerId) {
      return;
    }
    resizeStateRef.current = null;
    event.currentTarget.releasePointerCapture(event.pointerId);
  };

  const handleSubClipPointerDown = (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    edge: 'move' | 'start' | 'end',
    event: ReactPointerEvent<HTMLElement>
  ) => {
    if (
      subClip.disabled === true ||
      (edge === 'move' &&
        event.target instanceof Element &&
        event.target.closest(classSelector(SCHEDULER_CLASS_NAMES.subClipGrip)) === null)
    ) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    markScheduleCustom();
    setSelectedBlockId(block.id);
    subClipEditStateRef.current = {
      blockEndHour: block.endHour,
      blockId: block.id,
      blockStartHour: block.startHour,
      edge,
      hourWidth,
      originEndHour: subClip.endHour,
      originStartHour: subClip.startHour,
      pointerId: event.pointerId,
      subClipId: subClip.id,
      x: event.clientX,
    };
  };

  const handleSubClipPointerMove = (event: ReactPointerEvent<HTMLElement>) => {
    const editState = subClipEditStateRef.current;
    if (editState === null || editState.pointerId !== event.pointerId) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const snapDelta = snapHour((event.clientX - editState.x) / editState.hourWidth, -24, 24);
    setPlacedBlocks((current) =>
      current.map((block) => {
        if (block.id !== editState.blockId) {
          return block;
        }
        return fitSubClipsToParent({
          ...block,
          subClips: subClipsForBlock(block).map((subClip) => {
            if (subClip.id !== editState.subClipId) {
              return subClip;
            }
            if (editState.edge === 'move') {
              const duration = editState.originEndHour - editState.originStartHour;
              const startHour = snapHour(
                editState.originStartHour + snapDelta,
                editState.blockStartHour,
                editState.blockEndHour - duration
              );
              return { ...subClip, endHour: startHour + duration, startHour };
            }
            if (editState.edge === 'start') {
              return {
                ...subClip,
                startHour: snapHour(
                  editState.originStartHour + snapDelta,
                  editState.blockStartHour,
                  editState.originEndHour - SCHEDULER_SNAP_HOURS
                ),
              };
            }
            return {
              ...subClip,
              endHour: snapHour(
                editState.originEndHour + snapDelta,
                editState.originStartHour + SCHEDULER_SNAP_HOURS,
                editState.blockEndHour
              ),
            };
          }),
        });
      })
    );
  };

  const handleSubClipPointerEnd = (event: ReactPointerEvent<HTMLElement>) => {
    const editState = subClipEditStateRef.current;
    if (editState === null || editState.pointerId !== event.pointerId) {
      return;
    }
    subClipEditStateRef.current = null;
    event.currentTarget.releasePointerCapture(event.pointerId);
  };

  const handleSubClipToggle = (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
    event.stopPropagation();
    markScheduleCustom();
    setSelectedBlockId(block.id);
    setPlacedBlocks((current) =>
      current.map((item) => {
        if (item.id !== block.id) {
          return item;
        }
        return {
          ...item,
          subClips:
            subClip.disabled === true
              ? [
                  ...subClipsForBlock(item).filter((candidate) => candidate.controlId !== subClip.controlId),
                  {
                    ...subClip,
                    disabled: false,
                    endHour: item.endHour,
                    id: `${item.id}-${stableDomId(subClip.controlId)}-enabled`,
                    startHour: item.startHour,
                    tone: defaultSubClipTone(item.tone),
                  },
                ]
              : subClipsForBlock(item).map((candidate) =>
                  candidate.controlId === subClip.controlId ? { ...candidate, disabled: true } : candidate
                ),
        };
      })
    );
  };

  const handleSubTrackDragOver = (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: DragEvent<HTMLElement>
  ) => {
    if (block.tone !== 'limit') {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const tone = dragTone ?? activeTone;
    event.dataTransfer.dropEffect = 'copy';
    setDragPreview(null);
    setSubClipDragPreview({
      blockId: block.id,
      controlId: subClip.controlId,
      startHour: subTrackHourFromClientX(block, event.currentTarget, event.clientX),
      tone,
    });
  };

  const handleSubTrackDragLeave = (event: DragEvent<HTMLElement>) => {
    if (event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget)) {
      return;
    }
    setSubClipDragPreview(null);
  };

  const handleSubTrackDrop = (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: DragEvent<HTMLElement>
  ) => {
    if (block.tone !== 'limit') {
      return;
    }
    dropHandledRef.current = true;
    event.stopPropagation();
    const tone = droppedTone(event) ?? activeTone;
    const startHour = subTrackHourFromClientX(block, event.currentTarget, event.clientX);
    markScheduleCustom();
    setDragTone(null);
    setDragPreview(null);
    setSubClipDragPreview(null);
    setSelectedBlockId(block.id);
    setPlacedBlocks((current) =>
      current.map((item) => (item.id === block.id ? insertSubClipOverride(item, subClip, tone, startHour) : item))
    );
  };

  const handleSubClipFit = (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
    event.stopPropagation();
    markScheduleCustom();
    setSelectedBlockId(block.id);
    setPlacedBlocks((current) =>
      current.map((item) => {
        if (item.id !== block.id) {
          return item;
        }
        return {
          ...item,
          subClips: [
            ...subClipsForBlock(item).filter((candidate) => candidate.controlId !== subClip.controlId),
            {
              ...subClip,
              disabled: false,
              endHour: item.endHour,
              id: `${item.id}-${stableDomId(subClip.controlId)}-fit`,
              startHour: item.startHour,
              tone: defaultSubClipTone(item.tone),
            },
          ],
        };
      })
    );
  };

  const handleBlockSubClipsFit = (blockId: string, event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    markScheduleCustom();
    setSelectedBlockId(blockId);
    setPlacedBlocks((current) =>
      current.map((item) => {
        if (item.id !== blockId) {
          return item;
        }
        return {
          ...item,
          subClips: subClipTracksForBlock(item).map((track) => ({
            ...(track.clips[0] as WeeklySchedulerSubClip),
            disabled: false,
            endHour: item.endHour,
            id: `${item.id}-${stableDomId(track.controlId)}-fit-all`,
            startHour: item.startHour,
            tone: defaultSubClipTone(item.tone),
          })),
        };
      })
    );
  };

  const handleBlockCollapseToggle = (blockId: string, event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    setSelectedBlockId(blockId);
    const isExpanding = collapsedBlockIds.includes(blockId);
    const block = scheduleBlocks.find((item) => item.id === blockId);
    setCollapsedBlockIds((current) =>
      current.includes(blockId) ? current.filter((item) => item !== blockId) : [...current, blockId]
    );
    if (isExpanding && block !== undefined && soloBlockId !== blockId) {
      const subTrackIds = collapsedSubClipIdsForBlock(block);
      setCollapsedSubClipIds((current) => Array.from(new Set([...current, ...subTrackIds])));
    }
  };

  const handleSubClipCollapseToggle = (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
    event.stopPropagation();
    setSelectedBlockId(block.id);
    const collapseId = subClipTrackCollapseId(block.id, subClip.controlId);
    setCollapsedSubClipIds((current) =>
      current.includes(collapseId) ? current.filter((item) => item !== collapseId) : [...current, collapseId]
    );
  };

  const handleDayCollapseToggle = (day: WeeklySchedulerDay, event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    setCollapsedDayIds((current) =>
      current.includes(day) ? current.filter((item) => item !== day) : [...current, day]
    );
  };

  const handleDaySolo = (day: WeeklySchedulerDay, event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    if (mode === 'daily' && selectedDays.length === 1 && selectedDays[0] === day) {
      setMode('weekly');
      setSelectedDays(SCHEDULER_DAYS.map((item) => item.id));
      setPresetScope('week');
      setDayMenuOpen(false);
      return;
    }
    setMode('daily');
    setSelectedDays([day]);
    setPresetScope('selected-day');
    setSelectedPresetDay(day);
    setDayMenuOpen(false);
    setCollapsedDayIds((current) => current.filter((item) => item !== day));
  };

  const handleToggleAllDays = (event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    setCollapsedDayIds((current) => {
      const visibleDayIds = visibleDays.map((day) => day.id);
      const visibleDayIdSet = new Set(visibleDayIds);
      const hiddenCollapsedDays = current.filter((day) => !visibleDayIdSet.has(day));
      if (visibleDayIds.every((day) => current.includes(day))) {
        return hiddenCollapsedDays;
      }
      return [...hiddenCollapsedDays, ...visibleDayIds];
    });
  };

  const handleToggleSubtracks = (event: ReactMouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    event.stopPropagation();
    setSubtracksHidden((current) => !current);
  };

  const handleClipMovePointerDown = (block: WeeklySchedulerBlock, event: ReactPointerEvent<HTMLDivElement>) => {
    if (soloBlockId !== null) {
      return;
    }
    const target = event.target;
    if (!(target instanceof Element)) {
      return;
    }
    const blockElement = event.currentTarget;
    const collapsed = blockElement.dataset['collapsed'] === 'true';
    const fromGrip = target.closest(classSelector(SCHEDULER_CLASS_NAMES.blockGrip)) !== null;
    const fromHeader = target.closest(classSelector(SCHEDULER_CLASS_NAMES.blockMain)) !== null;
    const fromResizeOrSubClip =
      target.closest(
        `${classSelector(SCHEDULER_CLASS_NAMES.blockResize)}, ${classSelector(SCHEDULER_CLASS_NAMES.subClip)}`
      ) !== null;
    if (
      event.button !== 0 ||
      target.closest(SCHEDULER_DOM_SELECTORS.button) !== null ||
      fromResizeOrSubClip ||
      (!collapsed && !fromGrip && !fromHeader)
    ) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    markScheduleCustom();
    setSelectedBlockId(block.id);
    setMovingBlockId(block.id);
    moveStateRef.current = {
      blockId: block.id,
      duration: block.endHour - block.startHour,
      hourWidth,
      originDay: block.day,
      originStartHour: block.startHour,
      pointerId: event.pointerId,
      x: event.clientX,
    };
  };

  const handleClipMovePointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    const moveState = moveStateRef.current;
    if (moveState === null || moveState.pointerId !== event.pointerId) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const hourDelta = snapHour((event.clientX - moveState.x) / moveState.hourWidth, -24, 24);
    const desiredStartHour = snapHour(moveState.originStartHour + hourDelta, 0, 24 - moveState.duration);
    const nextDay = timelineDayFromPoint(event.clientX, event.clientY) ?? moveState.originDay;
    setPlacedBlocks((current) =>
      mergeCompatibleBlocks(
        current.map((block) => {
          if (block.id !== moveState.blockId) {
            return block;
          }
          const nextStartHour = constrainedStartForMove(
            current,
            nextDay,
            desiredStartHour,
            moveState.duration,
            moveState.blockId
          );
          if (nextStartHour === null) {
            return block;
          }
          return shiftBlockTo(
            {
              ...block,
              day: nextDay,
              endHour: nextStartHour + moveState.duration,
              startHour: nextStartHour,
            },
            block
          );
        })
      )
    );
  };

  const handleClipMovePointerEnd = (event: ReactPointerEvent<HTMLDivElement>) => {
    const moveState = moveStateRef.current;
    if (moveState === null || moveState.pointerId !== event.pointerId) {
      return;
    }
    moveStateRef.current = null;
    setMovingBlockId(null);
    event.currentTarget.releasePointerCapture(event.pointerId);
  };

  const handleBlockKeyDown = (blockId: string, event: ReactKeyboardEvent<HTMLDivElement>) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      setSelectedBlockId(blockId);
    }
  };

  const handleNavigatorPointerDown = (edge: 'move' | 'start' | 'end', event: ReactPointerEvent<HTMLSpanElement>) => {
    const rail = event.currentTarget.closest(classSelector(SCHEDULER_CLASS_NAMES.navigatorRail));
    if (!(rail instanceof HTMLElement)) {
      return;
    }
    const railRect = rail.getBoundingClientRect();
    event.preventDefault();
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    navigatorDragStateRef.current = {
      edge,
      originEndHour: navigatorWindow.endHour,
      originStartHour: navigatorWindow.startHour,
      pointerId: event.pointerId,
      railLeft: railRect.left,
      railWidth: railRect.width,
      x: event.clientX,
    };
  };

  const handleNavigatorPointerMove = (event: ReactPointerEvent<HTMLSpanElement>) => {
    const dragState = navigatorDragStateRef.current;
    if (dragState === null || dragState.pointerId !== event.pointerId) {
      return;
    }
    event.preventDefault();
    event.stopPropagation();
    const deltaHours = ((event.clientX - dragState.x) / dragState.railWidth) * 24;
    if (dragState.edge === 'move') {
      const duration = dragState.originEndHour - dragState.originStartHour;
      const startHour = clamp(dragState.originStartHour + deltaHours, 0, 24 - duration);
      setTimelineWindow({ endHour: startHour + duration, startHour });
      return;
    }
    if (dragState.edge === 'start') {
      setTimelineWindow({
        endHour: dragState.originEndHour,
        startHour: dragState.originStartHour + deltaHours,
      });
      return;
    }
    setTimelineWindow({
      endHour: dragState.originEndHour + deltaHours,
      startHour: dragState.originStartHour,
    });
  };

  const handleNavigatorPointerEnd = (event: ReactPointerEvent<HTMLSpanElement>) => {
    const dragState = navigatorDragStateRef.current;
    if (dragState === null || dragState.pointerId !== event.pointerId) {
      return;
    }
    navigatorDragStateRef.current = null;
    event.currentTarget.releasePointerCapture(event.pointerId);
  };

  useLayoutEffect(() => {
    const viewport = viewportRef.current;
    if (viewport === null) {
      return;
    }
    const syncViewportWidth = () => setViewportWidth(viewport.clientWidth);
    syncViewportWidth();
    const resizeObserver = new ResizeObserver(syncViewportWidth);
    resizeObserver.observe(viewport);
    window.addEventListener('resize', syncViewportWidth);
    return () => {
      resizeObserver.disconnect();
      window.removeEventListener('resize', syncViewportWidth);
    };
  }, [fullscreen]);

  useEffect(() => {
    if (openMenuId === null) {
      return;
    }
    scheduleMenuClose(openMenuId);
    return clearMenuCloseTimer;
  }, [openMenuId]);

  useEffect(() => {
    return () => {
      clearMenuCloseTimer();
      clearMenuClosingTimer();
    };
  }, []);

  useEffect(() => {
    if (openTone === null && !dayMenuOpen && !presetMenuOpen) {
      return;
    }
    const closeOnOutsideClick = (event: PointerEvent) => {
      if (
        event.target instanceof Element &&
        event.target.closest(classSelector(SCHEDULER_CLASS_NAMES.presetControl)) !== null
      ) {
        return;
      }
      if (
        event.target instanceof Element &&
        event.target.closest(classSelector(SCHEDULER_CLASS_NAMES.legendItem)) !== null
      ) {
        return;
      }
      if (
        event.target instanceof Element &&
        event.target.closest(classSelector(SCHEDULER_CLASS_NAMES.modeToggle)) !== null
      ) {
        return;
      }
      if (openMenuId !== null) {
        closeMenuAnimated(openMenuId);
      }
    };
    document.addEventListener('pointerdown', closeOnOutsideClick);
    return () => document.removeEventListener('pointerdown', closeOnOutsideClick);
  }, [dayMenuOpen, openMenuId, openTone, presetMenuOpen]);

  useEffect(() => {
    if (timeEditorTarget === null) {
      return;
    }
    const closeOnOutsideClick = (event: PointerEvent) => {
      if (
        event.target instanceof Element &&
        event.target.closest(classSelector(SCHEDULER_CLASS_NAMES.blockTimeEditor)) !== null
      ) {
        return;
      }
      setTimeEditorTarget(null);
    };
    const closeOnEscape = (event: globalThis.KeyboardEvent) => {
      if (event.key === 'Escape') {
        setTimeEditorTarget(null);
      }
    };
    document.addEventListener('pointerdown', closeOnOutsideClick, true);
    document.addEventListener('keydown', closeOnEscape);
    return () => {
      document.removeEventListener('pointerdown', closeOnOutsideClick, true);
      document.removeEventListener('keydown', closeOnEscape);
    };
  }, [timeEditorTarget]);

  useLayoutEffect(() => {
    if (!embedded || !fullscreen) {
      setFullscreenBounds(null);
      return;
    }

    const updateFullscreenBounds = () => {
      const shell =
        fullscreenAnchorRef.current?.closest('.oc-unified-shell') ?? document.querySelector('.oc-unified-shell');
      const headerSlot = shell?.querySelector<HTMLElement>('.oc-unified-shell__slot--header') ?? null;
      const footerSlot = shell?.querySelector<HTMLElement>('.oc-unified-shell__slot--footer') ?? null;
      const top = Math.max(0, headerSlot?.getBoundingClientRect().bottom ?? 0);
      const bottom = Math.max(0, footerSlot === null ? 0 : window.innerHeight - footerSlot.getBoundingClientRect().top);
      setFullscreenBounds({ bottom, top });
    };

    updateFullscreenBounds();
    const observer = new ResizeObserver(updateFullscreenBounds);
    const shell =
      fullscreenAnchorRef.current?.closest('.oc-unified-shell') ?? document.querySelector('.oc-unified-shell');
    const headerSlot = shell?.querySelector<HTMLElement>('.oc-unified-shell__slot--header') ?? null;
    const footerSlot = shell?.querySelector<HTMLElement>('.oc-unified-shell__slot--footer') ?? null;
    if (shell instanceof Element) {
      observer.observe(shell);
    }
    if (headerSlot !== null) {
      observer.observe(headerSlot);
    }
    if (footerSlot !== null) {
      observer.observe(footerSlot);
    }
    window.addEventListener('resize', updateFullscreenBounds);
    return () => {
      observer.disconnect();
      window.removeEventListener('resize', updateFullscreenBounds);
    };
  }, [embedded, fullscreen]);

  useEffect(() => {
    if (!fullscreen) {
      document.documentElement.removeAttribute(SCHEDULER_DOCUMENT_ATTRIBUTES.fullscreen);
      return;
    }
    document.documentElement.setAttribute(SCHEDULER_DOCUMENT_ATTRIBUTES.fullscreen, 'true');
    const closeOnEscape = (event: globalThis.KeyboardEvent) => {
      if (event.key === 'Escape') {
        setFullscreen(false);
      }
    };
    document.addEventListener('keydown', closeOnEscape);
    return () => {
      document.documentElement.removeAttribute(SCHEDULER_DOCUMENT_ATTRIBUTES.fullscreen);
      document.removeEventListener('keydown', closeOnEscape);
    };
  }, [fullscreen]);

  useEffect(() => {
    onScheduleChange?.(scheduleEdl);
  }, [onScheduleChange, scheduleEdlJson]);

  const schedulerPage = (
    <article
      className="weekly-scheduler-page"
      data-embedded={embedded}
      data-fullscreen={fullscreen}
      ref={
        fullscreen
          ? undefined
          : (node) => {
              fullscreenAnchorRef.current = node;
            }
      }
      style={pageStyle}
    >
      <header className="weekly-scheduler-hero">
        <QuickPresetControl
          activePreset={activePreset}
          closing={closingMenu === 'preset'}
          open={presetMenuOpen}
          onMenuMouseEnter={() => handleMenuMouseEnter('preset')}
          onMenuMouseLeave={() => handleMenuMouseLeave('preset')}
          onOpenChange={togglePresetMenu}
          onPresetSelect={applyPreset}
          onPresetScopeChange={setPresetScope}
          onProfileSave={saveCurrentProfile}
          presets={allPresets}
          presetScope={presetScope}
          selectedDayLabel={dayLabel(selectedPresetDay)}
        />
        <div className="weekly-scheduler-legend" aria-label="Schedule legend">
          {SCHEDULER_LEGENDS.map((legend) => (
            <LegendControl
              active={legend.tone === activeTone}
              activeGroupId={activeGroupByTone[legend.tone]}
              closing={closingMenu === toneMenuId(legend.tone)}
              groups={profile.controlGroupsByTone[legend.tone]}
              key={legend.tone}
              label={legend.label}
              onGroupChange={(groupId) => {
                setActiveGroupByTone((current) => ({ ...current, [legend.tone]: groupId }));
              }}
              onControlToggle={(controlId) => {
                setToneControls((current) => ({
                  ...current,
                  [legend.tone]: toggleControl(current[legend.tone], controlId),
                }));
              }}
              onSwatchClick={() => setActiveTone(legend.tone)}
              onSwatchDragEnd={(event) => handleLegendDragEnd(legend.tone, event)}
              onSwatchDragStart={(event) => handleLegendDragStart(legend.tone, event)}
              onMenuMouseEnter={() => handleMenuMouseEnter(toneMenuId(legend.tone))}
              onMenuMouseLeave={() => handleMenuMouseLeave(toneMenuId(legend.tone))}
              onTextClick={() => {
                setActiveTone(legend.tone);
                toggleToneMenu(legend.tone);
              }}
              open={openTone === legend.tone}
              selectedControls={toneControls[legend.tone]}
              tone={legend.tone}
            />
          ))}
        </div>
        <div className="weekly-scheduler-mode-toggle" aria-label={SCHEDULER_COPY.modeLabel} role="tablist">
          <div className="weekly-scheduler-day-mode">
            <ModeButton
              active={mode === 'daily'}
              label={SCHEDULER_COPY.daily}
              onClick={() => {
                setMode('daily');
                toggleDayMenu();
              }}
            />
            {mode === 'daily' ? (
              <button
                aria-expanded={dayMenuOpen}
                aria-label="Choose visible day"
                className="weekly-scheduler-day-mode-trigger"
                onClick={toggleDayMenu}
                type="button"
              >
                {daySelectionLabel(selectedDays)}
              </button>
            ) : null}
            {dayMenuOpen || closingMenu === 'day' ? (
              <div
                className="weekly-scheduler-day-menu"
                data-state={dayMenuOpen ? 'open' : 'closing'}
                onMouseEnter={() => handleMenuMouseEnter('day')}
                onMouseLeave={() => handleMenuMouseLeave('day')}
                role="menu"
              >
                <button
                  aria-checked={selectedDays.length === SCHEDULER_DAYS.length}
                  className="weekly-scheduler-day-choice"
                  data-selected={selectedDays.length === SCHEDULER_DAYS.length}
                  onClick={() => {
                    setMode('daily');
                    setPresetScope('week');
                    setSelectedDays((current) => toggleAllDaySelection(current));
                  }}
                  role="menuitemcheckbox"
                  type="button"
                >
                  <span className="weekly-scheduler-choice-circle" />
                  <span aria-hidden="true" className="weekly-scheduler-choice-divider">
                    |
                  </span>
                  <span>All</span>
                </button>
                {SCHEDULER_DAYS.map((day) => (
                  <button
                    aria-checked={selectedDays.includes(day.id)}
                    className="weekly-scheduler-day-choice"
                    data-selected={selectedDays.includes(day.id)}
                    key={day.id}
                    onClick={() => {
                      setMode('daily');
                      setPresetScope('selected-day');
                      setSelectedPresetDay(day.id);
                      setSelectedDays((current) => toggleDaySelection(current, day.id));
                    }}
                    role="menuitemcheckbox"
                    type="button"
                  >
                    <span className="weekly-scheduler-choice-circle" />
                    <span aria-hidden="true" className="weekly-scheduler-choice-divider">
                      |
                    </span>
                    <span>{day.label}</span>
                  </button>
                ))}
              </div>
            ) : null}
          </div>
          <ModeButton
            active={mode === 'weekly'}
            label={SCHEDULER_COPY.weekly}
            onClick={() => {
              setMode('weekly');
              setPresetScope('week');
              if (dayMenuOpen) {
                closeMenuAnimated('day');
              }
            }}
          />
          <ToolButton
            active={selectedBlock !== null}
            label={selectedBlock === null || selectedClipIsFit ? 'Fit full width' : 'Fit selected clip'}
            onClick={handleFitTimelineWindow}
          >
            {'<-->'}
          </ToolButton>
          <ToolButton
            active={fullscreen}
            label={fullscreen ? 'Exit fullscreen editor' : 'Fullscreen editor'}
            onClick={() => setFullscreen((current) => !current)}
          >
            <FullscreenIcon active={fullscreen} />
          </ToolButton>
        </div>
      </header>

      <section className="weekly-scheduler-shell" aria-label={`${mode} schedule board`}>
        <div
          className="weekly-scheduler-board"
          data-clip-solo={soloBlock !== null}
          data-density={visibleDays.length > 2 ? 'compact' : 'expanded'}
          data-fit={fitRows}
          data-hour-density={hourWidth < 16 ? 'micro' : hourWidth < 32 ? 'dense' : 'normal'}
          style={schedulerStyle}
        >
          <div className="weekly-scheduler-board-viewport" data-dragging={dragTone !== null} ref={viewportRef}>
            <div className="weekly-scheduler-board-header">
              <div className="weekly-scheduler-time-corner">
                <button
                  aria-label={allVisibleDaysCollapsed ? 'Expand all track rows' : 'Fold all track rows'}
                  aria-pressed={!allVisibleDaysCollapsed}
                  className="weekly-scheduler-track-fold-all"
                  data-expanded={!allVisibleDaysCollapsed}
                  onClick={handleToggleAllDays}
                  title={allVisibleDaysCollapsed ? 'Expand all track rows' : 'Fold all track rows'}
                  type="button"
                >
                  <span className="weekly-scheduler-track-fold-all-icon" aria-hidden="true" />
                </button>
                <button
                  aria-label={subtracksHidden ? 'Show clip control tracks' : 'Hide clip control tracks'}
                  aria-pressed={subtracksHidden}
                  className="weekly-scheduler-subtracks-toggle"
                  data-active={subtracksHidden}
                  onClick={handleToggleSubtracks}
                  title={subtracksHidden ? 'Show clip control tracks' : 'Hide clip control tracks'}
                  type="button"
                >
                  <span className="weekly-scheduler-subtracks-toggle-icon" aria-hidden="true" />
                </button>
              </div>
              <div className="weekly-scheduler-time-axis-viewport">
                <div className="weekly-scheduler-time-axis-track">
                  {SCHEDULER_HOURS.map((hour) => (
                    <div className="weekly-scheduler-hour-heading" data-major={hour % 3 === 0} key={hour}>
                      {gridHourLabel(hour)}
                    </div>
                  ))}
                </div>
              </div>
            </div>
            <div className="weekly-scheduler-grid" data-all-folded={allVisibleDaysCollapsed}>
              {visibleDays.map((day) => {
                const dayBlocks = scheduleBlocks.filter((block) => block.day === day.id);
                const observeOnly = dayBlocks.length === 0;
                return (
                  <div
                    className="weekly-scheduler-day-row"
                    data-folded={collapsedDayIds.includes(day.id)}
                    data-preset-selected={presetScope === 'selected-day' && selectedPresetDay === day.id}
                    data-whole-day-drop={dayDropTarget === day.id}
                    key={day.id}
                  >
                    <div
                      className="weekly-scheduler-day-side-heading"
                      onClick={() => selectPresetDay(day.id)}
                      onDragLeave={handleDayApplyDragLeave}
                      onDragOver={(event) => handleDayApplyDragOver(event, day.id)}
                      onDrop={(event) => handleDayApplyDrop(event, day.id)}
                    >
                      <div className="weekly-scheduler-day-side-tools">
                        <button
                          aria-label={
                            collapsedDayIds.includes(day.id)
                              ? `Expand ${day.label} track`
                              : `Collapse ${day.label} track`
                          }
                          aria-pressed={!collapsedDayIds.includes(day.id)}
                          className="weekly-scheduler-day-foldout"
                          data-expanded={!collapsedDayIds.includes(day.id)}
                          onClick={(event) => handleDayCollapseToggle(day.id, event)}
                          type="button"
                        >
                          <span className="weekly-scheduler-day-foldout-icon" aria-hidden="true" />
                        </button>
                      </div>
                      <span className="weekly-scheduler-day-side-label">{day.label}</span>
                      <div className="weekly-scheduler-day-side-footer">
                        <button
                          aria-label={`Solo ${day.label} track`}
                          aria-pressed={mode === 'daily' && selectedDays.length === 1 && selectedDays[0] === day.id}
                          className="weekly-scheduler-day-solo"
                          data-active={mode === 'daily' && selectedDays.length === 1 && selectedDays[0] === day.id}
                          onClick={(event) => handleDaySolo(day.id, event)}
                          title={`Solo ${day.label}`}
                          type="button"
                        >
                          Solo
                        </button>
                      </div>
                    </div>
                    <div className="weekly-scheduler-day-viewport">
                      <div
                        aria-label={`${day.label} timeline track`}
                        className="weekly-scheduler-day-slots"
                        data-drag-target={dragPreview?.day === day.id}
                        data-timeline-day={day.id}
                        onClick={() => selectPresetDay(day.id)}
                        onDragLeave={handleTrackDragLeave}
                        onDragOver={(event) => handleTrackDragOver(event, day.id)}
                        onDrop={(event) => handleTrackDrop(event, day.id)}
                        role="presentation"
                      >
                        {observeOnly ? (
                          <div className="weekly-scheduler-observe-only-state">
                            <span className="weekly-scheduler-observe-only-badge">Observe</span>
                            <span className="weekly-scheduler-observe-only-copy">Report activity only</span>
                            <span className="weekly-scheduler-observe-only-hint">
                              Drag an action from top to manage this track
                            </span>
                          </div>
                        ) : null}
                        {trackObserveGaps(scheduleBlocks, day.id).map((gap) => (
                          <div
                            aria-hidden="true"
                            className="weekly-scheduler-observe-gap"
                            key={`${day.id}-${gap.startHour}-${gap.endHour}`}
                            style={gapStyle(gap, hourWidth)}
                          >
                            <span
                              className="weekly-scheduler-observe-gap-time"
                              data-hidden={rangeLabelHidden(gap.startHour, gap.endHour, hourWidth)}
                            >
                              {compactTimeRangeLabel(gap.startHour, gap.endHour)}
                            </span>
                          </div>
                        ))}
                        {dragPreview?.day === day.id ? (
                          <div
                            aria-hidden="true"
                            className={`weekly-scheduler-drag-preview weekly-scheduler-block--${dragPreview.tone}`}
                            data-valid={dragPreview.valid}
                            style={previewStyle(dragPreview.startHour, hourWidth)}
                          >
                            <span
                              className="weekly-scheduler-drag-preview-time"
                              data-hidden={rangeLabelHidden(
                                dragPreview.startHour,
                                dragPreview.startHour + 1,
                                hourWidth
                              )}
                            >
                              {compactTimeRangeLabel(dragPreview.startHour, dragPreview.startHour + 1)}
                            </span>
                          </div>
                        ) : null}
                        {dayBlocks.map((block) => (
                          <MainTimelineClip
                            block={block}
                            collapsed={subtracksHidden || collapsedBlockIds.includes(block.id)}
                            controlsHidden={clipControlsHidden}
                            hourWidth={hourWidth}
                            key={block.id}
                            moving={movingBlockId === block.id}
                            onClick={(event) => handleBlockClick(block, event)}
                            onDoubleClick={(event) => {
                              event.preventDefault();
                              event.stopPropagation();
                              enterClipSolo(block);
                            }}
                            onDragOver={(event) => handleClipDragOver(event, block)}
                            onDrop={(event) => handleClipDrop(event, block)}
                            onKeyDown={(event) => handleBlockKeyDown(block.id, event)}
                            onPointerCancel={handleClipMovePointerEnd}
                            onPointerDown={(event) => handleClipMovePointerDown(block, event)}
                            onPointerMove={handleClipMovePointerMove}
                            onPointerUp={handleClipMovePointerEnd}
                            onCollapseToggle={handleBlockCollapseToggle}
                            onFitAllSubClips={handleBlockSubClipsFit}
                            onDelete={handleBlockDelete}
                            onSolo={handleBlockSolo}
                            onResizePointerCancel={handleResizePointerEnd}
                            onResizePointerDown={handleResizePointerDown}
                            onResizePointerMove={handleResizePointerMove}
                            onResizePointerUp={handleResizePointerEnd}
                            onTimeEditorApply={applyTimeEditorDraft}
                            onTimeEditorOpen={openBlockTimeEditor}
                            onTimeDraftChange={updateTimeEditorDraft}
                            onSubClipTimeEditorOpen={openSubClipTimeEditor}
                            onSubClipCollapseToggle={handleSubClipCollapseToggle}
                            onSubClipFit={handleSubClipFit}
                            onSubTrackDragLeave={handleSubTrackDragLeave}
                            onSubTrackDragOver={handleSubTrackDragOver}
                            onSubTrackDrop={handleSubTrackDrop}
                            onSubClipPointerCancel={handleSubClipPointerEnd}
                            onSubClipPointerDown={handleSubClipPointerDown}
                            onSubClipPointerMove={handleSubClipPointerMove}
                            onSubClipToggle={handleSubClipToggle}
                            onSubClipPointerUp={handleSubClipPointerEnd}
                            selected={selectedBlockId === block.id}
                            soloActive={soloBlockId === block.id}
                            soloMode={soloBlockId !== null}
                            timeEditorDraft={timeEditorDraft}
                            timeEditorTarget={timeEditorTarget}
                            subClipDragPreview={subClipDragPreview}
                            collapsedSubClipIds={collapsedSubClipIds}
                          />
                        ))}
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </div>
        <TimelineNavigator
          onPointerDown={handleNavigatorPointerDown}
          onPointerEnd={handleNavigatorPointerEnd}
          onPointerMove={handleNavigatorPointerMove}
          windowState={navigatorWindow}
        />
      </section>
    </article>
  );

  if (embedded && fullscreen && typeof document !== 'undefined') {
    return (
      <>
        <div
          aria-hidden="true"
          className="weekly-scheduler-page-placeholder"
          ref={(node) => {
            fullscreenAnchorRef.current = node;
          }}
          style={embeddedPageStyle}
        />
        {createPortal(schedulerPage, document.body)}
      </>
    );
  }

  return schedulerPage;
}

function MainTimelineClip({
  block,
  collapsed,
  collapsedSubClipIds,
  controlsHidden,
  hourWidth,
  moving,
  onClick,
  onDoubleClick,
  onDragOver,
  onDrop,
  onKeyDown,
  onPointerCancel,
  onCollapseToggle,
  onPointerDown,
  onPointerMove,
  onPointerUp,
  onResizePointerCancel,
  onResizePointerDown,
  onResizePointerMove,
  onResizePointerUp,
  onTimeEditorApply,
  onTimeEditorOpen,
  onTimeDraftChange,
  onFitAllSubClips,
  onDelete,
  onSolo,
  onSubClipCollapseToggle,
  onSubClipFit,
  onSubTrackDragLeave,
  onSubTrackDragOver,
  onSubTrackDrop,
  onSubClipPointerCancel,
  onSubClipPointerDown,
  onSubClipPointerMove,
  onSubClipToggle,
  onSubClipTimeEditorOpen,
  onSubClipPointerUp,
  selected,
  soloActive,
  soloMode,
  timeEditorDraft,
  timeEditorTarget,
  subClipDragPreview,
}: {
  readonly block: WeeklySchedulerBlock;
  readonly collapsed: boolean;
  readonly controlsHidden: boolean;
  readonly hourWidth: number;
  readonly moving: boolean;
  readonly onClick: (event: ReactMouseEvent<HTMLDivElement>) => void;
  readonly onDoubleClick: (event: ReactMouseEvent<HTMLDivElement>) => void;
  readonly onDragOver: (event: DragEvent<HTMLDivElement>) => void;
  readonly onDrop: (event: DragEvent<HTMLDivElement>) => void;
  readonly onKeyDown: (event: ReactKeyboardEvent<HTMLDivElement>) => void;
  readonly onPointerCancel: (event: ReactPointerEvent<HTMLDivElement>) => void;
  readonly onCollapseToggle: (blockId: string, event: ReactMouseEvent<HTMLButtonElement>) => void;
  readonly onPointerDown: (event: ReactPointerEvent<HTMLDivElement>) => void;
  readonly onPointerMove: (event: ReactPointerEvent<HTMLDivElement>) => void;
  readonly onPointerUp: (event: ReactPointerEvent<HTMLDivElement>) => void;
  readonly onResizePointerCancel: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onResizePointerDown: (
    block: WeeklySchedulerBlock,
    edge: 'start' | 'end',
    event: ReactPointerEvent<HTMLElement>
  ) => void;
  readonly onResizePointerMove: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onResizePointerUp: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onTimeEditorApply: () => void;
  readonly onTimeEditorOpen: (block: WeeklySchedulerBlock, event: ReactMouseEvent<HTMLButtonElement>) => void;
  readonly onTimeDraftChange: (
    edge: WeeklySchedulerTimeEditorEdge,
    part: WeeklySchedulerTimeEditorPart,
    value: string
  ) => void;
  readonly onFitAllSubClips: (blockId: string, event: ReactMouseEvent<HTMLButtonElement>) => void;
  readonly onDelete: (block: WeeklySchedulerBlock, event: ReactMouseEvent<HTMLButtonElement>) => void;
  readonly onSolo: (block: WeeklySchedulerBlock, event: ReactMouseEvent<HTMLButtonElement>) => void;
  readonly onSubClipCollapseToggle: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly onSubClipFit: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly onSubTrackDragLeave: (event: DragEvent<HTMLElement>) => void;
  readonly onSubTrackDragOver: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: DragEvent<HTMLElement>
  ) => void;
  readonly onSubTrackDrop: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: DragEvent<HTMLElement>
  ) => void;
  readonly onSubClipPointerCancel: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onSubClipPointerDown: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    edge: 'move' | 'start' | 'end',
    event: ReactPointerEvent<HTMLElement>
  ) => void;
  readonly onSubClipPointerMove: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onSubClipToggle: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly onSubClipTimeEditorOpen: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly onSubClipPointerUp: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly selected: boolean;
  readonly soloActive: boolean;
  readonly soloMode: boolean;
  readonly timeEditorDraft: WeeklySchedulerTimeEditorDraft;
  readonly timeEditorTarget: WeeklySchedulerTimeEditorTarget | null;
  readonly subClipDragPreview: {
    readonly blockId: string;
    readonly controlId: WeeklySchedulerControlId;
    readonly startHour: number;
    readonly tone: WeeklySchedulerActionId;
  } | null;
  readonly collapsedSubClipIds: readonly string[];
}): ReactElement {
  const subClipTracks = subClipTracksForBlock(block);
  const blockTimeEditorOpen = isBlockTimeEditorTarget(timeEditorTarget, block.id);
  const subClipTimeEditorOpen = timeEditorTarget?.kind === 'subclip' && timeEditorTarget.blockId === block.id;

  return (
    <div
      aria-label={`${toneLabel(block.tone)} ${timeLabel(block.startHour)} to ${timeLabel(block.endHour)}`}
      className={`weekly-scheduler-block weekly-scheduler-block--${block.tone}`}
      data-block-id={block.id}
      data-collapsed={collapsed}
      data-controls-hidden={controlsHidden}
      data-moving={moving}
      data-selected={selected}
      data-solo-active={soloActive}
      data-solo-mode={soloMode}
      data-subclip-count={subClipTracks.length}
      data-time-editor-open={blockTimeEditorOpen || subClipTimeEditorOpen}
      onClick={onClick}
      onDoubleClick={onDoubleClick}
      onDragOver={onDragOver}
      onDrop={onDrop}
      onKeyDown={onKeyDown}
      onPointerCancel={onPointerCancel}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      role="button"
      style={blockStyle(block, hourWidth)}
      tabIndex={0}
    >
      <span
        aria-hidden="true"
        className="weekly-scheduler-block-resize weekly-scheduler-block-resize--start"
        onPointerCancel={onResizePointerCancel}
        onPointerDown={(event) => onResizePointerDown(block, 'start', event)}
        onPointerMove={onResizePointerMove}
        onPointerUp={onResizePointerUp}
      />
      <span className="weekly-scheduler-block-main" title="Drag clip">
        <button
          aria-label={collapsed ? 'Expand clip controls' : 'Collapse clip controls'}
          aria-pressed={!collapsed}
          className="weekly-scheduler-block-collapse"
          data-expanded={!collapsed}
          onClick={(event) => onCollapseToggle(block.id, event)}
          onPointerDown={(event) => event.stopPropagation()}
          type="button"
        >
          <span className="weekly-scheduler-collapse-icon" aria-hidden="true" />
        </button>
        <button
          aria-label={`Edit time from ${timeLabel(block.startHour)} to ${timeLabel(block.endHour)}`}
          className="weekly-scheduler-block-time"
          data-hidden={rangeLabelHidden(
            block.startHour,
            block.endHour,
            hourWidth,
            SCHEDULER_PARENT_TIME_LABEL_MIN_WIDTH
          )}
          onClick={(event) => onTimeEditorOpen(block, event)}
          onPointerDown={(event) => event.stopPropagation()}
          title="Edit clip time"
          type="button"
        >
          {compactTimeRangeLabel(block.startHour, block.endHour)}
        </button>
        {blockTimeEditorOpen ? (
          <ClipTimeEditor draft={timeEditorDraft} onApply={onTimeEditorApply} onDraftChange={onTimeDraftChange} />
        ) : null}
        <span className="weekly-scheduler-block-grip" aria-hidden="true" />
        <span className="weekly-scheduler-block-actions">
          <button
            aria-label={soloActive ? 'Exit clip solo' : `Solo ${toneLabel(block.tone)} clip`}
            className="weekly-scheduler-block-solo"
            data-active={soloActive}
            onClick={(event) => onSolo(block, event)}
            onPointerDown={(event) => event.stopPropagation()}
            title={soloActive ? 'Exit clip solo' : 'Solo clip'}
            type="button"
          >
            {soloActive ? 'Exit' : 'Solo'}
          </button>
          <button
            aria-label="Fit all clip controls"
            className="weekly-scheduler-block-fit"
            onClick={(event) => onFitAllSubClips(block.id, event)}
            onPointerDown={(event) => event.stopPropagation()}
            title="Fit all clip controls to parent clip time"
            type="button"
          >
            {'<-->'}
          </button>
          <button
            aria-label="Delete clip"
            className="weekly-scheduler-block-delete"
            onClick={(event) => onDelete(block, event)}
            onPointerDown={(event) => event.stopPropagation()}
            title="Delete clip"
            type="button"
          >
            <span className="weekly-scheduler-trash-icon" aria-hidden="true" />
          </button>
        </span>
      </span>
      <span
        className="weekly-scheduler-subclip-layer"
        data-collapsed={collapsed}
        data-subclip-count={subClipTracks.length}
        onPointerDown={(event) => event.stopPropagation()}
      >
        {subClipTracks.map((track, subClipIndex) => (
          <SubTimelineTrack
            block={block}
            key={track.controlId}
            collapsed={collapsedSubClipIds.includes(subClipTrackCollapseId(block.id, track.controlId))}
            onCollapseToggle={onSubClipCollapseToggle}
            onFit={onSubClipFit}
            onDragLeave={onSubTrackDragLeave}
            onDragOver={onSubTrackDragOver}
            onDrop={onSubTrackDrop}
            onPointerCancel={onSubClipPointerCancel}
            onPointerDown={onSubClipPointerDown}
            onPointerMove={onSubClipPointerMove}
            onToggle={onSubClipToggle}
            onTimeEditorApply={onTimeEditorApply}
            onTimeDraftChange={onTimeDraftChange}
            onTimeEditorOpen={onSubClipTimeEditorOpen}
            onPointerUp={onSubClipPointerUp}
            preview={subClipDragPreview}
            supportsOverrides={block.tone === 'limit'}
            track={track}
            tracks={subClipTracks}
            subClipIndex={subClipIndex}
            hourWidth={hourWidth}
            timeEditorDraft={timeEditorDraft}
            timeEditorTarget={timeEditorTarget}
          />
        ))}
      </span>
      <span
        aria-hidden="true"
        className="weekly-scheduler-block-resize weekly-scheduler-block-resize--end"
        onPointerCancel={onResizePointerCancel}
        onPointerDown={(event) => onResizePointerDown(block, 'end', event)}
        onPointerMove={onResizePointerMove}
        onPointerUp={onResizePointerUp}
      />
    </div>
  );
}

function ClipTimeEditor({
  draft,
  onApply,
  onDraftChange,
}: {
  readonly draft: WeeklySchedulerTimeEditorDraft;
  readonly onApply: () => void;
  readonly onDraftChange: (
    edge: WeeklySchedulerTimeEditorEdge,
    part: WeeklySchedulerTimeEditorPart,
    value: string
  ) => void;
}): ReactElement {
  const start = timeEditorMinuteParts(draft.startMinute);
  const end = timeEditorMinuteParts(draft.endMinute);
  return (
    <form
      aria-label="Clip time editor"
      className="weekly-scheduler-block-time-editor"
      onClick={(event) => event.stopPropagation()}
      onDoubleClick={(event) => event.stopPropagation()}
      onSubmit={(event) => {
        event.preventDefault();
        event.stopPropagation();
        onApply();
      }}
      onPointerDown={(event) => event.stopPropagation()}
    >
      <span>From</span>
      <input
        aria-label="Clip start hour"
        className="weekly-scheduler-time-number weekly-scheduler-time-number--hour"
        max={24}
        min={0}
        onChange={(event) => onDraftChange('start', 'hour', event.currentTarget.value)}
        step={1}
        type="number"
        value={start.hour}
      />
      <span className="weekly-scheduler-time-editor-mark">:</span>
      <input
        aria-label="Clip start minute"
        className="weekly-scheduler-time-number weekly-scheduler-time-number--minute"
        max={55}
        min={0}
        onChange={(event) => onDraftChange('start', 'minute', event.currentTarget.value)}
        step={SCHEDULER_SNAP_MINUTES}
        type="number"
        value={start.minute}
      />
      <span>to</span>
      <input
        aria-label="Clip end hour"
        className="weekly-scheduler-time-number weekly-scheduler-time-number--hour"
        max={24}
        min={0}
        onChange={(event) => onDraftChange('end', 'hour', event.currentTarget.value)}
        step={1}
        type="number"
        value={end.hour}
      />
      <span className="weekly-scheduler-time-editor-mark">:</span>
      <input
        aria-label="Clip end minute"
        className="weekly-scheduler-time-number weekly-scheduler-time-number--minute"
        max={55}
        min={0}
        onChange={(event) => onDraftChange('end', 'minute', event.currentTarget.value)}
        step={SCHEDULER_SNAP_MINUTES}
        type="number"
        value={end.minute}
      />
      <button type="submit">Set</button>
    </form>
  );
}

function SubTimelineTrack({
  block,
  collapsed,
  hourWidth,
  onDragLeave,
  onDragOver,
  onDrop,
  onCollapseToggle,
  onFit,
  onPointerCancel,
  onPointerDown,
  onPointerMove,
  onPointerUp,
  onToggle,
  onTimeEditorApply,
  onTimeDraftChange,
  onTimeEditorOpen,
  preview,
  supportsOverrides,
  track,
  tracks,
  subClipIndex,
  timeEditorDraft,
  timeEditorTarget,
}: {
  readonly block: WeeklySchedulerBlock;
  readonly collapsed: boolean;
  readonly hourWidth: number;
  readonly onDragLeave: (event: DragEvent<HTMLElement>) => void;
  readonly onDragOver: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: DragEvent<HTMLElement>
  ) => void;
  readonly onDrop: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: DragEvent<HTMLElement>
  ) => void;
  readonly onCollapseToggle: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly onFit: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly onPointerCancel: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onPointerDown: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    edge: 'move' | 'start' | 'end',
    event: ReactPointerEvent<HTMLElement>
  ) => void;
  readonly onPointerMove: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onPointerUp: (event: ReactPointerEvent<HTMLElement>) => void;
  readonly onToggle: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly onTimeEditorApply: () => void;
  readonly onTimeDraftChange: (
    edge: WeeklySchedulerTimeEditorEdge,
    part: WeeklySchedulerTimeEditorPart,
    value: string
  ) => void;
  readonly onTimeEditorOpen: (
    block: WeeklySchedulerBlock,
    subClip: WeeklySchedulerSubClip,
    event: ReactMouseEvent<HTMLButtonElement>
  ) => void;
  readonly preview: {
    readonly blockId: string;
    readonly controlId: WeeklySchedulerControlId;
    readonly startHour: number;
    readonly tone: WeeklySchedulerActionId;
  } | null;
  readonly supportsOverrides: boolean;
  readonly track: WeeklySchedulerSubClipTrack;
  readonly tracks: readonly WeeklySchedulerSubClipTrack[];
  readonly subClipIndex: number;
  readonly timeEditorDraft: WeeklySchedulerTimeEditorDraft;
  readonly timeEditorTarget: WeeklySchedulerTimeEditorTarget | null;
}): ReactElement {
  const representativeSubClip = track.clips[0] as WeeklySchedulerSubClip;
  const disabled = track.disabled;
  const visiblePreview =
    preview !== null && preview.blockId === block.id && preview.controlId === track.controlId ? preview : null;
  const previewEndHour = visiblePreview === null ? null : subClipPreviewEndHour(block, visiblePreview.startHour);

  return (
    <span
      className="weekly-scheduler-subtrack"
      data-disabled={disabled}
      data-folded={collapsed}
      style={subClipTrackStyle(track, subClipIndex, tracks)}
      title={`${track.label} control row`}
    >
      <span className="weekly-scheduler-subtrack-header">
        <button
          aria-label={collapsed ? `Expand ${track.label}` : `Collapse ${track.label}`}
          aria-pressed={!collapsed}
          className="weekly-scheduler-subtrack-foldout"
          data-expanded={!collapsed}
          onClick={(event) => onCollapseToggle(block, representativeSubClip, event)}
          onPointerDown={(event) => event.stopPropagation()}
          type="button"
        >
          <span className="weekly-scheduler-subtrack-foldout-icon" aria-hidden="true" />
        </button>
        <span className="weekly-scheduler-subtrack-name">{track.label}</span>
        <button
          aria-label={`Fit ${track.label}`}
          className="weekly-scheduler-subtrack-fit"
          onClick={(event) => onFit(block, representativeSubClip, event)}
          onPointerDown={(event) => event.stopPropagation()}
          title={`Fit ${track.label} to clip time`}
          type="button"
        >
          {'<-->'}
        </button>
      </span>
      <span className="weekly-scheduler-subtrack-body">
        <button
          aria-label={`${disabled ? 'Enable' : 'Disable'} ${track.label}`}
          aria-pressed={!disabled}
          className="weekly-scheduler-subtrack-toggle"
          data-enabled={!disabled}
          onClick={(event) => onToggle(block, representativeSubClip, event)}
          onPointerDown={(event) => event.stopPropagation()}
          title={`${disabled ? 'Enable' : 'Disable'} ${track.label}`}
          type="button"
        >
          <span className="weekly-scheduler-eye" aria-hidden="true" />
        </button>
        <span
          className="weekly-scheduler-subtrack-surface"
          data-disabled={disabled}
          onDragLeave={onDragLeave}
          onDragOver={(event) => onDragOver(block, representativeSubClip, event)}
          onDrop={(event) => onDrop(block, representativeSubClip, event)}
        >
          {supportsOverrides ? <span className="weekly-scheduler-subtrack-blocked-label">Blocked</span> : null}
          {visiblePreview === null ? null : (
            <span
              aria-hidden="true"
              className={`weekly-scheduler-subclip-preview weekly-scheduler-subclip--${visiblePreview.tone}`}
              style={subClipPreviewStyle(block, visiblePreview.startHour)}
            >
              <span
                className="weekly-scheduler-subclip-time weekly-scheduler-subclip-time--preview"
                data-hidden={
                  previewEndHour === null || rangeLabelHidden(visiblePreview.startHour, previewEndHour, hourWidth)
                }
              >
                {previewEndHour === null ? null : compactTimeRangeLabel(visiblePreview.startHour, previewEndHour)}
              </span>
            </span>
          )}
          {track.clips.map((subClip) => {
            const subClipTimeEditorOpen = isSubClipTimeEditorTarget(timeEditorTarget, block.id, subClip.id);
            return (
              <span
                className={`weekly-scheduler-subclip weekly-scheduler-subclip--${subClipTone(block, subClip)}`}
                data-disabled={subClip.disabled === true}
                data-time-editor-open={subClipTimeEditorOpen}
                key={subClip.id}
                onPointerCancel={onPointerCancel}
                onPointerDown={(event) => onPointerDown(block, subClip, 'move', event)}
                onPointerMove={onPointerMove}
                onPointerUp={onPointerUp}
                style={subClipSegmentStyle(block, subClip)}
              >
                <span
                  className="weekly-scheduler-subclip-handle weekly-scheduler-subclip-handle--start"
                  onPointerCancel={onPointerCancel}
                  onPointerDown={(event) => onPointerDown(block, subClip, 'start', event)}
                  onPointerMove={onPointerMove}
                  onPointerUp={onPointerUp}
                />
                <span className="weekly-scheduler-subclip-grip" aria-hidden="true" />
                <button
                  aria-label={`Edit ${subClip.label} time from ${timeLabel(subClip.startHour)} to ${timeLabel(
                    subClip.endHour
                  )}`}
                  className="weekly-scheduler-subclip-time"
                  data-hidden={rangeLabelHidden(subClip.startHour, subClip.endHour, hourWidth)}
                  disabled={subClip.disabled === true}
                  onClick={(event) => onTimeEditorOpen(block, subClip, event)}
                  onPointerDown={(event) => event.stopPropagation()}
                  title={`Edit ${subClip.label} time`}
                  type="button"
                >
                  {compactTimeRangeLabel(subClip.startHour, subClip.endHour)}
                </button>
                {subClipTimeEditorOpen ? (
                  <ClipTimeEditor
                    draft={timeEditorDraft}
                    onApply={onTimeEditorApply}
                    onDraftChange={onTimeDraftChange}
                  />
                ) : null}
                <span
                  className="weekly-scheduler-subclip-handle weekly-scheduler-subclip-handle--end"
                  onPointerCancel={onPointerCancel}
                  onPointerDown={(event) => onPointerDown(block, subClip, 'end', event)}
                  onPointerMove={onPointerMove}
                  onPointerUp={onPointerUp}
                />
              </span>
            );
          })}
        </span>
      </span>
    </span>
  );
}

function TimelineNavigator({
  onPointerDown,
  onPointerEnd,
  onPointerMove,
  windowState,
}: {
  readonly onPointerDown: (edge: 'move' | 'start' | 'end', event: ReactPointerEvent<HTMLSpanElement>) => void;
  readonly onPointerEnd: (event: ReactPointerEvent<HTMLSpanElement>) => void;
  readonly onPointerMove: (event: ReactPointerEvent<HTMLSpanElement>) => void;
  readonly windowState: WeeklySchedulerNavigatorState;
}): ReactElement {
  const leftPercent = (windowState.startHour / 24) * 100;
  const widthPercent = ((windowState.endHour - windowState.startHour) / 24) * 100;
  return (
    <div className="weekly-scheduler-navigator" aria-label="Timeline navigator">
      <div className="weekly-scheduler-navigator-rail">
        <span className="weekly-scheduler-navigator-fill" />
        <span
          className="weekly-scheduler-navigator-window"
          onPointerCancel={onPointerEnd}
          onPointerDown={(event) => onPointerDown('move', event)}
          onPointerMove={onPointerMove}
          onPointerUp={onPointerEnd}
          style={{ left: `${leftPercent}%`, width: `${widthPercent}%` }}
        >
          <span
            className="weekly-scheduler-navigator-handle weekly-scheduler-navigator-handle--start"
            onPointerCancel={onPointerEnd}
            onPointerDown={(event) => onPointerDown('start', event)}
            onPointerMove={onPointerMove}
            onPointerUp={onPointerEnd}
          />
          <span
            className="weekly-scheduler-navigator-grip"
            onPointerCancel={onPointerEnd}
            onPointerDown={(event) => onPointerDown('move', event)}
            onPointerMove={onPointerMove}
            onPointerUp={onPointerEnd}
          />
          <span
            className="weekly-scheduler-navigator-handle weekly-scheduler-navigator-handle--end"
            onPointerCancel={onPointerEnd}
            onPointerDown={(event) => onPointerDown('end', event)}
            onPointerMove={onPointerMove}
            onPointerUp={onPointerEnd}
          />
        </span>
      </div>
    </div>
  );
}

function QuickPresetControl({
  activePreset,
  closing,
  onOpenChange,
  onMenuMouseEnter,
  onMenuMouseLeave,
  onPresetSelect,
  onPresetScopeChange,
  onProfileSave,
  open,
  presets,
  presetScope,
  selectedDayLabel,
}: {
  readonly activePreset: WeeklySchedulerPreset | null;
  readonly closing: boolean;
  readonly onMenuMouseEnter: () => void;
  readonly onMenuMouseLeave: () => void;
  readonly onOpenChange: () => void;
  readonly onPresetSelect: (presetId: WeeklySchedulerPresetId) => void;
  readonly onPresetScopeChange: (scope: WeeklySchedulerPresetScope) => void;
  readonly onProfileSave: () => void;
  readonly open: boolean;
  readonly presets: readonly WeeklySchedulerPreset[];
  readonly presetScope: WeeklySchedulerPresetScope;
  readonly selectedDayLabel: string;
}): ReactElement {
  return (
    <div className="weekly-scheduler-preset-control" data-open={open}>
      <button
        aria-expanded={open}
        aria-label={SCHEDULER_COPY.quickPreset}
        className="weekly-scheduler-preset-trigger"
        onClick={onOpenChange}
        type="button"
      >
        <span className="weekly-scheduler-preset-kicker">{SCHEDULER_COPY.quickPreset}</span>
        <span className="weekly-scheduler-preset-name">{activePreset?.label ?? 'Custom'}</span>
        <span className="weekly-scheduler-legend-menu-icon" aria-hidden="true" />
      </button>
      {open || closing ? (
        <div
          className="weekly-scheduler-preset-popover"
          data-state={open ? 'open' : 'closing'}
          onMouseEnter={onMenuMouseEnter}
          onMouseLeave={onMenuMouseLeave}
          role="menu"
        >
          <div className="weekly-scheduler-preset-scope" role="group" aria-label="Preset applies to">
            <button
              className="weekly-scheduler-preset-scope-button"
              data-active={presetScope === 'selected-day'}
              onClick={() => onPresetScopeChange('selected-day')}
              type="button"
            >
              {selectedDayLabel}
            </button>
            <button
              className="weekly-scheduler-preset-scope-button"
              data-active={presetScope === 'week'}
              onClick={() => onPresetScopeChange('week')}
              type="button"
            >
              Week
            </button>
          </div>
          {presets.map((preset) => (
            <button
              aria-checked={preset.id === activePreset?.id}
              className="weekly-scheduler-preset-option"
              data-selected={preset.id === activePreset?.id}
              key={preset.id}
              onClick={() => onPresetSelect(preset.id)}
              role="menuitemradio"
              type="button"
            >
              <span className="weekly-scheduler-choice-circle" />
              <span aria-hidden="true" className="weekly-scheduler-choice-divider">
                |
              </span>
              <span className="weekly-scheduler-preset-option-copy">
                <span className="weekly-scheduler-preset-option-title">{preset.label}</span>
                <span className="weekly-scheduler-preset-option-description">{preset.description}</span>
              </span>
            </button>
          ))}
          <button className="weekly-scheduler-preset-save" onClick={onProfileSave} role="menuitem" type="button">
            Save profile
          </button>
        </div>
      ) : null}
    </div>
  );
}

function LegendControl({
  active,
  activeGroupId,
  closing,
  groups,
  label,
  onGroupChange,
  onControlToggle,
  onMenuMouseEnter,
  onMenuMouseLeave,
  onSwatchClick,
  onSwatchDragEnd,
  onSwatchDragStart,
  onTextClick,
  open,
  selectedControls,
  tone,
}: {
  readonly active: boolean;
  readonly activeGroupId: WeeklySchedulerControlGroupId;
  readonly closing: boolean;
  readonly groups: readonly WeeklySchedulerControlGroup[];
  readonly label: string;
  readonly onGroupChange: (groupId: WeeklySchedulerControlGroupId) => void;
  readonly onControlToggle: (controlId: WeeklySchedulerControlId) => void;
  readonly onMenuMouseEnter: () => void;
  readonly onMenuMouseLeave: () => void;
  readonly onSwatchClick: () => void;
  readonly onSwatchDragEnd: (event: DragEvent<HTMLElement>) => void;
  readonly onSwatchDragStart: (event: DragEvent<HTMLElement>) => void;
  readonly onTextClick: () => void;
  readonly open: boolean;
  readonly selectedControls: readonly WeeklySchedulerControlId[];
  readonly tone: WeeklySchedulerActionId;
}): ReactElement {
  const activeGroup = groups.find((group) => group.id === activeGroupId) ??
    groups[0] ?? { choices: [], id: activeGroupId, label };
  return (
    <div className="weekly-scheduler-legend-item" data-active={active} data-tone={tone}>
      <button
        aria-label={`Drag ${label}`}
        className="weekly-scheduler-legend-swatch"
        draggable
        onClick={onSwatchClick}
        onDragEnd={onSwatchDragEnd}
        onDragStart={onSwatchDragStart}
        type="button"
      />
      <div
        className="weekly-scheduler-legend-text"
        draggable
        onDragEnd={onSwatchDragEnd}
        onDragStart={onSwatchDragStart}
      >
        <span className="weekly-scheduler-legend-badge">{selectedControls.length}</span>
        <span className="weekly-scheduler-legend-name">{legendCountLabel(label, selectedControls.length)}</span>
        <button
          aria-expanded={open}
          aria-label={`${label} options, ${selectedControls.length} selected`}
          className="weekly-scheduler-legend-menu-button"
          draggable={false}
          onClick={onTextClick}
          onDragStart={(event) => event.preventDefault()}
          type="button"
        >
          <span className="weekly-scheduler-legend-menu-icon" aria-hidden="true" />
        </button>
      </div>
      {open || closing ? (
        <div
          className="weekly-scheduler-option-popover"
          data-state={open ? 'open' : 'closing'}
          onMouseEnter={onMenuMouseEnter}
          onMouseLeave={onMenuMouseLeave}
        >
          <div className="weekly-scheduler-option-tabs" role="tablist">
            {groups.map((group) => (
              <button
                aria-selected={group.id === activeGroup.id}
                data-active={group.id === activeGroup.id}
                key={group.id}
                onClick={() => onGroupChange(group.id)}
                role="tab"
                type="button"
              >
                {group.label}
              </button>
            ))}
          </div>
          {activeGroup.choices.map((choice) => (
            <button
              className="weekly-scheduler-choice-toggle"
              data-selected={selectedControls.includes(choice.id)}
              key={choice.id}
              onClick={() => onControlToggle(choice.id)}
              type="button"
            >
              <span className="weekly-scheduler-choice-circle" />
              <span aria-hidden="true" className="weekly-scheduler-choice-divider">
                |
              </span>
              <span>{choice.label}</span>
            </button>
          ))}
        </div>
      ) : null}
    </div>
  );
}

function ModeButton({
  active,
  label,
  onClick,
}: {
  readonly active: boolean;
  readonly label: string;
  readonly onClick: () => void;
}): ReactElement {
  return (
    <button aria-selected={active} data-active={active} onClick={onClick} role="tab" type="button">
      {label}
    </button>
  );
}

function ToolButton({
  active = false,
  children,
  label,
  onClick,
}: {
  readonly active?: boolean;
  readonly children: ReactNode;
  readonly label: string;
  readonly onClick: () => void;
}): ReactElement {
  return (
    <button
      aria-label={label}
      aria-pressed={active}
      className="weekly-scheduler-tool-button"
      data-active={active}
      onClick={onClick}
      title={label}
      type="button"
    >
      {children}
    </button>
  );
}

function FullscreenIcon({ active }: { readonly active: boolean }): ReactElement {
  return (
    <span aria-hidden="true" className="weekly-scheduler-fullscreen-icon" data-active={active}>
      <span className="weekly-scheduler-fullscreen-corner weekly-scheduler-fullscreen-corner--top-left" />
      <span className="weekly-scheduler-fullscreen-corner weekly-scheduler-fullscreen-corner--top-right" />
      <span className="weekly-scheduler-fullscreen-corner weekly-scheduler-fullscreen-corner--bottom-left" />
      <span className="weekly-scheduler-fullscreen-corner weekly-scheduler-fullscreen-corner--bottom-right" />
    </span>
  );
}

function areCompatibleClips(first: WeeklySchedulerBlock, second: WeeklySchedulerBlock): boolean {
  return first.day === second.day && first.tone === second.tone && first.label === second.label;
}

function canInsertClip(
  blocks: readonly WeeklySchedulerBlock[],
  candidate: WeeklySchedulerBlock,
  ignoreId: string | null = null
): boolean {
  return blocks.every(
    (block) =>
      block.id === ignoreId ||
      block.day !== candidate.day ||
      areCompatibleClips(block, candidate) ||
      !rangesOverlap(candidate.startHour, candidate.endHour, block.startHour, block.endHour)
  );
}

function schedulerProfileForPolicyArea(policyArea: WeeklySchedulerPolicyArea): WeeklySchedulerProfile {
  return SCHEDULER_PROFILE_BY_POLICY_AREA[policyArea] ?? SCHEDULER_PROFILE_BY_POLICY_AREA.browser;
}

function presetControlsFromGroups(groups: WeeklySchedulerControlGroupsByTone): WeeklySchedulerPresetControls {
  return {
    allowAll: controlChoicesForTone('allow', groups).map((choice) => choice.id),
    askAll: controlChoicesForTone('ask', groups).map((choice) => choice.id),
    blockAll: groups.block.some((group) => group.choices.some((choice) => choice.id === 'block.all'))
      ? ['block.all']
      : controlChoicesForTone('block', groups).map((choice) => choice.id),
    limitAll: controlChoicesForTone('limit', groups).map((choice) => choice.id),
    observeAll: groups.observe.some((group) => group.choices.some((choice) => choice.id === 'observe.all'))
      ? ['observe.all']
      : controlChoicesForTone('observe', groups).map((choice) => choice.id),
  };
}

function createSchedulerPresets(
  controls: WeeklySchedulerPresetControls,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone,
  noun: WeeklySchedulerPresetNoun
): readonly WeeklySchedulerPreset[] {
  const limitAllBlocks = SCHEDULER_DAY_IDS.map((day) =>
    createLimitPresetBlock('limit-all', day, 'All day limit', controls.limitAll, controlGroupsByTone, () => [
      { endHour: 24, startHour: 0, tone: 'limit' },
    ])
  );
  const limitBalancedBlocks = SCHEDULER_DAY_IDS.map((day) =>
    createLimitPresetBlock(
      'limit-balanced',
      day,
      'Balanced limit',
      controls.limitAll,
      controlGroupsByTone,
      (controlId, controlIndex) => limitBalancedWindowsForControl(day, controlIndex)
    )
  );

  return [
    {
      description: 'Implicit observe for every track.',
      id: 'observe-only',
      label: 'Observe only',
      segments: [],
    },
    {
      description: 'Block midnight to morning and late night.',
      id: 'night-block',
      label: 'Night block',
      segments: [
        { controls: controls.blockAll, endHour: 8, label: 'Night block', startHour: 0, tone: 'block' },
        { controls: controls.blockAll, endHour: 24, label: 'Late block', startHour: 22, tone: 'block' },
      ],
    },
    {
      description: 'Light guardrails with a long allowed window.',
      id: 'relaxed',
      label: 'Relaxed',
      segments: [
        { controls: controls.blockAll, endHour: 7, label: 'Night block', startHour: 0, tone: 'block' },
        { controls: controls.allowAll, endHour: 21, label: 'Allowed window', startHour: 7, tone: 'allow' },
        { controls: controls.askAll, endHour: 23, label: 'Ask parent', startHour: 21, tone: 'ask' },
        { controls: controls.blockAll, endHour: 24, label: 'Bedtime block', startHour: 23, tone: 'block' },
      ],
    },
    {
      description: `Balanced ${noun} controls across school, evening, and bedtime.`,
      id: 'balanced',
      label: 'Medium',
      segments: [
        { controls: controls.blockAll, endHour: 7.5, label: 'Night block', startHour: 0, tone: 'block' },
        { controls: controls.limitAll, endHour: 16, label: 'School limits', startHour: 7.5, tone: 'limit' },
        { controls: controls.allowAll, endHour: 19, label: 'Allowed window', startHour: 16, tone: 'allow' },
        { controls: controls.askAll, endHour: 21.5, label: 'Ask parent', startHour: 19, tone: 'ask' },
        { controls: controls.blockAll, endHour: 24, label: 'Bedtime block', startHour: 21.5, tone: 'block' },
      ],
    },
    {
      description: 'Mostly controlled, small allowed gap, early bedtime.',
      id: 'strict',
      label: 'Strict',
      segments: [
        { controls: controls.blockAll, endHour: 8, label: 'Night block', startHour: 0, tone: 'block' },
        { controls: controls.limitAll, endHour: 18, label: `Limited ${noun}`, startHour: 8, tone: 'limit' },
        { controls: controls.askAll, endHour: 20, label: 'Ask parent', startHour: 18, tone: 'ask' },
        { controls: controls.allowAll, endHour: 20.5, label: 'Short allow', startHour: 20, tone: 'allow' },
        { controls: controls.blockAll, endHour: 24, label: 'Bedtime block', startHour: 20.5, tone: 'block' },
      ],
    },
    {
      blocks: limitAllBlocks,
      description: 'One all-day limit clip with every limit track capped.',
      id: 'limit-all',
      label: 'Limit',
      segments: [],
    },
    {
      blocks: limitBalancedBlocks,
      description: 'All-day limit clips with ask, allow, and capped examples inside each limit track.',
      id: 'limit-balanced',
      label: 'Limit balanced',
      segments: [],
    },
    {
      description: 'One all-day ask-parent clip.',
      id: 'ask-all',
      label: 'Ask all',
      segments: [{ controls: controls.askAll, endHour: 24, label: 'Ask parent', startHour: 0, tone: 'ask' }],
    },
    {
      description: 'One all-day allow clip.',
      id: 'allow-all',
      label: 'Allow all',
      segments: [{ controls: controls.allowAll, endHour: 24, label: 'All allowed', startHour: 0, tone: 'allow' }],
    },
  ];
}

function createBlock(
  id: string,
  day: WeeklySchedulerDay,
  startHour: number,
  tone: WeeklySchedulerActionId,
  label: string,
  selectedControls: readonly WeeklySchedulerControlId[] = [],
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): WeeklySchedulerBlock {
  const endHour = Math.min(24, startHour + 1);
  return {
    id,
    day,
    endHour,
    label,
    startHour,
    subClips: subClipsFromControls(id, tone, selectedControls, startHour, endHour, controlGroupsByTone),
    tone,
  };
}

function createBlockWithRange(
  id: string,
  day: WeeklySchedulerDay,
  startHour: number,
  endHour: number,
  tone: WeeklySchedulerActionId,
  label: string,
  selectedControls: readonly WeeklySchedulerControlId[] = [],
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): WeeklySchedulerBlock {
  const clippedStartHour = clampHalfHour(startHour, 0, 24 - SCHEDULER_MIN_BLOCK_DURATION);
  const clippedEndHour = clampHalfHour(endHour, clippedStartHour + SCHEDULER_MIN_BLOCK_DURATION, 24);
  return {
    day,
    endHour: clippedEndHour,
    id,
    label,
    startHour: clippedStartHour,
    subClips: subClipsFromControls(id, tone, selectedControls, clippedStartHour, clippedEndHour, controlGroupsByTone),
    tone,
  };
}

function blocksForPreset(
  presetId: WeeklySchedulerPresetId,
  presets: readonly WeeklySchedulerPreset[] = SCHEDULER_PRESETS,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): readonly WeeklySchedulerBlock[] {
  const preset = presetById(presetId, presets);
  if (preset === null) {
    return [];
  }
  if (preset.blocks !== undefined) {
    return preset.blocks.map(cloneBlock);
  }
  return SCHEDULER_DAY_IDS.flatMap((day) =>
    preset.segments
      .filter((segment) => segment.days === undefined || segment.days.includes(day))
      .map((segment) => createPresetBlock(preset.id, day, segment, controlGroupsByTone))
  );
}

function blocksForPresetDays(
  presetId: WeeklySchedulerPresetId,
  days: readonly WeeklySchedulerDay[],
  presets: readonly WeeklySchedulerPreset[] = SCHEDULER_PRESETS,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): readonly WeeklySchedulerBlock[] {
  const daySet = new Set(days);
  return blocksForPreset(presetId, presets, controlGroupsByTone).filter((block) => daySet.has(block.day));
}

function replaceBlocksForDays(
  currentBlocks: readonly WeeklySchedulerBlock[],
  days: readonly WeeklySchedulerDay[],
  nextDayBlocks: readonly WeeklySchedulerBlock[]
): readonly WeeklySchedulerBlock[] {
  const daySet = new Set(days);
  return [...currentBlocks.filter((block) => !daySet.has(block.day)), ...nextDayBlocks.map(cloneBlock)].sort(
    compareBlocks
  );
}

function presetById(
  presetId: WeeklySchedulerPresetId,
  presets: readonly WeeklySchedulerPreset[] = SCHEDULER_PRESETS
): WeeklySchedulerPreset | null {
  return presets.find((preset) => preset.id === presetId) ?? null;
}

function cloneBlock(block: WeeklySchedulerBlock): WeeklySchedulerBlock {
  const subClips = block.subClips?.map((subClip) => ({ ...subClip }));
  return subClips === undefined ? { ...block } : { ...block, subClips };
}

function savedPresetsFromStorage(storageKey: string): readonly WeeklySchedulerPreset[] {
  try {
    const rawPresets = window.localStorage.getItem(storageKey);
    if (rawPresets === null) {
      return [];
    }
    const parsedPresets = JSON.parse(rawPresets) as readonly WeeklySchedulerPreset[];
    return Array.isArray(parsedPresets)
      ? parsedPresets.filter((preset) => typeof preset.id === 'string' && Array.isArray(preset.blocks))
      : [];
  } catch {
    return [];
  }
}

function savePresetsToStorage(storageKey: string, presets: readonly WeeklySchedulerPreset[]): void {
  window.localStorage.setItem(storageKey, JSON.stringify(presets));
}

function createLimitPresetBlock(
  presetId: Extract<WeeklySchedulerPresetId, 'limit-all' | 'limit-balanced'>,
  day: WeeklySchedulerDay,
  label: string,
  limitControlIds: readonly WeeklySchedulerControlId[],
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone,
  windowsForControl: (
    controlId: WeeklySchedulerControlId,
    controlIndex: number
  ) => readonly WeeklySchedulerSubClipWindow[]
): WeeklySchedulerBlock {
  const blockId = ['preset', presetId, day, 'all-day-limit'].join('-');
  return {
    day,
    endHour: 24,
    id: blockId,
    label,
    startHour: 0,
    subClips: limitControlIds.flatMap((controlId, controlIndex) =>
      windowsForControl(controlId, controlIndex).map((window, windowIndex) => ({
        controlId,
        disabled: false,
        endHour: window.endHour,
        id: [
          blockId,
          stableDomId(controlId),
          window.tone,
          windowIndex.toString().padStart(2, '0'),
          timeInputValue(window.startHour).replace(':', ''),
          timeInputValue(window.endHour).replace(':', ''),
        ].join('-'),
        label: controlLabel(controlId, controlGroupsByTone),
        startHour: window.startHour,
        tone: window.tone,
      }))
    ),
    tone: 'limit',
  };
}

function limitBalancedWindowsForControl(
  day: WeeklySchedulerDay,
  controlIndex: number
): readonly WeeklySchedulerSubClipWindow[] {
  const weekend = SCHEDULER_WEEKEND_IDS.includes(day);
  const morningEnd = weekend ? 9 : 7.5;
  const dayStart = weekend ? 9 : 7.5;
  const afternoonStart = weekend ? 13 : 15;
  const eveningStart = weekend ? 18 : 19;
  const bedtimeStart = weekend ? 22.5 : 21.5;
  const pattern = controlIndex % 4;

  if (pattern === 0) {
    return [
      { endHour: morningEnd, startHour: 0, tone: 'limit' },
      { endHour: afternoonStart, startHour: dayStart, tone: 'ask' },
      { endHour: eveningStart, startHour: afternoonStart, tone: 'allow' },
      { endHour: 24, startHour: eveningStart, tone: 'limit' },
    ];
  }

  if (pattern === 1) {
    return [
      { endHour: morningEnd + 1, startHour: 0, tone: 'limit' },
      { endHour: eveningStart, startHour: morningEnd + 1, tone: 'allow' },
      { endHour: bedtimeStart, startHour: eveningStart, tone: 'ask' },
      { endHour: 24, startHour: bedtimeStart, tone: 'limit' },
    ];
  }

  if (pattern === 2) {
    return [
      { endHour: dayStart, startHour: 0, tone: 'limit' },
      { endHour: afternoonStart + 1, startHour: dayStart, tone: 'allow' },
      { endHour: bedtimeStart, startHour: afternoonStart + 1, tone: 'ask' },
      { endHour: 24, startHour: bedtimeStart, tone: 'limit' },
    ];
  }

  return [
    { endHour: afternoonStart, startHour: 0, tone: 'limit' },
    { endHour: bedtimeStart - 2, startHour: afternoonStart, tone: 'ask' },
    { endHour: bedtimeStart, startHour: bedtimeStart - 2, tone: 'allow' },
    { endHour: 24, startHour: bedtimeStart, tone: 'limit' },
  ];
}

function createPresetBlock(
  presetId: WeeklySchedulerPresetId,
  day: WeeklySchedulerDay,
  segment: WeeklySchedulerPresetSegment,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): WeeklySchedulerBlock {
  const blockId = [
    'preset',
    presetId,
    day,
    stableDomId(segment.label),
    timeInputValue(segment.startHour).replace(':', ''),
    timeInputValue(segment.endHour).replace(':', ''),
  ].join('-');
  return {
    day,
    endHour: segment.endHour,
    id: blockId,
    label: segment.label,
    startHour: segment.startHour,
    subClips: subClipsFromControls(
      blockId,
      segment.tone,
      segment.controls,
      segment.startHour,
      segment.endHour,
      controlGroupsByTone
    ),
    tone: segment.tone,
  };
}

function gapStyle(gap: { readonly endHour: number; readonly startHour: number }, hourWidth: number): CSSProperties {
  return {
    left: `${gap.startHour * hourWidth}px`,
    width: `${(gap.endHour - gap.startHour) * hourWidth}px`,
  };
}

function fitSubClipsToParent(block: WeeklySchedulerBlock): WeeklySchedulerBlock {
  return {
    ...block,
    subClips: subClipsForBlock(block).map((subClip) => {
      const clippedStartHour = clamp(subClip.startHour, block.startHour, block.endHour);
      const clippedEndHour = clamp(subClip.endHour, block.startHour, block.endHour);
      if (clippedEndHour - clippedStartHour < SCHEDULER_SNAP_HOURS) {
        if (subClip.startHour < block.startHour) {
          return {
            ...subClip,
            endHour: Math.min(block.endHour, block.startHour + SCHEDULER_SNAP_HOURS),
            startHour: block.startHour,
          };
        }
        return {
          ...subClip,
          endHour: block.endHour,
          startHour: Math.max(block.startHour, block.endHour - SCHEDULER_SNAP_HOURS),
        };
      }
      const startHour = snapHour(clippedStartHour, block.startHour, block.endHour - SCHEDULER_SNAP_HOURS);
      const endHour = snapHour(clippedEndHour, startHour + SCHEDULER_SNAP_HOURS, block.endHour);
      return { ...subClip, endHour, startHour };
    }),
  };
}

function mergeSubClips(first: WeeklySchedulerBlock, second: WeeklySchedulerBlock): readonly WeeklySchedulerSubClip[] {
  return [...subClipsForBlock(first), ...subClipsForBlock(second)]
    .sort((left, right) => {
      const leftOrder = subClipControlOrder(first.tone, left.controlId);
      const rightOrder = subClipControlOrder(first.tone, right.controlId);
      return leftOrder === rightOrder ? compareSubClipsForTrack(left, right) : leftOrder - rightOrder;
    })
    .reduce<WeeklySchedulerSubClip[]>((mergedSubClips, subClip) => {
      const previousSubClip = mergedSubClips[mergedSubClips.length - 1];
      if (
        previousSubClip !== undefined &&
        previousSubClip.controlId === subClip.controlId &&
        previousSubClip.label === subClip.label &&
        previousSubClip.disabled === subClip.disabled &&
        previousSubClip.tone === subClip.tone &&
        rangesTouchOrOverlap(previousSubClip.startHour, previousSubClip.endHour, subClip.startHour, subClip.endHour)
      ) {
        mergedSubClips[mergedSubClips.length - 1] = {
          ...previousSubClip,
          endHour: Math.max(previousSubClip.endHour, subClip.endHour),
          startHour: Math.min(previousSubClip.startHour, subClip.startHour),
        };
        return mergedSubClips;
      }
      mergedSubClips.push(subClip);
      return mergedSubClips;
    }, []);
}

function shiftBlockTo(nextBlock: WeeklySchedulerBlock, previousBlock: WeeklySchedulerBlock): WeeklySchedulerBlock {
  const delta = nextBlock.startHour - previousBlock.startHour;
  return {
    ...nextBlock,
    subClips: subClipsForBlock(previousBlock).map((subClip) => {
      const startHour = snapHour(
        subClip.startHour + delta,
        nextBlock.startHour,
        nextBlock.endHour - SCHEDULER_SNAP_HOURS
      );
      const endHour = snapHour(subClip.endHour + delta, startHour + SCHEDULER_SNAP_HOURS, nextBlock.endHour);
      return { ...subClip, endHour, startHour };
    }),
  };
}

function subClipsForBlock(
  block: WeeklySchedulerBlock,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): readonly WeeklySchedulerSubClip[] {
  if (block.subClips !== undefined && block.subClips.length > 0) {
    return block.subClips;
  }
  return gangedSubClipsForAction(block.id, block.tone, block.startHour, block.endHour, controlGroupsByTone);
}

function subClipsFromControls(
  blockId: string,
  tone: WeeklySchedulerActionId,
  selectedControls: readonly WeeklySchedulerControlId[],
  startHour: number,
  endHour: number,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): readonly WeeklySchedulerSubClip[] {
  const allSelected = selectedControls.some((controlId) => controlId.endsWith('.all'));
  const selectedControlSet = new Set(selectedControls);
  return controlChoicesForTone(tone, controlGroupsByTone).map((choice) => ({
    controlId: choice.id,
    disabled: !allSelected && !selectedControlSet.has(choice.id),
    endHour,
    id: `${blockId}-${stableDomId(choice.id)}`,
    label: choice.label,
    startHour,
    tone: defaultSubClipTone(tone),
  }));
}

function gangedSubClipsForAction(
  blockId: string,
  tone: WeeklySchedulerActionId,
  startHour: number,
  endHour: number,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): readonly WeeklySchedulerSubClip[] {
  return controlChoicesForTone(tone, controlGroupsByTone).map((choice) => ({
    controlId: choice.id,
    disabled: true,
    endHour,
    id: `${blockId}-${stableDomId(choice.id)}`,
    label: choice.label,
    startHour,
    tone: defaultSubClipTone(tone),
  }));
}

function controlChoicesForTone(
  tone: WeeklySchedulerActionId,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): readonly WeeklySchedulerControlChoice[] {
  return controlGroupsByTone[tone].flatMap((group) => group.choices.filter((choice) => !choice.id.endsWith('.all')));
}

function subClipTracksForBlock(block: WeeklySchedulerBlock): readonly WeeklySchedulerSubClipTrack[] {
  const groupedTracks = new Map<WeeklySchedulerControlId, WeeklySchedulerSubClipTrack>();
  for (const subClip of subClipsForBlock(block)) {
    const currentTrack = groupedTracks.get(subClip.controlId);
    if (currentTrack === undefined) {
      groupedTracks.set(subClip.controlId, {
        clips: [subClip],
        controlId: subClip.controlId,
        disabled: subClip.disabled === true,
        label: subClip.label,
      });
      continue;
    }
    const clips = [...currentTrack.clips, subClip].sort(compareSubClipsForTrack);
    groupedTracks.set(subClip.controlId, {
      ...currentTrack,
      clips,
      disabled: clips.every((clip) => clip.disabled === true),
    });
  }
  return Array.from(groupedTracks.values()).sort((first, second) => {
    const firstOrder = subClipControlOrder(block.tone, first.controlId);
    const secondOrder = subClipControlOrder(block.tone, second.controlId);
    return firstOrder === secondOrder ? first.label.localeCompare(second.label) : firstOrder - secondOrder;
  });
}

function subClipTrackStyle(
  track: WeeklySchedulerSubClipTrack,
  subClipIndex: number,
  tracks: readonly WeeklySchedulerSubClipTrack[]
): WeeklySchedulerCssVars {
  const laneWeights = tracks.map((item) => subClipLaneWeight(item, tracks.length));
  const laneWeightTotal = laneWeights.reduce((total, weight) => total + weight, 0);
  const laneTopWeight = laneWeights.slice(0, subClipIndex).reduce((total, weight) => total + weight, 0);
  const laneHeight = (laneWeights[subClipIndex] ?? 1) / laneWeightTotal;
  return {
    '--scheduler-subclip-height-share': `${laneHeight * 100}%`,
    '--scheduler-subclip-left': '0%',
    '--scheduler-subclip-index': `${subClipIndex}`,
    '--scheduler-subclip-top': `${(laneTopWeight / laneWeightTotal) * 100}%`,
    '--scheduler-subclip-width': '100%',
  };
}

function subClipSegmentStyle(block: WeeklySchedulerBlock, subClip: WeeklySchedulerSubClip): WeeklySchedulerCssVars {
  const blockDuration = Math.max(SCHEDULER_SNAP_HOURS, block.endHour - block.startHour);
  const startOffset = clamp(subClip.startHour - block.startHour, 0, blockDuration);
  const endOffset = clamp(subClip.endHour - block.startHour, startOffset + SCHEDULER_SNAP_HOURS, blockDuration);
  const leftPercent = (startOffset / blockDuration) * 100;
  const widthPercent = ((endOffset - startOffset) / blockDuration) * 100;
  return {
    '--scheduler-subclip-left': `${leftPercent}%`,
    '--scheduler-subclip-width': `${widthPercent}%`,
  };
}

function subClipPreviewStyle(block: WeeklySchedulerBlock, startHour: number): WeeklySchedulerCssVars {
  const previewEndHour = subClipPreviewEndHour(block, startHour);
  return subClipSegmentStyle(block, {
    controlId: 'preview',
    endHour: previewEndHour,
    id: 'preview',
    label: 'Preview',
    startHour,
  });
}

function subClipPreviewEndHour(block: WeeklySchedulerBlock, startHour: number): number {
  return clamp(startHour + defaultOverrideDuration(block, startHour), startHour, block.endHour);
}

function subClipLaneWeight(track: WeeklySchedulerSubClipTrack, subClipCount: number): number {
  if (subClipCount <= 3) {
    return 1;
  }
  return track.disabled ? 0.58 : 1.45;
}

function subClipTone(block: WeeklySchedulerBlock, subClip: WeeklySchedulerSubClip): WeeklySchedulerActionId {
  return subClip.tone ?? defaultSubClipTone(block.tone);
}

function defaultSubClipTone(parentTone: WeeklySchedulerActionId): WeeklySchedulerActionId {
  return parentTone === 'limit' ? 'allow' : parentTone;
}

function subClipTrackCollapseId(blockId: string, controlId: WeeklySchedulerControlId): string {
  return `${blockId}-${stableDomId(controlId)}-track`;
}

function collapsedSubClipIdsForBlocks(blocks: readonly WeeklySchedulerBlock[]): readonly string[] {
  return blocks.flatMap((block) => collapsedSubClipIdsForBlock(block));
}

function collapsedBlockIdsForBlocks(blocks: readonly WeeklySchedulerBlock[]): readonly string[] {
  return blocks.map((block) => block.id);
}

function collapsedSubClipIdsForBlock(block: WeeklySchedulerBlock): readonly string[] {
  return subClipTracksForBlock(block).map((track) => subClipTrackCollapseId(block.id, track.controlId));
}

function compareSubClipsForTrack(first: WeeklySchedulerSubClip, second: WeeklySchedulerSubClip): number {
  if (first.startHour !== second.startHour) {
    return first.startHour - second.startHour;
  }
  if (first.endHour !== second.endHour) {
    return first.endHour - second.endHour;
  }
  return subClipToneOrder(first.tone) - subClipToneOrder(second.tone);
}

function subClipToneOrder(tone: WeeklySchedulerActionId | undefined): number {
  if (tone === 'block') {
    return 0;
  }
  if (tone === 'ask') {
    return 1;
  }
  if (tone === 'limit') {
    return 2;
  }
  if (tone === 'allow') {
    return 3;
  }
  return 4;
}

function subClipControlOrder(tone: WeeklySchedulerActionId, controlId: WeeklySchedulerControlId): number {
  const controlIndex = controlChoicesForTone(tone).findIndex((choice) => choice.id === controlId);
  return controlIndex === -1 ? Number.MAX_SAFE_INTEGER : controlIndex;
}

function subTrackHourFromClientX(
  block: WeeklySchedulerBlock,
  target: EventTarget & HTMLElement,
  clientX: number
): number {
  const rect = target.getBoundingClientRect();
  const blockDuration = Math.max(SCHEDULER_SNAP_HOURS, block.endHour - block.startHour);
  const rawHour =
    block.startHour + (clamp(clientX - rect.left, 0, rect.width) / Math.max(1, rect.width)) * blockDuration;
  return snapHour(rawHour, block.startHour, block.endHour - SCHEDULER_SNAP_HOURS);
}

function defaultOverrideDuration(block: WeeklySchedulerBlock, startHour: number): number {
  return Math.min(1, Math.max(SCHEDULER_SNAP_HOURS, block.endHour - startHour));
}

function insertSubClipOverride(
  block: WeeklySchedulerBlock,
  targetSubClip: WeeklySchedulerSubClip,
  tone: WeeklySchedulerActionId,
  startHour: number
): WeeklySchedulerBlock {
  const overrideStartHour = snapHour(startHour, block.startHour, block.endHour - SCHEDULER_SNAP_HOURS);
  const overrideEndHour = snapHour(
    overrideStartHour + defaultOverrideDuration(block, overrideStartHour),
    overrideStartHour + SCHEDULER_SNAP_HOURS,
    block.endHour
  );
  const overrideClip: WeeklySchedulerSubClip = {
    controlId: targetSubClip.controlId,
    disabled: false,
    endHour: overrideEndHour,
    id: [
      block.id,
      stableDomId(targetSubClip.controlId),
      tone,
      timeInputValue(overrideStartHour).replace(':', ''),
      timeInputValue(overrideEndHour).replace(':', ''),
    ].join('-'),
    label: targetSubClip.label,
    startHour: overrideStartHour,
    tone,
  };
  const retainedClips = subClipsForBlock(block).flatMap((subClip) => {
    if (subClip.controlId !== targetSubClip.controlId) {
      return [subClip];
    }
    if (subClip.disabled === true) {
      return [];
    }
    if (!rangesOverlap(overrideStartHour, overrideEndHour, subClip.startHour, subClip.endHour)) {
      return [subClip];
    }
    const splitClips: WeeklySchedulerSubClip[] = [];
    if (subClip.startHour < overrideStartHour) {
      splitClips.push({
        ...subClip,
        endHour: overrideStartHour,
        id: `${subClip.id}-before-${timeInputValue(overrideStartHour).replace(':', '')}`,
      });
    }
    if (subClip.endHour > overrideEndHour) {
      splitClips.push({
        ...subClip,
        id: `${subClip.id}-after-${timeInputValue(overrideEndHour).replace(':', '')}`,
        startHour: overrideEndHour,
      });
    }
    return splitClips;
  });
  return {
    ...block,
    subClips: [...retainedClips, overrideClip].sort((first, second) => {
      const firstOrder = subClipControlOrder(block.tone, first.controlId);
      const secondOrder = subClipControlOrder(block.tone, second.controlId);
      return firstOrder === secondOrder ? compareSubClipsForTrack(first, second) : firstOrder - secondOrder;
    }),
  };
}

function mergeClipIntoBlocks(
  blocks: readonly WeeklySchedulerBlock[],
  candidate: WeeklySchedulerBlock
): readonly WeeklySchedulerBlock[] | null {
  if (!canInsertClip(blocks, candidate)) {
    return null;
  }
  return mergeCompatibleBlocks([...blocks, candidate]);
}

function mergeCompatibleBlocks(blocks: readonly WeeklySchedulerBlock[]): readonly WeeklySchedulerBlock[] {
  return closeTinyGaps(blocks)
    .slice()
    .sort(compareBlocks)
    .reduce<WeeklySchedulerBlock[]>((mergedBlocks, block) => {
      const previousBlock = mergedBlocks[mergedBlocks.length - 1];
      if (
        previousBlock !== undefined &&
        areCompatibleClips(previousBlock, block) &&
        rangesTouchOrOverlap(previousBlock.startHour, previousBlock.endHour, block.startHour, block.endHour)
      ) {
        mergedBlocks[mergedBlocks.length - 1] = {
          ...previousBlock,
          endHour: Math.max(previousBlock.endHour, block.endHour),
          startHour: Math.min(previousBlock.startHour, block.startHour),
          subClips: mergeSubClips(previousBlock, block),
        };
        return mergedBlocks;
      }
      mergedBlocks.push(block);
      return mergedBlocks;
    }, []);
}

function selectedIdForMergedClip(blocks: readonly WeeklySchedulerBlock[], candidate: WeeklySchedulerBlock): string {
  return (
    blocks.find(
      (block) =>
        areCompatibleClips(block, candidate) &&
        block.startHour <= candidate.startHour &&
        block.endHour >= candidate.endHour
    )?.id ?? candidate.id
  );
}

function closeTinyGaps(blocks: readonly WeeklySchedulerBlock[]): readonly WeeklySchedulerBlock[] {
  const nextBlocks = blocks
    .slice()
    .sort(compareBlocks)
    .map((block) => ({ ...block }));
  for (let index = 0; index < nextBlocks.length - 1; index += 1) {
    const currentBlock = nextBlocks[index];
    const nextBlock = nextBlocks[index + 1];
    if (currentBlock === undefined || nextBlock === undefined) {
      continue;
    }
    if (currentBlock.day !== nextBlock.day) {
      continue;
    }
    const gap = nextBlock.startHour - currentBlock.endHour;
    if (gap > 0 && gap < SCHEDULER_TINY_GAP_HOURS) {
      currentBlock.endHour = nextBlock.startHour;
    }
  }
  return nextBlocks;
}

function resizeBlocksWithBoundaryTransfer(
  blocks: readonly WeeklySchedulerBlock[],
  blockId: string,
  edge: 'start' | 'end',
  desiredHour: number
): readonly WeeklySchedulerBlock[] {
  const activeBlock = blocks.find((block) => block.id === blockId);
  if (activeBlock === undefined) {
    return blocks;
  }

  if (edge === 'end') {
    const nextBlock = adjacentResizeNeighbor(blocks, activeBlock, 'end');
    if (nextBlock !== null) {
      const boundaryHour = snapHour(
        desiredHour,
        activeBlock.startHour + SCHEDULER_MIN_BLOCK_DURATION,
        nextBlock.endHour - SCHEDULER_MIN_BLOCK_DURATION
      );
      return blocks.map((block) => {
        if (block.id === activeBlock.id) {
          return fitSubClipsToParent({ ...block, endHour: boundaryHour });
        }
        if (block.id === nextBlock.id) {
          return fitSubClipsToParent({ ...block, startHour: boundaryHour });
        }
        return block;
      });
    }
    return blocks.map((block) =>
      block.id === activeBlock.id
        ? fitSubClipsToParent({ ...block, endHour: constrainedEndForResize(blocks, block, desiredHour) })
        : block
    );
  }

  const previousBlock = adjacentResizeNeighbor(blocks, activeBlock, 'start');
  if (previousBlock !== null) {
    const boundaryHour = snapHour(
      desiredHour,
      previousBlock.startHour + SCHEDULER_MIN_BLOCK_DURATION,
      activeBlock.endHour - SCHEDULER_MIN_BLOCK_DURATION
    );
    return blocks.map((block) => {
      if (block.id === previousBlock.id) {
        return fitSubClipsToParent({ ...block, endHour: boundaryHour });
      }
      if (block.id === activeBlock.id) {
        return fitSubClipsToParent({ ...block, startHour: boundaryHour });
      }
      return block;
    });
  }

  return blocks.map((block) =>
    block.id === activeBlock.id
      ? fitSubClipsToParent({ ...block, startHour: constrainedStartForResize(blocks, block, desiredHour) })
      : block
  );
}

function blockWithEditedTimeRange(
  blocks: readonly WeeklySchedulerBlock[],
  activeBlock: WeeklySchedulerBlock,
  desiredStartHour: number,
  desiredEndHour: number
): WeeklySchedulerBlock {
  const blockers = blocks.filter(
    (block) => block.id !== activeBlock.id && block.day === activeBlock.day && !areCompatibleClips(block, activeBlock)
  );
  const minStartHour = blockers
    .filter((block) => block.endHour <= activeBlock.startHour)
    .reduce((maxEndHour, block) => Math.max(maxEndHour, block.endHour), 0);
  const maxEndHour = blockers
    .filter((block) => block.startHour >= activeBlock.endHour)
    .reduce((minStartHour, block) => Math.min(minStartHour, block.startHour), 24);
  const nextStartHour = snapHour(desiredStartHour, minStartHour, maxEndHour - SCHEDULER_MIN_BLOCK_DURATION);
  const nextEndHour = snapHour(desiredEndHour, nextStartHour + SCHEDULER_MIN_BLOCK_DURATION, maxEndHour);
  return fitSubClipsToParent({
    ...activeBlock,
    endHour: nextEndHour,
    startHour: nextStartHour,
  });
}

function subClipWithEditedTimeRange(
  block: WeeklySchedulerBlock,
  activeSubClip: WeeklySchedulerSubClip,
  desiredStartHour: number,
  desiredEndHour: number
): WeeklySchedulerSubClip {
  const siblingClips = subClipsForBlock(block).filter(
    (subClip) =>
      subClip.id !== activeSubClip.id && subClip.controlId === activeSubClip.controlId && subClip.disabled !== true
  );
  const minStartHour = siblingClips
    .filter((subClip) => subClip.endHour <= activeSubClip.startHour)
    .reduce((maxEndHour, subClip) => Math.max(maxEndHour, subClip.endHour), block.startHour);
  const maxEndHour = siblingClips
    .filter((subClip) => subClip.startHour >= activeSubClip.endHour)
    .reduce((minStartHour, subClip) => Math.min(minStartHour, subClip.startHour), block.endHour);
  const nextStartHour = snapHour(desiredStartHour, minStartHour, maxEndHour - SCHEDULER_SNAP_HOURS);
  const nextEndHour = snapHour(desiredEndHour, nextStartHour + SCHEDULER_SNAP_HOURS, maxEndHour);
  return {
    ...activeSubClip,
    disabled: false,
    endHour: nextEndHour,
    startHour: nextStartHour,
  };
}

function adjacentResizeNeighbor(
  blocks: readonly WeeklySchedulerBlock[],
  block: WeeklySchedulerBlock,
  edge: 'start' | 'end'
): WeeklySchedulerBlock | null {
  const candidates = blocks.filter((item) => {
    if (item.id === block.id || item.day !== block.day || areCompatibleClips(item, block)) {
      return false;
    }
    return edge === 'end'
      ? Math.abs(item.startHour - block.endHour) <= SCHEDULER_SNAP_HOURS
      : Math.abs(item.endHour - block.startHour) <= SCHEDULER_SNAP_HOURS;
  });
  if (edge === 'end') {
    return candidates.sort((first, second) => first.startHour - second.startHour)[0] ?? null;
  }
  return candidates.sort((first, second) => second.endHour - first.endHour)[0] ?? null;
}

function constrainedEndForResize(
  blocks: readonly WeeklySchedulerBlock[],
  block: WeeklySchedulerBlock,
  desiredEndHour: number
): number {
  const nextBlockStart = blocks
    .filter(
      (item) =>
        item.id !== block.id &&
        item.day === block.day &&
        item.startHour >= block.startHour &&
        !areCompatibleClips(item, block)
    )
    .reduce((minStart, item) => Math.min(minStart, item.startHour), 24);
  return clampHalfHour(desiredEndHour, block.startHour + SCHEDULER_MIN_BLOCK_DURATION, nextBlockStart);
}

function constrainedStartForMove(
  blocks: readonly WeeklySchedulerBlock[],
  day: WeeklySchedulerDay,
  desiredStartHour: number,
  duration: number,
  ignoreId: string
): number | null {
  const candidates = freeWindowsForDay(blocks, day, ignoreId)
    .filter((window) => window.end - window.start >= duration)
    .map((window) => clampHalfHour(desiredStartHour, window.start, window.end - duration));
  if (candidates.length === 0) {
    return null;
  }
  return candidates.reduce((best, candidate) =>
    Math.abs(candidate - desiredStartHour) < Math.abs(best - desiredStartHour) ? candidate : best
  );
}

function constrainedStartForResize(
  blocks: readonly WeeklySchedulerBlock[],
  block: WeeklySchedulerBlock,
  desiredStartHour: number
): number {
  const previousBlockEnd = blocks
    .filter(
      (item) =>
        item.id !== block.id &&
        item.day === block.day &&
        item.endHour <= block.endHour &&
        !areCompatibleClips(item, block)
    )
    .reduce((maxEnd, item) => Math.max(maxEnd, item.endHour), 0);
  return clampHalfHour(desiredStartHour, previousBlockEnd, block.endHour - SCHEDULER_MIN_BLOCK_DURATION);
}

function freeWindowsForDay(
  blocks: readonly WeeklySchedulerBlock[],
  day: WeeklySchedulerDay,
  ignoreId: string
): readonly { readonly end: number; readonly start: number }[] {
  const sortedBlocks = blocks
    .filter((block) => block.id !== ignoreId && block.day === day)
    .slice()
    .sort((first, second) => first.startHour - second.startHour);
  const windows: { readonly end: number; readonly start: number }[] = [];
  let cursor = 0;
  for (const block of sortedBlocks) {
    if (block.startHour > cursor) {
      windows.push({ end: block.startHour, start: cursor });
    }
    cursor = Math.max(cursor, block.endHour);
  }
  if (cursor < 24) {
    windows.push({ end: 24, start: cursor });
  }
  return windows;
}

function compareBlocks(first: WeeklySchedulerBlock, second: WeeklySchedulerBlock): number {
  const dayDelta = dayIndex(first.day) - dayIndex(second.day);
  return dayDelta !== 0 ? dayDelta : first.startHour - second.startHour;
}

function dayIndex(day: WeeklySchedulerDay): number {
  return SCHEDULER_DAYS.findIndex((item) => item.id === day);
}

function rangesOverlap(firstStart: number, firstEnd: number, secondStart: number, secondEnd: number): boolean {
  return firstStart < secondEnd && firstEnd > secondStart;
}

function rangesTouchOrOverlap(firstStart: number, firstEnd: number, secondStart: number, secondEnd: number): boolean {
  return firstStart <= secondEnd && firstEnd >= secondStart;
}

function trackObserveGaps(
  blocks: readonly WeeklySchedulerBlock[],
  day: WeeklySchedulerDay
): readonly { readonly endHour: number; readonly startHour: number }[] {
  const sortedBlocks = blocks
    .filter((block) => block.day === day)
    .slice()
    .sort((first, second) => first.startHour - second.startHour);
  const gaps: { readonly endHour: number; readonly startHour: number }[] = [];
  let cursor = 0;
  for (const block of sortedBlocks) {
    if (block.startHour > cursor) {
      gaps.push({ endHour: block.startHour, startHour: cursor });
    }
    cursor = Math.max(cursor, block.endHour);
  }
  if (cursor < 24) {
    gaps.push({ endHour: 24, startHour: cursor });
  }
  return gaps;
}

function scheduleBlocksToEdl(
  blocks: readonly WeeklySchedulerBlock[],
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): WeeklySchedulerEdl {
  const explicitItems = blocks.map((block) => blockToEdlItem(block, controlGroupsByTone));
  const implicitItems = SCHEDULER_DAYS.flatMap((day) =>
    trackObserveGaps(blocks, day.id).map((gap) => observeGapToEdlItem(day.id, gap.startHour, gap.endHour))
  );
  return {
    actions: SCHEDULER_LEGENDS.map((legend) => ({
      groups: controlGroupsByTone[legend.tone],
      id: legend.tone,
      label: legend.label,
    })),
    fallbackActionId: SCHEDULER_RESULT.fallbackActionId,
    horizon: {
      endMinute: 24 * 60,
      startMinute: 0,
    },
    items: [...explicitItems, ...implicitItems].sort(compareEdlItems),
    snapMinutes: SCHEDULER_SNAP_MINUTES,
    tracks: SCHEDULER_DAYS,
    version: SCHEDULER_RESULT.version,
  };
}

function blockToEdlItem(
  block: WeeklySchedulerBlock,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): WeeklySchedulerEdlItem {
  const controls = subClipsForBlock(block, controlGroupsByTone).map((subClip) => subClipToEdlControl(block, subClip));
  return {
    actionId: block.tone,
    actionLabel: toneLabel(block.tone),
    controls,
    endMinute: hourToMinute(block.endHour),
    id: block.id,
    label: block.label,
    source: SCHEDULER_EDL_SOURCES.explicit,
    startMinute: hourToMinute(block.startHour),
    trackId: block.day,
    trackLabel: dayLabel(block.day),
  };
}

function observeGapToEdlItem(day: WeeklySchedulerDay, startHour: number, endHour: number): WeeklySchedulerEdlItem {
  return {
    actionId: 'observe',
    actionLabel: toneLabel('observe'),
    controls: [
      {
        actionId: 'observe',
        actionLabel: toneLabel('observe'),
        controlId: SCHEDULER_RESULT.implicitObserveControlId,
        disabled: true,
        endMinute: hourToMinute(endHour),
        label: SCHEDULER_COPY.implicitObserve,
        mode: SCHEDULER_CONTROL_MODES.ganged,
        startMinute: hourToMinute(startHour),
      },
    ],
    endMinute: hourToMinute(endHour),
    id: `${day}-${timeInputValue(startHour).replace(':', '')}-${timeInputValue(endHour).replace(':', '')}-observe`,
    label: SCHEDULER_COPY.observe,
    source: SCHEDULER_EDL_SOURCES.implicit,
    startMinute: hourToMinute(startHour),
    trackId: day,
    trackLabel: dayLabel(day),
  };
}

function subClipToEdlControl(block: WeeklySchedulerBlock, subClip: WeeklySchedulerSubClip): WeeklySchedulerEdlControl {
  const actionId = subClipTone(block, subClip);
  return {
    actionId,
    actionLabel: toneLabel(actionId),
    controlId: subClip.controlId,
    disabled: subClip.disabled === true,
    endMinute: hourToMinute(subClip.endHour),
    label: subClip.label,
    mode: subClip.disabled === true ? SCHEDULER_CONTROL_MODES.ganged : SCHEDULER_CONTROL_MODES.independent,
    startMinute: hourToMinute(subClip.startHour),
  };
}

function compareEdlItems(first: WeeklySchedulerEdlItem, second: WeeklySchedulerEdlItem): number {
  if (first.trackId !== second.trackId) {
    return dayOrder(first.trackId) - dayOrder(second.trackId);
  }
  if (first.startMinute !== second.startMinute) {
    return first.startMinute - second.startMinute;
  }
  if (first.source !== second.source) {
    return first.source === SCHEDULER_EDL_SOURCES.explicit ? -1 : 1;
  }
  return first.endMinute - second.endMinute;
}

function toggleControl(
  current: readonly WeeklySchedulerControlId[],
  controlId: WeeklySchedulerControlId
): readonly WeeklySchedulerControlId[] {
  if (controlId.endsWith('.all')) {
    return current.includes(controlId) ? [] : [controlId];
  }
  const withoutAll = current.filter((item) => !item.endsWith('.all'));
  return withoutAll.includes(controlId) ? withoutAll.filter((item) => item !== controlId) : [...withoutAll, controlId];
}

function blockLabel(
  selectedControls: readonly WeeklySchedulerControlId[],
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): string {
  if (selectedControls.length === 0) {
    return SCHEDULER_COPY.nothing;
  }
  if (selectedControls.length === 1) {
    const selectedControl = selectedControls[0];
    return selectedControl === undefined ? SCHEDULER_COPY.nothing : controlLabel(selectedControl, controlGroupsByTone);
  }
  return `${selectedControls.length} targets`;
}

function legendCountLabel(label: string, count: number): string {
  if (count === 1) {
    return label;
  }
  const pluralLabels: Record<string, string> = {
    Allow: 'Allows',
    Ask: 'Asks',
    Block: 'Blocks',
    Limit: 'Limits',
    Observe: 'Observe',
  };
  return pluralLabels[label] ?? label;
}

function controlLabel(
  controlId: WeeklySchedulerControlId,
  controlGroupsByTone: WeeklySchedulerControlGroupsByTone = SCHEDULER_CONTROL_GROUPS_BY_TONE
): string {
  for (const groups of Object.values(controlGroupsByTone)) {
    for (const group of groups) {
      const choice = group.choices.find((item) => item.id === controlId);
      if (choice !== undefined) {
        return choice.label;
      }
    }
  }
  return SCHEDULER_COPY.browserActivity;
}

function stableDomId(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, '-').replace(/^-|-$/g, '');
}

function classSelector(className: (typeof SCHEDULER_CLASS_NAMES)[keyof typeof SCHEDULER_CLASS_NAMES]): string {
  return `.${className}`;
}

function droppedTone(event: DragEvent): WeeklySchedulerActionId | null {
  event.preventDefault();
  const value =
    event.dataTransfer.getData(SCHEDULER_DRAG_FORMATS.actionId) ||
    event.dataTransfer.getData(SCHEDULER_DRAG_FORMATS.plainText);
  return isSchedulerTone(value) ? value : null;
}

function isSchedulerTone(value: string): value is WeeklySchedulerActionId {
  return value === 'ask' || value === 'allow' || value === 'block' || value === 'limit' || value === 'observe';
}

function toneMenuId(tone: WeeklySchedulerActionId): WeeklySchedulerMenuId {
  return `tone-${tone}`;
}

function toneFromMenuId(menuId: WeeklySchedulerMenuId): WeeklySchedulerActionId | null {
  if (!menuId.startsWith('tone-')) {
    return null;
  }
  const tone = menuId.slice('tone-'.length);
  return isSchedulerTone(tone) ? tone : null;
}

function isSchedulerDay(value: string): value is WeeklySchedulerDay {
  return (
    value === 'mon' ||
    value === 'tue' ||
    value === 'wed' ||
    value === 'thu' ||
    value === 'fri' ||
    value === 'sat' ||
    value === 'sun'
  );
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function qaWidthFromLocation(): number | null {
  const widthValue = new URLSearchParams(window.location.search).get(SCHEDULER_QA_WIDTH_PARAM);
  if (widthValue === null) {
    return null;
  }
  const width = Number(widthValue);
  return Number.isFinite(width) && width > 0 ? Math.round(width) : null;
}

function blockStyle(block: WeeklySchedulerBlock, hourWidth: number): WeeklySchedulerCssVars {
  const subClipCount = subClipsForBlock(block).length;
  const subClipGap = subClipCount > 8 ? 1 : 2;
  return {
    '--scheduler-block-subclip-count': `${subClipCount}`,
    '--scheduler-subclip-gap': `${subClipGap}px`,
    left: `${block.startHour * hourWidth}px`,
    width: `${(block.endHour - block.startHour) * hourWidth}px`,
  };
}

function gridHourLabel(hour: number): string {
  return hour.toString().padStart(2, '0');
}

function blockIdForPlacement(day: WeeklySchedulerDay, startHour: number, tone: WeeklySchedulerActionId): string {
  return `${day}-${timeInputValue(startHour).replace(':', '')}-${tone}`;
}

function dayLabel(day: WeeklySchedulerDay): string {
  return SCHEDULER_DAYS.find((item) => item.id === day)?.label ?? 'Day';
}

function dayOrder(day: WeeklySchedulerDay): number {
  return SCHEDULER_DAYS.findIndex((item) => item.id === day);
}

function toneLabel(tone: WeeklySchedulerActionId): string {
  return SCHEDULER_LEGENDS.find((item) => item.tone === tone)?.label ?? tone;
}

function daySelectionLabel(selectedDays: readonly WeeklySchedulerDay[]): string {
  if (selectedDays.length === SCHEDULER_DAYS.length) {
    return 'All';
  }
  if (selectedDays.length === 1) {
    return dayLabel(selectedDays[0] ?? SCHEDULER_DEFAULT_DAY);
  }
  return `${selectedDays.length} days`;
}

function previewStyle(startHour: number, hourWidth: number): CSSProperties {
  return {
    left: `${startHour * hourWidth}px`,
    width: `${hourWidth}px`,
  };
}

function adjacentDropStartHour(block: WeeklySchedulerBlock, clientX: number, blockElement: HTMLElement): number {
  const rect = blockElement.getBoundingClientRect();
  return clientX < rect.left + rect.width / 2
    ? clampHalfHour(block.startHour - 1, 0, 23)
    : clampHalfHour(block.endHour, 0, 23);
}

function clampHalfHour(value: number, min: number, max: number): number {
  return snapHour(value, min, max);
}

function snapHour(value: number, min: number, max: number): number {
  const snappedValue = Math.round(value / SCHEDULER_SNAP_HOURS) * SCHEDULER_SNAP_HOURS;
  return Number(clamp(snappedValue, min, max).toFixed(4));
}

function timeInputValue(hour: number): string {
  const totalMinutes = Math.round(clamp(hour, 0, 24) * 60);
  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;
  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;
}

function timeLabel(hour: number): string {
  return timeInputValue(hour);
}

function compactTimeRangeLabel(startHour: number, endHour: number): string {
  if (startHour <= 0 && endHour >= 24) {
    return 'All day';
  }
  const start = compactClockLabel(startHour);
  const end = compactClockLabel(endHour);
  if (start.period === end.period) {
    return `${start.value} - ${end.value} ${end.period}`;
  }
  return `${start.value} ${start.period} - ${end.value} ${end.period}`;
}

function compactClockLabel(hour: number): { readonly period: 'AM' | 'PM'; readonly value: string } {
  const totalMinutes = Math.round(clamp(hour, 0, 24) * 60) % (24 * 60);
  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;
  const period = hours < 12 ? 'AM' : 'PM';
  const displayHour = hours % 12 === 0 ? 12 : hours % 12;
  const minuteSuffix = minutes === 0 ? '' : `:${minutes.toString().padStart(2, '0')}`;
  return { period, value: `${displayHour.toString().padStart(2, '0')}${minuteSuffix}` };
}

function rangeLabelHidden(
  startHour: number,
  endHour: number,
  hourWidth: number,
  minWidth = SCHEDULER_TIME_LABEL_MIN_WIDTH
): boolean {
  return Math.max(0, endHour - startHour) * hourWidth < minWidth;
}

function hourToMinute(hour: number): number {
  return Math.round(clamp(hour, 0, 24) * 60);
}

function minuteToHour(minute: number): number {
  return clamp(minute, 0, 24 * 60) / 60;
}

function timeEditorDraftFromRange(startHour: number, endHour: number): WeeklySchedulerTimeEditorDraft {
  return {
    endMinute: hourToMinute(endHour),
    startMinute: hourToMinute(startHour),
  };
}

function timeEditorRangeFromDraft(draft: WeeklySchedulerTimeEditorDraft): {
  readonly endHour: number;
  readonly startHour: number;
} {
  return {
    endHour: minuteToHour(draft.endMinute),
    startHour: minuteToHour(draft.startMinute),
  };
}

function timeEditorMinuteParts(minute: number): { readonly hour: number; readonly minute: number } {
  const normalizedMinute = snapTimeEditorMinute(minute);
  return {
    hour: Math.floor(normalizedMinute / 60),
    minute: normalizedMinute % 60,
  };
}

function timeEditorDraftWithPart(
  draft: WeeklySchedulerTimeEditorDraft,
  edge: WeeklySchedulerTimeEditorEdge,
  part: WeeklySchedulerTimeEditorPart,
  value: string
): WeeklySchedulerTimeEditorDraft {
  const numericValue = Number(value);
  if (!Number.isFinite(numericValue)) {
    return draft;
  }
  const currentMinute = edge === 'start' ? draft.startMinute : draft.endMinute;
  const currentParts = timeEditorMinuteParts(currentMinute);
  const nextHour = part === 'hour' ? Math.trunc(clamp(numericValue, 0, 24)) : currentParts.hour;
  const nextMinute = part === 'minute' ? snapTimeEditorMinute(clamp(numericValue, 0, 55)) : currentParts.minute;
  const nextTotalMinute = snapTimeEditorMinute(nextHour * 60 + (nextHour >= 24 ? 0 : nextMinute));
  return edge === 'start' ? { ...draft, startMinute: nextTotalMinute } : { ...draft, endMinute: nextTotalMinute };
}

function snapTimeEditorMinute(minute: number): number {
  return Math.round(clamp(minute, 0, 24 * 60) / SCHEDULER_SNAP_MINUTES) * SCHEDULER_SNAP_MINUTES;
}

function isBlockTimeEditorTarget(target: WeeklySchedulerTimeEditorTarget | null, blockId: string): boolean {
  return target?.kind === 'block' && target.blockId === blockId;
}

function isSubClipTimeEditorTarget(
  target: WeeklySchedulerTimeEditorTarget | null,
  blockId: string,
  subClipId: string
): boolean {
  return target?.kind === 'subclip' && target.blockId === blockId && target.subClipId === subClipId;
}

function normalizeNavigatorWindow(windowState: WeeklySchedulerNavigatorState): WeeklySchedulerNavigatorState {
  const startHour = clamp(windowState.startHour, 0, 24 - SCHEDULER_NAVIGATOR_MIN_HOURS);
  const endHour = clamp(windowState.endHour, startHour + SCHEDULER_NAVIGATOR_MIN_HOURS, 24);
  if (endHour - startHour >= SCHEDULER_NAVIGATOR_MIN_HOURS) {
    return { endHour, startHour };
  }
  return { endHour: startHour + SCHEDULER_NAVIGATOR_MIN_HOURS, startHour };
}

function timelineWindowForClip(startHour: number, endHour: number): WeeklySchedulerNavigatorState {
  const clipDuration = Math.max(SCHEDULER_SNAP_HOURS, endHour - startHour);
  const windowDuration = clamp(Math.max(SCHEDULER_NAVIGATOR_MIN_HOURS, clipDuration * 1.75), 4, 24);
  const clipCenter = startHour + clipDuration / 2;
  const windowStartHour = clamp(clipCenter - windowDuration / 2, 0, 24 - windowDuration);
  return { endHour: windowStartHour + windowDuration, startHour: windowStartHour };
}

function toggleDaySelection(
  current: readonly WeeklySchedulerDay[],
  day: WeeklySchedulerDay
): readonly WeeklySchedulerDay[] {
  if (current.includes(day)) {
    const next = current.filter((item) => item !== day);
    return next.length === 0 ? [day] : next;
  }
  return SCHEDULER_DAYS.map((item) => item.id).filter((item) => item === day || current.includes(item));
}

function toggleAllDaySelection(current: readonly WeeklySchedulerDay[]): readonly WeeklySchedulerDay[] {
  if (current.length === SCHEDULER_DAYS.length) {
    return [SCHEDULER_DEFAULT_DAY];
  }
  return SCHEDULER_DAYS.map((day) => day.id);
}

function timelineDayFromPoint(clientX: number, clientY: number): WeeklySchedulerDay | null {
  for (const element of document.elementsFromPoint(clientX, clientY)) {
    if (element instanceof HTMLElement && element.dataset['timelineDay'] !== undefined) {
      const timelineDay = element.dataset['timelineDay'];
      return isSchedulerDay(timelineDay) ? timelineDay : null;
    }
  }
  return null;
}

function timelineHourFromClientX(track: HTMLElement, clientX: number, hourWidth: number): number {
  const rect = track.getBoundingClientRect();
  return clampHalfHour((clientX - rect.left) / hourWidth, 0, 23);
}
