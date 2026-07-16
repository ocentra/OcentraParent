import { PortalRoute } from './portal-contract-adapter';
import { PortalFrameTuner } from './frame-tuner';
import { portalRouteHashPath } from './routes';

export type PortalAppLayoutSurfaceKey = (typeof PortalFrameTuner.AppSurface)[keyof typeof PortalFrameTuner.AppSurface];

export type PortalAppLayoutContentAreaKey =
  (typeof PortalFrameTuner.AppContentArea)[keyof typeof PortalFrameTuner.AppContentArea];

export type PortalAppLayoutTone = (typeof PortalFrameTuner.AppLayoutTone)[keyof typeof PortalFrameTuner.AppLayoutTone];

export type PortalAppLayoutButtonDraft = {
  readonly id: string;
  readonly label: string;
  readonly detail: string;
  readonly routePath: string;
  readonly icon: string;
  readonly tone: PortalAppLayoutTone;
};

export type PortalAppLayoutFoldoutDraft = {
  readonly id: string;
  readonly label: string;
  readonly tone: PortalAppLayoutTone;
  readonly buttons: readonly PortalAppLayoutButtonDraft[];
};

export type PortalAppLayoutSurfaceContentDraft = {
  readonly sidePanelFoldouts: readonly PortalAppLayoutFoldoutDraft[];
  readonly mainPanelTop: readonly PortalAppLayoutFoldoutDraft[];
  readonly mainPanelBottom: readonly PortalAppLayoutFoldoutDraft[];
};

export type PortalAppLayoutContentDraft = Record<PortalAppLayoutSurfaceKey, PortalAppLayoutSurfaceContentDraft>;

const text = PortalFrameTuner;

export function defaultPortalAppLayoutContentDraft(): PortalAppLayoutContentDraft {
  return {
    mainApp: defaultPortalAppLayoutSurfaceContent(text.AppSurface.MainApp),
    chatInterface: defaultPortalAppLayoutSurfaceContent(text.AppSurface.ChatInterface),
  };
}

export function defaultPortalAppLayoutSurfaceContent(
  surface: PortalAppLayoutSurfaceKey
): PortalAppLayoutSurfaceContentDraft {
  if (surface === text.AppSurface.ChatInterface) return defaultChatInterfaceSurfaceContent();
  return defaultMainAppSurfaceContent();
}

function defaultChatInterfaceSurfaceContent(): PortalAppLayoutSurfaceContentDraft {
  return {
    sidePanelFoldouts: defaultChatSidePanelFoldouts(),
    mainPanelTop: defaultChatMainPanelTop(),
    mainPanelBottom: defaultChatMainPanelBottom(),
  };
}

function defaultMainAppSurfaceContent(): PortalAppLayoutSurfaceContentDraft {
  return {
    sidePanelFoldouts: defaultMainSidePanelFoldouts(),
    mainPanelTop: defaultMainPanelTopFoldouts(),
    mainPanelBottom: defaultMainPanelBottomFoldouts(),
  };
}

function defaultChatSidePanelFoldouts(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('assistant-quick-actions', 'Quick action', text.AppLayoutTone.Cyan, [
      button(
        'today-report',
        'Today report',
        'Open stored activity and summary evidence.',
        portalRouteHashPath(PortalRoute.Overview),
        'today-report'
      ),
      button(
        'browser-state',
        'Browser state',
        'Supported browsers, unmanaged risk, web evidence.',
        portalRouteHashPath(PortalRoute.Browser),
        'browser-state',
        text.AppLayoutTone.Gold
      ),
      button(
        'rules',
        'Rules',
        'House rules, allow, ask, explain, block.',
        portalRouteHashPath(PortalRoute.Policy),
        'rules',
        text.AppLayoutTone.Red
      ),
      button(
        'ai-setup',
        'AI setup',
        'Local AI, API providers, model state.',
        portalRouteHashPath(PortalRoute.AiRuntime),
        'ai-setup',
        text.AppLayoutTone.Purple
      ),
      button(
        'drives',
        'Drives',
        'Connect parent-owned exports and custody.',
        portalRouteHashPath(PortalRoute.DriveConnections),
        'drives',
        text.AppLayoutTone.Gold
      ),
      button(
        'support-api',
        'Support',
        'Send a parent-authored support message.',
        portalRouteHashPath(PortalRoute.Diagnostics),
        'support-api'
      ),
    ]),
  ];
}

function defaultChatMainPanelTop(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('assistant-prompts', 'Quick prompts', text.AppLayoutTone.Cyan, [
      button(
        'what-happened-today',
        'What happened today?',
        'Ask MIA for a daily activity summary.',
        portalRouteHashPath(PortalRoute.Assistant),
        'question'
      ),
      button(
        'open-browser-rules',
        'Open browser rules',
        'Ask MIA to prepare browser rule changes.',
        portalRouteHashPath(PortalRoute.Assistant),
        'browser-state',
        text.AppLayoutTone.Gold
      ),
      button(
        'change-rule',
        'Change a rule',
        'Ask MIA to guide a rule update.',
        portalRouteHashPath(PortalRoute.Assistant),
        'rules',
        text.AppLayoutTone.Purple
      ),
      button(
        'support-bundle',
        'Support message',
        'Ask MIA to draft a support message.',
        portalRouteHashPath(PortalRoute.Assistant),
        'support-api',
        text.AppLayoutTone.Purple
      ),
    ]),
  ];
}

