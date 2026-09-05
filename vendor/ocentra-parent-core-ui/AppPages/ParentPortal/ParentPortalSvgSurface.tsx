import {
  useCallback,
  useEffect,
  useId,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent,
  type MouseEvent,
  type PointerEvent,
  type ReactElement,
  type ReactNode,
  type WheelEvent,
} from 'react';
import {
  getParentPortalPlaceholderImageUrl,
  parentPortalPlaceholderImageCount,
} from '../../shims/parent-portal-assets-placeholders';
import {
  parentPortalAiSetupQuickActionIconUrl,
  parentPortalBrowserStateQuickActionIconUrl,
  parentPortalDrivesQuickActionIconUrl,
  parentPortalFoldoutClosedIconUrl,
  parentPortalFoldoutOpenIconUrl,
  parentPortalRulesQuickActionIconUrl,
  parentPortalSidePanelHandleLeftIconUrl,
  parentPortalSidePanelHandleRightIconUrl,
  parentPortalSupportApiQuickActionIconUrl,
  parentPortalTodayReportQuickActionIconUrl,
} from '../../shims/parent-portal-assets-common';
import {
  parentPortalAiBannerImageUrl,
  parentPortalBrowserBannerImageUrl,
  parentPortalOverviewBannerImageUrl,
} from '../../shims/parent-portal-assets-banners';
import { normalizeParentPortalSvgControls, type ParentPortalSvgControls } from './ParentPortalSvgSurfaceControls';
import {
  normalizeParentPortalContent,
  type ParentPortalControlArea,
  type ParentPortalGuideNote,
  type ParentPortalGuideTopic,
  type ParentPortalIconName,
  type ParentPortalNavGroup,
  type ParentPortalNavItem,
  type ParentPortalContentData,
  type ParentPortalQuickControl,
  type ParentPortalTabDetail,
  type ParentPortalTabId,
  type ParentPortalTone,
  type PartialParentPortalContentData,
} from './ParentPortalSvgContent';
import {
  createGoldenFrameVariantConfig,
  createGoldenFrameFrameOnlySvgDataUri,
} from './ParentPortalGoldenFrameForeignObject';
import { DeviceChoiceGrid } from './DeviceChoiceGrid/DeviceChoiceGrid';
import type { DeviceChoiceGridProps, DeviceKind, DeviceSlot } from './DeviceChoiceGrid/DeviceChoiceGridTypes';
import {
  createParentPortalActivityUiIntent,
  createParentPortalCanonicalDeviceSlots,
  createParentPortalLanPairingPortalIds,
  createParentPortalLanPairingUiSlots,
  parentPortalActivityAdapterRecord,
} from './activity-ui-intent';
import type { ParentPortalActivityStateLike } from './activity-ui-intent';
import type {
  ParentPortalAppGameDashboardIntent,
  ParentPortalAppGameDashboardMetric,
  ParentPortalAppGameDashboardRow,
  ParentPortalAppGameDashboardTone,
} from './app-game-dashboard-intent';
import type { ParentPortalAppGameSourcePanelSection } from './app-game-source-panel-intent';
import { WeeklySchedulerScratchPage } from './WeeklySchedulerScratchPage';
import { AnimatedSidebarIconButton } from './AnimatedSidebarIconButton';
import { ChatBubbleSvg, estimateChatBubbleHeight } from './ParentPortalChatBubble';
import {
  PortalAgentCommand as AgentCommand,
  PortalAgentActivityReportFrequency,
  PortalAgentActivitySurfaceScopeKind,
  type PortalAgentCommandName as AgentCommandName,
  PortalAgentLanHouseholdActionKind,
  PortalAgentLanHouseholdDeviceKindValues,
  PortalAgentPeerDefaults,
  PortalAgentProtocolField,
  PortalAgentTargetDefaults,
  PortalLanPairingScan,
} from '@ocentra-parent/portal-domain/contracts';
import type { AgentEventId } from '@ocentra-parent/schema-domain/event-primitives';
import {
  PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS,
  type ParentAssistantPortalQuickActionId,
} from '@ocentra-parent/portal-domain/parent-assistant-chat';
import {
  PARENT_PORTAL_GUIDE_QUERY,
  PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES,
  PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS,
} from '@ocentra-parent/portal-domain/parent-portal-guide-controls';
import { parentPortalManageLaneForRoute } from '@ocentra-parent/portal-domain/parent-portal-data';
import {
  defaultManageTargetSelection,
  readStoredManageTargetSelection,
  withManageTargetSelectionDevice,
  writeStoredManageTargetSelection,
  type ManageTargetSelection,
} from '@ocentra-parent/portal-domain/manage-target-selection';
import { portalRouteFromHashPath } from '@ocentra-parent/portal-domain/routes';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalAssets, PortalUnifiedChrome } from '@ocentra-parent/portal-domain/unified-chrome';
import { ParentPortalPanelFrame } from './ParentPortalPanelFrame';
import {
  AccountProfileIcon,
  AiGuideIdeaIcon,
  ActivityNetworkIcon,
  AiMemoryCircuitIcon,
  AiMemorySetBrainIcon,
  AiSetupSearchIcon,
  AlertNotificationBellIcon,
  AppIcon,
  ApiKeysChipIcon,
  AuditCloudLogsIcon,
  BrowserStackIcon,
  DataPrivacyServerShieldIcon,
  DevicesMultiScreenIcon,
  DrivesCloudIcon,
  EnforcementOfficerIcon,
  ExportRetentionIcon,
  GamesIcon,
  GuideBookIcon,
  LanNetworkMonitorsIcon,
  ManageFileSettingsIcon,
  OverviewListIcon,
  PolicyShieldDocumentIcon,
  PortalGatewayIcon,
  QuickGlanceGlasses,
  ReportDocumentIcon,
  RemoteAccessMonitorsIcon,
  RulesGavelDocumentIcon,
  ScheduleCalendarClockIcon,
  ScreenAnalysisIcon,
  StartDataAnalysisIcon,
  TrackingLocationIcon,
  UpdatesSyncDocumentIcon,
  WebGlobeIcon,
  parentNavIconAssetUrls,
} from '../../Common/NavSvgIcons/ParentNavSvgIcons';
import { ScopeToggle } from './ScopeToggle/ScopeToggle';
import {
  BrowserRulesQuestionnaire,
  type BrowserRulesChoiceOption,
  type BrowserRulesQuestion,
} from './BrowserRulesQuestionnaire';
import './ParentPortalSvgSurface.css';

type IconProps = {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  color?: string;
  strokeWidth?: number;
};

type IconComponent = (props: IconProps) => ReactElement;
type Tone = ParentPortalTone;
type DetailMode = 'row' | 'control' | 'season';
type ParentPortalFocusSection = 'highlights' | 'table';
type ParentPortalTopCardTheme = {
  themeColor?: string;
};

type ParentPortalTopCardItem =
  | ({
      kind: 'row';
      key: string;
      row: DisplayRow;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    } & ParentPortalTopCardTheme)
  | ({
      kind: 'control';
      key: string;
      control: ControlArea | QuickControl;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    } & ParentPortalTopCardTheme)
  | ({
      kind: 'guide';
      key: string;
      topic: ParentPortalGuideTopic;
      title: string;
      subtitle: string;
      value: string;
      detail: string;
      tone: Tone;
    } & ParentPortalTopCardTheme);

export type ParentPortalMode = 'parentOverview' | 'parentManage' | 'parentGuide';

function clampNumber(value: number, min: number, max: number): number {
  const lower = Math.min(min, max);
  const upper = Math.max(min, max);
  return Math.min(Math.max(value, lower), upper);
}

export type ParentPortalRow = {
  label: string;
  order: number;
  signalScore: number;
  readyCount?: number;
  gapCount?: number;
  primaryArea?: string;
  trend?: string;
  tone?: Tone;
};

type ParentPortalSvgSurfaceProps = {
  pageMode: ParentPortalMode;
  controlCode: number;
  seasonId: string;
  lastUpdated: string;
  parentPortalRows: ParentPortalRow[];
  userEntry: ParentPortalRow | null;
  nearbyAbove: ParentPortalRow[];
  nearbyBelow: ParentPortalRow[];
  controlId?: string;
  loading?: boolean;
  error?: string | null;
  statusMessage?: string | null;
  controls?: Partial<ParentPortalSvgControls> | null;
  content?: PartialParentPortalContentData | null;
  initialNavLabel?: string;
  initialSelectedControlId?: string;
  assistantRouteActive?: boolean;
  assistantRoutePath?: string;
  assistantReturnRoutePath?: string;
  assistantCommandAvailable?: boolean;
  assistantResponse?: ParentPortalAssistantResponse | null;
  activityState?: ParentPortalActivityState | null;
  lanPairingAutoScanSequence?: number;
  workspaceVisible?: boolean;
  onRefreshParentPortal: (controlCode: number) => void;
  onMatchmaking: () => void;
  onNavigate?: (routePath: string) => boolean | void;
  onAssistantCommand?: (command: AgentCommandName, payload: Record<string, string>) => void;
  onInitialLayoutReady?: () => void;
};

type ParentPortalAssistantResponse = Readonly<{
  eventId: string;
  kind: 'answer' | 'error' | 'unavailable';
  state: string;
  text: string;
}>;

type ParentPortalActivityState = ParentPortalActivityStateLike & {
  ingestStatus?: Record<string, unknown> | null;
  recentSummary?: Record<string, unknown> | null;
  browserEvidenceReadModel?: Record<string, unknown> | null;
  browserManagedStatus?: Record<string, unknown> | null;
  activityMemoryGraphReadModel?: Record<string, unknown> | null;
  browserInterventionReadModel?: Record<string, unknown> | null;
  networkFlowReadModel?: Record<string, unknown> | null;
  lanPairingBrowserDiscoveryEvent?: { eventId?: AgentEventId } | null;
  lanAddDeviceReadModel?: Record<string, unknown> | null;
  screenEvidenceRecentSummary?: Record<string, unknown> | null;
  appGameSessionReport?: Record<string, unknown> | null;
  appGameSessionQueryResult?: Record<string, unknown> | null;
  gameSessionReport?: Record<string, unknown> | null;
  gameSessionQueryResult?: Record<string, unknown> | null;
  policyPreviewReadModel?: Record<string, unknown> | null;
  activityTrackingReadModel?: Parameters<typeof parentPortalActivityAdapterRecord>[0];
};

type NavItem = Omit<ParentPortalNavItem, 'icon'> & {
  icon: IconComponent;
  imageUrl: string;
};

type NavGroup = ParentPortalNavGroup & {
  items: NavItem[];
};

type TabDetail = ParentPortalTabDetail;

type DisplayRow = {
  id: string;
  order: number;
  label: string;
  signal: string;
  signals: string;
  readyCount: string;
  gapCount: string;
  readiness: string;
  primaryArea: string;
  trend: string;
  tone: Tone;
};

type ParentPortalTableVariant = 'statusRows' | 'controls' | 'ai' | 'routines' | 'support' | 'ownership';

type ControlArea = ParentPortalControlArea;

type QuickControl = Omit<ParentPortalQuickControl, 'icon'> & {
  icon: IconComponent;
};

type SelectableControl = ParentPortalControlArea | ParentPortalQuickControl;

type ControlCategorySummary = {
  id: string;
  label: string;
  detail: string;
  count: number;
  tone: Tone;
  sampleControl: QuickControl;
  subcategories: ControlSubcategorySummary[];
};

type ControlSubcategorySummary = {
  id: string;
  label: string;
  count: number;
  tone: Tone;
  sampleControl: QuickControl;
};

const PARENT_PORTAL_RESPONSIVE_MIN_LEFT_W = 210;
const PARENT_PORTAL_RESPONSIVE_MIN_RIGHT_W = 250;
const PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W = 560;
const PARENT_PORTAL_RESPONSIVE_MOBILE_SURFACE_W = 768;
const PARENT_PORTAL_RESPONSIVE_MOBILE_MIN_CANVAS_W = 320;
const PARENT_PORTAL_RESPONSIVE_MOBILE_MIN_CANVAS_H = 900;
const PARENT_PORTAL_RESPONSIVE_COMPACT_SURFACE_W = 1600;
const PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_W = 8192;
const PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_H = 2800;
const PARENT_PORTAL_INITIAL_RENDER_SPINNER_MS = 140;
const PARENT_PORTAL_ROUTE_RENDER_SPINNER_MS = 1000;
const PARENT_PORTAL_SIDE_NAV_FOLD_MS = 220;
const PARENT_PORTAL_TOP_CAROUSEL_MAX_VISIBLE = 5;
const PARENT_PORTAL_SIDE_HANDLE_W = 15;
const PARENT_PORTAL_SIDE_HANDLE_OVERLAP = 1;
const PARENT_PORTAL_CATEGORY_LABELS = [
  'Browser',
  'Policy',
  'Activity',
  'Privacy',
  'Memory',
  'AI',
  'Devices',
  'Support',
] as const;

type AssistantQuickActionId = ParentAssistantPortalQuickActionId;

type AssistantQuickActionIconAssetUrl =
  | typeof parentPortalAiSetupQuickActionIconUrl
  | typeof parentPortalBrowserStateQuickActionIconUrl
  | typeof parentPortalDrivesQuickActionIconUrl
  | typeof parentPortalRulesQuickActionIconUrl
  | typeof parentPortalSupportApiQuickActionIconUrl
  | typeof parentPortalTodayReportQuickActionIconUrl;

type AssistantQuickChoice = {
  readonly label: string;
  readonly promptTemplateId: string;
  readonly prompt: string;
  readonly reply: string;
  readonly followUps: readonly string[];
};

type AssistantQuestionnaireOption = {
  readonly label: string;
  readonly prompt: string;
  readonly choice?: AssistantQuickChoice;
};

type AssistantQuickAction = {
  readonly id: AssistantQuickActionId;
  readonly label: string;
  readonly detail: string;
  readonly starterPromptTemplateId: string;
  readonly prompt: string;
  readonly guide: string;
  readonly reply: string;
  readonly chips: readonly string[];
  readonly choices: readonly AssistantQuickChoice[];
  readonly tone: Tone;
  readonly iconAssetUrl: AssistantQuickActionIconAssetUrl;
};

type AssistantPanelTab = 'history' | 'quickAction';

const ASSISTANT_PANEL_TAB = {
  History: 'history',
  QuickAction: 'quickAction',
} as const satisfies Record<string, AssistantPanelTab>;

type AssistantTranscriptMessage = {
  readonly id: string;
  readonly sender: 'assistant' | 'user';
  readonly text: string;
  readonly action?: AssistantQuickAction | null;
  readonly choices?: readonly AssistantQuickChoice[];
  readonly choiceActionLabel?: string;
};

const ASSISTANT_READY_TEXT = 'Ask MIA about activity, rules, reports, setup, or choose a quick action.';

function assistantReadyMessage(commandAvailable: boolean): AssistantTranscriptMessage {
  return {
    id: 'mia-ready',
    sender: 'assistant',
    text: commandAvailable
      ? `${ASSISTANT_READY_TEXT} Answers appear only after the service returns them.`
      : 'MIA is unavailable because the local service is not connected. Open Start Here to reconnect.',
  };
}

const ASSISTANT_DEFAULT_FOLLOW_UPS = [
  'Give me the overall report',
  'What needs attention?',
  'Explain recent blocks',
  'What should I change first?',
] as const;

const ASSISTANT_QUICK_ACTIONS: readonly AssistantQuickAction[] = PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS.map(
  (action) => ({
    id: action.quickActionId,
    label: action.title,
    detail: action.description,
    starterPromptTemplateId: action.starterPromptTemplateId,
    prompt: action.starterPrompt,
    guide: action.starterGuide,
    reply: action.scaffoldReply,
    chips: action.chips,
    choices: action.choices.map((choice) => ({
      label: choice.label,
      promptTemplateId: choice.promptTemplateId,
      prompt: choice.resolvedPromptPreview,
      reply: choice.assistantReply,
      followUps: choice.followUps,
    })),
    tone: assistantToneForQuickAction(action.quickActionId),
    iconAssetUrl: assistantIconForQuickAction(action.quickActionId),
  })
);

function assistantToneForQuickAction(id: AssistantQuickActionId): Tone {
  if (id === 'overview' || id === 'start' || id === 'report' || id === 'support-api' || id === 'devices') return 'cyan';
  if (id === 'browser-state' || id === 'drives' || id === 'alerts') return 'gold';
  if (id === 'rules' || id === 'private') return 'red';
  return 'purple';
}

function assistantIconForQuickAction(id: AssistantQuickActionId): AssistantQuickActionIconAssetUrl {
  if (id === 'overview' || id === 'start' || id === 'report' || id === 'memory') {
    return parentPortalTodayReportQuickActionIconUrl;
  }
  if (id === 'browser-state') return parentPortalBrowserStateQuickActionIconUrl;
  if (id === 'rules') return parentPortalRulesQuickActionIconUrl;
  if (id === 'ai-setup') return parentPortalAiSetupQuickActionIconUrl;
  if (id === 'drives' || id === 'private' || id === 'devices' || id === 'alerts')
    return parentPortalDrivesQuickActionIconUrl;
  if (id === 'support-api') return parentPortalSupportApiQuickActionIconUrl;
  return parentPortalTodayReportQuickActionIconUrl;
}

function assistantThreadCreatePayload(): Record<string, string> {
  return {
    [PortalAgentProtocolField.ParentAssistantStarterCategory]: 'freeform',
    [PortalAgentProtocolField.ParentAssistantInputSource]: 'quick-action',
  };
}

function assistantQuickActionCommandPayload(action: AssistantQuickAction): Record<string, string> {
  return {
    [PortalAgentProtocolField.ParentAssistantQuickActionId]: action.id,
    [PortalAgentProtocolField.ParentAssistantStarterCategory]: action.id,
    [PortalAgentProtocolField.ParentAssistantPromptTemplateId]: action.starterPromptTemplateId,
    [PortalAgentProtocolField.ParentAssistantInputText]: action.prompt,
    [PortalAgentProtocolField.ParentAssistantInputSource]: 'quick-action',
  };
}

function assistantMessageCommandPayload(
  prompt: string,
  action: AssistantQuickAction | null,
  choice: AssistantQuickChoice | null,
  inputSource: 'typed' | 'choice'
): Record<string, string> {
  const payload: Record<string, string> = {
    [PortalAgentProtocolField.ParentAssistantInputText]: prompt,
    [PortalAgentProtocolField.ParentAssistantInputSource]: inputSource,
  };
  if (action) {
    payload[PortalAgentProtocolField.ParentAssistantQuickActionId] = action.id;
    payload[PortalAgentProtocolField.ParentAssistantStarterCategory] = action.id;
    payload[PortalAgentProtocolField.ParentAssistantPromptTemplateId] =
      choice?.promptTemplateId ?? action.starterPromptTemplateId;
  }
  return payload;
}

function assistantQuickActionById(id: AssistantQuickActionId | null): AssistantQuickAction | null {
  return ASSISTANT_QUICK_ACTIONS.find((action) => action.id === id) ?? null;
}

function assistantQuestionForQuickAction(action: AssistantQuickAction): string {
  if (action.id === 'overview') return 'Overview: what should MIA summarize first?';
  if (action.id === 'start') return 'Start: what setup path should MIA guide?';
  if (action.id === 'report') return 'Report: what kind of report do you need?';
  if (action.id === 'browser-state') return 'Browser: what should MIA inspect first?';
  if (action.id === 'rules') return 'Rules: what do you want to adjust or understand?';
  if (action.id === 'memory') return 'Memory: what context boundary should MIA explain?';
  if (action.id === 'ai-setup') return 'AI setup: what should MIA help configure?';
  if (action.id === 'private') return 'Private: what privacy or custody question matters?';
  if (action.id === 'devices') return 'Devices: what device state should MIA inspect?';
  if (action.id === 'alerts') return 'Alerts: what notification path should MIA shape?';
  if (action.id === 'drives') return 'Data: what should MIA prepare or export?';
  if (action.id === 'support-api') return 'Support: what should MIA check first?';
  return `${action.label}: what should MIA help with?`;
}

function assistantQuestionnaireState(
  action: AssistantQuickAction | null,
  choice: AssistantQuickChoice | null
): { question: string; options: readonly AssistantQuestionnaireOption[] } {
  if (!action) {
    return {
      question: 'Ask:',
      options: ASSISTANT_DEFAULT_FOLLOW_UPS.map((prompt) => ({ label: prompt, prompt })),
    };
  }

  if (choice?.followUps.length) {
    return {
      question: `${choice.label}: what detail should MIA narrow next?`,
      options: choice.followUps.map((prompt) => ({ label: prompt, prompt })),
    };
  }

  return {
    question: assistantQuestionForQuickAction(action),
    options: action.choices.map((quickChoice) => ({
      label: quickChoice.label,
      prompt: quickChoice.prompt,
      choice: quickChoice,
    })),
  };
}

const ASSISTANT_NAV_ROUTE_PREFIX = 'assistant-action:';

const ASSISTANT_NAV_GROUP_ACTION_IDS = {
  quickGlance: ['overview'],
  guide: ['start', 'rules', 'memory', 'ai-setup', 'report', 'private'],
  manage: ['browser-state', 'devices', 'alerts', 'drives', 'support-api'],
} as const;

const ASSISTANT_NEW_CHAT_NAV_ITEM = {
  label: 'NEW CHAT',
  detail: 'Start fresh with MIA',
  icon: AiGuideIdeaIcon,
  imageUrl: '',
  tabId: 'aiStatus',
  groupId: 'quickGlance',
  tone: 'cyan',
  routePath: `${ASSISTANT_NAV_ROUTE_PREFIX}new-chat`,
} satisfies NavItem;

function assistantHistoryNavItem(action: AssistantQuickAction): NavItem {
  return {
    ...assistantQuickActionNavItem(action, ASSISTANT_PANEL_TAB.History),
    label: `${action.label.toUpperCase()} HISTORY`,
    detail: `Review previous ${action.label.toLowerCase()} assistant prompts.`,
  };
}

function assistantQuickActionNavGroups(navGroups: NavGroup[]): NavGroup[] {
  return navGroups
    .map((group) => {
      const actionIds = ASSISTANT_NAV_GROUP_ACTION_IDS[group.id as keyof typeof ASSISTANT_NAV_GROUP_ACTION_IDS];
      if (!actionIds) return null;
      return {
        ...group,
        items: actionIds
          .map((actionId) => assistantQuickActionById(actionId))
          .filter((action): action is AssistantQuickAction => action !== null)
          .map((action) => assistantQuickActionNavItem(action, group.id)),
      };
    })
    .filter((group): group is NavGroup => group !== null && group.items.length > 0);
}

function assistantQuickActionNavItem(action: AssistantQuickAction, groupId: string): NavItem {
  return {
    label: action.label.toUpperCase(),
    detail: action.detail,
    icon: assistantIconComponentForQuickAction(action.id),
    imageUrl: action.iconAssetUrl,
    tabId: 'aiStatus',
    groupId,
    tone: action.tone,
    routePath: `${ASSISTANT_NAV_ROUTE_PREFIX}${action.id}`,
  };
}

function assistantActionIdForNavItem(item: NavItem): AssistantQuickActionId | null {
  const routePath = item.routePath ?? '';
  if (!routePath.startsWith(ASSISTANT_NAV_ROUTE_PREFIX)) return null;
  const actionId = routePath.slice(ASSISTANT_NAV_ROUTE_PREFIX.length);
  return ASSISTANT_QUICK_ACTIONS.some((action) => action.id === actionId) ? (actionId as AssistantQuickActionId) : null;
}

function assistantIconComponentForQuickAction(id: AssistantQuickActionId): IconComponent {
  if (id === 'overview') return OverviewListIcon;
  if (id === 'start') return StartDataAnalysisIcon;
  if (id === 'report') return ReportDocumentIcon;
  if (id === 'browser-state') return BrowserStackIcon;
  if (id === 'rules') return RulesGavelDocumentIcon;
  if (id === 'memory') return AiMemoryCircuitIcon;
  if (id === 'ai-setup') return AiMemorySetBrainIcon;
  if (id === 'private') return DataPrivacyServerShieldIcon;
  if (id === 'devices') return DevicesMultiScreenIcon;
  if (id === 'alerts') return AlertNotificationBellIcon;
  if (id === 'drives') return DrivesCloudIcon;
  if (id === 'support-api') return AccountProfileIcon;
  return AiGuideIdeaIcon;
}

const ASSISTANT_INCOMING_CHAT_BUBBLE_CONFIG = {
  body: {
    clampOnLeft: true,
    minHeight: 50,
    maxHeight: 150,
  },
  colors: {
    aiBodyTop: '#c8f8d6',
    aiBodyBottom: '#a9ecc1',
  },
  text: {
    fontSize: 12,
    lineHeight: 1.42,
  },
};

const ASSISTANT_OUTGOING_CHAT_BUBBLE_CONFIG = {
  body: {
    clampOnLeft: false,
    minHeight: 42,
    maxHeight: 118,
  },
  colors: {
    userBodyTop: '#e5f6ff',
    userBodyBottom: '#acdfff',
  },
  text: {
    fontSize: 12,
    lineHeight: 1.42,
  },
};

const ASSISTANT_CHAT_SURFACE_FILL = 'rgba(2, 12, 20, 0.52)';
const ASSISTANT_QUESTIONNAIRE_SURFACE_FILL = ASSISTANT_CHAT_SURFACE_FILL;
const ASSISTANT_QUESTIONNAIRE_BODY_FILL = 'rgba(7, 26, 42, 0.34)';
const PARENT_PORTAL_TAB_SURFACE_FILL = {
  active: 'rgba(2, 12, 22, 0.74)',
  hover: 'rgba(2, 12, 22, 0.52)',
  idle: 'rgba(2, 12, 22, 0.38)',
  lanActive: 'rgba(2, 12, 22, 0.78)',
  lanIdle: 'rgba(2, 12, 22, 0.48)',
  lanMuted: 'rgba(2, 12, 22, 0.34)',
} as const;
const ASSISTANT_FOLLOW_UP_PAD_X = 24;
const ASSISTANT_FOLLOW_UP_PAD_Y = 6;
const ASSISTANT_FOLLOW_UP_HEADER_H = 24;
const ASSISTANT_FOLLOW_UP_ROW_H = 32;
const ASSISTANT_FOLLOW_UP_GAP = 10;

const ASSISTANT_SIDE_PANEL_ICON_CONFIG = {
  colors: {
    frame: '#cffbff',
    frameHover: '#ffffff',
    frameOutline: '#f0ffff',
    border: '#38dfff',
    borderHover: '#c8fbff',
    outerGlow: '#38dfff',
    outerGlowSoft: '#0ea5e9',
    panelTop: '#e4fdff',
    panelMid: '#38c8ff',
    panelBottom: '#126aa8',
    panelHoverTop: '#ffffff',
    panelHoverMid: '#68e4ff',
    panelHoverBottom: '#1684c8',
    panelGlow: '#67e8f9',
  },
  outerBorder: {
    opacityIdle: 0.86,
    opacityOpen: 1,
  },
  doc: {
    outlineOpacityIdle: 0.66,
    outlineOpacityHover: 0.92,
  },
  panel: {
    outlineOpacityIdle: 0.72,
    outlineOpacityOpen: 0.9,
    topShineOpacityIdle: 0.84,
    topShineOpacityOpen: 0.9,
    edgeOpacityIdle: 0.72,
    edgeOpacityOpen: 0.84,
  },
  filters: {
    outerGlow: {
      opacityAIdle: 0.76,
      opacityAOpen: 0.98,
      opacityBIdle: 0.34,
      opacityBOpen: 0.52,
    },
    panelGlow: {
      glowOpacityIdle: 0.58,
      glowOpacityOpen: 0.74,
    },
  },
} as const;

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

function compactControlStatLabel(value: string): string {
  const text = value.trim().replace(/\s+/g, ' ');
  if (!text) return '';
  if (/^\d+(?:\.\d+)?\s*[kmb]$/i.test(text) || /^[+-]?\d+(?:\.\d+)?%$/.test(text)) return text;
  const range = text.match(/\b\d+\s*[-–]\s*\d+\b/);
  if (range) return range[0].replace(/\s+/g, '');
  const number = text.match(/\b\d+\b/);
  return number ? number[0] : text;
}

function minimumParentPortalCanvasWidth(cfg: ParentPortalSvgControls): number {
  return Math.max(
    cfg.canvas.width,
    Math.ceil(
      cfg.layout.outerPad * 2 +
        PARENT_PORTAL_RESPONSIVE_MIN_LEFT_W +
        PARENT_PORTAL_RESPONSIVE_MIN_RIGHT_W +
        PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W +
        cfg.layout.gap * 2
    )
  );
}

function parentPortalCanvasWidthForSurface(
  cfg: ParentPortalSvgControls,
  surfaceSize: { width: number; height: number }
): number {
  if (surfaceSize.width <= 0 || surfaceSize.height <= 0) return cfg.canvas.width;
  const minimumWidth = minimumParentPortalCanvasWidth(cfg);
  const ratioWidth = Math.round(cfg.canvas.height * (surfaceSize.width / surfaceSize.height));
  return Math.max(minimumWidth, Math.min(PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_W, ratioWidth));
}

function compactParentPortalCanvasWidth(cfg: ParentPortalSvgControls): number {
  return Math.ceil(cfg.layout.outerPad * 2 + cfg.layout.leftW + cfg.layout.gap + PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W);
}

function isParentPortalMobileSurface(surfaceSize: { width: number; height: number }): boolean {
  return surfaceSize.width > 0 && surfaceSize.width < PARENT_PORTAL_RESPONSIVE_MOBILE_SURFACE_W;
}

function parentPortalCanvasSizeForSurface(
  cfg: ParentPortalSvgControls,
  surfaceSize: { width: number; height: number }
): { width: number; height: number } {
  if (surfaceSize.width <= 0 || surfaceSize.height <= 0) {
    return cfg.canvas;
  }

  if (isParentPortalMobileSurface(surfaceSize)) {
    return {
      width: Math.max(PARENT_PORTAL_RESPONSIVE_MOBILE_MIN_CANVAS_W, Math.round(surfaceSize.width)),
      height: Math.max(PARENT_PORTAL_RESPONSIVE_MOBILE_MIN_CANVAS_H, cfg.canvas.height),
    };
  }

  if (surfaceSize.width >= PARENT_PORTAL_RESPONSIVE_COMPACT_SURFACE_W) {
    const width = parentPortalCanvasWidthForSurface(cfg, surfaceSize);
    const aspectHeight = Math.round(width * (surfaceSize.height / surfaceSize.width));
    return {
      width,
      height: clampValue(aspectHeight, cfg.canvas.height, PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_H),
    };
  }

  const compactWidth = compactParentPortalCanvasWidth(cfg);
  const aspectWidth = Math.round(cfg.canvas.height * (surfaceSize.width / surfaceSize.height));
  const width = clampValue(aspectWidth, compactWidth, cfg.canvas.width);
  const aspectHeight = Math.round(width * (surfaceSize.height / surfaceSize.width));

  return {
    width,
    height: clampValue(aspectHeight, cfg.canvas.height, PARENT_PORTAL_RESPONSIVE_MAX_CANVAS_H),
  };
}

function responsiveParentPortalColumnWidths(
  canvasWidth: number,
  cfg: ParentPortalSvgControls
): { leftW: number; mainW: number; rightW: number } {
  if (canvasWidth < PARENT_PORTAL_RESPONSIVE_MOBILE_SURFACE_W) {
    const mainW = Math.max(1, canvasWidth - cfg.layout.outerPad * 2);
    return { leftW: 0, mainW, rightW: 0 };
  }
  const availableW = canvasWidth - cfg.layout.outerPad * 2 - cfg.layout.gap;
  const leftW = clampValue(cfg.layout.leftW, PARENT_PORTAL_RESPONSIVE_MIN_LEFT_W, cfg.layout.leftW);
  const mainW = Math.max(PARENT_PORTAL_RESPONSIVE_MIN_MAIN_W, availableW - leftW);
  return { leftW, mainW, rightW: 0 };
}

const iconByName: Record<ParentPortalIconName, IconComponent> = {
  'quick-glance': QuickGlanceGlasses,
  overview: OverviewListIcon,
  start: StartDataAnalysisIcon,
  guide: GuideBookIcon,
  manage: ManageFileSettingsIcon,
  policy: PolicyShieldDocumentIcon,
  browser: BrowserStackIcon,
  chat: AiGuideIdeaIcon,
  web: WebGlobeIcon,
  schedule: ScheduleCalendarClockIcon,
  alerts: AlertNotificationBellIcon,
  report: ReportDocumentIcon,
  rules: RulesGavelDocumentIcon,
  updates: UpdatesSyncDocumentIcon,
  activity: ActivityNetworkIcon,
  app: AppIcon,
  games: GamesIcon,
  portal: PortalGatewayIcon,
  privacy: DataPrivacyServerShieldIcon,
  lan: LanNetworkMonitorsIcon,
  devices: DevicesMultiScreenIcon,
  screen: ScreenAnalysisIcon,
  remote: RemoteAccessMonitorsIcon,
  'ai-setup': AiMemorySetBrainIcon,
  'ai-guide': AiMemorySetBrainIcon,
  'ai-memory-set': AiMemorySetBrainIcon,
  api: ApiKeysChipIcon,
  export: ExportRetentionIcon,
  drives: DrivesCloudIcon,
  audit: AuditCloudLogsIcon,
  'ai-memory': AiMemorySetBrainIcon,
  account: AccountProfileIcon,
  enforcement: EnforcementOfficerIcon,
};

function iconForName(icon: ParentPortalIconName): IconComponent {
  return iconByName[icon] ?? OverviewListIcon;
}

function iconForNavItem(item: ParentPortalNavItem): IconComponent {
  if (assetKey(`${item.label} ${item.routePath ?? ''}`).includes('tracking')) {
    return TrackingLocationIcon;
  }
  return iconForName(item.icon);
}

function toneColor(tone: Tone, cfg: ParentPortalSvgControls): string {
  return cfg.colors[tone];
}

function colorAlpha(color: string, alphaHex: string): string {
  return color.startsWith('#') ? `${color}${alphaHex}` : color;
}

const PARENT_PORTAL_GLASS = {
  panelFill: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  panelFillSoft: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  panelFillStrong: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  panelFillDeep: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  cardFill: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  cardFillStrong: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  controlFill: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  dialogFill: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  dialogFillStrongTop: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  dialogFillStrongBottom: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFill,
  dialogScrim: PortalUnifiedChrome.CssVarRefs.FrameScrimFill,
} as const;

const PARENT_PORTAL_CONTENT_SURFACE_OPACITY = 0.94;

const PARENT_PORTAL_FRAME_MATERIAL = {
  bodyFill: PortalUnifiedChrome.CssVarRefs.FrameBodyFill,
  bodyStrokeOpacity: PortalUnifiedChrome.CssVarRefs.FrameBodyStrokeOpacity,
  disabledFillOpacity: PortalUnifiedChrome.CssVarRefs.FrameSurfaceDisabledFillOpacity,
  fillOpacity: PortalUnifiedChrome.CssVarRefs.FrameSurfaceFillOpacity,
  footerLineOpacity: PortalUnifiedChrome.CssVarRefs.FrameFooterLineOpacity,
  headerLineOpacity: PortalUnifiedChrome.CssVarRefs.FrameHeaderLineOpacity,
  transparentFill: PortalUnifiedChrome.CssVarRefs.FrameTransparentFill,
} as const;

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

function rowAvatarImageUrl(label: string): string {
  const imageIndex =
    parentPortalPlaceholderImageCount > 0 ? hashString(`row:${label}`) % parentPortalPlaceholderImageCount : 0;
  return getParentPortalPlaceholderImageUrl(imageIndex);
}

function parentPortalControlImageUrl(value?: string): string | null {
  const key = assetKey(value);
  if (!key) return null;
  if (key.includes('ai-benchmark') || key.includes('model')) return parentPortalAiBannerImageUrl;
  if (key.includes('parent-overview') || key.includes('quick-access') || key.includes('hub'))
    return parentPortalOverviewBannerImageUrl;
  if (
    key.includes('all-controls') ||
    key.includes('catalog') ||
    key.includes('browser') ||
    key.includes('web') ||
    key.includes('device') ||
    key.includes('lan')
  ) {
    return parentPortalBrowserBannerImageUrl;
  }
  return parentPortalOverviewBannerImageUrl;
}

function parentPortalControlArtworkUrl(control: Pick<ControlArea | QuickControl, 'id' | 'name'>): string {
  const key = control.id || control.name;
  const imageIndex = parentPortalPlaceholderImageCount > 0 ? hashString(key) % parentPortalPlaceholderImageCount : 0;
  return getParentPortalPlaceholderImageUrl(imageIndex);
}

function parentPortalControlCategoryImageUrl(category: ControlCategorySummary): string | null {
  return category.count > 0 ? parentPortalControlArtworkUrl(category.sampleControl) : null;
}

function navItemImageUrl(item: ParentPortalNavItem): string {
  const key = assetKey(item.label);
  if (key.includes('overview')) return parentPortalOverviewBannerImageUrl;
  if (key.includes('overall') || key.includes('global') || key.includes('family'))
    return parentPortalOverviewBannerImageUrl;
  if (key.includes('ai')) return parentPortalAiBannerImageUrl;
  if (
    key.includes('category') ||
    key.includes('control') ||
    key.includes('device') ||
    key.includes('browser') ||
    key.includes('web')
  ) {
    return parentPortalBrowserBannerImageUrl;
  }
  return parentPortalOverviewBannerImageUrl;
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

function topRoundedRectPath(x: number, y: number, w: number, h: number, radius: number) {
  const r = Math.min(radius, w / 2, h / 2);
  return [
    `M ${x} ${y + h}`,
    `V ${y + r}`,
    `C ${x} ${y + r * 0.45} ${x + r * 0.45} ${y} ${x + r} ${y}`,
    `H ${x + w - r}`,
    `C ${x + w - r * 0.45} ${y} ${x + w} ${y + r * 0.45} ${x + w} ${y + r}`,
    `V ${y + h}`,
    `H ${x}`,
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

type ParentPortalRect = { x: number; y: number; w: number; h: number };

function parentPortalFrameRects(
  x: number,
  y: number,
  w: number,
  h: number,
  footerH = 38,
  headerH = 48,
  bodyInset = 32
): { body: ParentPortalRect; footer: ParentPortalRect; headerH: number; footerH: number } {
  return {
    body: {
      x: x + bodyInset,
      y: y + headerH + 10,
      w: Math.max(1, w - bodyInset * 2),
      h: Math.max(1, h - headerH - footerH - 18),
    },
    footer: {
      x: x + bodyInset,
      y: y + h - footerH,
      w: Math.max(1, w - bodyInset * 2),
      h: footerH,
    },
    headerH,
    footerH,
  };
}

function formatRouteScope(value?: string): string {
  if (!value) return 'All controls';
  return value
    .split(/[/:_-]/)
    .filter(Boolean)
    .slice(0, 3)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function initialTabForPageMode(pageMode: ParentPortalMode, content: ParentPortalContentData): ParentPortalTabId {
  return content.modes[pageMode]?.defaultTab ?? 'overall';
}

function initialNavLabelForTab(navItems: NavItem[], tab: ParentPortalTabId): string {
  return navItems.find((item) => item.tabId === tab)?.label ?? navItems[0]?.label ?? '';
}

function groupedParentPortalNavItems(navGroups: ParentPortalNavGroup[], navItems: NavItem[]): NavGroup[] {
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

function navSectionsForGroup(group: NavGroup): Array<{ id: string; label: string; items: NavItem[] }> {
  const sections: Array<{ id: string; label: string; items: NavItem[] }> = [];
  const sectionByLabel = new Map<string, { id: string; label: string; items: NavItem[] }>();
  let unsectionedIndex = 0;
  for (const item of group.items) {
    const label = item.sectionLabel ?? '';
    if (!label) {
      let unsectioned = sections.at(-1);
      if (unsectioned?.label) {
        unsectioned = undefined;
      }
      if (!unsectioned) {
        unsectioned = { id: `${group.id}:items:${unsectionedIndex}`, label: '', items: [] };
        unsectionedIndex += 1;
        sections.push(unsectioned);
      }
      unsectioned.items.push(item);
      continue;
    }
    let section = sectionByLabel.get(label);
    if (!section) {
      section = { id: navSectionId(group.id, label), label, items: [] };
      sectionByLabel.set(label, section);
      sections.push(section);
    }
    section.items.push(item);
  }
  return sections;
}

function navSectionIcon(section: { label: string; items: NavItem[] }): IconComponent {
  const key = assetKey(section.label);
  if (key.includes('polic')) return PolicyShieldDocumentIcon;
  if (key.includes('activity')) return ActivityNetworkIcon;
  if (key.includes('portal')) return PortalGatewayIcon;
  if (key === 'data' || key.includes('data-privacy')) return DataPrivacyServerShieldIcon;
  if (key === 'ai' || key.includes('ai-memory')) return AiMemorySetBrainIcon;
  if (key.includes('account')) return AccountProfileIcon;
  if (key.includes('remote')) return RemoteAccessMonitorsIcon;
  if (key.includes('lan')) return LanNetworkMonitorsIcon;
  if (key.includes('device')) return DevicesMultiScreenIcon;
  return section.items[0]?.icon ?? OverviewListIcon;
}

function navItemKey(item: Pick<NavItem, 'label' | 'routePath'>): string {
  return item.routePath || item.label;
}

function navItemMatches(item: NavItem, navKey: string, navLabel: string): boolean {
  return navKey ? navItemKey(item) === navKey : item.label === navLabel;
}

function navGroupIdForNavKey(navGroups: NavGroup[], navKey: string, navLabel: string): string {
  return navGroups.find((group) => group.items.some((item) => navItemMatches(item, navKey, navLabel)))?.id ?? '';
}

function initialOpenNavGroupIds(navGroups: NavGroup[], navKey: string, navLabel: string): Record<string, boolean> {
  const activeGroupId = navGroupIdForNavKey(navGroups, navKey, navLabel);
  return Object.fromEntries(
    navGroups.map((group, index) => [group.id, group.id === activeGroupId || (!activeGroupId && index === 0)])
  );
}

function navSectionId(groupId: string, sectionLabel: string): string {
  return `${groupId}:${sectionLabel}`;
}

function navSectionIdsForGroups(navGroups: NavGroup[]): string[] {
  return navGroups.flatMap((group) => {
    const labels = new Set(group.items.map((item) => item.sectionLabel).filter(Boolean));
    return Array.from(labels, (label) => navSectionId(group.id, label as string));
  });
}

function navSectionIdForNavKey(navGroups: NavGroup[], navKey: string, navLabel: string): string | null {
  for (const group of navGroups) {
    const item = group.items.find((entry) => navItemMatches(entry, navKey, navLabel));
    if (item?.sectionLabel) return navSectionId(group.id, item.sectionLabel);
  }
  return null;
}

function initialOpenNavSectionIds(navGroups: NavGroup[], navKey: string, navLabel: string): Record<string, boolean> {
  const activeSectionId = navSectionIdForNavKey(navGroups, navKey, navLabel);
  return Object.fromEntries(
    navGroups.flatMap((group) => {
      const labels = Array.from(new Set(group.items.map((item) => item.sectionLabel).filter(Boolean)));
      return labels.map((label) => {
        const sectionId = navSectionId(group.id, label as string);
        return [sectionId, sectionId === activeSectionId];
      });
    })
  );
}

function ensureOpenNavGroupIds(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  navKey: string,
  navLabel: string
): Record<string, boolean> {
  const activeGroupId = navGroupIdForNavKey(navGroups, navKey, navLabel);
  return Object.fromEntries(
    navGroups.map((group) => [group.id, activeGroupId ? group.id === activeGroupId : Boolean(current[group.id])])
  );
}

function toggleOpenNavGroupId(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  groupId: string
): Record<string, boolean> {
  const nextOpen = !current[groupId];
  return Object.fromEntries(
    navGroups.map((group) => [
      group.id,
      group.id === groupId ? nextOpen : nextOpen ? false : Boolean(current[group.id]),
    ])
  );
}

function toggleOpenNavSectionId(
  current: Record<string, boolean>,
  navGroups: NavGroup[],
  sectionId: string
): Record<string, boolean> {
  const nextOpen = !current[sectionId];
  return Object.fromEntries(
    navSectionIdsForGroups(navGroups).map((entryId) => [
      entryId,
      entryId === sectionId ? nextOpen : nextOpen ? false : Boolean(current[entryId]),
    ])
  );
}

function normalizeSelectionId(value?: string): string {
  return (value ?? '').trim().toLowerCase();
}

function findSelectedControl(
  content: ParentPortalContentData,
  selectedControlId: string
): SelectableControl | undefined {
  const normalizedId = normalizeSelectionId(selectedControlId);
  return [...content.controlAreas, ...content.quickControls.filter(isParentPortalControlEntry)].find(
    (control) => normalizeSelectionId(control.id) === normalizedId
  );
}

function initialNavItemForContext(
  navItems: NavItem[],
  content: ParentPortalContentData,
  initialNavLabel: string | undefined,
  selectedControlId: string | undefined,
  preferredTabId: ParentPortalTabId,
  preferredGroupId: ParentPortalNavGroup['id']
): NavItem | undefined {
  const selectedControl = selectedControlId ? findSelectedControl(content, selectedControlId) : undefined;
  if (selectedControl?.routePath) {
    const routeItem = navItems.find((item) => item.routePath === selectedControl.routePath);
    if (routeItem && (!initialNavLabel || routeItem.label === initialNavLabel)) return routeItem;
    const deviceRouteItem = deviceOpsNavItemForRoute(navItems, selectedControl.routePath);
    if (deviceRouteItem) return deviceRouteItem;
  }
  return (
    navItems.find((item) => item.label === initialNavLabel && item.groupId === preferredGroupId) ??
    navItems.find((item) => item.label === initialNavLabel && item.tabId === preferredTabId) ??
    navItems.find((item) => item.label === initialNavLabel)
  );
}

function preferredNavGroupIdForPageMode(pageMode: ParentPortalMode): ParentPortalNavGroup['id'] {
  if (pageMode === 'parentManage') return 'manage';
  if (pageMode === 'parentGuide') return 'guide';
  return 'quickGlance';
}

const MANAGE_DEVICE_OPS_ROUTE_KEYS = new Set([
  'lan-pairing',
  'capability-status',
  'platforms-install',
  'install-updates',
]);

function deviceOpsNavItemForRoute(navItems: NavItem[], routePath: string): NavItem | undefined {
  if (!MANAGE_DEVICE_OPS_ROUTE_KEYS.has(assetKey(routePath))) return undefined;
  return navItems.find((item) => item.routePath === '#/devices');
}

function routeControlIdForRoutePath(content: ParentPortalContentData, routePath?: string): string | undefined {
  if (!isHashRoutePath(routePath)) return undefined;
  const contentControls = [...content.controlAreas, ...content.quickControls.filter(isParentPortalControlEntry)];
  const exactControl = contentControls.find((control) => control.routePath === routePath);
  if (exactControl) return exactControl.id;
  const routeKey = assetKey(routePath);
  const fallbackControlId =
    routeKey === 'policy-screen'
      ? 'screen-analysis'
      : routeKey === 'policy-network'
        ? 'network-activity'
        : routeKey === 'rule-management' ||
            routeKey === 'schedules' ||
            routeKey === 'approvals' ||
            routeKey === 'enforcement'
          ? 'browser-settings'
          : routeKey === 'notifications' || routeKey === 'notification-channels'
            ? 'family-settings'
            : routeKey === 'export-retention' || routeKey === 'audit-history'
              ? 'drive-exports'
              : routeKey === 'api-providers' || routeKey === 'memory-settings'
                ? 'ai-runtime'
                : routeKey === 'diagnostics' || routeKey === 'entitlements'
                  ? 'subscription-plans'
                  : undefined;
  if (!fallbackControlId) return undefined;
  return contentControls.find((control) => normalizeSelectionId(control.id) === fallbackControlId)?.id;
}

function initialControlIdForPageMode(
  pageMode: ParentPortalMode,
  content: ParentPortalContentData,
  controlId?: string
): string {
  const routeId = normalizeSelectionId(controlId);
  const contentControls = content.quickControls.filter(isParentPortalControlEntry);
  const routeControl = [...content.controlAreas, ...contentControls].find(
    (control) => normalizeSelectionId(control.id) === routeId
  );
  if (routeControl) return routeControl.id;
  const routePathControlId = routeControlIdForRoutePath(content, `#/${routeId}`);
  if (routePathControlId) return routePathControlId;
  return content.modes[pageMode]?.selectedControlId ?? content.controlAreas[0]?.id ?? contentControls[0]?.id ?? '';
}

function isParentPortalControlEntry(control: Pick<ParentPortalQuickControl, 'id' | 'name' | 'routePath'>): boolean {
  const id = normalizeSelectionId(control.id);
  const name = normalizeSelectionId(control.name);
  if (id === 'parent-portal-hub' || id === 'quick-access' || id === 'all-controls') return false;
  if (name === 'quick-access' || name === 'all-controls') return false;
  return true;
}

function isHashRoutePath(routePath?: string): routePath is string {
  return typeof routePath === 'string' && routePath.startsWith('#/');
}

function rowSourceForPageMode(
  content: ParentPortalContentData,
  pageMode: ParentPortalMode,
  parentPortalRows: ParentPortalRow[]
): ParentPortalRow[] {
  const source = content.modes[pageMode]?.rowSource ?? 'api';
  if (source === 'aiBenchmarkRows') return content.aiBenchmarkRows;
  if (source === 'fallbackRows') return content.fallbackRows;
  return parentPortalRows;
}

function toDisplayRows(
  rows: ParentPortalRow[],
  pageMode: ParentPortalMode,
  selectedControlName: string,
  controlId?: string
): DisplayRow[] {
  const primaryArea = pageMode === 'parentManage' ? selectedControlName || formatRouteScope(controlId) : 'Mixed';
  return rows.map((row, index) => {
    const readyCount = row.readyCount ?? Math.max(0, Math.round(row.signalScore * 0.32));
    const gapCount = row.gapCount ?? Math.max(1, Math.round(readyCount * 0.62));
    const readiness =
      readyCount + gapCount > 0 ? `${Math.round((readyCount / (readyCount + gapCount)) * 1000) / 10}%` : '-';
    const tones: Tone[] = ['purple', 'red', 'cyan', 'gold', 'purple', 'red', 'cyan'];
    return {
      id: row.label || `row-${row.order}`,
      order: row.order,
      label: row.label || `Item ${row.order}`,
      signal: row.signalScore.toLocaleString(),
      signals: (readyCount + gapCount).toLocaleString(),
      readyCount: readyCount.toLocaleString(),
      gapCount: gapCount.toLocaleString(),
      readiness,
      primaryArea: row.primaryArea ?? primaryArea,
      trend: row.trend ?? (index % 3 === 0 ? '+2' : index % 3 === 1 ? '+1' : '-'),
      tone: row.tone ?? tones[index % tones.length] ?? 'purple',
    };
  });
}

function unavailableDisplayRow(primaryArea: string): DisplayRow {
  return {
    id: 'service-state-unavailable',
    order: 0,
    label: 'Service state unavailable',
    signal: '-',
    signals: '-',
    readyCount: '-',
    gapCount: '-',
    readiness: 'Unavailable',
    primaryArea: primaryArea || 'Parent portal',
    trend: 'Manual required',
    tone: 'muted',
  };
}

function tableVariantForContext(activeNavLabel: string, activeTab: ParentPortalTabId): ParentPortalTableVariant {
  const key = assetKey(activeNavLabel);
  if (key.includes('ai') || activeTab === 'aiStatus') return 'ai';
  if (key.includes('friend') || key.includes('guild') || activeTab === 'support') return 'support';
  if (key.includes('per-game') || key.includes('per-category') || activeTab === 'controls') return 'controls';
  if (key.includes('tournament') || key.includes('season') || key.includes('reward') || activeTab === 'routines')
    return 'routines';
  return 'statusRows';
}

function parentPortalScopeKey(value?: string): string {
  return (value ?? '').toLowerCase().replace(/[^a-z0-9]/g, '');
}

function rowsForControlScope(rows: DisplayRow[], selectedControlName: string, selectedControlId: string): DisplayRow[] {
  const controlKeys = new Set(
    [
      parentPortalScopeKey(selectedControlName),
      parentPortalScopeKey(selectedControlId),
      parentPortalScopeKey(selectedControlName.replace(/^three card/i, '3 card')),
      parentPortalScopeKey(selectedControlName.replace(/^3 card/i, 'three card')),
    ].filter(Boolean)
  );
  const matched = rows.filter((row) => controlKeys.has(parentPortalScopeKey(row.primaryArea)));
  const source = matched.length > 0 ? matched : rows.slice(0, Math.min(10, rows.length));
  return source.map((row, index) => ({
    ...row,
    order: index + 1,
    primaryArea: selectedControlName || row.primaryArea,
  }));
}

function rowsForCategoryScope(rows: DisplayRow[], selectedCategoryLabel: string): DisplayRow[] {
  const categoryKey = assetKey(selectedCategoryLabel);
  const matched = rows.filter(
    (row) => assetKey(controlCategoryLabel({ id: row.primaryArea, name: row.primaryArea })) === categoryKey
  );
  const source = matched.length > 0 ? matched : rows.slice(0, Math.min(10, rows.length));
  return source.map((row, index) => ({
    ...row,
    order: index + 1,
    primaryArea: selectedCategoryLabel || row.primaryArea,
  }));
}

const PORTAL_PRODUCT_ROUTE_STATUS_MAX_ROWS = 4;
const PORTAL_PRODUCT_TREND_LABELS: Record<string, string> = {
  'backend-not-connected': 'Backend not connected',
  'implemented-boundary': 'Implemented boundary',
  'manual-required': 'Manual required',
  'not-claimed': 'Not claimed',
  'not-reported': 'Not reported',
  'permission-required': 'Permission required',
  'read-only': 'Read only',
  'scaffold-only': 'Scaffold only',
};

function formatPortalTrendLabel(value: string): string {
  const normalized = assetKey(value);
  if (PORTAL_PRODUCT_TREND_LABELS[normalized]) return PORTAL_PRODUCT_TREND_LABELS[normalized];
  if (!normalized) return 'Not reported';
  if (/^[+-]?\d+$/.test(value.trim())) return value;
  return titleCaseControlName(normalized.replace(/-/g, ' '));
}

function productShellReadinessDetail(row: DisplayRow): string {
  return `${row.readyCount}/${row.signals} ready`;
}

function productShellRouteKeywords(routeKey: string): readonly string[] {
  if (routeKey.includes('browser') || routeKey.includes('web')) {
    return ['browser', 'managed-web', 'browser-activity', 'enforcement-readiness'];
  }
  if (routeKey.includes('rule') || routeKey.includes('policy')) {
    return ['policy', 'browser', 'managed-web', 'schedule-plan', 'approval-queue', 'enforcement-readiness'];
  }
  if (routeKey.includes('schedule')) return ['schedule-plan', 'policy', 'browser'];
  if (routeKey.includes('approval')) return ['approval-queue', 'policy', 'enforcement-readiness'];
  if (routeKey.includes('enforce')) return ['enforcement-readiness', 'policy', 'browser'];
  if (routeKey.includes('app')) return ['app-policy', 'app-and-game-sessions', 'enforcement-readiness'];
  if (routeKey.includes('game')) return ['game-policy', 'app-and-game-sessions', 'enforcement-readiness'];
  if (routeKey.includes('screen')) return ['screen-analysis', 'remote-screen-policy', 'enforcement-readiness'];
  if (routeKey.includes('network')) return ['network-activity', 'network-tracking', 'enforcement-readiness'];
  if (routeKey.includes('tracking')) return ['tracking-policy', 'remote-access', 'manual-required'];
  if (routeKey.includes('device') || routeKey.includes('lan') || routeKey.includes('capability')) {
    return ['lan-discovery', 'household-setup', 'capability-status'];
  }
  if (routeKey.includes('remote')) return ['remote-access', 'remote-screen-policy', 'backend-not-connected'];
  if (routeKey.includes('report') || routeKey.includes('activity')) {
    return ['reports-surface', 'activity-reports', 'activity-store', 'network-activity'];
  }
  if (routeKey.includes('ai') || routeKey.includes('api') || routeKey.includes('memory')) {
    return ['assistant-entry', 'api-providers', 'memory-setup'];
  }
  if (
    routeKey.includes('drive') ||
    routeKey.includes('export') ||
    routeKey.includes('audit') ||
    routeKey.includes('data')
  ) {
    return ['data-custody', 'drive', 'export-retention', 'audit-history'];
  }
  if (routeKey.includes('notification') || routeKey.includes('alert')) {
    return ['alerts', 'notification-channels', 'household-setup'];
  }
  if (routeKey.includes('subscription') || routeKey.includes('entitlement')) {
    return ['subscription', 'entitlements', 'support'];
  }
  if (routeKey.includes('support') || routeKey.includes('diagnostic')) return ['support', 'capability-status'];
  return ['household-setup', 'family-settings', 'capability-status'];
}

function productShellDisplayRowsForRoute(
  parentPortalRows: ParentPortalRow[],
  activeNavLabel: string,
  selectedControlName: string,
  specTitle: string
): DisplayRow[] {
  const rows = toDisplayRows(parentPortalRows, 'parentManage', selectedControlName);
  if (rows.length === 0) return [];
  const routeKey = assetKey(`${activeNavLabel} ${selectedControlName} ${specTitle}`);
  const keywords = productShellRouteKeywords(routeKey);
  const matched = rows.filter((row) => {
    const rowKey = assetKey(`${row.label} ${row.primaryArea} ${row.trend}`);
    return keywords.some((keyword) => rowKey.includes(keyword));
  });
  const source = matched.length > 0 ? matched : rows;
  const seen = new Set<string>();
  const uniqueRows: DisplayRow[] = [];
  for (const row of source) {
    const key = assetKey(`${row.id} ${row.label} ${row.primaryArea}`);
    if (seen.has(key)) continue;
    seen.add(key);
    uniqueRows.push(row);
    if (uniqueRows.length >= PORTAL_PRODUCT_ROUTE_STATUS_MAX_ROWS) break;
  }
  return uniqueRows;
}

function rowTopCard(row: DisplayRow): ParentPortalTopCardItem {
  return {
    kind: 'row',
    key: `row:${row.id}`,
    row,
    title: row.label,
    subtitle: row.primaryArea,
    value: formatPortalTrendLabel(row.trend),
    detail: productShellReadinessDetail(row),
    tone: row.tone,
  };
}

function titleCaseControlName(value: string): string {
  return value
    .toLowerCase()
    .split(' ')
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function controlCategoryLabel(
  control: Pick<ControlArea | QuickControl, 'id' | 'name'> & { category?: string }
): string {
  if (control.category) return control.category;
  const key = assetKey(`${control.id} ${control.name}`);
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

function controlSubcategoryLabel(control: { detail?: string; subcategory?: string | null }): string {
  const value = control.subcategory?.trim() || control.detail?.trim();
  return value ? taxonomyLeafLabel(value) : 'General';
}

function controlTopCard(
  control: ControlArea | QuickControl,
  statsControl?: ControlArea,
  index = 0
): ParentPortalTopCardItem {
  const title = control.name === control.name.toUpperCase() ? titleCaseControlName(control.name) : control.name;
  const value = 'matches' in control ? control.matches : (statsControl?.matches ?? control.detail);
  const detail = 'growth' in control ? control.growth : (statsControl?.growth ?? control.subcategory ?? 'View control');
  return {
    kind: 'control',
    key: `control:${normalizeSelectionId(control.id)}:${assetKey(control.name)}:${index}`,
    control,
    title,
    subtitle: controlCategoryLabel(control),
    value,
    detail,
    tone: control.tone,
  };
}

function guideTopCard(topic: ParentPortalGuideTopic): ParentPortalTopCardItem {
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

function buildControlCategorySummaries(controls: QuickControl[]): ControlCategorySummary[] {
  const summaries = new Map<string, ControlCategorySummary>();
  const subcategoryMaps = new Map<string, Map<string, ControlSubcategorySummary>>();
  const fallbackControl = controls[0];
  const tones: Tone[] = ['gold', 'cyan', 'purple', 'red', 'muted'];
  for (const control of controls) {
    const label = controlCategoryLabel(control);
    const id = assetKey(label);
    const subcategoryLabel = controlSubcategoryLabel(control);
    const subcategoryId = assetKey(subcategoryLabel);
    const existing = summaries.get(id);
    if (existing) {
      existing.count += 1;
      const subcategories = subcategoryMaps.get(id) ?? new Map<string, ControlSubcategorySummary>();
      const existingSubcategory = subcategories.get(subcategoryId);
      if (existingSubcategory) {
        existingSubcategory.count += 1;
      } else {
        subcategories.set(subcategoryId, {
          id: subcategoryId,
          label: subcategoryLabel,
          count: 1,
          tone: control.tone,
          sampleControl: control,
        });
      }
      subcategoryMaps.set(id, subcategories);
      existing.subcategories = Array.from(subcategories.values()).sort(
        (a, b) => b.count - a.count || a.label.localeCompare(b.label)
      );
      continue;
    }
    const firstSubcategory: ControlSubcategorySummary = {
      id: subcategoryId,
      label: subcategoryLabel,
      count: 1,
      tone: control.tone,
      sampleControl: control,
    };
    subcategoryMaps.set(id, new Map([[subcategoryId, firstSubcategory]]));
    summaries.set(id, {
      id,
      label,
      detail: subcategoryLabel,
      count: 1,
      tone: control.tone,
      sampleControl: control,
      subcategories: [firstSubcategory],
    });
  }
  PARENT_PORTAL_CATEGORY_LABELS.forEach((label, index) => {
    const id = assetKey(label);
    if (summaries.has(id) || !fallbackControl) return;
    summaries.set(id, {
      id,
      label,
      detail: 'Catalog scope',
      count: 0,
      tone: tones[index % tones.length] ?? 'purple',
      sampleControl: fallbackControl,
      subcategories: [],
    });
  });
  return Array.from(summaries.values()).sort((a, b) => {
    if (a.count !== b.count) return b.count - a.count;
    const aKnown = PARENT_PORTAL_CATEGORY_LABELS.indexOf(a.label as (typeof PARENT_PORTAL_CATEGORY_LABELS)[number]);
    const bKnown = PARENT_PORTAL_CATEGORY_LABELS.indexOf(b.label as (typeof PARENT_PORTAL_CATEGORY_LABELS)[number]);
    if (aKnown >= 0 && bKnown >= 0) return aKnown - bKnown;
    if (aKnown >= 0) return -1;
    if (bKnown >= 0) return 1;
    return a.label.localeCompare(b.label);
  });
}

function detailForNav(activeNavLabel: string, detail: TabDetail, activeNavItem?: NavItem | null): TabDetail {
  if (activeNavItem?.groupId === 'devTools') {
    return {
      ...detail,
      eyebrow: 'Developer tools',
      title: activeNavItem.label,
      summary: activeNavItem.detail,
      primary: 'Local inspection',
      secondary: 'Service authority stays fail-closed',
      action: 'Inspect local state',
      tone: activeNavItem.tone ?? 'cyan',
    };
  }
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

function SurfacePanel({
  x,
  y,
  w,
  h,
  tone = 'cyan',
  accentColor,
  frame = 'default',
  selected = false,
  disabled = false,
  frameCornerThicknessScale = 1,
  frameOuterTabWidth,
  frameInnerTabWidth,
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
  accentColor?: string;
  frame?: 'default' | 'deckSide';
  selected?: boolean;
  disabled?: boolean;
  frameCornerThicknessScale?: number;
  frameOuterTabWidth?: number;
  frameInnerTabWidth?: number;
  onClick?: () => void;
  ariaLabel?: string;
  children?: ReactNode;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = accentColor ?? toneColor(tone, cfg);
  const actionable = Boolean(onClick);
  const interactive = actionable && !disabled;
  const active = selected || hovered;
  const frameFillOpacity = disabled
    ? PARENT_PORTAL_FRAME_MATERIAL.disabledFillOpacity
    : PARENT_PORTAL_FRAME_MATERIAL.fillOpacity;
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
      className={interactive ? 'parent-portal-svg-clickable' : undefined}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      onMouseEnter={() => interactive && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      role={actionable ? 'button' : undefined}
      tabIndex={actionable ? (disabled ? -1 : 0) : undefined}
      aria-label={ariaLabel}
      aria-disabled={disabled || undefined}
    >
      {frame === 'deckSide' ? (
        <>
          <ParentPortalPanelFrame
            x={x}
            y={y}
            w={w}
            h={h}
            color={color}
            active={active}
            fill={PARENT_PORTAL_GLASS.panelFill}
            fillOpacity={frameFillOpacity}
            cornerThicknessScale={frameCornerThicknessScale}
            outerTabWidth={frameOuterTabWidth ?? cfg.chrome.frameOuterBulgeWidth}
            innerTabWidth={frameInnerTabWidth ?? cfg.chrome.frameInnerBulgeWidth}
          />
        </>
      ) : (
        <>
          <ParentPortalPanelFrame
            x={x}
            y={y}
            w={w}
            h={h}
            color={color}
            active={active}
            fill={PARENT_PORTAL_GLASS.panelFill}
            fillOpacity={frameFillOpacity}
            cornerThicknessScale={frameCornerThicknessScale}
            outerTabWidth={frameOuterTabWidth ?? cfg.chrome.frameOuterBulgeWidth}
            innerTabWidth={frameInnerTabWidth ?? cfg.chrome.frameInnerBulgeWidth}
          />
        </>
      )}
      {children}
    </g>
  );
}

function ParentPortalFrameSideHandle({
  x,
  y,
  side,
  disabled = false,
  height = 132,
  width = 22,
  accentColor,
  onClick,
  cfg,
}: {
  x: number;
  y: number;
  side: 'left' | 'right';
  disabled?: boolean;
  height?: number;
  width?: number;
  accentColor?: string;
  onClick: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const handleW = width;
  const handleH = height;
  const color = accentColor ?? cfg.colors.cyan;
  const glowFilter = color === cfg.colors.gold ? 'url(#parentPortalGoldGlow)' : 'url(#parentPortalGlow)';
  const compact = handleH <= 64;
  const visualH = compact ? 40 : handleH;
  const visualY = y + (handleH - visualH) / 2;
  const tipInset = Math.max(5, Math.min(7, handleW * 0.32));
  const iconSize = compact ? 13 : 18;
  const iconX = x + (handleW - iconSize) / 2;
  const iconY = visualY + (visualH - iconSize) / 2;
  const bodyPath =
    side === 'left'
      ? `M ${x + tipInset} ${visualY} H ${x + handleW} V ${visualY + visualH} H ${x + tipInset} L ${x} ${visualY + visualH - tipInset} V ${visualY + tipInset} Z`
      : `M ${x} ${visualY} H ${x + handleW - tipInset} L ${x + handleW} ${visualY + tipInset} V ${visualY + visualH - tipInset} L ${x + handleW - tipInset} ${visualY + visualH} H ${x} Z`;
  const iconHref = side === 'left' ? parentPortalSidePanelHandleLeftIconUrl : parentPortalSidePanelHandleRightIconUrl;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={disabled ? -1 : 0}
      aria-disabled={disabled}
      aria-label={side === 'left' ? 'Previous parent portal carousel page' : 'Next parent portal carousel page'}
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
        <path d={bodyPath} fill={color} opacity={0.24} filter={glowFilter} pointerEvents="none" />
      ) : null}
      <path
        d={bodyPath}
        fill={hovered && !disabled ? colorAlpha(color, '2d') : colorAlpha(color, '16')}
        stroke={color}
        strokeWidth={hovered && !disabled ? 2 : 1.35}
      />
      <path
        d={bodyPath}
        fill="url(#parentPortalFrameShine)"
        opacity={hovered && !disabled ? 0.95 : 0.76}
        pointerEvents="none"
      />
      <image
        href={iconHref}
        x={iconX}
        y={iconY}
        width={iconSize}
        height={iconSize}
        preserveAspectRatio="xMidYMid meet"
        opacity={hovered && !disabled ? 1 : 0.88}
        pointerEvents="none"
      />
      <rect x={x - 6} y={y - 6} width={handleW + 12} height={handleH + 12} fill="transparent" />
    </g>
  );
}

function ParentPortalFrameDots({
  x,
  y,
  page,
  pageCount,
  accentColor,
  onPageChange,
  cfg,
}: {
  x: number;
  y: number;
  page: number;
  pageCount: number;
  accentColor?: string;
  onPageChange: (page: number) => void;
  cfg: ParentPortalSvgControls;
}) {
  const color = accentColor ?? cfg.colors.cyan;
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
            className="parent-portal-svg-clickable"
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
              fill={active ? colorAlpha(color, '44') : colorAlpha(color, '10')}
              stroke={color}
              strokeWidth={active ? 1.5 : 1.1}
              strokeOpacity={active ? 0.95 : 0.58}
              filter={active ? 'url(#parentPortalGlow)' : undefined}
            />
          </g>
        );
      })}
    </g>
  );
}

function ParentPortalHeaderAction({
  x,
  y,
  w,
  h,
  label,
  iconHref,
  tone = 'cyan',
  accentColor,
  active = false,
  disabled = false,
  onClick,
  ariaLabel,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  iconHref?: string;
  tone?: Tone;
  accentColor?: string;
  active?: boolean;
  disabled?: boolean;
  onClick: () => void;
  ariaLabel?: string;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = accentColor ?? toneColor(tone, cfg);
  const lit = !disabled && (active || hovered);
  const iconBoxSize = iconHref ? Math.max(20, Math.min(25, h - 4)) : 0;
  const iconSize = iconHref ? Math.max(16, iconBoxSize - 5) : 0;
  const iconGap = iconHref ? 7 : 0;
  const iconBoxX = x + 7;
  const iconBoxY = y + (h - iconBoxSize) / 2;
  const iconX = iconBoxX + (iconBoxSize - iconSize) / 2;
  const iconY = iconBoxY + (iconBoxSize - iconSize) / 2;
  const labelMaxW = iconHref ? w - iconBoxSize - iconGap - 24 : w - 16;
  const labelX = iconHref ? iconBoxX + iconBoxSize + iconGap + labelMaxW / 2 : x + w / 2;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={disabled ? -1 : 0}
      aria-label={ariaLabel ?? label}
      aria-disabled={disabled || undefined}
      opacity={disabled ? 0.46 : 1}
      onClick={(event) => {
        if (disabled) return;
        event.stopPropagation();
        onClick();
      }}
      onKeyDown={(event) => {
        if (disabled || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onClick();
      }}
      onMouseEnter={() => {
        if (!disabled) setHovered(true);
      }}
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
          filter="url(#parentPortalGlow)"
        />
      ) : null}
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={3}
        fill={lit ? `${color}24` : PARENT_PORTAL_GLASS.panelFill}
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
      {iconHref ? (
        <image
          href={iconHref}
          x={iconX}
          y={iconY}
          width={iconSize}
          height={iconSize}
          opacity={1}
          preserveAspectRatio="xMidYMid meet"
          pointerEvents="none"
          filter={lit ? 'url(#parentPortalGlow)' : undefined}
        />
      ) : null}
      <text
        x={labelX}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={fitSingleLineTextSize(label, labelMaxW, 8.5, 11.5, 0.58)}
        fontWeight={900}
        fill={cfg.colors.bodyText}
        pointerEvents="none"
      >
        {label}
      </text>
    </g>
  );
}

function ParentPortalInfoButton({
  x,
  y,
  size = 24,
  accentColor,
  label,
  onClick,
  cfg,
}: {
  x: number;
  y: number;
  size?: number;
  accentColor?: string;
  label: string;
  onClick: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = accentColor ?? cfg.colors.cyan;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={label}
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
      <title>{label}</title>
      <circle
        cx={x + size / 2}
        cy={y + size / 2}
        r={size / 2 - 1}
        fill={hovered ? colorAlpha(color, '35') : colorAlpha(color, '18')}
        stroke={color}
        strokeWidth={hovered ? 1.3 : 0.9}
        filter={hovered ? 'url(#parentPortalGlow)' : undefined}
      />
      <text
        x={x + size / 2}
        y={y + size / 2 + 0.5}
        textAnchor="middle"
        dominantBaseline="middle"
        fontSize={Math.max(12, size * 0.62)}
        fontWeight={950}
        fill={cfg.colors.bodyText}
      >
        i
      </text>
    </g>
  );
}

function ParentPortalSectionFrame({
  x,
  y,
  w,
  h,
  title,
  subtitle,
  headerIcon,
  tone = 'cyan',
  accentColor,
  headerSlot,
  headerRight,
  headerInfoLabel,
  onHeaderInfoClick,
  footer,
  footerH,
  headerH,
  bodyInset,
  fullHeaderLine = false,
  bodyStrokeOpacity = PARENT_PORTAL_FRAME_MATERIAL.bodyStrokeOpacity,
  bodyFill = PARENT_PORTAL_FRAME_MATERIAL.bodyFill,
  footerLineOpacity = PARENT_PORTAL_FRAME_MATERIAL.footerLineOpacity,
  showSideHandles = false,
  sideDisabled = false,
  onPrevious,
  onNext,
  onWheel,
  selected = false,
  cfg,
  children,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  title: string;
  subtitle?: string;
  headerIcon?: IconComponent;
  tone?: Tone;
  accentColor?: string;
  headerSlot?: ReactNode;
  headerRight?: ReactNode;
  headerInfoLabel?: string;
  onHeaderInfoClick?: () => void;
  footer?: (rect: ParentPortalRect) => ReactNode;
  footerH?: number;
  headerH?: number;
  bodyInset?: number;
  fullHeaderLine?: boolean;
  bodyStrokeOpacity?: number | string;
  bodyFill?: string;
  footerLineOpacity?: number | string;
  showSideHandles?: boolean;
  sideDisabled?: boolean;
  onPrevious?: () => void;
  onNext?: () => void;
  onWheel?: (event: WheelEvent<SVGGElement>) => void;
  selected?: boolean;
  cfg: ParentPortalSvgControls;
  children: (rect: ParentPortalRect) => ReactNode;
}) {
  const color = accentColor ?? toneColor(tone, cfg);
  const {
    body,
    footer: footerRect,
    headerH: resolvedHeaderH,
  } = parentPortalFrameRects(x, y, w, h, footerH, headerH, bodyInset);
  const active = selected;
  const headerInset = Math.max(48, Math.min(76, w * 0.045));
  const headerRightInset = Math.max(48, Math.min(76, w * 0.045));
  const prominentHeader = fullHeaderLine;
  const headerRightReserveW = headerRight ? 144 : 0;
  const headerLineStart = fullHeaderLine ? x + 12 : x + headerInset;
  const headerLineEnd = fullHeaderLine
    ? x + w - 12 - headerRightReserveW
    : x + w - headerRightInset - (headerRight ? 132 : 0);
  const headerLineY = y + resolvedHeaderH - 7;
  const iconBoxW = headerIcon
    ? prominentHeader
      ? Math.max(26, Math.min(32, resolvedHeaderH - 24))
      : Math.max(32, Math.min(38, resolvedHeaderH - 6))
    : 0;
  const titleY = y + (prominentHeader ? 14 : 9);
  const titleH = Math.max(26, Math.min(prominentHeader ? 42 : 32, resolvedHeaderH - 10));
  const headerIconX = x + headerInset;
  const prominentHeaderTopClearance = prominentHeader ? Math.max(14, Math.min(18, resolvedHeaderH * 0.28)) : 0;
  const prominentHeaderCenterY = y + prominentHeaderTopClearance + (headerLineY - y - prominentHeaderTopClearance) / 2;
  const headerIconY = prominentHeader
    ? prominentHeaderCenterY - iconBoxW / 2
    : y + Math.max(7, (resolvedHeaderH - iconBoxW) / 2 - 1);
  const titleX = headerIcon ? headerIconX + iconBoxW + (prominentHeader ? 14 : 12) : headerIconX;
  const hasHeaderInfo = Boolean(headerInfoLabel && onHeaderInfoClick);
  const headerInfoSize = hasHeaderInfo ? (prominentHeader ? 25 : 22) : 0;
  const headerInfoReservedW = hasHeaderInfo ? headerInfoSize + 18 : 0;
  const titleMaxW = Math.max(80, headerLineEnd - titleX - 12 - headerInfoReservedW);
  const responsiveTitle =
    prominentHeader && title === PortalLanPairingScan.Text.HeaderTitle && titleMaxW < 320 ? 'LAN' : title;
  const titleFontSize = fitSingleLineTextSize(
    responsiveTitle,
    titleMaxW,
    prominentHeader ? 15 : 12,
    prominentHeader ? 17 : 14,
    0.56
  );
  const titleText = truncateTextForWidth(responsiveTitle, titleMaxW, titleFontSize, 0.56);
  const titleBaseline = prominentHeader
    ? prominentHeaderCenterY + titleFontSize * 0.35
    : headerIcon
      ? headerIconY + iconBoxW * 0.65
      : titleY + titleH * 0.68;
  const titleTextWidthFactor = prominentHeader ? 0.62 : 0.56;
  const headerInfoGap = prominentHeader ? 17 : 12;
  const titleTextW = Math.min(titleMaxW, Math.ceil(titleText.length * titleFontSize * titleTextWidthFactor));
  const headerInfoX = Math.min(titleX + titleTextW + headerInfoGap, headerLineEnd - headerInfoSize - 4);
  const headerInfoY = prominentHeader
    ? prominentHeaderCenterY - headerInfoSize / 2
    : headerIcon
      ? headerIconY + (iconBoxW - headerInfoSize) / 2
      : y + (resolvedHeaderH - headerInfoSize) / 2 - 1;
  const HeaderIcon = headerIcon;
  const showHeaderSubtitle = Boolean(subtitle && !headerIcon);
  const sideHandleW = PARENT_PORTAL_SIDE_HANDLE_W;
  const sideHandleH = Math.max(72, Math.min(128, body.h - 28));
  const sideHandleY = body.y + Math.max(12, (body.h - sideHandleH) / 2);
  const leftHandleX = x - sideHandleW + PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
  const rightHandleX = x + w - PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
  const showFooterLine = footerLineOpacity !== 0 && footerLineOpacity !== '0';
  return (
    <g onWheel={onWheel}>
      <ParentPortalPanelFrame
        x={x}
        y={y}
        w={w}
        h={h}
        color={color}
        active={active}
        fill="url(#parentPortalFrameFill)"
        fillOpacity={PARENT_PORTAL_FRAME_MATERIAL.fillOpacity}
        cornerThicknessScale={0.86}
        outerTabWidth={cfg.chrome.frameOuterBulgeWidth}
        innerTabWidth={cfg.chrome.frameInnerBulgeWidth}
      />
      {headerSlot ? (
        headerSlot
      ) : (
        <>
          {HeaderIcon ? (
            <HeaderIcon
              x={headerIconX + (prominentHeader ? 3 : 5)}
              y={headerIconY + (prominentHeader ? 3 : 5)}
              width={iconBoxW - (prominentHeader ? 6 : 10)}
              height={iconBoxW - (prominentHeader ? 6 : 10)}
              color={color}
              strokeWidth={prominentHeader ? 2.6 : 2.3}
            />
          ) : null}
          <text
            x={titleX}
            y={titleBaseline}
            fontSize={titleFontSize}
            fontWeight={950}
            fill={cfg.colors.bodyText}
            stroke="#03121f"
            strokeWidth={0.8}
            strokeOpacity={0.78}
            paintOrder="stroke"
            letterSpacing={0}
          >
            {titleText}
          </text>
          <line
            x1={headerLineStart}
            y1={headerLineY}
            x2={headerLineEnd}
            y2={headerLineY}
            stroke={color}
            strokeWidth={1.1}
            opacity={PARENT_PORTAL_FRAME_MATERIAL.headerLineOpacity}
          />
          {headerInfoLabel && onHeaderInfoClick ? (
            <ParentPortalInfoButton
              x={headerInfoX}
              y={headerInfoY}
              size={headerInfoSize}
              accentColor={color}
              label={headerInfoLabel}
              onClick={onHeaderInfoClick}
              cfg={cfg}
            />
          ) : null}
          {showHeaderSubtitle ? (
            <text x={x + 24} y={y + resolvedHeaderH - 12} fontSize={10.2} fontWeight={820} fill={cfg.colors.mutedText}>
              {subtitle}
            </text>
          ) : null}
          {headerRight}
        </>
      )}
      <path
        d={cutRectPath(body.x, body.y, body.w, body.h, 7)}
        fill={bodyFill}
        stroke={color}
        strokeWidth={0.9}
        strokeOpacity={bodyStrokeOpacity}
      />
      {children(body)}
      {showFooterLine ? (
        <line
          x1={footerRect.x + 12}
          y1={footerRect.y + 3}
          x2={footerRect.x + footerRect.w - 12}
          y2={footerRect.y + 3}
          stroke={color}
          strokeWidth={1.1}
          opacity={footerLineOpacity}
        />
      ) : null}
      {footer?.(footerRect)}
      {showSideHandles && onPrevious ? (
        <ParentPortalFrameSideHandle
          x={leftHandleX}
          y={sideHandleY}
          side="left"
          height={sideHandleH}
          width={sideHandleW}
          disabled={sideDisabled}
          onClick={onPrevious}
          accentColor={color}
          cfg={cfg}
        />
      ) : null}
      {showSideHandles && onNext ? (
        <ParentPortalFrameSideHandle
          x={rightHandleX}
          y={sideHandleY}
          side="right"
          height={sideHandleH}
          width={sideHandleW}
          disabled={sideDisabled}
          onClick={onNext}
          accentColor={color}
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
  cfg: ParentPortalSvgControls;
}) {
  const rawId = useId();
  const color = toneColor(tone, cfg);
  const cx = x + w / 2;
  const cy = y + h / 2;
  const inset = Math.min(8, w * 0.18, h * 0.18);
  const radius = Math.min(w, h) / 2 - 1;
  const clipId = `parent-portal-art-${rawId.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const primaryText = compact ? 'MISS' : 'MISSING';
  const primaryFontSize = compact ? Math.min(7, Math.max(4.5, w * 0.24)) : Math.min(11, Math.max(7.5, w * 0.09));
  const secondaryFontSize = Math.min(8.5, Math.max(6.5, w * 0.06));
  const fill = imageUrl ? PARENT_PORTAL_GLASS.panelFillDeep : 'rgba(48, 12, 23, 0.72)';
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

function NavRow({
  item,
  active,
  x,
  w,
  y,
  rowH,
  iconSize,
  nested = false,
  branchColor,
  visible = true,
  disabled = false,
  ariaLabel,
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
  nested?: boolean;
  branchColor?: string;
  visible?: boolean;
  disabled?: boolean;
  ariaLabel?: string;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const interactive = visible && !disabled;
  const color = active ? cfg.colors.bodyText : '#d8eaff';
  const rowX = x + 14;
  const rowW = w - 28;
  const lit = active || hovered;
  const branchAccent = branchColor ?? toneColor(item.tone ?? 'cyan', cfg);
  const accent = branchAccent;
  const slotX = x + 22;
  const slotY = y + (rowH - iconSize) / 2;
  const slotW = Math.max(32, iconSize - 2);
  const textX = slotX + slotW + 8;
  const labelW = Math.max(48, x + w - 28 - textX);
  const labelSize = 10.9;
  const arrowTop = y + 7;
  const arrowBottom = y + rowH - 7;
  const arrowMid = y + rowH / 2;
  const arrowOffset = Math.max(4, cfg.chrome.panelStrokeWidth * 2.4);
  const arrowBaseX = rowX + rowW + arrowOffset;
  const arrowTipX = arrowBaseX + 20;
  return (
    <g
      className={interactive ? 'parent-portal-svg-clickable' : undefined}
      onClick={(event) => {
        if (!interactive) return;
        event.stopPropagation();
        onSelect();
      }}
      onMouseEnter={() => interactive && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (!interactive || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onSelect();
      }}
      role={visible ? 'button' : undefined}
      tabIndex={interactive ? 0 : -1}
      aria-hidden={visible ? undefined : true}
      aria-disabled={visible && disabled ? true : undefined}
      aria-current={visible && active ? 'page' : undefined}
      aria-label={ariaLabel ?? `Open ${item.label}`}
    >
      <rect
        x={rowX - 6}
        y={y - 4}
        width={rowW + 28}
        height={rowH + 8}
        fill="transparent"
        pointerEvents={interactive ? 'all' : 'none'}
      />
      {nested ? (
        <>
          <path
            d={`M ${rowX - 8} ${y + 7} V ${y + rowH - 7}`}
            stroke={branchAccent}
            strokeWidth={active ? 2.2 : 1.4}
            strokeLinecap="round"
            opacity={active ? 0.72 : 0.5}
            filter={active ? 'url(#parentPortalGlow)' : undefined}
            pointerEvents="none"
          />
          <path
            d={`M ${rowX - 8} ${y + rowH / 2} H ${rowX - 1}`}
            stroke={branchAccent}
            strokeWidth={active ? 2 : 1.2}
            strokeLinecap="round"
            opacity={active ? 0.66 : 0.48}
            pointerEvents="none"
          />
        </>
      ) : null}
      {lit ? (
        <>
          <path
            d={cutRectPath(rowX - 3, y - 3, rowW + 6, rowH + 6, 11)}
            fill="none"
            stroke={accent}
            strokeWidth={active ? 2.2 : 2}
            opacity={active ? 0.42 : 0.34}
            filter="url(#parentPortalGlow)"
            pointerEvents="none"
          />
          {lit ? (
            <path
              d={`M ${arrowBaseX} ${arrowTop} L ${arrowTipX} ${arrowMid} L ${arrowBaseX} ${arrowBottom} Z`}
              fill={accent}
              opacity={active ? 0.82 : 0.42}
              filter="url(#parentPortalGlow)"
              pointerEvents="none"
            />
          ) : null}
        </>
      ) : null}
      <path
        d={cutRectPath(rowX, y, rowW, rowH, 8)}
        fill={
          active ? colorAlpha(accent, '30') : hovered ? colorAlpha(branchAccent, '18') : colorAlpha(branchAccent, '08')
        }
        fillOpacity={1}
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
          opacity={active ? 0.68 : 0.42}
          pointerEvents="none"
        />
      ) : null}
      <NavIconSlot
        Icon={item.icon}
        x={slotX}
        y={slotY + 3}
        size={slotW}
        color={accent}
        lit={lit}
        selected={active}
        cfg={cfg}
      />
      <text
        x={textX}
        y={y + rowH * 0.64}
        fontSize={labelSize}
        fontWeight={940}
        fill={color}
        stroke="#03121f"
        strokeWidth={0.7}
        strokeOpacity={0.78}
        paintOrder="stroke"
        pointerEvents="none"
      >
        {truncateTextForWidth(item.label, labelW, labelSize, 0.54)}
      </text>
    </g>
  );
}

function NavIconSlot({
  Icon,
  x,
  y,
  size,
  color,
  lit,
  selected = false,
  cfg,
}: {
  Icon: IconComponent;
  x: number;
  y: number;
  size: number;
  color: string;
  lit: boolean;
  selected?: boolean;
  cfg: ParentPortalSvgControls;
}) {
  const iconSize = Math.max(24, size - 2);
  const iconX = x + (size - iconSize) / 2;
  const iconY = y + (size - iconSize) / 2;
  return (
    <g pointerEvents="none">
      <Icon
        x={iconX}
        y={iconY}
        width={iconSize}
        height={iconSize}
        color={selected || lit ? cfg.colors.bodyText : color}
        strokeWidth={selected ? 2.35 : 2.05}
      />
    </g>
  );
}

function FoldoutTriangleIndicator({
  x,
  y,
  size,
  open,
  hovered,
  accent,
  glowFilter,
}: {
  x: number;
  y: number;
  size: number;
  open: boolean;
  hovered: boolean;
  accent: string;
  glowFilter: string;
}) {
  const lit = open || hovered;
  const indicatorColor = accent;
  const iconInset = Math.max(4, size * 0.22);
  return (
    <g pointerEvents="none">
      {lit ? (
        <path
          d={cutRectPath(x - 2, y - 2, size + 4, size + 4, 5)}
          fill="none"
          stroke={indicatorColor}
          strokeWidth={open ? 1.45 : 1.15}
          opacity={open ? 0.44 : 0.3}
          filter={glowFilter}
        />
      ) : null}
      <path
        d={cutRectPath(x, y, size, size, 4)}
        fill={colorAlpha(indicatorColor, open ? '26' : hovered ? '1e' : '10')}
        stroke={indicatorColor}
        strokeWidth={lit ? 1.15 : 0.85}
        strokeOpacity={lit ? 0.9 : 0.58}
      />
      <image
        href={open ? parentPortalFoldoutOpenIconUrl : parentPortalFoldoutClosedIconUrl}
        x={x + iconInset}
        y={y + iconInset}
        width={size - iconInset * 2}
        height={size - iconInset * 2}
        preserveAspectRatio="xMidYMid meet"
        opacity={lit ? 0.98 : 0.78}
        filter={lit ? glowFilter : undefined}
      />
    </g>
  );
}

function NavSectionHeader({
  label,
  icon: Icon,
  open,
  x,
  w,
  y,
  h,
  accentColor,
  glowFilter,
  visible = true,
  onToggle,
  cfg,
}: {
  label: string;
  icon: IconComponent;
  open: boolean;
  x: number;
  w: number;
  y: number;
  h: number;
  accentColor?: string;
  glowFilter?: string;
  visible?: boolean;
  onToggle: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const lit = open || hovered;
  const baseAccent = accentColor ?? cfg.colors.cyan;
  const activeGlowFilter = glowFilter ?? 'url(#parentPortalGlow)';
  const accent = baseAccent;
  const rowX = x + 24;
  const rowW = w - 40;
  const panelY = y;
  const panelH = h;
  const slotSize = Math.max(28, Math.min(32, h - 6));
  const slotX = rowX + 8;
  const slotY = panelY + (panelH - slotSize) / 2;
  const indicatorSize = 20;
  const indicatorX = rowX + rowW - indicatorSize - 10;
  const indicatorY = panelY + (panelH - indicatorSize) / 2;
  const textX = slotX + slotSize + 10;
  const labelW = Math.max(40, indicatorX - textX - 10);
  const labelSize = 11.8;
  return (
    <g
      className="parent-portal-svg-clickable"
      onClick={(event) => {
        if (!visible) return;
        event.stopPropagation();
        onToggle();
      }}
      onMouseEnter={() => visible && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (!visible || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onToggle();
      }}
      role={visible ? 'button' : undefined}
      tabIndex={visible ? 0 : -1}
      aria-hidden={visible ? undefined : true}
      aria-label={`${open ? 'Collapse' : 'Expand'} ${label}`}
      aria-expanded={visible ? open : undefined}
    >
      <rect
        x={rowX - 7}
        y={panelY - 4}
        width={rowW + 14}
        height={panelH + 8}
        fill="transparent"
        pointerEvents={visible ? 'all' : 'none'}
      />
      <path
        d={`M ${x + 18} ${panelY + panelH / 2} H ${rowX - 8}`}
        stroke={accent}
        strokeWidth={1}
        opacity={lit ? 0.56 : 0.38}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX - 3, panelY - 3, rowW + 6, panelH + 6, 10)}
          fill="none"
          stroke={accent}
          strokeWidth={open ? 1.9 : 1.5}
          opacity={open ? 0.34 : 0.25}
          filter={activeGlowFilter}
          pointerEvents="none"
        />
      ) : null}
      {open ? (
        <path
          d={cutRectPath(rowX - 1, panelY + 6, 5, panelH - 12, 2)}
          fill={accent}
          opacity={0.84}
          filter={activeGlowFilter}
          pointerEvents="none"
        />
      ) : null}
      <path
        d={cutRectPath(rowX, panelY, rowW, panelH, 6)}
        fill={open ? colorAlpha(baseAccent, '19') : lit ? colorAlpha(baseAccent, '14') : colorAlpha(baseAccent, '08')}
        stroke={accent}
        strokeWidth={lit ? 1.25 : 0.85}
        strokeOpacity={lit ? 0.84 : 0.46}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX + 3, panelY + 3, rowW - 6, panelH - 6, 5)}
          fill="none"
          stroke={accent}
          strokeWidth={0.9}
          opacity={open ? 0.52 : 0.38}
          pointerEvents="none"
        />
      ) : null}
      <NavIconSlot Icon={Icon} x={slotX} y={slotY} size={slotSize} color={accent} lit={lit} selected={open} cfg={cfg} />
      <text
        x={textX}
        y={panelY + panelH * 0.61}
        fontSize={labelSize}
        fontWeight={940}
        fill={cfg.colors.bodyText}
        stroke="#03121f"
        strokeWidth={0.8}
        strokeOpacity={0.78}
        paintOrder="stroke"
        pointerEvents="none"
      >
        {truncateTextForWidth(label, labelW, labelSize, 0.54)}
      </text>
      <FoldoutTriangleIndicator
        x={indicatorX}
        y={indicatorY}
        size={indicatorSize}
        open={open}
        hovered={hovered}
        accent={baseAccent}
        glowFilter={activeGlowFilter}
      />
    </g>
  );
}

function navGroupThemeColor(groupId: string, cfg: ParentPortalSvgControls): string {
  if (groupId === 'manage') return '#4ff2d2';
  if (groupId === 'guide') return '#5ecfff';
  if (groupId === 'quickGlance') return '#45e8ef';
  return cfg.colors.cyan;
}

function navGroupFoldKey(groupId: string): string {
  return `group:${groupId}`;
}

function navSectionFoldKey(sectionId: string): string {
  return `section:${sectionId}`;
}

function navFoldoutClassName(opening: boolean, closing: boolean): string {
  const baseClassName = 'parent-portal-nav-foldout';
  if (closing) return `${baseClassName} ${baseClassName}--closing`;
  if (opening) return `${baseClassName} ${baseClassName}--opening`;
  return baseClassName;
}

function NavGroupHeader({
  group,
  open,
  x,
  w,
  y,
  h,
  visible = true,
  onToggle,
  cfg,
}: {
  group: NavGroup;
  open: boolean;
  x: number;
  w: number;
  y: number;
  h: number;
  visible?: boolean;
  onToggle: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const rowX = x + 14;
  const rowW = w - 28;
  const lit = open || hovered;
  const accent = navGroupThemeColor(group.id, cfg);
  const glowFilter = 'url(#parentPortalGlow)';
  const Icon =
    group.id === 'quickGlance'
      ? QuickGlanceGlasses
      : group.id === 'guide'
        ? GuideBookIcon
        : group.id === 'manage'
          ? ManageFileSettingsIcon
          : (group.items[0]?.icon ?? OverviewListIcon);
  const slotSize = Math.max(32, Math.min(38, h - 6));
  const slotX = rowX + 8;
  const slotY = y + (h - slotSize) / 2;
  const indicatorSize = 26;
  const indicatorX = rowX + rowW - indicatorSize - 10;
  const indicatorY = y + (h - indicatorSize) / 2;
  const textX = slotX + slotSize + 10;
  const labelW = Math.max(46, indicatorX - textX - 10);
  const labelSize = 13.8;
  return (
    <g
      className="parent-portal-svg-clickable"
      onClick={(event) => {
        if (!visible) return;
        event.stopPropagation();
        onToggle();
      }}
      onMouseEnter={() => visible && setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onKeyDown={(event) => {
        if (!visible || (event.key !== 'Enter' && event.key !== ' ')) return;
        event.preventDefault();
        event.stopPropagation();
        onToggle();
      }}
      role={visible ? 'button' : undefined}
      tabIndex={visible ? 0 : -1}
      aria-hidden={visible ? undefined : true}
      aria-label={`${open ? 'Collapse' : 'Expand'} ${group.label}`}
      aria-expanded={visible ? open : undefined}
    >
      <rect
        x={rowX - 6}
        y={y - 4}
        width={rowW + 12}
        height={h + 8}
        fill="transparent"
        pointerEvents={visible ? 'all' : 'none'}
      />
      {lit ? (
        <path
          d={cutRectPath(rowX - 3, y - 3, rowW + 6, h + 6, 11)}
          fill="none"
          stroke={accent}
          strokeWidth={open ? 2 : 1.6}
          opacity={open ? 0.35 : 0.23}
          filter={glowFilter}
          pointerEvents="none"
        />
      ) : null}
      {open ? (
        <path
          d={cutRectPath(rowX - 1, y + 6, 5, h - 12, 2)}
          fill={accent}
          opacity={0.78}
          filter={glowFilter}
          pointerEvents="none"
        />
      ) : null}
      <path
        d={cutRectPath(rowX, y, rowW, h, 8)}
        fill={lit ? colorAlpha(accent, open ? '22' : '16') : colorAlpha(accent, '0a')}
        fillOpacity={1}
        stroke={accent}
        strokeWidth={lit ? 1.45 : 0.8}
        strokeOpacity={lit ? 0.86 : 0.48}
        pointerEvents="none"
      />
      {lit ? (
        <path
          d={cutRectPath(rowX + 3, y + 3, rowW - 6, h - 6, 6)}
          fill="none"
          stroke={accent}
          strokeWidth={0.95}
          opacity={open ? 0.58 : 0.38}
          pointerEvents="none"
        />
      ) : null}
      <NavIconSlot Icon={Icon} x={slotX} y={slotY} size={slotSize} color={accent} lit={lit} selected={open} cfg={cfg} />
      <text
        x={textX}
        y={y + h * 0.62}
        fontSize={labelSize}
        fontWeight={940}
        fill={cfg.colors.bodyText}
        stroke="#03121f"
        strokeWidth={0.8}
        strokeOpacity={0.78}
        paintOrder="stroke"
        pointerEvents="none"
      >
        {truncateTextForWidth(group.label, labelW, labelSize, 0.54)}
      </text>
      <FoldoutTriangleIndicator
        x={indicatorX}
        y={indicatorY}
        size={indicatorSize}
        open={open}
        hovered={hovered}
        accent={accent}
        glowFilter={glowFilter}
      />
    </g>
  );
}

function ParentPortalMobileNavigation({
  activeNavRouteKey,
  assistantActive,
  assistantRoutePath,
  navGroups,
  onAssistantOpen,
  onSelect,
}: {
  activeNavRouteKey: string;
  assistantActive: boolean;
  assistantRoutePath: string;
  navGroups: NavGroup[];
  onAssistantOpen: () => void;
  onSelect: (item: NavItem) => void;
}): ReactElement {
  return (
    <nav className="parent-portal-mobile-nav" aria-label="Parent portal sections">
      <label className="parent-portal-mobile-nav__field">
        <span className="parent-portal-mobile-nav__label">Section</span>
        <select
          className="parent-portal-mobile-nav__select"
          aria-label="Choose parent portal section"
          value={assistantActive ? assistantRoutePath : activeNavRouteKey}
          onChange={(event) => {
            if (event.target.value === assistantRoutePath) {
              onAssistantOpen();
              return;
            }
            const item = navGroups
              .flatMap((group) => group.items)
              .find((entry) => navItemKey(entry) === event.target.value);
            if (item) onSelect(item);
          }}
        >
          {navGroups.map((group) => (
            <optgroup key={group.id} label={group.label}>
              {group.items.map((item) => (
                <option key={navItemKey(item)} value={navItemKey(item)}>
                  {item.label}
                </option>
              ))}
            </optgroup>
          ))}
          <optgroup label="Assistant">
            <option value={assistantRoutePath}>AI Assistant</option>
          </optgroup>
        </select>
      </label>
    </nav>
  );
}

function parentPortalNavControlIsVisible(
  y: number,
  height: number,
  scroll: number,
  viewportY: number,
  viewportHeight: number
): boolean {
  const translatedTop = y - scroll;
  const translatedBottom = translatedTop + height;
  return translatedTop >= viewportY && translatedBottom <= viewportY + viewportHeight;
}

function NavPanel({
  activeNavLabel,
  activeNavRouteKey,
  navGroups,
  openGroupIds,
  assistantOpen,
  assistantCommandAvailable,
  selectedAssistantActionId,
  onAssistantNewChat,
  onNavGroupToggle,
  onNavItemSelect,
  onAssistantOpen,
  onAssistantActionSelect,
  cfg,
}: {
  activeNavLabel: string;
  activeNavRouteKey: string;
  navGroups: NavGroup[];
  openGroupIds: Record<string, boolean>;
  assistantOpen: boolean;
  assistantCommandAvailable: boolean;
  selectedAssistantActionId: AssistantQuickActionId | null;
  onAssistantNewChat: () => void;
  onNavGroupToggle: (groupId: string) => void;
  onNavItemSelect: (item: NavItem) => void;
  onAssistantOpen: () => void;
  onAssistantActionSelect: (actionId: AssistantQuickActionId) => void;
  cfg: ParentPortalSvgControls;
}) {
  const { outerPad, leftW, topY } = cfg.layout;
  const navItems = navGroups.flatMap((group) => group.items);
  const activeNavGroupId =
    navGroupIdForNavKey(navGroups, activeNavRouteKey, activeNavLabel) || navGroups[0]?.id || 'quickGlance';
  const activeNavThemeColor = navGroupThemeColor(activeNavGroupId, cfg);
  const rawNavClipId = useId();
  const navClipId = `parent-portal-nav-clip-${rawNavClipId.replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const [navScroll, setNavScroll] = useState(0);
  const [openSectionIds, setOpenSectionIds] = useState(() =>
    initialOpenNavSectionIds(navGroups, activeNavRouteKey, activeNavLabel)
  );
  const [openingFoldIds, setOpeningFoldIds] = useState<Record<string, boolean>>({});
  const [closingFoldIds, setClosingFoldIds] = useState<Record<string, boolean>>({});
  const foldTimerIdsRef = useRef<Record<string, number>>({});
  const clearFoldTimer = useCallback((foldKey: string) => {
    const timerId = foldTimerIdsRef.current[foldKey];
    if (timerId !== undefined) {
      window.clearTimeout(timerId);
      delete foldTimerIdsRef.current[foldKey];
    }
  }, []);
  const beginFoldPhase = useCallback(
    (foldKey: string, phase: 'opening' | 'closing') => {
      clearFoldTimer(foldKey);
      setOpeningFoldIds((current) => {
        const next = { ...current };
        if (phase === 'opening') next[foldKey] = true;
        else delete next[foldKey];
        return next;
      });
      setClosingFoldIds((current) => {
        const next = { ...current };
        if (phase === 'closing') next[foldKey] = true;
        else delete next[foldKey];
        return next;
      });
      foldTimerIdsRef.current[foldKey] = window.setTimeout(() => {
        delete foldTimerIdsRef.current[foldKey];
        setOpeningFoldIds((current) => {
          if (!current[foldKey]) return current;
          const next = { ...current };
          delete next[foldKey];
          return next;
        });
        setClosingFoldIds((current) => {
          if (!current[foldKey]) return current;
          const next = { ...current };
          delete next[foldKey];
          return next;
        });
      }, PARENT_PORTAL_SIDE_NAV_FOLD_MS);
    },
    [clearFoldTimer]
  );
  useEffect(() => {
    return () => {
      Object.values(foldTimerIdsRef.current).forEach((timerId) => window.clearTimeout(timerId));
      foldTimerIdsRef.current = {};
    };
  }, []);
  const groupH = 46;
  const sectionH = 42;
  const rowH = 38;
  const rowStep = navItems.length > 8 ? 40 : 42;
  const groupGap = 6;
  const sectionGap = 5;
  const iconSize = 38;
  const assistantDockH = 74;
  const assistantGap = 10;
  const sideTopY = Math.max(0, topY - 14);
  const rowTop = sideTopY + 28;
  const navH = Math.max(300, cfg.canvas.height - sideTopY - assistantDockH - assistantGap - 2);
  const assistantPanelH = Math.max(300, cfg.canvas.height - sideTopY - 2);
  const assistantY = sideTopY + navH + assistantGap;
  const navViewportY = rowTop;
  const navViewportH = Math.max(72, sideTopY + navH - rowTop - 22);
  const groupVisuallyOpen = (groupId: string) =>
    Boolean(openGroupIds[groupId] || closingFoldIds[navGroupFoldKey(groupId)]);
  const sectionVisuallyOpen = (section: { id: string; label: string; items: NavItem[] }) =>
    !section.label || Boolean(openSectionIds[section.id] || closingFoldIds[navSectionFoldKey(section.id)]);
  const navContentH = navGroups.reduce((height, group) => {
    if (!groupVisuallyOpen(group.id)) return height + groupH + groupGap;
    const itemHeight = navSectionsForGroup(group).reduce((nextHeight, section) => {
      const sectionDelta = section.label ? sectionH : 0;
      const sectionOpen = sectionVisuallyOpen(section);
      return (
        nextHeight +
        sectionDelta +
        (sectionOpen ? section.items.length * rowStep : 0) +
        (section.label ? sectionGap : 0)
      );
    }, 0);
    return height + groupH + itemHeight + groupGap;
  }, 0);
  let activeNavRowY: number | null = null;
  let scanY = rowTop;
  for (const group of navGroups) {
    scanY += groupH;
    if (groupVisuallyOpen(group.id)) {
      for (const section of navSectionsForGroup(group)) {
        if (section.label) {
          scanY += sectionH;
        }
        if (section.label && !sectionVisuallyOpen(section)) continue;
        for (const item of section.items) {
          if (navItemMatches(item, activeNavRouteKey, activeNavLabel)) {
            activeNavRowY = scanY;
          }
          scanY += rowStep;
        }
        if (section.label) scanY += sectionGap;
      }
    }
    scanY += groupGap;
  }
  const maxNavScroll = Math.max(0, navContentH - navViewportH);
  const safeNavScroll = clampValue(navScroll, 0, maxNavScroll);
  useEffect(() => {
    if (safeNavScroll !== navScroll) setNavScroll(safeNavScroll);
  }, [navScroll, safeNavScroll]);
  useEffect(() => {
    setOpenSectionIds(initialOpenNavSectionIds(navGroups, activeNavRouteKey, activeNavLabel));
  }, [activeNavLabel, activeNavRouteKey, navGroups]);
  useEffect(() => {
    if (activeNavRowY === null || maxNavScroll <= 0) return;
    const topPad = 6;
    const visibleTop = navViewportY + topPad;
    const visibleBottom = navViewportY + navViewportH - topPad;
    setNavScroll((value) => {
      const activeTop = activeNavRowY - value;
      const trailingContentHeight = Math.min(
        Math.max(0, rowTop + navContentH - activeNavRowY - rowH),
        navViewportH * 0.8
      );
      const activeBottom = activeTop + rowH + trailingContentHeight;
      if (activeTop < visibleTop) {
        return clampValue(activeNavRowY - visibleTop, 0, maxNavScroll);
      }
      if (activeBottom > visibleBottom) {
        return clampValue(activeNavRowY + rowH + trailingContentHeight - visibleBottom, 0, maxNavScroll);
      }
      return value;
    });
  }, [
    activeNavLabel,
    activeNavRouteKey,
    activeNavRowY,
    maxNavScroll,
    navContentH,
    navViewportH,
    navViewportY,
    rowH,
    rowTop,
  ]);
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
  const toggleNavSection = (sectionId: string) => {
    beginFoldPhase(navSectionFoldKey(sectionId), openSectionIds[sectionId] ? 'closing' : 'opening');
    setOpenSectionIds((current) => toggleOpenNavSectionId(current, navGroups, sectionId));
  };
  const handleNavGroupToggle = (groupId: string) => {
    beginFoldPhase(navGroupFoldKey(groupId), openGroupIds[groupId] ? 'closing' : 'opening');
    onNavGroupToggle(groupId);
  };
  if (assistantOpen) {
    return (
      <g className="parent-portal-study-side-pane">
        <AssistantQuickActionPanel
          x={outerPad}
          y={sideTopY}
          w={leftW}
          h={assistantPanelH}
          navGroups={navGroups}
          selectedActionId={selectedAssistantActionId}
          commandAvailable={assistantCommandAvailable}
          onNewChat={onAssistantNewChat}
          onActionSelect={onAssistantActionSelect}
          cfg={cfg}
        />
      </g>
    );
  }
  let cursorY = rowTop;
  return (
    <g className="parent-portal-study-side-pane">
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
                const childStartY = cursorY;
                const open = Boolean(openGroupIds[group.id]);
                const groupFoldKeyValue = navGroupFoldKey(group.id);
                const groupOpening = Boolean(openingFoldIds[groupFoldKeyValue]);
                const groupClosing = Boolean(closingFoldIds[groupFoldKeyValue]);
                const groupDisplayOpen = open || groupClosing;
                const groupAccent = navGroupThemeColor(group.id, cfg);
                const groupGlowFilter = 'url(#parentPortalGlow)';
                const rows = groupDisplayOpen
                  ? navSectionsForGroup(group).flatMap((section) => {
                      const sectionRows: ReactNode[] = [];
                      const sectionFoldKeyValue = navSectionFoldKey(section.id);
                      const sectionOpen = !section.label || Boolean(openSectionIds[section.id]);
                      const sectionClosing = Boolean(closingFoldIds[sectionFoldKeyValue]);
                      const sectionOpening = Boolean(openingFoldIds[sectionFoldKeyValue]);
                      const sectionDisplayOpen = !section.label || sectionOpen || sectionClosing;
                      if (section.label) {
                        const sectionY = cursorY;
                        cursorY += sectionH;
                        sectionRows.push(
                          <NavSectionHeader
                            key={section.id}
                            label={section.label}
                            icon={navSectionIcon(section)}
                            open={sectionOpen}
                            x={outerPad + 8}
                            w={leftW - 8}
                            y={sectionY}
                            h={sectionH}
                            accentColor={groupAccent}
                            glowFilter={groupGlowFilter}
                            visible={parentPortalNavControlIsVisible(
                              sectionY,
                              sectionH,
                              safeNavScroll,
                              navViewportY,
                              navViewportH
                            )}
                            onToggle={() => toggleNavSection(section.id)}
                            cfg={cfg}
                          />
                        );
                      }
                      if (!sectionDisplayOpen) return sectionRows;
                      const itemRows: ReactNode[] = [];
                      for (const item of section.items) {
                        const itemY = cursorY;
                        cursorY += rowStep;
                        const nested = Boolean(section.label);
                        const rowInset = nested ? 30 : 8;
                        itemRows.push(
                          <NavRow
                            key={navItemKey(item)}
                            item={item}
                            active={navItemMatches(item, activeNavRouteKey, activeNavLabel)}
                            x={outerPad + rowInset}
                            w={leftW - rowInset}
                            y={itemY}
                            rowH={rowH}
                            iconSize={iconSize}
                            nested={nested}
                            branchColor={groupAccent}
                            visible={parentPortalNavControlIsVisible(
                              itemY,
                              rowH,
                              safeNavScroll,
                              navViewportY,
                              navViewportH
                            )}
                            onSelect={() => onNavItemSelect(item)}
                            cfg={cfg}
                          />
                        );
                      }
                      if (section.label) {
                        sectionRows.push(
                          <g
                            key={`${section.id}:items`}
                            className={navFoldoutClassName(sectionOpening, sectionClosing)}
                          >
                            {itemRows}
                          </g>
                        );
                      } else {
                        sectionRows.push(...itemRows);
                      }
                      if (section.label) cursorY += sectionGap;
                      return sectionRows;
                    })
                  : null;
                const childEndY = cursorY;
                const childRailH = Math.max(0, childEndY - childStartY - 2);
                cursorY += groupGap;
                return (
                  <g key={group.id}>
                    {groupDisplayOpen ? (
                      <g className={navFoldoutClassName(groupOpening, groupClosing)}>
                        {childRailH > 0 ? (
                          <>
                            <path
                              d={cutRectPath(outerPad + 22, childStartY + 3, leftW - 35, childRailH, 9)}
                              fill="rgba(2, 12, 20, 0.34)"
                              stroke={groupAccent}
                              strokeWidth={0.85}
                              strokeOpacity={0.38}
                              pointerEvents="none"
                            />
                            <path
                              d={`M ${outerPad + 28} ${childStartY + 9} V ${childStartY + childRailH - 6}`}
                              stroke={groupAccent}
                              strokeWidth={1.45}
                              strokeLinecap="round"
                              opacity={0.58}
                              pointerEvents="none"
                            />
                          </>
                        ) : null}
                        {rows}
                      </g>
                    ) : null}
                    <NavGroupHeader
                      group={group}
                      open={open}
                      x={outerPad}
                      w={leftW}
                      y={groupY}
                      h={groupH}
                      visible={parentPortalNavControlIsVisible(
                        groupY,
                        groupH,
                        safeNavScroll,
                        navViewportY,
                        navViewportH
                      )}
                      onToggle={() => handleNavGroupToggle(group.id)}
                      cfg={cfg}
                    />
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
              stroke={activeNavThemeColor}
              strokeWidth={1.4}
              strokeLinecap="round"
              opacity={0.28}
            />
            <path
              d={`M ${outerPad + leftW - 10} ${thumbY} V ${thumbY + thumbH}`}
              stroke={activeNavThemeColor}
              strokeWidth={3.4}
              strokeLinecap="round"
              opacity={0.82}
              filter="url(#parentPortalGlow)"
            />
          </g>
        ) : null}
      </SurfacePanel>
      <AssistantDock
        x={outerPad}
        y={assistantY}
        w={leftW}
        h={assistantDockH}
        open={assistantOpen}
        onOpen={onAssistantOpen}
        cfg={cfg}
      />
    </g>
  );
}

function AssistantDock({
  x,
  y,
  w,
  h,
  open,
  onOpen,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  open: boolean;
  onOpen: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const color = toneColor(open ? 'purple' : 'cyan', cfg);
  const iconSize = 30;
  const label = 'AI ASSISTANT';
  const labelSize = 16;
  const labelWidth = label.length * labelSize * 0.58;
  const groupGap = 10;
  const groupW = iconSize + groupGap + labelWidth;
  const centerY = y + h / 2;
  const groupX = x + w / 2 - groupW / 2;
  const iconX = groupX;
  const iconY = centerY - iconSize / 2;
  const textX = iconX + iconSize + groupGap;
  const labelBaseline = centerY + labelSize * 0.35;
  return (
    <SurfacePanel
      x={x}
      y={y}
      w={w}
      h={h}
      tone={open ? 'purple' : 'cyan'}
      frame="deckSide"
      selected={open}
      frameCornerThicknessScale={0.34}
      frameOuterTabWidth={Math.min(42, Math.max(32, w * 0.18))}
      frameInnerTabWidth={Math.min(42, Math.max(32, w * 0.18))}
      onClick={onOpen}
      ariaLabel="Open AI assistant"
      cfg={cfg}
    >
      <rect x={x} y={y} width={w} height={h} fill="transparent" pointerEvents="all" />
      <AiMemorySetBrainIcon x={iconX} y={iconY} width={iconSize} height={iconSize} color={color} />
      <text
        x={textX}
        y={labelBaseline}
        fontSize={labelSize}
        fontWeight={980}
        fill={cfg.colors.bodyText}
        pointerEvents="none"
      >
        {label}
      </text>
      <path
        d={`M ${groupX} ${y + h - 14} H ${groupX + groupW}`}
        stroke={color}
        strokeWidth={0.9}
        opacity={open ? 0.78 : 0.44}
        pointerEvents="none"
      />
    </SurfacePanel>
  );
}

function AssistantQuickActionPanel({
  x,
  y,
  w,
  h,
  navGroups,
  selectedActionId,
  commandAvailable,
  onNewChat,
  onActionSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  navGroups: NavGroup[];
  selectedActionId: AssistantQuickActionId | null;
  commandAvailable: boolean;
  onNewChat: () => void;
  onActionSelect: (actionId: AssistantQuickActionId) => void;
  cfg: ParentPortalSvgControls;
}) {
  const assistantGroups = assistantQuickActionNavGroups(navGroups);
  const [activePanelTab, setActivePanelTab] = useState<AssistantPanelTab>(ASSISTANT_PANEL_TAB.QuickAction);
  const [openActionGroupIds, setOpenActionGroupIds] = useState(() =>
    Object.fromEntries(assistantGroups.map((group) => [group.id, true]))
  );
  const tabY = y + 24;
  const tabH = 30;
  const tabGap = 6;
  const tabW = (w - 24 - tabGap) / 2;
  const rowTop = y + 66;
  const rowH = 38;
  const rowStep = 40;
  const iconSize = 34;
  const groupH = 46;
  const groupGap = 7;
  const panelBottom = y + h - 18;
  const panelRows: ReactNode[] = [];
  let cursorY = rowTop;

  const toggleActionGroup = (groupId: string) => {
    setOpenActionGroupIds((current) => ({
      ...current,
      [groupId]: !(current[groupId] ?? true),
    }));
  };

  panelRows.push(
    <g key="assistant:panel-tabs" role="tablist" aria-label="Assistant side panel">
      <AssistantPanelTabButton
        x={x + 12}
        y={tabY}
        w={tabW}
        h={tabH}
        label="History"
        active={activePanelTab === ASSISTANT_PANEL_TAB.History}
        onSelect={() => setActivePanelTab(ASSISTANT_PANEL_TAB.History)}
        cfg={cfg}
      />
      <AssistantPanelTabButton
        x={x + 12 + tabW + tabGap}
        y={tabY}
        w={tabW}
        h={tabH}
        label="Quick Action"
        active={activePanelTab === ASSISTANT_PANEL_TAB.QuickAction}
        onSelect={() => setActivePanelTab(ASSISTANT_PANEL_TAB.QuickAction)}
        cfg={cfg}
      />
    </g>
  );

  if (cursorY + rowH <= panelBottom) {
    panelRows.push(
      <NavRow
        key="assistant:new-chat"
        item={ASSISTANT_NEW_CHAT_NAV_ITEM}
        active={false}
        x={x + 8}
        w={w - 8}
        y={cursorY}
        rowH={rowH}
        iconSize={iconSize}
        branchColor={cfg.colors.cyan}
        ariaLabel="Start new MIA chat"
        disabled={!commandAvailable}
        onSelect={onNewChat}
        cfg={cfg}
      />
    );
    cursorY += rowStep + groupGap;
  }

  if (activePanelTab === ASSISTANT_PANEL_TAB.History) {
    ASSISTANT_QUICK_ACTIONS.forEach((action) => {
      if (cursorY + rowH > panelBottom) return;
      const itemY = cursorY;
      cursorY += rowStep;
      panelRows.push(
        <NavRow
          key={`assistant:history:${action.id}`}
          item={assistantHistoryNavItem(action)}
          active={false}
          x={x + 8}
          w={w - 8}
          y={itemY}
          rowH={rowH}
          iconSize={iconSize}
          branchColor={toneColor(action.tone, cfg)}
          ariaLabel={`${action.label} history`}
          disabled={!commandAvailable}
          onSelect={() => onActionSelect(action.id)}
          cfg={cfg}
        />
      );
    });
    return (
      <SurfacePanel x={x} y={y} w={w} h={h} tone="cyan" frame="deckSide" selected cfg={cfg}>
        {panelRows}
      </SurfacePanel>
    );
  }

  assistantGroups.forEach((group) => {
    if (cursorY + groupH > panelBottom) return;
    const groupY = cursorY;
    const open = openActionGroupIds[group.id] ?? true;
    const groupAccent = navGroupThemeColor(group.id, cfg);
    cursorY += groupH;

    const itemRows: ReactNode[] = [];
    const childStartY = cursorY;
    if (open) {
      for (const item of group.items) {
        const itemY = cursorY;
        if (itemY + rowH > panelBottom) break;
        cursorY += rowStep;
        const actionId = assistantActionIdForNavItem(item);
        const actionLabel = actionId ? assistantQuickActionById(actionId)?.label : null;
        itemRows.push(
          <NavRow
            key={navItemKey(item)}
            item={item}
            active={actionId === selectedActionId}
            x={x + 8}
            w={w - 8}
            y={itemY}
            rowH={rowH}
            iconSize={iconSize}
            branchColor={groupAccent}
            ariaLabel={`Ask MIA about ${actionLabel ?? item.label}`}
            disabled={!commandAvailable}
            onSelect={() => {
              if (actionId) onActionSelect(actionId);
            }}
            cfg={cfg}
          />
        );
      }
    }

    const childEndY = cursorY;
    const childRailH = Math.max(0, childEndY - childStartY - 2);
    cursorY += groupGap;

    panelRows.push(
      <g key={group.id}>
        {open && childRailH > 0 ? (
          <g pointerEvents="none">
            <path
              d={cutRectPath(x + 22, childStartY + 3, w - 35, childRailH, 9)}
              fill="rgba(2, 12, 20, 0.34)"
              stroke={groupAccent}
              strokeWidth={0.85}
              strokeOpacity={0.38}
            />
            <path
              d={`M ${x + 28} ${childStartY + 9} V ${childStartY + childRailH - 6}`}
              stroke={groupAccent}
              strokeWidth={1.45}
              strokeLinecap="round"
              opacity={0.58}
            />
          </g>
        ) : null}
        {itemRows}
        <NavGroupHeader
          group={group}
          open={open}
          x={x}
          w={w}
          y={groupY}
          h={groupH}
          onToggle={() => toggleActionGroup(group.id)}
          cfg={cfg}
        />
      </g>
    );
  });

  return (
    <SurfacePanel x={x} y={y} w={w} h={h} tone="cyan" frame="deckSide" selected cfg={cfg}>
      {panelRows}
    </SurfacePanel>
  );
}

function AssistantPanelTabButton({
  x,
  y,
  w,
  h,
  label,
  active,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  active: boolean;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const fill = active
    ? PARENT_PORTAL_TAB_SURFACE_FILL.active
    : hovered
      ? PARENT_PORTAL_TAB_SURFACE_FILL.hover
      : PARENT_PORTAL_TAB_SURFACE_FILL.idle;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="tab"
      tabIndex={0}
      aria-label={label}
      aria-selected={active}
      onClick={onSelect}
      onKeyDown={(event) => {
        if (event.key !== 'Enter' && event.key !== ' ') return;
        event.preventDefault();
        onSelect();
      }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      <title>{label}</title>
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={0}
        fill={fill}
        stroke={active || hovered ? cfg.colors.cyan : cfg.colors.panelStroke}
        strokeWidth={active ? 1.15 : 0.82}
        opacity={active ? 1 : 0.82}
      />
      <path
        d={`M ${x + 8} ${y + 1.5} H ${x + w - 8}`}
        stroke={active ? cfg.colors.cyan : cfg.colors.panelStroke}
        strokeWidth={active ? 1.65 : 0.9}
        strokeLinecap="round"
        opacity={active ? 0.95 : hovered ? 0.52 : 0.28}
      />
      <path
        d={`M ${x + 10} ${y + h - 3} H ${x + w - 10}`}
        stroke={cfg.colors.cyan}
        strokeWidth={active ? 2.05 : 1.1}
        strokeLinecap="round"
        opacity={active ? 0.92 : hovered ? 0.46 : 0.24}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={10.8}
        fontWeight={960}
        fill={cfg.colors.bodyText}
      >
        {truncateTextForWidth(label.toUpperCase(), w - 16, 10.8, 0.58)}
      </text>
    </g>
  );
}

type ManageControlOption = {
  readonly label: string;
  readonly detail: string;
  readonly enabled: boolean;
  readonly tone: Tone;
};

type ManageControlAction = {
  readonly label: string;
  readonly detail: string;
  readonly tone: Tone;
};

type ManageControlSpec = {
  readonly title: string;
  readonly devices: readonly string[];
  readonly modes: readonly ManageControlAction[];
  readonly options: readonly ManageControlOption[];
  readonly actions: readonly ManageControlAction[];
  readonly status: readonly ManageControlAction[];
};

type ManageLaneId = 'portal' | 'childPolicy' | 'deviceOps';
type ManageScopeId = ManageTargetSelection['scope'];

type ManageTargetChoice = {
  readonly label: string;
  readonly detail: string;
  readonly tone: Tone;
  readonly scope?: ManageScopeId;
};

type ManageWorkspaceKind = 'portal' | 'account' | 'data' | 'ai' | 'policy';
type ManageWorkspaceTarget = 'family' | 'perDevice' | 'portal';

type ManageWorkspaceTab = {
  readonly id: string;
  readonly label: string;
  readonly icon: IconComponent;
  readonly tone: Tone;
};

type ManageWorkspaceCard = {
  readonly label: string;
  readonly value: string;
  readonly body: string;
  readonly tone: Tone;
  readonly action?: {
    readonly label: string;
    readonly routePath: string;
  };
};

type ManageWorkspaceTargetOption = {
  readonly id: ManageWorkspaceTarget;
  readonly label: string;
  readonly detail: string;
  readonly tone: Tone;
};

const MANAGE_LANES: readonly {
  readonly id: ManageLaneId;
  readonly label: string;
  readonly detail: string;
  readonly tone: Tone;
}[] = [
  { id: 'portal', label: 'PORTAL', detail: '', tone: 'cyan' },
  { id: 'childPolicy', label: 'POLICY', detail: '', tone: 'gold' },
  { id: 'deviceOps', label: 'DEVICE', detail: '', tone: 'purple' },
];

function manageLaneForRoutePath(routePath: string | undefined): ManageLaneId | null {
  if (typeof routePath !== 'string') return null;
  const route = portalRouteFromHashPath(routePath);
  if (route === null) return null;
  return parentPortalManageLaneForRoute(route);
}

function guideRoutePathForManageKey(activeNavLabel: string, selectedControlName: string): string {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (
    key.includes('support') ||
    key.includes('diagnostic') ||
    key.includes('subscription') ||
    key.includes('entitlement') ||
    key.includes('platform') ||
    key.includes('update') ||
    key.includes('install') ||
    key.includes('setting')
  )
    return '#/start';
  if (key.includes('device') || key.includes('lan') || key.includes('capability')) return '#/start';
  if (
    key.includes('browser') ||
    key.includes('rule') ||
    key.includes('policy') ||
    key.includes('schedule') ||
    key.includes('approval') ||
    key.includes('enforce')
  )
    return '#/policy';
  if (key.includes('drive') || key.includes('export') || key.includes('private') || key.includes('remote'))
    return '#/privacy-design';
  if (key.includes('ai') || key.includes('api') || key.includes('memory')) return '#/ai-guide';
  if (key.includes('report')) return '#/reports-guide';
  if (
    key.includes('screen') ||
    key.includes('apps-games') ||
    key.includes('app-use') ||
    key.includes('app-game') ||
    key.includes('network') ||
    key.includes('alert') ||
    key.includes('notification') ||
    key.includes('channel') ||
    key.includes('audit')
  )
    return '#/activity';
  return '#/start';
}

function guideRoutePathForManageTab(activeNavLabel: string, selectedControlName: string, tabId: string): string {
  const topicId = policyGuideTopicIdForManageKey(activeNavLabel, selectedControlName);
  if (!topicId) return guideRoutePathForManageKey(activeNavLabel, selectedControlName);
  const page = policyGuidePageForManageTab(tabId);
  const query = new URLSearchParams({
    [PARENT_PORTAL_GUIDE_QUERY.Topic]: topicId,
    [PARENT_PORTAL_GUIDE_QUERY.Page]: String(page),
  });
  return `#/policy?${query.toString()}`;
}

function policyGuideTopicIdForManageKey(activeNavLabel: string, selectedControlName: string): string | null {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (key.includes('game')) return PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Games;
  if (key.includes('app')) return PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Apps;
  if (key.includes('screen') || key.includes('network')) return PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.ScreenNetwork;
  if (key.includes('tracking') || key.includes('location')) return PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Tracking;
  if (
    key.includes('browser') ||
    key.includes('rule') ||
    key.includes('policy') ||
    key.includes('schedule') ||
    key.includes('budget') ||
    key.includes('approval') ||
    key.includes('audit') ||
    key.includes('enforce')
  ) {
    return PARENT_PORTAL_POLICY_GUIDE_TOPIC_IDS.Browser;
  }
  return null;
}

function policyGuidePageForManageTab(tabId: string): number {
  if (tabId === 'schedule') return PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES.Schedule;
  if (tabId === 'budget') return PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES.Budget;
  if (tabId === 'approvals') return PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES.Approvals;
  if (tabId === 'audit') return PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES.Audit;
  return PARENT_PORTAL_POLICY_GUIDE_TAB_PAGES.Rules;
}

function guideRouteFocusFromHash(): { topicId: string; page: number } | null {
  if (typeof window === 'undefined') return null;
  const hashQueryStart = window.location.hash.indexOf('?');
  if (hashQueryStart < 0) return null;
  const params = new URLSearchParams(window.location.hash.slice(hashQueryStart + 1));
  const topicId = params.get(PARENT_PORTAL_GUIDE_QUERY.Topic);
  if (!topicId) return null;
  const rawPage = params.get(PARENT_PORTAL_GUIDE_QUERY.Page);
  const page = rawPage === null ? 0 : Number.parseInt(rawPage, 10);
  return { topicId, page: Number.isFinite(page) ? page : 0 };
}

function manageLaneForKey(activeNavLabel: string, selectedControlName: string): ManageLaneId {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (
    key.includes('channel') ||
    key.includes('drive') ||
    key.includes('api-key') ||
    key.includes('api-providers') ||
    key.includes('notification') ||
    key.includes('alert') ||
    key.includes('subscription') ||
    key.includes('entitlement') ||
    key.includes('export') ||
    key.includes('retention') ||
    key.includes('support') ||
    key.includes('diagnostic') ||
    key.includes('audit') ||
    key.includes('family-setting') ||
    key.includes('settings-rules')
  ) {
    return 'portal';
  }
  if (
    key.includes('lan') ||
    key.includes('capability') ||
    key.includes('remote') ||
    key.includes('platform') ||
    key.includes('update') ||
    key.includes('install') ||
    key.includes('devices')
  ) {
    return 'deviceOps';
  }
  return 'childPolicy';
}

function manageLaneForControl(control: ControlArea | QuickControl): ManageLaneId {
  const routeLane = manageLaneForRoutePath(control.routePath);
  if (routeLane) return routeLane;
  return manageLaneForKey(
    control.name,
    `${control.category ?? ''} ${control.subcategory ?? ''} ${control.routePath ?? ''}`
  );
}

function manageScopeForLane(lane: ManageLaneId): ManageScopeId {
  return lane === 'portal' ? 'global' : 'perDevice';
}

function isBrowserManageKey(activeNavLabel: string, selectedControlName: string): boolean {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  return key.includes('browser') || key.includes('web');
}

function manageScopeChoicesForLane(lane: ManageLaneId): readonly ManageTargetChoice[] {
  if (lane === 'portal') {
    return [{ label: 'Parent profile', detail: 'Portal setting only.', tone: 'cyan', scope: 'global' }];
  }
  if (lane === 'deviceOps') {
    return [
      { label: 'All devices', detail: 'Apply to every paired child device.', tone: 'cyan', scope: 'global' },
      { label: 'Selected device', detail: 'Send only to one child device.', tone: 'gold', scope: 'perDevice' },
    ];
  }
  return [
    { label: 'Family', detail: 'Base rule for all children.', tone: 'cyan', scope: 'global' },
    { label: 'Child override', detail: 'Override one child device.', tone: 'gold', scope: 'perDevice' },
  ];
}

function manageControlDisplayTitle(title: string): string {
  const key = assetKey(title);
  if (key.includes('lan-pairing')) return 'LAN Pairing';
  if (key.includes('api-provider')) return 'API Providers';
  if (key.includes('api-key')) return 'API Keys';
  if (key.includes('ai-setup') || key.includes('ai-runtime')) return 'AI Setup';
  if (key.includes('memory-setup') || key.includes('memory-set')) return 'Memory';
  if (key.includes('platform')) return 'Platforms';
  if (key.includes('install') || key.includes('update')) return 'Install Updates';
  if (key.includes('capability')) return 'Capability Status';
  if (key.includes('remote')) return 'Remote Access';
  if (key.includes('device')) return 'Devices';
  if (key.includes('rule') || key.includes('policy')) return 'Rule';
  if (key.includes('report')) return 'Report';
  if (key.includes('browser')) return 'Browser';
  if (key.includes('screen')) return 'Screen';
  if (key.includes('memory')) return 'Memory';
  if (key.includes('ai')) return 'AI';
  return (
    title
      .replace(/\s+Setup$/i, '')
      .replace(/\s+Set$/i, '')
      .trim() || title
  );
}

function manageGlobalTargetLabel(lane: ManageLaneId): string {
  if (lane === 'portal') return 'Parent profile';
  if (lane === 'deviceOps') return 'All devices';
  return 'Family';
}

function manageDeviceChoices(devices: readonly string[], runtimeSlots: readonly DeviceSlot[] = []): readonly string[] {
  const choices: string[] = [];
  for (const slot of runtimeSlots) {
    const label = slot.label || slot.device?.name || slot.value;
    if (label) choices.push(label);
  }
  choices.push(...devices.filter((device) => !assetKey(device).includes('family-default')));
  return uniqueManageDeviceChoices(choices).slice(0, 6);
}

function uniqueManageDeviceChoices(choices: readonly string[]): readonly string[] {
  const seen = new Set<string>();
  return choices.filter((choice) => {
    const key = assetKey(choice);
    if (!key || seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function manageDefaultDeviceSelection(spec: ManageControlSpec, runtimeSlots: readonly DeviceSlot[] = []): string {
  void spec;
  void runtimeSlots;
  return '';
}

function manageInitialScopeForSpec(
  lane: ManageLaneId,
  spec: ManageControlSpec,
  runtimeSlots: readonly DeviceSlot[] = []
): ManageScopeId {
  if (lane === 'portal') return 'global';
  return manageDeviceChoices(spec.devices, runtimeSlots).length > 0 ? manageScopeForLane(lane) : 'global';
}

function isLanPairingManageTitle(title: string): boolean {
  return assetKey(title).includes('lan-pairing');
}

function isReportsManageTitle(title: string): boolean {
  return assetKey(title).includes('report');
}

function isAppGameDashboardManageContext(activeNavLabel: string, selectedControlName: string, title: string): boolean {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)} ${assetKey(title)}`;
  return (
    key.includes('app-game-sessions') ||
    key.includes('apps-games') ||
    key.includes('app-and-game') ||
    key.includes('app-game') ||
    key.includes('app-use')
  );
}

function emptyReportPlanSeatSlot(slotIndex: number): DeviceSlot {
  return {
    value: `report-empty-seat-${slotIndex + 1}`,
    label: '',
    status: 'empty',
    slotIndex,
  };
}

function reportPlanSeatSlots(planSeatLimit: number): DeviceSlot[] {
  const seatLimit = clampValue(Math.round(planSeatLimit), 1, ACTIVITY_REPORT_MAX_CHILD_DEVICE_SEATS);
  const slots: DeviceSlot[] = [];
  while (slots.length < seatLimit) {
    slots.push(emptyReportPlanSeatSlot(slots.length));
  }
  return slots;
}

function reportSelectedSlot(slots: readonly DeviceSlot[], selection: ManageTargetSelection): DeviceSlot | undefined {
  return slots.find((slot) => slotMatchesManageTargetSelection(slot, selection));
}

function selectedDeviceIdentity(slot: DeviceSlot | null | undefined): string {
  return slot?.device?.name ?? slot?.label ?? '';
}

function reportSelectedSlotValue(slots: readonly DeviceSlot[], selection: ManageTargetSelection): string | undefined {
  return reportSelectedSlot(slots, selection)?.value;
}

function slotMatchesManageTargetSelection(slot: DeviceSlot, selection: ManageTargetSelection): boolean {
  const deviceId = selection.deviceId.trim();
  if (deviceId.length > 0) {
    return slot.value === deviceId;
  }
  const deviceLabel = selection.device.trim();
  return deviceLabel.length > 0 && (slot.label === deviceLabel || slot.device?.name === deviceLabel);
}

function reportDeviceSelectionAvailable(slots: readonly DeviceSlot[], selection: ManageTargetSelection): boolean {
  return Boolean(reportSelectedSlot(slots, selection));
}

function selectedManageTargetSelectionForSlot(
  selection: ManageTargetSelection,
  slot: DeviceSlot | null | undefined
): ManageTargetSelection {
  return withManageTargetSelectionDevice(selection, slot?.value ?? '', selectedDeviceIdentity(slot));
}

type ManageDeviceGridConfigOverride = NonNullable<DeviceChoiceGridProps['config']>;
const FAMILY_DEVICE_SCOPE_ICONS: NonNullable<DeviceChoiceGridProps['scopeIcons']> = {
  lan: { href: parentNavIconAssetUrls.FamilyIcon },
  parent: { href: parentNavIconAssetUrls.DevicesMultiScreenIcon },
  portal: { href: parentNavIconAssetUrls.PortalGatewayIcon },
};
const MANAGE_DEVICE_GRID_CELL_W = 104;
const MANAGE_DEVICE_GRID_CELL_H = 40;
const MANAGE_DEVICE_GRID_CELL_MAX_W = 148;
const MANAGE_DEVICE_GRID_GAP_X = 8;
const MANAGE_DEVICE_GRID_GAP_Y = 8;

function mergeManageDeviceGridConfig(
  base: ManageDeviceGridConfigOverride,
  override?: ManageDeviceGridConfigOverride
): ManageDeviceGridConfigOverride {
  if (!override) return base;

  return {
    ...base,
    ...override,
    debug: { ...base.debug, ...override.debug },
    preview: { ...base.preview, ...override.preview },
    svg: { ...base.svg, ...override.svg },
    statusOrder: { ...base.statusOrder, ...override.statusOrder },
    connector: { ...base.connector, ...override.connector },
    layout: { ...base.layout, ...override.layout },
    text: { ...base.text, ...override.text },
  };
}

function manageDeviceGridConfig(
  width: number,
  height: number,
  override?: ManageDeviceGridConfigOverride
): ManageDeviceGridConfigOverride {
  const compact = width < 480;
  return mergeManageDeviceGridConfig(
    {
      debug: {
        showBounds: false,
      },
      preview: { background: 'transparent', padding: 0 },
      svg: { width, height, inset: 0 },
      layout: {
        legendX: compact ? 10 : 14,
        legendDotR: 4.4,
        legendItemGap: 19,
        legendTextOffset: 12,
        legendY: compact ? 58 : 10,
        titleY: compact ? 8 : 10,
        cellW: MANAGE_DEVICE_GRID_CELL_W,
        cellH: MANAGE_DEVICE_GRID_CELL_H,
        cellMaxW: MANAGE_DEVICE_GRID_CELL_MAX_W,
        gapX: MANAGE_DEVICE_GRID_GAP_X,
        gapY: MANAGE_DEVICE_GRID_GAP_Y,
        addButtonSize: 22,
        addButtonCutoutPad: 3,
        selectedInfoH: 36,
        selectedInfoIconBox: 23,
        selectedInfoIconGap: 8,
        selectedInfoYGap: 8,
        scopeIconSize: compact ? 14 : 18,
        scopeIconGap: compact ? 4 : 7,
        scopeOptionW: compact ? Math.max(88, (width - 16) / 3) : 148,
      },
      text: {
        optionSize: compact ? 11.5 : 13,
        legendSize: compact ? 10.5 : 13,
        selectedInfoSize: 14,
      },
    },
    override
  );
}

function activityScopeToggleConfig(width: number, height = 66) {
  const safeWidth = Math.max(1, width);
  const compact = safeWidth < 260;
  const titleBoxMinWidth = compact ? 40 : 74;
  const titleBoxPaddingX = compact ? 2 : 12;
  const titleReserve = compact ? 48 : 86;
  return {
    svg: {
      width: safeWidth,
      height,
      viewportInset: compact ? 0 : 6,
    },
    layout: {
      titleAnchorX: 0,
      titleBoxY: 17,
      titleBoxMinWidth,
      titleBoxPaddingX,
      titleBoxHeight: 32,
      trackY: 15,
      trackMinWidth: Math.max(1, safeWidth - titleReserve),
      trackHeight: 36,
      optionPaddingX: compact ? 2 : 15,
      outerPaddingRight: 0,
    },
    text: {
      titleFontSize: compact ? 11.5 : 14,
      optionFontSize: compact ? 11.5 : 14,
    },
  };
}

const LAN_PAIRING_BASIC_PORTAL_SLOT_LIMIT = 4;
const ACTIVITY_REPORT_BASIC_CHILD_DEVICE_SEATS = Math.max(1, LAN_PAIRING_BASIC_PORTAL_SLOT_LIMIT - 1);
const ACTIVITY_REPORT_MAX_CHILD_DEVICE_SEATS = 10;
const ACTIVITY_REPORT_SELECTOR_BASE_H = 146;
const ACTIVITY_REPORT_SELECTOR_ROW_H = MANAGE_DEVICE_GRID_CELL_H + MANAGE_DEVICE_GRID_GAP_Y;

type LanPairingDetailTabId = 'info' | 'pair' | 'update' | 'capability';

type LanPairingDetailTab = {
  readonly id: LanPairingDetailTabId;
  readonly label: string;
  readonly icon: IconComponent;
  readonly tone: Tone;
};

const LAN_PAIRING_DETAIL_TABS: readonly [LanPairingDetailTab, ...LanPairingDetailTab[]] = [
  { id: 'info', label: 'Info', icon: OverviewListIcon, tone: 'cyan' },
  { id: 'pair', label: 'Pair', icon: PortalGatewayIcon, tone: 'gold' },
  { id: 'update', label: 'Update', icon: UpdatesSyncDocumentIcon, tone: 'gold' },
  { id: 'capability', label: 'Capability', icon: ScreenAnalysisIcon, tone: 'purple' },
];

type LanPairingDetailRow = {
  readonly label: string;
  readonly value: string;
  readonly tone: Tone;
};

type LanPairingContextRow = {
  readonly label: string;
  readonly value: string;
  readonly tone: Tone;
};

type LanPairingPendingDeviceIdentity = {
  readonly householdName: string;
  readonly detectedName: string;
  readonly deviceKind: DeviceKind;
};

type LanPairingPendingDeviceIdentities = Record<string, LanPairingPendingDeviceIdentity>;

type LanPairingActionId = 'pair' | 'add' | 'select' | 'trust' | 'ignore' | 'restore' | 'revoke';

type LanPairingActionButton = {
  readonly id: LanPairingActionId;
  readonly label: string;
  readonly tone: Tone;
  readonly enabled: boolean;
  readonly command: AgentCommandName;
  readonly payload: Record<string, string> | null;
  readonly status: string;
};

function lanPairingMissingDeviceValue(value?: string): string {
  const trimmed = value?.trim();
  return trimmed ? trimmed : 'Not reported';
}

function lanPairingHumanLabel(value?: string): string {
  const source = value?.trim();
  if (!source) return 'Unknown';
  return source
    .split(/[-_\s]+/)
    .filter(Boolean)
    .map((part) => part.slice(0, 1).toUpperCase() + part.slice(1))
    .join(' ');
}

function lanPairingOptionalHumanLabel(value?: string): string {
  const source = value?.trim();
  return source ? lanPairingHumanLabel(source) : 'Not reported';
}

function lanPairingDeviceName(slot: DeviceSlot): string {
  return lanPairingMissingDeviceValue(slot.device?.name ?? slot.label);
}

function lanPairingDetectedDeviceName(slot: DeviceSlot): string {
  const detectedName = lanPairingMissingDeviceValue(
    slot.device?.detectedName || slot.device?.hostname || slot.device?.model
  );
  return detectedName === 'Not reported' ? lanPairingDeviceName(slot) : detectedName;
}

function lanPairingHouseholdNameDraftFor(slot: DeviceSlot): string {
  const savedName = slot.device?.householdName?.trim();
  if (savedName) return savedName;
  const detectedName = lanPairingDetectedDeviceName(slot);
  const displayedName = slot.label.trim();
  if (!displayedName || displayedName.toLowerCase().startsWith('lan ')) return '';
  return detectedName === 'Not reported' || displayedName !== detectedName ? displayedName : '';
}

function lanPairingDeviceKindDraftFor(slot: DeviceSlot): DeviceKind {
  return slot.device?.parentDeviceKind ?? slot.device?.type ?? 'unknown';
}

function lanPairingDeviceKindOptionLabel(kind: DeviceKind): string {
  if (kind === 'unknown') return 'Unknown';
  return lanPairingHumanLabel(kind);
}

function lanPairingDeviceIdentityKey(slot: DeviceSlot | null): string {
  return slot?.device?.id || slot?.value || '';
}

function lanPairingCanEditDeviceIdentity(slot: DeviceSlot | null): boolean {
  return Boolean(slot?.device && slot.status !== 'empty');
}

function applyLanPairingPendingIdentities(
  slots: readonly DeviceSlot[],
  pendingIdentities: LanPairingPendingDeviceIdentities
): readonly DeviceSlot[] {
  if (Object.keys(pendingIdentities).length === 0) return slots;
  return slots.map((slot) => {
    const identity = pendingIdentities[lanPairingDeviceIdentityKey(slot)];
    return identity ? applyLanPairingPendingIdentity(slot, identity) : slot;
  });
}

function applyLanPairingPendingIdentity(slot: DeviceSlot, identity: LanPairingPendingDeviceIdentity): DeviceSlot {
  if (!slot.device) return slot;
  const householdName = identity.householdName.trim();
  if (!householdName) return slot;
  return {
    ...slot,
    label: householdName,
    device: {
      ...slot.device,
      name: householdName,
      householdName,
      detectedName: slot.device.detectedName || identity.detectedName,
      parentDeviceKind: identity.deviceKind,
      type: identity.deviceKind,
    },
  };
}

function lanPairingDevicePlatform(slot: DeviceSlot): string {
  return lanPairingHumanLabel(slot.platform ?? slot.device?.platform);
}

function lanPairingDeviceType(slot: DeviceSlot): string {
  return lanPairingHumanLabel(slot.device?.type);
}

function lanPairingDeviceSource(slot: DeviceSlot): string {
  if (lanPairingDeviceHasAgent(slot) && (slot.device?.ip || slot.device?.mac)) return 'Child agent + LAN evidence';
  if (lanPairingDeviceHasAgent(slot)) return 'Local child agent';
  if (lanPairingDeviceIsInfrastructure(slot)) return 'Network infrastructure';
  if (slot.device?.ip || slot.device?.mac || slot.device?.hostname) return 'LAN discovered';
  return 'Service state';
}

function lanPairingDeviceControlState(slot: DeviceSlot): string {
  const state = slot.device?.sourceState || slot.badge || slot.status;
  if (lanPairingDeviceIsInfrastructure(slot)) return 'Visible only';
  if (state === 'ignored') return 'Ignored';
  if (state === 'revoked') return 'Revoked';
  if (state === 'manual-required') return 'Manual required';
  if (state === 'stale') return 'Stale';
  if (state === 'unavailable' || state === 'degraded') return lanPairingHumanLabel(state);
  if (slot.status === 'offline' || state === 'offline') return 'Offline';
  if (slot.status === 'unsupported') return 'Visible only';
  if (slot.status === 'connected' && lanPairingDeviceHasAgent(slot)) return 'Policy target';
  if (lanPairingDeviceHasAgent(slot)) return 'Setup needed';
  return 'Visible only';
}

function lanPairingDeviceRoute(slot: DeviceSlot): string {
  return lanPairingMissingDeviceValue(slot.device?.routeId);
}

function lanPairingDeviceRouteState(slot: DeviceSlot): string {
  return lanPairingOptionalHumanLabel(slot.device?.routeState || slot.device?.readinessState);
}

function lanPairingDeviceCustody(slot: DeviceSlot): string {
  return lanPairingOptionalHumanLabel(slot.device?.custodyLabel || slot.device?.relayCacheCustody);
}

function lanPairingDeviceParentDecision(slot: DeviceSlot): string {
  return lanPairingOptionalHumanLabel(slot.device?.parentDecision);
}

function lanPairingDeviceSignedProof(slot: DeviceSlot): string {
  return lanPairingOptionalHumanLabel(slot.device?.signedProofCheck);
}

function lanPairingDeviceSignedProofState(slot: DeviceSlot): string {
  return lanPairingOptionalHumanLabel(slot.device?.signedProofState);
}

function lanPairingDeviceRouteSafety(slot: DeviceSlot): string {
  return lanPairingOptionalHumanLabel(slot.device?.routeSafety);
}

function lanPairingDeviceRouteSafetyResult(slot: DeviceSlot): string {
  const state = lanPairingOptionalHumanLabel(slot.device?.routeSafetyState);
  const reason = lanPairingOptionalHumanLabel(slot.device?.routeSafetyReason);
  return reason === 'Not reported' ? state : `${state} / ${reason}`;
}

function lanPairingDeviceRelayCache(slot: DeviceSlot): string {
  const check = lanPairingOptionalHumanLabel(slot.device?.relayCacheCheck);
  const state = lanPairingOptionalHumanLabel(slot.device?.relayCacheState);
  return check === 'Not reported' ? state : `${check} / ${state}`;
}

function lanPairingDeviceManualProof(slot: DeviceSlot): string {
  return lanPairingMissingDeviceValue(slot.device?.manualProof);
}

function lanPairingDeviceClaimsNotProved(slot: DeviceSlot): string {
  return lanPairingMissingDeviceValue(slot.device?.claimsNotProved);
}

function lanPairingDeviceAudit(slot: DeviceSlot): string {
  return lanPairingMissingDeviceValue(slot.device?.auditLabel);
}

function lanPairingDeviceRequirement(slot: DeviceSlot): string {
  return lanPairingMissingDeviceValue(slot.device?.requirementLabel);
}

function lanPairingDeviceEvidence(slot: DeviceSlot): string {
  return lanPairingMissingDeviceValue(slot.device?.evidenceLabel || slot.device?.sourceConfidence);
}

function lanPairingDeviceState(slot: DeviceSlot): string {
  return lanPairingHumanLabel(slot.device?.sourceState || slot.badge || slot.status);
}

function lanPairingDeviceHasAgent(slot: DeviceSlot): boolean {
  return Boolean(slot.device?.agentStatus?.trim());
}

function lanPairingDeviceIsInfrastructure(slot: DeviceSlot): boolean {
  return slot.badge === 'infrastructure' || slot.device?.type === 'router' || slot.device?.platform === 'router';
}

function lanPairingDevicePairedOrConfirmed(slot: DeviceSlot | null): boolean {
  if (!slot || slot.status === 'empty' || slot.status === 'unsupported' || lanPairingDeviceIsInfrastructure(slot)) {
    return false;
  }
  const trustState = slot.device?.trustState?.trim();
  const readinessState = slot.device?.readinessState?.trim();
  const sourceState = slot.device?.sourceState?.trim();
  return (
    lanPairingDeviceHasAgent(slot) &&
    (slot.status === 'connected' ||
      readinessState === 'ready-for-control' ||
      readinessState === 'ready' ||
      trustState === 'paired' ||
      sourceState === 'paired' ||
      sourceState === 'ready' ||
      Boolean(slot.device?.pairingId && slot.device?.proofDigest))
  );
}

function lanPairingDeviceReadyForControl(slot: DeviceSlot | null): boolean {
  if (!lanPairingDevicePairedOrConfirmed(slot) || !slot) {
    return false;
  }
  const readinessState = slot.device?.readinessState?.trim();
  const sourceState = slot.device?.sourceState?.trim();
  return (
    slot.status === 'connected' ||
    readinessState === 'ready-for-control' ||
    readinessState === 'ready' ||
    sourceState === 'ready'
  );
}

function lanPairingDeviceNeedsPairing(slot: DeviceSlot | null): boolean {
  return Boolean(
    slot &&
    slot.status !== 'empty' &&
    slot.status !== 'unsupported' &&
    !lanPairingDeviceIsInfrastructure(slot) &&
    !lanPairingDevicePairedOrConfirmed(slot)
  );
}

function lanPairingDetailTabsFor(slot: DeviceSlot | null): readonly LanPairingDetailTab[] {
  return LAN_PAIRING_DETAIL_TABS.filter((tab) => tab.id !== 'pair' || lanPairingDeviceNeedsPairing(slot));
}

function lanPairingDetailTabUnavailableReason(tab: LanPairingDetailTabId, slot: DeviceSlot | null): string | null {
  if (tab !== 'update' && tab !== 'capability') return null;
  if (!slot) return 'Select a LAN device first';
  if (lanPairingDeviceReadyForControl(slot)) return null;
  if (lanPairingDevicePairedOrConfirmed(slot)) return 'Paired device is not connected';
  if (lanPairingDeviceIsInfrastructure(slot)) return 'Network infrastructure is visible only';
  if (slot.status === 'unsupported') return 'This device cannot run the child agent';
  return 'Pair or connect a child agent first';
}

function lanPairingSetupStatusFor(slot: DeviceSlot): string {
  if (lanPairingDeviceReadyForControl(slot)) return 'Already connected';
  if (lanPairingDevicePairedOrConfirmed(slot)) return 'Already paired';
  if (slot.status === 'offline') return 'Device offline';
  if (lanPairingAddDeviceCommandPayload(slot)) return 'Ready for pairing challenge';
  if (lanPairingDeviceIsInfrastructure(slot)) return 'Infrastructure visible only';
  if (slot.status === 'unsupported') return 'Unsupported for child agent';
  return 'Install child agent first';
}

function lanPairingPairingNextStepFor(slot: DeviceSlot): string {
  if (lanPairingDeviceReadyForControl(slot)) return 'Update and Capability are available';
  if (lanPairingDevicePairedOrConfirmed(slot)) return 'Reconnect child agent before update or capability';
  if (slot.status === 'offline') return 'Bring the device online before pairing';
  if (lanPairingAddDeviceCommandPayload(slot)) return 'Start pairing, then submit child-agent proof';
  return 'Install or run the Ocentra child agent on this device';
}

function lanPairingPairingActionStateFor(slot: DeviceSlot): string {
  if (lanPairingPairCommandPayload(slot)) return 'Ready: sends signed add-device request';
  if (lanPairingDeviceReadyForControl(slot)) return 'Already connected';
  if (lanPairingDevicePairedOrConfirmed(slot)) return 'Already paired';
  if (slot.status === 'offline') return 'Unavailable: device offline';
  if (lanPairingDeviceIsInfrastructure(slot)) return 'Unavailable: infrastructure only';
  if (slot.status === 'unsupported') return 'Unavailable: child agent unsupported';
  return 'Unavailable: no child-agent route';
}

function LanPairingDeviceEditDialog({
  cfg,
  x,
  y,
  w,
  h,
  overlayX,
  overlayY,
  overlayW,
  overlayH,
  slot,
  detectedName,
  householdName,
  deviceKind,
  onHouseholdNameChange,
  onDeviceKindChange,
  onSave,
  onClose,
}: {
  cfg: ParentPortalSvgControls;
  x: number;
  y: number;
  w: number;
  h: number;
  overlayX: number;
  overlayY: number;
  overlayW: number;
  overlayH: number;
  slot: DeviceSlot | null;
  detectedName: string;
  householdName: string;
  deviceKind: DeviceKind;
  onHouseholdNameChange: (value: string) => void;
  onDeviceKindChange: (value: DeviceKind) => void;
  onSave: () => void;
  onClose: () => void;
}): ReactElement | null {
  if (!slot) return null;
  const borderColor = toneColor('cyan', cfg);
  const accentColor = toneColor('gold', cfg);
  const dialogFontFamily = 'inherit';
  const fieldStyle: CSSProperties = {
    width: '100%',
    boxSizing: 'border-box',
    background: PARENT_PORTAL_GLASS.dialogFill,
    border: `1px solid ${colorAlpha(borderColor, 'AA')}`,
    borderRadius: 6,
    color: cfg.colors.bodyText,
    fontFamily: dialogFontFamily,
    fontSize: 13,
    fontWeight: 760,
    outline: 'none',
    padding: '8px 10px',
  };

  return (
    <g>
      <rect
        x={overlayX}
        y={overlayY}
        width={overlayW}
        height={overlayH}
        fill={PARENT_PORTAL_GLASS.dialogScrim}
        rx={12}
        onClick={(event) => {
          event.stopPropagation();
          onClose();
        }}
      />
      <foreignObject x={x} y={y} width={w} height={h}>
        <div
          onClick={(event) => event.stopPropagation()}
          onPointerDown={(event) => event.stopPropagation()}
          style={{
            width: '100%',
            height: '100%',
            boxSizing: 'border-box',
            background: `linear-gradient(180deg, ${PARENT_PORTAL_GLASS.dialogFillStrongTop}, ${PARENT_PORTAL_GLASS.dialogFillStrongBottom})`,
            border: `1px solid ${borderColor}`,
            borderRadius: 8,
            boxShadow: `0 16px 42px rgba(0, 0, 0, 0.42), 0 0 22px ${colorAlpha(borderColor, '44')}`,
            color: cfg.colors.bodyText,
            fontFamily: dialogFontFamily,
            padding: '14px 16px 16px',
            position: 'relative',
            WebkitBackdropFilter: 'blur(10px)',
            backdropFilter: 'blur(10px)',
          }}
        >
          <button
            type="button"
            aria-label="Close device editor"
            onClick={onClose}
            style={{
              position: 'absolute',
              right: 8,
              top: 8,
              width: 24,
              height: 24,
              border: `1px solid ${colorAlpha(borderColor, 'AA')}`,
              borderRadius: 6,
              background: PARENT_PORTAL_GLASS.panelFillStrong,
              color: cfg.colors.bodyText,
              cursor: 'pointer',
              fontFamily: dialogFontFamily,
              fontSize: 13,
              fontWeight: 950,
              lineHeight: '20px',
              padding: 0,
            }}
          >
            x
          </button>
          <div
            style={{
              color: borderColor,
              fontSize: 13,
              fontWeight: 950,
              letterSpacing: 0,
              paddingRight: 30,
              textTransform: 'uppercase',
            }}
          >
            Edit device
          </div>
          <div style={{ color: cfg.colors.mutedText, fontSize: 11, fontWeight: 780, marginTop: 5 }}>{slot.label}</div>
          <label style={{ display: 'block', marginTop: 14 }}>
            <span style={{ color: accentColor, display: 'block', fontSize: 10, fontWeight: 950, marginBottom: 5 }}>
              Detected name
            </span>
            <input aria-label="Detected name" readOnly value={detectedName} style={{ ...fieldStyle, opacity: 0.74 }} />
          </label>
          <label style={{ display: 'block', marginTop: 10 }}>
            <span style={{ color: borderColor, display: 'block', fontSize: 10, fontWeight: 950, marginBottom: 5 }}>
              Household name
            </span>
            <input
              aria-label="Household name"
              value={householdName}
              onChange={(event) => onHouseholdNameChange(event.currentTarget.value)}
              placeholder={detectedName === 'Not reported' ? 'Name this device' : detectedName}
              style={fieldStyle}
            />
          </label>
          <label style={{ display: 'block', marginTop: 10 }}>
            <span style={{ color: borderColor, display: 'block', fontSize: 10, fontWeight: 950, marginBottom: 5 }}>
              Device type
            </span>
            <select
              aria-label="Device type"
              value={deviceKind}
              onChange={(event) => onDeviceKindChange(event.currentTarget.value as DeviceKind)}
              style={fieldStyle}
            >
              {PortalAgentLanHouseholdDeviceKindValues.map((kind) => (
                <option key={`lan-device-kind:${kind}`} value={kind}>
                  {lanPairingDeviceKindOptionLabel(kind)}
                </option>
              ))}
            </select>
          </label>
          <div style={{ display: 'flex', gap: 10, justifyContent: 'flex-end', marginTop: 14 }}>
            <button
              type="button"
              onClick={onClose}
              style={{
                background: PARENT_PORTAL_GLASS.panelFillStrong,
                border: `1px solid ${colorAlpha(cfg.colors.panelStroke, 'AA')}`,
                borderRadius: 6,
                color: cfg.colors.mutedText,
                cursor: 'pointer',
                fontFamily: dialogFontFamily,
                fontSize: 12,
                fontWeight: 850,
                padding: '7px 16px',
              }}
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={onSave}
              style={{
                background: `linear-gradient(180deg, ${colorAlpha(borderColor, 'E6')}, ${colorAlpha(borderColor, '99')})`,
                border: `1px solid ${colorAlpha(borderColor, 'DD')}`,
                borderRadius: 6,
                color: '#03131f',
                cursor: 'pointer',
                fontFamily: dialogFontFamily,
                fontSize: 12,
                fontWeight: 950,
                padding: '7px 18px',
              }}
            >
              Save
            </button>
          </div>
        </div>
      </foreignObject>
    </g>
  );
}

function lanPairingActionButtonsFor(slot: DeviceSlot | null): readonly LanPairingActionButton[] {
  return [
    lanPairingActionButton(
      'select',
      'Select',
      'gold',
      AgentCommand.LanPairingRouteSelect,
      lanPairingRouteIntentCommandPayload(slot)
    ),
    lanPairingActionButton(
      'trust',
      'Trust',
      'cyan',
      AgentCommand.LanPairingAddDeviceRequest,
      lanPairingHouseholdActionCommandPayload(slot, PortalAgentLanHouseholdActionKind.Trust)
    ),
    lanPairingActionButton(
      'ignore',
      'Ignore',
      'gold',
      AgentCommand.LanPairingAddDeviceRequest,
      lanPairingHouseholdActionCommandPayload(slot, PortalAgentLanHouseholdActionKind.Ignore)
    ),
    lanPairingActionButton(
      'restore',
      'Restore',
      'purple',
      AgentCommand.LanPairingAddDeviceRequest,
      lanPairingHouseholdActionCommandPayload(slot, PortalAgentLanHouseholdActionKind.Restore)
    ),
    lanPairingActionButton(
      'revoke',
      'Revoke',
      'gold',
      AgentCommand.LanPairingRouteRevoke,
      lanPairingRouteIntentCommandPayload(slot)
    ),
  ];
}

function lanPairingPairActionButtonsFor(slot: DeviceSlot | null): readonly LanPairingActionButton[] {
  const payload = lanPairingPairCommandPayload(slot);
  if (!payload) return [];
  return [lanPairingActionButton('pair', 'Start pairing', 'gold', AgentCommand.LanPairingAddDeviceRequest, payload)];
}

function lanPairingActionButton(
  id: LanPairingActionId,
  label: string,
  tone: Tone,
  command: AgentCommandName,
  payload: Record<string, string> | null
): LanPairingActionButton {
  return {
    id,
    label,
    tone,
    enabled: payload !== null,
    command,
    payload,
    status: payload ? 'Ready' : 'Unavailable',
  };
}

function lanPairingAddDeviceCommandPayload(slot: DeviceSlot | null): Record<string, string> | null {
  if (!slot) return null;
  if (lanPairingDeviceIsInfrastructure(slot) || slot.status === 'unsupported') return null;
  const childDeviceId = slot.device?.id || slot.value;
  const routeId = slot.device?.routeId;
  if (!childDeviceId || !routeId) return null;
  const issuedAt = new Date().toISOString();
  const expiresAt = new Date(Date.now() + 5 * 60 * 1000).toISOString();
  const origin = typeof window === 'undefined' ? 'http://127.0.0.1:4678' : window.location.origin;
  return {
    [PortalAgentProtocolField.LanChildDeviceId]: childDeviceId,
    [PortalAgentProtocolField.LanParentDeviceId]: PortalAgentPeerDefaults.PortalDev.peerId,
    [PortalAgentProtocolField.LanRouteId]: routeId,
    [PortalAgentProtocolField.Origin]: origin,
    [PortalAgentProtocolField.StartedAt]: issuedAt,
    [PortalAgentProtocolField.StaleAt]: expiresAt,
  };
}

function lanPairingPairCommandPayload(slot: DeviceSlot | null): Record<string, string> | null {
  if (!slot || slot.status === 'offline' || lanPairingDevicePairedOrConfirmed(slot)) return null;
  return lanPairingAddDeviceCommandPayload(slot);
}

export function lanPairingHouseholdActionCommandPayload(
  slot: DeviceSlot | null,
  actionKind: string,
  override?: { readonly displayName?: string; readonly deviceKind?: DeviceKind; readonly requiresRoute?: boolean }
): Record<string, string> | null {
  if (!slot) return null;
  const supportedAction = Object.values(PortalAgentLanHouseholdActionKind).some((value) => value === actionKind);
  if (!supportedAction) return null;
  if (override?.displayName !== undefined && override.displayName.trim().length === 0) return null;
  if (override?.requiresRoute && !slot.device?.routeId) return null;
  // Household decisions require an owner-issued intent, controller lease, and
  // parent authority. The current DeviceSlot/read-model projection carries
  // discovery and pairing evidence only, so no mutation payload can satisfy
  // the Rust-owned boundary. Keep the visible controls unavailable until the
  // owner-backed authority contract is projected here; never synthesize it.
  return null;
}

export function lanPairingRouteIntentCommandPayload(slot: DeviceSlot | null): Record<string, string> | null {
  if (!slot) return null;
  // Route select/revoke also require an owner-issued intent, controller lease,
  // and parent authority. The current projection has no such fields, so keep
  // these commands unavailable rather than minting caller-owned authority.
  return null;
}

function lanPairingContextRowsFor(selectedDevice: DeviceSlot | null): readonly LanPairingContextRow[] {
  if (!selectedDevice) {
    return [
      { label: 'Selected device', value: 'No valid service-backed device selected', tone: 'gold' },
      { label: 'Source', value: 'Unavailable', tone: 'purple' },
      { label: 'Control', value: 'Unavailable', tone: 'cyan' },
      { label: 'Custody', value: 'Not reported', tone: 'gold' },
    ];
  }
  return [
    { label: 'Selected device', value: lanPairingDeviceName(selectedDevice), tone: 'cyan' },
    { label: 'Source', value: lanPairingDeviceSource(selectedDevice), tone: 'purple' },
    { label: 'Control', value: lanPairingDeviceControlState(selectedDevice), tone: 'gold' },
    { label: 'Custody', value: lanPairingDeviceCustody(selectedDevice), tone: 'cyan' },
  ];
}

function lanPairingDetailRowsFor(
  tab: LanPairingDetailTabId,
  selectedDevice: DeviceSlot | null
): readonly LanPairingDetailRow[] {
  if (!selectedDevice) {
    return [{ label: 'Selection', value: 'No device selected', tone: 'gold' }];
  }

  if (tab === 'pair') {
    return [
      { label: 'Pair action', value: lanPairingPairingActionStateFor(selectedDevice), tone: 'gold' },
      { label: 'Setup state', value: lanPairingSetupStatusFor(selectedDevice), tone: 'gold' },
      { label: 'Next step', value: lanPairingPairingNextStepFor(selectedDevice), tone: 'cyan' },
      { label: 'Route', value: lanPairingDeviceRoute(selectedDevice), tone: 'gold' },
      { label: 'Signed proof', value: lanPairingDeviceSignedProof(selectedDevice), tone: 'purple' },
      { label: 'Confirmation', value: 'Signed child-agent hello required', tone: 'gold' },
      { label: 'Detected name', value: lanPairingDetectedDeviceName(selectedDevice), tone: 'purple' },
      { label: 'Device type', value: lanPairingDeviceType(selectedDevice), tone: 'cyan' },
      { label: 'IP', value: lanPairingMissingDeviceValue(selectedDevice.device?.ip), tone: 'cyan' },
      { label: 'Host', value: lanPairingMissingDeviceValue(selectedDevice.device?.hostname), tone: 'gold' },
    ];
  }

  const unavailableReason = lanPairingDetailTabUnavailableReason(tab, selectedDevice);
  if (unavailableReason) {
    return [
      { label: 'Blocked', value: unavailableReason, tone: 'gold' },
      { label: 'Device', value: lanPairingDeviceName(selectedDevice), tone: 'cyan' },
      { label: 'Current state', value: lanPairingDeviceState(selectedDevice), tone: 'purple' },
      { label: 'Control state', value: lanPairingDeviceControlState(selectedDevice), tone: 'gold' },
      { label: 'Next step', value: lanPairingPairingNextStepFor(selectedDevice), tone: 'cyan' },
      { label: 'Signed proof', value: lanPairingDeviceSignedProof(selectedDevice), tone: 'purple' },
      { label: 'Route', value: lanPairingDeviceRoute(selectedDevice), tone: 'gold' },
      { label: 'Evidence', value: lanPairingDeviceEvidence(selectedDevice), tone: 'cyan' },
    ];
  }

  const sharedRows: readonly LanPairingDetailRow[] = [
    { label: 'Name', value: lanPairingDeviceName(selectedDevice), tone: 'cyan' },
    { label: 'State', value: lanPairingDeviceState(selectedDevice), tone: 'gold' },
    { label: 'Control', value: lanPairingDeviceControlState(selectedDevice), tone: 'cyan' },
    { label: 'Platform', value: lanPairingDevicePlatform(selectedDevice), tone: 'purple' },
  ];

  if (tab === 'update') {
    return [
      ...sharedRows,
      { label: 'Source', value: lanPairingDeviceSource(selectedDevice), tone: 'purple' },
      { label: 'Route', value: lanPairingDeviceRoute(selectedDevice), tone: 'gold' },
      { label: 'Route state', value: lanPairingDeviceRouteState(selectedDevice), tone: 'cyan' },
      { label: 'Parent decision', value: lanPairingDeviceParentDecision(selectedDevice), tone: 'purple' },
      { label: 'Route safety', value: lanPairingDeviceRouteSafety(selectedDevice), tone: 'gold' },
      { label: 'Route result', value: lanPairingDeviceRouteSafetyResult(selectedDevice), tone: 'purple' },
      { label: 'Relay/cache', value: lanPairingDeviceRelayCache(selectedDevice), tone: 'cyan' },
      { label: 'Audit', value: lanPairingDeviceAudit(selectedDevice), tone: 'gold' },
      { label: 'IP', value: lanPairingMissingDeviceValue(selectedDevice.device?.ip), tone: 'cyan' },
      { label: 'MAC', value: lanPairingMissingDeviceValue(selectedDevice.device?.mac), tone: 'purple' },
      { label: 'Host', value: lanPairingMissingDeviceValue(selectedDevice.device?.hostname), tone: 'gold' },
      {
        label: 'Interface',
        value: lanPairingMissingDeviceValue(selectedDevice.device?.networkInterface),
        tone: 'cyan',
      },
      {
        label: 'Device ID',
        value: lanPairingMissingDeviceValue(selectedDevice.device?.id ?? selectedDevice.value),
        tone: 'gold',
      },
    ];
  }
  if (tab === 'capability') {
    return [
      ...sharedRows,
      {
        label: 'Device ID',
        value: lanPairingMissingDeviceValue(selectedDevice.device?.id ?? selectedDevice.value),
        tone: 'gold',
      },
      { label: 'Signed proof', value: lanPairingDeviceSignedProof(selectedDevice), tone: 'gold' },
      { label: 'Proof state', value: lanPairingDeviceSignedProofState(selectedDevice), tone: 'cyan' },
      { label: 'Custody', value: lanPairingDeviceCustody(selectedDevice), tone: 'purple' },
      { label: 'Manual proof', value: lanPairingDeviceManualProof(selectedDevice), tone: 'gold' },
      { label: 'Unproved claim', value: lanPairingDeviceClaimsNotProved(selectedDevice), tone: 'purple' },
      { label: 'Requirement', value: lanPairingDeviceRequirement(selectedDevice), tone: 'cyan' },
      { label: 'Evidence', value: lanPairingDeviceEvidence(selectedDevice), tone: 'gold' },
      { label: 'Type', value: lanPairingDeviceType(selectedDevice), tone: 'cyan' },
      { label: 'Agent', value: lanPairingMissingDeviceValue(selectedDevice.device?.agentStatus), tone: 'gold' },
      { label: 'CPU', value: lanPairingMissingDeviceValue(selectedDevice.device?.cpuModel), tone: 'purple' },
      { label: 'Cores', value: lanPairingMissingDeviceValue(selectedDevice.device?.cpuCores), tone: 'cyan' },
      { label: 'Memory', value: lanPairingMissingDeviceValue(selectedDevice.device?.memoryTotal), tone: 'gold' },
      { label: 'GPU', value: lanPairingMissingDeviceValue(selectedDevice.device?.gpuModel), tone: 'purple' },
      { label: 'GPU driver', value: lanPairingMissingDeviceValue(selectedDevice.device?.gpuDriver), tone: 'cyan' },
      { label: 'GPU memory', value: lanPairingMissingDeviceValue(selectedDevice.device?.gpuMemory), tone: 'gold' },
      { label: 'NVIDIA SMI', value: lanPairingMissingDeviceValue(selectedDevice.device?.nvidiaSmi), tone: 'purple' },
    ];
  }
  return [
    { label: 'Display name', value: lanPairingDeviceName(selectedDevice), tone: 'cyan' },
    { label: 'Detected name', value: lanPairingDetectedDeviceName(selectedDevice), tone: 'purple' },
    { label: 'Device type', value: lanPairingDeviceType(selectedDevice), tone: 'gold' },
    { label: 'IP', value: lanPairingMissingDeviceValue(selectedDevice.device?.ip), tone: 'gold' },
    { label: 'Host', value: lanPairingMissingDeviceValue(selectedDevice.device?.hostname), tone: 'cyan' },
    { label: 'State', value: lanPairingDeviceState(selectedDevice), tone: 'cyan' },
    { label: 'Platform', value: lanPairingDevicePlatform(selectedDevice), tone: 'purple' },
  ];
}

type ActivityManageTabId =
  | 'reports'
  | 'screen'
  | 'tracking'
  | 'remoteScreen'
  | 'apps'
  | 'browser'
  | 'games'
  | 'network';

type ActivityManageTab = {
  readonly id: ActivityManageTabId;
  readonly label: string;
  readonly icon: IconComponent;
  readonly tone: Tone;
};

const ACTIVITY_MANAGE_TABS: readonly [ActivityManageTab, ...ActivityManageTab[]] = [
  { id: 'reports', label: 'Reports', icon: ReportDocumentIcon, tone: 'purple' },
  { id: 'browser', label: 'Browser', icon: BrowserStackIcon, tone: 'cyan' },
  { id: 'apps', label: 'App Use', icon: AppIcon, tone: 'gold' },
  { id: 'games', label: 'Games', icon: GamesIcon, tone: 'purple' },
  { id: 'screen', label: 'Screen', icon: ScreenAnalysisIcon, tone: 'cyan' },
  { id: 'network', label: 'Network', icon: WebGlobeIcon, tone: 'cyan' },
  { id: 'tracking', label: 'Tracking', icon: TrackingLocationIcon, tone: 'gold' },
  { id: 'remoteScreen', label: 'Remote Screen', icon: RemoteAccessMonitorsIcon, tone: 'purple' },
];

type ActivityReportFrequencyOption = {
  value: string;
  label: string;
  command: AgentCommandName;
};

const ACTIVITY_REPORT_DAILY_OPTION: ActivityReportFrequencyOption = {
  value: PortalAgentActivityReportFrequency.Daily,
  label: 'Daily',
  command: AgentCommand.ActivityReportDailyGenerate,
};

const ACTIVITY_REPORT_FREQUENCY_OPTIONS: ActivityReportFrequencyOption[] = [
  ACTIVITY_REPORT_DAILY_OPTION,
  {
    value: PortalAgentActivityReportFrequency.Weekly,
    label: 'Weekly',
    command: AgentCommand.ActivityReportWeeklyGenerate,
  },
  {
    value: PortalAgentActivityReportFrequency.Monthly,
    label: 'Monthly',
    command: AgentCommand.ActivityReportMonthlyGenerate,
  },
];

const ACTIVITY_REPORT_OVERRIDE_OPTIONS = [
  { value: 'family-defaults', label: 'Default' },
  { value: 'override', label: 'Override' },
];

type ActivityManageDetailRow = {
  readonly label: string;
  readonly value: string;
  readonly tone: Tone;
};

function activityManageTargetLabel(scopeValue: string, selectedDevice: DeviceSlot | null): string {
  if (scopeValue !== 'device') return 'Family';
  return selectedDevice?.label ?? 'Select device';
}

function activityReportScopeCommandPayload(
  familyScope: boolean,
  selectedDevice: DeviceSlot | null
): Record<string, string> | null {
  if (familyScope) {
    return {
      [PortalAgentProtocolField.ScopeKind]: PortalAgentActivitySurfaceScopeKind.Family,
    };
  }
  const deviceId = selectedDevice?.device?.id?.trim() ?? '';
  if (!deviceId) return null;
  return {
    [PortalAgentProtocolField.ScopeKind]: PortalAgentActivitySurfaceScopeKind.Device,
    [PortalAgentProtocolField.DeviceId]: deviceId,
  };
}

function activityReportSaveCommandPayload(report: Record<string, unknown> | null): Record<string, string> | null {
  if (!report) return null;
  return {
    [PortalAgentProtocolField.ActivityReportDocument]: JSON.stringify(report),
  };
}

function activityManageTabLabel(tab: ActivityManageTabId): string {
  return ACTIVITY_MANAGE_TABS.find((item) => item.id === tab)?.label ?? 'Activity';
}

function activityTabRequiresDevice(tab: ActivityManageTabId): boolean {
  return tab !== 'reports';
}

type ActivityReportScopeStatus = {
  readonly ariaLabel: string;
  readonly eyebrow: string;
  readonly detail: string;
};

function activityReportScopeStatus(familyScope: boolean, currentDeviceCount: number): ActivityReportScopeStatus | null {
  if (familyScope) {
    return {
      ariaLabel: 'Whole family activity report scope',
      eyebrow: 'WHOLE FAMILY REPORT',
      detail: 'Family reports cover every current household device. Switch to Per Device to inspect one child.',
    };
  }
  if (currentDeviceCount === 0) {
    return {
      ariaLabel: 'No current activity device targets',
      eyebrow: 'NO CURRENT DEVICE TARGETS',
      detail: 'Connect the local service and load a current household device before choosing per-device activity.',
    };
  }
  return null;
}

function activityDetailTabUnavailableReason(
  tab: ActivityManageTab,
  familyScope: boolean,
  selectedDevice: DeviceSlot | null
): string | null {
  if (!activityTabRequiresDevice(tab.id)) return null;
  if (familyScope) return `${tab.label} requires Per Device activity scope`;
  if (!selectedDevice) return `${tab.label} requires a current device selection`;
  return null;
}

function activityStateValue(value: unknown, fallback = 'Not reported'): string {
  if (value === null || value === undefined || value === '') return fallback;
  if (typeof value === 'number') return Number.isFinite(value) ? String(value) : fallback;
  if (typeof value === 'boolean') return value ? 'Yes' : 'No';
  if (Array.isArray(value)) return value.length > 0 ? String(value.length) : fallback;
  if (typeof value === 'object') return fallback;
  return String(value);
}

function activityRecord(value: unknown): Record<string, unknown> | null {
  return value && typeof value === 'object' && !Array.isArray(value) ? (value as Record<string, unknown>) : null;
}

function activityRecordArray(value: unknown): readonly Record<string, unknown>[] {
  return Array.isArray(value)
    ? value.map(activityRecord).filter((record): record is Record<string, unknown> => record !== null)
    : [];
}

function activityStateRows(model: Record<string, unknown> | null | undefined): readonly Record<string, unknown>[] {
  return activityRecordArray(model?.['rows']);
}

function activityLatestRow(model: Record<string, unknown> | null | undefined): Record<string, unknown> | null {
  return activityStateRows(model)[0] ?? null;
}

function activityFormatDurationMs(value: unknown): string {
  const duration = typeof value === 'number' && Number.isFinite(value) ? value : null;
  if (duration === null) return 'Not reported';
  const minutes = Math.round(duration / 60000);
  if (minutes < 1) return '< 1 min';
  if (minutes < 60) return `${minutes} min`;
  const hours = Math.floor(minutes / 60);
  const remainder = minutes % 60;
  return remainder > 0 ? `${hours}h ${remainder}m` : `${hours}h`;
}

function activityFormatBytes(value: unknown): string {
  const bytes = typeof value === 'number' && Number.isFinite(value) ? value : null;
  if (bytes === null) return 'Not reported';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${Math.round(bytes / 1024)} KB`;
  return `${Math.round(bytes / (1024 * 102.4)) / 10} MB`;
}

function activityEndpointLabel(value: unknown): string {
  const endpoint = activityRecord(value);
  if (!endpoint) return activityStateValue(value);
  const ip = activityStateValue(endpoint['ip'], '');
  const port = activityStateValue(endpoint['port'], '');
  if (ip && port) return `${ip}:${port}`;
  return ip || port || 'Not reported';
}

function activityEvidenceCount(value: unknown): string {
  return Array.isArray(value) ? String(value.length) : activityStateValue(value);
}

function activityPercentValue(value: unknown): string {
  const number = typeof value === 'number' && Number.isFinite(value) ? value : null;
  if (number === null) return 'Not reported';
  return `${Math.round(number * 100)}%`;
}

function activityDigestLabel(value: unknown): string {
  const text = activityStateValue(value, '');
  if (!text) return 'Not reported';
  return text.length > 28 ? `${text.slice(0, 28)}...` : text;
}

function activityStringArray(value: unknown): readonly string[] {
  return Array.isArray(value)
    ? value.map((item) => activityStateValue(item, '')).filter((item) => item.length > 0)
    : [];
}

function activityListSummary(value: unknown): string {
  const items = activityStringArray(value);
  if (items.length <= 2) return items.join(', ') || 'Not reported';
  return `${items.slice(0, 2).join(', ')} +${items.length - 2}`;
}

function activityLanRouteSafetyRow(
  signedSpine: Record<string, unknown> | null,
  selectedDevice: DeviceSlot | null
): Record<string, unknown> | null {
  const rows = activityRecordArray(signedSpine?.['routeSafetyRows']);
  const selectedRouteId = activityStateValue(selectedDevice?.device?.routeId, '');
  return (
    rows.find((row) => activitySameDeviceValue(activityStateValue(row['routeId'], ''), selectedRouteId)) ??
    rows[0] ??
    null
  );
}

function activityLanSpineStateRow(
  signedSpine: Record<string, unknown> | null,
  key: string
): Record<string, unknown> | null {
  return activityRecordArray(signedSpine?.[key])[0] ?? null;
}

function activityLanSignedAdapterRow(signedSpine: Record<string, unknown> | null): Record<string, unknown> | null {
  const rows = activityRecordArray(signedSpine?.['adapterRows']);
  return (
    rows.find((row) => activityStateValue(row['adapter'], '') === 'signed-child-agent-heartbeat') ??
    rows.find((row) => activityStateValue(row['adapter'], '') === 'signed-child-agent-hello') ??
    rows[0] ??
    null
  );
}

function activityLanAdapterLabel(row: Record<string, unknown> | null): string {
  if (!row) return 'Not reported';
  return `${activityStateValue(row['adapter'])}; ${activityStateValue(row['proofState'])}`;
}

function activityLanSignedProofLabel(row: Record<string, unknown> | null): string {
  if (!row) return 'Not reported';
  return `${activityStateValue(row['check'])}; ${activityStateValue(row['proofState'])}`;
}

function activityLanRouteSafetyLabel(row: Record<string, unknown> | null): string {
  if (!row) return 'Not reported';
  const reason = activityStateValue(row['rejectionReason'], '');
  const result = activityStateValue(row['responseState'] ?? row['discoveryState']);
  return reason
    ? `${activityStateValue(row['check'])}; ${result}; ${reason}`
    : `${activityStateValue(row['check'])}; ${result}`;
}

function activityLanRelayCacheLabel(row: Record<string, unknown> | null): string {
  if (!row) return 'Not reported';
  return `${activityStateValue(row['check'])}; ${activityStateValue(row['decisionState'] ?? row['proofState'])}`;
}

function activityPerDeviceGateRows(
  tab: ActivityManageTabId,
  scopeValue: string,
  selectedDevice: DeviceSlot | null
): readonly ActivityManageDetailRow[] {
  const tabLabel = activityManageTabLabel(tab);
  if (scopeValue !== 'device') {
    return [
      {
        label: 'Family aggregate',
        value: `Showing household-level ${tabLabel.toLowerCase()} summary when the Rust read model reports it.`,
        tone: 'cyan',
      },
    ];
  }
  if (!selectedDevice || selectedDevice.status === 'empty') {
    return [
      {
        label: 'Select device',
        value: `Select a child device above to inspect ${tabLabel.toLowerCase()} activity.`,
        tone: 'gold',
      },
    ];
  }
  if (selectedDevice.status === 'offline') {
    return [
      {
        label: 'Device offline',
        value: 'Rust can only show cached evidence until the child agent is reachable.',
        tone: 'gold',
      },
    ];
  }
  return [];
}

function activityUnavailableRows(
  readModelName: string,
  exposedCommand: string,
  gateRows: readonly ActivityManageDetailRow[]
): readonly ActivityManageDetailRow[] {
  return [
    ...gateRows,
    { label: readModelName, value: `${exposedCommand} is not reported by the Rust portal command yet.`, tone: 'cyan' },
  ];
}

function activityLanDiagnosticsRows(
  addDeviceReadModel: Record<string, unknown> | null | undefined,
  scopeValue: string,
  selectedDevice: DeviceSlot | null,
  gateRows: readonly ActivityManageDetailRow[]
): readonly ActivityManageDetailRow[] | null {
  if (!addDeviceReadModel) return null;
  const canonicalDevices = activityRecordArray(addDeviceReadModel['canonicalHouseholdDevices']);
  const householdDecisions = activityRecordArray(addDeviceReadModel['householdDeviceDecisions']);
  const scanSummary = activityRecord(addDeviceReadModel['scanSummary']);
  const signedSpine = activityRecord(addDeviceReadModel['signedDiscoveryRelaySpine']);
  const sourceMatrix = activityRecord(addDeviceReadModel['lanDiscoverySourceMatrix']);
  const signedProofRow = activityLanSpineStateRow(signedSpine, 'signedProofRows');
  const routeSafetyRow = activityLanRouteSafetyRow(signedSpine, selectedDevice);
  const relayCacheRow = activityLanSpineStateRow(signedSpine, 'relayCacheRows');
  const signedAdapterRow = activityLanSignedAdapterRow(signedSpine);
  const selectedReadiness = activityRecord(addDeviceReadModel['selectedDeviceReadiness']);
  const selectedCanonicalDevice =
    scopeValue === 'device' && selectedDevice
      ? canonicalDevices.find((device) => activityCanonicalDeviceMatchesSlot(device, selectedDevice))
      : null;
  const relevantDevices = selectedCanonicalDevice ? [selectedCanonicalDevice] : canonicalDevices;
  const relevantDecisions = selectedCanonicalDevice
    ? householdDecisions.filter((decision) =>
        activitySameDeviceValue(
          activityStateValue(decision['canonicalDeviceId'], ''),
          activityStateValue(selectedCanonicalDevice['canonicalDeviceId'], '')
        )
      )
    : householdDecisions;
  const evidenceRecords = relevantDevices.flatMap((device) =>
    activityRecordArray(activityRecord(device['networkIdentity'])?.['evidenceRecords'])
  );
  const latestEvidence = evidenceRecords[0] ?? null;
  const latestDecision = relevantDecisions[0] ?? null;
  const policyTargetSurfaces = relevantDevices.flatMap((device) => activityStringArray(device['policyTargetSurfaces']));
  const sourceLabels = Array.from(
    new Set(
      evidenceRecords.map((record) => activityStateValue(record['source'], '')).filter((source) => source.length > 0)
    )
  );
  const targetLabel = selectedCanonicalDevice
    ? activityStateValue(selectedCanonicalDevice['displayName'])
    : scopeValue === 'device'
      ? activityManageTargetLabel(scopeValue, selectedDevice)
      : 'Family';
  return [
    ...gateRows,
    { label: 'LAN target', value: targetLabel, tone: 'cyan' },
    { label: 'LAN read model', value: activityStateValue(addDeviceReadModel['addDeviceState']), tone: 'gold' },
    { label: 'Cloud relay', value: activityStateValue(addDeviceReadModel['cloudRelayState']), tone: 'purple' },
    {
      label: 'Physical LAN',
      value: activityStateValue(addDeviceReadModel['physicalHouseholdLanState']),
      tone: 'purple',
    },
    ...activityLanSourceMatrixRows(sourceMatrix),
    {
      label: 'Selected route',
      value: `${activityStateValue(selectedReadiness?.['routeId'])}; ${activityStateValue(selectedReadiness?.['trustState'])}`,
      tone: 'cyan',
    },
    { label: 'Signed proof', value: activityLanSignedProofLabel(signedProofRow), tone: 'gold' },
    { label: 'Route safety', value: activityLanRouteSafetyLabel(routeSafetyRow), tone: 'purple' },
    { label: 'Relay/cache', value: activityLanRelayCacheLabel(relayCacheRow), tone: 'cyan' },
    {
      label: 'Manual proof',
      value: activityListSummary(signedSpine?.['manualProofRequired']),
      tone: 'gold',
    },
    {
      label: 'Unproved claims',
      value: activityListSummary(signedSpine?.['claimsNotProved'] ?? addDeviceReadModel['honestNonClaims']),
      tone: 'purple',
    },
    {
      label: 'Route requirements',
      value: activityListSummary(addDeviceReadModel['routeRequirementLabels']),
      tone: 'cyan',
    },
    {
      label: 'Audit checks',
      value: activityListSummary(addDeviceReadModel['auditCheckLabels']),
      tone: 'gold',
    },
    {
      label: 'Canonical devices',
      value: activityStateValue(selectedCanonicalDevice ? 1 : canonicalDevices.length),
      tone: 'cyan',
    },
    { label: 'Evidence records', value: activityStateValue(evidenceRecords.length), tone: 'gold' },
    { label: 'Parent decisions', value: activityStateValue(relevantDecisions.length), tone: 'purple' },
    {
      label: 'Latest decision',
      value: activityStateValue(latestDecision?.['actionKind'] ?? latestDecision?.['displayName']),
      tone: 'gold',
    },
    {
      label: 'Sources',
      value: sourceLabels.length > 0 ? sourceLabels.join(', ') : activityStateValue(scanSummary?.['sourceLabels']),
      tone: 'cyan',
    },
    {
      label: 'Latest evidence',
      value: activityStateValue(latestEvidence?.['evidenceKind'] ?? latestEvidence?.['value']),
      tone: 'purple',
    },
    {
      label: 'Scan summary',
      value: `agent ${activityStateValue(scanSummary?.['agentDeviceCount'])} / passive ${activityStateValue(
        scanSummary?.['passiveDeviceCount']
      )} / infrastructure ${activityStateValue(scanSummary?.['infrastructureDeviceCount'])}`,
      tone: 'gold',
    },
    {
      label: 'Scan first seen',
      value: activityStateValue(latestEvidence?.['firstSeenAt'] ?? addDeviceReadModel['generatedAt']),
      tone: 'cyan',
    },
    {
      label: 'Scan last seen',
      value: activityStateValue(latestEvidence?.['lastSeenAt'] ?? addDeviceReadModel['generatedAt']),
      tone: 'gold',
    },
    {
      label: 'Evidence expiry',
      value: activityStateValue(latestEvidence?.['expiresAt']),
      tone: 'purple',
    },
    {
      label: 'Signed adapter',
      value: activityLanAdapterLabel(signedAdapterRow),
      tone: 'cyan',
    },
    {
      label: 'Policy targets',
      value: activityListSummary(policyTargetSurfaces),
      tone: 'gold',
    },
  ];
}

function activityLanSourceMatrixRows(sourceMatrix: Record<string, unknown> | null): readonly ActivityManageDetailRow[] {
  if (!sourceMatrix) return [];
  const workpackRows = activityRecordArray(sourceMatrix['workpackRows']);
  const sourceRows = activityRecordArray(sourceMatrix['sourceRows']);
  const implemented = workpackRows.filter((row) => activityStateValue(row['status'], '') === 'implemented').length;
  const partial = workpackRows.filter((row) => activityStateValue(row['status'], '') === 'partial').length;
  const manual = workpackRows.filter((row) => activityStateValue(row['status'], '') === 'manual-required').length;
  const missing = workpackRows.filter((row) => activityStateValue(row['status'], '') === 'not-implemented').length;
  const implementedSources = sourceRows
    .filter((row) => activityStateValue(row['status'], '') === 'implemented')
    .map((row) => activityStateValue(row['source'], ''))
    .filter((source) => source.length > 0);
  const weakSources = sourceRows.filter(
    (row) => row['canConfirmChildAgent'] !== true && row['canAssignChildProfile'] !== true
  ).length;
  return [
    {
      label: 'LAN workpacks',
      value: `${implemented}/${workpackRows.length} implemented; ${partial} partial; ${manual} manual; ${missing} missing`,
      tone: 'cyan',
    },
    {
      label: 'Source proof',
      value: activityListSummary(implementedSources),
      tone: 'gold',
    },
    {
      label: 'Weak source fence',
      value: `weak sources cannot confirm or assign: ${weakSources}/${sourceRows.length}`,
      tone: 'purple',
    },
    {
      label: 'Matrix generated',
      value: activityStateValue(sourceMatrix['generatedAt']),
      tone: 'cyan',
    },
  ];
}

function activityCanonicalDeviceMatchesSlot(device: Record<string, unknown>, slot: DeviceSlot): boolean {
  const networkIdentity = activityRecord(device['networkIdentity']);
  const deviceIds = [activityStateValue(device['canonicalDeviceId'], ''), activityStateValue(device['routeId'], '')];
  const ipAddresses = activityRecordArray(networkIdentity?.['ipAddresses']);
  const networkIps = Array.isArray(networkIdentity?.['ipAddresses'])
    ? networkIdentity['ipAddresses'].map((value) => activityStateValue(value, ''))
    : [];
  return (
    deviceIds.some(
      (deviceId) => activitySameDeviceValue(deviceId, slot.value) || activitySameDeviceValue(deviceId, slot.device?.id)
    ) ||
    activitySameDeviceValue(activityStateValue(networkIdentity?.['macAddress'], ''), slot.device?.mac) ||
    networkIps.some((ip) => activitySameDeviceValue(ip, slot.device?.ip)) ||
    ipAddresses.some((ip) => activitySameDeviceValue(activityStateValue(ip, ''), slot.device?.ip))
  );
}

function activitySameDeviceValue(left: string | undefined, right: string | undefined): boolean {
  const normalizedLeft = left?.trim().toLowerCase();
  const normalizedRight = right?.trim().toLowerCase();
  return !!normalizedLeft && normalizedLeft === normalizedRight;
}

function activityReportHistoryLabel(reportHistory: Record<string, unknown> | null): string {
  if (!reportHistory) return 'Unavailable';
  const reports = reportHistory['reports'];
  const count = Array.isArray(reports) ? reports.length : 0;
  const state = activityStateValue(reportHistory['state']);
  const storage = activityStateValue(reportHistory['storageState']);
  return count > 0 ? `${count} saved (${storage})` : `${state}; ${storage}`;
}

function activityRowsFromReadModels(
  tab: ActivityManageTabId,
  scopeValue: string,
  selectedDevice: DeviceSlot | null,
  frequencyLabel: string,
  overrideLabel: string,
  syncStatus: string,
  lastAction: string,
  activityState?: ParentPortalActivityState | null
): readonly ActivityManageDetailRow[] {
  const targetLabel = activityManageTargetLabel(scopeValue, selectedDevice);
  const gateRows = tab === 'reports' ? [] : activityPerDeviceGateRows(tab, scopeValue, selectedDevice);
  const recent = activityState?.recentSummary;
  const reportDocument = parentPortalActivityAdapterRecord(activityState?.activityReport);
  const reportHistory = parentPortalActivityAdapterRecord(activityState?.activityReportHistory);
  const screen = parentPortalActivityAdapterRecord(activityState?.activityScreenReadModel);
  const appUse = parentPortalActivityAdapterRecord(activityState?.activityAppUseReadModel);
  const browser = parentPortalActivityAdapterRecord(activityState?.activityBrowserReadModel);
  const games = parentPortalActivityAdapterRecord(activityState?.activityGamesReadModel);
  const network = parentPortalActivityAdapterRecord(activityState?.activityNetworkReadModel);
  const tracking = parentPortalActivityAdapterRecord(activityState?.activityTrackingReadModel);
  const browserManaged = activityState?.browserManagedStatus;
  const browserEvidence = activityState?.browserEvidenceReadModel;
  const networkFlow = activityState?.networkFlowReadModel;
  const lanAddDeviceReadModel = activityState?.lanAddDeviceReadModel;
  const ingest = activityState?.ingestStatus;
  const screenRow = activityLatestRow(screen);
  const appRow = activityLatestRow(appUse);
  const browserRow = activityLatestRow(browser) ?? activityLatestRow(browserEvidence);
  const gameRow = activityLatestRow(games);
  const networkRow = activityLatestRow(network) ?? activityLatestRow(networkFlow);
  const trackingRow = activityLatestRow(tracking);

  if (tab === 'reports') {
    return [
      { label: 'Report target', value: targetLabel, tone: 'cyan' },
      { label: 'Frequency', value: frequencyLabel, tone: 'gold' },
      { label: 'Mode', value: scopeValue === 'device' ? overrideLabel : 'Family defaults', tone: 'purple' },
      {
        label: 'Report state',
        value: activityStateValue(
          reportDocument?.['savedMetadata'] ? 'saved' : reportDocument?.['generatedAt'],
          'Unavailable'
        ),
        tone: 'cyan',
      },
      {
        label: 'History',
        value: activityReportHistoryLabel(reportHistory),
        tone: 'cyan',
      },
      { label: 'Generate', value: syncStatus || 'Report command not exposed', tone: 'gold' },
      { label: 'Last action', value: lastAction || 'Ready', tone: 'purple' },
    ];
  }

  if (
    tab !== 'network' &&
    gateRows.length > 0 &&
    scopeValue === 'device' &&
    (!selectedDevice || selectedDevice.status === 'empty')
  ) {
    return gateRows;
  }

  if (tab === 'screen') {
    if (!screen) {
      return activityUnavailableRows('Screen read model', 'ActivityScreenReadModelReported', gateRows);
    }
    return [
      ...gateRows,
      {
        label: 'Latest summary',
        value: activityStateValue(screen['summary'], 'No screen summary reported'),
        tone: 'cyan',
      },
      { label: 'Read model state', value: activityStateValue(screen['state']), tone: 'gold' },
      { label: 'Top row', value: activityStateValue(screenRow?.['label']), tone: 'purple' },
      { label: 'Trigger', value: activityStateValue(screenRow?.['captureReason']), tone: 'cyan' },
      { label: 'Capture scope', value: activityStateValue(screenRow?.['captureScope']), tone: 'gold' },
      { label: 'Capability', value: activityStateValue(screenRow?.['capabilityStatus']), tone: 'purple' },
      { label: 'AI provider', value: activityStateValue(screenRow?.['providerKind']), tone: 'cyan' },
      { label: 'Category', value: activityStateValue(screenRow?.['primaryCategory']), tone: 'purple' },
      { label: 'Confidence', value: activityPercentValue(screenRow?.['confidence']), tone: 'gold' },
      { label: 'Policy eligible', value: activityStateValue(screenRow?.['policyEligible']), tone: 'cyan' },
      { label: 'Raw image', value: activityStateValue(screenRow?.['imageDeletionState']), tone: 'gold' },
      { label: 'Custody', value: activityStateValue(screenRow?.['custodyState']), tone: 'purple' },
      { label: 'Queue job', value: activityStateValue(screenRow?.['queueJobId']), tone: 'cyan' },
      { label: 'Image digest', value: activityDigestLabel(screenRow?.['imageDigest']), tone: 'gold' },
      { label: 'Evidence refs', value: activityEvidenceCount(screenRow?.['evidence']), tone: 'gold' },
    ];
  }

  if (tab === 'tracking') {
    if (!tracking || !trackingRow) {
      return activityUnavailableRows('Tracking read model', 'ActivityTrackingReadModelReported', gateRows);
    }
    return [
      ...gateRows,
      {
        label: 'Read model state',
        value: 'Available',
        tone: 'cyan',
      },
      {
        label: 'Rows returned',
        value: activityStateValue(tracking['returned'], String(activityStateRows(tracking).length)),
        tone: 'gold',
      },
      {
        label: 'Latest activity',
        value: activityStateValue(trackingRow['kind']),
        tone: 'purple',
      },
      {
        label: 'Device',
        value: activityStateValue(trackingRow['subjectDisplayName'] ?? trackingRow['deviceId']),
        tone: 'cyan',
      },
      {
        label: 'Observed',
        value: activityStateValue(tracking['latestObservedAt'] ?? trackingRow['observedAt']),
        tone: 'gold',
      },
      { label: 'Platform', value: activityStateValue(trackingRow['platform']), tone: 'purple' },
      { label: 'Visibility', value: activityStateValue(trackingRow['queryVisibility']), tone: 'cyan' },
      {
        label: 'Capability',
        value: activityStateValue(trackingRow['capabilityStatus'] ?? tracking['capabilityStatus']),
        tone: 'gold',
      },
      { label: 'Custody', value: activityStateValue(tracking['custodyLabel']), tone: 'purple' },
      {
        label: 'Evidence refs',
        value: activityEvidenceCount(trackingRow['evidenceReferenceIds']),
        tone: 'gold',
      },
      {
        label: 'Deleted evidence refs',
        value: activityEvidenceCount(tracking['deletedEvidenceReferenceIds']),
        tone: 'red',
      },
    ];
  }

  if (tab === 'remoteScreen') {
    return [
      ...gateRows,
      {
        label: 'Backend not implemented yet',
        value: 'Live remote screen is a separate Rust capability and is not wired yet.',
        tone: 'gold',
      },
      {
        label: 'Not screen analysis',
        value:
          'Screen analysis is periodic local summaries; remote screen is parent live view with permission and audit.',
        tone: 'cyan',
      },
      {
        label: 'Required proof',
        value: 'Needs route/session/capability state, custody labels, platform permission, and stop/revoke audit.',
        tone: 'purple',
      },
    ];
  }

  if (tab === 'apps') {
    if (!appUse && !appRow) {
      return activityUnavailableRows('App use read model', 'ActivityAppUseReadModelReported', gateRows);
    }
    return [
      ...gateRows,
      {
        label: 'Current app',
        value: activityStateValue(appRow?.['appName']),
        tone: 'cyan',
      },
      {
        label: 'Read model state',
        value: activityStateValue(appRow?.['state'] ?? appUse?.['state']),
        tone: 'gold',
      },
      {
        label: 'Total time',
        value: activityFormatDurationMs(appRow?.['totalMs']),
        tone: 'purple',
      },
      {
        label: 'Launches',
        value: activityStateValue(appRow?.['launchCount']),
        tone: 'cyan',
      },
      {
        label: 'Summary',
        value: activityStateValue(appUse?.['summary']),
        tone: 'gold',
      },
      {
        label: 'Evidence refs',
        value: activityEvidenceCount(appRow?.['evidence']),
        tone: 'purple',
      },
      {
        label: 'Generated',
        value: activityStateValue(appUse?.['generatedAt']),
        tone: 'cyan',
      },
    ];
  }

  if (tab === 'browser') {
    if (!browserRow && !browser && !browserManaged && !browserEvidence) {
      return activityUnavailableRows('Browser activity read model', 'ActivityBrowserReadModelReported', gateRows);
    }
    return [
      ...gateRows,
      {
        label: 'Domain',
        value: activityStateValue(browserRow?.['domainLabel'] ?? browserRow?.['domain'] ?? browserRow?.['origin']),
        tone: 'cyan',
      },
      { label: 'Visits', value: activityStateValue(browserRow?.['visitCount']), tone: 'gold' },
      { label: 'Total time', value: activityFormatDurationMs(browserRow?.['totalMs']), tone: 'purple' },
      {
        label: 'Browser',
        value: activityStateValue(browserRow?.['browserFamily'] ?? browserManaged?.['browserFamily']),
        tone: 'cyan',
      },
      {
        label: 'Read model state',
        value: activityStateValue(browserRow?.['state'] ?? browser?.['state']),
        tone: 'gold',
      },
      {
        label: 'Capability',
        value: activityStateValue(
          browserRow?.['capabilityStatus'] ?? browserEvidence?.['capabilityStatus'] ?? browser?.['state']
        ),
        tone: 'purple',
      },
      { label: 'Managed state', value: activityStateValue(browserManaged?.['managedState']), tone: 'cyan' },
      {
        label: 'Evidence',
        value: activityStateValue(browserRow?.['evidenceDigest'] ?? browserEvidence?.['latestEventId']),
        tone: 'gold',
      },
    ];
  }

  if (tab === 'games') {
    if (!games && !gameRow) {
      return activityUnavailableRows('Games read model', 'ActivityGamesReadModelReported', gateRows);
    }
    return [
      ...gateRows,
      {
        label: 'Current game',
        value: activityStateValue(gameRow?.['displayName']),
        tone: 'cyan',
      },
      {
        label: 'Read model state',
        value: activityStateValue(gameRow?.['state'] ?? games?.['state']),
        tone: 'gold',
      },
      {
        label: 'Total time',
        value: activityFormatDurationMs(gameRow?.['totalMs']),
        tone: 'purple',
      },
      {
        label: 'Sessions',
        value: activityStateValue(gameRow?.['sessionCount']),
        tone: 'cyan',
      },
      {
        label: 'Summary',
        value: activityStateValue(games?.['summary']),
        tone: 'gold',
      },
      {
        label: 'Evidence refs',
        value: activityEvidenceCount(gameRow?.['evidence']),
        tone: 'cyan',
      },
    ];
  }

  if (tab === 'network') {
    if (!networkRow && !network && !networkFlow) {
      return (
        activityLanDiagnosticsRows(lanAddDeviceReadModel, scopeValue, selectedDevice, gateRows) ??
        activityUnavailableRows('Network activity read model', 'ActivityNetworkReadModelReported', gateRows)
      );
    }
    const endpoint =
      networkRow?.['destinationLabel'] ??
      networkRow?.['destinationDomain'] ??
      activityEndpointLabel(networkRow?.['destinationEndpoint']);
    const counters = activityRecord(networkRow?.['counters']);
    const networkRows: ActivityManageDetailRow[] = [
      ...gateRows,
      { label: 'Destination', value: activityStateValue(endpoint), tone: 'purple' },
      { label: 'Process', value: activityStateValue(networkRow?.['processName']), tone: 'cyan' },
      {
        label: 'Read model state',
        value: activityStateValue(networkRow?.['state'] ?? network?.['state']),
        tone: 'gold',
      },
      {
        label: 'Connections',
        value: activityStateValue(networkRow?.['connectionCount'] ?? counters?.['connectionCount']),
        tone: 'gold',
      },
      { label: 'Total bytes', value: activityFormatBytes(networkRow?.['totalBytes']), tone: 'cyan' },
      { label: 'Received', value: activityFormatBytes(counters?.['bytesReceived']), tone: 'cyan' },
      { label: 'Sent', value: activityFormatBytes(counters?.['bytesSent']), tone: 'gold' },
      { label: 'Summary', value: activityStateValue(network?.['summary'] ?? networkFlow?.['custody']), tone: 'purple' },
    ];
    const lanRows = activityLanDiagnosticsRows(lanAddDeviceReadModel, scopeValue, selectedDevice, []);
    return lanRows ? [...networkRows, ...lanRows] : networkRows;
  }

  return [
    { label: 'Target', value: targetLabel, tone: 'cyan' },
    { label: 'Recent subject', value: activityStateValue(recent?.['mostRecentSubjectName']), tone: 'gold' },
    { label: 'Ingested', value: activityStateValue(ingest?.['eventsIngested']), tone: 'purple' },
    { label: 'Stored', value: activityStateValue(ingest?.['eventsStored']), tone: 'cyan' },
  ];
}

function ParentPortalAppGameDashboardPanel({
  x,
  y,
  w,
  h,
  dashboard,
  themeColor,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  dashboard: ParentPortalAppGameDashboardIntent;
  themeColor?: string;
  cfg: ParentPortalSvgControls;
}) {
  const color = themeColor ?? appGameDashboardToneColor(appGameDashboardStateTone(dashboard.state), cfg);
  const compact = w < 860;
  const headerH = compact ? 96 : 72;
  const titleY = y + (compact ? 20 : 22);
  const titleFontSize = compact ? 15 : 18;
  const stateY = y + (compact ? 43 : 22);
  const stateMaxWidth = compact ? w : Math.min(260, w * 0.28);
  const dividerY = y + (compact ? 53 : 37);
  const summaryY = y + (compact ? 71 : 57);
  const bodyY = y + headerH;
  const metricColumns = w > 1220 ? 5 : w > 900 ? 4 : 2;
  const metricRows = Math.ceil(Math.min(dashboard.metrics.length, compact ? 6 : 10) / metricColumns);
  const metricGap = 8;
  const metricH = 46;
  const metricsH = Math.max(metricH, metricRows * metricH + Math.max(0, metricRows - 1) * metricGap);
  const metricW = (w - metricGap * (metricColumns - 1)) / metricColumns;
  const metrics = dashboard.metrics.slice(0, metricColumns * metricRows);
  const lowerY = bodyY + metricsH + 14;
  const lowerH = Math.max(1, y + h - lowerY);
  const sideW = compact ? 0 : Math.max(280, Math.min(390, Math.round(w * 0.31)));
  const rowsW = compact ? w : Math.max(1, w - sideW - 14);
  const rowsColumns = rowsW > 920 ? 2 : 1;
  const rowGap = 10;
  const rowCardH = 84;
  const rowCardW = (rowsW - rowGap * (rowsColumns - 1)) / rowsColumns;
  const visibleRowCount = Math.max(1, Math.floor(Math.max(1, lowerH - 32) / (rowCardH + rowGap)) * rowsColumns);
  const visibleRows = dashboard.rows.slice(0, visibleRowCount);
  const sideX = x + rowsW + 14;
  const sidePanelH = compact ? 0 : (lowerH - 20) / 3;
  const summaryLines = wrapCardText(dashboard.summary, w - 24, 12, 2);

  return (
    <g>
      <text x={x} y={titleY} fontSize={titleFontSize} fontWeight={950} fill={cfg.colors.bodyText}>
        APP/GAME READ MODEL DASHBOARD
      </text>
      <text x={x + w} y={stateY} textAnchor="end" fontSize={10.5} fontWeight={950} fill={color}>
        {truncateTextForWidth(`STATE ${dashboard.state.toUpperCase()}`, stateMaxWidth, 10.5, 0.58)}
      </text>
      <path d={`M ${x} ${dividerY} H ${x + w}`} stroke={color} strokeWidth={1.1} opacity={0.5} />
      {summaryLines.map((line, index) => (
        <text
          key={`app-game-dashboard-summary:${index}`}
          x={x}
          y={summaryY + index * 16}
          fontSize={12}
          fontWeight={760}
          fill={cfg.colors.mutedText}
        >
          {line}
        </text>
      ))}

      {dashboard.metricsState === 'unavailable' && dashboard.metricsUnavailableMessage ? (
        <g aria-label="App and game measured totals unavailable">
          <rect
            x={x}
            y={bodyY}
            width={w}
            height={metricH}
            rx={8}
            fill={cfg.colors.panelFill}
            stroke={color}
            strokeWidth={1}
            opacity={0.88}
          />
          <text x={x + 12} y={bodyY + 17} fontSize={10.5} fontWeight={950} fill={color}>
            MEASURED TOTALS UNAVAILABLE
          </text>
          <text x={x + 12} y={bodyY + 35} fontSize={11.2} fontWeight={740} fill={cfg.colors.mutedText}>
            {truncateTextForWidth(dashboard.metricsUnavailableMessage, w - 24, 11.2, 0.58)}
          </text>
        </g>
      ) : null}

      {metrics.map((metric, index) => {
        const column = index % metricColumns;
        const row = Math.floor(index / metricColumns);
        return (
          <ParentPortalAppGameDashboardMetricCard
            key={`app-game-dashboard-metric:${metric.label}`}
            x={x + column * (metricW + metricGap)}
            y={bodyY + row * (metricH + metricGap)}
            w={metricW}
            h={metricH}
            metric={metric}
            cfg={cfg}
          />
        );
      })}

      <text x={x} y={lowerY + 14} fontSize={10.5} fontWeight={950} fill={color}>
        SERVICE ROWS
      </text>
      {dashboard.rows.length === 0 ? (
        <ParentPortalAppGameDashboardUnavailableCards
          x={x}
          y={lowerY + 28}
          w={rowsW}
          h={Math.max(1, lowerH - 36)}
          emptyMessage={dashboard.emptyMessage}
          cfg={cfg}
        />
      ) : null}
      {visibleRows.map((row, index) => {
        const column = index % rowsColumns;
        const rowIndex = Math.floor(index / rowsColumns);
        return (
          <ParentPortalAppGameDashboardRowCard
            key={`app-game-dashboard-row:${row.sourceKind}:${row.rowId}`}
            x={x + column * (rowCardW + rowGap)}
            y={lowerY + 24 + rowIndex * (rowCardH + rowGap)}
            w={rowCardW}
            h={rowCardH}
            row={row}
            cfg={cfg}
          />
        );
      })}

      {compact ? null : (
        <>
          <ParentPortalAppGameDashboardMetricList
            x={sideX}
            y={lowerY}
            w={sideW}
            h={sidePanelH}
            title="CAPABILITY MATRIX"
            rows={dashboard.capabilityRows}
            cfg={cfg}
          />
          <ParentPortalAppGameDashboardMetricList
            x={sideX}
            y={lowerY + sidePanelH + 10}
            w={sideW}
            h={sidePanelH}
            title="SOURCE FRESHNESS"
            rows={appGameSourcePanelMetrics(dashboard.sourcePanelSections)}
            cfg={cfg}
          />
          <ParentPortalAppGameDashboardMetricList
            x={sideX}
            y={lowerY + (sidePanelH + 10) * 2}
            w={sideW}
            h={sidePanelH}
            title="EVIDENCE DRAWER"
            rows={dashboard.evidenceRows}
            cfg={cfg}
          />
        </>
      )}
    </g>
  );
}

const APP_GAME_DASHBOARD_UNAVAILABLE_CARDS = [
  {
    label: 'Activity rows',
    value: 'Not reported',
    body: 'No app, game, running, foreground, or launcher totals are shown without service rows.',
    tone: 'cyan',
  },
  {
    label: 'Capability status',
    value: 'Not reported',
    body: 'Capability and source freshness remain unknown until the local service reports typed rows.',
    tone: 'gold',
  },
  {
    label: 'Evidence status',
    value: 'Not reported',
    body: 'No evidence reference, policy readiness, or delivery state is inferred from an empty snapshot.',
    tone: 'purple',
  },
] as const;

function ParentPortalAppGameDashboardUnavailableCards({
  x,
  y,
  w,
  h,
  emptyMessage,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  emptyMessage: string;
  cfg: ParentPortalSvgControls;
}) {
  const columns = w > 760 ? 3 : w > 480 ? 2 : 1;
  const gap = 10;
  const rows = Math.ceil(APP_GAME_DASHBOARD_UNAVAILABLE_CARDS.length / columns);
  const cardW = (w - gap * (columns - 1)) / columns;
  const cardH = Math.max(96, Math.min(220, (h - gap * (rows - 1)) / rows));
  return (
    <g aria-label="App and game status unavailable">
      {APP_GAME_DASHBOARD_UNAVAILABLE_CARDS.map((card, index) => {
        const column = index % columns;
        const row = Math.floor(index / columns);
        const cardX = x + column * (cardW + gap);
        const cardY = y + row * (cardH + gap);
        const color = appGameDashboardToneColor(card.tone, cfg);
        const body = index === 0 ? `${emptyMessage} ${card.body}` : card.body;
        const bodyLines = wrapCardText(body, cardW - 28, 10.8, Math.max(2, Math.floor((cardH - 76) / 15)));
        return (
          <g key={`app-game-unavailable-card:${card.label}`}>
            <path
              d={cutRectPath(cardX, cardY, cardW, cardH, 10)}
              fill={colorAlpha(color, '12')}
              stroke={color}
              strokeWidth={0.9}
              opacity={0.96}
            />
            <text x={cardX + 14} y={cardY + 22} fontSize={9.8} fontWeight={950} fill={color}>
              {card.label.toUpperCase()}
            </text>
            <text x={cardX + 14} y={cardY + 50} fontSize={16} fontWeight={930} fill={cfg.colors.bodyText}>
              {card.value}
            </text>
            {bodyLines.map((line, lineIndex) => (
              <text
                key={`app-game-unavailable-card-body:${card.label}:${lineIndex}`}
                x={cardX + 14}
                y={cardY + 76 + lineIndex * 15}
                fontSize={10.8}
                fontWeight={720}
                fill={cfg.colors.mutedText}
              >
                {line}
              </text>
            ))}
          </g>
        );
      })}
    </g>
  );
}

function appGameSourcePanelMetrics(
  sections: readonly ParentPortalAppGameSourcePanelSection[]
): readonly ParentPortalAppGameDashboardMetric[] {
  return sections.flatMap((section) => [
    {
      label: section.title,
      value: section.subtitle,
      tone: section.tone,
    },
    {
      label: `${section.title} evidence`,
      value: `${section.evidenceCount} refs; ${section.manualRequiredCount} manual-required`,
      tone: section.evidenceCount > 0 ? 'cyan' : 'gold',
    },
  ]);
}

function ParentPortalAppGameDashboardMetricCard({
  x,
  y,
  w,
  h,
  metric,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  metric: ParentPortalAppGameDashboardMetric;
  cfg: ParentPortalSvgControls;
}) {
  const color = appGameDashboardToneColor(metric.tone, cfg);
  return (
    <g>
      <path
        d={cutRectPath(x, y, w, h, 7)}
        fill={colorAlpha(color, '14')}
        stroke={color}
        strokeWidth={0.82}
        opacity={0.94}
      />
      <text x={x + 12} y={y + 17} fontSize={9.2} fontWeight={950} fill={color}>
        {truncateTextForWidth(metric.label.toUpperCase(), w - 24, 9.2, 0.58)}
      </text>
      <text x={x + 12} y={y + 35} fontSize={12.2} fontWeight={850} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(metric.value, w - 24, 12.2, 0.58)}
      </text>
    </g>
  );
}

function ParentPortalAppGameDashboardRowCard({
  x,
  y,
  w,
  h,
  row,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  row: ParentPortalAppGameDashboardRow;
  cfg: ParentPortalSvgControls;
}) {
  const color = appGameDashboardToneColor(row.tone, cfg);
  const titleSize = fitSingleLineTextSize(row.label, w - 30, 10.6, 14.5, 0.58);
  const statusLine = `${row.sourceLabel} / ${row.state} / ${row.capabilityStatus}`;
  const countsLine = `Inventory ${row.inventoryCount} / Running ${row.runningCount} / Foreground ${row.foregroundCount} / Launcher ${row.launcherCount}`;
  const evidenceLine = `${row.totalDurationLabel} / ${row.eventCountLabel} / Evidence ${row.evidenceCount}`;
  const warningLine = row.manualRequired
    ? 'Manual-required capability'
    : row.unknownApproval
      ? 'Unknown review candidate'
      : row.launcherOnly
        ? 'Launcher-only, not active game'
        : 'Service-backed row';

  return (
    <g>
      <path
        d={cutRectPath(x, y, w, h, 9)}
        fill={colorAlpha(color, '12')}
        stroke={color}
        strokeWidth={0.9}
        opacity={0.96}
      />
      <circle cx={x + 15} cy={y + 16} r={3.5} fill={color} opacity={0.96} />
      <text x={x + 26} y={y + 18} fontSize={9.4} fontWeight={950} fill={color}>
        {truncateTextForWidth(warningLine.toUpperCase(), w - 42, 9.4, 0.58)}
      </text>
      <text x={x + 14} y={y + 40} fontSize={titleSize} fontWeight={930} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(row.label, w - 28, titleSize, 0.58)}
      </text>
      <text x={x + 14} y={y + 58} fontSize={10.4} fontWeight={760} fill={cfg.colors.mutedText}>
        {truncateTextForWidth(statusLine, w - 28, 10.4, 0.58)}
      </text>
      <text x={x + 14} y={y + 73} fontSize={9.8} fontWeight={720} fill={cfg.colors.mutedText}>
        {truncateTextForWidth(`${countsLine} / ${evidenceLine}`, w - 28, 9.8, 0.58)}
      </text>
    </g>
  );
}

function ParentPortalAppGameDashboardMetricList({
  x,
  y,
  w,
  h,
  title,
  rows,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  title: string;
  rows: readonly ParentPortalAppGameDashboardMetric[];
  cfg: ParentPortalSvgControls;
}) {
  const headerColor = cfg.colors.cyan;
  const rowH = 36;
  const visibleRows = rows.slice(0, Math.max(1, Math.floor((h - 38) / rowH)));
  return (
    <g>
      <path
        d={topRoundedRectPath(x, y, w, h, 10)}
        fill="rgba(2, 12, 22, 0.58)"
        stroke={headerColor}
        strokeWidth={0.85}
        opacity={0.94}
      />
      <text x={x + 14} y={y + 22} fontSize={10.4} fontWeight={950} fill={headerColor}>
        {title}
      </text>
      <path d={`M ${x + 12} ${y + 32} H ${x + w - 12}`} stroke={headerColor} strokeWidth={0.7} opacity={0.32} />
      {visibleRows.map((row, index) => {
        const rowColor = appGameDashboardToneColor(row.tone, cfg);
        const rowY = y + 48 + index * rowH;
        return (
          <g key={`app-game-dashboard-list:${title}:${row.label}:${index}`}>
            <circle cx={x + 16} cy={rowY - 3} r={3} fill={rowColor} opacity={0.95} />
            <text x={x + 26} y={rowY} fontSize={9.2} fontWeight={950} fill={rowColor}>
              {truncateTextForWidth(row.label.toUpperCase(), w - 40, 9.2, 0.58)}
            </text>
            <text x={x + 14} y={rowY + 17} fontSize={10.4} fontWeight={760} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(row.value, w - 28, 10.4, 0.58)}
            </text>
          </g>
        );
      })}
    </g>
  );
}

function appGameDashboardStateTone(state: string): ParentPortalAppGameDashboardTone {
  if (/manual|required|permission|unavailable|stale/u.test(state.toLowerCase())) return 'gold';
  if (/review|risk|unknown/u.test(state.toLowerCase())) return 'red';
  return 'cyan';
}

function appGameDashboardToneColor(tone: ParentPortalAppGameDashboardTone, cfg: ParentPortalSvgControls): string {
  return toneColor(tone as Tone, cfg);
}

function manageTargetLabel(lane: ManageLaneId, selection: ManageTargetSelection): string {
  return selection.scope === 'global' ? manageGlobalTargetLabel(lane) : selection.device;
}

function manageSelectionLabel(
  activeNavLabel: string,
  selectedControlName: string,
  lane: ManageLaneId,
  selection: ManageTargetSelection
): string {
  const targetLabel = manageTargetLabel(lane, selection);
  return isBrowserManageKey(activeNavLabel, selectedControlName)
    ? `${targetLabel} / ${selection.browser}`
    : targetLabel;
}

function manageBrowserTargetsForKey(
  activeNavLabel: string,
  selectedControlName: string
): readonly ManageTargetChoice[] {
  if (!isBrowserManageKey(activeNavLabel, selectedControlName)) return [];
  return [
    { label: 'Chrome', detail: 'Chrome browser policy target.', tone: 'cyan' },
    { label: 'Edge', detail: 'Edge browser policy target.', tone: 'gold' },
    { label: 'Firefox', detail: 'Firefox browser policy target.', tone: 'purple' },
    { label: 'All browsers', detail: 'Every detected browser family.', tone: 'red' },
  ];
}

function scheduleOptionsForManageKey(
  activeNavLabel: string,
  selectedControlName: string
): readonly ManageControlAction[] {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (
    key.includes('browser') ||
    key.includes('app') ||
    key.includes('game') ||
    key.includes('network') ||
    key.includes('alert') ||
    key.includes('rule') ||
    key.includes('schedule')
  ) {
    return [
      { label: 'Always', detail: 'Applies all day.', tone: 'cyan' },
      { label: 'School', detail: 'School-hour rule.', tone: 'gold' },
      { label: 'Homework', detail: 'Homework window.', tone: 'purple' },
      { label: 'Bedtime', detail: 'Sleep cutoff.', tone: 'red' },
      { label: 'Weekend', detail: 'Weekend limits.', tone: 'cyan' },
      { label: 'Custom', detail: 'Parent-defined window.', tone: 'gold' },
    ];
  }
  return [];
}

function manageControlSpecFor(activeNavLabel: string, selectedControlName: string): ManageControlSpec | null {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  const devices: readonly string[] = [];
  const baseStatus = [
    { label: 'Scope', detail: 'Per child device', tone: 'cyan' as Tone },
    { label: 'Custody', detail: 'Local first', tone: 'gold' as Tone },
    { label: 'Audit', detail: 'Every change recorded', tone: 'purple' as Tone },
  ];

  if (key.includes('browser')) {
    return {
      title: 'Browser Setup',
      devices,
      modes: [
        { label: 'Advisory', detail: 'Show risk without blocking.', tone: 'cyan' },
        { label: 'Ask first', detail: 'Ask parent before risky browsing.', tone: 'gold' },
        { label: 'Block', detail: 'Block unsupported browser paths.', tone: 'red' },
      ],
      options: [
        {
          label: 'Require managed browser',
          detail: 'Prefer browser paths with visible evidence.',
          enabled: true,
          tone: 'cyan',
        },
        {
          label: 'Flag unsupported browsers',
          detail: 'Show installed browsers that cannot be controlled yet.',
          enabled: true,
          tone: 'gold',
        },
        {
          label: 'Explain before block',
          detail: 'Tell the child why a page or browser path is blocked.',
          enabled: true,
          tone: 'purple',
        },
        {
          label: 'Allow parent override',
          detail: 'Parent can approve a timed exception.',
          enabled: false,
          tone: 'cyan',
        },
      ],
      actions: [
        { label: 'Scan browsers', detail: 'Refresh supported and unsupported browser state.', tone: 'cyan' },
        { label: 'Apply policy', detail: 'Save this browser rule for the selected device.', tone: 'gold' },
        { label: 'Open web activity', detail: 'Review URL evidence and browser family.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('rule') || key.includes('policy')) {
    return {
      title: 'Rules',
      devices,
      modes: [
        { label: 'Allow', detail: 'Let it pass and record the decision.', tone: 'cyan' },
        { label: 'Ask', detail: 'Pause and ask the parent.', tone: 'gold' },
        { label: 'Block', detail: 'Stop the activity after policy review.', tone: 'red' },
      ],
      options: [
        { label: 'School hours', detail: 'Use school-day defaults for apps and sites.', enabled: true, tone: 'cyan' },
        { label: 'Bedtime quiet', detail: 'Reduce distractions during sleep windows.', enabled: true, tone: 'purple' },
        {
          label: 'New apps ask first',
          detail: 'Require approval for newly detected apps.',
          enabled: true,
          tone: 'gold',
        },
        {
          label: 'Mature content review',
          detail: 'Review risky categories before allow.',
          enabled: false,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Preview decision', detail: 'See what the rule would do before applying.', tone: 'cyan' },
        { label: 'Save rule', detail: 'Keep the rule in family policy.', tone: 'gold' },
        { label: 'View audit', detail: 'Open recent rule changes and parent approvals.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('schedule')) {
    return {
      title: 'Schedules',
      devices,
      modes: [
        { label: 'School day', detail: 'Use weekday learning windows.', tone: 'cyan' },
        { label: 'Weekend', detail: 'Use weekend family limits.', tone: 'gold' },
        { label: 'Temporary', detail: 'Grant or reduce time today only.', tone: 'purple' },
      ],
      options: [
        { label: 'Homework window', detail: 'Focus mode during homework time.', enabled: true, tone: 'cyan' },
        { label: 'Sleep lock', detail: 'Use bedtime cutoff and morning resume.', enabled: true, tone: 'purple' },
        { label: 'Game budget', detail: 'Cap game time separately from school tools.', enabled: false, tone: 'gold' },
        { label: 'Parent extension', detail: 'Allow one-time time grants.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Edit week', detail: 'Open weekly schedule controls.', tone: 'cyan' },
        { label: 'Grant time', detail: 'Create a temporary time exception.', tone: 'gold' },
        { label: 'Pause today', detail: 'Pause this schedule for the selected child.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('approval')) {
    return {
      title: 'Approvals',
      devices,
      modes: [
        { label: 'Ask parent', detail: 'Require an explicit parent answer.', tone: 'gold' },
        { label: 'Auto explain', detail: 'Give an explanation without changing rule.', tone: 'cyan' },
        { label: 'Auto deny', detail: 'Deny known restricted requests.', tone: 'red' },
      ],
      options: [
        { label: 'Require reason', detail: 'Child must explain why they need access.', enabled: true, tone: 'cyan' },
        { label: 'Timed approval', detail: 'Approvals expire automatically.', enabled: true, tone: 'gold' },
        { label: 'Notify parent', detail: 'Send parent a minimal alert.', enabled: true, tone: 'purple' },
        { label: 'Remember answer', detail: 'Reuse this approval rule next time.', enabled: false, tone: 'cyan' },
      ],
      actions: [
        { label: 'Open queue', detail: 'Review pending child requests.', tone: 'gold' },
        { label: 'Approve once', detail: 'Grant a one-time exception.', tone: 'cyan' },
        { label: 'Set default', detail: 'Choose the default answer for this class.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('enforce')) {
    return {
      title: 'Enforcement',
      devices,
      modes: [
        { label: 'Observe', detail: 'Record would-enforce outcomes.', tone: 'cyan' },
        { label: 'Dry run', detail: 'Validate without executing adapters.', tone: 'gold' },
        { label: 'Enforce', detail: 'Apply supported adapter actions.', tone: 'red' },
      ],
      options: [
        { label: 'Require evidence ref', detail: 'Never act from AI text alone.', enabled: true, tone: 'cyan' },
        { label: 'Timer rollback', detail: 'Expire temporary controls automatically.', enabled: true, tone: 'gold' },
        { label: 'Capability gate', detail: 'Unavailable adapters stay honest.', enabled: true, tone: 'purple' },
        { label: 'Parent override', detail: 'Parent can cancel or supersede action.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Test dry run', detail: 'Run policy preview without adapter action.', tone: 'cyan' },
        { label: 'Apply timed block', detail: 'Create a typed temporary block.', tone: 'red' },
        { label: 'Rollback', detail: 'Cancel or expire the active control.', tone: 'gold' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('report')) {
    return {
      title: key.includes('build') ? 'Report Compiler' : 'Reports',
      devices,
      modes: [
        { label: 'Daily', detail: 'Short day summary.', tone: 'cyan' },
        { label: 'Weekly', detail: 'Patterns and changes.', tone: 'purple' },
        { label: 'Monthly', detail: 'Longer family review.', tone: 'gold' },
      ],
      options: [
        { label: 'Activity summary', detail: 'Apps, sites, focus, and screen analysis.', enabled: true, tone: 'cyan' },
        {
          label: 'Policy decisions',
          detail: 'Include blocks, asks, approvals, and overrides.',
          enabled: true,
          tone: 'gold',
        },
        { label: 'AI citations', detail: 'Use cited local memory in explanations.', enabled: true, tone: 'purple' },
        { label: 'Export copy', detail: 'Write a parent-owned report export.', enabled: false, tone: 'cyan' },
      ],
      actions: [
        { label: 'Compile now', detail: 'Build a stateless report from selected evidence.', tone: 'cyan' },
        { label: 'Schedule report', detail: 'Set report time and cadence.', tone: 'gold' },
        { label: 'Open reports', detail: 'Review generated report history.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('capability')) {
    return {
      title: 'Capability Status',
      devices,
      modes: [
        { label: 'Detect', detail: 'Refresh available child-device capabilities.', tone: 'cyan' },
        { label: 'Degrade', detail: 'Use safer reduced behavior when a capability is partial.', tone: 'gold' },
        { label: 'Disable', detail: 'Keep unsupported controls unavailable.', tone: 'red' },
      ],
      options: [
        {
          label: 'Browser control',
          detail: 'Supported, degraded, or unavailable browser management.',
          enabled: true,
          tone: 'cyan',
        },
        { label: 'App control', detail: 'Supported app and process policy actions.', enabled: true, tone: 'gold' },
        { label: 'Screen summary', detail: 'Local screen analysis readiness.', enabled: false, tone: 'purple' },
        { label: 'Network metadata', detail: 'Domain and flow visibility state.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Refresh status', detail: 'Query the selected child device.', tone: 'cyan' },
        { label: 'Show gaps', detail: 'List controls that cannot run on this device.', tone: 'gold' },
        { label: 'Open install', detail: 'Go to install and update controls.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('lan-pairing') || key.includes('lan pairing')) {
    return {
      title: 'LAN Pairing',
      devices,
      modes: [
        { label: 'Discover', detail: 'Find child agents on the same LAN.', tone: 'cyan' },
        { label: 'Pair', detail: 'Require challenge and trusted proof.', tone: 'gold' },
        { label: 'Revoke', detail: 'Remove device trust before new commands.', tone: 'red' },
      ],
      options: [
        {
          label: 'Require pairing proof',
          detail: 'Reject anonymous or stale LAN attempts.',
          enabled: true,
          tone: 'cyan',
        },
        { label: 'Origin check', detail: 'Only accept allowed parent origins.', enabled: true, tone: 'gold' },
        {
          label: 'Replay protection',
          detail: 'Reject stale or reused pairing attempts.',
          enabled: true,
          tone: 'purple',
        },
        { label: 'Fail closed', detail: 'Unpaired devices receive no control intents.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Start pairing', detail: 'Begin a trusted local pairing challenge.', tone: 'gold' },
        { label: 'Select child', detail: 'Make this device the active control target.', tone: 'cyan' },
        { label: 'Revoke trust', detail: 'Stop accepting control intents from this device.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('remote-screen')) {
    return {
      title: 'Remote Screen Policy',
      devices,
      modes: [
        {
          label: 'Unavailable',
          detail: 'No service-reported remote screen policy is connected.',
          tone: 'red',
        },
      ],
      options: [
        {
          label: 'Remote screen runtime',
          detail: 'Live viewing requires an owner-backed session, capability, permission, route, and custody report.',
          enabled: false,
          tone: 'red',
        },
      ],
      actions: [],
      status: [
        { label: 'Runtime', detail: 'Unavailable', tone: 'red' },
        { label: 'Authority', detail: 'Not reported', tone: 'gold' },
        { label: 'Custody', detail: 'Not reported', tone: 'purple' },
      ],
    };
  }

  if (key.includes('screen')) {
    return {
      title: 'Screen Analysis',
      devices,
      modes: [
        { label: 'Off', detail: 'Do not run screen summaries.', tone: 'cyan' },
        { label: 'Summary', detail: 'Create parent-readable local summaries.', tone: 'gold' },
        { label: 'Ask first', detail: 'Require parent approval before enabling.', tone: 'purple' },
      ],
      options: [
        {
          label: 'Local summarizer',
          detail: 'Summaries run on the child device when available.',
          enabled: false,
          tone: 'cyan',
        },
        { label: 'No screenshots in alerts', detail: 'Alerts never send raw screenshots.', enabled: true, tone: 'red' },
        { label: 'Evidence citations', detail: 'Show why a summary was produced.', enabled: true, tone: 'gold' },
        { label: 'Sensitive redaction', detail: 'Avoid raw private evidence exposure.', enabled: true, tone: 'cyan' },
      ],
      actions: [
        { label: 'Check support', detail: 'Verify local screen analysis capability.', tone: 'cyan' },
        { label: 'Set privacy', detail: 'Choose summary and evidence level.', tone: 'gold' },
        { label: 'Open guide', detail: 'Explain screen analysis to the parent.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (
    key.includes('apps-games') ||
    key.includes('app-and-game') ||
    key.includes('app-use') ||
    key.includes('app-game')
  ) {
    return {
      title: 'App Use',
      devices,
      modes: [
        { label: 'Observe', detail: 'Record app sessions without blocking.', tone: 'cyan' },
        { label: 'Limit', detail: 'Apply time budgets and schedules.', tone: 'gold' },
        { label: 'Block', detail: 'Block selected apps when supported.', tone: 'red' },
      ],
      options: [
        { label: 'Known apps', detail: 'Show named app sessions.', enabled: true, tone: 'cyan' },
        { label: 'New app asks', detail: 'Ask parent before unknown apps get time.', enabled: true, tone: 'gold' },
        { label: 'Focus budget', detail: 'Separate focus tools from entertainment.', enabled: false, tone: 'purple' },
        {
          label: 'Suspicious app flag',
          detail: 'Surface apps that do not match normal use.',
          enabled: true,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Choose apps', detail: 'Open app allow, ask, block list.', tone: 'cyan' },
        { label: 'Set budget', detail: 'Set time limit and schedule.', tone: 'gold' },
        { label: 'Review sessions', detail: 'Open recent app timeline.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('network')) {
    return {
      title: 'Network Activity',
      devices,
      modes: [
        { label: 'Metadata', detail: 'Keep domain and flow metadata.', tone: 'cyan' },
        { label: 'Review', detail: 'Ask parent on risky destinations.', tone: 'gold' },
        { label: 'Block', detail: 'Block destinations when adapters support it.', tone: 'red' },
      ],
      options: [
        { label: 'Domain list', detail: 'Show domains, not raw packet contents.', enabled: true, tone: 'cyan' },
        { label: 'No payload capture', detail: 'Never collect private payload bodies.', enabled: true, tone: 'red' },
        { label: 'School allowlist', detail: 'Permit known learning destinations.', enabled: false, tone: 'gold' },
        { label: 'Unsupported warning', detail: 'Show when OS control is unavailable.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Open domains', detail: 'Review recent domains.', tone: 'cyan' },
        { label: 'Edit allowlist', detail: 'Choose trusted domains.', tone: 'gold' },
        { label: 'Export summary', detail: 'Save parent-owned network summary.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('channel')) {
    return {
      title: 'Notification Channels',
      devices,
      modes: [
        {
          label: 'Unavailable',
          detail: 'No service-reported notification channel registry is connected.',
          tone: 'red',
        },
      ],
      options: [
        {
          label: 'Channel registry',
          detail: 'A verified parent-owned channel must be reported by the notification service.',
          enabled: false,
          tone: 'red',
        },
      ],
      actions: [],
      status: [
        { label: 'Registry', detail: 'Not reported', tone: 'red' },
        { label: 'Delivery', detail: 'Unavailable', tone: 'gold' },
        { label: 'Receipts', detail: 'Not reported', tone: 'purple' },
      ],
    };
  }

  if (key.includes('alert') || key.includes('notification')) {
    return {
      title: 'Alerts',
      devices,
      modes: [
        {
          label: 'Unavailable',
          detail: 'No service-reported parent notification state is connected.',
          tone: 'red',
        },
      ],
      options: [
        {
          label: 'Notification intent',
          detail: 'A current service-issued intent and provider state are required before alerts can be shown.',
          enabled: false,
          tone: 'red',
        },
      ],
      actions: [],
      status: [
        { label: 'Intent', detail: 'Not reported', tone: 'red' },
        { label: 'Preferences', detail: 'Not reported', tone: 'gold' },
        { label: 'Delivery', detail: 'Unavailable', tone: 'purple' },
      ],
    };
  }

  if (key.includes('drive')) {
    return {
      title: 'Drives',
      devices,
      modes: [
        { label: 'Disconnected', detail: 'No drive export destination.', tone: 'cyan' },
        { label: 'Connect', detail: 'Use parent-owned storage.', tone: 'gold' },
        { label: 'Review', detail: 'Preview before any export.', tone: 'purple' },
      ],
      options: [
        { label: 'Google Drive', detail: 'Connect parent-owned Drive export.', enabled: false, tone: 'cyan' },
        { label: 'OneDrive', detail: 'Use a parent-owned OneDrive target.', enabled: false, tone: 'purple' },
        { label: 'Report exports', detail: 'Allow selected reports to export.', enabled: true, tone: 'gold' },
        {
          label: 'Support message record',
          detail: 'Review parent message before sharing.',
          enabled: true,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Connect drive', detail: 'Start parent-owned drive connection.', tone: 'cyan' },
        { label: 'Test export', detail: 'Write a minimal test file.', tone: 'gold' },
        { label: 'Disconnect', detail: 'Remove this storage target.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('export') || key.includes('retention') || key.includes('private')) {
    return {
      title: 'Export Delete Retention',
      devices,
      modes: [
        { label: 'Keep local', detail: 'No export destination.', tone: 'cyan' },
        { label: 'Export copy', detail: 'Write selected data to parent storage.', tone: 'gold' },
        { label: 'Delete class', detail: 'Delete selected local data class.', tone: 'red' },
      ],
      options: [
        { label: 'Activity summaries', detail: 'Include daily and weekly reports.', enabled: true, tone: 'cyan' },
        { label: 'Audit log', detail: 'Include rule, pairing, and export records.', enabled: true, tone: 'gold' },
        { label: 'Retention window', detail: 'Choose what local data expires.', enabled: true, tone: 'purple' },
        {
          label: 'Raw evidence excluded',
          detail: 'Do not export raw private evidence by default.',
          enabled: true,
          tone: 'red',
        },
      ],
      actions: [
        { label: 'Export report', detail: 'Write selected report to storage.', tone: 'gold' },
        { label: 'Preview data', detail: 'Show selected data classes.', tone: 'cyan' },
        { label: 'Delete selected', detail: 'Delete selected local data class.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('api')) {
    return {
      title: 'API Providers',
      devices,
      modes: [
        { label: 'Disabled', detail: 'Do not use external AI providers.', tone: 'cyan' },
        { label: 'Ask parent', detail: 'Require parent choice per provider.', tone: 'gold' },
        { label: 'Enabled', detail: 'Use configured provider limits.', tone: 'purple' },
      ],
      options: [
        { label: 'OpenAI key', detail: 'Parent-owned provider key.', enabled: false, tone: 'cyan' },
        { label: 'Per-device limits', detail: 'Limit which child devices can call APIs.', enabled: true, tone: 'gold' },
        { label: 'No raw evidence', detail: 'Do not send raw evidence to providers.', enabled: true, tone: 'red' },
        { label: 'Cost guard', detail: 'Use budget and request limits.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Add provider', detail: 'Add parent-owned API provider.', tone: 'gold' },
        { label: 'Test key', detail: 'Validate provider connection.', tone: 'cyan' },
        { label: 'Disable API', detail: 'Turn off external providers.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('memory')) {
    return {
      title: 'Memory Setup',
      devices,
      modes: [
        { label: 'Off', detail: 'Do not store cited memory.', tone: 'cyan' },
        { label: 'Cited', detail: 'Store only cited parent-approved context.', tone: 'gold' },
        { label: 'Review', detail: 'Require parent review before reuse.', tone: 'purple' },
      ],
      options: [
        { label: 'Cited answers', detail: 'Show sources for activity explanations.', enabled: true, tone: 'purple' },
        { label: 'Parent review', detail: 'Parent can review and revoke memory.', enabled: true, tone: 'cyan' },
        { label: 'Per-device memory', detail: 'Keep child-device context separated.', enabled: true, tone: 'gold' },
        { label: 'Export/delete', detail: 'Parent controls memory export and deletion.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Review memory', detail: 'Open cited memory controls.', tone: 'purple' },
        { label: 'Export memory', detail: 'Save parent-owned memory copy.', tone: 'gold' },
        { label: 'Clear memory', detail: 'Delete selected memory records.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('ai')) {
    return {
      title: 'AI Setup',
      devices,
      modes: [
        { label: 'Local AI', detail: 'Use local model when available.', tone: 'cyan' },
        { label: 'Local hub', detail: 'Queue to stronger local family machine.', tone: 'gold' },
        { label: 'API opt-in', detail: 'Use external provider only by parent choice.', tone: 'purple' },
      ],
      options: [
        { label: 'Cited answers', detail: 'Show sources for activity explanations.', enabled: true, tone: 'purple' },
        { label: 'Per-device model', detail: 'Choose model by child device capability.', enabled: true, tone: 'cyan' },
        {
          label: 'External provider keys',
          detail: 'Parent supplies and controls API keys.',
          enabled: false,
          tone: 'gold',
        },
        { label: 'Memory review', detail: 'Review, revoke, and export cited memory.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Choose model', detail: 'Set local or provider model for this device.', tone: 'cyan' },
        { label: 'Add API key', detail: 'Connect an optional parent-owned provider.', tone: 'gold' },
        { label: 'Review memory', detail: 'Open cited memory controls.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('entitlement')) {
    return {
      title: 'Entitlements',
      devices,
      modes: [{ label: 'Unavailable', detail: 'No verified entitlement snapshot is connected.', tone: 'red' }],
      options: [
        {
          label: 'Verified snapshot',
          detail: 'Requires a current owner-verified entitlement snapshot.',
          enabled: false,
          tone: 'red',
        },
        { label: 'Device seats', detail: 'No billing-backed seat limit is reported.', enabled: false, tone: 'gold' },
        {
          label: 'Feature access',
          detail: 'No paid feature access is inferred locally.',
          enabled: false,
          tone: 'purple',
        },
        {
          label: 'Billing service',
          detail: 'Connect the authenticated billing service to refresh this page.',
          enabled: false,
          tone: 'cyan',
        },
      ],
      actions: [],
      status: baseStatus,
    };
  }

  if (key.includes('subscription')) {
    return {
      title: 'Subscription',
      devices,
      modes: [{ label: 'Unavailable', detail: 'No authenticated subscription state is connected.', tone: 'red' }],
      options: [
        { label: 'Current plan', detail: 'No billing-backed plan is reported.', enabled: false, tone: 'cyan' },
        { label: 'Device seats', detail: 'No billing-backed seat limit is reported.', enabled: false, tone: 'gold' },
        {
          label: 'Billing portal',
          detail: 'No authenticated portal handoff is available.',
          enabled: false,
          tone: 'purple',
        },
        { label: 'Subscription state', detail: 'No lifecycle state is reported.', enabled: false, tone: 'red' },
      ],
      actions: [],
      status: baseStatus,
    };
  }

  if (key.includes('support') || key.includes('diagnostic')) {
    return {
      title: 'Support',
      devices,
      modes: [
        { label: 'Self check', detail: 'Run local health checks.', tone: 'cyan' },
        { label: 'Message', detail: 'Draft parent-authored support text.', tone: 'gold' },
        { label: 'Contact', detail: 'Prepare support request.', tone: 'purple' },
      ],
      options: [
        { label: 'Include versions', detail: 'Attach app and service versions.', enabled: true, tone: 'cyan' },
        {
          label: 'Include capability',
          detail: 'Attach supported/degraded/unavailable states.',
          enabled: true,
          tone: 'gold',
        },
        {
          label: 'Exclude evidence',
          detail: 'Do not include raw child evidence by default.',
          enabled: true,
          tone: 'red',
        },
        { label: 'Parent review', detail: 'Parent sees message before sending.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Run diagnostics', detail: 'Check portal and child service health.', tone: 'cyan' },
        { label: 'Write message', detail: 'Draft parent-reviewed support text.', tone: 'gold' },
        { label: 'Open support', detail: 'Open help and contact options.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('update') || key.includes('install')) {
    return {
      title: 'Install Updates',
      devices,
      modes: [
        { label: 'Stable', detail: 'Use stable update channel.', tone: 'cyan' },
        { label: 'Preview', detail: 'Use preview builds when selected.', tone: 'gold' },
        { label: 'Rollback', detail: 'Return to last working build.', tone: 'red' },
      ],
      options: [
        { label: 'Auto check', detail: 'Check for updates at startup.', enabled: true, tone: 'cyan' },
        { label: 'Ask before install', detail: 'Parent approves update install.', enabled: true, tone: 'gold' },
        { label: 'Rollback point', detail: 'Keep last working installer state.', enabled: true, tone: 'purple' },
        { label: 'Mobile app', detail: 'Show mobile app status separately.', enabled: false, tone: 'red' },
      ],
      actions: [
        { label: 'Check update', detail: 'Check available versions.', tone: 'cyan' },
        { label: 'Install now', detail: 'Apply approved update.', tone: 'gold' },
        { label: 'Rollback app', detail: 'Use last known good install.', tone: 'red' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('platform')) {
    return {
      title: 'Platforms',
      devices,
      modes: [
        { label: 'Desktop', detail: 'Tauri parent and child desktop apps.', tone: 'cyan' },
        { label: 'Mobile', detail: 'Mobile parent app target.', tone: 'gold' },
        { label: 'Unsupported', detail: 'Show honest platform gaps.', tone: 'red' },
      ],
      options: [
        { label: 'Desktop app', detail: 'Tauri parent portal target.', enabled: true, tone: 'cyan' },
        { label: 'Child service', detail: 'Headless child-device agent target.', enabled: true, tone: 'gold' },
        { label: 'Mobile app', detail: 'Mobile parent controls target.', enabled: false, tone: 'purple' },
        { label: 'Honest limits', detail: 'Show unavailable platform capabilities.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Open install', detail: 'Open installation checklist.', tone: 'cyan' },
        { label: 'Check platform', detail: 'Verify current platform support.', tone: 'gold' },
        { label: 'Open guide', detail: 'Explain platform targets.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('remote')) {
    return {
      title: 'Remote Access',
      devices,
      modes: [{ label: 'Unavailable', detail: 'No authenticated remote session is reported.', tone: 'red' }],
      options: [
        {
          label: 'Authenticated session',
          detail: 'No owner-backed remote session is connected.',
          enabled: false,
          tone: 'cyan',
        },
        { label: 'Trusted target', detail: 'No current trusted target is reported.', enabled: false, tone: 'gold' },
        {
          label: 'Transport route',
          detail: 'No verified remote transport route is reported.',
          enabled: false,
          tone: 'purple',
        },
        { label: 'Current authority', detail: 'No current parent authority is reported.', enabled: false, tone: 'red' },
      ],
      actions: [],
      status: baseStatus,
    };
  }

  if (key.includes('audit')) {
    return {
      title: 'Audit History',
      devices,
      modes: [
        { label: 'Today', detail: 'Recent decisions.', tone: 'cyan' },
        { label: 'By device', detail: 'Filter to child device.', tone: 'gold' },
        { label: 'By area', detail: 'Rules, exports, AI, pairing.', tone: 'purple' },
      ],
      options: [
        { label: 'Rule changes', detail: 'Show policy edits and approvals.', enabled: true, tone: 'gold' },
        { label: 'Export events', detail: 'Show data export/delete records.', enabled: true, tone: 'cyan' },
        {
          label: 'Pairing attempts',
          detail: 'Show accepted and rejected LAN attempts.',
          enabled: true,
          tone: 'purple',
        },
        { label: 'Failures', detail: 'Show degraded and unavailable outcomes.', enabled: true, tone: 'red' },
      ],
      actions: [
        { label: 'Filter audit', detail: 'Choose device, area, and time range.', tone: 'cyan' },
        { label: 'Export audit', detail: 'Save parent-owned audit report.', tone: 'gold' },
        { label: 'Open event', detail: 'Inspect selected decision details.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  if (key.includes('setting')) {
    return {
      title: 'Family Settings',
      devices,
      modes: [
        { label: 'Family', detail: 'Apply to all child devices.', tone: 'cyan' },
        { label: 'Child', detail: 'Override for one child.', tone: 'gold' },
        { label: 'Parent', detail: 'Parent identity and preferences.', tone: 'purple' },
      ],
      options: [
        { label: 'Your House Your Rule', detail: 'Use family-defined controls first.', enabled: true, tone: 'gold' },
        { label: 'Light and dark', detail: 'Theme follows parent choice.', enabled: true, tone: 'cyan' },
        { label: 'Require parent session', detail: 'Protect changes behind parent login.', enabled: true, tone: 'red' },
        { label: 'Child profile defaults', detail: 'Create defaults for new devices.', enabled: true, tone: 'purple' },
      ],
      actions: [
        { label: 'Edit family', detail: 'Open family defaults.', tone: 'cyan' },
        { label: 'Add child', detail: 'Create a child profile.', tone: 'gold' },
        { label: 'Parent login', detail: 'Protect this console session.', tone: 'purple' },
      ],
      status: baseStatus,
    };
  }

  return null;
}

function manageWorkspaceKindFor(
  activeNavLabel: string,
  selectedControlName: string,
  title?: string
): ManageWorkspaceKind | null {
  const navKey = assetKey(activeNavLabel);
  const selectedControlKey = assetKey(selectedControlName);
  if (selectedControlKey.includes('remote-access')) return null;
  if (navKey.includes('device') || navKey.includes('activity')) return null;
  if (navKey.includes('portal')) return 'portal';
  if (navKey.includes('account')) return 'account';
  if (navKey.includes('data')) return 'data';
  if (navKey === 'ai' || navKey.includes('ai-memory')) return 'ai';
  if (
    navKey.includes('policy') ||
    navKey.includes('browser') ||
    navKey.includes('app') ||
    navKey.includes('game') ||
    navKey.includes('screen') ||
    navKey.includes('network') ||
    navKey.includes('tracking') ||
    navKey.includes('remote-screen')
  )
    return 'policy';
  const key = `${navKey} ${assetKey(selectedControlName)} ${assetKey(title ?? '')}`;
  if (key.includes('lan-pairing') || key.includes('local-area-network') || key.includes('report')) return null;
  if (
    key.includes('portal') ||
    key.includes('family-setting') ||
    key.includes('settings-rules') ||
    key.includes('notification') ||
    key.includes('alert') ||
    key.includes('channel')
  )
    return 'portal';
  if (
    key.includes('subscription') ||
    key.includes('entitlement') ||
    key.includes('account') ||
    key.includes('support') ||
    key.includes('diagnostic') ||
    key.includes('access') ||
    key.includes('plan')
  )
    return 'account';
  if (
    key.includes('data') ||
    key.includes('drive') ||
    key.includes('export') ||
    key.includes('retention') ||
    key.includes('audit') ||
    key.includes('remote-access')
  )
    return 'data';
  if (
    key.includes('ai') ||
    key.includes('api') ||
    key.includes('model') ||
    key.includes('inference') ||
    key.includes('memory')
  )
    return 'ai';
  if (
    key.includes('policy') ||
    key.includes('browser') ||
    key.includes('rule') ||
    key.includes('schedule') ||
    key.includes('approval') ||
    key.includes('enforce') ||
    key.includes('app') ||
    key.includes('game') ||
    key.includes('screen') ||
    key.includes('network') ||
    key.includes('tracking') ||
    key.includes('remote-screen')
  )
    return 'policy';
  return null;
}

function manageWorkspaceTabs(kind: ManageWorkspaceKind): readonly ManageWorkspaceTab[] {
  if (kind === 'portal') {
    return [
      { id: 'settings', label: 'Settings', icon: ManageFileSettingsIcon, tone: 'cyan' },
      { id: 'alerts', label: 'Alerts', icon: AlertNotificationBellIcon, tone: 'red' },
      { id: 'channels', label: 'Channels', icon: AlertNotificationBellIcon, tone: 'gold' },
      { id: 'runtime', label: 'Runtime', icon: PortalGatewayIcon, tone: 'cyan' },
    ];
  }
  if (kind === 'account') {
    return [
      { id: 'plan', label: 'Plan', icon: AccountProfileIcon, tone: 'gold' },
      { id: 'access', label: 'Access', icon: PolicyShieldDocumentIcon, tone: 'cyan' },
      { id: 'support', label: 'Support', icon: PortalGatewayIcon, tone: 'purple' },
    ];
  }
  if (kind === 'data') {
    return [
      { id: 'storage', label: 'Storage', icon: DrivesCloudIcon, tone: 'gold' },
      { id: 'export', label: 'Export', icon: ExportRetentionIcon, tone: 'cyan' },
      { id: 'retention', label: 'Retention', icon: DataPrivacyServerShieldIcon, tone: 'purple' },
      { id: 'audit', label: 'Audit', icon: AuditCloudLogsIcon, tone: 'gold' },
    ];
  }
  if (kind === 'ai') {
    return [
      { id: 'runtime', label: 'Runtime', icon: AiSetupSearchIcon, tone: 'cyan' },
      { id: 'hardware', label: 'Hardware', icon: DevicesMultiScreenIcon, tone: 'gold' },
      { id: 'models', label: 'Models', icon: AiMemoryCircuitIcon, tone: 'purple' },
      { id: 'inference', label: 'Inference', icon: AiGuideIdeaIcon, tone: 'cyan' },
      { id: 'templates', label: 'Templates', icon: ReportDocumentIcon, tone: 'gold' },
      { id: 'providers', label: 'Providers', icon: ApiKeysChipIcon, tone: 'purple' },
      { id: 'memory', label: 'Memory', icon: AiMemorySetBrainIcon, tone: 'purple' },
      { id: 'activity', label: 'Activity', icon: ActivityNetworkIcon, tone: 'cyan' },
    ];
  }
  return [
    { id: 'rules', label: 'Rules', icon: PolicyShieldDocumentIcon, tone: 'gold' },
    { id: 'schedule', label: 'Schedule', icon: ScheduleCalendarClockIcon, tone: 'purple' },
    { id: 'budget', label: 'Budget', icon: ScheduleCalendarClockIcon, tone: 'gold' },
    { id: 'approvals', label: 'Approvals', icon: AlertNotificationBellIcon, tone: 'cyan' },
    { id: 'audit', label: 'Audit', icon: AuditCloudLogsIcon, tone: 'cyan' },
  ];
}

function manageWorkspaceDefaultTabId(
  kind: ManageWorkspaceKind,
  activeNavLabel: string,
  selectedControlName: string
): string {
  const key = `${assetKey(activeNavLabel)} ${assetKey(selectedControlName)}`;
  if (kind === 'portal') {
    if (key.includes('channel')) return 'channels';
    if (key.includes('alert') || key.includes('notification')) return 'alerts';
    if (key.includes('runtime')) return 'runtime';
    return 'settings';
  }
  if (kind === 'account') {
    if (key.includes('support') || key.includes('diagnostic')) return 'support';
    if (key.includes('access') || key.includes('entitlement')) return 'access';
    return 'plan';
  }
  if (kind === 'data') {
    if (key.includes('audit')) return 'audit';
    if (key.includes('retention')) return 'retention';
    if (key.includes('drive') || key.includes('storage')) return 'storage';
    if (key.includes('export')) return 'export';
    return 'storage';
  }
  if (kind === 'ai') {
    if (key.includes('hardware')) return 'hardware';
    if (key.includes('model')) return 'models';
    if (key.includes('inference')) return 'inference';
    if (key.includes('template')) return 'templates';
    if (key.includes('provider') || key.includes('api')) return 'providers';
    if (key.includes('memory')) return 'memory';
    return 'runtime';
  }
  if (key.includes('budget')) return 'budget';
  if (key.includes('schedule')) return 'schedule';
  if (key.includes('approval') || key.includes('ask')) return 'approvals';
  if (key.includes('enforce') || key.includes('dry-run')) return 'rules';
  if (key.includes('audit') || key.includes('preview')) return 'audit';
  return 'rules';
}

function manageWorkspaceTitle(kind: ManageWorkspaceKind): string {
  if (kind === 'portal') return 'Portal';
  if (kind === 'account') return 'Account';
  if (kind === 'data') return 'Data';
  if (kind === 'ai') return 'AI';
  return 'Policy';
}

function managePolicyAreaLabel(activeNavLabel: string, selectedControlName: string): string {
  const navKey = assetKey(activeNavLabel);
  const selectedControlKey = assetKey(selectedControlName);
  const key = `${navKey} ${selectedControlKey}`;
  if (navKey === 'approvals') return 'Approvals';
  if (navKey === 'schedules') return 'Schedules';
  if (navKey === 'enforce' || navKey === 'enforcement') return 'Enforcement';
  if (navKey === 'rules' || navKey === 'policy') return 'Rules';
  if (navKey === 'remote-screen' || selectedControlKey.includes('remote-screen')) return 'Remote Screen';
  if (
    navKey === 'app-use' ||
    key.includes('app-game-sessions') ||
    key.includes('apps-games') ||
    key.includes('app-and-game')
  )
    return 'App Use';
  if (navKey === 'apps' || selectedControlKey.includes('app-policy')) return 'Apps';
  if (navKey === 'games' || selectedControlKey.includes('game-policy')) return 'Games';
  if (navKey === 'screen' || selectedControlKey.includes('screen-analysis')) return 'Screen';
  if (navKey === 'network' || selectedControlKey.includes('network-activity')) return 'Network';
  if (navKey === 'tracking' || key.includes('tracking') || key.includes('location')) return 'Tracking';
  return 'Browser';
}

function managePolicyAreaIcon(activeNavLabel: string, selectedControlName: string): IconComponent {
  const area = managePolicyAreaLabel(activeNavLabel, selectedControlName);
  if (area === 'Rules') return PolicyShieldDocumentIcon;
  if (area === 'Schedules') return ScheduleCalendarClockIcon;
  if (area === 'Approvals') return AlertNotificationBellIcon;
  if (area === 'Enforcement') return EnforcementOfficerIcon;
  if (area === 'App Use') return AppIcon;
  if (area === 'Apps') return AppIcon;
  if (area === 'Games') return GamesIcon;
  if (area === 'Remote Screen') return RemoteAccessMonitorsIcon;
  if (area === 'Screen') return ScreenAnalysisIcon;
  if (area === 'Network') return WebGlobeIcon;
  if (area === 'Tracking') return TrackingLocationIcon;
  return BrowserStackIcon;
}

function manageWorkspaceTargetOptions(kind: ManageWorkspaceKind): readonly ManageWorkspaceTargetOption[] {
  if (kind === 'account') {
    return [
      { id: 'family', label: 'Family', detail: 'Family plan, support, and gate defaults.', tone: 'cyan' },
      { id: 'perDevice', label: 'Per Device', detail: 'Child seat, entitlement, and support scope.', tone: 'gold' },
    ];
  }
  if (kind === 'policy') {
    return [
      { id: 'family', label: 'Family', detail: 'Family default policy.', tone: 'cyan' },
      { id: 'perDevice', label: 'Per Device', detail: 'Child override policy.', tone: 'gold' },
    ];
  }
  if (kind === 'data') {
    return [
      { id: 'family', label: 'Family', detail: 'Family custody/export view.', tone: 'cyan' },
      { id: 'perDevice', label: 'Per Device', detail: 'Child evidence/data view.', tone: 'gold' },
    ];
  }
  if (kind === 'ai') {
    return [
      { id: 'family', label: 'Family', detail: 'Family AI defaults.', tone: 'cyan' },
      { id: 'perDevice', label: 'Per Device', detail: 'Child runtime setup.', tone: 'gold' },
      { id: 'portal', label: 'Portal', detail: 'Portal assistant setup.', tone: 'purple' },
    ];
  }
  return [];
}

function manageWorkspaceTargetLabel(target: ManageWorkspaceTarget): string {
  if (target === 'perDevice') return 'Per device';
  if (target === 'portal') return 'Portal';
  return 'Family';
}

type ManageWorkspaceTargetStatus = {
  readonly ariaLabel: string;
  readonly eyebrow: string;
  readonly detail: string;
};

function manageWorkspaceTargetStatus(
  target: ManageWorkspaceTarget,
  selectedDevice: DeviceSlot | null,
  hasRuntimeDeviceSlots: boolean
): ManageWorkspaceTargetStatus | null {
  if (target === 'family') {
    return {
      ariaLabel: 'Whole family manage target scope',
      eyebrow: 'WHOLE FAMILY TARGET',
      detail: hasRuntimeDeviceSlots
        ? 'Family-wide settings are shown below. Choose Per Device to inspect a current child.'
        : 'Family-wide settings are shown below. Per Device appears after a current child is reported.',
    };
  }
  if (target === 'portal') {
    return {
      ariaLabel: 'Parent console manage target scope',
      eyebrow: 'PARENT CONSOLE TARGET',
      detail: 'This workspace is scoped to the parent console. Child-device settings remain separate.',
    };
  }
  if (!selectedDevice) {
    return {
      ariaLabel: 'No current manage device target',
      eyebrow: 'NO CURRENT DEVICE TARGET',
      detail: 'Connect the local service and choose a current household device before using per-device controls.',
    };
  }
  return null;
}

function sharedWorkspaceTargetForOptions(
  targetOptions: readonly ManageWorkspaceTargetOption[],
  sharedTargetSelection: ManageTargetSelection
): ManageWorkspaceTarget {
  const defaultTarget = targetOptions[0]?.id ?? 'family';
  if (!targetOptions.some((option) => option.id === 'perDevice')) {
    return defaultTarget;
  }
  return sharedTargetSelection.scope === 'perDevice' ? 'perDevice' : defaultTarget;
}

function reconciledWorkspaceTargetForOptions(
  targetOptions: readonly ManageWorkspaceTargetOption[],
  sharedTargetSelection: ManageTargetSelection,
  currentTarget: ManageWorkspaceTarget
): ManageWorkspaceTarget {
  if (
    sharedTargetSelection.scope !== 'perDevice' &&
    currentTarget === 'portal' &&
    targetOptions.some((option) => option.id === 'portal')
  ) {
    return 'portal';
  }
  return sharedWorkspaceTargetForOptions(targetOptions, sharedTargetSelection);
}

function manageWorkspaceSummary(
  kind: ManageWorkspaceKind,
  activeTab: string,
  activeNavLabel = '',
  selectedControlName = '',
  workspaceTarget: ManageWorkspaceTarget = 'family'
): string {
  if (kind === 'portal') {
    if (activeTab === 'alerts')
      return 'No service-reported notification intent, preference, or delivery state is available.';
    if (activeTab === 'channels')
      return 'No verified parent-owned notification channel registry or delivery receipt is available.';
    if (activeTab === 'runtime') return 'Local portal health, Rust service state, version, and update posture.';
    return 'Parent profile defaults, privacy posture, login protection, and console preferences.';
  }
  if (kind === 'account') {
    const target = manageWorkspaceTargetLabel(workspaceTarget).toLowerCase();
    if (activeTab === 'access')
      return `No verified ${target} entitlement snapshot is connected. This page does not infer paid access.`;
    if (activeTab === 'support') return `Send a parent-authored support message for the ${target} account scope.`;
    return `No authenticated ${target} subscription summary is connected. Pricing and billing actions stay unavailable.`;
  }
  if (kind === 'data') {
    const target = manageWorkspaceTargetLabel(workspaceTarget).toLowerCase();
    if (activeTab === 'export')
      return `No ${target} export capability or destination is reported. Export actions remain unavailable.`;
    if (activeTab === 'retention') return `No ${target} retention or deletion policy snapshot is reported.`;
    if (activeTab === 'audit') return `No ${target} custody audit rows are reported to this page.`;
    return `No ${target} storage or connector state is reported to this page.`;
  }
  if (kind === 'ai') {
    const target = manageWorkspaceTargetLabel(workspaceTarget).toLowerCase();
    if (activeTab === 'hardware') return `Review ${target} CPU, RAM, GPU, VRAM, NPU, disk, battery, and thermal fit.`;
    if (activeTab === 'models') return `Manage ${target} GGUF model inventory, downloads, load state, and device fit.`;
    if (activeTab === 'inference')
      return `Tune ${target} llama.cpp generation settings, limits, routing, and degraded modes.`;
    if (activeTab === 'templates')
      return `Manage ${target} prompt templates for reports, screen summaries, assistant, and structured output.`;
    if (activeTab === 'providers')
      return `No owner-backed ${target} external AI provider, key, budget, or raw-evidence policy state is available.`;
    if (activeTab === 'memory')
      return `No service-reported ${target} cited-memory registry, review state, export state, or audit state is available.`;
    if (activeTab === 'activity')
      return `Inspect ${target} AI job queue, load/unload events, failed inference, and evaluator traces.`;
    return `No service-reported ${target} local AI runtime or household job state is available.`;
  }
  const area = managePolicyAreaLabel(activeNavLabel, selectedControlName).toLowerCase();
  const scope =
    workspaceTarget === 'portal'
      ? 'parent portal behavior'
      : workspaceTarget === 'family'
        ? 'family defaults'
        : 'per-device overrides';
  if (area === 'remote screen') {
    if (activeTab === 'schedule')
      return `Configure ${scope} for remote screen request windows, child consent, and stop/revoke behavior. Backend not implemented yet.`;
    if (activeTab === 'audit')
      return `Review ${scope} for remote screen session requests, custody labels, parent actions, and stop/revoke audit. Backend not implemented yet.`;
    return `Remote screen live-view backend is not implemented yet; Rust session, capability, permission, custody, and audit wiring are required.`;
  }
  if (activeTab === 'schedule')
    return `Review ${scope} for ${area} time windows. Connect the local service to make changes.`;
  if (activeTab === 'budget')
    return `Review ${scope} for ${area} caps and reset windows. Connect the local service to make changes.`;
  if (activeTab === 'approvals')
    return `Review ${scope} for ${area} ask-parent requests. Connect the local service to make changes.`;
  if (activeTab === 'enforcement')
    return `Review ${scope} for ${area} enforcement status. Connect the local service to make changes.`;
  if (activeTab === 'audit')
    return `Review ${scope} for ${area} previews, capability results, parent changes, and child-device event evidence.`;
  return `Review ${scope} for ${area} decisions. Connect the local service to make changes.`;
}

function manageWorkspaceCards(
  kind: ManageWorkspaceKind,
  activeTab: string,
  activeNavLabel = '',
  selectedControlName = '',
  workspaceTarget: ManageWorkspaceTarget = 'family'
): readonly ManageWorkspaceCard[] {
  if (kind === 'portal') {
    if (activeTab === 'alerts') {
      return [
        {
          label: 'Notification state',
          value: 'Not reported',
          body: 'No generic parent notification read model is connected to this route.',
          tone: 'red',
        },
        {
          label: 'Intent authority',
          value: 'Unavailable',
          body: 'No current service-issued notification intent is available to render or deliver.',
          tone: 'gold',
        },
        {
          label: 'Quiet hours',
          value: 'Not reported',
          body: 'No persisted parent notification preference is connected to this route.',
          tone: 'purple',
        },
        {
          label: 'Delivery history',
          value: 'Not reported',
          body: 'No provider receipt or in-app notification history is connected.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'channels') {
      return [
        {
          label: 'Channel registry',
          value: 'Not reported',
          body: 'No verified parent-owned notification destination is connected.',
          tone: 'red',
        },
        {
          label: 'Portal notices',
          value: 'Not reported',
          body: 'No generic in-app notification state is connected to this route.',
          tone: 'gold',
        },
        {
          label: 'External delivery',
          value: 'Unavailable',
          body: 'No email, SMS, WhatsApp, or other provider delivery owner is connected.',
          tone: 'purple',
        },
        {
          label: 'Test message',
          value: 'Unavailable',
          body: 'A test cannot be sent until a verified channel and notification service report current state.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'runtime') {
      return [
        {
          label: 'Rust agent',
          value: 'Read from service',
          body: 'Health, version, command surface, and LAN route state come from the local agent.',
          tone: 'cyan',
        },
        {
          label: 'Portal build',
          value: 'Package metadata',
          body: 'Version display stays honest and must follow package metadata.',
          tone: 'gold',
        },
        {
          label: 'Updates',
          value: 'Parent approved',
          body: 'Check, install, and rollback stay explicit parent actions.',
          tone: 'purple',
        },
        {
          label: 'Support message',
          value: 'Review before send',
          body: 'No support message leaves without parent-visible contents.',
          tone: 'red',
        },
      ];
    }
    return [
      {
        label: 'Family defaults',
        value: 'Parent owned',
        body: 'House rules, new-device defaults, and privacy level are portal settings.',
        tone: 'cyan',
      },
      {
        label: 'Parent session',
        value: 'Required',
        body: 'Sensitive account, channel, and policy edits require a parent session.',
        tone: 'red',
      },
      {
        label: 'Privacy posture',
        value: 'Local first',
        body: 'Cloud connectors stay opt-in and scoped.',
        tone: 'gold',
      },
      {
        label: 'Theme and console',
        value: 'Portal only',
        body: 'Visual preferences do not change child-device policy.',
        tone: 'purple',
      },
    ];
  }
  if (kind === 'account') {
    if (activeTab === 'access') {
      return [
        {
          label: 'Entitlement snapshot',
          value: 'Not reported',
          body: 'No current owner-verified entitlement snapshot is connected.',
          tone: 'red',
        },
        {
          label: 'Device seats',
          value: 'Not reported',
          body: 'No billing-backed device limit is available to this surface.',
          tone: 'gold',
        },
        {
          label: 'Feature access',
          value: 'Not evaluated',
          body: 'The portal does not infer paid feature access from local state.',
          tone: 'purple',
        },
        {
          label: 'Billing service',
          value: 'Unavailable',
          body: 'Connect the authenticated billing service to refresh entitlement state.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'support') {
      return [];
    }
    return [
      {
        label: 'Current plan',
        value: 'Not reported',
        body: 'No billing-backed plan summary is connected to the parent portal.',
        tone: 'cyan',
      },
      {
        label: 'Subscription status',
        value: 'Unavailable',
        body: 'No authenticated subscription lifecycle state is available.',
        tone: 'red',
      },
      {
        label: 'Device seats',
        value: 'Not reported',
        body: 'The portal does not invent a device limit without entitlement truth.',
        tone: 'gold',
      },
      {
        label: 'Billing actions',
        value: 'Unavailable',
        body: 'Change-plan and billing-portal actions require an authenticated handoff.',
        tone: 'purple',
      },
    ];
  }
  if (kind === 'data') {
    const targetLabel = manageWorkspaceTargetLabel(workspaceTarget);
    const targetCard: ManageWorkspaceCard = {
      label: 'Target',
      value: targetLabel,
      body:
        workspaceTarget === 'family'
          ? 'Family data views aggregate child-device reports and custody state under parent control.'
          : 'Per-device data views inspect one child device before export, retention, or delete intent.',
      tone: workspaceTarget === 'family' ? 'cyan' : 'gold',
    };
    if (activeTab === 'export') {
      return [
        targetCard,
        {
          label: 'Export service',
          value: 'Unavailable',
          body: 'No owner-backed export service is connected to this page.',
          tone: 'cyan',
        },
        {
          label: 'Destination',
          value: 'Not reported',
          body: 'No local folder or cloud destination is reported.',
          tone: 'gold',
        },
        {
          label: 'Drive sync',
          value: 'Unavailable',
          body: 'No authenticated storage connector is connected.',
          tone: 'purple',
        },
        {
          label: 'Raw evidence',
          value: 'Not reported',
          body: 'No export data-class selection is reported.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'retention') {
      return [
        targetCard,
        {
          label: 'Retention snapshot',
          value: 'Not reported',
          body: 'No owner-backed retention policy snapshot is connected.',
          tone: 'cyan',
        },
        {
          label: 'Report retention',
          value: 'Not reported',
          body: 'No report retention window is reported.',
          tone: 'gold',
        },
        {
          label: 'Evidence retention',
          value: 'Not reported',
          body: 'No evidence-summary retention window is reported.',
          tone: 'purple',
        },
        {
          label: 'Delete proof',
          value: 'Not reported',
          body: 'No deletion receipt or custody reference is reported.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'audit') {
      return [
        targetCard,
        {
          label: 'Audit history',
          value: 'Not reported',
          body: 'No typed custody audit history is connected to this page.',
          tone: 'gold',
        },
        {
          label: 'Recorded events',
          value: '0 reported',
          body: 'No audit event rows, timestamps, or identifiers were reported.',
          tone: 'red',
        },
        {
          label: 'Actor and source',
          value: 'Not reported',
          body: 'No authenticated actor, authority, or source is reported.',
          tone: 'cyan',
        },
        {
          label: 'Export and delete history',
          value: 'Not reported',
          body: 'No export, deletion, connector, or support event is inferred.',
          tone: 'purple',
        },
      ];
    }
    return [
      targetCard,
      {
        label: 'Connector state',
        value: 'Unavailable',
        body: 'No authenticated storage connector state is connected.',
        tone: 'cyan',
      },
      {
        label: 'Google Drive',
        value: 'Not reported',
        body: 'No verified Google Drive connection is reported.',
        tone: 'gold',
      },
      {
        label: 'OneDrive',
        value: 'Not reported',
        body: 'No verified OneDrive connection is reported.',
        tone: 'purple',
      },
      {
        label: 'Remote read',
        value: 'Unavailable',
        body: 'No owner-backed remote storage reader is connected.',
        tone: 'cyan',
      },
    ];
  }
  if (kind === 'ai') {
    const targetLabel = manageWorkspaceTargetLabel(workspaceTarget);
    const targetCard: ManageWorkspaceCard = {
      label: 'Target',
      value: targetLabel,
      body:
        workspaceTarget === 'portal'
          ? 'Portal AI covers parent assistant, providers, reports, and account-level budgets.'
          : workspaceTarget === 'perDevice'
            ? 'Per-device AI sends runtime, model, and hardware intent to one selected child device.'
            : 'Family AI defaults define shared runtime posture before child overrides exist.',
      tone: workspaceTarget === 'portal' ? 'purple' : workspaceTarget === 'perDevice' ? 'gold' : 'cyan',
    };
    if (activeTab === 'hardware') {
      return [
        targetCard,
        {
          label: 'CPU / RAM',
          value: 'Probe required',
          body: 'Child device reports core count, memory, disk, battery, and thermal limits.',
          tone: 'cyan',
        },
        {
          label: 'GPU / VRAM',
          value: 'Probe required',
          body: 'GPU layers, VRAM, and device split determine local model fit.',
          tone: 'gold',
        },
        {
          label: 'NPU',
          value: 'Optional',
          body: 'NPU capability is shown when platform probes can prove it.',
          tone: 'purple',
        },
        {
          label: 'Mobile',
          value: 'Strict limits',
          body: 'Mobile devices may need hub/API fallback instead of local SLM.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'models') {
      return [
        targetCard,
        {
          label: 'Model format',
          value: 'GGUF first',
          body: 'llama.cpp is the first-class runtime path.',
          tone: 'cyan',
        },
        {
          label: 'Download state',
          value: 'Typed progress',
          body: 'Queued, downloading, verifying, ready, failed, and deleted states.',
          tone: 'gold',
        },
        {
          label: 'Fit check',
          value: 'Before load',
          body: 'Model selection validates RAM/VRAM/context before loading.',
          tone: 'purple',
        },
        {
          label: 'Top picks',
          value: 'Curated list',
          body: 'Hugging Face candidates should be refreshed through a typed backend job.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'inference') {
      return [
        targetCard,
        {
          label: 'Profiles',
          value: 'Safe defaults',
          body: 'Faster, careful, low memory, and advanced profiles map to llama.cpp settings.',
          tone: 'cyan',
        },
        {
          label: 'Limits',
          value: 'Parent chosen',
          body: 'Max tokens, timeout, context, temperature, top_p, top_k, and repetition penalty.',
          tone: 'gold',
        },
        {
          label: 'Device routing',
          value: 'One runtime',
          body: 'Scheduler prevents duplicate local model runtime on the same physical device.',
          tone: 'purple',
        },
        {
          label: 'Failure state',
          value: 'Visible',
          body: 'Unavailable, degraded, queued, failed, and cancelled states are user-facing.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'providers') {
      return [
        targetCard,
        {
          label: 'Control state',
          value: 'Unavailable',
          body: 'No owner-backed provider registry, key, budget, or current policy was reported.',
          tone: 'red',
        },
        {
          label: 'Provider authority',
          value: 'Required',
          body: 'External AI requires a current parent-owned provider key and budget.',
          tone: 'purple',
        },
        {
          label: 'Raw evidence',
          value: 'Blocked by default',
          body: 'External AI does not receive raw child evidence unless future policy permits it.',
          tone: 'red',
        },
        {
          label: 'Fallback',
          value: 'Unavailable',
          body: 'No API fallback is offered until the parent-owned provider state is current.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'memory') {
      return [
        targetCard,
        {
          label: 'Control state',
          value: 'Unavailable',
          body: 'No cited-memory registry, review state, export state, or audit state was reported.',
          tone: 'red',
        },
        {
          label: 'Citations',
          value: 'Required',
          body: 'Memory-backed answers cite local sources or show unavailable.',
          tone: 'cyan',
        },
        {
          label: 'Parent controls',
          value: 'Not reported',
          body: 'Inspect, revoke, export, and delete controls require current owner-backed memory state.',
          tone: 'gold',
        },
        {
          label: 'Per device',
          value: 'Separated',
          body: 'Child-device context stays separated unless parent merges it.',
          tone: 'purple',
        },
        {
          label: 'Audit',
          value: 'Not reported',
          body: 'No creation, reuse, export, or deletion audit state is available.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'activity') {
      return [
        targetCard,
        {
          label: 'AI jobs',
          value: 'Activity gap',
          body: 'Activity should gain an AI read-model tab for queued/running/failed jobs.',
          tone: 'cyan',
        },
        {
          label: 'Load events',
          value: 'Visible',
          body: 'Model load, unload, download, and verify events belong in diagnostics.',
          tone: 'gold',
        },
        {
          label: 'Safety eval',
          value: 'Traceable',
          body: 'Safety evaluator inputs/outputs need typed references and redaction.',
          tone: 'purple',
        },
        {
          label: 'Prompt version',
          value: 'Logged',
          body: 'Template id/version and output schema should be visible for reports.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'templates') {
      return [
        targetCard,
        {
          label: 'Report prompt',
          value: 'Versioned',
          body: 'Daily/weekly/monthly report prompts have ids and schema expectations.',
          tone: 'cyan',
        },
        {
          label: 'Screen summary',
          value: 'Versioned',
          body: 'Screen prompts stay parent-controlled and citation based.',
          tone: 'gold',
        },
        {
          label: 'Assistant',
          value: 'Versioned',
          body: 'Assistant prompts separate parent guidance from child safety behavior.',
          tone: 'purple',
        },
        {
          label: 'Structured output',
          value: 'Schema bound',
          body: 'JSON output must validate before UI renders it.',
          tone: 'red',
        },
      ];
    }
    return [
      targetCard,
      {
        label: 'Runtime',
        value: 'Not reported',
        body: 'No service-reported local runtime is available in this overview.',
        tone: 'cyan',
      },
      {
        label: 'Family hub',
        value: 'Not reported',
        body: 'No household AI job state is available in this overview.',
        tone: 'gold',
      },
      {
        label: 'Load state',
        value: 'Not reported',
        body: 'Use the service-reported panel below for current runtime state.',
        tone: 'purple',
      },
      {
        label: 'Parent control',
        value: 'Unavailable',
        body: 'No owner-backed AI runtime control is connected to this overview.',
        tone: 'red',
      },
    ];
  }
  const area = managePolicyAreaLabel(activeNavLabel, selectedControlName);
  const scope = workspaceTarget === 'portal' ? 'Portal' : workspaceTarget === 'family' ? 'Family' : 'Per device';
  const targetBody =
    workspaceTarget === 'portal'
      ? `Portal ${area.toLowerCase()} settings control parent console behavior, not child-device enforcement.`
      : workspaceTarget === 'family'
        ? `Family ${area.toLowerCase()} defaults apply until a child override exists.`
        : `A child device must be selected before ${area.toLowerCase()} overrides can be sent.`;
  if (area === 'Remote Screen') {
    if (activeTab === 'schedule') {
      return [
        { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
        {
          label: 'Backend',
          value: 'Not implemented yet',
          body: 'Rust remote screen request windows, session timers, consent, and revoke flow are not wired.',
          tone: 'red',
        },
        {
          label: 'Permission',
          value: 'Required',
          body: 'Child-agent capability and platform permission state must be reported before live view is enabled.',
          tone: 'gold',
        },
        {
          label: 'Stop/revoke',
          value: 'Required',
          body: 'Remote screen sessions need a visible child-device stop path and parent-side revoke audit.',
          tone: 'purple',
        },
      ];
    }
    if (activeTab === 'audit') {
      return [
        { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
        {
          label: 'Backend',
          value: 'Not implemented yet',
          body: 'Remote screen audit requires Rust session, route, custody, permission, start, stop, and revoke events.',
          tone: 'red',
        },
        {
          label: 'Custody',
          value: 'Required',
          body: 'Audit must label local, LAN, relay, parent cache, or unavailable source before showing live-view history.',
          tone: 'gold',
        },
        {
          label: 'Separate feature',
          value: 'Not screen analysis',
          body: 'Screen analysis records periodic local summaries; remote screen is parent live view.',
          tone: 'purple',
        },
      ];
    }
    return [
      { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
      {
        label: 'Backend',
        value: 'Not implemented yet',
        body: 'Live remote screen requires Rust route/session/capability wiring before any parent can view a child screen.',
        tone: 'red',
      },
      {
        label: 'Permission',
        value: 'Capability gated',
        body: 'The child agent must report supported, permission-required, denied, or platform-unsupported state.',
        tone: 'gold',
      },
      {
        label: 'Scope',
        value: 'View only',
        body: 'Remote input/control remains a separate later capability, not part of the live-view display boundary.',
        tone: 'purple',
      },
    ];
  }
  if (activeTab === 'schedule') {
    return [
      { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
      {
        label: 'Time windows',
        value: 'School / Bedtime',
        body: 'Named windows define when this policy area changes behavior.',
        tone: 'purple',
      },
      {
        label: 'Action window',
        value: 'Allow / Ask / Block',
        body: 'The timeline decides when a rule posture is active; numeric caps stay in Budget.',
        tone: 'gold',
      },
      {
        label: 'Exceptions',
        value: 'Temporary',
        body: 'Parent-approved one-off windows should create an audit event.',
        tone: 'cyan',
      },
    ];
  }
  if (activeTab === 'budget') {
    return [
      { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
      {
        label: 'Cap',
        value: 'Daily / Weekly',
        body: `${area} caps define how many minutes are available before the rule posture changes.`,
        tone: 'gold',
      },
      {
        label: 'Counting',
        value: 'Evidence based',
        body: 'Only child-device session evidence should spend budget time; the portal does not run timers.',
        tone: 'purple',
      },
      {
        label: 'Override',
        value: 'Inherited first',
        body: 'Per-device budgets stay grey until the parent explicitly overrides the family default.',
        tone: 'cyan',
      },
    ];
  }
  if (activeTab === 'approvals') {
    return [
      { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
      {
        label: 'Ask flow',
        value: 'Reason required',
        body: `${area} requests carry child reason, parent answer, expiry, and result.`,
        tone: 'gold',
      },
      {
        label: 'Timeout',
        value: 'Policy owned',
        body: 'Expired asks fall back to the configured family or override rule.',
        tone: 'purple',
      },
      {
        label: 'Notification',
        value: 'Parent channel',
        body: 'Delivery uses verified portal channels and never sends raw child evidence by default.',
        tone: 'red',
      },
    ];
  }
  if (activeTab === 'enforcement') {
    return [
      { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
      {
        label: 'Mode',
        value: 'Observe / Dry-run / Apply',
        body: 'Parents choose posture; child agents report capability before action.',
        tone: 'red',
      },
      {
        label: 'Capability',
        value: 'Required',
        body: `${area} enforcement is disabled when the device cannot prove support.`,
        tone: 'gold',
      },
      {
        label: 'Fallback',
        value: 'Explain',
        body: 'Unsupported enforcement should become visible advice, not an unverified success claim.',
        tone: 'purple',
      },
    ];
  }
  if (activeTab === 'audit') {
    return [
      { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
      {
        label: 'Preview',
        value: 'Before send',
        body: 'Policy diffs should be previewed before they become parent intents.',
        tone: 'gold',
      },
      {
        label: 'Journal',
        value: 'Typed event',
        body: 'Child-device results and parent edits should produce typed audit entries.',
        tone: 'purple',
      },
      {
        label: 'Evidence',
        value: 'Referenced',
        body: 'Audit displays evidence references and status, not raw payloads by default.',
        tone: 'red',
      },
    ];
  }
  return [
    { label: 'Target', value: scope, body: targetBody, tone: 'cyan' },
    {
      label: 'Decision',
      value: 'Allow / Ask / Block',
      body: `${area} rules own the actual parent decision ladder.`,
      tone: 'gold',
    },
    {
      label: 'Conditions',
      value: 'Context aware',
      body: 'Rules can combine category, app/browser identity, schedule, and capability.',
      tone: 'purple',
    },
    {
      label: 'Explanation',
      value: 'Parent text',
      body: 'Warn/explain states should show the child a parent-authored reason.',
      tone: 'cyan',
    },
  ];
}

type ManageWorkspaceChoiceOption = {
  readonly value: string;
  readonly label: string;
  readonly disabled?: boolean;
};

function managePolicyPrimaryChoiceTitle(activeTab: string): string {
  if (activeTab === 'schedule') return 'Window';
  if (activeTab === 'budget') return 'Cap';
  if (activeTab === 'approvals') return 'Request';
  if (activeTab === 'enforcement') return 'Adapter';
  if (activeTab === 'audit') return 'View';
  return 'Decision';
}

function managePolicyPrimaryChoiceOptions(activeTab: string): readonly ManageWorkspaceChoiceOption[] {
  if (activeTab === 'schedule') {
    return [
      { value: 'always', label: 'Always' },
      { value: 'school', label: 'School' },
      { value: 'bedtime', label: 'Bedtime' },
      { value: 'custom', label: 'Custom' },
    ];
  }
  if (activeTab === 'budget') {
    return [
      { value: 'none', label: 'None' },
      { value: 'total', label: 'Total' },
      { value: 'target', label: 'Per Target' },
      { value: 'strict', label: 'Strict' },
    ];
  }
  if (activeTab === 'approvals') {
    return [
      { value: 'ask', label: 'Ask' },
      { value: 'reason', label: 'Reason' },
      { value: 'timebox', label: 'Time Box' },
    ];
  }
  if (activeTab === 'enforcement') {
    return [
      { value: 'observe', label: 'Observe' },
      { value: 'dry-run', label: 'Dry Run' },
      { value: 'eligible', label: 'Eligible' },
    ];
  }
  if (activeTab === 'audit') {
    return [
      { value: 'recent', label: 'Recent' },
      { value: 'preview', label: 'Preview' },
      { value: 'changes', label: 'Changes' },
    ];
  }
  return [
    { value: 'allow', label: 'Allow' },
    { value: 'warn', label: 'Warn' },
    { value: 'ask', label: 'Ask' },
    { value: 'limit', label: 'Limit' },
    { value: 'block', label: 'Block' },
  ];
}

function managePolicySecondaryChoiceTitle(activeTab: string): string {
  if (activeTab === 'schedule') return 'Action';
  if (activeTab === 'budget') return 'Counts';
  if (activeTab === 'approvals') return 'Expiry';
  if (activeTab === 'enforcement') return 'Mode';
  if (activeTab === 'audit') return 'Source';
  return 'Posture';
}

function managePolicySecondaryChoiceOptions(activeTab: string): readonly ManageWorkspaceChoiceOption[] {
  if (activeTab === 'schedule') {
    return [
      { value: 'allow', label: 'Allow' },
      { value: 'ask', label: 'Ask' },
      { value: 'block', label: 'Block' },
    ];
  }
  if (activeTab === 'budget') {
    return [
      { value: 'all', label: 'All' },
      { value: 'managed', label: 'Managed' },
      { value: 'unmanaged', label: 'Unmanaged' },
      { value: 'selected', label: 'Selected' },
    ];
  }
  if (activeTab === 'approvals') {
    return [
      { value: 'once', label: 'Once' },
      { value: 'session', label: 'Session' },
      { value: 'schedule', label: 'Schedule' },
    ];
  }
  if (activeTab === 'audit') {
    return [
      { value: 'family', label: 'Family' },
      { value: 'device', label: 'Device' },
      { value: 'agent', label: 'Agent' },
    ];
  }
  return [
    { value: 'observe', label: 'Observe' },
    { value: 'dry-run', label: 'Dry Run' },
    { value: 'enforce', label: 'Enforce' },
  ];
}

function managePolicySettingRows(
  area: string,
  activeTab: string,
  workspaceTarget: ManageWorkspaceTarget,
  selectedDeviceLabel: string | null
): readonly ManageWorkspaceCard[] {
  const target =
    workspaceTarget === 'portal'
      ? 'Portal'
      : workspaceTarget === 'perDevice'
        ? selectedDeviceLabel
          ? `Device ${selectedDeviceLabel}`
          : 'Select device'
        : 'Family default';
  const missingDevice =
    workspaceTarget === 'perDevice' && !selectedDeviceLabel
      ? [
          {
            label: 'Device override',
            value: 'Select a device',
            body: 'Per-device policy stays disabled until the parent chooses a known portal device from the top selector.',
            tone: 'red' as Tone,
          },
        ]
      : [];

  if (activeTab === 'budget') {
    return [
      ...missingDevice,
      {
        label: 'Total cap',
        value: area === 'Browser' ? 'Managed + unmanaged' : `${area} evidence`,
        body:
          area === 'Browser'
            ? 'Overall browser time is the parent cap; managed and unmanaged browser time can each spend from it.'
            : `${area} budgets count real child-device session evidence before changing rule posture.`,
        tone: 'gold',
      },
      {
        label: 'Target caps',
        value: 'Optional override',
        body: `${area} categories can inherit the total cap or set their own daily or weekly limit.`,
        tone: 'cyan',
      },
      {
        label: 'Schedule link',
        value: 'Windows gate caps',
        body: 'Schedules decide when a cap is active; Budget decides how much time can be spent.',
        tone: 'purple',
      },
      {
        label: 'Inheritance',
        value: target,
        body: 'Family budgets apply first. Per-device rows stay disabled until a parent turns on an override.',
        tone: 'cyan',
      },
    ];
  }

  if (area === 'Browser') {
    if (activeTab === 'schedule') {
      return [
        ...missingDevice,
        {
          label: 'Target',
          value: target,
          body: 'Browser time windows are scoped to family defaults or the selected child override.',
          tone: 'cyan',
        },
        {
          label: 'School window',
          value: 'Allow managed study sites',
          body: 'School domains can be allowed during school/homework windows using typed schedules.',
          tone: 'gold',
        },
        {
          label: 'Bedtime',
          value: 'Ask or block video/social',
          body: 'Video, social, and unknown categories can switch behavior after the bedtime window starts.',
          tone: 'purple',
        },
        {
          label: 'Temporary pass',
          value: 'Parent approved',
          body: 'One-off browser exceptions expire and create audit events before child-device validation.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'approvals') {
      return [
        ...missingDevice,
        {
          label: 'Blocked site ask',
          value: 'Reason required',
          body: 'Child requests carry target, reason, evidence refs, expiry, and parent response state.',
          tone: 'gold',
        },
        {
          label: 'Unmanaged browser',
          value: 'Parent decision',
          body: 'Chrome/Edge/Firefox outside the managed boundary become bypass requests, not URL evidence.',
          tone: 'red',
        },
        {
          label: 'Download request',
          value: 'Ask parent',
          body: 'Downloads can require approval without sending raw page body, cookies, or form data.',
          tone: 'cyan',
        },
        {
          label: 'Remember answer',
          value: 'Time boxed',
          body: 'Approved browser exceptions can be once, session, schedule, or custom expiry.',
          tone: 'purple',
        },
      ];
    }
    if (activeTab === 'enforcement') {
      return [
        ...missingDevice,
        {
          label: 'Managed browser',
          value: 'Edge / Chrome first',
          body: 'Exact URL policy is enforcement-eligible only inside an Ocentra-managed browser session.',
          tone: 'cyan',
        },
        {
          label: 'Unmanaged browser',
          value: 'Monitor, warn, ask, relaunch, or block',
          body: 'The parent chooses bypass handling and the child agent reports capability before action.',
          tone: 'gold',
        },
        {
          label: 'Dry-run preview',
          value: 'Before apply',
          body: 'Browser enforcement must produce the same typed decision in preview before changing device behavior.',
          tone: 'purple',
        },
        {
          label: 'Unsupported browser',
          value: 'Honest unavailable',
          body: 'Firefox, Opera, portable browsers, and unknown forks remain unsupported until adapter proof exists.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'audit') {
      return [
        ...missingDevice,
        {
          label: 'Evidence ref',
          value: 'Managed tab only',
          body: 'Audit can cite URL/title/domain evidence only when it came from the managed browser boundary.',
          tone: 'cyan',
        },
        {
          label: 'Bypass event',
          value: 'Separate from URL capture',
          body: 'Unmanaged browser use is logged as possible bypass and does not invent exact URLs.',
          tone: 'red',
        },
        {
          label: 'Policy diff',
          value: 'Parent authored',
          body: 'Rule edits show previous version, new version, actor, and child-device validation result.',
          tone: 'gold',
        },
        {
          label: 'Adapter result',
          value: 'Applied / unavailable / monitor-only',
          body: 'Enforcement outcomes are journaled by the child-device agent, not the portal UI.',
          tone: 'purple',
        },
      ];
    }
    return [
      ...missingDevice,
      {
        label: 'Managed boundary',
        value: 'Exact URL evidence',
        body: 'Edge/Chrome managed sessions can produce URL, title, domain, source id, and freshness state.',
        tone: 'cyan',
      },
      {
        label: 'Decision ladder',
        value: 'Allow / Warn / Ask / Limit / Block',
        body: 'Rules operate on site, domain, category, video/channel, browser process, or session targets.',
        tone: 'gold',
      },
      {
        label: 'Unmanaged browsers',
        value: 'Bypass state',
        body: 'Detected personal browsers are possible bypass evidence until a supported managed adapter proves URL capture.',
        tone: 'red',
      },
      {
        label: 'Explanation',
        value: 'Parent text',
        body: 'Warn/ask/block states carry parent-authored reason text and typed reason codes.',
        tone: 'purple',
      },
    ];
  }

  if (area === 'Apps') {
    if (activeTab === 'schedule') {
      return [
        ...missingDevice,
        {
          label: 'School apps',
          value: 'Allow window',
          body: 'Productivity and school app categories can be allowed during homework/school windows.',
          tone: 'cyan',
        },
        {
          label: 'Blocked window',
          value: 'Bedtime / custom',
          body: 'Chat/media app behavior can change by local child-device schedule and parent rule version.',
          tone: 'purple',
        },
        {
          label: 'Foreground budget',
          value: 'Daily / weekly',
          body: 'App budgets use session summaries from stored process/window evidence.',
          tone: 'gold',
        },
        {
          label: 'Grace period',
          value: 'Typed timer',
          body: 'Grace, expiry, and reset behavior belong to the child-device timer path.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'approvals') {
      return [
        ...missingDevice,
        {
          label: 'New app',
          value: 'Ask until reviewed',
          body: 'Unknown new apps can ask parent first without pretending classification is known.',
          tone: 'gold',
        },
        {
          label: 'Blocked app open',
          value: 'Reason required',
          body: 'Child requests carry app/process evidence refs and parent response state.',
          tone: 'cyan',
        },
        {
          label: 'Extend budget',
          value: 'Time boxed',
          body: 'Extra app time creates an expiry and audit record.',
          tone: 'purple',
        },
        {
          label: 'Install/update',
          value: 'Parent review',
          body: 'Installer or app-update requests stay typed intents, not portal-side execution.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'enforcement') {
      return [
        ...missingDevice,
        {
          label: 'Process action',
          value: 'Observe / terminate / block',
          body: 'App enforcement is capability-gated by platform adapter and stored process evidence.',
          tone: 'red',
        },
        {
          label: 'Foreground watcher',
          value: 'Required',
          body: 'Time limits need foreground/running state from child-device evidence, not portal timers.',
          tone: 'cyan',
        },
        {
          label: 'Unknown app',
          value: 'Ask or observe',
          body: 'Unknown stays unknown until evidence or catalog proof supports a stronger claim.',
          tone: 'gold',
        },
        {
          label: 'Rollback',
          value: 'Adapter reported',
          body: 'Failed, partial, unavailable, already exited, and rollback states are parent-visible.',
          tone: 'purple',
        },
      ];
    }
    if (activeTab === 'audit') {
      return [
        ...missingDevice,
        {
          label: 'Inventory diff',
          value: 'Installed / removed',
          body: 'App inventory changes show source, signature/hash where available, and last probe time.',
          tone: 'cyan',
        },
        {
          label: 'Session result',
          value: 'Duration refs',
          body: 'Audit cites running/foreground summaries and evidence ids, not portal guesses.',
          tone: 'gold',
        },
        {
          label: 'Parent change',
          value: 'Versioned',
          body: 'App rules, budgets, and approval settings produce policy version events.',
          tone: 'purple',
        },
        {
          label: 'Adapter state',
          value: 'Ready / unavailable / unsupported',
          body: 'Policy screens stay honest when app control capability is missing.',
          tone: 'red',
        },
      ];
    }
    return [
      ...missingDevice,
      {
        label: 'Inventory',
        value: 'Installed and running apps',
        body: 'Process, path/signature/hash, foreground state, and category candidates come from child-device evidence.',
        tone: 'cyan',
      },
      {
        label: 'Decision ladder',
        value: 'Allow / Warn / Ask / Limit / Block',
        body: 'Targets include app, process, window, category, and activity type.',
        tone: 'gold',
      },
      {
        label: 'Unknown apps',
        value: 'Ask by default',
        body: 'Unknown or suspicious apps can ask parent until a catalog or evidence rule classifies them.',
        tone: 'purple',
      },
      {
        label: 'School allowlist',
        value: 'Schedule aware',
        body: 'School and homework apps can be allowed while other categories stay limited.',
        tone: 'cyan',
      },
    ];
  }

  if (area === 'Games') {
    if (activeTab === 'schedule') {
      return [
        ...missingDevice,
        {
          label: 'School day',
          value: 'Strict budget',
          body: 'Game rules can use school-day budgets separate from weekend rules.',
          tone: 'gold',
        },
        {
          label: 'Weekend',
          value: 'Larger budget',
          body: 'Weekly or weekend reset behavior is explicit and journaled.',
          tone: 'cyan',
        },
        {
          label: 'Bedtime',
          value: 'Block or ask',
          body: 'Late game launches can ask parent or block based on household rule.',
          tone: 'purple',
        },
        {
          label: 'Temporary pass',
          value: 'Expires',
          body: 'Extra game time is time-boxed and auditable.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'approvals') {
      return [
        ...missingDevice,
        {
          label: 'Start blocked game',
          value: 'Ask parent',
          body: 'Requests carry launcher/game/session evidence and the child reason.',
          tone: 'gold',
        },
        {
          label: 'Extend play',
          value: 'Time boxed',
          body: 'Parent can approve one game session without changing the family default.',
          tone: 'cyan',
        },
        {
          label: 'Unknown game',
          value: 'Review first',
          body: 'Unknown or ambiguous game candidates ask instead of claiming a game title.',
          tone: 'purple',
        },
        {
          label: 'Voice/chat risk',
          value: 'Explicit rule',
          body: 'Voice/chat game risks require parent-authored rules and evidence refs.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'enforcement') {
      return [
        ...missingDevice,
        {
          label: 'Launcher action',
          value: 'Observe / block',
          body: 'Steam, Epic, Xbox, Riot, Battle.net, EA, Ubisoft, GOG, Roblox, and Minecraft need adapter proof.',
          tone: 'cyan',
        },
        {
          label: 'Game process',
          value: 'Terminate or leave running',
          body: 'The child-device adapter reports stopped, already exited, failed, unavailable, or observe-only.',
          tone: 'red',
        },
        {
          label: 'Time limit',
          value: 'Timer-backed',
          body: 'Running/foreground duration comes from stored session summaries before enforcement.',
          tone: 'gold',
        },
        {
          label: 'Child explanation',
          value: 'Parent rule',
          body: 'Stopped by parent policy / ask parent / time limit reached must be stable text refs.',
          tone: 'purple',
        },
      ];
    }
    if (activeTab === 'audit') {
      return [
        ...missingDevice,
        {
          label: 'Catalog match',
          value: 'Known / possible / unknown',
          body: 'Game classification stays evidence-backed and shows ambiguity.',
          tone: 'cyan',
        },
        {
          label: 'Session summary',
          value: 'Running + foreground',
          body: 'Audit shows duration, run count, launcher, rule, and result refs.',
          tone: 'gold',
        },
        {
          label: 'Approval trail',
          value: 'Request and response',
          body: 'Game asks record expiry and parent response state.',
          tone: 'purple',
        },
        {
          label: 'Enforcement result',
          value: 'Adapter journal',
          body: 'Terminate/block results are stored by the child-device agent.',
          tone: 'red',
        },
      ];
    }
    return [
      ...missingDevice,
      {
        label: 'Game identity',
        value: 'Launcher + process + catalog',
        body: 'Rules distinguish launcher-only activity from an actual native game session.',
        tone: 'cyan',
      },
      {
        label: 'Decision ladder',
        value: 'Allow / Warn / Ask / Limit / Block',
        body: 'Targets include game title, launcher, process, category, and unknown game.',
        tone: 'gold',
      },
      {
        label: 'Time budgets',
        value: 'Daily / weekly',
        body: 'Game budgets rely on stored session summaries before dry-run or enforcement.',
        tone: 'purple',
      },
      {
        label: 'Unknown game',
        value: 'Ask parent',
        body: 'Unknown possible-game evidence asks rather than silently allowing or blocking.',
        tone: 'red',
      },
    ];
  }

  if (area === 'Screen') {
    if (activeTab === 'schedule') {
      return [
        ...missingDevice,
        {
          label: 'Capture windows',
          value: 'Parent controlled',
          body: 'Screen analysis runs only inside parent-selected windows/triggers.',
          tone: 'cyan',
        },
        {
          label: 'Cadence',
          value: '5 min / 1 min strict / trigger',
          body: 'Cadence and strict mode are explicit settings and not hidden defaults.',
          tone: 'gold',
        },
        {
          label: 'Protected surfaces',
          value: 'Unavailable state',
          body: 'Lock screen, secure desktop, password prompts, and protected content produce typed unavailable state.',
          tone: 'red',
        },
        {
          label: 'TTL',
          value: 'Short queue',
          body: 'Temporary images expire if analysis cannot complete.',
          tone: 'purple',
        },
      ];
    }
    if (activeTab === 'approvals') {
      return [
        ...missingDevice,
        {
          label: 'One-time capture',
          value: 'Ask parent',
          body: 'Manual diagnostic capture requires explicit parent action and audit.',
          tone: 'gold',
        },
        {
          label: 'Live view',
          value: 'Permission required',
          body: 'Live viewing is not silently enabled from Activity/Policy screens.',
          tone: 'red',
        },
        {
          label: 'Retain image',
          value: 'Off by default',
          body: 'Raw image retention needs a separate future parent-approved feature.',
          tone: 'purple',
        },
        {
          label: 'Reason',
          value: 'Required',
          body: 'Screen-derived asks cite summary refs, category, confidence, and deletion state.',
          tone: 'cyan',
        },
      ];
    }
    if (activeTab === 'enforcement') {
      return [
        ...missingDevice,
        {
          label: 'Policy use',
          value: 'Summary only',
          body: 'Policy consumes schema-valid screen summaries, not raw screenshots or untyped AI text.',
          tone: 'cyan',
        },
        {
          label: 'Low confidence',
          value: 'No-op / warn / ask',
          body: 'Low-confidence categories degrade according to parent rules.',
          tone: 'gold',
        },
        {
          label: 'Delete after analysis',
          value: 'Required default',
          body: 'Temporary image deletion status remains parent-visible.',
          tone: 'purple',
        },
        {
          label: 'Unsupported',
          value: 'Honest unavailable',
          body: 'No enforcement is claimed when screen capture/model capability is missing.',
          tone: 'red',
        },
      ];
    }
    if (activeTab === 'audit') {
      return [
        ...missingDevice,
        {
          label: 'Summary ref',
          value: 'No raw image',
          body: 'Audit shows summary id, category, confidence, evidence refs, and deletion result.',
          tone: 'cyan',
        },
        {
          label: 'Queue state',
          value: 'Processed / failed / expired',
          body: 'Screen queue failures are visible instead of silently retaining images.',
          tone: 'gold',
        },
        {
          label: 'Parent setting',
          value: 'Versioned',
          body: 'Enablement, cadence, triggers, OCR, and retention changes are versioned.',
          tone: 'purple',
        },
        {
          label: 'AI result',
          value: 'Schema-valid only',
          body: 'Invalid local model output cannot drive policy.',
          tone: 'red',
        },
      ];
    }
    return [
      ...missingDevice,
      {
        label: 'Enablement',
        value: 'Explicit parent opt-in',
        body: 'Screen analysis is local-first and parent-controlled; the UI does not silently decide capture.',
        tone: 'cyan',
      },
      {
        label: 'Policy signals',
        value: 'Category + confidence',
        body: 'Visible categories and risk signals can inform rules only as typed summaries.',
        tone: 'gold',
      },
      {
        label: 'Retention',
        value: 'Delete raw image',
        body: 'Temporary images are encrypted while queued and deleted after successful analysis by default.',
        tone: 'purple',
      },
      {
        label: 'Disclosure',
        value: 'Required',
        body: 'Policy must be honest about local screen analysis, unavailable states, and deletion failures.',
        tone: 'red',
      },
    ];
  }

  if (activeTab === 'schedule') {
    return [
      ...missingDevice,
      {
        label: 'School mode',
        value: 'Domain/process rules',
        body: 'Network rules can shift by school, bedtime, weekend, and temporary exception windows.',
        tone: 'cyan',
      },
      {
        label: 'VPN/proxy window',
        value: 'Ask or block',
        body: 'VPN/proxy/tunnel indicators can have schedule-specific behavior.',
        tone: 'gold',
      },
      {
        label: 'High volume',
        value: 'Budget-like digest',
        body: 'Bandwidth-heavy flows can warn or ask based on summary evidence.',
        tone: 'purple',
      },
      {
        label: 'Exception expiry',
        value: 'Typed timer',
        body: 'Network exceptions expire and journal the parent action.',
        tone: 'cyan',
      },
    ];
  }
  if (activeTab === 'approvals') {
    return [
      ...missingDevice,
      {
        label: 'Blocked endpoint',
        value: 'Ask parent',
        body: 'Domain/IP/process requests carry evidence refs and expiry.',
        tone: 'gold',
      },
      {
        label: 'VPN exception',
        value: 'Parent choice',
        body: 'VPN/proxy exceptions can be one-time, session, schedule, or custom.',
        tone: 'red',
      },
      {
        label: 'New destination',
        value: 'Review',
        body: 'Unknown destinations are shown as metadata, not exact page content.',
        tone: 'cyan',
      },
      {
        label: 'Reason',
        value: 'Child note optional',
        body: 'Network asks can collect a child reason without packet payloads.',
        tone: 'purple',
      },
    ];
  }
  if (activeTab === 'enforcement') {
    return [
      ...missingDevice,
      {
        label: 'DNS/domain block',
        value: 'Capability-gated',
        body: 'Domain blocking acts only after typed policy decisions and adapter support.',
        tone: 'cyan',
      },
      {
        label: 'Process flow',
        value: 'Metadata only',
        body: 'Network enforcement references flow summaries, not decrypted payloads.',
        tone: 'gold',
      },
      {
        label: 'VPN/proxy',
        value: 'Ask / block / observe',
        body: 'Tunnel indicators are policy targets when evidence is available.',
        tone: 'purple',
      },
      {
        label: 'Unavailable adapter',
        value: 'No adapter success claim',
        body: 'Adapter-unavailable states remain visible and cannot be hidden by portal UI.',
        tone: 'red',
      },
    ];
  }
  if (activeTab === 'audit') {
    return [
      ...missingDevice,
      {
        label: 'Flow summary',
        value: 'Process/domain/IP refs',
        body: 'Audit cites top destination, protocol, bytes/counts where supported, and evidence id.',
        tone: 'cyan',
      },
      {
        label: 'Encrypted content',
        value: 'Unavailable',
        body: 'The UI must not imply decrypted HTTPS payloads or page URLs from flow metadata.',
        tone: 'red',
      },
      {
        label: 'Policy decision',
        value: 'Rule + evidence refs',
        body: 'Network policy decisions reference stored evidence and parent rule ids.',
        tone: 'gold',
      },
      {
        label: 'Adapter result',
        value: 'Applied / failed / observe-only',
        body: 'Network enforcement outcomes are journaled by the child-device agent.',
        tone: 'purple',
      },
    ];
  }
  return [
    ...missingDevice,
    {
      label: 'Network metadata',
      value: 'Process / domain / IP / protocol',
      body: 'Rules consume flow summaries and destination metadata, not exact browser URLs or payloads.',
      tone: 'cyan',
    },
    {
      label: 'Decision ladder',
      value: 'Allow / Warn / Ask / Block',
      body: 'Targets include process, domain, IP, protocol, category, VPN/proxy, and unusual digest.',
      tone: 'gold',
    },
    {
      label: 'Bypass indicators',
      value: 'VPN / proxy / tunnel',
      body: 'Bypass-like flows are typed indicators and can trigger parent rules.',
      tone: 'purple',
    },
    {
      label: 'Unknown traffic',
      value: 'Ask or observe',
      body: 'Unknown/IP-only/encrypted states remain honest until evidence supports stronger claims.',
      tone: 'red',
    },
  ];
}

function ManageSupportContactForm({
  x,
  y,
  w,
  h,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  cfg: ParentPortalSvgControls;
}) {
  const color = toneColor('purple', cfg);
  const cyan = toneColor('cyan', cfg);
  const gold = toneColor('gold', cfg);
  const pad = 18;
  const conversationH = Math.max(108, Math.min(152, h * 0.28));
  const composerY = y + conversationH + 18;
  const composerH = Math.max(150, h - conversationH - 76);
  const buttonY = y + h - 42;
  const actionW = 154;
  const draftX = x + w - pad - actionW * 2 - 12;
  const sendX = x + w - pad - actionW;
  const fieldW = Math.max(160, (w - pad * 2 - 16) / 3);
  const fieldY = composerY + 42;
  const fieldH = 34;
  const messageY = fieldY + fieldH + 13;
  const messageH = Math.max(74, composerY + composerH - messageY - 16);
  const fields = [
    { label: 'CATEGORY', value: 'Unavailable' },
    { label: 'REPLY EMAIL', value: 'Unavailable' },
    { label: 'SUBJECT', value: 'Unavailable' },
  ] as const;
  return (
    <g role="group" aria-label="Support connector unavailable" aria-disabled="true">
      <path
        d={cutRectPath(x, y, w, h, 10)}
        fill="rgba(3, 17, 31, 0.72)"
        stroke={color}
        strokeWidth={1.05}
        opacity={0.98}
      />
      <path d={`M ${x + 16} ${y + 42} H ${x + w - 16}`} stroke={color} strokeWidth={0.9} opacity={0.46} />
      <text x={x + pad} y={y + 27} fontSize={16} fontWeight={950} fill={cfg.colors.bodyText}>
        Support / Contact
      </text>
      <text x={x + w - pad} y={y + 27} textAnchor="end" fontSize={10.5} fontWeight={860} fill={cfg.colors.mutedText}>
        Read-only preview
      </text>
      <rect
        x={x + pad}
        y={y + 56}
        width={w - pad * 2}
        height={conversationH - 58}
        rx={8}
        fill="rgba(6, 28, 45, 0.64)"
        stroke={cyan}
        strokeWidth={0.78}
        opacity={0.88}
      />
      <text x={x + pad + 16} y={y + 82} fontSize={11.2} fontWeight={940} fill={cyan}>
        SUPPORT CONNECTOR UNAVAILABLE
      </text>
      <text x={x + pad + 16} y={y + 104} fontSize={12.5} fontWeight={780} fill={cfg.colors.bodyText}>
        No authenticated support connector or parent-owned draft store is mounted for this route.
      </text>
      <text x={x + pad + 16} y={y + 126} fontSize={11.5} fontWeight={720} fill={cfg.colors.mutedText}>
        Contact fields stay unavailable until the Account owner supplies both capabilities.
      </text>

      <path
        d={cutRectPath(x + pad, composerY, w - pad * 2, composerH, 8)}
        fill={PARENT_PORTAL_GLASS.controlFill}
        stroke={cyan}
        strokeWidth={0.86}
        opacity={0.94}
      />
      <text x={x + pad + 16} y={composerY + 25} fontSize={11.2} fontWeight={950} fill={gold}>
        CONTACT PREVIEW
      </text>
      {fields.map((field, index) => {
        const fieldX = x + pad + index * (fieldW + 8);
        return (
          <g key={`support-contact-field:${field.label}`}>
            <rect
              x={fieldX}
              y={fieldY}
              width={fieldW}
              height={fieldH}
              rx={4}
              fill={colorAlpha(cyan, '13')}
              stroke={cyan}
              strokeWidth={0.72}
              opacity={0.92}
            />
            <text x={fieldX + 10} y={fieldY + 13} fontSize={8.8} fontWeight={950} fill={cyan}>
              {field.label}
            </text>
            <text x={fieldX + 10} y={fieldY + 27} fontSize={10.7} fontWeight={820} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(field.value, fieldW - 20, 10.7, 0.56)}
            </text>
          </g>
        );
      })}
      <rect
        x={x + pad}
        y={messageY}
        width={w - pad * 2}
        height={messageH}
        rx={5}
        fill="rgba(2, 13, 24, 0.66)"
        stroke={color}
        strokeWidth={0.78}
        opacity={0.95}
      />
      <text x={x + pad + 12} y={messageY + 19} fontSize={9.3} fontWeight={950} fill={color}>
        MESSAGE
      </text>
      <text x={x + pad + 12} y={messageY + 43} fontSize={12.4} fontWeight={780} fill={cfg.colors.mutedText}>
        No message can be drafted or sent from this unavailable surface.
      </text>
      <path
        d={`M ${x + pad + 12} ${messageY + 61} H ${x + w - pad - 12}`}
        stroke={cfg.colors.panelStroke}
        strokeWidth={0.7}
        opacity={0.45}
      />
      <path
        d={`M ${x + pad + 12} ${messageY + 82} H ${x + w - pad - 12}`}
        stroke={cfg.colors.panelStroke}
        strokeWidth={0.7}
        opacity={0.32}
      />
      <path
        d={`M ${x + pad + 12} ${messageY + 103} H ${x + w - pad - 12}`}
        stroke={cfg.colors.panelStroke}
        strokeWidth={0.7}
        opacity={0.22}
      />
      <rect
        x={draftX}
        y={buttonY}
        width={actionW}
        height={28}
        rx={4}
        fill="rgba(3, 18, 32, 0.72)"
        stroke={gold}
        strokeWidth={0.9}
      />
      <text x={draftX + actionW / 2} y={buttonY + 18} textAnchor="middle" fontSize={11.2} fontWeight={950} fill={gold}>
        DRAFT UNAVAILABLE
      </text>
      <rect
        x={sendX}
        y={buttonY}
        width={actionW}
        height={28}
        rx={4}
        fill={colorAlpha(cyan, '32')}
        stroke={cyan}
        strokeWidth={1}
        opacity={0.5}
      />
      <text
        x={sendX + actionW / 2}
        y={buttonY + 18}
        textAnchor="middle"
        fontSize={11.2}
        fontWeight={950}
        fill={cfg.colors.bodyText}
      >
        SEND UNAVAILABLE
      </text>
    </g>
  );
}

type PolicyFirstPassRuleQuestionDefinition = {
  readonly id: string;
  readonly title: string;
  readonly compactTitle: string;
  readonly selectionMode: 'single' | 'multi';
  readonly options: readonly BrowserRulesChoiceOption[];
};

const POLICY_FIRST_PASS_COMMON_OPTIONS = {
  active: [
    { value: 'off', label: 'Off' },
    { value: 'on', label: 'On' },
    { value: 'paused', label: 'Paused' },
    { value: 'emergency-allow', label: 'Emergency allow' },
    { value: 'emergency-block', label: 'Emergency block' },
  ],
  behavior: [
    { value: 'observe', label: 'Observe' },
    { value: 'warn', label: 'Warn' },
    { value: 'ask-parent', label: 'Ask parent' },
    { value: 'limit', label: 'Limit' },
    { value: 'block', label: 'Block' },
  ],
  fallback: [
    { value: 'allow-report', label: 'Allow + report' },
    { value: 'observe-only', label: 'Observe only' },
    { value: 'warn-child', label: 'Warn child' },
    { value: 'ask-parent', label: 'Ask parent' },
    { value: 'block', label: 'Block' },
    { value: 'unavailable', label: 'Show unavailable' },
  ],
} as const;

const POLICY_FIRST_PASS_AREA_TARGETS = {
  Rules: [
    { value: 'apps', label: 'Apps' },
    { value: 'games', label: 'Games' },
    { value: 'browser', label: 'Browser' },
    { value: 'screen', label: 'Screen' },
    { value: 'network', label: 'Network' },
    { value: 'tracking', label: 'Tracking' },
  ],
  Browser: [
    { value: 'site-domain', label: 'Site domain' },
    { value: 'page-title', label: 'Page title' },
    { value: 'navigation-route', label: 'Navigation route' },
    { value: 'tab-session', label: 'Tab/session' },
    { value: 'download-target', label: 'Download target' },
    { value: 'unknown-site', label: 'Unknown site' },
  ],
  Apps: [
    { value: 'installed-apps', label: 'Installed apps' },
    { value: 'running-processes', label: 'Running processes' },
    { value: 'foreground-window', label: 'Foreground window' },
    { value: 'app-category', label: 'App category' },
    { value: 'new-unknown-apps', label: 'New/unknown apps' },
    { value: 'installers-updaters', label: 'Install/update' },
  ],
  Games: [
    { value: 'native-games', label: 'Native games' },
    { value: 'launchers', label: 'Launchers/stores' },
    { value: 'browser-games', label: 'Browser games' },
    { value: 'cloud-games', label: 'Cloud games' },
    { value: 'unknown-candidates', label: 'Unknown candidates' },
    { value: 'ratings-categories', label: 'Ratings/categories' },
  ],
  Screen: [
    { value: 'screen-summary', label: 'Screen summaries' },
    { value: 'risk-category', label: 'Risk category' },
    { value: 'ocr-text', label: 'OCR text' },
    { value: 'active-window', label: 'Active window' },
    { value: 'manual-capture', label: 'Manual capture' },
    { value: 'protected-state', label: 'Protected state' },
  ],
  'Remote Screen': [
    { value: 'live-view-session', label: 'Live view session' },
    { value: 'parent-request', label: 'Parent request' },
    { value: 'child-device-screen', label: 'Child device screen' },
    { value: 'consent-mode', label: 'Consent mode' },
    { value: 'stop-revoke', label: 'Stop/revoke' },
    { value: 'unsupported-state', label: 'Unsupported state' },
  ],
  Network: [
    { value: 'domain', label: 'Domain' },
    { value: 'ip-address', label: 'IP address' },
    { value: 'protocol-port', label: 'Protocol/port' },
    { value: 'process-flow', label: 'Process flow' },
    { value: 'vpn-proxy', label: 'VPN/proxy' },
    { value: 'unknown-traffic', label: 'Unknown traffic' },
  ],
  Tracking: [
    { value: 'device-location', label: 'Device location' },
    { value: 'geofence', label: 'Geofence' },
    { value: 'trip-route', label: 'Trip/route' },
    { value: 'arrival-departure', label: 'Arrival/departure' },
    { value: 'offline-stale', label: 'Offline/stale' },
    { value: 'location-sharing', label: 'Location sharing' },
  ],
} as const satisfies Record<string, readonly BrowserRulesChoiceOption[]>;

const POLICY_FIRST_PASS_AREA_PROOF = {
  Rules: [
    { value: 'effective-policy', label: 'Effective policy' },
    { value: 'parent-approval', label: 'Parent approval' },
    { value: 'child-identity', label: 'Child identity' },
    { value: 'capability-state', label: 'Capability state' },
    { value: 'decision-audit', label: 'Decision audit' },
    { value: 'enforcement-result', label: 'Enforcement result' },
  ],
  Browser: [
    { value: 'url-title', label: 'URL/title' },
    { value: 'history-row', label: 'History row' },
    { value: 'page-snapshot', label: 'Page snapshot' },
    { value: 'network-trace', label: 'Network trace' },
    { value: 'block-signal', label: 'Block signal' },
  ],
  Apps: [
    { value: 'process-window', label: 'Process + window' },
    { value: 'install-records', label: 'Install records' },
    { value: 'signature-hash', label: 'Signature/hash' },
    { value: 'package-id', label: 'Package id' },
    { value: 'parent-catalog', label: 'Parent catalog' },
  ],
  Games: [
    { value: 'foreground-session', label: 'Foreground session' },
    { value: 'launcher-manifest', label: 'Launcher manifest' },
    { value: 'process-identity', label: 'Process identity' },
    { value: 'managed-browser', label: 'Managed browser' },
    { value: 'platform-family', label: 'Platform family' },
  ],
  Screen: [
    { value: 'local-summary', label: 'Local summary' },
    { value: 'confidence', label: 'Confidence' },
    { value: 'evidence-ref', label: 'Evidence ref' },
    { value: 'delete-proof', label: 'Delete proof' },
    { value: 'unavailable-state', label: 'Unavailable state' },
  ],
  'Remote Screen': [
    { value: 'session-capability', label: 'Session capability' },
    { value: 'device-agent-state', label: 'Child agent state' },
    { value: 'custody-audit', label: 'Custody/audit' },
    { value: 'permission-state', label: 'Permission state' },
    { value: 'stop-event', label: 'Stop event' },
  ],
  Network: [
    { value: 'dns-domain', label: 'DNS/domain' },
    { value: 'flow-summary', label: 'Flow summary' },
    { value: 'process-link', label: 'Process link' },
    { value: 'adapter-proof', label: 'Adapter proof' },
    { value: 'weak-signal', label: 'Weak signal' },
  ],
  Tracking: [
    { value: 'child-device-gps', label: 'Child device GPS' },
    { value: 'wifi-cell', label: 'Wi-Fi/cell' },
    { value: 'freshness', label: 'Freshness' },
    { value: 'permission-state', label: 'Permission state' },
    { value: 'battery-accuracy', label: 'Battery/accuracy' },
  ],
} as const satisfies Record<string, readonly BrowserRulesChoiceOption[]>;

type PolicyFirstPassTargetArea = keyof typeof POLICY_FIRST_PASS_AREA_TARGETS;
type PolicyFirstPassProofArea = keyof typeof POLICY_FIRST_PASS_AREA_PROOF;

function isPolicyFirstPassTargetArea(area: string): area is PolicyFirstPassTargetArea {
  return Object.prototype.hasOwnProperty.call(POLICY_FIRST_PASS_AREA_TARGETS, area);
}

function isPolicyFirstPassProofArea(area: string): area is PolicyFirstPassProofArea {
  return Object.prototype.hasOwnProperty.call(POLICY_FIRST_PASS_AREA_PROOF, area);
}

function policyFirstPassRuleQuestions(area: string): readonly PolicyFirstPassRuleQuestionDefinition[] {
  const targets = isPolicyFirstPassTargetArea(area)
    ? POLICY_FIRST_PASS_AREA_TARGETS[area]
    : POLICY_FIRST_PASS_AREA_TARGETS.Network;
  const proof = isPolicyFirstPassProofArea(area)
    ? POLICY_FIRST_PASS_AREA_PROOF[area]
    : POLICY_FIRST_PASS_AREA_PROOF.Network;
  if (area === 'Rules') {
    return [
      {
        id: '1',
        title: 'Should family rules be active?',
        compactTitle: 'Family rules active?',
        selectionMode: 'single',
        options: POLICY_FIRST_PASS_COMMON_OPTIONS.active,
      },
      {
        id: '2',
        title: 'What should family rules do?',
        compactTitle: 'Family rule actions',
        selectionMode: 'multi',
        options: POLICY_FIRST_PASS_COMMON_OPTIONS.behavior,
      },
      {
        id: '3',
        title: 'Which policy areas should family rules cover?',
        compactTitle: 'Family policy areas',
        selectionMode: 'multi',
        options: targets,
      },
      {
        id: '4',
        title: 'What proof should count for family rules?',
        compactTitle: 'Family rule evidence',
        selectionMode: 'multi',
        options: proof,
      },
      {
        id: '5',
        title: 'What if proof for family rules is weak or missing?',
        compactTitle: 'Missing-proof response',
        selectionMode: 'single',
        options: POLICY_FIRST_PASS_COMMON_OPTIONS.fallback,
      },
    ];
  }
  const areaName = area.toLowerCase();
  return [
    {
      id: '1',
      title: `Should ${areaName} policy be active?`,
      compactTitle: `${area} policy active?`,
      selectionMode: 'single',
      options: POLICY_FIRST_PASS_COMMON_OPTIONS.active,
    },
    {
      id: '2',
      title: `What should controlled ${areaName} activity do?`,
      compactTitle: `${area} policy actions`,
      selectionMode: 'multi',
      options: POLICY_FIRST_PASS_COMMON_OPTIONS.behavior,
    },
    {
      id: '3',
      title: `What should ${areaName} rules target?`,
      compactTitle: `${area} targets`,
      selectionMode: 'multi',
      options: targets,
    },
    {
      id: '4',
      title: `What proof should count for ${areaName}?`,
      compactTitle: `${area} evidence`,
      selectionMode: 'multi',
      options: proof,
    },
    {
      id: '5',
      title: `What if ${areaName} proof is weak or missing?`,
      compactTitle: 'Missing-proof response',
      selectionMode: 'single',
      options: POLICY_FIRST_PASS_COMMON_OPTIONS.fallback,
    },
  ];
}

function PolicyRulesGridGuide({
  policyAreaLabel,
  x,
  y,
  w,
  h,
  disabled,
  enforcementChoice,
  onEnforcementChange,
  onInfoClick,
}: {
  policyAreaLabel: string;
  x: number;
  y: number;
  w: number;
  h: number;
  disabled?: boolean;
  enforcementChoice: string;
  onEnforcementChange: (value: string) => void;
  onInfoClick?: () => void;
}) {
  if (policyAreaLabel === 'Browser') {
    return (
      <BrowserRulesGridGuide
        x={x}
        y={y}
        w={w}
        h={h}
        {...(disabled === undefined ? {} : { disabled })}
        enforcementChoice={enforcementChoice}
        onEnforcementChange={onEnforcementChange}
        {...(onInfoClick === undefined ? {} : { onInfoClick })}
      />
    );
  }

  return (
    <GenericPolicyRulesGridGuide
      policyAreaLabel={policyAreaLabel}
      x={x}
      y={y}
      w={w}
      h={h}
      {...(disabled === undefined ? {} : { disabled })}
      enforcementChoice={enforcementChoice}
      onEnforcementChange={onEnforcementChange}
      {...(onInfoClick === undefined ? {} : { onInfoClick })}
    />
  );
}

function GenericPolicyRulesGridGuide({
  policyAreaLabel,
  x,
  y,
  w,
  h,
  disabled,
  enforcementChoice,
  onEnforcementChange,
  onInfoClick,
}: {
  policyAreaLabel: string;
  x: number;
  y: number;
  w: number;
  h: number;
  disabled?: boolean;
  enforcementChoice: string;
  onEnforcementChange: (value: string) => void;
  onInfoClick?: () => void;
}) {
  const questionDefinitions = useMemo(() => policyFirstPassRuleQuestions(policyAreaLabel), [policyAreaLabel]);
  const [collapsedBubbleIds, setCollapsedBubbleIds] = useState<readonly string[]>([]);
  const [bubbleEnforcementChoices, setBubbleEnforcementChoices] = useState<Record<string, string>>({});
  const [singleChoices, setSingleChoices] = useState<Record<string, string>>({ 1: 'on', 5: 'ask-parent' });
  const [multiSelections, setMultiSelections] = useState<Record<string, readonly string[]>>({
    2: ['observe', 'ask-parent', 'limit', 'block'],
    3: questionDefinitions[2]?.options.slice(0, 3).map((option) => option.value) ?? [],
    4: questionDefinitions[3]?.options.slice(0, 2).map((option) => option.value) ?? [],
  });

  useEffect(() => {
    setCollapsedBubbleIds([]);
    setBubbleEnforcementChoices({});
    setSingleChoices({ 1: 'on', 5: 'ask-parent' });
    setMultiSelections({
      2: ['observe', 'ask-parent', 'limit', 'block'],
      3: questionDefinitions[2]?.options.slice(0, 3).map((option) => option.value) ?? [],
      4: questionDefinitions[3]?.options.slice(0, 2).map((option) => option.value) ?? [],
    });
  }, [policyAreaLabel, questionDefinitions]);

  const policyOff = singleChoices[1] === 'off';
  const questions: readonly BrowserRulesQuestion[] = questionDefinitions.map((question) => {
    const multiSelect = question.selectionMode === 'multi';
    const selected = multiSelect ? (multiSelections[question.id] ?? []) : [singleChoices[question.id] ?? ''];
    const questionDisabled = disabled === true || (policyOff && question.id !== '1');
    return {
      id: question.id,
      header: `${question.id}. ${question.title}`,
      compactHeader: `${question.id}. ${question.compactTitle}`,
      title: question.id,
      kind: 'multi',
      disabled: questionDisabled,
      enforcementDisabled: question.id === '1' ? policyOff : questionDisabled,
      multiSelect,
      selected,
      options: question.options,
      collapsed: collapsedBubbleIds.includes(question.id),
      onCollapsedChange: (nextCollapsed) => {
        setCollapsedBubbleIds((current) => {
          const hasId = current.includes(question.id);
          if (nextCollapsed && !hasId) return [...current, question.id];
          if (!nextCollapsed && hasId) return current.filter((currentId) => currentId !== question.id);
          return current;
        });
      },
      onMultiChange: (nextSelected) => {
        if (multiSelect) {
          setMultiSelections((current) => ({ ...current, [question.id]: nextSelected }));
          return;
        }
        setSingleChoices((current) => ({ ...current, [question.id]: nextSelected[0] ?? '' }));
        if (question.id === '1') {
          setCollapsedBubbleIds([]);
        }
      },
      enforcementValue:
        question.id === '1' && policyOff ? 'observe' : (bubbleEnforcementChoices[question.id] ?? enforcementChoice),
      onEnforcementChange: (nextValue) => {
        setBubbleEnforcementChoices((current) => ({ ...current, [question.id]: nextValue }));
        onEnforcementChange(nextValue);
      },
    };
  });

  return (
    <foreignObject x={x} y={y} width={w} height={h}>
      <div style={{ width: w, height: h }}>
        <BrowserRulesQuestionnaire
          availableWidth={w}
          guideLabel={
            policyAreaLabel === 'Rules'
              ? 'Open family rules guide'
              : `Open ${policyAreaLabel.toLowerCase()} rules guide`
          }
          {...(disabled === undefined ? {} : { disabled })}
          {...(onInfoClick === undefined ? {} : { onInfoClick })}
          questions={questions}
        />
      </div>
    </foreignObject>
  );
}

function BrowserRulesGridGuide({
  x,
  y,
  w,
  h,
  disabled,
  enforcementChoice,
  onEnforcementChange,
  onInfoClick,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  disabled?: boolean;
  enforcementChoice: string;
  onEnforcementChange: (value: string) => void;
  onInfoClick?: () => void;
}) {
  const questionDefinitions = useMemo(() => policyFirstPassRuleQuestions('Browser'), []);
  const [collapsedBubbleIds, setCollapsedBubbleIds] = useState<readonly string[]>([]);
  const [bubbleEnforcementChoices, setBubbleEnforcementChoices] = useState<Record<string, string>>({});
  const [singleChoices, setSingleChoices] = useState<Record<string, string>>({ 1: 'on', 5: 'ask-parent' });
  const [multiSelections, setMultiSelections] = useState<Record<string, readonly string[]>>({
    2: ['observe', 'ask-parent', 'limit', 'block'],
    3: questionDefinitions[2]?.options.slice(0, 3).map((option) => option.value) ?? [],
    4: questionDefinitions[3]?.options.slice(0, 2).map((option) => option.value) ?? [],
  });

  useEffect(() => {
    setCollapsedBubbleIds([]);
    setBubbleEnforcementChoices({});
    setSingleChoices({ 1: 'on', 5: 'ask-parent' });
    setMultiSelections({
      2: ['observe', 'ask-parent', 'limit', 'block'],
      3: questionDefinitions[2]?.options.slice(0, 3).map((option) => option.value) ?? [],
      4: questionDefinitions[3]?.options.slice(0, 2).map((option) => option.value) ?? [],
    });
  }, [questionDefinitions]);

  const policyOff = singleChoices[1] === 'off';
  const questions: readonly BrowserRulesQuestion[] = questionDefinitions.map((question) => {
    const multiSelect = question.selectionMode === 'multi';
    const selected = multiSelect ? (multiSelections[question.id] ?? []) : [singleChoices[question.id] ?? ''];
    const questionDisabled = disabled === true || (policyOff && question.id !== '1');
    return {
      id: question.id,
      header: `${question.id}. ${question.title}`,
      compactHeader: `${question.id}. ${question.compactTitle}`,
      title: question.id,
      kind: 'multi',
      disabled: questionDisabled,
      enforcementDisabled: question.id === '1' ? policyOff : questionDisabled,
      multiSelect,
      selected,
      options: question.options,
      collapsed: collapsedBubbleIds.includes(question.id),
      onCollapsedChange: (nextCollapsed) => {
        setCollapsedBubbleIds((current) => {
          const hasId = current.includes(question.id);
          if (nextCollapsed && !hasId) return [...current, question.id];
          if (!nextCollapsed && hasId) return current.filter((currentId) => currentId !== question.id);
          return current;
        });
      },
      onMultiChange: (nextSelected) => {
        if (multiSelect) {
          setMultiSelections((current) => ({ ...current, [question.id]: nextSelected }));
          return;
        }
        setSingleChoices((current) => ({ ...current, [question.id]: nextSelected[0] ?? '' }));
        if (question.id === '1') {
          setCollapsedBubbleIds([]);
        }
      },
      enforcementValue:
        question.id === '1' && policyOff ? 'observe' : (bubbleEnforcementChoices[question.id] ?? enforcementChoice),
      onEnforcementChange: (nextValue) => {
        setBubbleEnforcementChoices((current) => ({ ...current, [question.id]: nextValue }));
        onEnforcementChange(nextValue);
      },
    };
  });

  return (
    <foreignObject x={x} y={y} width={w} height={h}>
      <div style={{ width: w, height: h }}>
        <BrowserRulesQuestionnaire
          availableWidth={w}
          guideLabel="Open browser rules guide"
          {...(disabled === undefined ? {} : { disabled })}
          {...(onInfoClick === undefined ? {} : { onInfoClick })}
          questions={questions}
        />
      </div>
    </foreignObject>
  );
}

const BROWSER_POLICY_APPROVAL_ROWS = [
  ['Unknown site', 'Ask', 'Yes', 'Deny', 'Once', 'Full'],
  ['Blocked site', 'Ask', 'Yes', 'Wait', 'Session', 'Full'],
  ['New domain', 'Ask', 'Yes', 'Observe', 'Today', 'Summary'],
  ['Unmanaged browser', 'Ask', 'Yes', 'Deny', 'Schedule', 'Full'],
  ['Download', 'Ask', 'Risky', 'Deny', 'Once', 'File meta'],
  ['Time extension', 'Ask', 'Yes', 'Deny', 'Custom', 'Audit'],
  ['Emergency unlock', 'Parent', 'Always', 'Deny', 'Custom', 'Full'],
  ['Setup repair', 'Parent', 'Yes', 'Wait', 'Session', 'Setup'],
] as const;
const POLICY_APPROVAL_ROWS_BY_AREA = {
  Apps: [
    ['Unknown app', 'Ask', 'Yes', 'Deny', 'Once', 'Full'],
    ['Blocked app', 'Ask', 'Yes', 'Wait', 'Session', 'Full'],
    ['New install', 'Ask', 'Yes', 'Deny', 'Today', 'Summary'],
    ['App update', 'Ask', 'Risky', 'Observe', 'Session', 'Setup'],
    ['Time extension', 'Ask', 'Yes', 'Deny', 'Custom', 'Audit'],
    ['Setup repair', 'Parent', 'Yes', 'Wait', 'Session', 'Setup'],
  ],
  Games: [
    ['Unknown game', 'Ask', 'Yes', 'Deny', 'Once', 'Full'],
    ['Blocked game', 'Ask', 'Yes', 'Wait', 'Session', 'Full'],
    ['Extra play', 'Ask', 'Yes', 'Deny', 'Custom', 'Audit'],
    ['Launcher issue', 'Parent', 'Yes', 'Observe', 'Session', 'Setup'],
    ['Cloud game', 'Ask', 'Risky', 'Deny', 'Today', 'Summary'],
    ['Emergency unlock', 'Parent', 'Always', 'Deny', 'Custom', 'Full'],
  ],
  Screen: [
    ['Manual capture', 'Parent', 'Always', 'Deny', 'Once', 'Full'],
    ['Risk summary', 'Ask', 'Yes', 'Observe', 'Session', 'Summary'],
    ['Live view', 'Parent', 'Always', 'Deny', 'Custom', 'Full'],
    ['Retain image', 'Parent', 'Always', 'Deny', 'Custom', 'Audit'],
    ['Low confidence', 'Ask', 'Yes', 'Observe', 'Once', 'Summary'],
    ['Setup repair', 'Parent', 'Yes', 'Wait', 'Session', 'Setup'],
  ],
  Network: [
    ['Unknown domain', 'Ask', 'Yes', 'Observe', 'Once', 'Summary'],
    ['Blocked endpoint', 'Ask', 'Yes', 'Deny', 'Session', 'Full'],
    ['VPN/proxy', 'Ask', 'Yes', 'Deny', 'Today', 'Full'],
    ['New process flow', 'Ask', 'Risky', 'Observe', 'Session', 'Summary'],
    ['High volume', 'Ask', 'Yes', 'Observe', 'Custom', 'Audit'],
    ['Adapter repair', 'Parent', 'Yes', 'Wait', 'Session', 'Setup'],
  ],
  Tracking: [
    ['Location share', 'Parent', 'Always', 'Deny', 'Custom', 'Full'],
    ['Geofence change', 'Parent', 'Yes', 'Wait', 'Today', 'Audit'],
    ['Arrival alert', 'Ask', 'Yes', 'Observe', 'Schedule', 'Summary'],
    ['Route history', 'Parent', 'Always', 'Deny', 'Custom', 'Full'],
    ['Offline stale', 'Notify', 'Yes', 'Observe', 'Session', 'Summary'],
    ['Permission repair', 'Parent', 'Yes', 'Wait', 'Session', 'Setup'],
  ],
} as const;
const BROWSER_POLICY_APPROVAL_COLUMNS = ['Request', 'Allowed', 'Notify', 'No answer', 'Duration', 'Record'] as const;
const BROWSER_POLICY_BUDGET_ROWS = [
  ['Total browser time', 'Managed + unmanaged sessions', '120 min/day', 'Active windows', 'Family default'],
  ['Managed browser time', 'Chrome / Edge managed tabs', '90 min/day', 'School can exclude', 'Override optional'],
  ['Unmanaged browser time', 'Detected browser process', 'Ask or block', 'Always counts', 'Strict override'],
  ['Social / video / games', 'Category targets', '45 min/day', 'Rule windows', 'Per target'],
  ['Search / downloads', 'Activity targets', 'Ask after cap', 'Bedtime stricter', 'Audit trail'],
] as const;
const POLICY_BUDGET_ROWS_BY_AREA = {
  Apps: [
    ['Total app time', 'Foreground app sessions', '120 min/day', 'Active windows', 'Family default'],
    ['School tools', 'Approved study apps', 'Uncapped window', 'School hours', 'Override optional'],
    ['Chat / media apps', 'Category targets', '45 min/day', 'Bedtime stricter', 'Per category'],
    ['Unknown apps', 'Unclassified launches', 'Ask first', 'Always counts', 'Strict override'],
    ['Grace time', 'Approved extensions', 'Parent entered', 'Expires', 'Audit trail'],
  ],
  Games: [
    ['Total game time', 'Foreground game sessions', '60 min/day', 'Active windows', 'Family default'],
    ['School day cap', 'Weekday sessions', '30 min/day', 'After homework', 'Override optional'],
    ['Weekend cap', 'Saturday / Sunday', '120 min/day', 'Weekend windows', 'Per device'],
    ['Cloud / browser games', 'Web game targets', '45 min/day', 'Rule windows', 'Per target'],
    ['Extra play', 'Approved extensions', 'Parent entered', 'Expires', 'Audit trail'],
  ],
  Screen: [
    ['Analysis budget', 'Screen review jobs', 'Digest cadence', 'Active windows', 'Family default'],
    ['Strict mode', 'High-risk triggers', 'Short interval', 'Rule windows', 'Override optional'],
    ['Live view', 'Parent request only', 'Session cap', 'Parent present', 'Audit trail'],
    ['Protected surfaces', 'Unavailable states', 'No capture', 'Always respected', 'Locked'],
  ],
  Network: [
    ['Total web traffic', 'Flow summaries', 'Daily cap', 'Active windows', 'Family default'],
    ['Unknown domains', 'New endpoint flows', 'Ask first', 'Always counts', 'Strict override'],
    ['VPN / proxy', 'Bypass signals', 'Block or ask', 'Always active', 'Audit trail'],
    ['High volume', 'Bandwidth summary', 'Notify cap', 'Schedule aware', 'Per device'],
  ],
  Tracking: [
    ['Location checks', 'Known-place updates', 'Normal cadence', 'Active windows', 'Family default'],
    ['School hours', 'Expected place proof', 'Strict cadence', 'School window', 'Override optional'],
    ['Unknown place', 'Unexpected location', 'Immediate ask', 'Always active', 'Audit trail'],
    ['Route history', 'Parent-visible trail', 'Retention cap', 'After schedule', 'Per device'],
  ],
} as const;
const BROWSER_POLICY_BUDGET_COLUMNS = ['Budget', 'What counts', 'Cap', 'Schedule link', 'Override'] as const;
const BROWSER_POLICY_AUDIT_COLUMNS = ['Check', 'Verifies', 'When', 'Result'] as const;
const BROWSER_POLICY_MATRIX_COLORS = {
  defaultRules: '#37d7ff',
  allow: '#41f385',
  limit: '#ffd36a',
  ask: '#a875ff',
  block: '#ff5e68',
} as const;

type PolicyMatrixArea = keyof typeof POLICY_APPROVAL_ROWS_BY_AREA;

function isPolicyMatrixArea(area: string): area is PolicyMatrixArea {
  return Object.prototype.hasOwnProperty.call(POLICY_APPROVAL_ROWS_BY_AREA, area);
}

function policyApprovalRowsForArea(area: string) {
  return isPolicyMatrixArea(area) ? POLICY_APPROVAL_ROWS_BY_AREA[area] : BROWSER_POLICY_APPROVAL_ROWS;
}

function policyBudgetRowsForArea(area: string) {
  return isPolicyMatrixArea(area) ? POLICY_BUDGET_ROWS_BY_AREA[area] : BROWSER_POLICY_BUDGET_ROWS;
}

function policyAuditRowsForArea(area: string) {
  return [
    [`${area} effective policy`, 'Family + override', 'Before apply', 'Pass / conflict'],
    ['Rule vs schedule', 'Action windows', 'Before apply', 'No overlap'],
    ['Budget proof', 'Cap + evidence path', 'Before apply', 'Timer ready'],
    ['Approval expiry', 'Ask result fallback', 'On request', 'Typed outcome'],
    ['Capability state', 'Adapter support', 'Before enforce', 'Ready / unavailable'],
    ['Audit custody', 'Event refs only', 'After apply', 'Retained'],
  ] as const;
}

function weeklySchedulerPolicyAreaForLabel(policyAreaLabel: string) {
  const normalizedLabel = policyAreaLabel.toLowerCase();
  if (normalizedLabel.includes('app')) {
    return 'apps';
  }
  if (normalizedLabel.includes('game')) {
    return 'games';
  }
  return 'browser';
}

function BrowserPolicyTabMatrixSurface({
  policyAreaLabel,
  tabId,
  x,
  y,
  w,
  h,
  disabled,
  cfg,
}: {
  policyAreaLabel: string;
  tabId: string;
  x: number;
  y: number;
  w: number;
  h: number;
  disabled?: boolean;
  cfg: ParentPortalSvgControls;
}) {
  if (tabId === 'schedule') {
    return (
      <foreignObject x={x} y={y} width={w} height={h}>
        <div style={{ height: h, overflow: 'hidden', width: w }}>
          <WeeklySchedulerScratchPage embedded policyArea={weeklySchedulerPolicyAreaForLabel(policyAreaLabel)} />
        </div>
      </foreignObject>
    );
  }
  if (tabId === 'approvals') {
    return (
      <BrowserPolicyApprovalsMatrix
        policyAreaLabel={policyAreaLabel}
        x={x}
        y={y}
        w={w}
        h={h}
        {...(disabled === undefined ? {} : { disabled })}
        cfg={cfg}
      />
    );
  }
  if (tabId === 'budget') {
    return (
      <BrowserPolicyBudgetMatrix
        policyAreaLabel={policyAreaLabel}
        x={x}
        y={y}
        w={w}
        h={h}
        {...(disabled === undefined ? {} : { disabled })}
        cfg={cfg}
      />
    );
  }
  return (
    <BrowserPolicyAuditMatrix
      policyAreaLabel={policyAreaLabel}
      x={x}
      y={y}
      w={w}
      h={h}
      {...(disabled === undefined ? {} : { disabled })}
      cfg={cfg}
    />
  );
}

function BrowserPolicyMatrixShell({
  x,
  y,
  w,
  h,
  title,
  subtitle,
  disabled,
  cfg,
  children,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  title: string;
  subtitle: string;
  disabled?: boolean;
  cfg: ParentPortalSvgControls;
  children: ReactNode;
}) {
  return (
    <g opacity={disabled ? 0.46 : 1}>
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={5}
        fill="rgba(2, 19, 35, 0.72)"
        stroke={cfg.colors.gold}
        strokeWidth={0.9}
      />
      <path d={`M ${x + 14} ${y + 36} H ${x + w - 14}`} stroke={cfg.colors.cyan} strokeWidth={0.85} opacity={0.62} />
      <text x={x + 18} y={y + 23} fontSize={13.5} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      {subtitle.length > 0 && w >= 760 ? (
        <text x={x + w - 18} y={y + 23} textAnchor="end" fontSize={10.8} fontWeight={760} fill={cfg.colors.mutedText}>
          {truncateTextForWidth(subtitle, Math.max(120, w * 0.42), 10.8, 0.55)}
        </text>
      ) : null}
      {children}
    </g>
  );
}

function UnavailableReadModelSurface({
  title,
  subtitle,
  statusLabel,
  headline,
  detail,
  x,
  y,
  w,
  h,
  cfg,
}: {
  title: string;
  subtitle: string;
  statusLabel: string;
  headline: string;
  detail: string;
  x: number;
  y: number;
  w: number;
  h: number;
  cfg: ParentPortalSvgControls;
}) {
  const detailLines = wrapCardText(detail, w - 76, 12.2, 4);
  return (
    <BrowserPolicyMatrixShell x={x} y={y} w={w} h={h} title={title} subtitle={subtitle} cfg={cfg}>
      <g role="status" aria-label={statusLabel}>
        <rect
          x={x + 18}
          y={y + 54}
          width={w - 36}
          height={Math.max(132, h - 84)}
          rx={5}
          fill={PARENT_PORTAL_GLASS.panelFill}
          stroke={cfg.colors.panelStroke}
        />
        <text x={x + 36} y={y + 88} fontSize={13} fontWeight={950} fill={cfg.colors.gold}>
          {headline}
        </text>
        {detailLines.map((line, index) => (
          <text
            key={`policy-preview-unavailable:${index}`}
            x={x + 36}
            y={y + 118 + index * 18}
            fontSize={12.2}
            fontWeight={760}
            fill={cfg.colors.mutedText}
          >
            {line}
          </text>
        ))}
      </g>
    </BrowserPolicyMatrixShell>
  );
}

function BrowserPolicyBudgetMatrix({
  policyAreaLabel,
  x,
  y,
  w,
  h,
  disabled,
  cfg,
}: {
  policyAreaLabel: string;
  x: number;
  y: number;
  w: number;
  h: number;
  disabled?: boolean;
  cfg: ParentPortalSvgControls;
}) {
  const rows = policyBudgetRowsForArea(policyAreaLabel);
  const tableX = x + 18;
  const tableY = y + 54;
  const tableW = w - 36;
  const tableH = Math.max(230, h - 84);
  const compact = tableW < 760;
  const visibleColumns = compact ? BROWSER_POLICY_BUDGET_COLUMNS.slice(0, 4) : BROWSER_POLICY_BUDGET_COLUMNS;
  const budgetW = Math.max(124, tableW * (compact ? 0.27 : 0.22));
  const capW = Math.max(86, tableW * 0.14);
  const scheduleW = Math.max(116, tableW * (compact ? 0.24 : 0.18));
  const overrideW = compact ? 0 : Math.max(108, tableW * 0.16);
  const countW = Math.max(120, tableW - budgetW - capW - scheduleW - overrideW);
  const columnWidths = compact ? [budgetW, countW, capW, scheduleW] : [budgetW, countW, capW, scheduleW, overrideW];
  const columnStarts: number[] = [];
  let runningColumnX = 0;
  columnWidths.forEach((columnWidth) => {
    columnStarts.push(runningColumnX);
    runningColumnX += columnWidth;
  });
  const rowH = Math.max(34, Math.min(50, (tableH - 74) / rows.length));
  const guideY = 30;
  const rowsY = 68;

  return (
    <BrowserPolicyMatrixShell
      x={x}
      y={y}
      w={w}
      h={h}
      title={`${policyAreaLabel}: budgets and caps`}
      subtitle="Limit means minutes here; schedules only choose when those caps run"
      {...(disabled === undefined ? {} : { disabled })}
      cfg={cfg}
    >
      <g transform={`translate(${tableX}, ${tableY})`}>
        <rect
          width={tableW}
          height={tableH}
          rx={5}
          fill={PARENT_PORTAL_GLASS.panelFill}
          stroke={cfg.colors.panelStroke}
        />
        <g transform={`translate(8, ${guideY - 18})`}>
          {(
            [
              ['Family default', 'applies first', cfg.colors.cyan],
              ['Device override', 'grey until enabled', cfg.colors.gold],
              ['Schedule', 'gates active time', BROWSER_POLICY_MATRIX_COLORS.ask],
            ] satisfies readonly (readonly [string, string, string])[]
          ).map((item, index) => {
            const pillW = compact ? (tableW - 28) / 3 : 172;
            const pillX = index * (pillW + 8);
            const label = item[0];
            const detail = item[1];
            const color = item[2];
            return (
              <g key={`browser-policy-budget-guide:${label}`} transform={`translate(${pillX}, 0)`}>
                <rect
                  width={pillW}
                  height={26}
                  rx={4}
                  fill="rgba(5, 27, 46, 0.64)"
                  stroke={color}
                  strokeWidth={0.72}
                  opacity={0.95}
                />
                <circle cx={11} cy={13} r={3.2} fill={color} />
                <text x={21} y={11} fontSize={8.9} fontWeight={950} fill={cfg.colors.bodyText}>
                  {truncateTextForWidth(label, pillW - 30, 8.9, 0.58)}
                </text>
                <text x={21} y={21} fontSize={7.8} fontWeight={760} fill={cfg.colors.mutedText}>
                  {truncateTextForWidth(detail, pillW - 30, 7.8, 0.58)}
                </text>
              </g>
            );
          })}
        </g>
        <g transform={`translate(8, ${rowsY - 23})`}>
          {visibleColumns.map((column, index) =>
            (() => {
              const columnStart = columnStarts[index];
              const columnWidth = columnWidths[index];
              if (columnStart === undefined || columnWidth === undefined) return null;
              return (
                <text
                  key={`browser-policy-budget-column:${column}`}
                  x={columnStart + (index === 0 ? 8 : columnWidth / 2)}
                  y={13}
                  textAnchor={index === 0 ? 'start' : 'middle'}
                  fontSize={9.8}
                  fontWeight={950}
                  fill={cfg.colors.gold}
                >
                  {column}
                </text>
              );
            })()
          )}
        </g>
        <g transform={`translate(8, ${rowsY})`}>
          {rows.map((row, rowIndex) => {
            const rowFill = rowIndex % 2 === 0 ? 'rgba(7, 27, 47, 0.74)' : 'rgba(7, 23, 42, 0.74)';
            const capText = row[2];
            return (
              <g key={`browser-policy-budget-row:${row[0]}`} transform={`translate(0, ${rowIndex * rowH})`}>
                <rect
                  width={tableW - 16}
                  height={rowH - 6}
                  rx={4}
                  fill={rowFill}
                  stroke={cfg.colors.panelStroke}
                  strokeWidth={0.72}
                />
                <rect
                  x={budgetW + countW + 8}
                  y={7}
                  width={Math.max(38, capW - 16)}
                  height={rowH - 20}
                  rx={3}
                  fill="rgba(255, 211, 106, 0.16)"
                  stroke={BROWSER_POLICY_MATRIX_COLORS.limit}
                  strokeWidth={0.75}
                />
                <path
                  d={`M ${budgetW + countW + 13} ${rowH - 13} H ${budgetW + countW + Math.max(34, capW - 13)}`}
                  stroke={BROWSER_POLICY_MATRIX_COLORS.limit}
                  strokeWidth={2.2}
                  strokeLinecap="round"
                  opacity={0.7}
                />
                {row.slice(0, visibleColumns.length).map((value, index) => {
                  const columnStart = columnStarts[index];
                  const columnWidth = columnWidths[index];
                  if (columnStart === undefined || columnWidth === undefined || value === undefined) return null;
                  const cellX = columnStart + 8;
                  const cellW = columnWidth - 16;
                  const fill =
                    index === 0
                      ? cfg.colors.bodyText
                      : index === 2
                        ? BROWSER_POLICY_MATRIX_COLORS.limit
                        : index === 4
                          ? cfg.colors.cyan
                          : cfg.colors.mutedText;
                  return (
                    <text
                      key={`browser-policy-budget-cell:${row[0]}:${index}`}
                      x={index === 0 ? cellX : cellX + cellW / 2}
                      y={rowH / 2 + 3}
                      textAnchor={index === 0 ? 'start' : 'middle'}
                      fontSize={fitSingleLineTextSize(value, cellW, 8.2, 10.8, 0.56)}
                      fontWeight={index === 0 || value === capText ? 920 : 780}
                      fill={fill}
                    >
                      {truncateTextForWidth(value, cellW, 10.8, 0.56)}
                    </text>
                  );
                })}
              </g>
            );
          })}
        </g>
      </g>
    </BrowserPolicyMatrixShell>
  );
}

function BrowserPolicyApprovalsMatrix({
  policyAreaLabel,
  x,
  y,
  w,
  h,
  disabled,
  cfg,
}: {
  policyAreaLabel: string;
  x: number;
  y: number;
  w: number;
  h: number;
  disabled?: boolean;
  cfg: ParentPortalSvgControls;
}) {
  const rows = policyApprovalRowsForArea(policyAreaLabel);
  const tableX = x + 18;
  const tableY = y + 54;
  const tableW = w - 36;
  const tableH = Math.max(230, h - 84);
  const rowH = Math.max(28, Math.min(44, (tableH - 28) / rows.length));
  const visibleColumns = tableW < 760 ? BROWSER_POLICY_APPROVAL_COLUMNS.slice(0, 5) : BROWSER_POLICY_APPROVAL_COLUMNS;
  const requestW = Math.max(118, tableW * (tableW < 760 ? 0.27 : 0.22));
  const colW = (tableW - requestW) / Math.max(1, visibleColumns.length - 1);

  return (
    <BrowserPolicyMatrixShell
      x={x}
      y={y}
      w={w}
      h={h}
      title={`${policyAreaLabel}: request matrix`}
      subtitle="What can ask, who is notified, and what happens if parent is quiet"
      {...(disabled === undefined ? {} : { disabled })}
      cfg={cfg}
    >
      <g transform={`translate(${tableX}, ${tableY})`}>
        <rect
          width={tableW}
          height={tableH}
          rx={5}
          fill={PARENT_PORTAL_GLASS.panelFill}
          stroke={cfg.colors.panelStroke}
        />
        {visibleColumns.map((column, index) => (
          <text
            key={`browser-policy-approval-column:${column}`}
            x={index === 0 ? 12 : requestW + (index - 1) * colW + colW / 2}
            y={18}
            textAnchor={index === 0 ? 'start' : 'middle'}
            fontSize={10.3}
            fontWeight={950}
            fill={cfg.colors.gold}
          >
            {column}
          </text>
        ))}
        {rows.map((row, rowIndex) => (
          <g key={`browser-policy-approval-row:${row[0]}`} transform={`translate(0, ${28 + rowIndex * rowH})`}>
            <rect
              x={4}
              y={3}
              width={tableW - 8}
              height={rowH - 6}
              rx={4}
              fill="rgba(8, 26, 47, 0.7)"
              stroke={cfg.colors.panelStroke}
            />
            {row.slice(0, visibleColumns.length).map((value, index) => {
              const cellX = index === 0 ? 12 : requestW + (index - 1) * colW + 6;
              const cellW = index === 0 ? requestW - 20 : colW - 12;
              const fill =
                index === 0
                  ? cfg.colors.bodyText
                  : index === 1
                    ? cfg.colors.cyan
                    : index === 2
                      ? BROWSER_POLICY_MATRIX_COLORS.allow
                      : cfg.colors.mutedText;
              return (
                <text
                  key={`browser-policy-approval-cell:${row[0]}:${index}`}
                  x={index === 0 ? cellX : cellX + cellW / 2}
                  y={rowH / 2 + 4}
                  textAnchor={index === 0 ? 'start' : 'middle'}
                  fontSize={fitSingleLineTextSize(value, cellW, 8.2, 10.8, 0.56)}
                  fontWeight={index === 0 ? 900 : 840}
                  fill={fill}
                >
                  {truncateTextForWidth(value, cellW, 10.8, 0.56)}
                </text>
              );
            })}
          </g>
        ))}
      </g>
    </BrowserPolicyMatrixShell>
  );
}

function BrowserPolicyAuditMatrix({
  policyAreaLabel,
  x,
  y,
  w,
  h,
  disabled,
  cfg,
}: {
  policyAreaLabel: string;
  x: number;
  y: number;
  w: number;
  h: number;
  disabled?: boolean;
  cfg: ParentPortalSvgControls;
}) {
  const rows = policyAuditRowsForArea(policyAreaLabel);
  const tableX = x + 18;
  const tableY = y + 54;
  const tableW = w - 36;
  const tableH = Math.max(220, h - 84);
  const rowH = Math.max(30, Math.min(46, (tableH - 30) / rows.length));
  const visibleColumns = tableW < 640 ? BROWSER_POLICY_AUDIT_COLUMNS.slice(0, 3) : BROWSER_POLICY_AUDIT_COLUMNS;
  const eventW = Math.max(136, tableW * (tableW < 640 ? 0.38 : 0.3));
  const colW = (tableW - eventW) / Math.max(1, visibleColumns.length - 1);

  return (
    <BrowserPolicyMatrixShell
      x={x}
      y={y}
      w={w}
      h={h}
      title={`${policyAreaLabel}: effective-policy checkpoint`}
      subtitle="Conflicts, inheritance, budgets, approvals, and capability state before apply"
      {...(disabled === undefined ? {} : { disabled })}
      cfg={cfg}
    >
      <g transform={`translate(${tableX}, ${tableY})`}>
        <rect
          width={tableW}
          height={tableH}
          rx={5}
          fill={PARENT_PORTAL_GLASS.panelFill}
          stroke={cfg.colors.panelStroke}
        />
        {visibleColumns.map((column, index) => (
          <text
            key={`browser-policy-audit-column:${column}`}
            x={index === 0 ? 12 : eventW + (index - 1) * colW + colW / 2}
            y={19}
            textAnchor={index === 0 ? 'start' : 'middle'}
            fontSize={10.5}
            fontWeight={950}
            fill={cfg.colors.gold}
          >
            {column}
          </text>
        ))}
        {rows.map((row, rowIndex) => (
          <g key={`browser-policy-audit-row:${row[0]}`} transform={`translate(0, ${30 + rowIndex * rowH})`}>
            <rect
              x={4}
              y={3}
              width={tableW - 8}
              height={rowH - 6}
              rx={4}
              fill={rowIndex % 2 === 0 ? 'rgba(8, 26, 47, 0.72)' : 'rgba(6, 22, 40, 0.72)'}
              stroke={cfg.colors.panelStroke}
              strokeWidth={0.72}
            />
            {row.slice(0, visibleColumns.length).map((value, index) => {
              const cellX = index === 0 ? 12 : eventW + (index - 1) * colW + 6;
              const cellW = index === 0 ? eventW - 20 : colW - 12;
              const fill =
                index === 0
                  ? cfg.colors.bodyText
                  : index === 1
                    ? cfg.colors.cyan
                    : index === 2
                      ? BROWSER_POLICY_MATRIX_COLORS.allow
                      : cfg.colors.gold;
              return (
                <text
                  key={`browser-policy-audit-cell:${row[0]}:${index}`}
                  x={index === 0 ? cellX : cellX + cellW / 2}
                  y={rowH / 2 + 4}
                  textAnchor={index === 0 ? 'start' : 'middle'}
                  fontSize={fitSingleLineTextSize(value, cellW, 8.4, 11, 0.56)}
                  fontWeight={index === 0 ? 900 : 840}
                  fill={fill}
                >
                  {truncateTextForWidth(value, cellW, 11, 0.56)}
                </text>
              );
            })}
          </g>
        ))}
      </g>
    </BrowserPolicyMatrixShell>
  );
}

function ManageWorkspacePanel({
  x,
  y,
  w,
  h,
  kind,
  activeTabId,
  defaultTabId,
  onTabChange,
  onNavigate,
  activeNavLabel,
  selectedControlName,
  runtimeDeviceSlots,
  sharedTargetSelection,
  onSharedTargetChange,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  kind: ManageWorkspaceKind;
  activeTabId: string;
  defaultTabId: string;
  onTabChange: (tabId: string) => void;
  onNavigate?: (routePath: string) => void;
  activeNavLabel: string;
  selectedControlName: string;
  runtimeDeviceSlots: readonly DeviceSlot[];
  sharedTargetSelection: ManageTargetSelection;
  onSharedTargetChange?: (selection: ManageTargetSelection) => void;
  cfg: ParentPortalSvgControls;
}) {
  const tabs = manageWorkspaceTabs(kind);
  const hasRuntimeDeviceSlots = runtimeDeviceSlots.length > 0;
  const targetOptions = useMemo(
    () =>
      manageWorkspaceTargetOptions(kind).filter(
        (targetOption) => targetOption.id !== 'perDevice' || hasRuntimeDeviceSlots
      ),
    [hasRuntimeDeviceSlots, kind]
  );
  const workspaceScopeValues = targetOptions.map((targetOption) =>
    targetOption.id === 'perDevice' ? 'parent' : targetOption.id === 'portal' ? 'portal' : 'lan'
  );
  const workspaceSelectionSlots = runtimeDeviceSlots;
  const [workspaceTarget, setWorkspaceTarget] = useState<ManageWorkspaceTarget>(() =>
    sharedWorkspaceTargetForOptions(targetOptions, sharedTargetSelection)
  );
  const [workspaceSelectedDeviceValue, setWorkspaceSelectedDeviceValue] = useState<string | undefined>(() =>
    reportSelectedSlotValue(workspaceSelectionSlots, sharedTargetSelection)
  );
  useEffect(() => {
    setWorkspaceTarget((currentTarget) =>
      reconciledWorkspaceTargetForOptions(targetOptions, sharedTargetSelection, currentTarget)
    );
    setWorkspaceSelectedDeviceValue(reportSelectedSlotValue(workspaceSelectionSlots, sharedTargetSelection));
  }, [activeNavLabel, kind, selectedControlName, sharedTargetSelection, targetOptions, workspaceSelectionSlots]);
  const activeTab =
    tabs.find((tab) => tab.id === activeTabId) ?? tabs.find((tab) => tab.id === defaultTabId) ?? tabs[0];
  const activeTabKey = activeTab?.id ?? defaultTabId;
  const activeColor = toneColor(activeTab?.tone ?? 'cyan', cfg);
  const activeTarget = targetOptions.find((option) => option.id === workspaceTarget) ?? targetOptions[0] ?? null;
  const workspaceTargetKey = activeTarget?.id ?? 'family';
  const targetColor = toneColor(activeTarget?.tone ?? activeTab?.tone ?? 'cyan', cfg);
  const policyAreaLabel = kind === 'policy' ? managePolicyAreaLabel(activeNavLabel, selectedControlName) : '';
  const policyAreaActive = kind === 'policy';
  const policyRulesChoiceMode = policyAreaActive && activeTabKey === 'rules';
  const policyMatrixMode =
    policyAreaActive &&
    (activeTabKey === 'schedule' ||
      activeTabKey === 'budget' ||
      activeTabKey === 'approvals' ||
      activeTabKey === 'audit');
  const policyCustomSurfaceMode = policyRulesChoiceMode || policyMatrixMode;
  const policyPrimaryOptions = useMemo(() => managePolicyPrimaryChoiceOptions(activeTabKey), [activeTabKey]);
  const policySecondaryOptions = useMemo(() => managePolicySecondaryChoiceOptions(activeTabKey), [activeTabKey]);
  const [policyPrimaryChoice, setPolicyPrimaryChoice] = useState(policyPrimaryOptions[0]?.value ?? '');
  const [policySecondaryChoice, setPolicySecondaryChoice] = useState(policySecondaryOptions[0]?.value ?? '');
  const [browserRulesEnforcementChoice, setBrowserRulesEnforcementChoice] = useState('observe');
  useEffect(() => {
    setPolicyPrimaryChoice(policyPrimaryOptions[0]?.value ?? '');
  }, [policyPrimaryOptions]);
  useEffect(() => {
    setPolicySecondaryChoice(policySecondaryOptions[0]?.value ?? '');
  }, [policySecondaryOptions]);
  const targetSurfaceEnabled = targetOptions.length > 0;
  const workspaceSlots = workspaceSelectionSlots;
  const workspacePortalIds = useMemo(
    () => workspaceSlots.filter((slot) => slot.device).map((slot) => slot.value),
    [workspaceSlots]
  );
  const workspaceDeviceCount = Math.max(1, workspaceSlots.length);
  const workspacePanelPadX = w < 480 ? 8 : Math.max(18, Math.min(34, Math.round(w * 0.018)));
  const workspaceAvailableW = Math.max(1, w - workspacePanelPadX * 2);
  const workspaceAvailableH = Math.max(1, h);
  const compact = workspaceAvailableW < 760;
  const workspaceGridColumnsByWidth = Math.max(
    1,
    Math.floor(
      (Math.max(1, workspaceAvailableW - 44) + MANAGE_DEVICE_GRID_GAP_X) /
        (MANAGE_DEVICE_GRID_CELL_W + MANAGE_DEVICE_GRID_GAP_X)
    )
  );
  const workspaceGridColumns = Math.max(1, Math.min(workspaceDeviceCount, workspaceGridColumnsByWidth));
  const workspaceGridRows =
    workspaceTargetKey === 'perDevice' ? Math.max(1, Math.ceil(workspaceDeviceCount / workspaceGridColumns)) : 1;
  const workspaceSelectorH = targetSurfaceEnabled
    ? Math.max(
        176,
        Math.min(
          workspaceAvailableH * 0.42,
          ACTIVITY_REPORT_SELECTOR_BASE_H + workspaceGridRows * ACTIVITY_REPORT_SELECTOR_ROW_H
        )
      )
    : 0;
  const workspaceTopX = x + workspacePanelPadX;
  const workspaceTopY = y;
  const workspaceDividerY = workspaceTopY + workspaceSelectorH;
  const workspaceBodyX = workspaceTopX;
  const workspaceBodyW = workspaceAvailableW;
  const workspaceGridHostStyle: CSSProperties = {
    width: workspaceAvailableW,
    height: workspaceSelectorH,
    position: 'relative',
  };
  const workspaceSelectedValue = workspaceTargetKey === 'perDevice' ? (workspaceSelectedDeviceValue ?? '') : '';
  const workspaceSelectedSlot = workspaceSlots.find((slot) => slot.value === workspaceSelectedValue) ?? null;
  const workspaceSelectedLabel = workspaceSelectedSlot?.label ?? null;
  const workspaceTargetStatus = manageWorkspaceTargetStatus(
    workspaceTargetKey,
    workspaceSelectedSlot,
    hasRuntimeDeviceSlots
  );
  const workspaceTargetStatusStyle: CSSProperties = {
    position: 'absolute',
    inset: compact ? '76px 8px 42px' : '76px 24px 42px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'column',
    gap: 6,
    padding: compact ? '8px 10px' : '10px 18px',
    border: `1px solid ${colorAlpha(targetColor, '40')}`,
    borderRadius: 8,
    background: compact ? 'rgba(3, 18, 32, 0.94)' : colorAlpha(targetColor, '12'),
    color: cfg.colors.bodyText,
    textAlign: 'center',
    pointerEvents: 'none',
  };
  const tabColumns = compact
    ? Math.min(workspaceBodyW < 480 ? 2 : 3, tabs.length)
    : Math.min(tabs.length, kind === 'ai' ? 4 : tabs.length);
  const tabRows = Math.max(1, Math.ceil(tabs.length / tabColumns));
  const tabInsetX = workspaceBodyW < 480 ? 8 : Math.max(16, Math.min(24, workspaceBodyW * 0.012));
  const tabW = Math.max(compact ? 82 : 112, Math.min(168, (workspaceBodyW - tabInsetX * 2) / tabColumns));
  const tabH = compact ? 31 : 39;
  const tabGap = 0;
  const tabAreaH = tabRows * tabH + 8;
  const tabsY = targetSurfaceEnabled ? workspaceDividerY + 16 : y;
  const bodyY = tabsY + tabAreaH - 1;
  const bodyH = Math.max(1, y + h - bodyY - 8);
  const summaryBaseY = targetSurfaceEnabled
    ? kind === 'policy'
      ? bodyY + (compact ? 92 : 88)
      : bodyY + 24
    : bodyY + 24;
  const summary = manageWorkspaceSummary(kind, activeTabKey, activeNavLabel, selectedControlName, workspaceTargetKey);
  const summaryLines = policyCustomSurfaceMode ? [] : wrapCardText(summary, workspaceBodyW - 42, 12.2, compact ? 2 : 1);
  const bodyHeaderH = targetSurfaceEnabled
    ? kind === 'policy'
      ? policyCustomSurfaceMode
        ? compact
          ? 98
          : 92
        : compact
          ? 134
          : 126
      : compact
        ? 68
        : 58
    : compact
      ? 76
      : 64;
  const cards = manageWorkspaceCards(kind, activeTabKey, activeNavLabel, selectedControlName, workspaceTargetKey);
  const supportContactFormEnabled = kind === 'account' && activeTabKey === 'support';
  const policyRows =
    kind === 'policy'
      ? managePolicySettingRows(policyAreaLabel, activeTabKey, workspaceTargetKey, workspaceSelectedLabel)
      : [];
  const cardGap = 10;
  const sparsePortalCards = kind === 'portal' && cards.length <= 4;
  const cardColumns = sparsePortalCards
    ? workspaceBodyW > 560
      ? 2
      : 1
    : workspaceBodyW > 1180
      ? 4
      : workspaceBodyW > 840
        ? 3
        : workspaceBodyW > 560
          ? 2
          : 1;
  const cardW = Math.max(1, (workspaceBodyW - 28 * 2 - cardGap * (cardColumns - 1)) / cardColumns);
  const cardRows = Math.ceil(cards.length / cardColumns);
  const cardsTop = bodyY + bodyHeaderH + 12;
  const availableCardH = Math.max(1, bodyY + bodyH - cardsTop - 18);
  const cardH = clampValue(
    (availableCardH - cardGap * Math.max(0, cardRows - 1)) / Math.max(1, cardRows),
    sparsePortalCards ? 96 : 70,
    sparsePortalCards ? 172 : 112
  );
  const policyChoiceBarY = bodyY + 12;
  const policyChoiceGap = 12;
  const policyChoicesStacked = compact && kind === 'policy';
  const policyChoiceW = policyChoicesStacked
    ? Math.max(220, workspaceBodyW - 44)
    : Math.max(220, (workspaceBodyW - 44 - policyChoiceGap) / 2);
  const policySecondaryX = policyChoicesStacked
    ? workspaceBodyX + 22
    : workspaceBodyX + 22 + policyChoiceW + policyChoiceGap;
  const policySecondaryY = policyChoicesStacked ? policyChoiceBarY + 58 : policyChoiceBarY;
  const policyMutationUnavailable = kind === 'policy';
  const policyMutationUnavailableLines = wrapCardText(
    'This overview is read-only. Choose a section to open its guide; current policy appears only when the local service reports it in a supported policy view.',
    workspaceBodyW - 44,
    11.5,
    compact ? 3 : 1
  );
  const policyChoiceDisabled =
    policyMutationUnavailable || (workspaceTargetKey === 'perDevice' && !workspaceSelectedSlot);
  const policyContentTop = bodyY + bodyHeaderH + 10;
  const policySurfaceTop = policyContentTop;
  const browserRulesChoiceTop = policySurfaceTop;
  const policyRowsTop = policySurfaceTop;
  const policyRowGap = 10;
  const policyRowColumns = workspaceBodyW > 1060 ? 2 : 1;
  const policyRowW = Math.max(1, (workspaceBodyW - 28 * 2 - policyRowGap * (policyRowColumns - 1)) / policyRowColumns);
  const policyRowH = clampValue(
    (Math.max(1, bodyY + bodyH - policyRowsTop - 18) -
      policyRowGap * Math.max(0, Math.ceil(policyRows.length / policyRowColumns) - 1)) /
      Math.max(1, Math.ceil(policyRows.length / policyRowColumns)),
    58,
    76
  );
  return (
    <g>
      {targetSurfaceEnabled ? (
        <>
          <foreignObject x={workspaceTopX} y={workspaceTopY} width={workspaceAvailableW} height={workspaceSelectorH}>
            <div style={workspaceGridHostStyle}>
              <DeviceChoiceGrid
                scope={
                  workspaceTargetKey === 'perDevice' ? 'parent' : workspaceTargetKey === 'portal' ? 'portal' : 'lan'
                }
                value={workspaceSelectedValue}
                options={[...workspaceSlots]}
                portalDeviceIds={[...workspacePortalIds]}
                rows={workspaceGridRows}
                columns={workspaceGridColumns}
                parentRows={workspaceGridRows}
                parentColumns={workspaceGridColumns}
                deviceSelectionDisabled={workspaceTargetKey !== 'perDevice'}
                scopeValues={workspaceScopeValues}
                scopeIcons={FAMILY_DEVICE_SCOPE_ICONS}
                onScopeChange={(nextScopeValue) => {
                  const nextTarget =
                    nextScopeValue === 'parent' ? 'perDevice' : nextScopeValue === 'portal' ? 'portal' : 'family';
                  setWorkspaceTarget(nextTarget);
                  const nextSelection = workspaceSelectedSlot
                    ? selectedManageTargetSelectionForSlot(sharedTargetSelection, workspaceSelectedSlot)
                    : sharedTargetSelection;
                  onSharedTargetChange?.({
                    ...nextSelection,
                    scope: nextTarget === 'perDevice' ? 'perDevice' : 'global',
                  });
                }}
                onChange={(choice) => {
                  setWorkspaceTarget('perDevice');
                  setWorkspaceSelectedDeviceValue(choice.value);
                  onSharedTargetChange?.(selectedManageTargetSelectionForSlot(sharedTargetSelection, choice));
                }}
                config={manageDeviceGridConfig(workspaceAvailableW, workspaceSelectorH, {
                  statusOrder: {
                    lan: ['connected', 'offline', 'empty'],
                    parent: ['connected', 'offline', 'empty'],
                    portal: ['connected', 'offline', 'empty'],
                  },
                  text: {
                    scopeOptions: { lan: 'Family', parent: 'Per Device', portal: 'Portal' },
                    selectedInfoLabel:
                      workspaceTargetKey === 'portal'
                        ? 'Portal target'
                        : kind === 'policy'
                          ? `${policyAreaLabel} target`
                          : `${manageWorkspaceTitle(kind)} device`,
                    selectedInfoEmptyLabel:
                      workspaceTargetKey === 'portal'
                        ? 'Parent console'
                        : workspaceTargetKey === 'family'
                          ? 'Whole family'
                          : 'No device selected',
                  },
                })}
              />
              {workspaceTargetStatus ? (
                <div role="status" aria-label={workspaceTargetStatus.ariaLabel} style={workspaceTargetStatusStyle}>
                  <span
                    style={{
                      color: targetColor,
                      fontSize: 13,
                      fontWeight: 950,
                      letterSpacing: '0.08em',
                    }}
                  >
                    {workspaceTargetStatus.eyebrow}
                  </span>
                  <span style={{ maxWidth: 680, fontSize: 13.5, fontWeight: 760, lineHeight: 1.4 }}>
                    {workspaceTargetStatus.detail}
                  </span>
                </div>
              ) : null}
            </div>
          </foreignObject>
          <path
            d={`M ${workspaceTopX} ${workspaceDividerY} H ${workspaceTopX + workspaceAvailableW}`}
            stroke={targetColor}
            strokeWidth={3}
            opacity={0.18}
          />
          <path
            d={`M ${workspaceTopX} ${workspaceDividerY} H ${workspaceTopX + workspaceAvailableW}`}
            stroke={targetColor}
            strokeWidth={1.35}
            opacity={0.72}
          />
        </>
      ) : null}
      <g role={policyMutationUnavailable ? 'group' : 'tablist'} aria-label={`${manageWorkspaceTitle(kind)} tabs`}>
        {tabs.map((tab, index) => {
          const selected = !policyMutationUnavailable && tab.id === activeTabKey;
          const tabColor = toneColor(tab.tone, cfg);
          const column = index % tabColumns;
          const row = Math.floor(index / tabColumns);
          const tabX = workspaceBodyX + tabInsetX + column * (tabW + tabGap);
          const tabY = tabsY + row * tabH + (selected ? 0 : 5);
          const currentTabH = selected ? tabH + 3 : tabH - 5;
          const tabIconSize = Math.max(14, Math.min(21, currentTabH - 12));
          const tabTextSize = compact ? 10.4 : 12.4;
          const showTabGuideInfo = kind === 'policy' && Boolean(onNavigate);
          const tabInfoR = showTabGuideInfo ? Math.max(6, Math.min(8, currentTabH * 0.24)) : 0;
          const tabInfoCx = tabX + tabW - 15;
          const tabInfoCy = tabY + currentTabH / 2;
          const tabTextMaxW = tabW - tabIconSize - (showTabGuideInfo ? 48 : 25);
          const tabText = truncateTextForWidth(tab.label, tabTextMaxW, tabTextSize, 0.58);
          const tabTextW = Math.min(tabTextMaxW, tabText.length * tabTextSize * 0.58);
          const tabGroupW = tabIconSize + 7 + tabTextW;
          const tabIconX = tabX + Math.max(8, ((showTabGuideInfo ? tabW - 26 : tabW) - tabGroupW) / 2);
          const tabIconY = tabY + (currentTabH - tabIconSize) / 2;
          const TabIcon = tab.icon;
          const tabGuideRoutePath = showTabGuideInfo
            ? guideRoutePathForManageTab(activeNavLabel, selectedControlName, tab.id)
            : '';
          const tabGuideAriaLabel =
            policyAreaLabel === 'Rules' && tab.id === 'rules'
              ? 'Open Family Rules guide'
              : `Open ${policyAreaLabel} ${tab.label} guide`;
          const tabOpensGuide = policyMutationUnavailable && showTabGuideInfo;
          const tabFill = selected ? PARENT_PORTAL_TAB_SURFACE_FILL.lanActive : PARENT_PORTAL_TAB_SURFACE_FILL.lanIdle;
          const tabStrokeOpacity = selected ? 0.94 : 0.54;
          return (
            <g
              key={`manage-workspace-tab:${kind}:${tab.id}`}
              className="parent-portal-svg-clickable"
              role={tabOpensGuide ? 'button' : 'tab'}
              tabIndex={0}
              aria-label={tabOpensGuide ? tabGuideAriaLabel : `Show ${tab.label}`}
              aria-selected={tabOpensGuide ? undefined : selected}
              onClick={(event) => {
                event.stopPropagation();
                if (tabOpensGuide) {
                  onNavigate?.(tabGuideRoutePath);
                  return;
                }
                onTabChange(tab.id);
              }}
              onKeyDown={(event) => {
                if (event.key !== 'Enter' && event.key !== ' ') return;
                event.preventDefault();
                event.stopPropagation();
                if (tabOpensGuide) {
                  onNavigate?.(tabGuideRoutePath);
                  return;
                }
                onTabChange(tab.id);
              }}
            >
              <rect x={tabX} y={tabsY + row * tabH - 4} width={tabW} height={tabH + 8} fill="transparent" />
              {selected ? (
                <rect
                  x={tabX + 1}
                  y={tabY - 2}
                  width={tabW - 2}
                  height={currentTabH + 5}
                  rx={0}
                  fill="none"
                  stroke={tabColor}
                  strokeWidth={2.3}
                  opacity={0.14}
                  filter="url(#parentPortalGlow)"
                />
              ) : null}
              <rect
                x={tabX}
                y={tabY}
                width={tabW}
                height={currentTabH}
                rx={0}
                fill={tabFill}
                opacity={selected ? 1 : 0.82}
              />
              <path
                d={`M ${tabX} ${tabY} H ${tabX + tabW}`}
                stroke={selected ? tabColor : cfg.colors.panelStroke}
                strokeWidth={selected ? 2.2 : 1}
                strokeLinecap="round"
                opacity={selected ? 0.95 : 0.34}
              />
              <path
                d={`M ${tabX} ${tabY} V ${tabY + currentTabH}`}
                stroke={cfg.colors.panelStroke}
                strokeWidth={0.8}
                strokeLinecap="round"
                opacity={column === 0 ? tabStrokeOpacity : 0.22}
              />
              <path
                d={`M ${tabX + tabW} ${tabY} V ${tabY + currentTabH}`}
                stroke={cfg.colors.panelStroke}
                strokeWidth={0.8}
                strokeLinecap="round"
                opacity={tabStrokeOpacity}
              />
              <path
                d={`M ${tabX + 12} ${tabY + currentTabH - 3} H ${tabX + tabW - 12}`}
                stroke={tabColor}
                strokeWidth={selected ? 2.25 : 1.15}
                strokeLinecap="round"
                opacity={selected ? 0.95 : 0.34}
              />
              <TabIcon x={tabIconX} y={tabIconY} width={tabIconSize} height={tabIconSize} />
              <text
                x={tabIconX + tabIconSize + 7}
                y={tabY + currentTabH * 0.64}
                fontSize={tabTextSize}
                fontWeight={selected ? 950 : 850}
                fill={selected ? cfg.colors.bodyText : cfg.colors.mutedText}
                pointerEvents="none"
              >
                {tabText}
              </text>
              {showTabGuideInfo && !tabOpensGuide ? (
                <g
                  className="parent-portal-svg-clickable"
                  role="button"
                  tabIndex={0}
                  aria-label={tabGuideAriaLabel}
                  onClick={(event) => {
                    event.preventDefault();
                    event.stopPropagation();
                    onNavigate?.(tabGuideRoutePath);
                  }}
                  onKeyDown={(event) => {
                    if (event.key !== 'Enter' && event.key !== ' ') return;
                    event.preventDefault();
                    event.stopPropagation();
                    onNavigate?.(tabGuideRoutePath);
                  }}
                >
                  <circle
                    cx={tabInfoCx}
                    cy={tabInfoCy}
                    r={tabInfoR}
                    fill={selected ? colorAlpha(tabColor, '36') : PARENT_PORTAL_GLASS.panelFillStrong}
                    stroke={selected ? cfg.colors.bodyText : tabColor}
                    strokeWidth={selected ? 1.1 : 0.85}
                    opacity={selected ? 0.95 : 0.76}
                  />
                  <text
                    x={tabInfoCx}
                    y={tabInfoCy + tabInfoR * 0.42}
                    textAnchor="middle"
                    fontSize={Math.max(8, tabInfoR * 1.32)}
                    fontWeight={950}
                    fill={selected ? cfg.colors.bodyText : tabColor}
                    pointerEvents="none"
                  >
                    i
                  </text>
                </g>
              ) : null}
            </g>
          );
        })}
      </g>
      <path
        d={topRoundedRectPath(workspaceBodyX, bodyY, workspaceBodyW, bodyH, 10)}
        fill={PARENT_PORTAL_GLASS.panelFill}
        stroke={activeColor}
        strokeWidth={1.12}
        opacity={PARENT_PORTAL_CONTENT_SURFACE_OPACITY}
      />
      {policyCustomSurfaceMode ? (
        <g role="status" aria-label="Policy changes unavailable">
          <text x={workspaceBodyX + 22} y={bodyY + 28} fontSize={11.2} fontWeight={950} fill={cfg.colors.red}>
            CHANGES ARE UNAVAILABLE
          </text>
          {policyMutationUnavailableLines.map((line, index) => (
            <text
              key={`policy-mutation-unavailable:${index}`}
              x={workspaceBodyX + 22}
              y={bodyY + 50 + index * 15}
              fontSize={11.5}
              fontWeight={760}
              fill={cfg.colors.mutedText}
            >
              {line}
            </text>
          ))}
        </g>
      ) : null}
      <path
        d={`M ${workspaceBodyX + 10} ${bodyY} H ${workspaceBodyX + workspaceBodyW - 10}`}
        stroke={activeColor}
        strokeWidth={2.2}
        opacity={0.55}
      />
      {kind === 'policy' && !policyCustomSurfaceMode ? (
        <g>
          <foreignObject x={workspaceBodyX + 22} y={policyChoiceBarY} width={policyChoiceW} height={66}>
            <div style={{ width: policyChoiceW, height: 66 }}>
              <ScopeToggle
                title={managePolicyPrimaryChoiceTitle(activeTabKey)}
                value={policyPrimaryChoice}
                options={[...policyPrimaryOptions]}
                disabled={policyChoiceDisabled}
                onChange={(nextValue) => setPolicyPrimaryChoice(nextValue)}
                config={activityScopeToggleConfig(policyChoiceW)}
              />
            </div>
          </foreignObject>
          <foreignObject x={policySecondaryX} y={policySecondaryY} width={policyChoiceW} height={66}>
            <div style={{ width: policyChoiceW, height: 66 }}>
              <ScopeToggle
                title={managePolicySecondaryChoiceTitle(activeTabKey)}
                value={policySecondaryChoice}
                options={[...policySecondaryOptions]}
                disabled={policyChoiceDisabled}
                onChange={(nextValue) => setPolicySecondaryChoice(nextValue)}
                config={activityScopeToggleConfig(policyChoiceW)}
              />
            </div>
          </foreignObject>
        </g>
      ) : null}
      {summaryLines.map((line, index) => (
        <text
          key={`manage-workspace-summary:${kind}:${activeTabKey}:${workspaceTargetKey}:${index}`}
          x={workspaceBodyX + 22}
          y={summaryBaseY + index * 16}
          fontSize={12.2}
          fontWeight={740}
          fill={cfg.colors.mutedText}
        >
          {line}
        </text>
      ))}
      {!policyCustomSurfaceMode ? (
        <path
          d={`M ${workspaceBodyX + 22} ${bodyY + bodyHeaderH} H ${workspaceBodyX + workspaceBodyW - 22}`}
          stroke={activeColor}
          strokeWidth={0.9}
          opacity={0.42}
        />
      ) : null}
      {supportContactFormEnabled ? (
        <ManageSupportContactForm
          x={workspaceBodyX + 28}
          y={cardsTop}
          w={workspaceBodyW - 56}
          h={Math.max(260, bodyY + bodyH - cardsTop - 18)}
          cfg={cfg}
        />
      ) : policyCustomSurfaceMode && policyMutationUnavailable ? (
        <UnavailableReadModelSurface
          title={`${policyAreaLabel}: policy overview`}
          subtitle="Current state is never inferred"
          statusLabel={`${policyAreaLabel} policy overview is read only`}
          headline="CURRENT POLICY NOT SHOWN HERE"
          detail="This visual overview does not receive current policy state. Current policy appears only in a supported service-reported policy view when it is available. No rule, approval, schedule, budget, or enforcement setting is inferred here."
          x={workspaceBodyX + 6}
          y={browserRulesChoiceTop}
          w={workspaceBodyW - 16}
          h={Math.min(240, Math.max(1, bodyY + bodyH - browserRulesChoiceTop - 18))}
          cfg={cfg}
        />
      ) : policyRulesChoiceMode ? (
        <PolicyRulesGridGuide
          policyAreaLabel={policyAreaLabel}
          x={workspaceBodyX + 6}
          y={browserRulesChoiceTop}
          w={workspaceBodyW - 16}
          h={Math.max(1, bodyY + bodyH - browserRulesChoiceTop - 18)}
          disabled={policyChoiceDisabled}
          enforcementChoice={browserRulesEnforcementChoice}
          onEnforcementChange={setBrowserRulesEnforcementChoice}
          onInfoClick={() =>
            onNavigate?.(guideRoutePathForManageTab(activeNavLabel, selectedControlName, activeTabKey))
          }
        />
      ) : policyMatrixMode ? (
        <BrowserPolicyTabMatrixSurface
          policyAreaLabel={policyAreaLabel}
          tabId={activeTabKey}
          x={workspaceBodyX + 6}
          y={browserRulesChoiceTop}
          w={workspaceBodyW - 16}
          h={Math.max(1, bodyY + bodyH - browserRulesChoiceTop - 18)}
          disabled={policyChoiceDisabled}
          cfg={cfg}
        />
      ) : kind === 'policy' ? (
        policyRows.map((rowItem, index) => {
          const column = index % policyRowColumns;
          const row = Math.floor(index / policyRowColumns);
          const rowX = workspaceBodyX + 28 + column * (policyRowW + policyRowGap);
          const rowY = policyRowsTop + row * (policyRowH + policyRowGap);
          const rowColor = toneColor(rowItem.tone, cfg);
          const valueSize = fitSingleLineTextSize(rowItem.value, policyRowW - 38, 12, 15, 0.58);
          const bodyLines = wrapCardText(rowItem.body, policyRowW - 36, 10.4, policyRowH > 66 ? 2 : 1);
          return (
            <g key={`manage-policy-setting:${policyAreaLabel}:${activeTabKey}:${workspaceTargetKey}:${rowItem.label}`}>
              <rect
                x={rowX}
                y={rowY}
                width={policyRowW}
                height={policyRowH}
                rx={4}
                fill="rgba(3, 18, 32, 0.68)"
                stroke={rowColor}
                strokeWidth={0.82}
                opacity={0.95}
              />
              <path
                d={`M ${rowX + 8} ${rowY + 6} H ${rowX + policyRowW - 8}`}
                stroke={cfg.colors.bodyText}
                strokeWidth={0.85}
                strokeLinecap="round"
                opacity={0.16}
              />
              <circle cx={rowX + 15} cy={rowY + 17} r={3.1} fill={rowColor} opacity={0.95} />
              <text x={rowX + 26} y={rowY + 19} fontSize={9.6} fontWeight={950} fill={rowColor}>
                {truncateTextForWidth(rowItem.label.toUpperCase(), policyRowW * 0.42, 9.6, 0.58)}
              </text>
              <text
                x={rowX + policyRowW - 14}
                y={rowY + 20}
                textAnchor="end"
                fontSize={valueSize}
                fontWeight={940}
                fill={cfg.colors.bodyText}
              >
                {truncateTextForWidth(rowItem.value, policyRowW * 0.48, valueSize, 0.58)}
              </text>
              {bodyLines.map((line, lineIndex) => (
                <text
                  key={`manage-policy-setting-body:${rowItem.label}:${lineIndex}`}
                  x={rowX + 14}
                  y={rowY + 42 + lineIndex * 14}
                  fontSize={10.4}
                  fontWeight={720}
                  fill={cfg.colors.mutedText}
                >
                  {line}
                </text>
              ))}
            </g>
          );
        })
      ) : (
        cards.map((card, index) => {
          const column = index % cardColumns;
          const row = Math.floor(index / cardColumns);
          const cardX = workspaceBodyX + 28 + column * (cardW + cardGap);
          const cardY = cardsTop + row * (cardH + cardGap);
          const cardColor = toneColor(card.tone, cfg);
          const valueSize = fitSingleLineTextSize(card.value, cardW - 30, 12, 16, 0.58);
          const bodyLines = wrapCardText(card.body, cardW - 30, 10.2, cardH > 86 ? 2 : 1);
          return (
            <g key={`manage-workspace-card:${kind}:${activeTabKey}:${workspaceTargetKey}:${index}:${card.label}`}>
              <rect
                x={cardX}
                y={cardY}
                width={cardW}
                height={cardH}
                rx={3}
                fill={colorAlpha(cardColor, '15')}
                stroke={cardColor}
                strokeWidth={0.82}
                opacity={0.94}
              />
              <path
                d={`M ${cardX + 9} ${cardY + 7} H ${cardX + cardW - 9}`}
                stroke={cfg.colors.bodyText}
                strokeWidth={0.8}
                strokeLinecap="round"
                opacity={0.18}
              />
              <circle cx={cardX + 15} cy={cardY + 18} r={3.2} fill={cardColor} opacity={0.95} />
              <text x={cardX + 26} y={cardY + 20} fontSize={9.6} fontWeight={950} fill={cardColor}>
                {truncateTextForWidth(card.label.toUpperCase(), cardW - 40, 9.6, 0.58)}
              </text>
              <text x={cardX + 14} y={cardY + 44} fontSize={valueSize} fontWeight={950} fill={cfg.colors.bodyText}>
                {truncateTextForWidth(card.value, cardW - 30, valueSize, 0.58)}
              </text>
              {bodyLines.map((line, lineIndex) => (
                <text
                  key={`manage-workspace-card-body:${card.label}:${lineIndex}`}
                  x={cardX + 14}
                  y={cardY + 64 + lineIndex * 14}
                  fontSize={10.2}
                  fontWeight={720}
                  fill={cfg.colors.mutedText}
                >
                  {line}
                </text>
              ))}
            </g>
          );
        })
      )}
    </g>
  );
}

function ManageTargetPanel({
  x,
  y,
  w,
  activeNavLabel,
  selectedControlName,
  spec,
  lane,
  runtimeDeviceSlots,
  themeColor,
  targetSelection,
  onTargetChange,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  activeNavLabel: string;
  selectedControlName: string;
  spec: ManageControlSpec;
  lane: ManageLaneId;
  runtimeDeviceSlots?: readonly DeviceSlot[];
  themeColor?: string;
  targetSelection: ManageTargetSelection;
  onTargetChange: (selection: ManageTargetSelection) => void;
  cfg: ParentPortalSvgControls;
}) {
  const scopeChoices = manageScopeChoicesForLane(lane);
  const browserChoices = manageBrowserTargetsForKey(activeNavLabel, selectedControlName);
  const color = themeColor ?? toneColor(MANAGE_LANES.find((item) => item.id === lane)?.tone ?? 'cyan', cfg);
  const deviceChoices = manageDeviceChoices(spec.devices, runtimeDeviceSlots);
  const browserRowVisible = browserChoices.length > 0;
  const globalChoice = scopeChoices.find((choice) => choice.scope === 'global') ?? scopeChoices[0];
  const perDeviceChoice = scopeChoices.find((choice) => choice.scope === 'perDevice') ?? null;
  const allowPerDevice = lane !== 'portal' && Boolean(perDeviceChoice) && deviceChoices.length > 0;
  const scopeOptions = allowPerDevice
    ? [
        { scope: 'global' as ManageScopeId, label: lane === 'deviceOps' ? 'All devices' : 'Family' },
        { scope: 'perDevice' as ManageScopeId, label: 'Per device' },
      ]
    : [{ scope: 'global' as ManageScopeId, label: globalChoice?.label ?? 'Parent profile' }];
  const scopeTrackW = allowPerDevice ? clampValue(w * 0.26, 220, 315) : clampValue(w * 0.2, 160, 240);
  const scopeTrackX = x + 76;
  const scopeTrackY = y + 12;
  const scopeTrackH = 30;
  const scopeCellW = scopeTrackW / scopeOptions.length;
  const selectedScopeIndex = Math.max(
    0,
    scopeOptions.findIndex((option) => option.scope === targetSelection.scope)
  );
  const targetSummary = manageSelectionLabel(activeNavLabel, selectedControlName, lane, targetSelection);
  const deviceColumns = w > 1050 ? 3 : w > 660 ? 2 : 1;
  const deviceGap = 8;
  const deviceW = (w - 24 - deviceGap * Math.max(0, deviceColumns - 1)) / Math.max(1, deviceColumns);
  const deviceY = y + 68;
  const deviceRows = Math.max(1, Math.ceil(deviceChoices.length / deviceColumns));
  const showDeviceGrid = allowPerDevice && targetSelection.scope === 'perDevice' && deviceChoices.length > 0;
  const browserY = showDeviceGrid ? deviceY + deviceRows * 34 + 28 : y + 68;
  const browserCount = Math.max(1, Math.min(browserChoices.length, 4));
  const browserW = (w - 26 - (browserCount - 1) * 8) / browserCount;
  const compactScopeHeader = w < 520;

  if (assetKey(selectedControlName).includes('remote-access')) {
    return (
      <g role="status" aria-label="Remote access target unavailable">
        <text x={x + 12} y={y + 32} fontSize={12.4} fontWeight={950} fill={color}>
          REMOTE TARGET NOT REPORTED
        </text>
        <text x={x + 12} y={y + 60} fontSize={11.5} fontWeight={780} fill={cfg.colors.mutedText}>
          {truncateTextForWidth(
            'A target appears after an authenticated remote session reports its current child and route.',
            w - 24,
            11.5,
            0.58
          )}
        </text>
      </g>
    );
  }

  return (
    <g>
      <text x={x + 12} y={scopeTrackY + 20} fontSize={12.4} fontWeight={950} fill={color}>
        Scope:
      </text>
      <rect
        x={scopeTrackX}
        y={scopeTrackY}
        width={scopeTrackW}
        height={scopeTrackH}
        rx={15}
        fill="rgba(2, 12, 22, 0.68)"
        stroke={color}
        strokeWidth={0.9}
        strokeOpacity={0.72}
      />
      <rect
        x={scopeTrackX + selectedScopeIndex * scopeCellW + 3}
        y={scopeTrackY + 3}
        width={scopeCellW - 6}
        height={scopeTrackH - 6}
        rx={12}
        fill={colorAlpha(color, '35')}
        stroke={color}
        strokeWidth={1}
        filter="url(#parentPortalGlow)"
      />
      {scopeOptions.map((option, index) => (
        <g
          key={`${selectedControlName}:scope:${option.scope}`}
          className="parent-portal-svg-clickable"
          role="button"
          tabIndex={0}
          aria-label={`Use ${option.label} scope`}
          aria-pressed={targetSelection.scope === option.scope}
          onClick={(event) => {
            event.stopPropagation();
            const selectedSlot = runtimeDeviceSlots
              ? reportSelectedSlot(runtimeDeviceSlots, targetSelection)
              : undefined;
            const nextSelection =
              option.scope === 'perDevice' && selectedSlot
                ? selectedManageTargetSelectionForSlot(targetSelection, selectedSlot)
                : targetSelection;
            onTargetChange({
              ...nextSelection,
              scope: option.scope,
            });
          }}
          onKeyDown={(event) => {
            if (event.key !== 'Enter' && event.key !== ' ') return;
            event.preventDefault();
            event.stopPropagation();
            onTargetChange({ ...targetSelection, scope: option.scope });
          }}
        >
          <rect
            x={scopeTrackX + index * scopeCellW}
            y={scopeTrackY}
            width={scopeCellW}
            height={scopeTrackH}
            fill="transparent"
          />
          <text
            x={scopeTrackX + index * scopeCellW + scopeCellW / 2}
            y={scopeTrackY + 20}
            textAnchor="middle"
            fontSize={11.2}
            fontWeight={950}
            fill={targetSelection.scope === option.scope ? cfg.colors.bodyText : cfg.colors.mutedText}
          >
            {option.label}
          </text>
        </g>
      ))}

      <text
        x={compactScopeHeader ? x + 12 : x + w - 12}
        y={compactScopeHeader ? scopeTrackY + 50 : scopeTrackY + 20}
        textAnchor={compactScopeHeader ? 'start' : 'end'}
        fontSize={11.5}
        fontWeight={900}
        fill={cfg.colors.bodyText}
      >
        {truncateTextForWidth(
          `${manageControlDisplayTitle(spec.title)} / ${targetSummary}`,
          compactScopeHeader ? w - 24 : Math.max(180, w - scopeTrackW - 112),
          11.5,
          0.58
        )}
      </text>

      {showDeviceGrid ? (
        <>
          <text x={x + 12} y={deviceY - 10} fontSize={10.8} fontWeight={950} fill={color}>
            CHILD DEVICE
          </text>
          {deviceChoices.map((choice, index) => {
            const col = index % deviceColumns;
            const row = Math.floor(index / deviceColumns);
            return (
              <ManagePill
                key={`${selectedControlName}:device-target:${choice}`}
                x={x + 12 + col * (deviceW + deviceGap)}
                y={deviceY + row * 34}
                w={deviceW}
                h={28}
                label={choice}
                selected={targetSelection.device === choice}
                tone={index === 1 ? 'gold' : index === 2 ? 'purple' : 'cyan'}
                {...(themeColor === undefined ? {} : { themeColor })}
                onSelect={() => {
                  const selectedSlot = runtimeDeviceSlots?.find(
                    (slot) => slot.label === choice || slot.device?.name === choice
                  );
                  onTargetChange(
                    selectedSlot
                      ? selectedManageTargetSelectionForSlot(targetSelection, selectedSlot)
                      : { ...targetSelection, scope: 'perDevice', device: choice, deviceId: '' }
                  );
                }}
                cfg={cfg}
              />
            );
          })}
        </>
      ) : (
        <text x={x + 12} y={y + 78} fontSize={11.2} fontWeight={780} fill={cfg.colors.mutedText}>
          {truncateTextForWidth(
            lane === 'portal' ? 'Parent portal setting.' : 'Family scope applies without choosing a child device.',
            w - 24,
            11.2,
            0.58
          )}
        </text>
      )}

      {browserRowVisible ? (
        <>
          <text x={x + 12} y={browserY - 10} fontSize={10.5} fontWeight={950} fill={color}>
            BROWSER TARGET
          </text>
          {browserChoices.slice(0, 4).map((choice, index) => (
            <ManagePill
              key={`${selectedControlName}:browser-target:${choice.label}`}
              x={x + 12 + index * (browserW + 8)}
              y={browserY}
              w={browserW}
              h={28}
              label={choice.label}
              selected={targetSelection.browser === choice.label}
              tone={choice.tone}
              {...(themeColor === undefined ? {} : { themeColor })}
              onSelect={() => onTargetChange({ ...targetSelection, browser: choice.label })}
              cfg={cfg}
            />
          ))}
        </>
      ) : null}
    </g>
  );
}

function ManageControlPanel({
  x,
  y,
  w,
  h,
  activeNavLabel,
  selectedControlName,
  spec,
  themeTone,
  themeColor,
  targetSelection,
  onTargetChange,
  activityState,
  parentPortalRows,
  onNavigate,
  onAgentCommand,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  activeNavLabel: string;
  selectedControlName: string;
  spec: ManageControlSpec;
  themeTone: Tone;
  themeColor?: string;
  targetSelection: ManageTargetSelection;
  onTargetChange?: (selection: ManageTargetSelection) => void;
  activityState?: ParentPortalActivityState | null;
  parentPortalRows: ParentPortalRow[];
  onNavigate?: (routePath: string) => void;
  onAgentCommand?: (command: AgentCommandName, payload: Record<string, string>) => void;
  cfg: ParentPortalSvgControls;
}) {
  const lane = manageLaneForKey(activeNavLabel, selectedControlName);
  const [mode, setMode] = useState(spec.modes[0]?.label ?? '');
  const [schedule, setSchedule] = useState('Always');
  const [enabled, setEnabled] = useState(
    () => new Set(spec.options.filter((option) => option.enabled).map((option) => option.label))
  );
  const [lastAction, setLastAction] = useState('Ready');
  const [syncStatus, setSyncStatus] = useState('Local draft');
  const [lanPairingActiveTab, setLanPairingActiveTab] = useState<LanPairingDetailTabId>('info');
  const [lanPairingSelectedSlot, setLanPairingSelectedSlot] = useState<DeviceSlot | null>(null);
  const [lanPairingEditSlot, setLanPairingEditSlot] = useState<DeviceSlot | null>(null);
  const [lanPairingHouseholdNameDraft, setLanPairingHouseholdNameDraft] = useState('');
  const [lanPairingDeviceKindDraft, setLanPairingDeviceKindDraft] = useState<DeviceKind>('unknown');
  const [lanPairingPendingIdentities, setLanPairingPendingIdentities] = useState<LanPairingPendingDeviceIdentities>({});
  const [activityManageActiveTab, setActivityManageActiveTab] = useState<ActivityManageTabId>('reports');
  const [activityReportFrequency, setActivityReportFrequency] = useState<string>(
    PortalAgentActivityReportFrequency.Daily
  );
  const [activityReportOverrideMode, setActivityReportOverrideMode] = useState('family-defaults');
  const [activityReportSelectedFileId, setActivityReportSelectedFileId] = useState<string | null>(null);
  const [manageWorkspaceActiveTab, setManageWorkspaceActiveTab] = useState('');
  const specKey = `${lane}:${spec.title}:${spec.options.map((option) => option.label).join('|')}`;
  useEffect(() => {
    setMode(spec.modes[0]?.label ?? '');
    setSchedule('Always');
    setEnabled(new Set(spec.options.filter((option) => option.enabled).map((option) => option.label)));
    setLastAction('Ready');
    setSyncStatus('Local draft');
    setLanPairingActiveTab('info');
    setLanPairingSelectedSlot(null);
    setLanPairingEditSlot(null);
    setLanPairingHouseholdNameDraft('');
    setLanPairingDeviceKindDraft('unknown');
    setLanPairingPendingIdentities({});
    setActivityManageActiveTab('reports');
    setActivityReportFrequency(PortalAgentActivityReportFrequency.Daily);
    setActivityReportOverrideMode('family-defaults');
    setActivityReportSelectedFileId(null);
    setManageWorkspaceActiveTab('');
  }, [specKey]);
  const compact = w < 560;
  const activeModeTone = spec.modes.find((item) => item.label === mode)?.tone ?? themeTone;
  const color = themeColor ?? toneColor(themeTone, cfg);
  const activeModeColor = themeColor ?? toneColor(activeModeTone, cfg);
  const headerH = compact ? 58 : 48;
  const leftW = compact ? w : Math.max(420, Math.round(w * 0.62));
  const rightW = compact ? w : w - leftW - 18;
  const controlY = y + headerH;
  const compactBodyH = Math.max(1, h - headerH - 12);
  const compactActionMinH = 190;
  const editorH = compact
    ? clampValue(Math.round(compactBodyH * 0.7), 300, Math.max(300, compactBodyH - compactActionMinH - 8))
    : Math.max(280, h - headerH - 10);
  const actionX = compact ? x : x + leftW + 18;
  const actionY = compact ? controlY + editorH + 8 : controlY;
  const actionH = compact ? Math.max(compactActionMinH, compactBodyH - editorH - 8) : Math.max(280, h - headerH - 10);
  const optionCount = compact ? 4 : Math.min(spec.options.length, 6);
  const optionRows = spec.options.slice(0, optionCount);
  const actionRows = spec.actions.slice(0, compact ? 2 : 3);
  const schedules = scheduleOptionsForManageKey(activeNavLabel, selectedControlName);
  const optionColumnCount = compact ? 1 : 2;
  const optionRowsUsed = Math.max(1, Math.ceil(optionRows.length / optionColumnCount));
  const scheduleY = controlY + 118 + optionRowsUsed * 40 + 38;
  const isPortalLane = lane === 'portal';
  const isDeviceOpsLane = lane === 'deviceOps';
  const manageWorkspaceKind = manageWorkspaceKindFor(activeNavLabel, selectedControlName, spec.title);
  const manageWorkspaceDefaultTab = manageWorkspaceKind
    ? manageWorkspaceDefaultTabId(manageWorkspaceKind, activeNavLabel, selectedControlName)
    : '';
  const globalTargetLabel = isPortalLane ? 'Parent profile' : isDeviceOpsLane ? 'All devices' : 'Family';
  const device = targetSelection.scope === 'global' ? globalTargetLabel : targetSelection.device;
  const targetLabel = isPortalLane ? 'Parent profile' : device;
  const selectionLabel = manageSelectionLabel(activeNavLabel, selectedControlName, lane, targetSelection);
  const panelTitle = compact
    ? manageControlDisplayTitle(spec.title)
    : `${manageControlDisplayTitle(spec.title)} / ${selectionLabel}`;
  const titleSize = fitSingleLineTextSize(panelTitle, w - 40, 17, 26, 0.58);
  const targetLabelUpper = targetLabel.toUpperCase();
  const controlsActive = false;
  const applyHeaderLabel = controlsActive
    ? isPortalLane
      ? 'APPLY'
      : `APPLY TO ${targetLabelUpper}`
    : 'CHANGES ARE UNAVAILABLE';
  const applyLabel = controlsActive
    ? isPortalLane
      ? 'Save portal'
      : isDeviceOpsLane
        ? `Send to ${targetLabel}`
        : `Sync ${targetLabel}`
    : 'Apply unavailable';
  const isLanPairingPanel = isLanPairingManageTitle(spec.title);
  const isReportsPanel = isReportsManageTitle(spec.title);
  const isAppGameDashboardPanel = isAppGameDashboardManageContext(activeNavLabel, selectedControlName, spec.title);
  const isRemoteScreenPolicyPanel = assetKey(selectedControlName).includes('remote-screen');
  const isRemoteAccessPanel = assetKey(selectedControlName).includes('remote-access');
  const lanPairingReadModelSlots = useMemo(
    () => createParentPortalLanPairingUiSlots(parentPortalRows, activityState?.lanAddDeviceReadModel),
    [activityState?.lanAddDeviceReadModel, parentPortalRows]
  );
  const lanPairingSlots = useMemo(
    () => applyLanPairingPendingIdentities(lanPairingReadModelSlots, lanPairingPendingIdentities),
    [lanPairingPendingIdentities, lanPairingReadModelSlots]
  );
  const lanPairingPortalIds = useMemo(() => createParentPortalLanPairingPortalIds(lanPairingSlots), [lanPairingSlots]);
  const firstLanPairingSelectableSlot = lanPairingSlots.find((slot) => slot.status !== 'empty') ?? null;
  const preferredLanPairingSelectedSlot = useMemo(
    () => reportSelectedSlot(lanPairingSlots, targetSelection) ?? firstLanPairingSelectableSlot,
    [firstLanPairingSelectableSlot, lanPairingSlots, targetSelection]
  );
  useEffect(() => {
    if (!isLanPairingPanel || !preferredLanPairingSelectedSlot) return;
    if (lanPairingSelectedSlot && lanPairingSlots.some((slot) => slot.value === lanPairingSelectedSlot.value)) return;
    setLanPairingSelectedSlot(preferredLanPairingSelectedSlot);
  }, [isLanPairingPanel, lanPairingSelectedSlot, lanPairingSlots, preferredLanPairingSelectedSlot]);
  useEffect(() => {
    if (!isLanPairingPanel || !lanPairingSelectedSlot) return;
    const selectedDeviceId = lanPairingSelectedSlot.value;
    const selectedDevice = selectedDeviceIdentity(lanPairingSelectedSlot);
    if (!selectedDevice) return;
    if (targetSelection.scope === 'perDevice' && targetSelection.deviceId === selectedDeviceId) return;
    onTargetChange?.(selectedManageTargetSelectionForSlot(targetSelection, lanPairingSelectedSlot));
  }, [isLanPairingPanel, lanPairingSelectedSlot, onTargetChange, targetSelection]);
  const openLanPairingDeviceEditDialog = useCallback(
    (choice: DeviceSlot) => {
      setLanPairingSelectedSlot(choice);
      setLanPairingEditSlot(choice);
      setLanPairingHouseholdNameDraft(lanPairingHouseholdNameDraftFor(choice));
      setLanPairingDeviceKindDraft(lanPairingDeviceKindDraftFor(choice));
      onTargetChange?.(selectedManageTargetSelectionForSlot(targetSelection, choice));
      setLastAction(`${choice.label} edit`);
      setSyncStatus('Editing device identity');
    },
    [onTargetChange, targetSelection]
  );
  const saveLanPairingDeviceEditDialog = useCallback(() => {
    if (!lanPairingEditSlot) return;
    const nextName = lanPairingHouseholdNameDraft.trim() || lanPairingDeviceName(lanPairingEditSlot);
    const payload = lanPairingHouseholdActionCommandPayload(
      lanPairingEditSlot,
      PortalAgentLanHouseholdActionKind.Rename,
      {
        displayName: nextName,
        deviceKind: lanPairingDeviceKindDraft,
        requiresRoute: false,
      }
    );
    if (!payload) {
      setLastAction('Device identity save unavailable');
      setSyncStatus('LAN device identity missing');
      return;
    }
    if (!onAgentCommand) {
      setLastAction('Device identity command unavailable');
      setSyncStatus('Portal command sender missing');
      return;
    }
    const nextIdentity = {
      householdName: nextName,
      detectedName: lanPairingDetectedDeviceName(lanPairingEditSlot),
      deviceKind: lanPairingDeviceKindDraft,
    };
    const identityKey = lanPairingDeviceIdentityKey(lanPairingEditSlot);
    if (identityKey) {
      setLanPairingPendingIdentities((pendingIdentities) => ({
        ...pendingIdentities,
        [identityKey]: nextIdentity,
      }));
    }
    setLanPairingSelectedSlot(applyLanPairingPendingIdentity(lanPairingEditSlot, nextIdentity));
    onAgentCommand(AgentCommand.LanPairingAddDeviceRequest, payload);
    setLanPairingEditSlot(null);
    setLastAction(`${nextName} identity saved`);
    setSyncStatus('LAN household decision sent');
  }, [lanPairingDeviceKindDraft, lanPairingEditSlot, lanPairingHouseholdNameDraft, onAgentCommand]);
  const lanPairingPanelPadX = w < 480 ? 8 : Math.max(18, Math.min(34, Math.round(w * 0.018)));
  const lanPairingPanelPadY = 0;
  const lanPairingAvailableW = Math.max(1, w - lanPairingPanelPadX * 2);
  const lanPairingAvailableH = Math.max(1, h - lanPairingPanelPadY * 2);
  const lanPairingGridTopH = clampValue(
    Math.round(lanPairingAvailableH * 0.42),
    Math.min(250, Math.round(lanPairingAvailableH * 0.48)),
    Math.max(260, lanPairingAvailableH - 360)
  );
  const lanPairingGridW = lanPairingAvailableW;
  const lanPairingGridH = lanPairingGridTopH;
  const lanPairingGridX = x + lanPairingPanelPadX;
  const lanPairingGridY = y + lanPairingPanelPadY;
  const lanPairingDividerY = lanPairingGridY + lanPairingGridTopH;
  const lanPairingGridHostStyle: CSSProperties = {
    width: lanPairingGridW,
    height: lanPairingGridH,
  };
  const lanPairingDetailTabs = useMemo(() => lanPairingDetailTabsFor(lanPairingSelectedSlot), [lanPairingSelectedSlot]);
  useEffect(() => {
    if (lanPairingDetailTabs.some((tab) => tab.id === lanPairingActiveTab)) return;
    setLanPairingActiveTab(lanPairingDetailTabs[0]?.id ?? 'info');
  }, [lanPairingActiveTab, lanPairingDetailTabs]);
  const lanPairingDetailTab =
    lanPairingDetailTabs.find((tab) => tab.id === lanPairingActiveTab) ??
    lanPairingDetailTabs[0] ??
    LAN_PAIRING_DETAIL_TABS[0];
  const lanPairingDetailColor = toneColor(lanPairingDetailTab.tone, cfg);
  const lanPairingDetailY = lanPairingDividerY + 16;
  const lanPairingDetailH = Math.max(1, y + h - lanPairingDetailY - 8);
  const lanPairingTabH = Math.max(34, Math.min(42, Math.round(lanPairingDetailH * 0.15)));
  const lanPairingTabGap = 0;
  const lanPairingTabsCompact = lanPairingGridW < 480;
  const lanPairingTabColumns = lanPairingTabsCompact
    ? Math.min(2, lanPairingDetailTabs.length)
    : lanPairingDetailTabs.length;
  const lanPairingTabRows = Math.max(1, Math.ceil(lanPairingDetailTabs.length / lanPairingTabColumns));
  const lanPairingTabInsetX =
    lanPairingGridW < 480 ? 8 : Math.max(16, Math.min(24, Math.round(lanPairingGridW * 0.012)));
  const lanPairingTabW = Math.max(1, Math.min(180, (lanPairingGridW - lanPairingTabInsetX * 2) / lanPairingTabColumns));
  const lanPairingTabsX = lanPairingGridX + lanPairingTabInsetX;
  const lanPairingBodyY = lanPairingDetailY + lanPairingTabRows * lanPairingTabH - 1;
  const lanPairingBodyH = Math.max(1, y + h - lanPairingBodyY - 8);
  const lanPairingBodyX = lanPairingGridX;
  const lanPairingBodyW = lanPairingGridW;
  const lanPairingEditDialogW = Math.max(320, Math.min(440, lanPairingBodyW - 72));
  const lanPairingEditDialogH = 312;
  const lanPairingEditDialogX = x + (w - lanPairingEditDialogW) / 2;
  const lanPairingEditDialogY = y + Math.max(24, (h - lanPairingEditDialogH) / 2);
  const lanPairingDetailColumnCount = lanPairingBodyW > 980 ? 3 : lanPairingBodyW > 620 ? 2 : 1;
  const lanPairingDetailRowGap = 10;
  const lanPairingDetailRowW = Math.max(
    120,
    (lanPairingBodyW - 40 - lanPairingDetailRowGap * (lanPairingDetailColumnCount - 1)) / lanPairingDetailColumnCount
  );
  const lanPairingDetailRowH = 44;
  const lanPairingDetailRows = lanPairingDetailRowsFor(lanPairingDetailTab.id, lanPairingSelectedSlot);
  const lanPairingContextRows = lanPairingContextRowsFor(lanPairingSelectedSlot);
  const lanPairingActionButtons =
    lanPairingDetailTab.id === 'pair'
      ? lanPairingPairActionButtonsFor(lanPairingSelectedSlot)
      : lanPairingDetailTab.id === 'update' &&
          !lanPairingDetailTabUnavailableReason(lanPairingDetailTab.id, lanPairingSelectedSlot)
        ? lanPairingActionButtonsFor(lanPairingSelectedSlot)
        : [];
  const lanPairingContextGap = 10;
  const lanPairingContextColumns = lanPairingBodyW > 760 ? 4 : 2;
  const lanPairingContextRowH = 38;
  const lanPairingContextRowW = Math.max(
    118,
    (lanPairingBodyW - 40 - lanPairingContextGap * (lanPairingContextColumns - 1)) / lanPairingContextColumns
  );
  const lanPairingContextY = lanPairingBodyY + 42;
  const lanPairingCanEditSelectedDevice = lanPairingCanEditDeviceIdentity(lanPairingSelectedSlot);
  const lanPairingEditButtonW = 146;
  const lanPairingEditButtonH = 28;
  const lanPairingEditButtonX = lanPairingBodyX + lanPairingBodyW - lanPairingEditButtonW - 20;
  const lanPairingEditButtonY = lanPairingBodyY + 8;
  const lanPairingActionGap = 8;
  const lanPairingActionColumns = lanPairingBodyW > 1120 ? 7 : lanPairingBodyW > 760 ? 4 : 2;
  const lanPairingActionRowH = 31;
  const lanPairingActionW = Math.max(
    88,
    (lanPairingBodyW - 40 - lanPairingActionGap * (lanPairingActionColumns - 1)) / lanPairingActionColumns
  );
  const lanPairingActionY =
    lanPairingContextY +
    Math.ceil(lanPairingContextRows.length / lanPairingContextColumns) * (lanPairingContextRowH + 8) +
    4;
  const lanPairingActionRowCount = Math.ceil(lanPairingActionButtons.length / lanPairingActionColumns);
  const lanPairingDetailRowTop = lanPairingActionY + lanPairingActionRowCount * (lanPairingActionRowH + 6) + 10;
  const lanPairingDetailRowsBottom = lanPairingBodyY + lanPairingBodyH - 20;
  const lanPairingDetailVisibleCount = Math.max(
    1,
    Math.floor(Math.max(1, lanPairingDetailRowsBottom - lanPairingDetailRowTop) / (lanPairingDetailRowH + 8)) *
      lanPairingDetailColumnCount
  );
  const lanPairingUnavailableLines = wrapCardText(
    'Connect the local service to scan and show current LAN devices.',
    lanPairingGridW - 32,
    11.5,
    2
  );
  const lanPairingVisibleRows = lanPairingDetailRows.slice(0, lanPairingDetailVisibleCount);
  const reportPlanSeatLimit = ACTIVITY_REPORT_BASIC_CHILD_DEVICE_SEATS;
  const activityUiIntent = useMemo(
    () => createParentPortalActivityUiIntent(activityState, reportPlanSeatLimit),
    [activityState, reportPlanSeatLimit]
  );
  const appGameDashboard = activityUiIntent.appGameDashboard;
  const runtimeDeviceSlots = useMemo(
    () => createParentPortalCanonicalDeviceSlots(activityUiIntent.deviceSlots, lanPairingSlots),
    [activityUiIntent.deviceSlots, lanPairingSlots]
  );
  const reportScopeValue = targetSelection.scope === 'perDevice' ? 'device' : 'family';
  const reportFamilyScope = reportScopeValue !== 'device';
  const reportSelectionSlots = runtimeDeviceSlots;
  const reportSlots = useMemo(
    () => (reportSelectionSlots.length > 0 ? reportSelectionSlots : reportPlanSeatSlots(reportPlanSeatLimit)),
    [reportPlanSeatLimit, reportSelectionSlots]
  );
  const reportPortalIds = useMemo(
    () => reportSlots.filter((slot) => slot.device).map((slot) => slot.value),
    [reportSlots]
  );
  const reportSelectedValue =
    reportScopeValue === 'device' ? (reportSelectedSlotValue(reportSlots, targetSelection) ?? '') : '';
  const reportSelectedDeviceSlot = reportSlots.find((slot) => slot.value === reportSelectedValue) ?? null;
  const reportScopeStatus = activityReportScopeStatus(reportFamilyScope, reportSelectionSlots.length);
  const activityReportFiles = activityUiIntent.reportFiles;
  const activityReportSelectedFile =
    activityReportFiles.find((file) => file.id === activityReportSelectedFileId) ?? activityReportFiles[0] ?? null;
  const activityReportViewerReport = activityReportSelectedFile?.report ?? null;
  const reportPanelPadX = w < 480 ? 8 : Math.max(18, Math.min(34, Math.round(w * 0.018)));
  const reportPanelPadY = 0;
  const reportAvailableW = Math.max(1, w - reportPanelPadX * 2);
  const reportAvailableH = Math.max(1, h - reportPanelPadY * 2);
  const reportDeviceCount = Math.max(1, reportSlots.length);
  const reportGridColumnsByWidth = Math.max(
    1,
    Math.floor(
      (Math.max(1, reportAvailableW - 44) + MANAGE_DEVICE_GRID_GAP_X) /
        (MANAGE_DEVICE_GRID_CELL_W + MANAGE_DEVICE_GRID_GAP_X)
    )
  );
  const reportGridColumns = Math.max(1, Math.min(reportDeviceCount, reportGridColumnsByWidth));
  const reportGridRows = reportFamilyScope ? 1 : Math.max(1, Math.ceil(reportDeviceCount / reportGridColumns));
  const reportCompactSelectorH = Math.max(
    176,
    Math.min(reportAvailableH * 0.42, ACTIVITY_REPORT_SELECTOR_BASE_H + reportGridRows * ACTIVITY_REPORT_SELECTOR_ROW_H)
  );
  const reportTopH = Math.max(1, reportCompactSelectorH);
  const reportTopX = x + reportPanelPadX;
  const reportTopY = y + reportPanelPadY;
  const reportDividerY = reportTopY + reportTopH;
  const reportSelectorY = reportTopY;
  const reportSelectorH = reportTopH;
  const reportGridHostStyle: CSSProperties = {
    width: reportAvailableW,
    height: reportSelectorH,
    position: 'relative',
  };
  const activityManageTab =
    ACTIVITY_MANAGE_TABS.find((tab) => tab.id === activityManageActiveTab) ?? ACTIVITY_MANAGE_TABS[0];
  const effectiveActivityManageTab =
    activityDetailTabUnavailableReason(activityManageTab, reportFamilyScope, reportSelectedDeviceSlot) !== null
      ? ACTIVITY_MANAGE_TABS[0]
      : activityManageTab;
  const effectiveActivityManageTabId = effectiveActivityManageTab.id;
  const activityManageTabColor = toneColor(effectiveActivityManageTab.tone, cfg);
  const reportScopeStatusStyle: CSSProperties = {
    position: 'absolute',
    inset: w < 480 ? '76px 8px 42px' : '76px 24px 42px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'column',
    gap: 6,
    padding: w < 480 ? '8px 10px' : '10px 18px',
    border: `1px solid ${colorAlpha(activityManageTabColor, '40')}`,
    borderRadius: 8,
    background: w < 480 ? 'rgba(3, 18, 32, 0.94)' : colorAlpha(activityManageTabColor, '12'),
    color: cfg.colors.bodyText,
    textAlign: 'center',
    pointerEvents: 'none',
  };
  const activityBodyY = reportDividerY + 16;
  const activityBodyAvailableH = Math.max(1, y + h - activityBodyY - 8);
  const activityTabsCompact = reportAvailableW < 560;
  const activityTabColumns = reportAvailableW < 480 ? 2 : activityTabsCompact ? 3 : ACTIVITY_MANAGE_TABS.length;
  const activityTabRows = Math.ceil(ACTIVITY_MANAGE_TABS.length / activityTabColumns);
  const activityTabH = activityTabsCompact ? 30 : Math.max(34, Math.min(42, Math.round(activityBodyAvailableH * 0.15)));
  const activityTabGap = 0;
  const activityTabInsetX =
    reportAvailableW < 480 ? 8 : Math.max(16, Math.min(24, Math.round(reportAvailableW * 0.012)));
  const activityTabMinW = reportAvailableW < 760 ? 74 : 92;
  const activityTabW = Math.max(
    activityTabsCompact ? 58 : activityTabMinW,
    Math.min(168, (reportAvailableW - activityTabInsetX * 2) / activityTabColumns)
  );
  const activityTabsX = reportTopX + activityTabInsetX;
  const activityBodyPanelY = activityBodyY + activityTabRows * activityTabH - 1;
  const activityBodyPanelH = Math.max(1, y + h - activityBodyPanelY - 8);
  const activityBodyPanelX = reportTopX;
  const activityBodyPanelW = reportAvailableW;
  const activityReportsSelected = effectiveActivityManageTabId === 'reports';
  const activityReportInnerX = activityBodyPanelX + 18;
  const activityReportInnerY = activityBodyPanelY + (activityReportsSelected ? 2 : 18);
  const activityReportInnerW = Math.max(1, activityBodyPanelW - 36);
  const activityReportInnerH = Math.max(1, activityBodyPanelY + activityBodyPanelH - activityReportInnerY - 16);
  const activityReportFrequencyOption =
    ACTIVITY_REPORT_FREQUENCY_OPTIONS.find((option) => option.value === activityReportFrequency) ??
    ACTIVITY_REPORT_DAILY_OPTION;
  const activityReportFrequencyLabel = activityReportFrequencyOption.label;
  const activityReportOverrideLabel = reportFamilyScope
    ? 'Family'
    : (ACTIVITY_REPORT_OVERRIDE_OPTIONS.find((option) => option.value === activityReportOverrideMode)?.label ??
      'Default');
  const activityReportOverrideActive = activityReportOverrideMode === 'override';
  const activityControlStacked = !reportFamilyScope && activityReportInnerW < 620;
  const activityReportControlBarH = activityReportsSelected ? (activityControlStacked ? 116 : 64) : 0;
  const activityReportControlBarX = activityReportInnerX;
  const activityReportControlBarY = activityReportInnerY;
  const activityReportControlBarW = activityReportInnerW;
  const activityReportControlPadX = 14;
  const activityReportControlGap = 12;
  const activityReportToggleAvailableW = Math.max(1, activityReportControlBarW - activityReportControlPadX * 2);
  const activityReportSingleToggleW = Math.min(520, activityReportToggleAvailableW);
  const activityReportSplitToggleW = activityControlStacked
    ? activityReportToggleAvailableW
    : Math.max(1, (activityReportToggleAvailableW - activityReportControlGap) / 2);
  const activityReportOverrideToggleX = activityReportControlBarX + activityReportControlPadX;
  const activityReportFrequencyToggleX = reportFamilyScope
    ? activityReportControlBarX + (activityReportControlBarW - activityReportSingleToggleW) / 2
    : activityControlStacked
      ? activityReportOverrideToggleX
      : activityReportOverrideToggleX + activityReportSplitToggleW + activityReportControlGap;
  const activityReportOverrideToggleY = activityReportControlBarY - 5;
  const activityReportFrequencyToggleY = reportFamilyScope
    ? activityReportControlBarY - 5
    : activityControlStacked
      ? activityReportControlBarY + 49
      : activityReportControlBarY - 5;
  const activityReportFrequencyToggleW = reportFamilyScope ? activityReportSingleToggleW : activityReportSplitToggleW;
  const activityReportFrequencyTitle = activityReportFrequencyToggleW < 220 ? 'Freq.' : 'Frequency';
  const activityReportFrequencyDisabled = !reportFamilyScope && !activityReportOverrideActive;
  const activityReportSplitY = activityReportInnerY + activityReportControlBarH;
  const activityReportSplitH = Math.max(1, activityReportInnerY + activityReportInnerH - activityReportSplitY);
  const activityReportStacked = activityReportInnerW < 620;
  const activityReportStackListMaxH = Math.max(104, activityReportSplitH - 300);
  const activityReportStackListH = activityReportStacked
    ? Math.min(activityReportStackListMaxH, Math.max(104, Math.round(activityReportSplitH * 0.28)))
    : activityReportSplitH;
  const activityReportSidePanelW = activityReportStacked
    ? activityReportInnerW
    : Math.max(220, Math.min(360, activityReportInnerW * 0.28));
  const activityReportVerticalDividerX = activityReportInnerX + activityReportSidePanelW;
  const activityReportHorizontalDividerY = activityReportSplitY + activityReportStackListH;
  const activityReportListX = activityReportInnerX + 12;
  const activityReportListY = activityReportSplitY + 16;
  const activityReportListW = Math.max(
    1,
    activityReportStacked ? activityReportInnerW - 24 : activityReportSidePanelW - 24
  );
  const activityReportViewerX = activityReportStacked ? activityReportInnerX + 12 : activityReportVerticalDividerX + 18;
  const activityReportViewerY = activityReportStacked
    ? activityReportHorizontalDividerY + 16
    : activityReportSplitY + 16;
  const activityReportViewerW = Math.max(1, activityReportInnerX + activityReportInnerW - activityReportViewerX - 12);
  const activityReportActionY = activityReportInnerY + activityReportInnerH - 40;
  const activityReportActionGap = 10;
  const activityReportGenerateButtonW = activityReportStacked
    ? Math.max(1, (activityReportViewerW - activityReportActionGap) / 2)
    : 156;
  const activityReportSaveButtonW = activityReportStacked
    ? Math.max(1, activityReportViewerW - activityReportGenerateButtonW - activityReportActionGap)
    : 126;
  const activityReportRowH = 62;
  const activityReportRowGap = 10;
  const activityReportVisibleRows = activityReportFiles.slice(
    0,
    Math.max(1, Math.floor(Math.max(1, activityReportStackListH - 52) / (activityReportRowH + activityReportRowGap)))
  );
  const activityReportViewerTarget =
    activityReportViewerReport?.targetLabel ??
    (reportFamilyScope ? 'Family' : (reportSelectedDeviceSlot?.label ?? 'Select a device'));
  const activityReportViewerState = activityReportViewerReport?.saved
    ? `Saved JSON: ${activityReportViewerReport.fileName}`
    : activityReportViewerReport
      ? `Draft JSON: ${activityReportViewerReport.fileName}`
      : 'No saved report read model reported';
  const activityReportViewerSections = activityReportViewerReport?.sections ?? [];
  const activityReportViewerSectionCount = Math.max(
    0,
    Math.min(2, Math.floor(Math.max(0, activityReportActionY - activityReportViewerY - 230) / 66))
  );
  const activityReportVisibleViewerSections = activityReportViewerSections.slice(0, activityReportViewerSectionCount);
  const activityReportGeneratePayload = activityReportScopeCommandPayload(reportFamilyScope, reportSelectedDeviceSlot);
  const activityReportSavePayload = activityReportSaveCommandPayload(activityUiIntent.reportDocument);
  const activityReportGenerateEnabled = Boolean(onAgentCommand && activityReportGeneratePayload);
  const activityReportSaveEnabled = Boolean(onAgentCommand && activityReportSavePayload);
  const requestActivityReportGenerate = () => {
    if (!onAgentCommand || !activityReportGeneratePayload) return;
    setActivityReportSelectedFileId(null);
    onAgentCommand(activityReportFrequencyOption.command, activityReportGeneratePayload);
    setLastAction(`${activityReportFrequencyOption.label} report requested`);
    setSyncStatus('Local service report request sent');
  };
  const requestActivityReportSave = () => {
    if (!onAgentCommand || !activityReportSavePayload) return;
    onAgentCommand(AgentCommand.ActivityReportSave, activityReportSavePayload);
    setLastAction('Activity report save requested');
    setSyncStatus('Local service save request sent');
  };
  const activityMonitorPanelX = activityReportInnerX;
  const activityMonitorPanelY = activityReportInnerY + activityReportControlBarH;
  const activityMonitorPanelW = activityReportInnerW;
  const activityMonitorPanelH = Math.max(1, activityReportInnerY + activityReportInnerH - activityMonitorPanelY);
  const activityMonitorPad = 14;
  const activityMonitorContentX = activityMonitorPanelX + activityMonitorPad;
  const activityMonitorContentY = activityMonitorPanelY + 22;
  const activityMonitorContentW = Math.max(1, activityMonitorPanelW - activityMonitorPad * 2);
  const activityMonitorContentH = Math.max(
    1,
    activityMonitorPanelY + activityMonitorPanelH - activityMonitorContentY - 18
  );
  const activityMonitorRows = activityRowsFromReadModels(
    effectiveActivityManageTabId,
    reportScopeValue,
    reportSelectedDeviceSlot,
    activityReportFrequencyLabel,
    activityReportOverrideLabel,
    syncStatus,
    lastAction,
    activityState
  );
  const activityMonitorRowGap = 10;
  const activityMonitorColumnCount = activityMonitorContentW > 920 ? 2 : 1;
  const activityMonitorRowW = Math.max(
    160,
    (activityMonitorContentW - activityMonitorRowGap * (activityMonitorColumnCount - 1)) / activityMonitorColumnCount
  );
  const activityMonitorRowH = 44;
  const activityMonitorVisibleCount = Math.max(
    1,
    Math.floor(Math.max(1, activityMonitorContentH) / (activityMonitorRowH + activityMonitorRowGap)) *
      activityMonitorColumnCount
  );
  const activityMonitorVisibleRows = activityMonitorRows.slice(0, activityMonitorVisibleCount);

  return (
    <g>
      {isRemoteScreenPolicyPanel ? (
        <UnavailableReadModelSurface
          title="Remote Screen Policy"
          subtitle="No live-view service read model is reported"
          statusLabel="Remote screen policy unavailable"
          headline="REMOTE SCREEN POLICY NOT AVAILABLE"
          detail="No owner-backed live-view session, child capability, permission, route, custody, or current authority is connected. This page does not infer a mode or offer a remote-view request."
          x={x}
          y={y}
          w={w}
          h={Math.min(260, h)}
          cfg={cfg}
        />
      ) : isLanPairingPanel ? (
        <>
          <foreignObject x={lanPairingGridX} y={lanPairingGridY} width={lanPairingGridW} height={lanPairingGridH}>
            <div style={lanPairingGridHostStyle}>
              <DeviceChoiceGrid
                defaultScope="lan"
                defaultPortalDeviceIds={[...lanPairingPortalIds]}
                options={[...lanPairingSlots]}
                parentRows={1}
                parentColumns={LAN_PAIRING_BASIC_PORTAL_SLOT_LIMIT}
                onChange={(choice) => {
                  setLanPairingSelectedSlot(choice);
                  if (choice.status === 'unsupported') {
                    onTargetChange?.({
                      ...selectedManageTargetSelectionForSlot(targetSelection, choice),
                    });
                    setLastAction(`${choice.label} cannot run the child agent`);
                    setSyncStatus('Unsupported LAN device');
                    return;
                  }
                  onTargetChange?.(selectedManageTargetSelectionForSlot(targetSelection, choice));
                  setLastAction(`${choice.label} selected`);
                  setSyncStatus('Draft changed');
                }}
                onAddToPortal={(choice) => {
                  setLanPairingSelectedSlot(choice);
                  onTargetChange?.(selectedManageTargetSelectionForSlot(targetSelection, choice));
                  const payload = lanPairingAddDeviceCommandPayload(choice);
                  if (!payload) {
                    setLastAction(`${choice.label} has no controllable LAN route`);
                    setSyncStatus('Visible only');
                    return;
                  }
                  onAgentCommand?.(AgentCommand.LanPairingAddDeviceRequest, payload);
                  setLastAction(`${choice.label} add-device requested`);
                  setSyncStatus('Pending LAN proof');
                }}
                onEditDevice={openLanPairingDeviceEditDialog}
                showAddControls={false}
                config={manageDeviceGridConfig(lanPairingGridW, lanPairingGridH, {
                  layout: {
                    cellW: 126,
                    cellH: 54,
                    cellMaxW: 188,
                    gapX: 10,
                    selectedInfoH: 42,
                    selectedInfoIconBox: 26,
                    selectedInfoYGap: 10,
                  },
                  text: {
                    optionSize: 13.4,
                    selectedInfoSize: 13.4,
                  },
                })}
              />
            </div>
          </foreignObject>
          {lanPairingSlots.length === 0 ? (
            <g role="status" aria-label="Device discovery unavailable" pointerEvents="none">
              <text
                x={lanPairingGridX + lanPairingGridW / 2}
                y={lanPairingGridY + lanPairingGridH * 0.46}
                textAnchor="middle"
                fontSize={15.5}
                fontWeight={950}
                letterSpacing={0.8}
                fill={cfg.colors.bodyText}
              >
                DEVICE DISCOVERY UNAVAILABLE
              </text>
              {lanPairingUnavailableLines.map((line, index) => (
                <text
                  key={`lan-pairing-unavailable:${index}`}
                  x={lanPairingGridX + lanPairingGridW / 2}
                  y={lanPairingGridY + lanPairingGridH * 0.46 + 24 + index * 15}
                  textAnchor="middle"
                  fontSize={11.5}
                  fontWeight={760}
                  fill={cfg.colors.mutedText}
                >
                  {line}
                </text>
              ))}
            </g>
          ) : null}
          <path
            d={`M ${lanPairingGridX} ${lanPairingDividerY} H ${lanPairingGridX + lanPairingGridW}`}
            stroke={color}
            strokeWidth={3}
            opacity={0.18}
          />
          <path
            d={`M ${lanPairingGridX} ${lanPairingDividerY} H ${lanPairingGridX + lanPairingGridW}`}
            stroke={color}
            strokeWidth={1.35}
            opacity={0.72}
          />
          <g>
            <path
              d={topRoundedRectPath(
                lanPairingBodyX + 1,
                lanPairingBodyY + 1,
                lanPairingBodyW - 2,
                lanPairingBodyH - 2,
                11
              )}
              fill="none"
              stroke={lanPairingDetailColor}
              strokeWidth={3}
              opacity={0.14}
              filter="url(#parentPortalGlow)"
            />
            <path
              d={topRoundedRectPath(lanPairingBodyX, lanPairingBodyY, lanPairingBodyW, lanPairingBodyH, 12)}
              fill={PARENT_PORTAL_GLASS.panelFill}
              stroke={lanPairingDetailColor}
              strokeWidth={1.15}
              opacity={PARENT_PORTAL_CONTENT_SURFACE_OPACITY}
            />
            <path
              d={topRoundedRectPath(
                lanPairingBodyX + cfg.chrome.panelInnerInset,
                lanPairingBodyY + cfg.chrome.panelInnerInset,
                lanPairingBodyW - cfg.chrome.panelInnerInset * 2,
                lanPairingBodyH - cfg.chrome.panelInnerInset * 2,
                8
              )}
              fill="none"
              stroke={lanPairingDetailColor}
              strokeWidth={0.7}
              opacity={0.25}
            />
            <text x={lanPairingBodyX + 20} y={lanPairingBodyY + 15} fontSize={10.4} fontWeight={950} fill={color}>
              SELECTED DEVICE CONTEXT
            </text>
            {lanPairingCanEditSelectedDevice && lanPairingSelectedSlot ? (
              <g
                className="parent-portal-svg-clickable"
                role="button"
                tabIndex={0}
                aria-label={`Edit identity for ${lanPairingDeviceName(lanPairingSelectedSlot)}`}
                onClick={(event) => {
                  event.stopPropagation();
                  openLanPairingDeviceEditDialog(lanPairingSelectedSlot);
                }}
                onKeyDown={(event) => {
                  if (event.key !== 'Enter' && event.key !== ' ') return;
                  event.preventDefault();
                  event.stopPropagation();
                  openLanPairingDeviceEditDialog(lanPairingSelectedSlot);
                }}
              >
                <title>Edit household name and device type</title>
                <path
                  d={cutRectPath(
                    lanPairingEditButtonX,
                    lanPairingEditButtonY,
                    lanPairingEditButtonW,
                    lanPairingEditButtonH,
                    8
                  )}
                  fill={colorAlpha(cfg.colors.gold, '24')}
                  stroke={cfg.colors.gold}
                  strokeWidth={0.95}
                />
                <path
                  d={`M ${lanPairingEditButtonX + 17} ${lanPairingEditButtonY + 18} L ${
                    lanPairingEditButtonX + 22
                  } ${lanPairingEditButtonY + 13} L ${lanPairingEditButtonX + 27} ${
                    lanPairingEditButtonY + 18
                  } L ${lanPairingEditButtonX + 22} ${lanPairingEditButtonY + 23} Z`}
                  fill="none"
                  stroke={cfg.colors.gold}
                  strokeWidth={1.15}
                  strokeLinejoin="round"
                />
                <text
                  x={lanPairingEditButtonX + 38}
                  y={lanPairingEditButtonY + 18}
                  fontSize={11.5}
                  fontWeight={920}
                  fill={cfg.colors.bodyText}
                >
                  Edit identity
                </text>
              </g>
            ) : null}
            {lanPairingContextRows.map((row, index) => {
              const rowColor = toneColor(row.tone, cfg);
              const column = index % lanPairingContextColumns;
              const rowIndex = Math.floor(index / lanPairingContextColumns);
              const rowX = lanPairingBodyX + 20 + column * (lanPairingContextRowW + lanPairingContextGap);
              const rowY = lanPairingContextY + rowIndex * (lanPairingContextRowH + 8);
              return (
                <g key={`lan-pairing-context:${row.label}`}>
                  <path
                    d={cutRectPath(rowX, rowY, lanPairingContextRowW, lanPairingContextRowH, 8)}
                    fill={colorAlpha(rowColor, '12')}
                    stroke={rowColor}
                    strokeWidth={0.82}
                    opacity={0.94}
                  />
                  <text x={rowX + 12} y={rowY + 14} fontSize={9.2} fontWeight={950} fill={rowColor}>
                    {truncateTextForWidth(row.label.toUpperCase(), lanPairingContextRowW - 24, 9.2, 0.58)}
                  </text>
                  <text x={rowX + 12} y={rowY + 29} fontSize={11.2} fontWeight={780} fill={cfg.colors.bodyText}>
                    {truncateTextForWidth(row.value, lanPairingContextRowW - 24, 11.2, 0.58)}
                  </text>
                </g>
              );
            })}
            {lanPairingActionButtons.map((action, index) => {
              const actionEnabled = action.enabled && Boolean(onAgentCommand);
              const actionColor = toneColor(action.tone, cfg);
              const column = index % lanPairingActionColumns;
              const rowIndex = Math.floor(index / lanPairingActionColumns);
              const actionX = lanPairingBodyX + 20 + column * (lanPairingActionW + lanPairingActionGap);
              const actionY = lanPairingActionY + rowIndex * (lanPairingActionRowH + 6);
              const actionOpacity = actionEnabled ? 0.96 : 0.42;
              const actionText = truncateTextForWidth(action.label, lanPairingActionW - 28, 11.2, 0.58);
              const handleLanAction = () => {
                if (!actionEnabled || !action.payload) {
                  setLastAction(`${action.label} unavailable`);
                  setSyncStatus('LAN proof missing');
                  return;
                }
                onAgentCommand?.(action.command, action.payload);
                setLastAction(`${action.label} requested`);
                setSyncStatus('LAN command sent');
              };
              return (
                <g
                  key={`lan-pairing-action:${action.id}`}
                  className={actionEnabled ? 'parent-portal-svg-clickable' : undefined}
                  role="button"
                  tabIndex={actionEnabled ? 0 : -1}
                  aria-disabled={!actionEnabled}
                  aria-label={`LAN ${action.label}`}
                  onClick={(event) => {
                    event.stopPropagation();
                    handleLanAction();
                  }}
                  onKeyDown={(event) => {
                    if (event.key !== 'Enter' && event.key !== ' ') return;
                    event.preventDefault();
                    event.stopPropagation();
                    handleLanAction();
                  }}
                >
                  <title>{action.status}</title>
                  <path
                    d={cutRectPath(actionX, actionY, lanPairingActionW, lanPairingActionRowH, 8)}
                    fill={actionEnabled ? colorAlpha(actionColor, '24') : 'rgba(2, 12, 22, 0.68)'}
                    stroke={actionEnabled ? actionColor : cfg.colors.panelStroke}
                    strokeWidth={0.9}
                    opacity={actionOpacity}
                  />
                  <circle cx={actionX + 14} cy={actionY + 15.5} r={3.5} fill={actionColor} opacity={actionOpacity} />
                  <text
                    x={actionX + 25}
                    y={actionY + 20}
                    fontSize={11.2}
                    fontWeight={900}
                    fill={actionEnabled ? cfg.colors.bodyText : cfg.colors.mutedText}
                  >
                    {actionText}
                  </text>
                </g>
              );
            })}
            {lanPairingVisibleRows.map((row, index) => {
              const rowColor = toneColor(row.tone, cfg);
              const column = index % lanPairingDetailColumnCount;
              const rowIndex = Math.floor(index / lanPairingDetailColumnCount);
              const rowX = lanPairingBodyX + 20 + column * (lanPairingDetailRowW + lanPairingDetailRowGap);
              const rowY = lanPairingDetailRowTop + rowIndex * (lanPairingDetailRowH + 8);
              return (
                <g key={`lan-pairing-detail:${lanPairingDetailTab.id}:${row.label}`}>
                  <path
                    d={cutRectPath(rowX, rowY, lanPairingDetailRowW, lanPairingDetailRowH, 9)}
                    fill={colorAlpha(rowColor, '18')}
                    stroke={rowColor}
                    strokeWidth={0.85}
                    opacity={0.92}
                  />
                  <circle cx={rowX + 15} cy={rowY + 15} r={3.5} fill={rowColor} opacity={0.96} />
                  <text x={rowX + 26} y={rowY + 17} fontSize={10} fontWeight={950} fill={rowColor}>
                    {truncateTextForWidth(row.label.toUpperCase(), lanPairingDetailRowW - 40, 10, 0.58)}
                  </text>
                  <text x={rowX + 14} y={rowY + 34} fontSize={12} fontWeight={800} fill={cfg.colors.bodyText}>
                    {truncateTextForWidth(row.value, lanPairingDetailRowW - 28, 12, 0.58)}
                  </text>
                </g>
              );
            })}
          </g>
          <g role="tablist" aria-label="LAN pairing detail tabs">
            <path
              d={`M ${lanPairingBodyX + 10} ${lanPairingBodyY} H ${lanPairingBodyX + lanPairingBodyW - 10}`}
              stroke={cfg.colors.panelStroke}
              strokeWidth={0.75}
              opacity={0.32}
            />
            {lanPairingDetailTabs.map((tab, index) => {
              const selected = tab.id === lanPairingActiveTab;
              const unavailableReason = lanPairingDetailTabUnavailableReason(tab.id, lanPairingSelectedSlot);
              const muted = Boolean(unavailableReason) && !selected;
              const tabColor = toneColor(tab.tone, cfg);
              const tabColumn = index % lanPairingTabColumns;
              const tabRow = Math.floor(index / lanPairingTabColumns);
              const tabBaseY = lanPairingDetailY + tabRow * lanPairingTabH;
              const tabX = lanPairingTabsX + tabColumn * (lanPairingTabW + lanPairingTabGap);
              const tabY = selected ? tabBaseY : tabBaseY + 5;
              const tabH = selected ? lanPairingTabH + (lanPairingTabsCompact ? 0 : 3) : lanPairingTabH - 5;
              const tabRadius = 0;
              const tabIconSize = lanPairingTabsCompact
                ? Math.max(15, Math.min(19, tabH - 14))
                : Math.max(17, Math.min(22, tabH - 12));
              const tabTextSize = lanPairingTabsCompact ? (selected ? 12.2 : 11.4) : selected ? 13.8 : 12.8;
              const tabText = truncateTextForWidth(tab.label, lanPairingTabW - tabIconSize - 28, tabTextSize, 0.58);
              const tabTextW = Math.min(lanPairingTabW - tabIconSize - 28, tabText.length * tabTextSize * 0.58);
              const tabGroupW = tabIconSize + 7 + tabTextW;
              const tabIconX = tabX + (lanPairingTabW - tabGroupW) / 2;
              const tabIconY = tabY + (tabH - tabIconSize) / 2;
              const TabIcon = tab.icon;
              const tabFill = selected
                ? PARENT_PORTAL_TAB_SURFACE_FILL.lanActive
                : muted
                  ? PARENT_PORTAL_TAB_SURFACE_FILL.lanMuted
                  : PARENT_PORTAL_TAB_SURFACE_FILL.lanIdle;
              const tabStrokeOpacity = selected ? 0.94 : muted ? 0.26 : 0.54;
              return (
                <g
                  key={`lan-pairing-tab:${tab.id}`}
                  className="parent-portal-svg-clickable"
                  role="tab"
                  tabIndex={0}
                  aria-label={`Show LAN pairing ${tab.label}`}
                  aria-selected={selected}
                  onClick={(event) => {
                    event.stopPropagation();
                    setLanPairingActiveTab(tab.id);
                    setLastAction(unavailableReason ?? `${tab.label} tab`);
                    if (unavailableReason) {
                      setSyncStatus('Pairing required');
                    }
                  }}
                  onKeyDown={(event) => {
                    if (event.key !== 'Enter' && event.key !== ' ') return;
                    event.preventDefault();
                    event.stopPropagation();
                    setLanPairingActiveTab(tab.id);
                    setLastAction(unavailableReason ?? `${tab.label} tab`);
                    if (unavailableReason) {
                      setSyncStatus('Pairing required');
                    }
                  }}
                >
                  <title>{unavailableReason ?? `${tab.label} detail`}</title>
                  <rect
                    x={tabX}
                    y={lanPairingTabsCompact ? tabBaseY : tabBaseY - 4}
                    width={lanPairingTabW}
                    height={lanPairingTabsCompact ? lanPairingTabH : lanPairingTabH + 8}
                    fill="transparent"
                  />
                  {selected ? (
                    <rect
                      x={tabX + 1}
                      y={tabY - 2}
                      width={lanPairingTabW - 2}
                      height={tabH + 5}
                      rx={0}
                      fill="none"
                      stroke={tabColor}
                      strokeWidth={2.3}
                      opacity={0.14}
                      filter="url(#parentPortalGlow)"
                    />
                  ) : null}
                  <rect
                    x={tabX}
                    y={tabY}
                    width={lanPairingTabW}
                    height={tabH}
                    rx={tabRadius}
                    fill={tabFill}
                    opacity={selected ? 1 : muted ? 0.42 : 0.78}
                  />
                  <path
                    d={`M ${tabX} ${tabY} H ${tabX + lanPairingTabW}`}
                    stroke={selected ? tabColor : cfg.colors.panelStroke}
                    strokeWidth={selected ? 2.2 : 1}
                    strokeLinecap="round"
                    opacity={selected ? 0.95 : muted ? 0.18 : 0.34}
                  />
                  <path
                    d={`M ${tabX} ${tabY} V ${tabY + tabH}`}
                    stroke={cfg.colors.panelStroke}
                    strokeWidth={0.8}
                    strokeLinecap="round"
                    opacity={index === 0 ? tabStrokeOpacity : 0.22}
                  />
                  <path
                    d={`M ${tabX + lanPairingTabW} ${tabY} V ${tabY + tabH}`}
                    stroke={cfg.colors.panelStroke}
                    strokeWidth={0.8}
                    strokeLinecap="round"
                    opacity={tabStrokeOpacity}
                  />
                  <path
                    d={`M ${tabX + 10} ${tabY + tabH - 3} H ${tabX + lanPairingTabW - 10}`}
                    stroke={tabColor}
                    strokeWidth={selected ? 2.4 : 1.2}
                    strokeLinecap="round"
                    opacity={selected ? 0.95 : muted ? 0.14 : 0.34}
                  />
                  <TabIcon x={tabIconX} y={tabIconY} width={tabIconSize} height={tabIconSize} />
                  <text
                    x={tabIconX + tabIconSize + 7}
                    y={tabY + tabH * 0.62}
                    textAnchor="start"
                    fontSize={tabTextSize}
                    fontWeight={selected ? 950 : 850}
                    fill={selected ? cfg.colors.bodyText : cfg.colors.mutedText}
                    opacity={muted ? 0.54 : 1}
                    pointerEvents="none"
                  >
                    {tabText}
                  </text>
                </g>
              );
            })}
          </g>
          <LanPairingDeviceEditDialog
            cfg={cfg}
            x={lanPairingEditDialogX}
            y={lanPairingEditDialogY}
            w={lanPairingEditDialogW}
            h={lanPairingEditDialogH}
            overlayX={x}
            overlayY={y}
            overlayW={w}
            overlayH={h}
            slot={lanPairingEditSlot}
            detectedName={lanPairingEditSlot ? lanPairingDetectedDeviceName(lanPairingEditSlot) : ''}
            householdName={lanPairingHouseholdNameDraft}
            deviceKind={lanPairingDeviceKindDraft}
            onHouseholdNameChange={setLanPairingHouseholdNameDraft}
            onDeviceKindChange={setLanPairingDeviceKindDraft}
            onSave={saveLanPairingDeviceEditDialog}
            onClose={() => {
              setLanPairingEditSlot(null);
              setLastAction('Device identity edit closed');
            }}
          />
        </>
      ) : isAppGameDashboardPanel ? (
        <ParentPortalAppGameDashboardPanel
          x={x}
          y={y}
          w={w}
          h={h}
          dashboard={appGameDashboard}
          {...(themeColor === undefined ? {} : { themeColor })}
          cfg={cfg}
        />
      ) : isRemoteAccessPanel ? (
        <UnavailableReadModelSurface
          title="Remote Access"
          subtitle="No authenticated session is reported"
          statusLabel="Remote access unavailable"
          headline="REMOTE ACCESS NOT AVAILABLE"
          detail="No owner-backed remote session, trusted target, transport route, or current authority is connected. This page does not infer a current selection or offer a local command draft."
          x={x}
          y={y}
          w={w}
          h={Math.min(260, h)}
          cfg={cfg}
        />
      ) : isReportsPanel ? (
        <>
          <foreignObject x={reportTopX} y={reportSelectorY} width={reportAvailableW} height={reportSelectorH}>
            <div style={reportGridHostStyle}>
              <DeviceChoiceGrid
                scope={reportScopeValue === 'device' ? 'parent' : 'lan'}
                value={reportSelectedValue}
                options={[...reportSlots]}
                portalDeviceIds={[...reportPortalIds]}
                rows={reportGridRows}
                columns={reportGridColumns}
                parentRows={reportGridRows}
                parentColumns={reportGridColumns}
                deviceSelectionDisabled={reportScopeValue !== 'device'}
                scopeIcons={FAMILY_DEVICE_SCOPE_ICONS}
                onScopeChange={(nextScopeValue) => {
                  const nextScope = nextScopeValue === 'parent' ? 'perDevice' : 'global';
                  onTargetChange?.({ ...targetSelection, scope: nextScope });
                  setLastAction(nextScope === 'perDevice' ? 'Per-device reports selected' : 'Family reports selected');
                  setSyncStatus('Report scope changed');
                }}
                onChange={(choice) => {
                  onTargetChange?.(selectedManageTargetSelectionForSlot(targetSelection, choice));
                  setLastAction(`${choice.label} report target`);
                  setSyncStatus('Report target changed');
                }}
                config={manageDeviceGridConfig(reportAvailableW, reportSelectorH, {
                  statusOrder: { lan: ['connected', 'offline', 'empty'], parent: ['connected', 'offline', 'empty'] },
                  text: {
                    scopeOptions: { lan: 'Family', parent: 'Per Device' },
                    selectedInfoLabel: 'Report device',
                    selectedInfoEmptyLabel: reportScopeValue === 'device' ? 'No device selected' : 'Whole family',
                  },
                })}
              />
              {reportScopeStatus ? (
                <div role="status" aria-label={reportScopeStatus.ariaLabel} style={reportScopeStatusStyle}>
                  <span
                    style={{
                      color: activityManageTabColor,
                      fontSize: 13,
                      fontWeight: 950,
                      letterSpacing: '0.08em',
                    }}
                  >
                    {reportScopeStatus.eyebrow}
                  </span>
                  <span style={{ maxWidth: 680, fontSize: 13.5, fontWeight: 760, lineHeight: 1.4 }}>
                    {reportScopeStatus.detail}
                  </span>
                </div>
              ) : null}
            </div>
          </foreignObject>

          <path
            d={`M ${reportTopX} ${reportDividerY} H ${reportTopX + reportAvailableW}`}
            stroke={color}
            strokeWidth={3}
            opacity={0.18}
          />
          <path
            d={`M ${reportTopX} ${reportDividerY} H ${reportTopX + reportAvailableW}`}
            stroke={color}
            strokeWidth={1.35}
            opacity={0.72}
          />
          <g>
            <path
              d={topRoundedRectPath(
                activityBodyPanelX + 1,
                activityBodyPanelY + 1,
                activityBodyPanelW - 2,
                activityBodyPanelH - 2,
                11
              )}
              fill="none"
              stroke={activityManageTabColor}
              strokeWidth={3}
              opacity={0.14}
              filter="url(#parentPortalGlow)"
            />
            <path
              d={topRoundedRectPath(activityBodyPanelX, activityBodyPanelY, activityBodyPanelW, activityBodyPanelH, 12)}
              fill={PARENT_PORTAL_GLASS.panelFill}
              stroke={activityManageTabColor}
              strokeWidth={1.15}
              opacity={PARENT_PORTAL_CONTENT_SURFACE_OPACITY}
            />
            <path
              d={topRoundedRectPath(
                activityBodyPanelX + cfg.chrome.panelInnerInset,
                activityBodyPanelY + cfg.chrome.panelInnerInset,
                activityBodyPanelW - cfg.chrome.panelInnerInset * 2,
                activityBodyPanelH - cfg.chrome.panelInnerInset * 2,
                8
              )}
              fill="none"
              stroke={activityManageTabColor}
              strokeWidth={0.7}
              opacity={0.25}
            />
            {activityReportsSelected && (
              <g>
                {!reportFamilyScope && (
                  <foreignObject
                    x={activityReportOverrideToggleX}
                    y={activityReportOverrideToggleY}
                    width={activityReportSplitToggleW}
                    height={66}
                  >
                    <div style={{ width: activityReportSplitToggleW, height: 66 }}>
                      <ScopeToggle
                        title="Report"
                        value={activityReportOverrideMode}
                        options={ACTIVITY_REPORT_OVERRIDE_OPTIONS}
                        onChange={(nextValue, option) => {
                          setActivityReportOverrideMode(nextValue);
                          setActivityReportSelectedFileId(null);
                          setLastAction(option.label);
                          setSyncStatus('Report override changed');
                        }}
                        config={activityScopeToggleConfig(activityReportSplitToggleW)}
                      />
                    </div>
                  </foreignObject>
                )}
                <foreignObject
                  x={activityReportFrequencyToggleX}
                  y={activityReportFrequencyToggleY}
                  width={activityReportFrequencyToggleW}
                  height={66}
                >
                  <div style={{ width: activityReportFrequencyToggleW, height: 66 }}>
                    <ScopeToggle
                      title={activityReportFrequencyTitle}
                      value={activityReportFrequency}
                      options={ACTIVITY_REPORT_FREQUENCY_OPTIONS}
                      disabled={activityReportFrequencyDisabled}
                      onChange={(nextValue, option) => {
                        setActivityReportFrequency(nextValue);
                        setActivityReportSelectedFileId(null);
                        setMode(option.label);
                        setLastAction(`${option.label} reports`);
                        setSyncStatus('Report frequency changed');
                      }}
                      config={activityScopeToggleConfig(activityReportFrequencyToggleW)}
                    />
                  </div>
                </foreignObject>
              </g>
            )}
            {activityReportsSelected ? (
              <g>
                <path
                  d={`M ${activityReportInnerX} ${activityReportSplitY} H ${activityReportInnerX + activityReportInnerW}`}
                  stroke={activityManageTabColor}
                  strokeWidth={1.15}
                  opacity={0.62}
                />
                {activityReportStacked ? (
                  <path
                    d={`M ${activityReportInnerX} ${activityReportHorizontalDividerY} H ${activityReportInnerX + activityReportInnerW}`}
                    stroke={activityManageTabColor}
                    strokeWidth={1.05}
                    opacity={0.52}
                  />
                ) : (
                  <path
                    d={`M ${activityReportVerticalDividerX} ${activityReportSplitY + 1} V ${activityReportInnerY + activityReportInnerH}`}
                    stroke={activityManageTabColor}
                    strokeWidth={1.05}
                    opacity={0.52}
                  />
                )}
                <text
                  x={activityReportListX}
                  y={activityReportListY - 4}
                  fontSize={11}
                  fontWeight={950}
                  fill={activityManageTabColor}
                >
                  Reports
                </text>
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY - 4}
                  fontSize={11}
                  fontWeight={950}
                  fill={activityManageTabColor}
                >
                  Report viewer
                </text>
                {activityReportVisibleRows.length === 0 ? (
                  <text
                    x={activityReportListX + 10}
                    y={activityReportListY + 32}
                    fontSize={12}
                    fontWeight={760}
                    fill={cfg.colors.mutedText}
                  >
                    No saved activity reports reported
                  </text>
                ) : null}
                {activityReportVisibleRows.map((row, index) => {
                  const selected = row.id === activityReportSelectedFile?.id;
                  const rowColor = selected ? activityManageTabColor : toneColor(row.saved ? 'cyan' : 'gold', cfg);
                  const rowY = activityReportListY + 10 + index * (activityReportRowH + activityReportRowGap);
                  return (
                    <g
                      key={`activity-report-row:${row.id}`}
                      className="parent-portal-svg-clickable"
                      role="button"
                      tabIndex={0}
                      aria-label={`Open ${row.fileName}`}
                      onClick={(event) => {
                        event.stopPropagation();
                        setActivityReportSelectedFileId(row.id);
                        setLastAction(row.fileName);
                        setSyncStatus('Historical report selected');
                      }}
                      onKeyDown={(event) => {
                        if (event.key !== 'Enter' && event.key !== ' ') return;
                        event.preventDefault();
                        event.stopPropagation();
                        setActivityReportSelectedFileId(row.id);
                        setLastAction(row.fileName);
                        setSyncStatus('Historical report selected');
                      }}
                    >
                      {selected ? (
                        <rect
                          x={activityReportListX - 3}
                          y={rowY - 3}
                          width={activityReportListW + 6}
                          height={activityReportRowH + 6}
                          rx={7}
                          fill="none"
                          stroke={rowColor}
                          strokeWidth={2.2}
                          opacity={0.2}
                          filter="url(#parentPortalGlow)"
                        />
                      ) : null}
                      <rect
                        x={activityReportListX}
                        y={rowY}
                        width={activityReportListW}
                        height={activityReportRowH}
                        rx={6}
                        fill={selected ? colorAlpha(rowColor, '20') : 'rgba(2, 12, 22, 0.54)'}
                        stroke={selected ? rowColor : cfg.colors.panelStroke}
                        strokeWidth={selected ? 1 : 0.75}
                        opacity={selected ? 0.98 : 0.76}
                      />
                      <circle cx={activityReportListX + 14} cy={rowY + 14} r={3.3} fill={rowColor} opacity={0.96} />
                      <text x={activityReportListX + 25} y={rowY + 15} fontSize={9.4} fontWeight={950} fill={rowColor}>
                        {truncateTextForWidth(row.fileName, activityReportListW - 38, 9.4, 0.56)}
                      </text>
                      <text
                        x={activityReportListX + 13}
                        y={rowY + 34}
                        fontSize={11.2}
                        fontWeight={850}
                        fill={cfg.colors.bodyText}
                      >
                        {truncateTextForWidth(
                          `${row.dateLabel} / ${row.rangeLabel}`,
                          activityReportListW - 26,
                          11.2,
                          0.56
                        )}
                      </text>
                      <text
                        x={activityReportListX + 13}
                        y={rowY + 52}
                        fontSize={10.2}
                        fontWeight={720}
                        fill={cfg.colors.mutedText}
                      >
                        {truncateTextForWidth(row.summary, activityReportListW - 26, 10.2, 0.56)}
                      </text>
                    </g>
                  );
                })}
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY + 26}
                  fontSize={10}
                  fontWeight={950}
                  fill={activityManageTabColor}
                >
                  SELECTED REPORT
                </text>
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY + 46}
                  fontSize={16}
                  fontWeight={900}
                  fill={cfg.colors.bodyText}
                >
                  {truncateTextForWidth(
                    activityReportViewerReport?.title ?? 'No report selected',
                    activityReportViewerW,
                    16,
                    0.58
                  )}
                </text>
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY + 68}
                  fontSize={12.2}
                  fontWeight={760}
                  fill={cfg.colors.mutedText}
                >
                  {truncateTextForWidth(
                    activityReportViewerReport?.summary ?? activityReportViewerState,
                    activityReportViewerW,
                    12.2,
                    0.58
                  )}
                </text>
                <path
                  d={`M ${activityReportViewerX} ${activityReportViewerY + 92} H ${activityReportViewerX + activityReportViewerW}`}
                  stroke={activityManageTabColor}
                  strokeWidth={0.8}
                  opacity={0.34}
                />
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY + 120}
                  fontSize={10}
                  fontWeight={950}
                  fill={toneColor('gold', cfg)}
                >
                  TARGET
                </text>
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY + 139}
                  fontSize={12.4}
                  fontWeight={820}
                  fill={cfg.colors.bodyText}
                >
                  {truncateTextForWidth(activityReportViewerTarget, activityReportViewerW, 12.4, 0.58)}
                </text>
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY + 166}
                  fontSize={10}
                  fontWeight={950}
                  fill={toneColor('purple', cfg)}
                >
                  STATE
                </text>
                <text
                  x={activityReportViewerX}
                  y={activityReportViewerY + 185}
                  fontSize={12.4}
                  fontWeight={820}
                  fill={cfg.colors.bodyText}
                >
                  {truncateTextForWidth(activityReportViewerState, activityReportViewerW, 12.4, 0.58)}
                </text>
                {activityReportVisibleViewerSections.map((section, index) => {
                  const sectionY = activityReportViewerY + 218 + index * 66;
                  return (
                    <g key={`activity-report-viewer-section:${section.title}`}>
                      <text
                        x={activityReportViewerX}
                        y={sectionY}
                        fontSize={10}
                        fontWeight={950}
                        fill={activityManageTabColor}
                      >
                        {section.title.toUpperCase()}
                      </text>
                      <text
                        x={activityReportViewerX}
                        y={sectionY + 20}
                        fontSize={12.1}
                        fontWeight={760}
                        fill={cfg.colors.bodyText}
                      >
                        {truncateTextForWidth(section.lines[0] ?? '', activityReportViewerW, 12.1, 0.56)}
                      </text>
                      <text
                        x={activityReportViewerX}
                        y={sectionY + 40}
                        fontSize={11.1}
                        fontWeight={700}
                        fill={cfg.colors.mutedText}
                      >
                        {truncateTextForWidth(section.lines[1] ?? '', activityReportViewerW, 11.1, 0.56)}
                      </text>
                    </g>
                  );
                })}
                <g
                  className={activityReportGenerateEnabled ? 'parent-portal-svg-clickable' : undefined}
                  role="button"
                  tabIndex={activityReportGenerateEnabled ? 0 : -1}
                  aria-label={`Generate ${activityReportFrequencyLabel} activity report`}
                  aria-disabled={!activityReportGenerateEnabled}
                  opacity={activityReportGenerateEnabled ? 1 : 0.44}
                  onClick={
                    activityReportGenerateEnabled
                      ? (event) => {
                          event.stopPropagation();
                          requestActivityReportGenerate();
                        }
                      : undefined
                  }
                  onKeyDown={
                    activityReportGenerateEnabled
                      ? (event) => {
                          if (event.key !== 'Enter' && event.key !== ' ') return;
                          event.preventDefault();
                          event.stopPropagation();
                          requestActivityReportGenerate();
                        }
                      : undefined
                  }
                >
                  <title>
                    {activityReportGenerateEnabled
                      ? `Generate ${activityReportFrequencyLabel} report from the local service`
                      : reportFamilyScope
                        ? 'Connect the local service to generate a report'
                        : 'Select a current device to generate a report'}
                  </title>
                  <rect
                    x={activityReportViewerX}
                    y={activityReportActionY}
                    width={activityReportGenerateButtonW}
                    height={30}
                    rx={7}
                    fill={colorAlpha(activityManageTabColor, '22')}
                    stroke={activityManageTabColor}
                    strokeWidth={1}
                    opacity={0.72}
                  />
                  <text
                    x={activityReportViewerX + activityReportGenerateButtonW / 2}
                    y={activityReportActionY + 20}
                    textAnchor="middle"
                    fontSize={11.5}
                    fontWeight={900}
                    fill={cfg.colors.bodyText}
                  >
                    Generate
                  </text>
                </g>
                <g
                  className={activityReportSaveEnabled ? 'parent-portal-svg-clickable' : undefined}
                  role="button"
                  tabIndex={activityReportSaveEnabled ? 0 : -1}
                  aria-label="Save generated activity report"
                  aria-disabled={!activityReportSaveEnabled}
                  opacity={activityReportSaveEnabled ? 1 : 0.44}
                  onClick={
                    activityReportSaveEnabled
                      ? (event) => {
                          event.stopPropagation();
                          requestActivityReportSave();
                        }
                      : undefined
                  }
                  onKeyDown={
                    activityReportSaveEnabled
                      ? (event) => {
                          if (event.key !== 'Enter' && event.key !== ' ') return;
                          event.preventDefault();
                          event.stopPropagation();
                          requestActivityReportSave();
                        }
                      : undefined
                  }
                >
                  <title>
                    {activityReportSaveEnabled
                      ? 'Save this report through the local service'
                      : 'Generate a report before saving it'}
                  </title>
                  <rect
                    x={activityReportViewerX + activityReportGenerateButtonW + activityReportActionGap}
                    y={activityReportActionY}
                    width={activityReportSaveButtonW}
                    height={30}
                    rx={7}
                    fill={colorAlpha(toneColor('gold', cfg), '20')}
                    stroke={toneColor('gold', cfg)}
                    strokeWidth={1}
                  />
                  <text
                    x={
                      activityReportViewerX +
                      activityReportGenerateButtonW +
                      activityReportActionGap +
                      activityReportSaveButtonW / 2
                    }
                    y={activityReportActionY + 20}
                    textAnchor="middle"
                    fontSize={11.5}
                    fontWeight={900}
                    fill={cfg.colors.bodyText}
                  >
                    Save
                  </text>
                </g>
              </g>
            ) : (
              <g>
                <path
                  d={topRoundedRectPath(
                    activityMonitorPanelX,
                    activityMonitorPanelY,
                    activityMonitorPanelW,
                    activityMonitorPanelH,
                    10
                  )}
                  fill={colorAlpha(activityManageTabColor, '0f')}
                  stroke={activityManageTabColor}
                  strokeWidth={0.95}
                  opacity={0.94}
                />
                <text
                  x={activityMonitorContentX}
                  y={activityMonitorPanelY + 17}
                  fontSize={11}
                  fontWeight={950}
                  fill={activityManageTabColor}
                >
                  {activityManageTab.label.toUpperCase()} EVIDENCE
                </text>
                <path
                  d={`M ${activityMonitorContentX} ${activityMonitorContentY - 7} H ${activityMonitorContentX + activityMonitorContentW}`}
                  stroke={activityManageTabColor}
                  strokeWidth={0.85}
                  opacity={0.42}
                />
                {activityMonitorVisibleRows.map((row, index) => {
                  const rowColor = toneColor(row.tone, cfg);
                  const column = index % activityMonitorColumnCount;
                  const rowIndex = Math.floor(index / activityMonitorColumnCount);
                  const rowX = activityMonitorContentX + column * (activityMonitorRowW + activityMonitorRowGap);
                  const rowY = activityMonitorContentY + rowIndex * (activityMonitorRowH + activityMonitorRowGap);
                  return (
                    <g key={`activity-evidence-row:${activityManageActiveTab}:${row.label}`}>
                      <rect
                        x={rowX}
                        y={rowY}
                        width={activityMonitorRowW}
                        height={activityMonitorRowH}
                        rx={2}
                        fill="rgba(2, 14, 25, 0.52)"
                        stroke={cfg.colors.panelStroke}
                        strokeWidth={0.72}
                        opacity={0.92}
                      />
                      <circle cx={rowX + 15} cy={rowY + 15} r={3.2} fill={rowColor} opacity={0.96} />
                      <text x={rowX + 26} y={rowY + 17} fontSize={9.8} fontWeight={950} fill={rowColor}>
                        {truncateTextForWidth(row.label.toUpperCase(), activityMonitorRowW - 40, 9.8, 0.58)}
                      </text>
                      <text x={rowX + 14} y={rowY + 34} fontSize={11.8} fontWeight={800} fill={cfg.colors.bodyText}>
                        {truncateTextForWidth(row.value, activityMonitorRowW - 28, 11.8, 0.58)}
                      </text>
                    </g>
                  );
                })}
              </g>
            )}
          </g>
          <g role="tablist" aria-label="Activity detail tabs">
            <path
              d={`M ${activityBodyPanelX + 10} ${activityBodyPanelY} H ${activityBodyPanelX + activityBodyPanelW - 10}`}
              stroke={cfg.colors.panelStroke}
              strokeWidth={0.75}
              opacity={0.32}
            />
            {ACTIVITY_MANAGE_TABS.map((tab, index) => {
              const unavailableReason = activityDetailTabUnavailableReason(
                tab,
                reportFamilyScope,
                reportSelectedDeviceSlot
              );
              const disabledTab = unavailableReason !== null;
              const selected = tab.id === effectiveActivityManageTabId;
              const tabColor = toneColor(tab.tone, cfg);
              const tabPaintColor = disabledTab ? cfg.colors.mutedText : tabColor;
              const tabColumn = index % activityTabColumns;
              const tabRow = Math.floor(index / activityTabColumns);
              const tabBaseY = activityBodyY + tabRow * activityTabH;
              const tabX = activityTabsX + tabColumn * (activityTabW + activityTabGap);
              const tabY = selected ? tabBaseY : tabBaseY + (activityTabsCompact ? 3 : 5);
              const tabH = selected ? activityTabH + 3 : activityTabH - (activityTabsCompact ? 3 : 5);
              const tabIconSize = activityTabsCompact
                ? Math.max(12, Math.min(16, tabH - 10))
                : Math.max(17, Math.min(22, tabH - 12));
              const tabTextSize = activityTabsCompact ? (selected ? 10.4 : 9.9) : selected ? 13.8 : 12.8;
              const tabIconGap = activityTabsCompact ? 5 : 7;
              const tabTextMaxW = Math.max(16, activityTabW - tabIconSize - (activityTabsCompact ? 16 : 28));
              const tabText = truncateTextForWidth(tab.label, tabTextMaxW, tabTextSize, 0.58);
              const tabTextW = Math.min(tabTextMaxW, tabText.length * tabTextSize * 0.58);
              const tabGroupW = tabIconSize + tabIconGap + tabTextW;
              const tabIconX = tabX + (activityTabW - tabGroupW) / 2;
              const tabIconY = tabY + (tabH - tabIconSize) / 2;
              const TabIcon = tab.icon;
              const tabFill = selected
                ? PARENT_PORTAL_TAB_SURFACE_FILL.lanActive
                : disabledTab
                  ? PARENT_PORTAL_TAB_SURFACE_FILL.lanMuted
                  : PARENT_PORTAL_TAB_SURFACE_FILL.lanIdle;
              const tabStrokeOpacity = selected ? 0.94 : disabledTab ? 0.26 : 0.54;
              return (
                <g
                  key={`activity-tab:${tab.id}`}
                  className={disabledTab ? undefined : 'parent-portal-svg-clickable'}
                  role="tab"
                  tabIndex={disabledTab ? -1 : 0}
                  aria-label={unavailableReason ?? `Show activity ${tab.label}`}
                  aria-disabled={disabledTab}
                  aria-selected={selected}
                  onClick={(event) => {
                    event.stopPropagation();
                    if (disabledTab) {
                      setLastAction(unavailableReason);
                      return;
                    }
                    setActivityManageActiveTab(tab.id);
                    setLastAction(`${tab.label} tab`);
                  }}
                  onKeyDown={(event) => {
                    if (event.key !== 'Enter' && event.key !== ' ') return;
                    event.preventDefault();
                    event.stopPropagation();
                    if (disabledTab) {
                      setLastAction(unavailableReason);
                      return;
                    }
                    setActivityManageActiveTab(tab.id);
                    setLastAction(`${tab.label} tab`);
                  }}
                >
                  <rect x={tabX} y={tabBaseY - 4} width={activityTabW} height={activityTabH + 8} fill="transparent" />
                  {selected ? (
                    <rect
                      x={tabX + 1}
                      y={tabY - 2}
                      width={activityTabW - 2}
                      height={tabH + 5}
                      rx={0}
                      fill="none"
                      stroke={tabPaintColor}
                      strokeWidth={2.3}
                      opacity={0.14}
                      filter="url(#parentPortalGlow)"
                    />
                  ) : null}
                  <rect
                    x={tabX}
                    y={tabY}
                    width={activityTabW}
                    height={tabH}
                    rx={0}
                    fill={tabFill}
                    opacity={disabledTab ? 0.42 : selected ? 1 : 0.78}
                  />
                  <path
                    d={`M ${tabX} ${tabY} H ${tabX + activityTabW}`}
                    stroke={selected ? tabPaintColor : cfg.colors.panelStroke}
                    strokeWidth={selected ? 2.2 : 1}
                    strokeLinecap="round"
                    opacity={selected ? 0.95 : disabledTab ? 0.18 : 0.34}
                  />
                  <path
                    d={`M ${tabX} ${tabY} V ${tabY + tabH}`}
                    stroke={cfg.colors.panelStroke}
                    strokeWidth={0.8}
                    strokeLinecap="round"
                    opacity={tabColumn === 0 ? tabStrokeOpacity : 0.22}
                  />
                  <path
                    d={`M ${tabX + activityTabW} ${tabY} V ${tabY + tabH}`}
                    stroke={cfg.colors.panelStroke}
                    strokeWidth={0.8}
                    strokeLinecap="round"
                    opacity={tabStrokeOpacity}
                  />
                  <path
                    d={`M ${tabX + 10} ${tabY + tabH - 3} H ${tabX + activityTabW - 10}`}
                    stroke={tabPaintColor}
                    strokeWidth={selected ? 2.4 : 1.2}
                    strokeLinecap="round"
                    opacity={disabledTab ? 0.14 : selected ? 0.95 : 0.34}
                  />
                  <TabIcon x={tabIconX} y={tabIconY} width={tabIconSize} height={tabIconSize} />
                  <text
                    x={tabIconX + tabIconSize + tabIconGap}
                    y={tabY + tabH * 0.62}
                    textAnchor="start"
                    fontSize={tabTextSize}
                    fontWeight={selected ? 950 : 850}
                    fill={disabledTab ? cfg.colors.mutedText : selected ? cfg.colors.bodyText : cfg.colors.mutedText}
                    opacity={disabledTab ? 0.56 : 1}
                    pointerEvents="none"
                  >
                    {tabText}
                  </text>
                </g>
              );
            })}
          </g>
        </>
      ) : manageWorkspaceKind ? (
        <ManageWorkspacePanel
          x={x}
          y={y}
          w={w}
          h={h}
          kind={manageWorkspaceKind}
          activeTabId={manageWorkspaceActiveTab}
          defaultTabId={manageWorkspaceDefaultTab}
          onTabChange={(tabId) => {
            setManageWorkspaceActiveTab(tabId);
            setLastAction(`${tabId} tab`);
            setSyncStatus('Workspace tab changed');
          }}
          {...(onNavigate === undefined ? {} : { onNavigate })}
          activeNavLabel={activeNavLabel}
          selectedControlName={selectedControlName}
          runtimeDeviceSlots={runtimeDeviceSlots}
          sharedTargetSelection={targetSelection}
          {...(onTargetChange === undefined ? {} : { onSharedTargetChange: onTargetChange })}
          cfg={cfg}
        />
      ) : (
        <>
          <text x={x} y={y + 24} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
            {truncateTextForWidth(panelTitle, w - 40, titleSize, 0.58)}
          </text>
          <path d={`M ${x} ${y + 39} H ${x + w}`} stroke={color} strokeWidth={1.1} opacity={0.55} />
          <SurfacePanel
            x={x}
            y={controlY}
            w={leftW}
            h={editorH}
            tone={themeTone}
            {...(themeColor === undefined ? {} : { accentColor: themeColor })}
            cfg={cfg}
          >
            <text x={x + 18} y={controlY + 28} fontSize={10} fontWeight={950} fill={activeModeColor}>
              {isDeviceOpsLane ? 'COMMAND PREVIEW' : 'SETTING PREVIEW'}
            </text>
            {spec.modes.map((item, index) => (
              <ManageModeButton
                key={`${spec.title}:mode:${item.label}`}
                x={x + 18 + index * ((leftW - 42) / Math.min(3, spec.modes.length))}
                y={controlY + 44}
                w={(leftW - 52) / Math.min(3, spec.modes.length)}
                h={36}
                item={item}
                selected={mode === item.label}
                disabled={!controlsActive}
                {...(themeColor === undefined ? {} : { themeColor })}
                onSelect={() => {
                  setMode(item.label);
                  setLastAction(`${item.label} selected`);
                  setSyncStatus('Draft changed');
                }}
                cfg={cfg}
              />
            ))}

            <text x={x + 18} y={controlY + 102} fontSize={10} fontWeight={950} fill={color}>
              SETTING CHOICES
            </text>
            {optionRows.map((option, index) => {
              const optionW = compact ? leftW - 36 : (leftW - 48) / 2;
              const optionX = x + 18 + (compact ? 0 : (index % 2) * (optionW + 12));
              const optionY = controlY + 118 + Math.floor(index / optionColumnCount) * 40;
              const controlOption = { ...option, detail: '' };
              return (
                <ManageToggle
                  key={`${spec.title}:option:${option.label}`}
                  x={optionX}
                  y={optionY}
                  w={optionW}
                  h={34}
                  option={controlOption}
                  selected={enabled.has(option.label)}
                  disabled={!controlsActive}
                  {...(themeColor === undefined ? {} : { themeColor })}
                  onToggle={() => {
                    if (!controlsActive) return;
                    setEnabled((current) => {
                      const next = new Set(current);
                      if (next.has(option.label)) next.delete(option.label);
                      else next.add(option.label);
                      return next;
                    });
                    setLastAction(`${option.label} changed`);
                    setSyncStatus('Draft changed');
                  }}
                  cfg={cfg}
                />
              );
            })}

            {!compact && schedules.length && scheduleY + 44 < controlY + editorH ? (
              <>
                <text x={x + 18} y={scheduleY} fontSize={10} fontWeight={950} fill={color}>
                  WHEN THIS APPLIES
                </text>
                {schedules.slice(0, 6).map((item, index) => {
                  const chipW = (leftW - 68) / 6;
                  return (
                    <ManagePill
                      key={`${spec.title}:schedule:${item.label}`}
                      x={x + 18 + index * (chipW + 6)}
                      y={scheduleY + 16}
                      w={chipW}
                      h={26}
                      label={item.label}
                      selected={schedule === item.label}
                      disabled={!controlsActive}
                      tone={item.tone}
                      {...(themeColor === undefined ? {} : { themeColor })}
                      onSelect={() => {
                        if (!controlsActive) return;
                        setSchedule(item.label);
                        setLastAction(`${item.label} schedule`);
                        setSyncStatus('Draft changed');
                      }}
                      cfg={cfg}
                    />
                  );
                })}
              </>
            ) : null}
          </SurfacePanel>

          <SurfacePanel
            x={actionX}
            y={actionY}
            w={rightW}
            h={actionH}
            tone={themeTone}
            {...(themeColor === undefined ? {} : { accentColor: themeColor })}
            cfg={cfg}
          >
            <text x={actionX + 18} y={actionY + 27} fontSize={10} fontWeight={950} fill={color}>
              {truncateTextForWidth(applyHeaderLabel, rightW - 36, 10, 0.58)}
            </text>
            <ManageActionButton
              x={actionX + 18}
              y={actionY + 43}
              w={rightW - 36}
              h={38}
              action={{ label: 'Validate Draft', detail: '', tone: 'cyan' }}
              disabled={!controlsActive}
              {...(themeColor === undefined ? {} : { themeColor })}
              onSelect={() => {
                setLastAction('Draft validated');
                setSyncStatus('Validated locally');
              }}
              cfg={cfg}
            />
            <ManageActionButton
              x={actionX + 18}
              y={actionY + 89}
              w={rightW - 36}
              h={38}
              action={{ label: applyLabel, detail: '', tone: 'gold' }}
              disabled={!controlsActive}
              {...(themeColor === undefined ? {} : { themeColor })}
              onSelect={() => {
                setLastAction(applyLabel);
                setSyncStatus(isPortalLane ? 'Saved in portal draft' : `Pending sync to ${targetLabel}`);
              }}
              cfg={cfg}
            />
            <ManageActionButton
              x={actionX + 18}
              y={actionY + 135}
              w={rightW - 36}
              h={38}
              action={{ label: 'Revert', detail: '', tone: 'red' }}
              disabled={!controlsActive}
              {...(themeColor === undefined ? {} : { themeColor })}
              onSelect={() => {
                setEnabled(new Set(spec.options.filter((option) => option.enabled).map((option) => option.label)));
                setMode(spec.modes[0]?.label ?? '');
                setSchedule('Always');
                setLastAction('Reverted');
                setSyncStatus('Local draft');
              }}
              cfg={cfg}
            />
            {!compact ? (
              <>
                <text x={actionX + 18} y={actionY + 195} fontSize={10} fontWeight={950} fill={color}>
                  SHORTCUTS
                </text>
                {actionRows.map((action, index) => (
                  <ManageActionButton
                    key={`${spec.title}:action:${action.label}`}
                    x={actionX + 18}
                    y={actionY + 211 + index * 44}
                    w={rightW - 36}
                    h={34}
                    action={{ ...action, detail: '' }}
                    disabled={!controlsActive}
                    {...(themeColor === undefined ? {} : { themeColor })}
                    onSelect={() => {
                      setLastAction(action.label);
                      setSyncStatus(
                        action.label.toLowerCase().includes('open') ? 'Navigated locally' : 'Pending Rust sync'
                      );
                    }}
                    cfg={cfg}
                  />
                ))}
                <path
                  d={`M ${actionX + 18} ${actionY + actionH - 82} H ${actionX + rightW - 18}`}
                  stroke={cfg.colors.panelStroke}
                  strokeWidth={0.8}
                  opacity={0.65}
                />
                <text x={actionX + 18} y={actionY + actionH - 58} fontSize={10} fontWeight={950} fill={color}>
                  CURRENT SELECTION
                </text>
                <text
                  x={actionX + 18}
                  y={actionY + actionH - 36}
                  fontSize={12}
                  fontWeight={900}
                  fill={cfg.colors.bodyText}
                >
                  {truncateTextForWidth(`${selectionLabel} / ${mode}`, rightW - 36, 12, 0.58)}
                </text>
                <text
                  x={actionX + 18}
                  y={actionY + actionH - 16}
                  fontSize={10.5}
                  fontWeight={760}
                  fill={cfg.colors.mutedText}
                >
                  {truncateTextForWidth(
                    controlsActive ? `${syncStatus}: ${lastAction}` : 'Connect the local service to make changes.',
                    rightW - 36,
                    10.5,
                    0.58
                  )}
                </text>
              </>
            ) : null}
          </SurfacePanel>
        </>
      )}
    </g>
  );
}

function AssistantModeBoard({
  x,
  y,
  w,
  h,
  actionsVisible,
  selectedAction,
  selectedActionSequence,
  threadSequence,
  commandAvailable,
  response,
  onChoiceSelect,
  onAssistantMessage,
  onActionToggle,
  onOpenSetup,
  onClose,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  actionsVisible: boolean;
  selectedAction: AssistantQuickAction | null;
  selectedActionSequence: number;
  threadSequence: number;
  commandAvailable: boolean;
  response: ParentPortalAssistantResponse | null;
  onChoiceSelect: (choice: AssistantQuickChoice) => void;
  onAssistantMessage: (payload: Record<string, string>) => void;
  onActionToggle: () => void;
  onOpenSetup: () => void;
  onClose: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [collapsedMessages, setCollapsedMessages] = useState<Record<string, boolean>>({});
  const [draftPrompt, setDraftPrompt] = useState('');
  const [currentAction, setCurrentAction] = useState<AssistantQuickAction | null>(null);
  const [currentChoice, setCurrentChoice] = useState<AssistantQuickChoice | null>(null);
  const [composerSplitOffset, setComposerSplitOffset] = useState(0);
  const [questionnaireSplitOffset, setQuestionnaireSplitOffset] = useState(0);
  const [resizingComposer, setResizingComposer] = useState(false);
  const [resizingQuestionnaire, setResizingQuestionnaire] = useState(false);
  const [awaitingResponse, setAwaitingResponse] = useState(false);
  const messageSequenceRef = useRef(1);
  const handledActionSequenceRef = useRef(selectedActionSequence);
  const handledResponseIdRef = useRef(response?.eventId ?? null);
  const chatClipId = useId().replace(/[^a-zA-Z0-9_-]/g, '');
  const commandAvailableRef = useRef(commandAvailable);
  commandAvailableRef.current = commandAvailable;
  const createReadyMessage = useCallback(
    (): AssistantTranscriptMessage => assistantReadyMessage(commandAvailableRef.current),
    []
  );
  const nextMessageId = useCallback((prefix: string) => `${prefix}-${messageSequenceRef.current++}`, []);
  const [messages, setMessages] = useState<AssistantTranscriptMessage[]>(() => [createReadyMessage()]);

  useEffect(() => {
    messageSequenceRef.current = 1;
    handledActionSequenceRef.current = 0;
    setMessages([createReadyMessage()]);
    setCurrentAction(null);
    setCurrentChoice(null);
    setDraftPrompt('');
    setCollapsedMessages({});
    setQuestionnaireSplitOffset(0);
    setAwaitingResponse(false);
  }, [createReadyMessage, threadSequence]);

  useEffect(() => {
    setMessages((current) =>
      current.map((message) => (message.id === 'mia-ready' ? assistantReadyMessage(commandAvailable) : message))
    );
    if (!commandAvailable) {
      setAwaitingResponse(false);
    }
  }, [commandAvailable]);

  useEffect(() => {
    if (response === null || response.eventId === handledResponseIdRef.current) return;
    handledResponseIdRef.current = response.eventId;
    setAwaitingResponse(false);
    setMessages((current) => [
      ...current,
      {
        id: nextMessageId(`mia-${response.kind}`),
        sender: 'assistant',
        text: response.text,
      },
    ]);
  }, [nextMessageId, response]);

  useEffect(() => {
    if (!selectedAction || selectedActionSequence <= handledActionSequenceRef.current) return;
    handledActionSequenceRef.current = selectedActionSequence;
    setCurrentAction(selectedAction);
    setCurrentChoice(null);
    setQuestionnaireSplitOffset(0);
  }, [selectedAction, selectedActionSequence]);

  const compactHeader = w < 640;
  const headerH = compactHeader ? 104 : 62;
  const pad = 18;
  const composerMinH = 52;
  const composerBottomInset = 8;
  const composerDividerGap = 8;
  const chatX = x + pad;
  const chatY = y + headerH + 12;
  const chatW = w - pad * 2;
  const composerSideInset = clampNumber(chatW * 0.018, 24, 38);
  const composerX = chatX + composerSideInset;
  const composerW = chatW - composerSideInset * 2;
  const defaultBottomReserve = composerMinH + composerBottomInset + composerDividerGap;
  const minBottomReserve = composerMinH + composerBottomInset + 18;
  const maxBottomReserve = Math.max(minBottomReserve, Math.min(260, h - headerH - 190));
  const bottomReserve = clampNumber(defaultBottomReserve + composerSplitOffset, minBottomReserve, maxBottomReserve);
  const splitterY = y + h - bottomReserve;
  const composerH = Math.max(composerMinH, bottomReserve - composerBottomInset - composerDividerGap);
  const composerY = y + h - composerH - composerBottomInset;
  const chatH = Math.max(180, splitterY - chatY);
  const headerCenterY = y + (compactHeader ? 30 : 34);
  const headerDividerY = y + headerH - 2;
  const sideToggleSize = compactHeader ? 30 : 36;
  const sideToggleX = x + (compactHeader ? 18 : 32);
  const sideToggleY = compactHeader ? y + 15 : headerDividerY - sideToggleSize;
  const closeButtonW = compactHeader ? 64 : 78;
  const closeButtonH = 24;
  const closeButtonX = compactHeader ? x + w - closeButtonW - 16 : x + w - closeButtonW - 42;
  const closeButtonY = compactHeader ? y + 18 : headerDividerY - closeButtonH;
  const recoveryButtonW = compactHeader ? Math.min(142, w - 32) : 142;
  const recoveryButtonX = compactHeader ? x + (w - recoveryButtonW) / 2 : x + w - closeButtonW - recoveryButtonW - 54;
  const recoveryButtonY = headerDividerY - closeButtonH;
  const titleGroupW = compactHeader ? 92 : 176;
  const pageCenterX = cfg.canvas.width / 2;
  const titleCenterX = compactHeader
    ? x + w / 2
    : clampNumber(pageCenterX, sideToggleX + sideToggleSize + titleGroupW / 2 + 28, x + w - 96 - titleGroupW / 2);
  const titleIconSize = compactHeader ? 28 : 32;
  const titleIconX = titleCenterX - titleGroupW / 2;
  const titleTextX = titleIconX + titleIconSize + (compactHeader ? 8 : 10);
  const titleUnderlineX = titleIconX - 2;
  const titleUnderlineW = titleGroupW + 4;
  const questionnaire = assistantQuestionnaireState(currentAction, currentChoice);
  const followUpPanelW = chatW;
  const questionnaireMaxH = clampNumber(chatH * 0.3, 104, Math.max(104, chatH - 90));
  const questionnaireBaseH =
    questionnaire.options.length > 0
      ? assistantFollowUpPanelHeight(followUpPanelW, questionnaire.options, questionnaireMaxH)
      : 0;
  const questionnaireMinH = questionnaire.options.length > 0 ? Math.min(questionnaireBaseH, 92) : 0;
  const followUpPanelH =
    questionnaire.options.length > 0
      ? clampNumber(questionnaireBaseH + questionnaireSplitOffset, questionnaireMinH, questionnaireMaxH)
      : 0;
  const followUpY = chatY + chatH - followUpPanelH;
  const firstBubbleY = chatY + 20;
  const messageGap = 16;
  const messageAvailableH = Math.max(90, followUpY - firstBubbleY - 18);
  const messageScrollRailEndY = Math.max(chatY + 54, followUpPanelH > 0 ? followUpY - 20 : chatY + chatH - 22);
  const messageScrollThumbEndY = Math.min(messageScrollRailEndY, chatY + 118);
  const messageLayouts = messages.map((message) => {
    const variant: 'incoming' | 'outgoing' = message.sender === 'user' ? 'outgoing' : 'incoming';
    const config =
      variant === 'outgoing' ? ASSISTANT_OUTGOING_CHAT_BUBBLE_CONFIG : ASSISTANT_INCOMING_CHAT_BUBBLE_CONFIG;
    const collapsed = Boolean(collapsedMessages[message.id]);
    const choices = collapsed ? undefined : message.choices;
    const bubbleW = assistantSmartBubbleWidth({
      text: message.text,
      availableW: chatW,
      variant,
      hasChoices: Boolean(choices?.length),
    });
    const choicesExtra = choices?.length ? assistantBubbleChoiceBodyHeight(bubbleW, choices.length) : 0;
    const bubbleH = assistantChatBubbleHeight(bubbleW, message.text, collapsed, config, choicesExtra);
    return { message, variant, collapsed, choices, bubbleW, bubbleH };
  });
  const messageTotalH = messageLayouts.reduce(
    (total, layout, index) => total + layout.bubbleH + (index === 0 ? 0 : messageGap),
    0
  );
  const showMessageScrollRail = messageTotalH > messageAvailableH + 4;
  const messageOffsetY = Math.min(0, messageAvailableH - messageTotalH);
  let nextMessageY = firstBubbleY + messageOffsetY;
  const messagePositions = messageLayouts.map((layout) => {
    const bubbleY = nextMessageY;
    nextMessageY += layout.bubbleH + messageGap;
    return { ...layout, y: bubbleY };
  });
  const updateComposerSplitFromPointer = (event: PointerEvent<SVGGElement>) => {
    const svg = event.currentTarget.ownerSVGElement;
    const rect = svg?.getBoundingClientRect();
    if (!rect || rect.height <= 0) return;
    const svgY = (event.clientY - rect.top) * (cfg.canvas.height / rect.height);
    const nextBottomReserve = y + h - svgY;
    setComposerSplitOffset(clampNumber(nextBottomReserve, minBottomReserve, maxBottomReserve) - defaultBottomReserve);
  };
  const updateQuestionnaireSplitFromPointer = (event: PointerEvent<SVGGElement>) => {
    const svg = event.currentTarget.ownerSVGElement;
    const rect = svg?.getBoundingClientRect();
    if (!rect || rect.height <= 0 || questionnaire.options.length === 0) return;
    const svgY = (event.clientY - rect.top) * (cfg.canvas.height / rect.height);
    const nextPanelH = chatY + chatH - svgY;
    setQuestionnaireSplitOffset(clampNumber(nextPanelH, questionnaireMinH, questionnaireMaxH) - questionnaireBaseH);
  };
  const toggleCollapsed = (messageId: string) => {
    setCollapsedMessages((current) => ({ ...current, [messageId]: !current[messageId] }));
  };
  const copyMessageText = (text: string) => {
    if (!navigator?.clipboard?.writeText) return;
    void navigator.clipboard.writeText(text).catch(() => undefined);
  };
  const emitAssistantMessage = (
    prompt: string,
    inputSource: 'typed' | 'choice',
    choice: AssistantQuickChoice | null = currentChoice,
    action: AssistantQuickAction | null = currentAction
  ) => {
    const cleanedPrompt = prompt.trim();
    if (!cleanedPrompt || !commandAvailable) return;
    onAssistantMessage(assistantMessageCommandPayload(cleanedPrompt, action, choice, inputSource));
    setAwaitingResponse(true);
  };
  const selectFollowUp = (option: AssistantQuestionnaireOption) => {
    const cleanedPrompt = option.prompt.trim();
    if (!cleanedPrompt || !commandAvailable) return;
    if (option.choice) {
      setCurrentChoice(option.choice);
      setQuestionnaireSplitOffset(0);
      onChoiceSelect(option.choice);
    }
    setMessages((current) => [
      ...current,
      {
        id: nextMessageId('you-followup'),
        sender: 'user',
        text: option.label,
        action: currentAction,
      },
    ]);
    if (!option.choice) {
      emitAssistantMessage(cleanedPrompt, 'typed', currentChoice, currentAction);
    }
  };
  const selectMainChoice = (choice: AssistantQuickChoice, action: AssistantQuickAction | null = currentAction) => {
    if (!commandAvailable) return;
    const actionForChoice = action ?? currentAction;
    setCurrentAction(actionForChoice);
    setCurrentChoice(choice);
    setQuestionnaireSplitOffset(0);
    onChoiceSelect(choice);
    setMessages((current) => [
      ...current,
      {
        id: nextMessageId('you-choice'),
        sender: 'user',
        text: choice.prompt,
        action: actionForChoice,
      },
    ]);
    emitAssistantMessage(choice.prompt, 'choice', choice, actionForChoice);
  };
  const sendDraftPrompt = () => {
    const cleanedPrompt = draftPrompt.trim();
    if (!cleanedPrompt || !commandAvailable) return;
    setMessages((current) => [
      ...current,
      {
        id: nextMessageId('you-typed'),
        sender: 'user',
        text: cleanedPrompt,
        action: currentAction,
      },
    ]);
    setDraftPrompt('');
    emitAssistantMessage(cleanedPrompt, 'typed', currentChoice, currentAction);
  };
  return (
    <g>
      <AssistantChatFrame x={x} y={y} w={w} h={h} underlineX={titleUnderlineX} underlineW={titleUnderlineW} cfg={cfg}>
        <foreignObject x={sideToggleX} y={sideToggleY} width={sideToggleSize} height={sideToggleSize}>
          <AnimatedSidebarIconButton
            isOpen={actionsVisible}
            size={sideToggleSize}
            centerPreview={false}
            title={actionsVisible ? 'Hide action panel' : 'Show action panel'}
            onClick={onActionToggle}
            className="parent-portal-assistant-sidepanel-toggle"
            config={ASSISTANT_SIDE_PANEL_ICON_CONFIG}
          />
        </foreignObject>
        <AiMemorySetBrainIcon
          x={titleIconX}
          y={headerCenterY - titleIconSize / 2}
          width={titleIconSize}
          height={titleIconSize}
          color={cfg.colors.cyan}
        />
        <text
          x={titleTextX}
          y={headerCenterY + 6}
          fontSize={compactHeader ? 15 : 17}
          fontWeight={980}
          fill={cfg.colors.bodyText}
          data-ocentra-assistant-header-title="true"
        >
          {compactHeader ? 'MIA' : 'AI ASSISTANT'}
        </text>
        <AssistantCloseButton
          x={closeButtonX}
          y={closeButtonY}
          w={closeButtonW}
          h={closeButtonH}
          ariaLabel="Close parent assistant"
          onSelect={onClose}
          cfg={cfg}
        />
        {!commandAvailable ? (
          <AssistantRecoveryButton
            x={recoveryButtonX}
            y={recoveryButtonY}
            w={recoveryButtonW}
            h={closeButtonH}
            onSelect={onOpenSetup}
            cfg={cfg}
          />
        ) : null}

        <rect
          x={chatX}
          y={chatY}
          width={chatW}
          height={chatH}
          rx={14}
          fill={ASSISTANT_CHAT_SURFACE_FILL}
          stroke={cfg.colors.panelStroke}
          strokeWidth={0.85}
        />
        <defs>
          <clipPath id={`${chatClipId}-assistantChat`}>
            <rect x={chatX} y={chatY} width={chatW} height={chatH} rx={14} />
          </clipPath>
        </defs>
        <g clipPath={`url(#${chatClipId}-assistantChat)`}>
          {messagePositions.map(({ message, variant, collapsed, choices, bubbleW, y: bubbleY }) => (
            <AssistantChatBubble
              key={message.id}
              id={message.id}
              x={variant === 'outgoing' ? chatX + chatW - bubbleW - 22 : chatX + 22}
              y={bubbleY}
              w={bubbleW}
              senderLabel={message.sender === 'user' ? 'YOU' : 'MIA'}
              text={message.text}
              variant={variant}
              collapsed={collapsed}
              onCollapsedChange={() => toggleCollapsed(message.id)}
              onCopy={() => copyMessageText(message.text)}
              {...(choices === undefined ? {} : { choices })}
              {...(message.choiceActionLabel === undefined ? {} : { choiceActionLabel: message.choiceActionLabel })}
              {...(message.action === undefined || message.action === null
                ? {}
                : { onChoiceSelect: (choice: AssistantQuickChoice) => selectMainChoice(choice, message.action) })}
            />
          ))}
        </g>
        {showMessageScrollRail ? (
          <>
            <path
              d={`M ${chatX + chatW - 13} ${chatY + 22} V ${messageScrollRailEndY}`}
              stroke={cfg.colors.cyan}
              strokeWidth={1.2}
              strokeLinecap="round"
              opacity={0.32}
            />
            <path
              d={`M ${chatX + chatW - 13} ${chatY + 40} V ${messageScrollThumbEndY}`}
              stroke={cfg.colors.cyan}
              strokeWidth={3.2}
              strokeLinecap="round"
              opacity={0.82}
              filter="url(#parentPortalGlow)"
            />
          </>
        ) : null}
        {questionnaire.options.length > 0 ? (
          <>
            <g clipPath={`url(#${chatClipId}-assistantChat)`}>
              <AssistantFollowUpPanel
                x={chatX}
                y={followUpY}
                w={followUpPanelW}
                h={followUpPanelH}
                question={questionnaire.question}
                options={questionnaire.options}
                disabled={!commandAvailable}
                onSelect={selectFollowUp}
                cfg={cfg}
              />
            </g>
            <AssistantComposerSplitter
              x={chatX + 6}
              y={followUpY}
              w={chatW - 12}
              dragging={resizingQuestionnaire}
              ariaLabel="Resize assistant questionnaire"
              onPointerDown={(event) => {
                event.preventDefault();
                setResizingQuestionnaire(true);
                event.currentTarget.setPointerCapture?.(event.pointerId);
                updateQuestionnaireSplitFromPointer(event);
              }}
              onPointerMove={(event) => {
                if (!resizingQuestionnaire) return;
                updateQuestionnaireSplitFromPointer(event);
              }}
              onPointerUp={(event) => {
                setResizingQuestionnaire(false);
                event.currentTarget.releasePointerCapture?.(event.pointerId);
              }}
              onPointerCancel={() => setResizingQuestionnaire(false)}
              cfg={cfg}
            />
          </>
        ) : null}
        <AssistantComposerSplitter
          x={composerX}
          y={splitterY}
          w={composerW}
          dragging={resizingComposer}
          ariaLabel="Resize assistant chat composer"
          onPointerDown={(event) => {
            event.preventDefault();
            setResizingComposer(true);
            event.currentTarget.setPointerCapture?.(event.pointerId);
            updateComposerSplitFromPointer(event);
          }}
          onPointerMove={(event) => {
            if (!resizingComposer) return;
            updateComposerSplitFromPointer(event);
          }}
          onPointerUp={(event) => {
            setResizingComposer(false);
            event.currentTarget.releasePointerCapture?.(event.pointerId);
          }}
          onPointerCancel={() => setResizingComposer(false)}
          cfg={cfg}
        />
        <AssistantComposer
          x={composerX}
          y={composerY}
          w={composerW}
          h={composerH}
          prompt={draftPrompt}
          disabled={!commandAvailable}
          awaitingResponse={awaitingResponse}
          onPromptChange={setDraftPrompt}
          onSend={sendDraftPrompt}
          cfg={cfg}
        />
      </AssistantChatFrame>
    </g>
  );
}

type AssistantChatFrameProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  underlineX: number;
  underlineW: number;
  children: ReactNode;
  cfg: ParentPortalSvgControls;
};

type AssistantHeaderDividerProps = {
  x: number;
  y: number;
  w: number;
  underlineX: number;
  underlineW: number;
  cfg: ParentPortalSvgControls;
};

type AssistantCloseButtonProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  ariaLabel: string;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
};

type AssistantRecoveryButtonProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
};

type AssistantChatBubbleConfig =
  | typeof ASSISTANT_INCOMING_CHAT_BUBBLE_CONFIG
  | typeof ASSISTANT_OUTGOING_CHAT_BUBBLE_CONFIG;

type AssistantChatBubbleProps = {
  id: string;
  x: number;
  y: number;
  w: number;
  senderLabel: string;
  text: string;
  variant: 'incoming' | 'outgoing';
  collapsed?: boolean;
  onCollapsedChange: () => void;
  onCopy: () => void;
  choices?: readonly AssistantQuickChoice[];
  choiceActionLabel?: string;
  onChoiceSelect?: (choice: AssistantQuickChoice) => void;
};

type AssistantChatBubbleBodyProps = {
  text: string;
  choices?: readonly AssistantQuickChoice[];
  choiceColumnCount: number;
  choiceActionLabel?: string;
  onChoiceSelect?: (choice: AssistantQuickChoice) => void;
};

type AssistantFollowUpPanelProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  question: string;
  options: readonly AssistantQuestionnaireOption[];
  disabled: boolean;
  onSelect: (option: AssistantQuestionnaireOption) => void;
  cfg: ParentPortalSvgControls;
};

type AssistantComposerSplitterProps = {
  x: number;
  y: number;
  w: number;
  dragging: boolean;
  ariaLabel: string;
  onPointerDown: (event: PointerEvent<SVGGElement>) => void;
  onPointerMove: (event: PointerEvent<SVGGElement>) => void;
  onPointerUp: (event: PointerEvent<SVGGElement>) => void;
  onPointerCancel: (event: PointerEvent<SVGGElement>) => void;
  cfg: ParentPortalSvgControls;
};

type AssistantComposerProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  prompt: string;
  disabled: boolean;
  awaitingResponse: boolean;
  onPromptChange: (value: string) => void;
  onSend: () => void;
  cfg: ParentPortalSvgControls;
};

type ManagePillProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  label: string;
  selected: boolean;
  disabled?: boolean;
  tone: Tone;
  themeColor?: string;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
};

type ManageModeButtonProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  item: ManageControlAction;
  selected: boolean;
  disabled?: boolean;
  themeColor?: string;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
};

type ManageToggleProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  option: ManageControlOption;
  selected: boolean;
  disabled?: boolean;
  themeColor?: string;
  onToggle: () => void;
  cfg: ParentPortalSvgControls;
};

type ManageActionButtonProps = {
  x: number;
  y: number;
  w: number;
  h: number;
  action: ManageControlAction;
  disabled?: boolean;
  themeColor?: string;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
};

function AssistantChatFrame({ x, y, w, h, underlineX, underlineW, children, cfg }: AssistantChatFrameProps) {
  const headerH = 62;
  return (
    <g>
      <ParentPortalPanelFrame
        x={x}
        y={y}
        w={w}
        h={h}
        color={cfg.colors.cyan}
        active
        fill={ASSISTANT_CHAT_SURFACE_FILL}
      />
      <AssistantHeaderDivider
        x={x}
        y={y + headerH - 2}
        w={w}
        underlineX={underlineX}
        underlineW={underlineW}
        cfg={cfg}
      />
      {children}
    </g>
  );
}

function AssistantHeaderDivider({ x, y, w, underlineX, underlineW, cfg }: AssistantHeaderDividerProps) {
  const handleW = clampNumber(underlineW, 112, 186);
  const handleX = clampNumber(underlineX, x + 36, x + w - 36 - handleW);
  const lineStart = x + 36;
  const lineEnd = x + w - 36;
  const handleGap = 8;
  return (
    <g pointerEvents="none">
      <path
        d={`M ${lineStart} ${y} H ${Math.max(lineStart, handleX - handleGap)}`}
        stroke={cfg.colors.cyan}
        strokeWidth={0.55}
        strokeLinecap="round"
        opacity={0.38}
      />
      <path
        d={`M ${Math.min(lineEnd, handleX + handleW + handleGap)} ${y} H ${lineEnd}`}
        stroke={cfg.colors.cyan}
        strokeWidth={0.55}
        strokeLinecap="round"
        opacity={0.38}
      />
      <rect
        x={handleX}
        y={y - 2.25}
        width={handleW}
        height={4.5}
        rx={2.25}
        fill="transparent"
        stroke={colorAlpha(cfg.colors.cyan, 'b0')}
        strokeWidth={0.55}
        filter="url(#parentPortalGlow)"
      />
    </g>
  );
}

function AssistantCloseButton({ x, y, w, h, ariaLabel, onSelect, cfg }: AssistantCloseButtonProps) {
  const [hovered, setHovered] = useState(false);
  const color = hovered ? cfg.colors.red : cfg.colors.cyan;
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={ariaLabel}
      onClick={onSelect}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      <title>{ariaLabel}</title>
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={7}
        fill={colorAlpha(cfg.colors.cyan, '1d')}
        stroke={color}
        strokeWidth={hovered ? 1.25 : 0.85}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={10.5}
        fontWeight={950}
        fill={cfg.colors.bodyText}
      >
        CLOSE
      </text>
    </g>
  );
}

function AssistantRecoveryButton({ x, y, w, h, onSelect, cfg }: AssistantRecoveryButtonProps) {
  const [hovered, setHovered] = useState(false);
  const color = hovered ? cfg.colors.bodyText : cfg.colors.gold;
  const selectWithKeyboard = (event: KeyboardEvent<SVGGElement>) => {
    if (event.key !== 'Enter' && event.key !== ' ') return;
    event.preventDefault();
    onSelect();
  };
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label="Open Start Here to reconnect MIA"
      onClick={onSelect}
      onKeyDown={selectWithKeyboard}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      <title>Open Start Here to reconnect MIA</title>
      <rect
        x={x}
        y={y}
        width={w}
        height={h}
        rx={7}
        fill={colorAlpha(cfg.colors.gold, '24')}
        stroke={color}
        strokeWidth={hovered ? 1.25 : 0.85}
      />
      <text
        x={x + w / 2}
        y={y + h / 2 + 4}
        textAnchor="middle"
        fontSize={9.8}
        fontWeight={950}
        fill={cfg.colors.bodyText}
      >
        OPEN START HERE
      </text>
    </g>
  );
}

function assistantChatBubbleHeight(
  w: number,
  text: string,
  collapsed: boolean,
  config: AssistantChatBubbleConfig,
  extraExpandedHeight = 0
) {
  return estimateChatBubbleHeight({ width: w, text, collapsed, config }) + (collapsed ? 0 : extraExpandedHeight);
}

function assistantSmartBubbleWidth({
  text,
  availableW,
  variant,
  hasChoices = false,
}: {
  text: string;
  availableW: number;
  variant: 'incoming' | 'outgoing';
  hasChoices?: boolean;
}) {
  const usableW = Math.max(220, availableW - 44);
  const maxW = Math.min(usableW, Math.max(260, availableW * 0.86));
  const minW = clampNumber(variant === 'outgoing' ? 230 : 300, 190, maxW);
  const normalizedText = (text || ' ').trim() || ' ';
  const textLines = normalizedText.split(/\r?\n/);
  const longestLineLength = textLines.reduce((longest, line) => Math.max(longest, line.length), 0);
  const longestWordLength = normalizedText.split(/\s+/).reduce((longest, word) => Math.max(longest, word.length), 0);
  const totalLength = normalizedText.length;
  const chromeW = variant === 'outgoing' ? 86 : 94;
  const lineDrivenW = longestLineLength * 5.55 + chromeW;
  const wordDrivenW = longestWordLength * 6.8 + chromeW;
  const paragraphDrivenW = totalLength > 130 ? 540 + (totalLength - 130) * 3.2 : 0;
  const choiceDrivenW = hasChoices ? 620 : 0;
  return clampNumber(Math.max(minW, lineDrivenW, wordDrivenW, paragraphDrivenW, choiceDrivenW), minW, maxW);
}

function AssistantChatBubble({
  id,
  x,
  y,
  w,
  senderLabel,
  text,
  variant,
  collapsed = false,
  onCollapsedChange,
  onCopy,
  choices,
  choiceActionLabel,
  onChoiceSelect,
}: AssistantChatBubbleProps) {
  const config = variant === 'outgoing' ? ASSISTANT_OUTGOING_CHAT_BUBBLE_CONFIG : ASSISTANT_INCOMING_CHAT_BUBBLE_CONFIG;
  const choicesExtra = choices?.length ? assistantBubbleChoiceBodyHeight(w, choices.length) : 0;
  const choiceColumnCount = choices?.length ? assistantBubbleChoiceColumnCount(w) : 1;
  const bubbleH = assistantChatBubbleHeight(w, text, collapsed, config, choicesExtra);
  const inset = 10;
  return (
    <foreignObject
      id={id}
      x={x - inset}
      y={y - inset}
      width={w + inset * 2}
      height={bubbleH + inset * 2}
      style={{ overflow: 'visible' }}
    >
      <div
        style={{
          width: w + inset * 2,
          height: bubbleH + inset * 2,
          overflow: 'visible',
        }}
      >
        <div style={{ transform: `translate(${inset}px, ${inset}px)` }}>
          <ChatBubbleSvg
            variant={variant}
            width={w}
            text={text}
            collapsed={collapsed}
            copyLabel={`Copy ${senderLabel} message`}
            collapseLabel={`Collapse ${senderLabel} message`}
            expandLabel={`Expand ${senderLabel} message`}
            messageLabel={`${senderLabel}: ${text}`}
            headerLabel={senderLabel}
            onCollapsedChange={onCollapsedChange}
            onCopyClick={onCopy}
            config={config}
            body={
              <AssistantChatBubbleBody
                text={text}
                choiceColumnCount={choiceColumnCount}
                {...(choices === undefined ? {} : { choices })}
                {...(choiceActionLabel === undefined ? {} : { choiceActionLabel })}
                {...(onChoiceSelect === undefined ? {} : { onChoiceSelect })}
              />
            }
          />
        </div>
      </div>
    </foreignObject>
  );
}

function AssistantChatBubbleBody({
  text,
  choices,
  choiceColumnCount,
  choiceActionLabel,
  onChoiceSelect,
}: AssistantChatBubbleBodyProps) {
  const choicesVisible = choices?.length && onChoiceSelect;
  return (
    <div
      style={{
        display: 'grid',
        gap: choicesVisible ? 10 : 4,
        padding: '0 2px',
      }}
    >
      <div style={{ whiteSpace: 'pre-wrap', overflowWrap: 'anywhere', wordBreak: 'break-word' }}>{text}</div>
      {choicesVisible ? (
        <div
          role="group"
          aria-label={`${choiceActionLabel ?? 'MIA'} choices`}
          style={{
            display: 'grid',
            gridTemplateColumns: `repeat(${choiceColumnCount}, minmax(0, 1fr))`,
            gap: 7,
          }}
        >
          {choices.map((choice, index) => (
            <button
              key={`${choice.label}:${index}`}
              type="button"
              aria-label={`Ask MIA about ${choiceActionLabel}: ${choice.label}`}
              onClick={(event) => {
                event.stopPropagation();
                onChoiceSelect(choice);
              }}
              style={{
                minHeight: 28,
                borderRadius: 7,
                border: '1px solid rgba(11, 95, 146, 0.34)',
                background: 'rgba(11, 95, 146, 0.08)',
                color: '#0b1f2f',
                font: 'inherit',
                fontWeight: 800,
                cursor: 'pointer',
                padding: '5px 8px',
                textAlign: 'left',
                overflowWrap: 'anywhere',
                wordBreak: 'break-word',
              }}
            >
              {choice.label}
            </button>
          ))}
        </div>
      ) : null}
    </div>
  );
}

function assistantBubbleChoiceColumnCount(w: number): number {
  if (w < 420) return 1;
  if (w < 620) return 2;
  return 4;
}

function assistantBubbleChoiceBodyHeight(w: number, choiceCount: number): number {
  const columnCount = assistantBubbleChoiceColumnCount(w);
  const rowCount = Math.max(1, Math.ceil(Math.min(4, choiceCount) / columnCount));
  return 12 + rowCount * 36;
}

function assistantFollowUpQuestionWidth(option: AssistantQuestionnaireOption): number {
  return clampNumber(option.label.length * 7.4 + 32, 132, 270);
}

function assistantFollowUpLayout(w: number, options: readonly AssistantQuestionnaireOption[]) {
  const usableW = Math.max(160, w);
  const visibleOptions = options;
  const items: Array<{
    option: AssistantQuestionnaireOption;
    x: number;
    y: number;
    w: number;
    showLeftDivider: boolean;
  }> = [];
  let cursorX = 0;
  let cursorY = 0;
  let rowW = 0;
  let maxRowW = 0;

  visibleOptions.forEach((option) => {
    const itemW = Math.min(assistantFollowUpQuestionWidth(option), usableW);
    if (cursorX > 0 && cursorX + itemW > usableW) {
      maxRowW = Math.max(maxRowW, rowW);
      cursorX = 0;
      cursorY += ASSISTANT_FOLLOW_UP_ROW_H;
      rowW = 0;
    }
    items.push({
      option,
      x: cursorX,
      y: cursorY,
      w: itemW,
      showLeftDivider: cursorX > 0,
    });
    rowW = cursorX + itemW;
    cursorX = rowW + ASSISTANT_FOLLOW_UP_GAP;
  });

  maxRowW = Math.max(maxRowW, rowW);
  const rowCount = items.length > 0 ? Math.floor(cursorY / ASSISTANT_FOLLOW_UP_ROW_H) + 1 : 0;
  return { items, rowCount, contentW: maxRowW };
}

function assistantFollowUpPanelHeight(
  w: number,
  options: readonly AssistantQuestionnaireOption[],
  maxHeight: number
): number {
  const layout = assistantFollowUpLayout(w - ASSISTANT_FOLLOW_UP_PAD_X * 2, options);
  const contentHeight =
    ASSISTANT_FOLLOW_UP_PAD_Y * 2 +
    ASSISTANT_FOLLOW_UP_HEADER_H +
    Math.max(1, layout.rowCount) * ASSISTANT_FOLLOW_UP_ROW_H;
  return Math.min(maxHeight, contentHeight);
}

function AssistantFollowUpPanel({
  x,
  y,
  w,
  h,
  question,
  options,
  disabled,
  onSelect,
  cfg,
}: AssistantFollowUpPanelProps) {
  const layout = assistantFollowUpLayout(w - ASSISTANT_FOLLOW_UP_PAD_X * 2, options);
  const labelY = y + ASSISTANT_FOLLOW_UP_PAD_Y + 20;
  const listY = y + ASSISTANT_FOLLOW_UP_PAD_Y + ASSISTANT_FOLLOW_UP_HEADER_H;
  const bodyH = Math.max(ASSISTANT_FOLLOW_UP_ROW_H, h - ASSISTANT_FOLLOW_UP_PAD_Y * 2 - ASSISTANT_FOLLOW_UP_HEADER_H);
  const listX = x + ASSISTANT_FOLLOW_UP_PAD_X;
  const bodyW = w - ASSISTANT_FOLLOW_UP_PAD_X * 2;
  const questionX = x + ASSISTANT_FOLLOW_UP_PAD_X;
  return (
    <g role="group" aria-label="Questionnaire suggestions">
      <rect x={x} y={y} width={w} height={h} rx={0} fill={ASSISTANT_QUESTIONNAIRE_SURFACE_FILL} pointerEvents="none" />
      <path d={`M ${x} ${y + 1} H ${x + w}`} stroke={cfg.colors.panelStroke} strokeWidth={0.8} opacity={0.78} />
      <text x={questionX} y={labelY} fontSize={14.6} fontWeight={970} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(question, w - ASSISTANT_FOLLOW_UP_PAD_X * 2 - 80, 14.6, 0.56)}
      </text>
      <rect
        x={x}
        y={listY - 4}
        width={w}
        height={bodyH + 8}
        rx={0}
        fill={ASSISTANT_QUESTIONNAIRE_BODY_FILL}
        pointerEvents="none"
      />
      <path d={`M ${x} ${listY - 4} H ${x + w}`} stroke={cfg.colors.panelStroke} strokeWidth={0.7} opacity={0.58} />
      <path
        d={`M ${x + 0.5} ${y + 0.5} H ${x + w - 0.5} V ${y + h - 0.5} H ${x + 0.5} Z`}
        fill="none"
        stroke={cfg.colors.panelStroke}
        strokeWidth={0.65}
        opacity={0.46}
        pointerEvents="none"
      />
      <foreignObject x={listX} y={listY} width={bodyW} height={bodyH}>
        <div
          style={{
            alignContent: 'flex-start',
            boxSizing: 'border-box',
            display: 'flex',
            flexWrap: 'wrap',
            gap: `${ASSISTANT_FOLLOW_UP_GAP}px`,
            height: '100%',
            overflowY: 'auto',
            padding: '0 2px 4px',
            scrollbarColor: `${colorAlpha(cfg.colors.cyan, '88')} transparent`,
            scrollbarWidth: 'thin',
            width: '100%',
          }}
        >
          {layout.items.map((item) => (
            <button
              key={item.option.label}
              type="button"
              aria-label={`Ask MIA: ${item.option.label}`}
              disabled={disabled}
              onClick={() => onSelect(item.option)}
              style={{
                background: colorAlpha(cfg.colors.cyan, '14'),
                border: `1px solid ${colorAlpha(cfg.colors.panelStroke, '66')}`,
                borderRadius: 6,
                color: cfg.colors.bodyText,
                cursor: disabled ? 'not-allowed' : 'pointer',
                flex: `0 0 ${item.w}px`,
                font: 'inherit',
                fontSize: 13,
                fontWeight: 930,
                minHeight: 28,
                padding: '5px 9px',
                textAlign: 'left',
                textShadow: '0 1px 2px rgba(0,0,0,0.5)',
                whiteSpace: 'normal',
              }}
            >
              {item.option.label}
            </button>
          ))}
        </div>
      </foreignObject>
    </g>
  );
}

function AssistantComposerSplitter({
  x,
  y,
  w,
  dragging,
  ariaLabel,
  onPointerDown,
  onPointerMove,
  onPointerUp,
  onPointerCancel,
  cfg,
}: AssistantComposerSplitterProps) {
  const [hovered, setHovered] = useState(false);
  const handleW = Math.min(148, Math.max(78, w * 0.14));
  const handleX = x + w / 2 - handleW / 2;
  const lineStart = x + 8;
  const lineEnd = x + w - 8;
  const handleGap = 8;
  const lit = hovered || dragging;
  return (
    <g
      role="separator"
      aria-label={ariaLabel}
      aria-orientation="horizontal"
      tabIndex={0}
      className="parent-portal-svg-clickable"
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerCancel={onPointerCancel}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={() => setHovered(true)}
      onBlur={() => setHovered(false)}
      style={{ cursor: 'ns-resize' }}
    >
      <rect x={x} y={y - 14} width={w} height={28} rx={10} fill="transparent" />
      <path
        d={`M ${lineStart} ${y} H ${Math.max(lineStart, handleX - handleGap)}`}
        stroke={cfg.colors.panelStroke}
        strokeWidth={0.75}
        strokeLinecap="round"
        opacity={lit ? 0.72 : 0.48}
      />
      <path
        d={`M ${Math.min(lineEnd, handleX + handleW + handleGap)} ${y} H ${lineEnd}`}
        stroke={cfg.colors.panelStroke}
        strokeWidth={0.75}
        strokeLinecap="round"
        opacity={lit ? 0.72 : 0.48}
      />
      <rect
        x={handleX}
        y={y - 3.5}
        width={handleW}
        height={7}
        rx={3.5}
        fill={lit ? colorAlpha(cfg.colors.cyan, '22') : 'transparent'}
        stroke={colorAlpha(cfg.colors.cyan, lit ? 'c8' : '99')}
        strokeWidth={0.7}
      />
    </g>
  );
}

function AssistantComposer({
  x,
  y,
  w,
  h,
  prompt,
  disabled,
  awaitingResponse,
  onPromptChange,
  onSend,
  cfg,
}: AssistantComposerProps) {
  const displayPrompt = disabled
    ? 'Connect the local service to use MIA.'
    : awaitingResponse
      ? 'Waiting for the service response...'
      : prompt || 'Ask MIA about activity, rules, reports, setup...';
  const attachSize = 30;
  const sendW = 58;
  const sendH = 34;
  const voiceSize = 32;
  const sendX = x + w - sendW - 14;
  const sendY = y + (h - sendH) / 2;
  const voiceX = sendX - voiceSize - 10;
  const inputX = x + 58;
  const inputW = Math.max(120, voiceX - inputX - 12);
  const planeCx = sendX + sendW / 2 + 2;
  const planeCy = sendY + sendH / 2;
  return (
    <g>
      <path
        d={topRoundedRectPath(x, y, w, h, 14)}
        fill="rgba(2, 12, 20, 0.66)"
        stroke={cfg.colors.cyan}
        strokeWidth={0.95}
      />
      <g role="button" tabIndex={-1} aria-label="Attach context to MIA" aria-disabled="true" opacity={0.46}>
        <title>Attach context is unavailable</title>
        <rect
          x={x + 14}
          y={y + 9}
          width={attachSize}
          height={attachSize}
          rx={8}
          fill={colorAlpha(cfg.colors.cyan, '18')}
          stroke={cfg.colors.cyan}
          strokeWidth={0.9}
        />
        <path
          d={`M ${x + 29} ${y + 17} V ${y + 31} M ${x + 22} ${y + 24} H ${x + 36}`}
          stroke={cfg.colors.cyan}
          strokeWidth={1.4}
          strokeLinecap="round"
        />
      </g>
      <foreignObject x={inputX} y={y + 9} width={inputW} height={Math.max(32, h - 18)}>
        <textarea
          aria-label="Message MIA"
          value={prompt}
          placeholder={displayPrompt}
          disabled={disabled || awaitingResponse}
          onChange={(event) => onPromptChange?.(event.currentTarget.value)}
          onKeyDown={(event) => {
            if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
              event.preventDefault();
              onSend?.();
            }
          }}
          style={{
            background: 'transparent',
            border: 0,
            boxSizing: 'border-box',
            color: prompt ? cfg.colors.bodyText : cfg.colors.mutedText,
            font: 'inherit',
            fontSize: 12.5,
            fontWeight: 780,
            height: '100%',
            lineHeight: 1.45,
            margin: 0,
            outline: 'none',
            overflowY: 'auto',
            padding: '3px 0',
            resize: 'none',
            scrollbarColor: `${colorAlpha(cfg.colors.cyan, '88')} transparent`,
            scrollbarWidth: 'thin',
            textShadow: '0 1px 2px rgba(0,0,0,0.44)',
            width: '100%',
          }}
        />
      </foreignObject>
      <g role="button" tabIndex={-1} aria-label="Use voice input for MIA" aria-disabled="true" opacity={0.46}>
        <title>Voice input is unavailable</title>
        <rect
          x={voiceX}
          y={y + 8}
          width={voiceSize}
          height={voiceSize}
          rx={10}
          fill={colorAlpha(cfg.colors.gold, '18')}
          stroke={cfg.colors.panelStroke}
          strokeWidth={0.9}
        />
        <rect
          x={voiceX + 12}
          y={y + 15}
          width={8}
          height={13}
          rx={4}
          fill="none"
          stroke={cfg.colors.bodyText}
          strokeWidth={1.35}
        />
        <path
          d={`M ${voiceX + 8} ${y + 24} C ${voiceX + 8} ${y + 32}, ${voiceX + 24} ${y + 32}, ${voiceX + 24} ${y + 24}`}
          fill="none"
          stroke={cfg.colors.bodyText}
          strokeWidth={1.25}
          strokeLinecap="round"
        />
        <path
          d={`M ${voiceX + 16} ${y + 32} V ${y + 36} M ${voiceX + 12} ${y + 36} H ${voiceX + 20}`}
          stroke={cfg.colors.bodyText}
          strokeWidth={1.25}
          strokeLinecap="round"
        />
      </g>
      <g
        role="button"
        tabIndex={disabled || awaitingResponse ? -1 : 0}
        aria-label="Send message to MIA"
        aria-disabled={disabled || awaitingResponse ? 'true' : undefined}
        className={disabled || awaitingResponse ? undefined : 'parent-portal-svg-clickable'}
        opacity={disabled || awaitingResponse ? 0.46 : 1}
        onClick={disabled || awaitingResponse ? undefined : onSend}
        onKeyDown={(event) => {
          if (disabled || awaitingResponse) return;
          if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            onSend?.();
          }
        }}
      >
        <title>Send message to MIA</title>
        <rect
          x={sendX}
          y={sendY}
          width={sendW}
          height={sendH}
          rx={sendH / 2}
          fill={colorAlpha(cfg.colors.cyan, '24')}
          stroke={cfg.colors.cyan}
          strokeWidth={1.05}
        />
        <rect
          x={sendX}
          y={sendY + 3}
          width={12}
          height={sendH - 6}
          rx={(sendH - 6) / 2}
          fill={colorAlpha(cfg.colors.cyan, '62')}
          stroke={colorAlpha(cfg.colors.bodyText, 'b8')}
          strokeWidth={0.55}
          filter="url(#parentPortalGlow)"
        />
        <path
          d={`M ${planeCx - 11} ${planeCy - 7} L ${planeCx + 12} ${planeCy} L ${planeCx - 11} ${planeCy + 7} L ${planeCx - 5} ${planeCy} Z`}
          fill={cfg.colors.bodyText}
          opacity={0.96}
        />
        <path
          d={`M ${planeCx - 5} ${planeCy} H ${planeCx + 8}`}
          stroke={cfg.colors.cyan}
          strokeWidth={1.25}
          strokeLinecap="round"
          opacity={0.9}
        />
      </g>
    </g>
  );
}

function ManagePill({
  x,
  y,
  w,
  h,
  label,
  selected,
  disabled = false,
  tone,
  themeColor,
  onSelect,
  cfg,
}: ManagePillProps) {
  const [hovered, setHovered] = useState(false);
  const color = themeColor ?? toneColor(tone, cfg);
  return (
    <g
      className={disabled ? undefined : 'parent-portal-svg-clickable'}
      role="button"
      tabIndex={disabled ? undefined : 0}
      aria-label={`Select ${label}`}
      aria-disabled={disabled || undefined}
      opacity={disabled ? 0.46 : 1}
      onMouseEnter={disabled ? undefined : () => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onClick={disabled ? undefined : onSelect}
      onKeyDown={
        disabled
          ? undefined
          : (event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              onSelect();
            }
      }
    >
      <title>{label}</title>
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '34') : hovered ? colorAlpha(color, '22') : colorAlpha(color, '08')}
        stroke={selected || hovered ? color : cfg.colors.panelStroke}
        strokeWidth={selected ? 1.25 : hovered ? 1 : 0.7}
        filter={selected || hovered ? 'url(#parentPortalGlow)' : undefined}
      />
      <text
        x={x + w / 2}
        y={y + 19}
        textAnchor="middle"
        fontSize={10.5}
        fontWeight={900}
        fill={selected ? cfg.colors.bodyText : cfg.colors.mutedText}
      >
        {truncateTextForWidth(label, w - 12, 10.5, 0.58)}
      </text>
    </g>
  );
}

function ManageModeButton({
  x,
  y,
  w,
  h,
  item,
  selected,
  disabled = false,
  themeColor,
  onSelect,
  cfg,
}: ManageModeButtonProps) {
  const [hovered, setHovered] = useState(false);
  const color = themeColor ?? toneColor(item.tone, cfg);
  const titleText = item.detail ? `${item.label}: ${item.detail}` : item.label;
  const labelFontSize = w < 76 ? 10 : 12;
  const labelInset = w < 76 ? 6 : 10;
  return (
    <g
      className={disabled ? undefined : 'parent-portal-svg-clickable'}
      role="button"
      tabIndex={disabled ? undefined : 0}
      aria-label={`Use ${item.label}`}
      aria-disabled={disabled || undefined}
      opacity={disabled ? 0.46 : 1}
      onMouseEnter={disabled ? undefined : () => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onClick={disabled ? undefined : onSelect}
      onKeyDown={
        disabled
          ? undefined
          : (event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              onSelect();
            }
      }
    >
      <title>{titleText}</title>
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '30') : hovered ? colorAlpha(color, '18') : colorAlpha(color, '08')}
        stroke={selected || hovered ? color : cfg.colors.panelStroke}
        strokeWidth={selected ? 1.25 : hovered ? 1 : 0.7}
        filter={selected || hovered ? 'url(#parentPortalGlow)' : undefined}
      />
      <text
        x={x + labelInset}
        y={y + h / 2 + 4}
        fontSize={labelFontSize}
        fontWeight={950}
        fill={selected ? cfg.colors.bodyText : color}
      >
        {truncateTextForWidth(item.label, w - labelInset * 2, labelFontSize, 0.58)}
      </text>
    </g>
  );
}

function ManageToggle({
  x,
  y,
  w,
  h,
  option,
  selected,
  disabled = false,
  themeColor,
  onToggle,
  cfg,
}: ManageToggleProps) {
  const [hovered, setHovered] = useState(false);
  const color = themeColor ?? toneColor(option.tone, cfg);
  return (
    <g
      className={disabled ? undefined : 'parent-portal-svg-clickable'}
      role="checkbox"
      tabIndex={disabled ? undefined : 0}
      aria-label={option.label}
      aria-checked={selected}
      aria-disabled={disabled || undefined}
      opacity={disabled ? 0.46 : 1}
      onMouseEnter={disabled ? undefined : () => setHovered(true)}
      onMouseLeave={disabled ? undefined : () => setHovered(false)}
      onClick={disabled ? undefined : onToggle}
      onKeyDown={
        disabled
          ? undefined
          : (event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              onToggle();
            }
      }
    >
      <title>{option.detail ? `${option.label}: ${option.detail}` : option.label}</title>
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={selected ? colorAlpha(color, '22') : hovered ? colorAlpha(color, '14') : colorAlpha(color, '08')}
        stroke={selected || hovered ? color : cfg.colors.panelStroke}
        strokeWidth={selected ? 1.05 : hovered ? 0.95 : 0.7}
        filter={selected || hovered ? 'url(#parentPortalGlow)' : undefined}
      />
      <path
        d={cutRectPath(x + 8, y + 9, 24, 24, 5)}
        fill={selected ? colorAlpha(color, '4c') : 'transparent'}
        stroke={color}
        strokeWidth={1}
      />
      {selected ? (
        <path
          d={`M ${x + 14} ${y + 22} l 5 5 l 9 -13`}
          fill="none"
          stroke={cfg.colors.bodyText}
          strokeWidth={2}
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      ) : null}
      <text x={x + 42} y={y + 18} fontSize={10.5} fontWeight={930} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(option.label, w - 50, 10.5, 0.58)}
      </text>
    </g>
  );
}

function ManageActionButton({
  x,
  y,
  w,
  h,
  action,
  disabled = false,
  themeColor,
  onSelect,
  cfg,
}: ManageActionButtonProps) {
  const [hovered, setHovered] = useState(false);
  const color = themeColor ?? toneColor(action.tone, cfg);
  return (
    <g
      className={disabled ? undefined : 'parent-portal-svg-clickable'}
      role="button"
      tabIndex={disabled ? undefined : 0}
      aria-label={action.label}
      aria-disabled={disabled || undefined}
      opacity={disabled ? 0.46 : 1}
      onClick={disabled ? undefined : onSelect}
      onKeyDown={
        disabled
          ? undefined
          : (event) => {
              if (event.key !== 'Enter' && event.key !== ' ') return;
              event.preventDefault();
              onSelect();
            }
      }
      onMouseEnter={disabled ? undefined : () => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onFocus={disabled ? undefined : () => setHovered(true)}
      onBlur={() => setHovered(false)}
    >
      <title>{action.detail ? `${action.label}: ${action.detail}` : action.label}</title>
      {hovered ? (
        <path
          d={cutRectPath(x - 4, y - 4, w + 8, h + 8, 10)}
          fill="none"
          stroke={color}
          strokeWidth={1.4}
          opacity={0.5}
          filter="url(#parentPortalGlow)"
        />
      ) : null}
      <path
        d={cutRectPath(x, y, w, h, 8)}
        fill={hovered ? colorAlpha(color, '32') : colorAlpha(color, '18')}
        stroke={hovered ? color : cfg.colors.panelStroke}
        strokeWidth={hovered ? 1.2 : 0.8}
      />
      <text x={x + 14} y={y + h / 2 + 4} fontSize={11.5} fontWeight={950} fill={cfg.colors.bodyText}>
        {truncateTextForWidth(action.label.toUpperCase(), w - 28, 11.5, 0.58)}
      </text>
    </g>
  );
}

function ProductShellRouteReadinessStrip({
  x,
  y,
  w,
  h,
  rows,
  themeTone,
  themeColor,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  rows: DisplayRow[];
  themeTone: Tone;
  themeColor?: string;
  cfg: ParentPortalSvgControls;
}) {
  if (rows.length === 0 || w <= 0 || h <= 0) return null;
  const color = themeColor ?? toneColor(themeTone, cfg);
  const chipCount = Math.min(rows.length, w < 650 ? 2 : w < 980 ? 3 : PORTAL_PRODUCT_ROUTE_STATUS_MAX_ROWS);
  const visibleRows = rows.slice(0, chipCount);
  const pad = 10;
  const labelW = w < 650 ? 106 : 146;
  const chipGap = 8;
  const chipW = Math.max(112, (w - pad * 2 - labelW - chipGap * visibleRows.length) / Math.max(1, visibleRows.length));
  const chipH = Math.max(30, h - pad * 2);
  const chipY = y + (h - chipH) / 2;
  return (
    <g aria-label="Route readiness product shell status" pointerEvents="none">
      <path
        d={cutRectPath(x, y, w, h, 9)}
        fill={PARENT_PORTAL_GLASS.panelFill}
        stroke={color}
        strokeWidth={1}
        strokeOpacity={0.58}
      />
      <path
        d={`M ${x + pad} ${y + h - 8} H ${x + Math.min(w - pad, labelW - 8)}`}
        stroke={color}
        strokeWidth={1.1}
        opacity={0.72}
        filter="url(#parentPortalGlow)"
      />
      <text x={x + pad} y={y + h * 0.42} fontSize={10.2} fontWeight={950} fill={cfg.colors.bodyText}>
        ROUTE
      </text>
      <text x={x + pad} y={y + h * 0.68} fontSize={10.2} fontWeight={950} fill={color}>
        READINESS
      </text>
      {visibleRows.map((row, index) => {
        const chipX = x + pad + labelW + index * (chipW + chipGap);
        const rowColor = toneColor(row.tone, cfg);
        const status = formatPortalTrendLabel(row.trend);
        const titleSize = fitSingleLineTextSize(row.label, chipW - 22, 9.8, 12.2, 0.57);
        const statusSize = fitSingleLineTextSize(status, chipW - 22, 8.2, 10.4, 0.57);
        return (
          <g key={`${row.id}:route-readiness:${index}`}>
            <path
              d={cutRectPath(chipX, chipY, chipW, chipH, 7)}
              fill={colorAlpha(rowColor, '1f')}
              stroke={rowColor}
              strokeWidth={1}
              strokeOpacity={0.7}
            />
            <path
              d={cutRectPath(chipX + 4, chipY + 4, chipW - 8, chipH - 8, 5)}
              fill="none"
              stroke={rowColor}
              strokeWidth={0.65}
              opacity={0.32}
            />
            <text
              x={chipX + 10}
              y={chipY + Math.max(14, chipH * 0.42)}
              fontSize={titleSize}
              fontWeight={940}
              fill={cfg.colors.bodyText}
            >
              {truncateTextForWidth(row.label, chipW - 20, titleSize, 0.57)}
            </text>
            <text
              x={chipX + 10}
              y={chipY + Math.max(26, chipH * 0.72)}
              fontSize={statusSize}
              fontWeight={820}
              fill={rowColor}
            >
              {truncateTextForWidth(`${status} / ${productShellReadinessDetail(row)}`, chipW - 20, statusSize, 0.57)}
            </text>
          </g>
        );
      })}
    </g>
  );
}

function ParentPortalDetailPanel({
  x,
  y,
  w,
  h,
  activeNavLabel,
  activeNavGroupId,
  detail,
  rows,
  parentPortalRows,
  selectedControlName,
  themeTone,
  themeColor,
  guideTopic,
  guidePage,
  onGuidePageChange,
  quickPanelMode,
  onQuickPanelModeChange,
  onGuideNoteSelect,
  manageTargetSelection,
  onManageTargetChange,
  activityState,
  onNavigate,
  onAgentCommand,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  activeNavLabel: string;
  activeNavGroupId: string;
  detail: TabDetail;
  rows: DisplayRow[];
  parentPortalRows: ParentPortalRow[];
  selectedControlName: string;
  themeTone?: Tone;
  themeColor?: string;
  guideTopic?: ParentPortalGuideTopic | null;
  guidePage: number;
  onGuidePageChange: (page: number) => void;
  quickPanelMode: 'read' | 'action';
  onQuickPanelModeChange: (mode: 'read' | 'action') => void;
  onGuideNoteSelect: (note: ParentPortalGuideNote) => void;
  manageTargetSelection?: ManageTargetSelection;
  onManageTargetChange?: (selection: ManageTargetSelection) => void;
  activityState?: ParentPortalActivityState | null;
  onNavigate?: (routePath: string) => void;
  onAgentCommand?: (command: AgentCommandName, payload: Record<string, string>) => void;
  cfg: ParentPortalSvgControls;
}) {
  if (guideTopic) {
    const routeStatusRows = productShellDisplayRowsForRoute(
      parentPortalRows,
      activeNavLabel,
      selectedControlName,
      guideTopic.title
    );
    const showRouteStatus = routeStatusRows.length > 0 && h >= 320;
    const routeStatusH = showRouteStatus ? (w < 660 ? 58 : 50) : 0;
    const routeStatusGap = showRouteStatus ? 8 : 0;
    const guidePanelH = Math.max(1, h - routeStatusH - routeStatusGap);
    return (
      <g>
        <GuideTopicDetailPanel
          x={x}
          y={y}
          w={w}
          h={guidePanelH}
          topic={guideTopic}
          page={guidePage}
          onPageChange={onGuidePageChange}
          quickPanelMode={quickPanelMode}
          onQuickPanelModeChange={onQuickPanelModeChange}
          onNoteSelect={onGuideNoteSelect}
          {...(themeColor === undefined ? {} : { themeColor })}
          cfg={cfg}
        />
        {showRouteStatus ? (
          <ProductShellRouteReadinessStrip
            x={x}
            y={y + guidePanelH + routeStatusGap}
            w={w}
            h={routeStatusH}
            rows={routeStatusRows}
            themeTone={guideTopic.tone}
            {...(themeColor === undefined ? {} : { themeColor })}
            cfg={cfg}
          />
        ) : null}
      </g>
    );
  }
  const manageSpec = activeNavGroupId === 'manage' ? manageControlSpecFor(activeNavLabel, selectedControlName) : null;
  if (manageSpec) {
    const manageSpecIsLanPairing = isLanPairingManageTitle(manageSpec.title);
    const routeStatusRows = productShellDisplayRowsForRoute(
      parentPortalRows,
      activeNavLabel,
      selectedControlName,
      manageSpec.title
    );
    const showRouteStatus = routeStatusRows.length > 0 && h >= 300 && !manageSpecIsLanPairing;
    const routeStatusH = showRouteStatus ? (w < 660 ? 58 : 50) : 0;
    const routeStatusGap = showRouteStatus ? 8 : 0;
    const managePanelH = Math.max(1, h - routeStatusH - routeStatusGap);
    return (
      <g>
        <ManageControlPanel
          x={x}
          y={y}
          w={w}
          h={managePanelH}
          activeNavLabel={activeNavLabel}
          selectedControlName={selectedControlName}
          spec={manageSpec}
          themeTone={themeTone ?? detail.tone}
          {...(themeColor === undefined ? {} : { themeColor })}
          targetSelection={
            manageTargetSelection ?? {
              scope: manageInitialScopeForSpec(manageLaneForKey(activeNavLabel, selectedControlName), manageSpec),
              device: manageDefaultDeviceSelection(manageSpec),
              deviceId: '',
              browser: manageBrowserTargetsForKey(activeNavLabel, selectedControlName)[0]?.label ?? 'All targets',
            }
          }
          {...(onManageTargetChange === undefined ? {} : { onTargetChange: onManageTargetChange })}
          {...(activityState === undefined ? {} : { activityState })}
          parentPortalRows={parentPortalRows}
          {...(onNavigate === undefined ? {} : { onNavigate })}
          {...(onAgentCommand === undefined ? {} : { onAgentCommand })}
          cfg={cfg}
        />
        {showRouteStatus ? (
          <ProductShellRouteReadinessStrip
            x={x}
            y={y + managePanelH + routeStatusGap}
            w={w}
            h={routeStatusH}
            rows={routeStatusRows}
            themeTone={themeTone ?? detail.tone}
            {...(themeColor === undefined ? {} : { themeColor })}
            cfg={cfg}
          />
        ) : null}
      </g>
    );
  }
  const color = themeColor ?? toneColor(detail.tone, cfg);
  const routeUnavailable = rows.length === 0;
  const usableH = Math.max(120, h);
  const title = activeNavLabel || detail.title;
  const bodyLines = wrapCardText(detail.summary, w - 40, 12, 2);
  const featureCards: ManageWorkspaceCard[] = [
    {
      label: 'WHAT PARENTS CONTROL',
      value: detail.primary,
      body: detail.secondary,
      tone: detail.tone,
    },
    {
      label: 'CURRENT AREA',
      value: activeNavLabel,
      body: 'Current status appears after the local service connects and reports this area.',
      tone: 'cyan',
    },
    {
      label: 'DATA CUSTODY',
      value: 'LOCAL FIRST',
      body: 'No cloud sharing by default. Drive exports and support messages are parent opt-in.',
      tone: 'gold',
    },
    ...(rows.length === 0
      ? [
          {
            label: 'SERVICE SNAPSHOT',
            value: 'UNAVAILABLE',
            body: 'Connect the local service for current device status. Controls stay read-only.',
            tone: 'muted' as const,
          },
        ]
      : rows.slice(0, 3).map((row) => ({
          label: row.primaryArea.toUpperCase(),
          value: row.label,
          body: `${row.trend} / ${row.readiness}`,
          tone: row.tone,
        }))),
    ...(routeUnavailable
      ? [
          {
            label: 'NEXT SAFE STEP',
            value: 'CONNECT LOCAL SERVICE',
            body: 'Open Start Here to restore the local service and load service-owned device state.',
            tone: 'cyan' as const,
            action: {
              label: 'OPEN START HERE',
              routePath: '#/start',
            },
          },
          {
            label: 'CONTROL AUTHORITY',
            value: 'READ ONLY',
            body: 'Controls stay off until the local service confirms the current device and permissions.',
            tone: 'muted' as const,
          },
        ]
      : []),
  ];
  const visibleCards = featureCards.slice(0, usableH < 300 ? 3 : 6);
  const columnCount = w > 1220 ? 3 : w > 760 ? 2 : 1;
  const rowCount = Math.max(1, Math.ceil(visibleCards.length / columnCount));
  const denseSingleColumnGrid = columnCount === 1 && rowCount >= 5;
  const cardGap = denseSingleColumnGrid ? 8 : 12;
  const headerH = bodyLines.length > 1 ? 78 : 62;
  const cardW = (w - cardGap * Math.max(0, columnCount - 1)) / columnCount;
  const cardH = clampValue(
    (usableH - headerH - cardGap * Math.max(0, rowCount - 1)) / rowCount,
    denseSingleColumnGrid ? 64 : 74,
    routeUnavailable ? 140 : 118
  );
  const titleSize = fitSingleLineTextSize(title, w - 40, 16, 26, 0.58);
  return (
    <g>
      <text x={x} y={y + 24} fontSize={titleSize} fontWeight={950} fill={cfg.colors.bodyText}>
        {title}
      </text>
      <path
        d={`M ${x} ${y + 39} H ${x + w}`}
        stroke={color}
        strokeWidth={1.1}
        opacity={PARENT_PORTAL_FRAME_MATERIAL.headerLineOpacity}
      />
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
        const cardColor = themeColor ?? toneColor(card.tone, cfg);
        const valueSize = fitSingleLineTextSize(card.value, cardW - 34, 12, 17, 0.58);
        const compactCard = cardH < 96;
        const cardBodyLines = wrapCardText(card.body, cardW - 34, 10.5, compactCard ? 1 : 2);
        const cardAction = card.action;
        const labelY = cardY + (compactCard ? 18 : 25);
        const valueY = cardY + (compactCard ? 37 : 49);
        const bodyY = cardY + (compactCard ? 57 : 70);
        const actionH = compactCard ? 22 : 32;
        const actionY = compactCard ? cardY + cardH - actionH - 5 : cardY + 70;
        return (
          <SurfacePanel
            key={`${card.label}:${index}`}
            x={cardX}
            y={cardY}
            w={cardW}
            h={cardH}
            tone={card.tone}
            {...(themeColor === undefined ? {} : { accentColor: themeColor })}
            cfg={cfg}
          >
            <text x={cardX + 16} y={labelY} fontSize={9.8} fontWeight={900} fill={cardColor}>
              {card.label}
            </text>
            <text x={cardX + 16} y={valueY} fontSize={valueSize} fontWeight={950} fill={cfg.colors.bodyText}>
              {truncateTextForWidth(card.value, cardW - 34, valueSize, 0.58)}
            </text>
            {cardAction ? (
              <ManageActionButton
                x={cardX + 16}
                y={actionY}
                w={Math.min(174, cardW - 32)}
                h={actionH}
                action={{ label: cardAction.label, detail: card.body, tone: card.tone }}
                themeColor={cardColor}
                onSelect={() => onNavigate?.(cardAction.routePath)}
                cfg={cfg}
              />
            ) : (
              cardBodyLines.map((line, lineIndex) => (
                <text
                  key={`${line}:${lineIndex}`}
                  x={cardX + 16}
                  y={bodyY + lineIndex * 15}
                  fontSize={10.5}
                  fontWeight={720}
                  fill={cfg.colors.mutedText}
                >
                  {line}
                </text>
              ))
            )}
          </SurfacePanel>
        );
      })}
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
  themeColor,
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
  themeColor?: string;
  onClick: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const color = themeColor ?? toneColor(tone, cfg);
  const lit = active || hovered;
  return (
    <g
      className="parent-portal-svg-clickable"
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
        fill={lit ? colorAlpha(color, active ? '42' : '24') : PARENT_PORTAL_GLASS.cardFillStrong}
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
  themeColor,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  note: ParentPortalGuideNote;
  mode: 'read' | 'action';
  themeColor?: string;
  onSelect: (note: ParentPortalGuideNote) => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const noteColor = themeColor ?? toneColor(note.tone, cfg);
  const actionable =
    mode === 'action' ||
    Boolean(note.targetRoutePath || note.targetNavLabel || note.targetTopicId || typeof note.targetPage === 'number');
  const lit = actionable && (hovered || mode === 'action');
  const labelSize = fitSingleLineTextSize(note.label, w - 20, 9.5, 12.5, 0.58);
  const noteLines = wrapCardText(note.body, w - 24, 10.2, h > 66 ? 3 : 2);
  return (
    <g
      className={actionable ? 'parent-portal-svg-clickable' : undefined}
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
            filter="url(#parentPortalGlow)"
          />
          {hovered ? (
            <path
              d={`M ${x + w - 10} ${y + 15} L ${x + w + 8} ${y + h / 2} L ${x + w - 10} ${y + h - 15} Z`}
              fill={noteColor}
              opacity={0.64}
              filter="url(#parentPortalGlow)"
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
  themeColor,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  topic: ParentPortalGuideTopic;
  page: number;
  onPageChange: (page: number) => void;
  quickPanelMode: 'read' | 'action';
  onQuickPanelModeChange: (mode: 'read' | 'action') => void;
  onNoteSelect: (note: ParentPortalGuideNote) => void;
  themeColor?: string;
  cfg: ParentPortalSvgControls;
}) {
  const color = themeColor ?? toneColor(topic.tone, cfg);
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
  const mainH = compact ? Math.max(240, h * 0.68) : h;
  const sideX = compact ? x : x + mainW + gap;
  const sideY = compact ? y + mainH + gap : y;
  const sideH = compact ? Math.max(160, h - mainH - gap) : h;
  const titleSize = fitSingleLineTextSize(topic.title, mainW - 36, compact ? 12.5 : 17, compact ? 18 : 25, 0.58);
  const subtitleLines = wrapCardText(topic.subtitle, mainW - 36, 12, 2);
  const pageTitleSize = compact ? 14.5 : 17;
  const pageTitleLineHeight = compact ? 17 : 19;
  const pageTitleLines = wrapCardText(currentPage.title, mainW - 36, pageTitleSize, compact ? 4 : 1);
  const bodyStartY = y + 151 + Math.max(0, pageTitleLines.length - 1) * pageTitleLineHeight;
  const bodyLines = wrapCardText(currentPage.body, mainW - 44, 13.2, compact ? 2 : 8);
  const stepStartY = bodyStartY + 7 + bodyLines.length * 19;
  const stepGap = compact ? 22 : 42;
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
      <SurfacePanel
        x={x}
        y={y}
        w={mainW}
        h={mainH}
        tone={topic.tone}
        {...(themeColor === undefined ? {} : { accentColor: themeColor })}
        cfg={cfg}
      >
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
        <g role="heading" aria-level={3} aria-label={currentPage.title}>
          {pageTitleLines.map((line, index) => (
            <text
              key={`${topic.id}:page-title:${index}`}
              x={x + 18}
              y={y + 127 + index * pageTitleLineHeight}
              fontSize={pageTitleSize}
              fontWeight={950}
              fill={cfg.colors.bodyText}
            >
              {line}
            </text>
          ))}
        </g>
        {bodyLines.map((line, index) => (
          <text
            key={`${topic.id}:body:${index}`}
            x={x + 18}
            y={bodyStartY + index * 19}
            fontSize={13.2}
            fontWeight={720}
            fill={cfg.colors.mutedText}
          >
            {line}
          </text>
        ))}
        <g role="list" aria-label={`${topic.title} guide steps`}>
          {visibleSteps.map((step, index) => {
            const stepY = stepStartY + index * stepGap;
            const stepLines = wrapCardText(step, mainW - 72, 12.2, compact ? 1 : 2);
            return (
              <g key={`${topic.id}:step:${index}`} role="listitem" aria-label={`Guide step ${index + 1}: ${step}`}>
                <circle
                  cx={x + 30}
                  cy={stepY - 4}
                  r={10}
                  fill={colorAlpha(color, '22')}
                  stroke={color}
                  strokeWidth={0.85}
                />
                <text x={x + 30} y={stepY} textAnchor="middle" fontSize={10.5} fontWeight={950} fill={color}>
                  {index + 1}
                </text>
                {stepLines.map((line, lineIndex) => (
                  <text
                    key={`${topic.id}:step:${index}:${lineIndex}`}
                    x={x + 52}
                    y={stepY + lineIndex * 15}
                    fontSize={12.2}
                    fontWeight={760}
                    fill={lineIndex === 0 ? cfg.colors.bodyText : cfg.colors.mutedText}
                  >
                    {line}
                  </text>
                ))}
              </g>
            );
          })}
        </g>
        {pageCount > 1 ? (
          <g>
            {topic.pages.map((item, index) => {
              const pillW = 30;
              const pillX = x + mainW - 18 - (pageCount - index) * (pillW + 6);
              const selected = index === safePage;
              return (
                <g
                  key={`${topic.id}:page:${item.title}`}
                  className="parent-portal-svg-clickable"
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
                    fill={selected ? colorAlpha(color, '44') : PARENT_PORTAL_GLASS.panelFillSoft}
                    stroke={color}
                    strokeWidth={selected ? 1.2 : 0.75}
                  />
                  <text
                    x={pillX + pillW / 2}
                    y={y + mainH - 17}
                    textAnchor="middle"
                    fontSize={9}
                    fontWeight={950}
                    fill={selected ? color : cfg.colors.mutedText}
                  >
                    {index + 1}
                  </text>
                </g>
              );
            })}
          </g>
        ) : null}
      </SurfacePanel>
      <SurfacePanel
        x={sideX}
        y={sideY}
        w={sideW}
        h={sideH}
        tone={topic.tone}
        {...(themeColor === undefined ? {} : { accentColor: themeColor })}
        cfg={cfg}
      >
        <GuideQuickTab
          x={sideX + 14}
          y={sideY + 14}
          w={(sideW - 34) / 2}
          h={28}
          label="QUICK READ"
          active={quickPanelMode === 'read'}
          tone={topic.tone}
          {...(themeColor === undefined ? {} : { themeColor })}
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
          {...(themeColor === undefined ? {} : { themeColor })}
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
            {...(themeColor === undefined ? {} : { themeColor })}
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
  themeColor,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  topics: ParentPortalGuideTopic[];
  selectedTopicId: string;
  onSelect: (topic: ParentPortalGuideTopic) => void;
  themeColor?: string;
  cfg: ParentPortalSvgControls;
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
  const setupColor = themeColor ?? (setupTopic ? toneColor(setupTopic.tone, cfg) : cfg.colors.cyan);
  const introTitleSize = fitSingleLineTextSize(setupTopic?.title ?? '', introSplit ? w * 0.42 : w - 36, 15, 20, 0.58);
  const [hoveredTopicId, setHoveredTopicId] = useState<string | null>(null);
  return (
    <g>
      {setupTopic ? (
        <g
          className="parent-portal-svg-clickable"
          role="button"
          tabIndex={0}
          aria-label={`Open ${setupTopic.title}`}
          aria-pressed={normalizeSelectionId(setupTopic.id) === normalizeSelectionId(selectedTopicId)}
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
            filter="url(#parentPortalGlow)"
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
        const color = themeColor ?? toneColor(topic.tone, cfg);
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
            className="parent-portal-svg-clickable"
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
              fill={selected ? colorAlpha(color, '2e') : PARENT_PORTAL_GLASS.cardFillStrong}
              stroke={color}
              strokeWidth={selected ? 1.7 : 0.95}
              strokeOpacity={selected ? 0.92 : 0.62}
              filter={selected ? 'url(#parentPortalGlow)' : undefined}
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
  let truncated = false;
  for (const word of words) {
    const next = current ? `${current} ${word}` : word;
    if (next.length * fontSize * 0.55 <= width || !current) {
      current = next;
      continue;
    }
    lines.push(current);
    current = word;
    if (lines.length >= maxLines) {
      truncated = true;
      break;
    }
  }
  if (current && lines.length < maxLines) lines.push(current);
  return lines.map((line, index) =>
    index === maxLines - 1 ? truncateTextForWidth(truncated ? `${line}...` : line, width, fontSize, 0.55) : line
  );
}

const PARENT_PORTAL_TOP_CARD_MIN_W = 300;
const PARENT_PORTAL_CONTROL_CARD_MIN_W = 245;

function rowFrameTone(): 'gold' | 'silver' | 'bronze' | 'blue' | 'red' {
  return 'blue';
}

function ParentPortalTopCarouselCard({
  item,
  x,
  y,
  w,
  h,
  selected,
  onSelect,
  onHoverChange,
  controlHoverAnchor = 'center',
  cfg,
}: {
  item: ParentPortalTopCardItem;
  x: number;
  y: number;
  w: number;
  h: number;
  selected: boolean;
  onSelect: () => void;
  onHoverChange?: (item: ParentPortalTopCardItem | null) => void;
  controlHoverAnchor?: 'center' | 'up';
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const rawControlClipId = useId();
  const controlClipId = `parent-control-card-${rawControlClipId.replace(/:/g, '')}`;
  const color = item.themeColor ?? toneColor(item.tone, cfg);
  const active = selected || hovered;
  const rowFrameConfig = useMemo(() => {
    if (item.kind !== 'row') return null;
    return createGoldenFrameVariantConfig({
      rank: String(item.row.order),
      name: item.row.label,
      statName: 'Global',
      statValue: item.row.signal,
      tone: rowFrameTone(),
    });
  }, [item]);
  const [rowFrameHref, setRowFrameHref] = useState('');
  useEffect(() => {
    setRowFrameHref(rowFrameConfig === null ? '' : createGoldenFrameFrameOnlySvgDataUri(rowFrameConfig));
  }, [rowFrameConfig]);
  const compactControlCard = item.kind === 'control' && !hovered;
  const controlW = item.kind === 'control' ? (hovered ? Math.min(w + 28, w * 1.08) : w) : w;
  const controlH =
    item.kind === 'control' ? (hovered ? clampValue(h * 1.82, 86, 112) : clampValue(h * 0.62, 38, 44)) : h;
  const controlX = x + (w - controlW) / 2;
  const controlY =
    item.kind === 'control' && hovered && controlHoverAnchor === 'up' ? y + h - controlH - 4 : y + (h - controlH) / 2;
  const controlPad = Math.max(10, Math.min(14, controlW * 0.045));
  const controlBannerX = controlX + controlPad;
  const controlBannerY = controlY + 18;
  const controlBannerW = controlW - controlPad * 2;
  const controlBannerH = Math.max(78, Math.min(controlH * 0.68, controlH - 58));
  const controlBodyY = controlBannerY + controlBannerH + 18;
  const controlImageUrl = item.kind === 'control' ? parentPortalControlArtworkUrl(item.control) : null;
  const controlCategoryText = item.kind === 'control' ? controlSubcategoryLabel(item.control) : '';
  const controlTitleW = Math.max(58, controlW - controlPad * 2 - 4);
  const controlTitleSize = fitSingleLineTextSize(item.title, controlTitleW, 13.5, 20, 0.56);
  const controlTitleBaseline = Math.min(controlY + controlH - 16, controlBodyY + controlTitleSize * 0.42);
  const controlCategorySize = fitSingleLineTextSize(controlCategoryText, controlBannerW - 20, 7.5, 10.5, 0.56);
  const controlTitleText = truncateTextForWidth(item.title, controlTitleW, controlTitleSize, 0.56);
  const controlCategoryDisplayText = truncateTextForWidth(
    controlCategoryText,
    controlBannerW - 34,
    controlCategorySize,
    0.56
  );
  const compactControlImageSize = Math.min(31, Math.max(23, controlH - 16));
  const compactControlImageX = controlX + 10;
  const compactControlImageY = controlY + (controlH - compactControlImageSize) / 2;
  const compactControlChipValue = item.kind === 'control' ? compactControlStatLabel(item.value) : '';
  const compactControlChipText = /\d|%/.test(compactControlChipValue) ? compactControlChipValue : '';
  const compactControlChipW = compactControlChipText
    ? Math.max(38, Math.min(54, compactControlChipText.length * 6.2 + 20))
    : 0;
  const compactControlTitleX = compactControlImageX + compactControlImageSize + 14;
  const compactControlTitleW = Math.max(46, controlX + controlW - compactControlTitleX - compactControlChipW - 18);
  const compactControlTitleSize = fitSingleLineTextSize(item.title, compactControlTitleW, 11.8, 14.2, 0.58);
  const rowFrameScale = Math.min(w / 1536, h / 864);
  const rowFrameW = 1536 * rowFrameScale;
  const rowFrameH = 864 * rowFrameScale;
  const rowFrameX = x + (w - rowFrameW) / 2;
  const rowFrameY = y + (h - rowFrameH) / 2;
  const rowHoverScale = hovered ? 1.075 : selected ? 1.012 : 1;
  const rowScaleCx = x + w / 2;
  const rowScaleCy = y + h / 2;
  const scaleFromCenter = (value: number, center: number) => center + (value - center) * rowHoverScale;
  const rowDrawFrameX = scaleFromCenter(rowFrameX, rowScaleCx);
  const rowDrawFrameY = scaleFromCenter(rowFrameY, rowScaleCy);
  const rowDrawFrameW = rowFrameW * rowHoverScale;
  const rowDrawFrameH = rowFrameH * rowHoverScale;
  const rowHoverBoxX = rowDrawFrameX + rowDrawFrameW * 0.035;
  const rowHoverBoxY = rowDrawFrameY + rowDrawFrameH * 0.13;
  const rowHoverBoxW = rowDrawFrameW * 0.93;
  const rowHoverBoxH = rowDrawFrameH * 0.72;
  const guidePad = Math.max(12, Math.min(18, w * 0.045));
  const guideRankW = 42;
  const guideTitleX = x + guidePad + guideRankW + 10;
  const guideTitleW = Math.max(60, w - guidePad * 2 - guideRankW - 12);
  const guideTitleSize = fitSingleLineTextSize(item.title, guideTitleW, 13, 18, 0.56);
  const guideSubtitleLines = wrapCardText(item.subtitle, guideTitleW, 10.6, 2);
  const guideDetailLines = wrapCardText(item.detail, w - guidePad * 2, 10.2, h > 102 ? 2 : 1);
  const rowHoverStrokeWidth = hovered ? 2.4 : selected ? 1.35 : 1.5;
  const rowHoverOuterOpacity = hovered ? 0.72 : selected ? 0.34 : 0.38;
  const rowHoverInnerOpacity = hovered ? 0.82 : selected ? 0.48 : 0.52;
  const rowHoverFill = hovered ? colorAlpha(color, '2c') : selected ? colorAlpha(color, '24') : colorAlpha(color, '16');
  const hitX = item.kind === 'control' ? Math.min(x - 4, controlX - 8) : x - 4;
  const hitY = item.kind === 'control' ? Math.min(y - 4, controlY - 8) : y - 4;
  const hitW = item.kind === 'control' ? Math.max(x + w + 4, controlX + controlW + 8) - hitX : w + 8;
  const hitH = item.kind === 'control' ? Math.max(y + h + 4, controlY + controlH + 8) - hitY : h + 8;
  const showHoverState = () => {
    setHovered(true);
    if (item.kind === 'control') onHoverChange?.(item);
  };
  const clearHoverState = () => {
    setHovered(false);
    if (item.kind === 'control') onHoverChange?.(null);
  };
  return (
    <g
      className="parent-portal-svg-clickable"
      role="button"
      tabIndex={0}
      aria-label={
        item.kind === 'control'
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
      {item.kind === 'control' && active && !compactControlCard ? (
        <path
          d={cutRectPath(x, y, w, h, 14)}
          fill={colorAlpha(color, selected ? '24' : '12')}
          stroke={color}
          strokeWidth={selected ? 2.1 : 1.4}
          opacity={selected ? 0.82 : 0.5}
          filter="url(#parentPortalGlow)"
          pointerEvents="none"
        />
      ) : null}
      {item.kind === 'row' ? (
        <>
          {active ? (
            <>
              <path
                d={cutRectPath(rowHoverBoxX - 8, rowHoverBoxY - 8, rowHoverBoxW + 16, rowHoverBoxH + 16, 13)}
                fill="none"
                stroke={color}
                strokeWidth={rowHoverStrokeWidth}
                opacity={rowHoverOuterOpacity}
                filter="url(#parentPortalGlow)"
                pointerEvents="none"
              />
              <path
                d={cutRectPath(rowHoverBoxX, rowHoverBoxY, rowHoverBoxW, rowHoverBoxH, 10)}
                fill={rowHoverFill}
                stroke={color}
                strokeWidth={hovered ? 1.6 : 1.1}
                strokeOpacity={rowHoverInnerOpacity}
                pointerEvents="none"
              />
            </>
          ) : null}
          {rowFrameHref ? (
            <image
              href={rowFrameHref}
              xlinkHref={rowFrameHref}
              x={rowDrawFrameX}
              y={rowDrawFrameY}
              width={rowDrawFrameW}
              height={rowDrawFrameH}
              preserveAspectRatio="xMidYMid meet"
              filter={active ? 'url(#parentPortalGlow)' : undefined}
              pointerEvents="none"
            />
          ) : null}
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.13}
            y={rowDrawFrameY + rowDrawFrameH * 0.36}
            fontSize={Math.max(12, rowDrawFrameH * 0.1)}
            fontWeight={950}
            fill={cfg.colors.bodyText}
            pointerEvents="none"
          >
            {item.title}
          </text>
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.13}
            y={rowDrawFrameY + rowDrawFrameH * 0.52}
            fontSize={Math.max(8, rowDrawFrameH * 0.064)}
            fontWeight={820}
            fill={cfg.colors.mutedText}
            pointerEvents="none"
          >
            {item.row.primaryArea}
          </text>
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.72}
            y={rowDrawFrameY + rowDrawFrameH * 0.42}
            textAnchor="middle"
            fontSize={Math.max(11, rowDrawFrameH * 0.082)}
            fontWeight={950}
            fill={color}
            pointerEvents="none"
          >
            {item.value}
          </text>
          <text
            x={rowDrawFrameX + rowDrawFrameW * 0.72}
            y={rowDrawFrameY + rowDrawFrameH * 0.56}
            textAnchor="middle"
            fontSize={Math.max(8, rowDrawFrameH * 0.056)}
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
            fill={active ? colorAlpha(color, selected ? '2e' : '20') : PARENT_PORTAL_GLASS.cardFillStrong}
            stroke={color}
            strokeWidth={selected ? 2 : hovered ? 1.55 : 1.05}
            strokeOpacity={selected ? 0.92 : hovered ? 0.8 : 0.56}
            filter={selected || hovered ? 'url(#parentPortalGlow)' : undefined}
            pointerEvents="none"
          />
          <path
            d={cutRectPath(x + 5, y + 5, w - 10, h - 10, 10)}
            fill="url(#parentPortalFrameGlass)"
            stroke={color}
            strokeWidth={0.75}
            strokeOpacity={active ? 0.48 : 0.26}
            pointerEvents="none"
          />
          <path
            d={bottomCutRectPath(x + w * 0.34, y - 5, w * 0.32, 12, 5)}
            fill={color}
            fillOpacity={active ? 0.42 : 0.24}
            stroke={color}
            strokeWidth={1}
            filter="url(#parentPortalGlow)"
            pointerEvents="none"
          />
          <path
            d={cutRectPath(x + guidePad, y + 17, guideRankW, 30, 7)}
            fill={colorAlpha(color, selected ? '44' : '28')}
            stroke={color}
            strokeWidth={selected ? 1.2 : 0.8}
            pointerEvents="none"
          />
          <text
            x={x + guidePad + guideRankW / 2}
            y={y + 37}
            textAnchor="middle"
            fontSize={13}
            fontWeight={950}
            fill={color}
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
          {compactControlCard ? (
            <>
              <path
                d={cutRectPath(controlX, controlY, controlW, controlH, 8)}
                fill={selected ? colorAlpha(color, '30') : PARENT_PORTAL_GLASS.controlFill}
                stroke={color}
                strokeWidth={selected ? 1.65 : 0.95}
                strokeOpacity={selected ? 0.96 : 0.7}
                filter={selected ? 'url(#parentPortalGlow)' : undefined}
                pointerEvents="none"
              />
              <ArtworkSlot
                x={compactControlImageX}
                y={compactControlImageY}
                w={compactControlImageSize}
                h={compactControlImageSize}
                label={`${item.title} image`}
                imageUrl={controlImageUrl}
                tone={item.tone}
                compact
                shape="rect"
                imageFit="slice"
                cfg={cfg}
              />
              <line
                x1={compactControlImageX + compactControlImageSize + 7}
                y1={controlY + 8}
                x2={compactControlImageX + compactControlImageSize + 7}
                y2={controlY + controlH - 8}
                stroke={color}
                strokeWidth={0.85}
                opacity={selected ? 0.64 : 0.42}
              />
              <text
                x={compactControlTitleX}
                y={controlY + controlH / 2 + compactControlTitleSize * 0.34}
                fontSize={compactControlTitleSize}
                fontWeight={950}
                fill={cfg.colors.bodyText}
              >
                {truncateTextForWidth(item.title, compactControlTitleW, compactControlTitleSize, 0.58)}
              </text>
              {compactControlChipText ? (
                <>
                  <path
                    d={cutRectPath(
                      controlX + controlW - compactControlChipW - 12,
                      controlY + (controlH - 24) / 2,
                      compactControlChipW,
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
                    x={controlX + controlW - 12 - compactControlChipW / 2}
                    y={controlY + controlH / 2 + 4}
                    textAnchor="middle"
                    fontSize={10.2}
                    fontWeight={950}
                    fill={color}
                  >
                    {compactControlChipText}
                  </text>
                </>
              ) : null}
            </>
          ) : (
            <>
              {active ? (
                <path
                  d={cutRectPath(controlX - 5, controlY - 5, controlW + 10, controlH + 10, 16)}
                  fill="none"
                  stroke={color}
                  strokeWidth={hovered ? 2.3 : 1.6}
                  opacity={hovered ? 0.48 : 0.34}
                  filter="url(#parentPortalGlow)"
                  pointerEvents="none"
                />
              ) : null}
              <path
                d={cutRectPath(controlX, controlY, controlW, controlH, 15)}
                fill={active ? colorAlpha(color, selected ? '24' : '18') : PARENT_PORTAL_GLASS.controlFill}
                stroke={active ? color : cfg.colors.panelStroke}
                strokeWidth={selected ? 2 : hovered ? 1.65 : 1.05}
                strokeOpacity={active ? 0.94 : 0.68}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(controlX + 4, controlY + 4, controlW - 8, controlH - 8, 13)}
                fill="url(#parentPortalFrameGlass)"
                stroke={color}
                strokeWidth={active ? 1.1 : 0.7}
                opacity={active ? 0.56 : 0.36}
                pointerEvents="none"
              />
              <path
                d={bottomCutRectPath(controlX + controlW * 0.32, controlY - 6, controlW * 0.36, 13, 5)}
                fill={color}
                fillOpacity={0.34}
                stroke={color}
                strokeWidth={1.1}
                filter="url(#parentPortalGlow)"
                pointerEvents="none"
              />
              <path
                d={bottomCutRectPath(controlX + controlW * 0.34, controlY - 3, controlW * 0.32, 7, 3)}
                fill={color}
                fillOpacity={0.48}
                pointerEvents="none"
              />
              <defs>
                <clipPath id={controlClipId}>
                  <path d={cutRectPath(controlBannerX, controlBannerY, controlBannerW, controlBannerH, 11)} />
                </clipPath>
              </defs>
              {controlImageUrl ? (
                <image
                  href={controlImageUrl}
                  x={controlBannerX}
                  y={controlBannerY}
                  width={controlBannerW}
                  height={controlBannerH}
                  preserveAspectRatio="xMidYMid slice"
                  clipPath={`url(#${controlClipId})`}
                  opacity={0.96}
                  pointerEvents="none"
                />
              ) : (
                <ArtworkSlot
                  x={controlBannerX}
                  y={controlBannerY}
                  w={controlBannerW}
                  h={controlBannerH}
                  label={`${item.title} image`}
                  imageUrl={null}
                  tone={item.tone}
                  shape="rect"
                  cfg={cfg}
                />
              )}
              <path
                d={cutRectPath(controlBannerX, controlBannerY, controlBannerW, controlBannerH, 11)}
                fill="url(#parentPortalCardBannerShade)"
                clipPath={`url(#${controlClipId})`}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(controlBannerX, controlBannerY, controlBannerW, controlBannerH, 11)}
                fill="none"
                stroke={color}
                strokeWidth={active ? 1.2 : 0.85}
                strokeOpacity={active ? 0.86 : 0.56}
                pointerEvents="none"
              />
              <path
                d={cutRectPath(
                  controlBannerX + 8,
                  controlBannerY + 7,
                  Math.min(controlBannerW - 16, controlCategoryDisplayText.length * 5.8 + 28),
                  20,
                  6
                )}
                fill={PARENT_PORTAL_GLASS.panelFillDeep}
                stroke="rgba(255,255,255,0.16)"
                strokeWidth={0.7}
                pointerEvents="none"
              />
              <text
                x={controlBannerX + 18}
                y={controlBannerY + 21.5}
                fontSize={controlCategorySize}
                fontWeight={900}
                fill="#e6f9ff"
              >
                {controlCategoryDisplayText}
              </text>
              <text
                x={controlX + controlPad + 2}
                y={controlTitleBaseline}
                fontSize={controlTitleSize}
                fontWeight={950}
                fill={cfg.colors.bodyText}
              >
                {controlTitleText}
              </text>
            </>
          )}
        </>
      )}
    </g>
  );
}

function ParentPortalTopCarousel({
  x,
  y,
  w,
  h,
  items,
  page,
  selectedKey,
  onSelect,
  onHoverChange,
  minCardW = PARENT_PORTAL_TOP_CARD_MIN_W,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  items: ParentPortalTopCardItem[];
  page: number;
  selectedKey: string;
  onSelect: (item: ParentPortalTopCardItem) => void;
  onHoverChange?: (item: ParentPortalTopCardItem | null) => void;
  minCardW?: number;
  cfg: ParentPortalSvgControls;
}) {
  const allRowItems = items.length > 0 && items.every((item) => item.kind === 'row');
  const allGuideItems = items.length > 0 && items.every((item) => item.kind === 'guide');
  const gap = allRowItems ? 10 : 16;
  const visibleCount = Math.max(
    1,
    Math.min(items.length || 1, PARENT_PORTAL_TOP_CAROUSEL_MAX_VISIBLE, Math.floor((w + gap) / (minCardW + gap)))
  );
  const pageCount = Math.max(1, Math.ceil(items.length / visibleCount));
  const safePage = wrapIndex(page, pageCount);
  const visibleItems = items.slice(safePage * visibleCount, safePage * visibleCount + visibleCount);
  const stretchedCardW =
    visibleItems.length > 0 ? (w - gap * Math.max(0, visibleItems.length - 1)) / visibleItems.length : w;
  const rowMaxCardW = visibleItems.length <= 3 ? 370 : 340;
  const guideMaxCardW = visibleItems.length <= 2 ? 520 : 430;
  const cardW = allRowItems
    ? Math.min(stretchedCardW, rowMaxCardW)
    : allGuideItems
      ? Math.min(stretchedCardW, guideMaxCardW)
      : stretchedCardW;
  const trackW = visibleItems.length > 0 ? visibleItems.length * cardW + Math.max(0, visibleItems.length - 1) * gap : 0;
  const startX = x + Math.max(0, (w - trackW) / 2);
  return (
    <g>
      {visibleItems.map((item, index) => {
        const selected = item.key === selectedKey || item.key.startsWith(`${selectedKey}:`);
        return (
          <ParentPortalTopCarouselCard
            key={item.key}
            item={item}
            x={startX + index * (cardW + gap)}
            y={y}
            w={cardW}
            h={h}
            selected={selected}
            onSelect={() => onSelect(item)}
            {...(onHoverChange === undefined ? {} : { onHoverChange })}
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

function ControlCategoryCard({
  category,
  x,
  y,
  w,
  h,
  selected,
  themeColor,
  onSelect,
  cfg,
}: {
  category: ControlCategorySummary;
  x: number;
  y: number;
  w: number;
  h: number;
  selected: boolean;
  themeColor?: string;
  onSelect: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const [hovered, setHovered] = useState(false);
  const active = selected || hovered;
  const color = themeColor ?? toneColor(category.tone, cfg);
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
      className="parent-portal-svg-clickable"
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
        fill={
          selected ? colorAlpha(color, '30') : hovered ? colorAlpha(color, '22') : PARENT_PORTAL_GLASS.cardFillStrong
        }
        stroke={color}
        strokeWidth={selected ? 1.65 : hovered ? 1.35 : 0.95}
        strokeOpacity={active ? 0.96 : 0.68}
        filter={active ? 'url(#parentPortalGlow)' : undefined}
        pointerEvents="none"
      />
      <ArtworkSlot
        x={imageX}
        y={imageY}
        w={imageSize}
        h={imageSize}
        label={`${category.label} artwork`}
        imageUrl={parentPortalControlCategoryImageUrl(category)}
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

function ControlCategoryGrid({
  x,
  y,
  w,
  h,
  categories,
  selectedCategoryId,
  page,
  pageCount,
  themeColor,
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
  categories: ControlCategorySummary[];
  selectedCategoryId: string;
  page: number;
  pageCount: number;
  themeColor?: string;
  handleLeftX?: number;
  handleRightX?: number;
  onPageChange: (page: number) => void;
  onSelect: (category: ControlCategorySummary) => void;
  cfg: ParentPortalSvgControls;
}) {
  if (categories.length === 0) return null;
  const gap = 10;
  const handleW = PARENT_PORTAL_SIDE_HANDLE_W;
  const handleH = Math.max(40, h - 10);
  const handleY = y + (h - handleH) / 2;
  const trackX = x;
  const trackW = Math.max(1, w);
  const cardW = (trackW - gap * Math.max(0, categories.length - 1)) / categories.length;
  const paged = pageCount > 1;
  return (
    <g>
      {paged ? (
        <>
          <ParentPortalFrameSideHandle
            x={handleLeftX ?? x}
            y={handleY}
            side="left"
            height={handleH}
            width={handleW}
            disabled={false}
            onClick={() => onPageChange(wrapIndex(page - 1, pageCount))}
            {...(themeColor === undefined ? {} : { accentColor: themeColor })}
            cfg={cfg}
          />
          <ParentPortalFrameSideHandle
            x={handleRightX ?? x + w - handleW}
            y={handleY}
            side="right"
            height={handleH}
            width={handleW}
            disabled={false}
            onClick={() => onPageChange(wrapIndex(page + 1, pageCount))}
            {...(themeColor === undefined ? {} : { accentColor: themeColor })}
            cfg={cfg}
          />
        </>
      ) : null}
      {categories.map((category, index) => {
        const cardX = trackX + index * (cardW + gap);
        return (
          <ControlCategoryCard
            key={category.id}
            category={category}
            x={cardX}
            y={y}
            w={cardW}
            h={h}
            selected={category.id === selectedCategoryId}
            {...(themeColor === undefined ? {} : { themeColor })}
            onSelect={() => onSelect(category)}
            cfg={cfg}
          />
        );
      })}
    </g>
  );
}

function ControlSubcategoryGrid({
  x,
  y,
  w,
  h,
  category,
  selectedSubcategoryId,
  themeColor,
  onSelect,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  category: ControlCategorySummary;
  selectedSubcategoryId: string | null;
  themeColor?: string;
  onSelect: (subcategory: ControlSubcategorySummary) => void;
  cfg: ParentPortalSvgControls;
}) {
  const subcategories = category.subcategories;
  if (subcategories.length === 0) return null;
  const categoryColor = themeColor ?? toneColor(category.tone, cfg);
  return (
    <foreignObject x={x} y={y} width={w} height={h}>
      <div
        className="parent-portal-subcategory-grid"
        style={
          {
            '--parent-portal-subcategory-accent': categoryColor,
            '--parent-portal-subcategory-text': cfg.colors.bodyText,
            '--parent-portal-subcategory-muted': cfg.colors.mutedText,
          } as CSSProperties
        }
        onClick={(event) => event.stopPropagation()}
        onWheel={(event) => event.stopPropagation()}
      >
        {subcategories.map((subcategory) => {
          const selected = selectedSubcategoryId === subcategory.id;
          const color = themeColor ?? toneColor(subcategory.tone, cfg);
          const style = {
            '--parent-portal-subcategory-color': color,
            '--parent-portal-subcategory-fill': selected ? colorAlpha(color, '30') : PARENT_PORTAL_GLASS.cardFill,
            '--parent-portal-subcategory-border': color,
          } as CSSProperties;
          return (
            <button
              key={subcategory.id}
              type="button"
              className={`parent-portal-subcategory-grid__item${selected ? ' parent-portal-subcategory-grid__item--selected' : ''}`}
              style={style}
              aria-label={`Filter ${category.label} by ${subcategory.label}`}
              aria-pressed={selected}
              onClick={(event) => {
                event.stopPropagation();
                onSelect(subcategory);
              }}
            >
              <span className="parent-portal-subcategory-grid__dot" />
              <span className="parent-portal-subcategory-grid__divider" />
              <span className="parent-portal-subcategory-grid__label">{subcategory.label}</span>
              <span className="parent-portal-subcategory-grid__count">{subcategory.count}</span>
            </button>
          );
        })}
      </div>
    </foreignObject>
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
        filter="url(#parentPortalGlow)"
        pointerEvents="none"
      />
      {arrow ? (
        <path
          d={`M ${x + w - 10} ${y + 14} L ${x + w + 10} ${y + h / 2} L ${x + w - 10} ${y + h - 14} Z`}
          fill={color}
          opacity={active ? 0.72 : 0.56}
          filter="url(#parentPortalGlow)"
          pointerEvents="none"
        />
      ) : null}
    </>
  );
}

function MainBoard({
  activeNavLabel,
  activeNavItem,
  activeNavGroupId,
  activeTab,
  rows,
  parentPortalRows,
  tabDetails,
  controlAreas,
  quickControls,
  guideTopics,
  season,
  uiCopy,
  selectedControlId,
  selectedRowId,
  selectedRow,
  selectedControlName,
  rowsPerPage,
  detailMode,
  onTabChange,
  onRowSelect,
  onPageChange,
  onDetailClose,
  onControlSelect,
  onNavigate,
  onAgentCommand,
  onReconnectLocalService,
  onSelectNavLabel,
  activityState,
  sourceRowsUnavailable,
  lanPairingAutoScanSequence,
  cfg,
  mainX,
  mainW,
  mainY,
  mainH,
}: {
  activeNavLabel: string;
  activeNavItem?: NavItem | null;
  activeNavGroupId: string;
  activeTab: ParentPortalTabId;
  rows: DisplayRow[];
  parentPortalRows: ParentPortalRow[];
  tabDetails: ParentPortalContentData['tabDetails'];
  controlAreas: ControlArea[];
  quickControls: QuickControl[];
  guideTopics: ParentPortalContentData['guideTopics'];
  season: ParentPortalContentData['season'];
  uiCopy: ParentPortalContentData['uiCopy'];
  selectedControlId: string;
  selectedRowId: string;
  selectedRow: DisplayRow;
  selectedControlName: string;
  rowsPerPage: number;
  detailMode: DetailMode | null;
  onTabChange: (tab: ParentPortalTabId) => void;
  onRowSelect: (rowId: string) => void;
  onPageChange: (page: number) => void;
  onDetailClose: () => void;
  onControlSelect: (controlId: string) => void;
  onNavigate?: (routePath: string) => void;
  onAgentCommand?: (command: AgentCommandName, payload: Record<string, string>) => void;
  onReconnectLocalService: () => void;
  onSelectNavLabel: (navLabel: string) => void;
  activityState?: ParentPortalActivityState | null;
  sourceRowsUnavailable: boolean;
  lanPairingAutoScanSequence: number;
  cfg: ParentPortalSvgControls;
  mainX: number;
  mainW: number;
  mainY: number;
  mainH: number;
}) {
  const detail = detailForNav(activeNavLabel, tabDetails[activeTab], activeNavItem);
  const activeNavKey = assetKey(activeNavLabel);
  const guideOverviewMode = activeNavKey.includes('start-here');
  const guideEligible = activeNavGroupId === 'guide';
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
  const guideDefaultTopicId = guideTopicPool[0]?.id ?? '';
  const [selectedGuideTopicId, setSelectedGuideTopicId] = useState(() => guideTopicPool[0]?.id ?? '');
  const [guidePage, setGuidePage] = useState(0);
  const [guideQuickPanelMode, setGuideQuickPanelMode] = useState<'read' | 'action'>('read');
  const [guideDashboardDrilldown, setGuideDashboardDrilldown] = useState(false);
  const selectedGuideTopic =
    guideTopicPool.find((topic) => normalizeSelectionId(topic.id) === normalizeSelectionId(selectedGuideTopicId)) ??
    guideTopicPool[0] ??
    null;
  const guideRouteFocus = guideMode ? guideRouteFocusFromHash() : null;
  const guideRouteFocusTopicId = guideRouteFocus?.topicId ?? '';
  const guideRouteFocusPage = guideRouteFocus?.page ?? 0;
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
    if (guideDefaultTopicId) {
      setSelectedGuideTopicId(guideDefaultTopicId);
    }
    setGuideDashboardDrilldown(false);
    setGuidePage(0);
    setGuideQuickPanelMode('read');
  }, [activeNavKey, guideDefaultTopicId]);
  useEffect(() => {
    if (!guideMode || !guideRouteFocusTopicId) return;
    const targetTopic = guideTopicPool.find(
      (topic) => normalizeSelectionId(topic.id) === normalizeSelectionId(guideRouteFocusTopicId)
    );
    if (!targetTopic) return;
    setSelectedGuideTopicId(targetTopic.id);
    setGuidePage(clampValue(guideRouteFocusPage, 0, Math.max(0, targetTopic.pages.length - 1)));
    setGuideDashboardDrilldown(true);
    setGuideQuickPanelMode('read');
  }, [guideMode, guideRouteFocusPage, guideRouteFocusTopicId, guideTopicPool]);
  useEffect(() => {
    setGuideQuickPanelMode('read');
  }, [selectedGuideTopicId]);
  const guideTopicById = useMemo(
    () => new Map(guideTopics.map((topic) => [normalizeSelectionId(topic.id), topic])),
    [guideTopics]
  );
  const handleGuideNoteSelect = (note: ParentPortalGuideNote) => {
    if (isHashRoutePath(note.targetRoutePath)) {
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
  const tableVariant = tableVariantForContext(activeNavLabel, activeTab);
  const isOverviewContext = activeNavKey.includes('overview') || activeNavKey.includes('today');
  const sortedRows = useMemo(() => [...rows].sort((a, b) => a.order - b.order), [rows]);
  const controlScopedRows = useMemo(
    () => rowsForControlScope(sortedRows, selectedControlName, selectedControlId),
    [selectedControlId, selectedControlName, sortedRows]
  );
  const controlCategories = useMemo(() => buildControlCategorySummaries(quickControls), [quickControls]);
  const selectedQuickControl = useMemo(
    () =>
      quickControls.find((control) => normalizeSelectionId(control.id) === normalizeSelectionId(selectedControlId)) ??
      quickControls[0],
    [quickControls, selectedControlId]
  );
  const [selectedCategoryIdOverride, setSelectedCategoryIdOverride] = useState<string | null>(null);
  const [selectedSubcategoryIdOverride, setSelectedSubcategoryIdOverride] = useState<string | null>(null);
  const [expandedCategoryId, setExpandedCategoryId] = useState<string | null>(null);
  const [hoveredTopControlKey, setHoveredControlAreaKey] = useState<string | null>(null);
  const [lanPairingScanRequestedAtMs, setLanPairingScanRequestedAtMs] = useState<number | null>(null);
  const lanPairingScanStartedAfterEventIdRef = useRef<AgentEventId | null>(null);
  const latestLanPairingScanEventIdRef = useRef<AgentEventId | null>(null);
  const manageLane = selectedQuickControl ? manageLaneForControl(selectedQuickControl) : 'childPolicy';
  const selectedControlCategoryId =
    selectedCategoryIdOverride ??
    (selectedQuickControl ? assetKey(controlCategoryLabel(selectedQuickControl)) : (controlCategories[0]?.id ?? ''));
  const selectedCategory =
    controlCategories.find((category) => category.id === selectedControlCategoryId) ?? controlCategories[0] ?? null;
  const selectedCategoryLabel =
    selectedCategory?.label ?? (selectedQuickControl ? controlCategoryLabel(selectedQuickControl) : 'Controls');
  const selectedSubcategoryId = selectedCategory?.subcategories.some(
    (subcategory) => subcategory.id === selectedSubcategoryIdOverride
  )
    ? selectedSubcategoryIdOverride
    : null;
  const perCategoryMode = activeNavKey.includes('category');
  const aiBrowserMode = tableVariant === 'ai' && (activeNavKey.includes('game') || activeNavKey.includes('category'));
  const categoryBrowserMode = perCategoryMode || activeNavKey.includes('ai-by-category');
  const manageMode = activeNavGroupId === 'manage';
  const activeGroupThemeColor = navGroupThemeColor(activeNavGroupId, cfg);
  const manageThemeTone = manageMode ? 'cyan' : detail.tone;
  const activeFrameTone = activeNavItem?.tone ?? manageThemeTone;
  const activeFrameIcon =
    activeNavKey.includes('ai') || activeNavKey.includes('memory')
      ? AiMemorySetBrainIcon
      : (activeNavItem?.icon ?? OverviewListIcon);
  const activeFrameRawTitle = activeNavItem?.label ?? activeNavLabel;
  const manageCurrentSpec = useMemo(
    () => (manageMode ? manageControlSpecFor(activeNavLabel, selectedControlName) : null),
    [activeNavLabel, manageMode, selectedControlName]
  );
  const manageActivityUiIntent = useMemo(
    () => createParentPortalActivityUiIntent(activityState, ACTIVITY_REPORT_BASIC_CHILD_DEVICE_SEATS),
    [activityState]
  );
  const manageLanPairingSlots = useMemo(
    () => createParentPortalLanPairingUiSlots(parentPortalRows, activityState?.lanAddDeviceReadModel),
    [activityState?.lanAddDeviceReadModel, parentPortalRows]
  );
  const manageRuntimeDeviceSlots = useMemo(
    () => createParentPortalCanonicalDeviceSlots(manageActivityUiIntent.deviceSlots, manageLanPairingSlots),
    [manageActivityUiIntent.deviceSlots, manageLanPairingSlots]
  );
  const manageWorkspaceKind =
    manageMode && manageCurrentSpec
      ? manageWorkspaceKindFor(activeNavLabel, selectedControlName, manageCurrentSpec.title)
      : null;
  const activeFrameTitle =
    manageWorkspaceKind === 'policy' ? managePolicyAreaLabel(activeNavLabel, selectedControlName) : activeFrameRawTitle;
  const manageWorkspaceFullFrameMode = Boolean(manageWorkspaceKind);
  const manageWorkspaceSupportsPerDevice = useMemo(
    () =>
      manageWorkspaceKind !== null &&
      manageWorkspaceTargetOptions(manageWorkspaceKind).some((option) => option.id === 'perDevice'),
    [manageWorkspaceKind]
  );
  const manageWorkspaceHeaderIcon =
    manageWorkspaceKind === 'portal'
      ? PortalGatewayIcon
      : manageWorkspaceKind === 'account'
        ? AccountProfileIcon
        : manageWorkspaceKind === 'data'
          ? DataPrivacyServerShieldIcon
          : manageWorkspaceKind === 'ai'
            ? AiMemorySetBrainIcon
            : manageWorkspaceKind === 'policy'
              ? managePolicyAreaIcon(activeNavLabel, selectedControlName)
              : activeFrameIcon;
  const manageWorkspaceHeaderTitle =
    manageWorkspaceKind === 'policy'
      ? managePolicyAreaLabel(activeNavLabel, selectedControlName)
      : manageWorkspaceKind
        ? manageWorkspaceTitle(manageWorkspaceKind)
        : activeFrameTitle;
  const manageBrowserTargets = useMemo(
    () => manageBrowserTargetsForKey(activeNavLabel, selectedControlName),
    [activeNavLabel, selectedControlName]
  );
  const [manageTargetSelection, setManageTargetSelection] = useState<ManageTargetSelection>(
    () => readStoredManageTargetSelection() ?? defaultManageTargetSelection()
  );
  useEffect(() => {
    writeStoredManageTargetSelection(manageTargetSelection);
  }, [manageTargetSelection]);
  const manageTargetContextKey = `${manageMode ? 'manage' : 'browse'}:${activeNavLabel}:${selectedControlName}:${
    manageCurrentSpec?.title ?? ''
  }:${manageLane}`;
  const previousManageTargetContextKeyRef = useRef<string | null>(null);
  useEffect(() => {
    if (!manageMode || !manageCurrentSpec) return;
    const contextChanged = previousManageTargetContextKeyRef.current !== manageTargetContextKey;
    previousManageTargetContextKeyRef.current = manageTargetContextKey;
    const defaultScope = manageInitialScopeForSpec(manageLane, manageCurrentSpec, manageRuntimeDeviceSlots);
    const defaultDevice = isLanPairingManageTitle(manageCurrentSpec.title)
      ? ''
      : manageDefaultDeviceSelection(manageCurrentSpec, manageRuntimeDeviceSlots);
    const defaultDeviceId = defaultDevice;
    const defaultBrowser = manageBrowserTargets[0]?.label ?? 'All targets';
    setManageTargetSelection((current) => {
      const preservePerDeviceReportScope =
        isReportsManageTitle(manageCurrentSpec.title) &&
        current.scope === 'perDevice' &&
        (current.deviceId.length > 0 || current.device.length > 0);
      const preservePerDeviceWorkspaceScope =
        manageWorkspaceSupportsPerDevice &&
        current.scope === 'perDevice' &&
        (current.deviceId.length > 0 || current.device.length > 0);
      const nextScope =
        contextChanged && (preservePerDeviceReportScope || preservePerDeviceWorkspaceScope)
          ? 'perDevice'
          : contextChanged
            ? defaultScope
            : current.scope;
      const deviceChoicesAvailable =
        manageDeviceChoices(manageCurrentSpec.devices, manageRuntimeDeviceSlots).length > 0;
      const currentSelectedSlot = reportSelectedSlot(manageRuntimeDeviceSlots, current);
      const currentDeviceAvailable = reportDeviceSelectionAvailable(manageRuntimeDeviceSlots, current);
      const nextDevice =
        nextScope !== 'perDevice'
          ? defaultDevice
          : currentSelectedSlot
            ? selectedDeviceIdentity(currentSelectedSlot)
            : currentDeviceAvailable || (current.device.length > 0 && !deviceChoicesAvailable)
              ? current.device
              : defaultDevice;
      const nextDeviceId =
        nextScope !== 'perDevice'
          ? defaultDeviceId
          : currentSelectedSlot
            ? currentSelectedSlot.value
            : currentDeviceAvailable || (current.deviceId.length > 0 && !deviceChoicesAvailable)
              ? current.deviceId
              : defaultDevice;
      const nextBrowser =
        !contextChanged && manageBrowserTargets.some((target) => target.label === current.browser)
          ? current.browser
          : defaultBrowser;
      if (
        current.scope === nextScope &&
        current.device === nextDevice &&
        current.deviceId === nextDeviceId &&
        current.browser === nextBrowser
      ) {
        return current;
      }
      return {
        scope: nextScope,
        device: nextDevice,
        deviceId: nextDeviceId,
        browser: nextBrowser,
      };
    });
  }, [
    manageBrowserTargets,
    manageCurrentSpec,
    manageLane,
    manageMode,
    manageRuntimeDeviceSlots,
    manageTargetContextKey,
    manageWorkspaceSupportsPerDevice,
  ]);
  const controlBrowserMode = !guideMode && !manageMode && (tableVariant === 'controls' || aiBrowserMode);
  const expandedControlCategory =
    controlBrowserMode && selectedCategory && expandedCategoryId === selectedCategory.id ? selectedCategory : null;
  const filteredCategoryControls = quickControls.filter(
    (control) => assetKey(controlCategoryLabel(control)) === selectedControlCategoryId
  );
  const filteredSubcategoryControls = selectedSubcategoryId
    ? filteredCategoryControls.filter((control) => assetKey(controlSubcategoryLabel(control)) === selectedSubcategoryId)
    : filteredCategoryControls;
  const sortedCategoryControls = [...filteredSubcategoryControls].sort(
    (a, b) =>
      (a.controlCode ?? Number.MAX_SAFE_INTEGER) - (b.controlCode ?? Number.MAX_SAFE_INTEGER) ||
      a.name.localeCompare(b.name)
  );
  const controlBrowserPoolBase = controlBrowserMode ? sortedCategoryControls : quickControls;
  const controlBrowserPool =
    activeNavGroupId === 'manage'
      ? quickControls.filter((control) => manageLaneForControl(control) === manageLane)
      : controlBrowserPoolBase;
  const controlAreasById = new Map(controlAreas.map((control) => [normalizeSelectionId(control.id), control]));
  const topItems: ParentPortalTopCardItem[] = (
    guideMode
      ? guideTopicPool.map(guideTopCard)
      : controlBrowserMode
        ? controlBrowserPool.map((control, index) =>
            controlTopCard(control, controlAreasById.get(normalizeSelectionId(control.id)), index)
          )
        : sortedRows.slice(0, isOverviewContext ? 3 : 10).map(rowTopCard)
  ).map((item) => ({ ...item, themeColor: activeGroupThemeColor }));
  const selectedTopKey =
    guideMode && selectedGuideTopic
      ? `guide:${normalizeSelectionId(selectedGuideTopic.id)}`
      : controlBrowserMode
        ? `control:${normalizeSelectionId(selectedControlId)}`
        : `row:${selectedRowId}`;
  const managePortalSection = manageMode && (manageLane === 'portal' || manageWorkspaceFullFrameMode);
  const lanPairingDeviceGridMode =
    manageMode && manageCurrentSpec ? isLanPairingManageTitle(manageCurrentSpec.title) : false;
  const activityManageGridMode =
    manageMode && manageCurrentSpec ? isReportsManageTitle(manageCurrentSpec.title) : false;
  const manageDeviceGridMode = lanPairingDeviceGridMode || activityManageGridMode;
  const latestLanPairingScanEventId = activityState?.lanPairingBrowserDiscoveryEvent?.eventId ?? null;
  useEffect(() => {
    latestLanPairingScanEventIdRef.current = latestLanPairingScanEventId;
  }, [latestLanPairingScanEventId]);
  useEffect(() => {
    if (!lanPairingDeviceGridMode || lanPairingAutoScanSequence <= 0) return;
    lanPairingScanStartedAfterEventIdRef.current = latestLanPairingScanEventIdRef.current;
    setLanPairingScanRequestedAtMs(Date.now());
  }, [lanPairingAutoScanSequence, lanPairingDeviceGridMode]);
  useEffect(() => {
    if (
      latestLanPairingScanEventId === null ||
      latestLanPairingScanEventId === lanPairingScanStartedAfterEventIdRef.current
    ) {
      return;
    }
    setLanPairingScanRequestedAtMs(null);
  }, [latestLanPairingScanEventId]);
  useEffect(() => {
    if (lanPairingScanRequestedAtMs === null) return;
    const timeoutId = window.setTimeout(
      () => setLanPairingScanRequestedAtMs(null),
      PortalLanPairingScan.PendingIndicatorMs
    );
    return () => window.clearTimeout(timeoutId);
  }, [lanPairingScanRequestedAtMs]);
  const manageSharedWorkspaceFrameMode = manageDeviceGridMode || manageWorkspaceFullFrameMode;
  const manageTopSelectorRequired = (!manageMode || !managePortalSection) && !manageDeviceGridMode;
  const detailPanelCanFocus = manageTopSelectorRequired;
  const focusContextKey = `${activeNavLabel}:${activeTab}`;
  const [focusState, setFocusState] = useState<{ contextKey: string; section: ParentPortalFocusSection }>(() => ({
    contextKey: focusContextKey,
    section: 'highlights',
  }));
  const focusedSection = focusState.contextKey === focusContextKey ? focusState.section : 'highlights';
  const setFocusedSection = (section: ParentPortalFocusSection) =>
    setFocusState({ contextKey: focusContextKey, section });
  const tableFocused = focusedSection === 'table';
  const showTopSection =
    !tableFocused &&
    manageTopSelectorRequired &&
    !(sourceRowsUnavailable && isOverviewContext && topItems.length === 0);
  const compactProofPanelSelector = activeNavKey === 'proof-panels' && mainW < 680;
  const compactDeveloperStatusPanel = activeNavKey === 'events' || activeNavKey === 'logs';
  const sectionGap = Math.max(8, Math.min(cfg.layout.gap, 14));
  const expandedTopPanelH = Math.max(276, Math.min(mainH - 210, clampValue(mainH * 0.46, 276, 334)));
  const hoverTopPanelH = Math.max(242, Math.min(mainH - 210, clampValue(mainH * 0.4, 242, 292)));
  const compactTopPanelH = Math.max(178, Math.min(mainH - 250, clampValue(mainH * 0.29, 178, 214)));
  const topPanelH = !showTopSection
    ? 0
    : compactProofPanelSelector
      ? 72
      : compactDeveloperStatusPanel
        ? mainW < 680
          ? 104
          : 154
        : manageMode
          ? clampValue(mainH * 0.23, 178, 228)
          : guideDashboardMode
            ? mainH
            : guideMode
              ? clampValue(mainH * 0.24, 164, 220)
              : controlBrowserMode
                ? hoveredTopControlKey
                  ? Math.max(hoverTopPanelH, expandedControlCategory ? expandedTopPanelH : 0)
                  : expandedControlCategory
                    ? expandedTopPanelH
                    : compactTopPanelH
                : clampValue(mainH * 0.39, 260, 294);
  const bottomPanelY = showTopSection ? mainY + topPanelH + sectionGap : mainY;
  const bottomPanelH = showTopSection ? mainH - topPanelH - sectionGap : mainH;
  const selectorHandleGutter = PARENT_PORTAL_SIDE_HANDLE_W;
  const rowHandleReserve = (PARENT_PORTAL_SIDE_HANDLE_W + 8) * 2;
  const topCardGap = 10;
  const topCardMinW = guideMode
    ? 250
    : controlBrowserMode
      ? PARENT_PORTAL_CONTROL_CARD_MIN_W
      : PARENT_PORTAL_TOP_CARD_MIN_W;
  const fullSelectorInnerW = Math.max(1, mainW - 36);
  const fullTopCarouselAvailableW = controlBrowserMode
    ? Math.max(1, fullSelectorInnerW - rowHandleReserve)
    : Math.max(1, mainW - 56);
  const fullCategoryTrackW = Math.max(1, fullSelectorInnerW - rowHandleReserve);
  const fullCategoryVisibleCount = Math.max(
    1,
    Math.min(controlCategories.length || 1, Math.floor((fullCategoryTrackW + 8) / 178), 6)
  );
  const fullCategoryPageCount = Math.max(1, Math.ceil(controlCategories.length / fullCategoryVisibleCount));
  const fullTopCardVisibleCount = Math.max(
    1,
    Math.min(
      topItems.length || 1,
      PARENT_PORTAL_TOP_CAROUSEL_MAX_VISIBLE,
      Math.floor((fullTopCarouselAvailableW + topCardGap) / (topCardMinW + topCardGap))
    )
  );
  const fullTopFramePageCount = Math.max(1, Math.ceil(topItems.length / fullTopCardVisibleCount));
  const topFrameUsesHandleGutter =
    showTopSection &&
    !manageMode &&
    !guideDashboardMode &&
    (fullTopFramePageCount > 1 || (controlBrowserMode && fullCategoryPageCount > 1));
  const selectorX = mainX + (topFrameUsesHandleGutter ? selectorHandleGutter : 0);
  const selectorW = Math.max(1, mainW - (topFrameUsesHandleGutter ? selectorHandleGutter * 2 : 0));
  const selectorInnerW = Math.max(1, selectorW - 36);
  const categoryTrackW = Math.max(1, selectorInnerW - rowHandleReserve);
  const categoryVisibleCount = Math.max(
    1,
    Math.min(controlCategories.length || 1, Math.floor((categoryTrackW + 8) / 178), 6)
  );
  const [categoryPage, setCategoryPage] = useState(0);
  const categoryPageCount = Math.max(1, Math.ceil(controlCategories.length / categoryVisibleCount));
  const safeCategoryPage = wrapIndex(categoryPage, categoryPageCount);
  const visibleControlCategories = controlCategories.slice(
    safeCategoryPage * categoryVisibleCount,
    safeCategoryPage * categoryVisibleCount + categoryVisibleCount
  );
  const tableFrame = parentPortalFrameRects(mainX, bottomPanelY, mainW, bottomPanelH);
  const tableY = bottomPanelY + 47;
  const rowStep = cfg.chrome.rowHeight + cfg.chrome.rowGap;
  const visibleRowCapacity = Math.max(1, Math.floor((tableFrame.body.y + tableFrame.body.h - tableY - 42) / rowStep));
  const effectiveRowsPerPage = Math.max(1, Math.min(rowsPerPage, visibleRowCapacity));
  const categoryScopedRows = rowsForCategoryScope(sortedRows, selectedCategoryLabel);
  const aiControlScopedRows = rowsForControlScope(sortedRows, selectedControlName, selectedControlId);
  const tableRows =
    tableVariant === 'controls'
      ? categoryBrowserMode
        ? categoryScopedRows
        : controlScopedRows
      : tableVariant === 'ai' && activeNavKey.includes('ai-by-category')
        ? categoryScopedRows
        : tableVariant === 'ai' && activeNavKey.includes('ai-by-game')
          ? aiControlScopedRows
          : isOverviewContext
            ? [
                ...sortedRows.filter((row) => row.id === selectedRowId && row.order <= 3),
                ...sortedRows.filter((row) => row.order > 3),
              ]
            : sortedRows;
  const [highlightPage, setHighlightPage] = useState(0);
  const topCarouselAvailableW = controlBrowserMode
    ? Math.max(1, selectorInnerW - rowHandleReserve)
    : Math.max(1, selectorW - 56);
  const topCardVisibleCount = Math.max(
    1,
    Math.min(
      topItems.length || 1,
      PARENT_PORTAL_TOP_CAROUSEL_MAX_VISIBLE,
      Math.floor((topCarouselAvailableW + topCardGap) / (topCardMinW + topCardGap))
    )
  );
  const highlightPageCount = Math.max(1, Math.ceil(topItems.length / topCardVisibleCount));
  const safeHighlightPage = wrapIndex(highlightPage, highlightPageCount);
  const shiftHighlightPage = (delta: number) =>
    setHighlightPage((value) => wrapIndex(value + delta, highlightPageCount));
  const framePage = safeHighlightPage;
  const framePageCount = highlightPageCount;
  const topFramePaged = topFrameUsesHandleGutter && !controlBrowserMode && framePageCount > 1;
  const topFrameFooterH = topFramePaged ? 30 : manageMode || guideDashboardMode ? 18 : 8;
  const shiftFramePage = (delta: number) => {
    shiftHighlightPage(delta);
  };
  const routeControlCategoryToBottom = (category: ControlCategorySummary) => {
    setFocusedSection('highlights');
    setSelectedCategoryIdOverride(category.id);
    setSelectedSubcategoryIdOverride(null);
    setExpandedCategoryId((value) => (value === category.id ? null : category.id));
    setHighlightPage(0);
    const nextRows =
      category.count > 0
        ? rowsForControlScope(sortedRows, category.sampleControl.name, category.sampleControl.id)
        : rowsForCategoryScope(sortedRows, category.label);
    if (nextRows[0]) onRowSelect(nextRows[0].id);
    if (category.count > 0) onControlSelect(category.sampleControl.id);
    onPageChange(1);
  };
  const routeControlSubcategoryToBottom = (subcategory: ControlSubcategorySummary) => {
    setFocusedSection('highlights');
    setSelectedSubcategoryIdOverride(subcategory.id);
    setExpandedCategoryId(null);
    setHighlightPage(0);
    const nextRows = rowsForControlScope(sortedRows, subcategory.sampleControl.name, subcategory.sampleControl.id);
    if (nextRows[0]) onRowSelect(nextRows[0].id);
    onControlSelect(subcategory.sampleControl.id);
    onPageChange(1);
  };
  const routeTopItemToBottom = (item: ParentPortalTopCardItem) => {
    setFocusedSection('highlights');
    if (item.kind === 'guide') {
      setSelectedGuideTopicId(item.topic.id);
      setGuidePage(0);
      onPageChange(1);
      return;
    }
    if (item.kind === 'control') {
      const nextCategoryId = assetKey(controlCategoryLabel(item.control));
      setSelectedCategoryIdOverride(nextCategoryId);
      setSelectedSubcategoryIdOverride(assetKey(controlSubcategoryLabel(item.control)));
      setExpandedCategoryId(null);
      const nextCategoryIndex = controlCategories.findIndex((category) => category.id === nextCategoryId);
      if (nextCategoryIndex >= 0) {
        setCategoryPage(Math.floor(nextCategoryIndex / categoryVisibleCount));
      }
      const nextRows = rowsForControlScope(sortedRows, item.control.name, item.control.id);
      if (nextRows[0]) onRowSelect(nextRows[0].id);
      onControlSelect(item.control.id);
      onPageChange(1);
      return;
    }
    onRowSelect(item.row.id);
    const targetIndex = tableRows.findIndex((row) => row.id === item.row.id);
    if (targetIndex >= 0) {
      onPageChange(Math.floor(targetIndex / effectiveRowsPerPage) + 1);
    }
  };
  const handleHighlightsWheel = (event: WheelEvent<SVGGElement>) => {
    const delta = Math.abs(event.deltaX) > Math.abs(event.deltaY) ? event.deltaX : event.deltaY;
    if (framePageCount <= 1 || Math.abs(delta) < 8) return;
    event.preventDefault();
    event.stopPropagation();
    shiftFramePage(delta > 0 ? 1 : -1);
  };
  const tableHeaderButtonY = bottomPanelY + 13;
  const manageGuideRoutePath =
    manageMode && manageCurrentSpec ? guideRoutePathForManageKey(activeNavLabel, selectedControlName) : null;
  const lanPairingScanAvailable = onAgentCommand !== undefined;
  const manageDeviceGridScanAction = lanPairingDeviceGridMode ? (
    <ParentPortalHeaderAction
      x={mainX + mainW - 154}
      y={bottomPanelY + 18}
      w={122}
      h={29}
      tone="cyan"
      accentColor={activeGroupThemeColor}
      active
      label={
        lanPairingScanAvailable
          ? lanPairingScanRequestedAtMs !== null
            ? PortalLanPairingScan.Text.Scanning
            : PortalLanPairingScan.Text.Scan
          : resolvePortalDevText(PortalDevTextToken.RetryStatus)
      }
      iconHref={PortalAssets.LanPairingScanIcon}
      onClick={() => {
        if (!onAgentCommand) {
          onReconnectLocalService();
          return;
        }
        setLanPairingScanRequestedAtMs(Date.now());
        onAgentCommand(AgentCommand.LanPairingBrowserDiscoveryScan, {
          [PortalAgentProtocolField.LanRouteId]: PortalAgentTargetDefaults.LocalNetworkWindowsAgent.route,
        });
      }}
      ariaLabel={
        lanPairingScanAvailable
          ? PortalLanPairingScan.Text.ScanLocalAreaNetwork
          : resolvePortalDevText(PortalDevTextToken.RetryStatus)
      }
      cfg={cfg}
    />
  ) : null;
  const manageHeaderInfoLabel =
    manageMode && manageCurrentSpec && manageGuideRoutePath
      ? lanPairingDeviceGridMode
        ? PortalLanPairingScan.Text.OpenLocalAreaNetworkGuide
        : `Open ${manageControlDisplayTitle(manageCurrentSpec.title)} guide`
      : undefined;
  const tableHeaderAction =
    manageGuideRoutePath && tableFocused && manageTopSelectorRequired ? (
      <ParentPortalHeaderAction
        x={mainX + mainW - 162}
        y={tableHeaderButtonY + 2}
        w={82}
        h={25}
        tone="gold"
        accentColor={activeGroupThemeColor}
        active
        label="TOP"
        onClick={() => setFocusedSection('highlights')}
        ariaLabel="Show target section"
        cfg={cfg}
      />
    ) : !manageGuideRoutePath && tableFocused ? (
      <ParentPortalHeaderAction
        x={mainX + mainW - 118}
        y={tableHeaderButtonY + 2}
        w={98}
        h={25}
        tone="gold"
        accentColor={activeGroupThemeColor}
        active
        label="TOP"
        onClick={() => setFocusedSection('highlights')}
        ariaLabel="Show highlights section"
        cfg={cfg}
      />
    ) : detailPanelCanFocus ? (
      <ParentPortalHeaderAction
        x={mainX + mainW - 118}
        y={tableHeaderButtonY + 2}
        w={98}
        h={25}
        tone="cyan"
        accentColor={activeGroupThemeColor}
        label="EXPAND"
        onClick={() => setFocusedSection('table')}
        ariaLabel="Expand parent detail panel"
        cfg={cfg}
      />
    ) : guideOverviewMode && guideDashboardDrilldown ? (
      <ParentPortalHeaderAction
        x={mainX + mainW - 118}
        y={tableHeaderButtonY + 2}
        w={98}
        h={25}
        tone="cyan"
        accentColor={activeGroupThemeColor}
        active
        label="MAP"
        onClick={() => setGuideDashboardDrilldown(false)}
        ariaLabel="Show guide setup map"
        cfg={cfg}
      />
    ) : null;
  return (
    <g
      className="parent-portal-study-main-board"
      data-ocentra-proof-panels-layout={compactProofPanelSelector ? 'compact' : undefined}
      data-ocentra-developer-status-layout={compactDeveloperStatusPanel ? 'compact' : undefined}
    >
      {showTopSection ? (
        <ParentPortalSectionFrame
          x={selectorX}
          y={mainY}
          w={selectorW}
          h={topPanelH}
          title={activeFrameTitle}
          subtitle={
            manageMode && manageCurrentSpec
              ? 'Choose global, child device, and browser target before editing below'
              : guideDashboardMode
                ? 'Set up parent app, child devices, controls, privacy, alerts, and storage'
                : guideMode && selectedGuideTopic
                  ? `${selectedGuideTopic.subtitle} / ${selectedGuideTopic.detail}`
                  : `${detail.eyebrow} / ${detail.primary}`
          }
          headerIcon={activeFrameIcon}
          tone={activeFrameTone}
          {...(activeGroupThemeColor === undefined ? {} : { accentColor: activeGroupThemeColor })}
          headerH={manageMode ? 44 : controlBrowserMode ? 48 : 40}
          footerH={topFrameFooterH}
          {...(manageMode || controlBrowserMode ? { bodyStrokeOpacity: 0 } : {})}
          {...(manageMode || controlBrowserMode ? { bodyFill: PARENT_PORTAL_FRAME_MATERIAL.transparentFill } : {})}
          {...(topFramePaged && !controlBrowserMode ? {} : { footerLineOpacity: 0 })}
          headerRight={null}
          showSideHandles={topFramePaged && !controlBrowserMode}
          sideDisabled={false}
          onPrevious={() => shiftFramePage(-1)}
          onNext={() => shiftFramePage(1)}
          selected={focusedSection === 'highlights'}
          footer={(footerRect) => (
            <>
              {topFramePaged ? (
                <ParentPortalFrameDots
                  x={footerRect.x + footerRect.w / 2}
                  y={footerRect.y + (controlBrowserMode ? 14 : 18)}
                  page={framePage}
                  pageCount={framePageCount}
                  accentColor={activeGroupThemeColor}
                  onPageChange={setHighlightPage}
                  cfg={cfg}
                />
              ) : null}
              {topFramePaged ? (
                <text
                  x={footerRect.x + footerRect.w - 22}
                  y={footerRect.y + (controlBrowserMode ? 18 : 23)}
                  textAnchor="end"
                  fontSize={controlBrowserMode ? 9.5 : 10}
                  fontWeight={900}
                  fill={cfg.colors.mutedText}
                >
                  {`${guideMode ? 'TOPICS' : controlBrowserMode ? 'AREAS' : isOverviewContext ? 'READY' : 'ITEMS'} ${
                    framePage + 1
                  }/${framePageCount}`}
                </text>
              ) : null}
            </>
          )}
          cfg={cfg}
        >
          {(body) => {
            const contentX = body.x + 2;
            const contentY = body.y + (controlBrowserMode ? 2 : -2);
            const contentW = body.w - 4;
            const contentH = body.h + (controlBrowserMode ? -2 : 4);
            const expandedCategory = expandedControlCategory;
            const categoryH = controlBrowserMode
              ? clampValue(
                  contentH * (expandedCategory ? 0.17 : 0.28),
                  expandedCategory ? 34 : 28,
                  expandedCategory ? 42 : 34
                )
              : 0;
            const subcategoryH = expandedCategory ? clampValue(contentH * 0.32, 82, 110) : 0;
            const categoryGap = controlBrowserMode ? 4 : 0;
            const subcategoryGap = expandedCategory ? 4 : 0;
            const subcategoryY = contentY + categoryH + categoryGap;
            const carouselY = subcategoryY + subcategoryH + subcategoryGap;
            const carouselH = expandedCategory
              ? Math.max(64, contentH - categoryH - categoryGap - subcategoryH - subcategoryGap)
              : Math.max(48, contentH - categoryH - categoryGap);
            const rowHandleW = PARENT_PORTAL_SIDE_HANDLE_W;
            const rowHandleLeftX = selectorX - rowHandleW + PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
            const rowHandleRightX = selectorX + selectorW - PARENT_PORTAL_SIDE_HANDLE_OVERLAP;
            const carouselTrackX = contentX;
            const carouselTrackW = contentW;
            const carouselHandleH = controlBrowserMode ? Math.max(44, Math.min(92, carouselH - 8)) : 0;
            const carouselHandleY = carouselY + Math.max(0, (carouselH - carouselHandleH) / 2);
            return (
              <g onWheel={handleHighlightsWheel}>
                <rect x={contentX - 4} y={contentY - 4} width={contentW + 8} height={contentH + 8} fill="transparent" />
                {manageMode && manageCurrentSpec ? (
                  <ManageTargetPanel
                    x={contentX + 8}
                    y={contentY + 10}
                    w={contentW - 16}
                    activeNavLabel={activeNavLabel}
                    selectedControlName={selectedControlName}
                    spec={manageCurrentSpec}
                    lane={manageLane}
                    runtimeDeviceSlots={manageRuntimeDeviceSlots}
                    themeColor={activeGroupThemeColor}
                    targetSelection={manageTargetSelection}
                    onTargetChange={setManageTargetSelection}
                    cfg={cfg}
                  />
                ) : guideDashboardMode ? (
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
                    themeColor={activeGroupThemeColor}
                    cfg={cfg}
                  />
                ) : controlBrowserMode ? (
                  <ControlCategoryGrid
                    x={contentX}
                    y={contentY}
                    w={contentW}
                    h={categoryH}
                    categories={visibleControlCategories}
                    selectedCategoryId={selectedControlCategoryId}
                    page={safeCategoryPage}
                    pageCount={categoryPageCount}
                    themeColor={activeGroupThemeColor}
                    handleLeftX={rowHandleLeftX}
                    handleRightX={rowHandleRightX}
                    onPageChange={setCategoryPage}
                    onSelect={routeControlCategoryToBottom}
                    cfg={cfg}
                  />
                ) : null}
                {!guideDashboardMode && expandedCategory ? (
                  <ControlSubcategoryGrid
                    x={carouselTrackX}
                    y={subcategoryY}
                    w={carouselTrackW}
                    h={subcategoryH}
                    category={expandedCategory}
                    selectedSubcategoryId={selectedSubcategoryId}
                    themeColor={activeGroupThemeColor}
                    onSelect={routeControlSubcategoryToBottom}
                    cfg={cfg}
                  />
                ) : null}
                {!guideDashboardMode && controlBrowserMode && highlightPageCount > 1 ? (
                  <>
                    <ParentPortalFrameSideHandle
                      x={rowHandleLeftX}
                      y={carouselHandleY}
                      side="left"
                      height={carouselHandleH}
                      width={rowHandleW}
                      disabled={highlightPageCount <= 1}
                      onClick={() => shiftHighlightPage(-1)}
                      accentColor={activeGroupThemeColor}
                      cfg={cfg}
                    />
                    <ParentPortalFrameSideHandle
                      x={rowHandleRightX}
                      y={carouselHandleY}
                      side="right"
                      height={carouselHandleH}
                      width={rowHandleW}
                      disabled={highlightPageCount <= 1}
                      onClick={() => shiftHighlightPage(1)}
                      accentColor={activeGroupThemeColor}
                      cfg={cfg}
                    />
                  </>
                ) : null}
                {!guideDashboardMode && !manageMode ? (
                  <ParentPortalTopCarousel
                    x={carouselTrackX}
                    y={controlBrowserMode ? carouselY : contentY}
                    w={carouselTrackW}
                    h={controlBrowserMode ? carouselH : contentH}
                    items={topItems}
                    page={safeHighlightPage}
                    selectedKey={selectedTopKey}
                    onSelect={routeTopItemToBottom}
                    onHoverChange={(item) => setHoveredControlAreaKey(item?.kind === 'control' ? item.key : null)}
                    minCardW={topCardMinW}
                    cfg={cfg}
                  />
                ) : null}
              </g>
            );
          }}
        </ParentPortalSectionFrame>
      ) : null}
      {!guideDashboardMode ? (
        <ParentPortalSectionFrame
          x={mainX}
          y={bottomPanelY}
          w={mainW}
          h={bottomPanelH}
          title={
            lanPairingDeviceGridMode
              ? PortalLanPairingScan.Text.HeaderTitle
              : manageSharedWorkspaceFrameMode
                ? manageWorkspaceHeaderTitle.toUpperCase()
                : manageWorkspaceHeaderTitle
          }
          headerIcon={lanPairingDeviceGridMode ? LanNetworkMonitorsIcon : manageWorkspaceHeaderIcon}
          tone={activeFrameTone}
          {...(activeGroupThemeColor === undefined ? {} : { accentColor: activeGroupThemeColor })}
          headerRight={manageDeviceGridMode ? manageDeviceGridScanAction : tableHeaderAction}
          {...(manageHeaderInfoLabel === undefined ? {} : { headerInfoLabel: manageHeaderInfoLabel })}
          {...(manageHeaderInfoLabel && manageGuideRoutePath && onNavigate
            ? { onHeaderInfoClick: () => onNavigate(manageGuideRoutePath) }
            : {})}
          {...(manageSharedWorkspaceFrameMode ? { headerH: 58 } : {})}
          {...(manageSharedWorkspaceFrameMode ? { footerH: 0 } : {})}
          {...(manageSharedWorkspaceFrameMode ? { bodyInset: 0 } : {})}
          fullHeaderLine={manageSharedWorkspaceFrameMode}
          bodyStrokeOpacity={0}
          bodyFill={PARENT_PORTAL_FRAME_MATERIAL.transparentFill}
          {...(manageSharedWorkspaceFrameMode ? { footerLineOpacity: 0 } : {})}
          selected={detailPanelCanFocus ? tableFocused : false}
          cfg={cfg}
        >
          {(body) => {
            const detailInset = manageSharedWorkspaceFrameMode ? 0 : 18;
            return (
              <ParentPortalDetailPanel
                x={body.x + detailInset}
                y={body.y + detailInset}
                w={body.w - detailInset * 2}
                h={body.h - detailInset * 2}
                activeNavLabel={activeNavLabel}
                activeNavGroupId={activeNavGroupId}
                detail={detail}
                rows={tableRows}
                parentPortalRows={parentPortalRows}
                selectedControlName={selectedControlName}
                themeTone={manageThemeTone}
                themeColor={activeGroupThemeColor}
                {...(selectedGuideTopic === undefined ? {} : { guideTopic: selectedGuideTopic })}
                guidePage={guidePage}
                onGuidePageChange={setGuidePage}
                quickPanelMode={guideQuickPanelMode}
                onQuickPanelModeChange={setGuideQuickPanelMode}
                onGuideNoteSelect={handleGuideNoteSelect}
                {...(manageTargetSelection === undefined ? {} : { manageTargetSelection })}
                onManageTargetChange={setManageTargetSelection}
                {...(activityState === undefined ? {} : { activityState })}
                {...(onNavigate === undefined ? {} : { onNavigate })}
                {...(onAgentCommand === undefined ? {} : { onAgentCommand })}
                cfg={cfg}
              />
            );
          }}
        </ParentPortalSectionFrame>
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
          controlAreas={controlAreas}
          uiCopy={uiCopy}
          selectedRow={selectedRow}
          selectedControlName={selectedControlName}
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
  controlAreas,
  uiCopy,
  selectedRow,
  selectedControlName,
  onTabChange,
  onClose,
  cfg,
}: {
  x: number;
  y: number;
  w: number;
  h: number;
  mode: DetailMode;
  activeTab: ParentPortalTabId;
  detail: TabDetail;
  season: ParentPortalContentData['season'];
  controlAreas: ControlArea[];
  uiCopy: ParentPortalContentData['uiCopy'];
  selectedRow: DisplayRow;
  selectedControlName: string;
  onTabChange: (tab: ParentPortalTabId) => void;
  onClose: () => void;
  cfg: ParentPortalSvgControls;
}) {
  const tone: Tone = mode === 'season' ? 'gold' : mode === 'control' ? 'purple' : selectedRow.tone;
  const color = toneColor(tone, cfg);
  const selectedControl = controlAreas.find(
    (control) => control.name === selectedControlName || control.id === selectedControlName
  );
  const title =
    mode === 'season'
      ? season.detailTitle
      : mode === 'control'
        ? `${selectedControlName.toUpperCase()} DRILLDOWN`
        : `${selectedRow.label.toUpperCase()} PROFILE`;
  const subtitle =
    mode === 'season'
      ? season.detailSubtitle
      : mode === 'control'
        ? `Control detail from ${detail.eyebrow}`
        : `${detail.eyebrow} / ${selectedControlName}`;
  const stats =
    mode === 'season'
      ? season.stats.map((stat) => [stat.label, stat.value] as [string, string])
      : mode === 'control'
        ? [
            ['MATCHES', selectedControl?.matches ?? 'N/A'],
            ['GROWTH', selectedControl?.growth ?? 'N/A'],
            ['AREAS', '100'],
          ]
        : [
            ['SIGNAL', selectedRow.signal],
            ['READINESS', selectedRow.readiness],
            ['TREND', selectedRow.trend],
          ];
  const primaryTab: ParentPortalTabId = mode === 'season' ? 'routines' : mode === 'control' ? 'controls' : activeTab;
  const primaryLabel = mode === 'season' ? 'OPEN EVENTS' : mode === 'control' ? 'OPEN CONTROL' : 'KEEP SELECTED';
  const artworkImageUrl =
    mode === 'season'
      ? parentPortalOverviewBannerImageUrl
      : mode === 'control'
        ? parentPortalControlImageUrl(selectedControl?.id ?? selectedControlName)
        : rowAvatarImageUrl(selectedRow.label);
  return (
    <g role="dialog" aria-label={title}>
      <path
        d={cutRectPath(x - 8, y - 8, w + 16, h + 16, 18)}
        fill="rgba(1, 5, 12, 0.58)"
        stroke={color}
        strokeWidth={1.2}
        opacity={0.98}
      />
      <path
        d={cutRectPath(x, y, w, h, 16)}
        fill="rgba(5, 17, 30, 0.76)"
        stroke={color}
        strokeWidth={1.4}
        filter="url(#parentPortalGlow)"
      />
      <ArtworkSlot
        x={x + 22}
        y={y + 24}
        w={92}
        h={82}
        label={mode === 'season' ? 'SEASON' : mode === 'control' ? 'CONTROL ART' : 'ROW'}
        imageUrl={artworkImageUrl}
        tone={tone}
        shape={mode === 'row' ? 'circle' : 'hex'}
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
      <ParentPortalHeaderAction
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
      <ParentPortalHeaderAction
        x={x + w - 68}
        y={y + 26}
        w={44}
        h={30}
        tone="muted"
        label="X"
        onClick={onClose}
        ariaLabel="Close parent portal detail"
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

function Defs() {
  return (
    <defs>
      <radialGradient id="parentPortalLoaderBg" cx="50%" cy="50%" r="72%">
        <stop offset="0%" stopColor="rgb(0, 110, 104)" />
        <stop offset="70%" stopColor="rgb(0, 50, 100)" />
        <stop offset="100%" stopColor="rgb(0, 5, 15)" />
      </radialGradient>
      <linearGradient id="parentPortalFrameFill" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop
          offset="0%"
          stopColor={PortalUnifiedChrome.CssVarRefs.FrameSurfaceColor}
          stopOpacity={PortalUnifiedChrome.CssVarRefs.FrameSurfaceOpacity}
        />
        <stop
          offset="100%"
          stopColor={PortalUnifiedChrome.CssVarRefs.FrameSurfaceColor}
          stopOpacity={PortalUnifiedChrome.CssVarRefs.FrameSurfaceOpacity}
        />
      </linearGradient>
      <linearGradient id="parentPortalFrameGlass" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop
          offset="0%"
          stopColor={PortalUnifiedChrome.CssVarRefs.FrameGlassColor}
          stopOpacity={PortalUnifiedChrome.CssVarRefs.FrameGlassOpacity}
        />
        <stop
          offset="100%"
          stopColor={PortalUnifiedChrome.CssVarRefs.FrameGlassColor}
          stopOpacity={PortalUnifiedChrome.CssVarRefs.FrameGlassOpacity}
        />
      </linearGradient>
      <linearGradient id="parentPortalCardBannerShade" x1="0%" y1="0%" x2="0%" y2="100%">
        <stop offset="0%" stopColor="#07111f" stopOpacity="0.16" />
        <stop offset="52%" stopColor="#06101f" stopOpacity="0.46" />
        <stop offset="100%" stopColor="#061525" stopOpacity="0.92" />
      </linearGradient>
      <filter id="parentPortalGlow" x="-35%" y="-35%" width="170%" height="170%">
        <feGaussianBlur stdDeviation={3.5} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
      <filter id="parentPortalGoldGlow" x="-60%" y="-60%" width="220%" height="220%">
        <feGaussianBlur stdDeviation={4} result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
    </defs>
  );
}

export function ParentPortalSvgSurface({
  pageMode,
  controlCode,
  parentPortalRows,
  userEntry,
  controlId,
  loading = false,
  error = null,
  statusMessage = null,
  controls,
  content,
  initialNavLabel,
  initialSelectedControlId,
  assistantRouteActive = false,
  assistantRoutePath = '#/assistant',
  assistantReturnRoutePath = '#/overview',
  assistantCommandAvailable = false,
  assistantResponse = null,
  activityState = null,
  lanPairingAutoScanSequence = 0,
  workspaceVisible = true,
  onRefreshParentPortal,
  onNavigate,
  onAssistantCommand,
  onInitialLayoutReady,
}: ParentPortalSvgSurfaceProps) {
  const mainRef = useRef<HTMLElement | null>(null);
  const baseCfg = useMemo(() => normalizeParentPortalSvgControls(controls), [controls]);
  const pageContent = useMemo(() => normalizeParentPortalContent(content), [content]);
  const tabDetails = pageContent.tabDetails;
  const navItems = useMemo<NavItem[]>(
    () =>
      pageContent.navItems.map((item) => ({
        ...item,
        icon: iconForNavItem(item),
        imageUrl: navItemImageUrl(item),
      })),
    [pageContent.navItems]
  );
  const navGroups = useMemo(
    () => groupedParentPortalNavItems(pageContent.navGroups, navItems),
    [navItems, pageContent.navGroups]
  );
  const controlAreas = pageContent.controlAreas;
  const quickControls = useMemo<QuickControl[]>(() => {
    const controlEntries = pageContent.quickControls.filter(isParentPortalControlEntry).map((control) => ({
      ...control,
      icon: iconForName(control.icon),
    }));
    if (controlEntries.length > 0) return controlEntries;
    return pageContent.controlAreas.map(
      (control): QuickControl => ({
        id: control.id,
        name: control.name,
        detail: typeof control.controlCode === 'number' ? `Area ${control.controlCode}` : 'View parent portal',
        icon: ManageFileSettingsIcon,
        tone: control.tone,
        ...(control.category === undefined ? {} : { category: control.category }),
        ...(control.subcategory === undefined ? {} : { subcategory: control.subcategory }),
        ...(control.controlCode === undefined ? {} : { controlCode: control.controlCode }),
        ...(control.routePath === undefined ? {} : { routePath: control.routePath }),
      })
    );
  }, [pageContent.quickControls, pageContent.controlAreas]);
  const renderTransitionKey = `${pageMode}:${controlId ?? ''}:${initialNavLabel ?? ''}:${
    initialSelectedControlId ?? ''
  }:${assistantRouteActive ? 'assistant' : 'main'}`;
  const [surfaceSize, setSurfaceSize] = useState({ width: 0, height: 0 });
  const [surfaceMeasured, setSurfaceMeasured] = useState(false);
  const [initialRenderSettled, setInitialRenderSettled] = useState(false);
  const [routeRenderPending, setRouteRenderPending] = useState(false);
  const initialRenderTimerRef = useRef<number | undefined>(undefined);
  const routeRenderTimerRef = useRef<number | undefined>(undefined);
  const previousRenderTransitionKeyRef = useRef(renderTransitionKey);
  const initialLayoutReadyReportedRef = useRef(false);
  const mobileLayout = isParentPortalMobileSurface(surfaceSize);
  const canvasSize = useMemo(
    () => parentPortalCanvasSizeForSurface(baseCfg, surfaceSize),
    [baseCfg, surfaceSize.height, surfaceSize.width]
  );
  const columns = useMemo(
    () => responsiveParentPortalColumnWidths(canvasSize.width, baseCfg),
    [baseCfg, canvasSize.width]
  );
  const cfg = useMemo<ParentPortalSvgControls>(
    () => ({
      ...baseCfg,
      canvas: {
        ...baseCfg.canvas,
        width: canvasSize.width,
        height: canvasSize.height,
      },
      layout: {
        ...baseCfg.layout,
        gap: mobileLayout ? 0 : baseCfg.layout.gap,
        leftW: columns.leftW,
        rightW: columns.rightW,
      },
    }),
    [baseCfg, canvasSize.height, canvasSize.width, columns.leftW, columns.rightW, mobileLayout]
  );
  const pageModeTab = initialTabForPageMode(pageMode, pageContent);
  const initialSelectedControlIdForRoute = initialControlIdForPageMode(pageMode, pageContent, controlId);
  const initialNavItem = initialNavItemForContext(
    navItems,
    pageContent,
    initialNavLabel,
    initialSelectedControlId ?? initialSelectedControlIdForRoute,
    pageModeTab,
    preferredNavGroupIdForPageMode(pageMode)
  );
  const initialTab = initialNavItem?.tabId ?? pageModeTab;
  const [activeTab, setActiveTab] = useState<ParentPortalTabId>(initialTab);
  const [activeNavLabel, setActiveNavLabel] = useState(
    () => initialNavItem?.label ?? initialNavLabelForTab(navItems, pageModeTab)
  );
  const [activeNavRouteKey, setActiveNavRouteKey] = useState(() => (initialNavItem ? navItemKey(initialNavItem) : ''));
  const [openNavGroupIds, setOpenNavGroupIds] = useState(() =>
    initialOpenNavGroupIds(
      navGroups,
      initialNavItem ? navItemKey(initialNavItem) : '',
      initialNavItem?.label ?? initialNavLabelForTab(navItems, pageModeTab)
    )
  );
  const [selectedControlId, setSelectedControlId] = useState(
    initialSelectedControlId ?? initialSelectedControlIdForRoute
  );
  const selectedControl = findSelectedControl(pageContent, selectedControlId);
  const selectedControlName = selectedControl?.name ?? formatRouteScope(selectedControlId);
  const baseSourceRows = useMemo(
    () => rowSourceForPageMode(pageContent, pageMode, parentPortalRows),
    [parentPortalRows, pageContent, pageMode]
  );
  const configuredRowSource = pageContent.modes[pageMode]?.rowSource ?? 'api';
  const sourceRows =
    activeTab === 'aiStatus' && configuredRowSource === 'aiBenchmarkRows'
      ? pageContent.aiBenchmarkRows
      : baseSourceRows;
  const rows = useMemo(
    () => toDisplayRows(sourceRows, pageMode, selectedControlName, controlId),
    [controlId, pageMode, selectedControlName, sourceRows]
  );
  const userDisplayRow = useMemo(
    () =>
      userEntry && pageMode !== 'parentGuide'
        ? toDisplayRows([userEntry], pageMode, selectedControlName, controlId)[0]
        : null,
    [controlId, pageMode, selectedControlName, userEntry]
  );
  const [selectedRowId, setSelectedRowId] = useState(userDisplayRow?.id ?? rows[0]?.id ?? '');
  const [, setPage] = useState(1);
  const rowsPerPage = 10;
  const [detailMode, setDetailMode] = useState<DetailMode | null>(null);
  const [assistantMode, setAssistantMode] = useState(assistantRouteActive);
  const [assistantActionsVisible, setAssistantActionsVisible] = useState(() => cfg.canvas.width >= 1000);
  const [selectedAssistantActionId, setSelectedAssistantActionId] = useState<AssistantQuickActionId | null>(null);
  const [, setSelectedAssistantChoice] = useState<AssistantQuickChoice | null>(null);
  const [assistantThreadSequence, setAssistantThreadSequence] = useState(0);
  const [assistantActionSequence, setAssistantActionSequence] = useState(0);
  const [assistantRouteTransition, setAssistantRouteTransition] = useState<'opening' | 'closing' | null>(null);

  useEffect(() => {
    const nextSelectedControlId =
      initialSelectedControlId ?? initialControlIdForPageMode(pageMode, pageContent, controlId);
    const nextNavItem = initialNavItemForContext(
      navItems,
      pageContent,
      initialNavLabel,
      nextSelectedControlId,
      pageModeTab,
      preferredNavGroupIdForPageMode(pageMode)
    );
    const nextTab = nextNavItem?.tabId ?? pageModeTab;
    const nextNavLabel = nextNavItem?.label ?? initialNavLabelForTab(navItems, nextTab);
    const nextNavRouteKey = nextNavItem ? navItemKey(nextNavItem) : '';
    setActiveTab(nextTab);
    setActiveNavLabel(nextNavLabel);
    setActiveNavRouteKey(nextNavRouteKey);
    setOpenNavGroupIds((current) => ensureOpenNavGroupIds(current, navGroups, nextNavRouteKey, nextNavLabel));
    setSelectedControlId(nextSelectedControlId);
    setAssistantMode(assistantRouteActive);
    setSelectedAssistantActionId(null);
    setSelectedAssistantChoice(null);
    setDetailMode(null);
    setPage(1);
    window.setTimeout(() => setAssistantRouteTransition(null), 180);
  }, [
    assistantRouteActive,
    controlId,
    initialNavLabel,
    initialSelectedControlId,
    navGroups,
    navItems,
    pageContent,
    pageMode,
    pageModeTab,
  ]);

  useEffect(() => {
    if (assistantMode && cfg.canvas.width < 1000) {
      setAssistantActionsVisible(false);
    }
  }, [assistantMode, cfg.canvas.width]);

  useEffect(() => {
    if (!assistantRouteTransition) return undefined;
    const timeout = window.setTimeout(() => setAssistantRouteTransition(null), 360);
    return () => window.clearTimeout(timeout);
  }, [assistantRouteTransition]);

  useEffect(() => {
    if (!surfaceMeasured) return undefined;
    if (initialRenderTimerRef.current !== undefined) {
      window.clearTimeout(initialRenderTimerRef.current);
    }
    initialRenderTimerRef.current = window.setTimeout(() => {
      initialRenderTimerRef.current = undefined;
      setInitialRenderSettled(true);
    }, PARENT_PORTAL_INITIAL_RENDER_SPINNER_MS);
    return () => {
      if (initialRenderTimerRef.current !== undefined) {
        window.clearTimeout(initialRenderTimerRef.current);
        initialRenderTimerRef.current = undefined;
      }
    };
  }, [surfaceMeasured]);

  useLayoutEffect(() => {
    const previousKey = previousRenderTransitionKeyRef.current;
    if (previousKey === renderTransitionKey) return;

    previousRenderTransitionKeyRef.current = renderTransitionKey;
    setRouteRenderPending(true);
    if (routeRenderTimerRef.current !== undefined) {
      window.clearTimeout(routeRenderTimerRef.current);
    }
    routeRenderTimerRef.current = window.setTimeout(() => {
      routeRenderTimerRef.current = undefined;
      setRouteRenderPending(false);
    }, PARENT_PORTAL_ROUTE_RENDER_SPINNER_MS);
  }, [renderTransitionKey]);

  useEffect(() => {
    if (initialLayoutReadyReportedRef.current || !initialRenderSettled || loading) {
      return undefined;
    }
    let frameA = 0;
    let frameB = 0;
    frameA = window.requestAnimationFrame(() => {
      frameB = window.requestAnimationFrame(() => {
        initialLayoutReadyReportedRef.current = true;
        onInitialLayoutReady?.();
      });
    });
    return () => {
      window.cancelAnimationFrame(frameA);
      window.cancelAnimationFrame(frameB);
    };
  }, [initialRenderSettled, loading, onInitialLayoutReady]);

  useEffect(() => {
    return () => {
      if (initialRenderTimerRef.current !== undefined) {
        window.clearTimeout(initialRenderTimerRef.current);
      }
      if (routeRenderTimerRef.current !== undefined) {
        window.clearTimeout(routeRenderTimerRef.current);
      }
    };
  }, []);

  useLayoutEffect(() => {
    const target = mainRef.current;
    if (!target) return undefined;
    const updateSurfaceSize = (width: number, height: number) => {
      const nextWidth = Math.round(width);
      const nextHeight = Math.round(height);
      if (!Number.isFinite(nextWidth) || !Number.isFinite(nextHeight) || nextWidth <= 0 || nextHeight <= 0) {
        return;
      }
      setSurfaceSize((current) =>
        current.width === nextWidth && current.height === nextHeight
          ? current
          : { width: nextWidth, height: nextHeight }
      );
      setSurfaceMeasured(true);
    };
    const initialRect = target.getBoundingClientRect();
    updateSurfaceSize(initialRect.width, initialRect.height);
    if (typeof ResizeObserver === 'undefined') return undefined;
    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;
      const { width, height } = entry.contentRect;
      updateSurfaceSize(width, height);
    });
    observer.observe(target);
    return () => observer.disconnect();
  }, []);

  const selectedRow =
    rows.find((row) => row.id === selectedRowId) ??
    (activeTab === 'aiStatus' ? null : userDisplayRow) ??
    rows[0] ??
    unavailableDisplayRow(selectedControlName);
  if (!selectedRow) return null;
  const leftX = cfg.layout.outerPad;
  const sideMainGap = Math.max(1, Math.round(cfg.layout.gap * 0.5));
  const mainX = leftX + cfg.layout.leftW + sideMainGap;
  const rightX = cfg.canvas.width - cfg.layout.outerPad - cfg.layout.rightW;
  const boardY = Math.max(0, cfg.layout.topY - 14);
  const assistantBoardY = boardY;
  const mainW = rightX - mainX;
  const assistantMainX = assistantMode && !assistantActionsVisible ? leftX : mainX;
  const assistantMainW = assistantMode && !assistantActionsVisible ? rightX - leftX : mainW;
  const mainH = cfg.canvas.height - boardY - 2;
  const assistantMainH = cfg.canvas.height - assistantBoardY - 2;
  const serviceErrorBannerVisible = Boolean(error) && !assistantMode;
  const serviceStatusBannerVisible = Boolean(statusMessage) && !assistantMode && !serviceErrorBannerVisible;
  const serviceFeedbackBannerH = serviceErrorBannerVisible || serviceStatusBannerVisible ? 68 : 0;
  const fixtureSourceBannerVisible = workspaceVisible && !assistantMode && configuredRowSource === 'aiBenchmarkRows';
  const fixtureSourceBannerH = fixtureSourceBannerVisible ? 58 : 0;
  const contentBoardY = boardY + serviceFeedbackBannerH + fixtureSourceBannerH;
  const contentMainH = Math.max(1, mainH - serviceFeedbackBannerH - fixtureSourceBannerH);
  const activeNavItem = activeNavRouteKey
    ? (navItems.find((item) => navItemKey(item) === activeNavRouteKey) ?? null)
    : (navItems.find((item) => item.label === activeNavLabel) ?? null);
  const activeNavGroupId = activeNavItem?.groupId ?? navGroupIdForNavKey(navGroups, activeNavRouteKey, activeNavLabel);
  const openAssistantRoute = () => {
    setAssistantRouteTransition('opening');
    setAssistantMode(true);
    setAssistantActionsVisible(cfg.canvas.width >= 1000);
    setDetailMode(null);
    onNavigate?.(assistantRoutePath);
  };
  const closeAssistantRoute = () => {
    setAssistantRouteTransition('closing');
    setSelectedAssistantActionId(null);
    setSelectedAssistantChoice(null);
    window.setTimeout(() => {
      setAssistantMode(false);
      onNavigate?.(assistantReturnRoutePath);
    }, 120);
  };
  const activateNavItem = (item: NavItem) => {
    const nextNavRouteKey = navItemKey(item);
    setActiveNavLabel(item.label);
    setActiveNavRouteKey(nextNavRouteKey);
    setOpenNavGroupIds((current) => ensureOpenNavGroupIds(current, navGroups, nextNavRouteKey, item.label));
  };
  const activateNavLabel = (navLabel: string) => {
    const item = navItems.find((entry) => entry.label === navLabel);
    if (item) {
      activateNavItem(item);
      return;
    }
    setActiveNavLabel(navLabel);
    setActiveNavRouteKey('');
    setOpenNavGroupIds((current) => ensureOpenNavGroupIds(current, navGroups, '', navLabel));
  };
  const changeTab = (tab: ParentPortalTabId) => {
    const nextNavLabel = initialNavLabelForTab(navItems, tab);
    setActiveTab(tab);
    activateNavLabel(nextNavLabel);
    setAssistantMode(false);
    setSelectedAssistantActionId(null);
    setSelectedAssistantChoice(null);
    setDetailMode(null);
    setPage(1);
  };
  const selectNavItem = (item: NavItem) => {
    if (isHashRoutePath(item.routePath) && onNavigate?.(item.routePath) === false) {
      return;
    }
    setActiveTab(item.tabId);
    activateNavItem(item);
    const routeControlId = routeControlIdForRoutePath(pageContent, item.routePath);
    if (routeControlId) setSelectedControlId(routeControlId);
    setAssistantMode(false);
    setSelectedAssistantActionId(null);
    setSelectedAssistantChoice(null);
    setDetailMode(null);
    setPage(1);
  };
  const openAssistantSetupRoute = () => {
    const startItem = navItems.find((item) => item.routePath === '#/start');
    if (startItem) {
      selectNavItem(startItem);
      return;
    }
    closeAssistantRoute();
  };
  const toggleNavGroup = (groupId: string) => {
    setOpenNavGroupIds((current) => toggleOpenNavGroupId(current, navGroups, groupId));
  };
  const selectRow = (rowId: string) => {
    setSelectedRowId(rowId);
    setDetailMode(null);
  };
  const selectControl = (controlIdValue: string) => {
    const control = findSelectedControl(pageContent, controlIdValue);
    if (isHashRoutePath(control?.routePath) && onNavigate?.(control.routePath) === false) {
      return;
    }
    setSelectedControlId(controlIdValue);
    const tab =
      control?.routePath === '#/ai-runtime' || assetKey(control?.category ?? '').includes('ai')
        ? 'aiStatus'
        : control?.routePath === '#/overview'
          ? 'overall'
          : 'controls';
    const nextNavItem = isHashRoutePath(control?.routePath)
      ? navItems.find((item) => item.routePath === control.routePath)
      : undefined;
    const fallbackNavLabel = initialNavLabelForTab(navItems, tab);
    const fallbackNavItem = navItems.find((item) => item.label === fallbackNavLabel);
    setActiveTab(nextNavItem?.tabId ?? tab);
    if (nextNavItem ?? fallbackNavItem) {
      activateNavItem((nextNavItem ?? fallbackNavItem) as NavItem);
    } else {
      activateNavLabel(fallbackNavLabel);
    }
    setAssistantMode(false);
    setSelectedAssistantActionId(null);
    setSelectedAssistantChoice(null);
    setDetailMode(null);
    setPage(1);
    if (pageMode === 'parentManage' && typeof control?.controlCode === 'number') {
      onRefreshParentPortal(control.controlCode);
    }
  };
  const transitionSpinnerLabel = loading
    ? pageContent.uiCopy.loadingTitle
    : !initialRenderSettled
      ? 'Preparing layout'
      : assistantRouteTransition === 'closing'
        ? 'Closing assistant'
        : assistantRouteTransition === 'opening'
          ? 'Opening assistant'
          : routeRenderPending
            ? 'Preparing panel'
            : null;
  return (
    <main ref={mainRef} className="parent-portal-svg-main" aria-busy={Boolean(transitionSpinnerLabel)}>
      {workspaceVisible ? (
        <h1 className="parent-portal-visually-hidden">{`${activeNavLabel} parent controls`}</h1>
      ) : null}
      {mobileLayout ? (
        <ParentPortalMobileNavigation
          activeNavRouteKey={activeNavRouteKey}
          assistantActive={assistantMode}
          assistantRoutePath={assistantRoutePath}
          navGroups={navGroups}
          onAssistantOpen={openAssistantRoute}
          onSelect={selectNavItem}
        />
      ) : null}
      <svg
        viewBox={`0 0 ${cfg.canvas.width} ${cfg.canvas.height}`}
        className="parent-portal-svg-surface"
        role="group"
        aria-label="Ocentra parent dashboard"
        preserveAspectRatio="xMidYMin meet"
      >
        <Defs />
        {!mobileLayout && (!assistantMode || assistantActionsVisible) ? (
          <NavPanel
            activeNavLabel={activeNavLabel}
            activeNavRouteKey={activeNavRouteKey}
            navGroups={navGroups}
            openGroupIds={openNavGroupIds}
            assistantOpen={assistantMode}
            assistantCommandAvailable={assistantCommandAvailable}
            selectedAssistantActionId={selectedAssistantActionId}
            onAssistantNewChat={() => {
              setSelectedAssistantActionId(null);
              setSelectedAssistantChoice(null);
              setAssistantThreadSequence((sequence) => sequence + 1);
              setAssistantMode(true);
              onAssistantCommand?.(AgentCommand.ParentAssistantThreadCreate, assistantThreadCreatePayload());
            }}
            onNavGroupToggle={toggleNavGroup}
            onNavItemSelect={selectNavItem}
            onAssistantActionSelect={(actionId) => {
              setSelectedAssistantActionId(actionId);
              setSelectedAssistantChoice(null);
              setAssistantActionSequence((sequence) => sequence + 1);
              setAssistantMode(true);
              const action = assistantQuickActionById(actionId);
              if (action) {
                onAssistantCommand?.(
                  AgentCommand.ParentAssistantQuickActionStart,
                  assistantQuickActionCommandPayload(action)
                );
              }
            }}
            onAssistantOpen={() => {
              openAssistantRoute();
            }}
            cfg={cfg}
          />
        ) : null}
        {workspaceVisible ? (
          assistantMode ? (
            <AssistantModeBoard
              x={assistantMainX}
              y={assistantBoardY}
              w={assistantMainW}
              h={assistantMainH}
              actionsVisible={assistantActionsVisible}
              selectedAction={assistantQuickActionById(selectedAssistantActionId)}
              selectedActionSequence={assistantActionSequence}
              threadSequence={assistantThreadSequence}
              commandAvailable={assistantCommandAvailable}
              response={assistantResponse}
              onChoiceSelect={setSelectedAssistantChoice}
              onAssistantMessage={(payload) => {
                onAssistantCommand?.(AgentCommand.ParentAssistantMessageSend, payload);
              }}
              onActionToggle={() => setAssistantActionsVisible((visible) => !visible)}
              onOpenSetup={openAssistantSetupRoute}
              onClose={() => {
                closeAssistantRoute();
              }}
              cfg={cfg}
            />
          ) : (
            <MainBoard
              activeNavLabel={activeNavLabel}
              {...(activeNavItem === null ? {} : { activeNavItem })}
              activeNavGroupId={activeNavGroupId}
              activeTab={activeTab}
              rows={rows}
              parentPortalRows={parentPortalRows}
              tabDetails={tabDetails}
              controlAreas={controlAreas}
              quickControls={quickControls}
              guideTopics={pageContent.guideTopics}
              season={pageContent.season}
              uiCopy={pageContent.uiCopy}
              selectedControlId={selectedControlId}
              selectedRowId={selectedRow.id}
              selectedRow={selectedRow}
              selectedControlName={selectedControlName}
              rowsPerPage={rowsPerPage}
              detailMode={detailMode}
              onTabChange={changeTab}
              onRowSelect={selectRow}
              onPageChange={setPage}
              onDetailClose={() => setDetailMode(null)}
              onControlSelect={selectControl}
              {...(onNavigate === undefined ? {} : { onNavigate })}
              {...(onAssistantCommand === undefined ? {} : { onAgentCommand: onAssistantCommand })}
              onReconnectLocalService={() => onRefreshParentPortal(controlCode)}
              onSelectNavLabel={(navLabel) => {
                const item = navItems.find((entry) => entry.label === navLabel);
                if (item) {
                  selectNavItem(item);
                  return;
                }
                activateNavLabel(navLabel);
              }}
              {...(activityState === undefined ? {} : { activityState })}
              sourceRowsUnavailable={configuredRowSource !== 'aiBenchmarkRows' && sourceRows.length === 0}
              lanPairingAutoScanSequence={lanPairingAutoScanSequence}
              cfg={cfg}
              mainX={mainX}
              mainW={mainW}
              mainY={contentBoardY}
              mainH={contentMainH}
            />
          )
        ) : null}
        {fixtureSourceBannerVisible ? (
          <g
            role="status"
            aria-label="Demo fixture, not runtime data, no product readiness claim"
            data-ocentra-parent-row-source="fixture"
          >
            <path
              d={cutRectPath(mainX + 4, boardY + serviceFeedbackBannerH + 4, mainW - 8, 48, 9)}
              fill="rgba(8, 17, 34, 0.92)"
              stroke={cfg.colors.gold}
              strokeWidth={1.2}
            />
            <text
              x={mainX + 24}
              y={boardY + serviceFeedbackBannerH + 24}
              fontSize={12.5}
              fontWeight={950}
              fill={cfg.colors.gold}
            >
              DEMO FIXTURE · NOT RUNTIME
            </text>
            <text
              x={mainX + 24}
              y={boardY + serviceFeedbackBannerH + 42}
              fontSize={10.5}
              fontWeight={760}
              fill={cfg.colors.mutedText}
            >
              NO PRODUCT READINESS CLAIM
            </text>
          </g>
        ) : null}
        {workspaceVisible && serviceStatusBannerVisible && statusMessage ? (
          <g aria-live="polite" role="status">
            <path
              d={cutRectPath(mainX + 4, boardY + 4, mainW - 8, 56, 9)}
              fill="rgba(3, 17, 30, 0.92)"
              stroke={cfg.colors.cyan}
              strokeWidth={1.2}
            />
            <text x={mainX + 24} y={boardY + 26} fontSize={13.5} fontWeight={950} fill={cfg.colors.bodyText}>
              STATUS UPDATE
            </text>
            <text x={mainX + 24} y={boardY + 45} fontSize={11} fontWeight={760} fill={cfg.colors.mutedText}>
              {truncateTextForWidth(statusMessage, mainW - 48, 11, 0.56)}
            </text>
          </g>
        ) : null}
        {workspaceVisible && serviceErrorBannerVisible && error ? (
          <g role="alert">
            <path
              d={cutRectPath(mainX + 4, boardY + 4, mainW - 8, 56, 9)}
              fill="rgba(3, 7, 18, 0.66)"
              stroke={cfg.colors.red}
              strokeWidth={1.2}
            />
            <text x={mainX + 24} y={boardY + 26} fontSize={13.5} fontWeight={950} fill={cfg.colors.bodyText}>
              {pageContent.uiCopy.errorTitle}
            </text>
            <text x={mainX + 24} y={boardY + 45} fontSize={11} fontWeight={760} fill={cfg.colors.mutedText}>
              {truncateTextForWidth(error, mainW - 48, 11, 0.56)}
            </text>
          </g>
        ) : null}
      </svg>
    </main>
  );
}
