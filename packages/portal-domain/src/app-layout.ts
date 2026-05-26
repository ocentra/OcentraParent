import { PortalFrameTuner } from './frame-tuner';

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
        '#/overview',
        'today-report'
      ),
      button(
        'browser-state',
        'Browser state',
        'Supported browsers, unmanaged risk, web evidence.',
        '#/browser',
        'browser-state',
        text.AppLayoutTone.Gold
      ),
      button('rules', 'Rules', 'House rules, allow, ask, explain, block.', '#/policy', 'rules', text.AppLayoutTone.Red),
      button(
        'ai-setup',
        'AI setup',
        'Local AI, API providers, model state.',
        '#/ai-runtime',
        'ai-setup',
        text.AppLayoutTone.Purple
      ),
      button(
        'drives',
        'Drives',
        'Connect parent-owned exports and custody.',
        '#/drive-connections',
        'drives',
        text.AppLayoutTone.Gold
      ),
      button(
        'support-api',
        'Support/API',
        'Diagnostics, route status, support bundles.',
        '#/diagnostics',
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
        '#/assistant',
        'question'
      ),
      button(
        'open-browser-rules',
        'Open browser rules',
        'Ask MIA to prepare browser rule changes.',
        '#/assistant',
        'browser-state',
        text.AppLayoutTone.Gold
      ),
      button(
        'change-rule',
        'Change a rule',
        'Ask MIA to guide a rule update.',
        '#/assistant',
        'rules',
        text.AppLayoutTone.Purple
      ),
      button(
        'support-bundle',
        'Open support bundle',
        'Ask MIA to prepare support context.',
        '#/assistant',
        'support-api',
        text.AppLayoutTone.Purple
      ),
    ]),
  ];
}

function defaultChatMainPanelBottom(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('assistant-composer', 'Composer', text.AppLayoutTone.Cyan, [
      button('message-mia', 'Message MIA', 'Typed parent question sent to AI.', '#/assistant', 'ai-guide'),
      button('send', 'Send', 'Submit the current parent prompt.', '#/assistant', 'send'),
    ]),
  ];
}

function defaultMainSidePanelFoldouts(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('quick-glance', 'Quick glance', text.AppLayoutTone.Cyan, [
      button('overview', 'Overview', 'Today control snapshot.', '#/overview', 'overview'),
    ]),
    foldout('guide', 'Guide', text.AppLayoutTone.Cyan, [
      button('start', 'Start here', 'Setup and controls map.', '#/start', 'start'),
      button('rules', 'Rules', 'Rules and policy guide.', '#/policy', 'rules', text.AppLayoutTone.Red),
      button('memory', 'Memory', 'Cited local memory.', '#/memory', 'memory'),
      button('ai', 'AI', 'Local AI and memory readiness.', '#/ai-runtime', 'ai-setup', text.AppLayoutTone.Purple),
      button('reports', 'Reports', 'Stored reports and summaries.', '#/report-settings', 'reports'),
      button('private', 'Private', 'Privacy and data custody guide.', '#/privacy-design', 'private'),
    ]),
    foldout('manage', 'Manage', text.AppLayoutTone.Cyan, [
      button(
        'browser',
        'Browser state',
        'Supported browsers and web evidence.',
        '#/browser-settings',
        'browser-state',
        text.AppLayoutTone.Gold
      ),
      button('devices', 'Devices', 'Child devices and pairing.', '#/devices', 'devices'),
      button('alerts', 'Alerts', 'Parent notification channels.', '#/notifications', 'alerts', text.AppLayoutTone.Red),
      button(
        'drives',
        'Drives',
        'Parent-owned exports and sync.',
        '#/drive-connections',
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
        '#/overview',
        'overview'
      ),
      button(
        'control-map',
        'What parents control',
        'Browser, app, rules, schedules, AI, reports.',
        '#/start',
        'start',
        text.AppLayoutTone.Gold
      ),
      button(
        'data-custody',
        'Data custody',
        'Local evidence and parent-owned export boundaries.',
        '#/privacy-design',
        'private',
        text.AppLayoutTone.Purple
      ),
    ]),
  ];
}

function defaultMainPanelBottomFoldouts(): readonly PortalAppLayoutFoldoutDraft[] {
  return [
    foldout('bottom-actions', 'Bottom actions', text.AppLayoutTone.Cyan, [
      button('open-assistant', 'AI assistant', 'Ask MIA instead of hunting settings.', '#/assistant', 'ai-guide'),
      button(
        'open-settings',
        'Settings',
        'Open family settings controls.',
        '#/settings-rules',
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
    routePath: '#/overview',
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