function defaultChatMainPanelBottom(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('assistant-composer', 'Composer', text.AppLayoutTone.Cyan, [
      button(
        'message-mia',
        'Message MIA',
        'Typed parent question sent to AI.',
        portalRouteHashPath(PortalRoute.Assistant),
        'ai-guide'
      ),
      button('send', 'Send', 'Submit the current parent prompt.', portalRouteHashPath(PortalRoute.Assistant), 'send'),
    ]),
  ];
}

function defaultMainSidePanelFoldouts(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('quick-glance', 'Quick glance', text.AppLayoutTone.Cyan, [
      button('overview', 'Overview', 'Today control snapshot.', portalRouteHashPath(PortalRoute.Overview), 'overview'),
    ]),
    foldout('guide', 'Guide', text.AppLayoutTone.Cyan, [
      button('start', 'Start here', 'Setup and controls map.', portalRouteHashPath(PortalRoute.Start), 'start'),
      button(
        'rules',
        'Rules',
        'Rules and policy guide.',
        portalRouteHashPath(PortalRoute.Policy),
        'rules',
        text.AppLayoutTone.Red
      ),
      button('memory', 'Memory', 'Cited local memory.', portalRouteHashPath(PortalRoute.Memory), 'memory'),
      button(
        'ai',
        'AI',
        'Local AI and memory readiness.',
        portalRouteHashPath(PortalRoute.AiRuntime),
        'ai-setup',
        text.AppLayoutTone.Purple
      ),
      button(
        'reports',
        'Reports',
        'Stored reports and summaries.',
        portalRouteHashPath(PortalRoute.Activity),
        'reports'
      ),
      button(
        'private',
        'Private',
        'Privacy and data custody guide.',
        portalRouteHashPath(PortalRoute.PrivacyDesign),
        'private'
      ),
    ]),
    foldout('manage', 'Manage', text.AppLayoutTone.Cyan, [
      button(
        'browser',
        'Browser state',
        'Supported browsers and web evidence.',
        portalRouteHashPath(PortalRoute.BrowserSettings),
        'browser-state',
        text.AppLayoutTone.Gold
      ),
      button('devices', 'Devices', 'Child devices and pairing.', portalRouteHashPath(PortalRoute.Devices), 'devices'),
      button(
        'alerts',
        'Alerts',
        'Parent notification channels.',
        portalRouteHashPath(PortalRoute.Notifications),
        'alerts',
        text.AppLayoutTone.Red
      ),
      button(
        'drives',
        'Drives',
        'Parent-owned exports and sync.',
        portalRouteHashPath(PortalRoute.DriveConnections),
        'drives',
        text.AppLayoutTone.Gold
      ),
    ]),
  ];
}

function defaultMainPanelTopFoldouts(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('top-cards', 'Top cards', text.AppLayoutTone.Cyan, [
      button(
        'snapshot',
        'Today control snapshot',
        'Family state, active rules, and evidence.',
        portalRouteHashPath(PortalRoute.Overview),
        'overview'
      ),
      button(
        'control-map',
        'What parents control',
        'Browser, app, rules, schedules, AI, reports.',
        portalRouteHashPath(PortalRoute.Start),
        'start',
        text.AppLayoutTone.Gold
      ),
      button(
        'data-custody',
        'Data custody',
        'Local evidence and parent-owned export boundaries.',
        portalRouteHashPath(PortalRoute.PrivacyDesign),
        'private',
        text.AppLayoutTone.Purple
      ),
    ]),
  ];
}

function defaultMainPanelBottomFoldouts(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('bottom-actions', 'Bottom actions', text.AppLayoutTone.Cyan, [
      button(
        'open-assistant',
        'AI assistant',
        'Ask MIA instead of hunting settings.',
        portalRouteHashPath(PortalRoute.Assistant),
        'ai-guide'
      ),
      button(
        'open-settings',
        'Settings',
        'Open family settings controls.',
        portalRouteHashPath(PortalRoute.SettingsRules),
        'settings',
        text.AppLayoutTone.Gold
      ),
    ]),
  ];
}

export function createPortalAppLayoutFoldoutDraft(
  area: PortalAppLayoutContentAreaKey,
  index: number
): PortalAppLayoutFoldoutDraft {
  return {
    id: `${area}-${index + 1}`,
    label: `New ${area}`,
    tone: text.AppLayoutTone.Cyan,
    buttons: [],
  };
}

export function createPortalAppLayoutButtonDraft(
  foldoutId: string,
  index: number,
  tone: PortalAppLayoutTone
): PortalAppLayoutButtonDraft {
  return {
    id: `${foldoutId}-button-${index + 1}`,
    label: `Button ${index + 1}`,
    detail: '',
    routePath: portalRouteHashPath(PortalRoute.Overview),
    icon: 'overview',
    tone,
  };
}

export function normalizePortalAppLayoutContentDraft(value: unknown): PortalAppLayoutContentDraft {
  const fallback = defaultPortalAppLayoutContentDraft();
  return {
    mainApp: normalizePortalAppLayoutSurfaceContentDraft(valueAt(value, [text.AppSurface.MainApp]), fallback.mainApp),
    chatInterface: normalizePortalAppLayoutSurfaceContentDraft(
      valueAt(value, [text.AppSurface.ChatInterface]),
      fallback.chatInterface
    ),
  };
}

export function normalizePortalAppLayoutSurfaceContentDraft(
  value: unknown,
  fallback: PortalAppLayoutSurfaceContentDraft
): PortalAppLayoutSurfaceContentDraft {
  return {
    sidePanelFoldouts: normalizeFoldouts(
      valueAt(value, [text.LayoutKey.SidePanelFoldouts]),
      fallback.sidePanelFoldouts
    ),
    mainPanelTop: normalizeFoldouts(valueAt(value, [text.LayoutKey.MainPanelTop]), fallback.mainPanelTop),
    mainPanelBottom: normalizeFoldouts(valueAt(value, [text.LayoutKey.MainPanelBottom]), fallback.mainPanelBottom),
  };
}

function normalizeFoldouts(
  value: unknown,
  fallback: readonly PortalAppLayoutFoldoutDraft[]
): readonly PortalAppLayoutFoldoutDraft[] {
  const foldouts = Array.isArray(value) ? value : fallback;
  return foldouts.map((foldoutValue, index) => normalizeFoldout(foldoutValue, fallback[index] ?? emptyFoldout(index)));
}

function normalizeFoldout(value: unknown, fallback: PortalAppLayoutFoldoutDraft): PortalAppLayoutFoldoutDraft {
  return {
    id: stringAt(value, [text.LayoutKey.Id], fallback.id),
    label: stringAt(value, [text.LayoutKey.Label], fallback.label),
    tone: toneAt(value, [text.LayoutKey.Tone], fallback.tone),
    buttons: normalizeButtons(valueAt(value, [text.LayoutKey.Buttons]), fallback.buttons),
  };
}

function normalizeButtons(
  value: unknown,
  fallback: readonly PortalAppLayoutButtonDraft[]
): readonly PortalAppLayoutButtonDraft[] {
  const buttons = Array.isArray(value) ? value : fallback;
  return buttons.map((buttonValue, index) => normalizeButton(buttonValue, fallback[index] ?? emptyButton(index)));
}

function normalizeButton(value: unknown, fallback: PortalAppLayoutButtonDraft): PortalAppLayoutButtonDraft {
  return {
    id: stringAt(value, [text.LayoutKey.Id], fallback.id),
    label: stringAt(value, [text.LayoutKey.Label], fallback.label),
    detail: stringAt(value, [text.LayoutKey.Detail], fallback.detail),
    routePath: stringAt(value, [text.LayoutKey.RoutePath], fallback.routePath),
    icon: stringAt(value, [text.LayoutKey.Icon], fallback.icon),
    tone: toneAt(value, [text.LayoutKey.Tone], fallback.tone),
  };
}

function foldout(
  id: string,
  label: string,
  tone: PortalAppLayoutTone,
  buttons: readonly PortalAppLayoutButtonDraft[]
): PortalAppLayoutFoldoutDraft {
  return { id, label, tone, buttons };
}

function button(
  id: string,
  label: string,
  detail: string,
  routePath: string,
  icon: string,
  tone: PortalAppLayoutTone = text.AppLayoutTone.Cyan
): PortalAppLayoutButtonDraft {
  return { id, label, detail, routePath, icon, tone };
}

function emptyFoldout(index: number): PortalAppLayoutFoldoutDraft {
  return createPortalAppLayoutFoldoutDraft(text.AppContentArea.SidePanelFoldouts, index);
}

function emptyButton(index: number): PortalAppLayoutButtonDraft {
  return createPortalAppLayoutButtonDraft(text.AppContentArea.SidePanelFoldouts, index, text.AppLayoutTone.Cyan);
}

function valueAt(root: unknown, path: readonly PropertyKey[]): unknown {
  let current = root;
  for (const key of path) {
    if (!isRecord(current)) {
      return undefined;
    }
    current = current[key];
  }
  return current;
}

function isRecord(value: unknown): value is Record<PropertyKey, unknown> {
  return value !== null && Object(value) === value;
}

function stringAt(root: unknown, path: readonly PropertyKey[], fallback: string): string {
  const value = valueAt(root, path);
  return typeof value === 'string' ? value : fallback;
}

function toneAt(root: unknown, path: readonly PropertyKey[], fallback: PortalAppLayoutTone): PortalAppLayoutTone {
  const value = valueAt(root, path);
  return Object.values(text.AppLayoutTone).includes(value as PortalAppLayoutTone)
    ? (value as PortalAppLayoutTone)
    : fallback;
}
