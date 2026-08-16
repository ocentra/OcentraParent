const ParentAssistantPortalQuickActionIdMap = {
  NewChat: 'new-chat',
  Overview: 'overview',
  Start: 'start',
  Report: 'report',
  BrowserState: 'browser-state',
  Rules: 'rules',
  Memory: 'memory',
  AiSetup: 'ai-setup',
  Private: 'private',
  Devices: 'devices',
  Alerts: 'alerts',
  Drives: 'drives',
  SupportApi: 'support-api',
} as const;

type ParentAssistantPortalQuickActionId =
  (typeof ParentAssistantPortalQuickActionIdMap)[keyof typeof ParentAssistantPortalQuickActionIdMap];

type ParentAssistantPortalSourceScope =
  | 'child-local-evidence'
  | 'parent-owned-thread'
  | 'parent-owned-storage'
  | 'device-lan-state'
  | 'parent-notification-state'
  | 'privacy-custody'
  | 'local-ai-runtime'
  | 'api-provider-status';

type ParentAssistantPortalActionKind =
  | 'query-overview'
  | 'query-start'
  | 'query-report'
  | 'query-browser-state'
  | 'query-rule-context'
  | 'query-memory-context'
  | 'query-privacy-context'
  | 'query-device-state'
  | 'query-alert-context'
  | 'preview-rule-change'
  | 'provider-status'
  | 'prepare-support-message';

type ParentAssistantPortalChoice = {
  readonly choiceId: string;
  readonly label: string;
  readonly promptTemplateId: string;
  readonly resolvedPromptPreview: string;
  readonly assistantReply: string;
  readonly requiredSourceScopes: readonly ParentAssistantPortalSourceScope[];
  readonly nextActionKind: ParentAssistantPortalActionKind;
  readonly followUps: readonly string[];
};

type ParentAssistantPortalQuickAction = {
  readonly quickActionId: ParentAssistantPortalQuickActionId;
  readonly title: string;
  readonly description: string;
  readonly starterPromptTemplateId: string;
  readonly starterPrompt: string;
  readonly starterGuide: string;
  readonly scaffoldReply: string;
  readonly chips: readonly string[];
  readonly choices: readonly ParentAssistantPortalChoice[];
  readonly category: ParentAssistantPortalQuickActionId | 'freeform';
  readonly requiredSourceScopes: readonly ParentAssistantPortalSourceScope[];
  readonly requiresParentConfirmation: boolean;
  readonly allowedActionKinds: readonly ParentAssistantPortalActionKind[];
};

const ParentAssistantPortalSourceScopeByQuickActionId: Readonly<
  Record<ParentAssistantPortalQuickActionId, ParentAssistantPortalSourceScope>
> = {
  [ParentAssistantPortalQuickActionIdMap.NewChat]: 'parent-owned-thread',
  [ParentAssistantPortalQuickActionIdMap.Overview]: 'child-local-evidence',
  [ParentAssistantPortalQuickActionIdMap.Start]: 'child-local-evidence',
  [ParentAssistantPortalQuickActionIdMap.Report]: 'child-local-evidence',
  [ParentAssistantPortalQuickActionIdMap.BrowserState]: 'child-local-evidence',
  [ParentAssistantPortalQuickActionIdMap.Rules]: 'child-local-evidence',
  [ParentAssistantPortalQuickActionIdMap.Memory]: 'child-local-evidence',
  [ParentAssistantPortalQuickActionIdMap.AiSetup]: 'local-ai-runtime',
  [ParentAssistantPortalQuickActionIdMap.Private]: 'privacy-custody',
  [ParentAssistantPortalQuickActionIdMap.Devices]: 'device-lan-state',
  [ParentAssistantPortalQuickActionIdMap.Alerts]: 'parent-notification-state',
  [ParentAssistantPortalQuickActionIdMap.Drives]: 'parent-owned-storage',
  [ParentAssistantPortalQuickActionIdMap.SupportApi]: 'api-provider-status',
};

const ParentAssistantPortalQuickActionCategoryByQuickActionId: Readonly<
  Record<ParentAssistantPortalQuickActionId, ParentAssistantPortalQuickActionId | 'freeform'>
> = {
  [ParentAssistantPortalQuickActionIdMap.NewChat]: 'freeform',
  [ParentAssistantPortalQuickActionIdMap.Overview]: ParentAssistantPortalQuickActionIdMap.Overview,
  [ParentAssistantPortalQuickActionIdMap.Start]: ParentAssistantPortalQuickActionIdMap.Start,
  [ParentAssistantPortalQuickActionIdMap.Report]: ParentAssistantPortalQuickActionIdMap.Report,
  [ParentAssistantPortalQuickActionIdMap.BrowserState]: ParentAssistantPortalQuickActionIdMap.BrowserState,
  [ParentAssistantPortalQuickActionIdMap.Rules]: ParentAssistantPortalQuickActionIdMap.Rules,
  [ParentAssistantPortalQuickActionIdMap.Memory]: ParentAssistantPortalQuickActionIdMap.Memory,
  [ParentAssistantPortalQuickActionIdMap.AiSetup]: ParentAssistantPortalQuickActionIdMap.AiSetup,
  [ParentAssistantPortalQuickActionIdMap.Private]: ParentAssistantPortalQuickActionIdMap.Private,
  [ParentAssistantPortalQuickActionIdMap.Devices]: ParentAssistantPortalQuickActionIdMap.Devices,
  [ParentAssistantPortalQuickActionIdMap.Alerts]: ParentAssistantPortalQuickActionIdMap.Alerts,
  [ParentAssistantPortalQuickActionIdMap.Drives]: ParentAssistantPortalQuickActionIdMap.Drives,
  [ParentAssistantPortalQuickActionIdMap.SupportApi]: ParentAssistantPortalQuickActionIdMap.SupportApi,
};

function sourceScopeForQuickAction(
  quickActionId: ParentAssistantPortalQuickActionId
): ParentAssistantPortalSourceScope {
  return ParentAssistantPortalSourceScopeByQuickActionId[quickActionId]!;
}

function choice(
  quickActionId: ParentAssistantPortalQuickActionId,
  choiceId: string,
  label: string,
  prompt: string,
  reply: string,
  nextActionKind: ParentAssistantPortalActionKind,
  followUps: readonly string[]
): ParentAssistantPortalChoice {
  return {
    choiceId: `${quickActionId}-${choiceId}`,
    label,
    promptTemplateId: `prompt-${quickActionId}-${choiceId}-v1`,
    resolvedPromptPreview: prompt,
    assistantReply: reply,
    requiredSourceScopes: [sourceScopeForQuickAction(quickActionId)],
    nextActionKind,
    followUps,
  };
}

function quickAction(
  quickActionId: ParentAssistantPortalQuickActionId,
  title: string,
  description: string,
  prompt: string,
  guide: string,
  reply: string,
  actionKind: ParentAssistantPortalActionKind,
  chips: readonly string[],
  choices: readonly ParentAssistantPortalChoice[]
): ParentAssistantPortalQuickAction {
  return {
    quickActionId,
    title,
    description,
    starterPromptTemplateId: `prompt-${quickActionId}-starter-v1`,
    starterPrompt: prompt,
    starterGuide: guide,
    scaffoldReply: reply,
    chips,
    choices,
    category: ParentAssistantPortalQuickActionCategoryByQuickActionId[quickActionId],
    requiredSourceScopes: [sourceScopeForQuickAction(quickActionId)],
    requiresParentConfirmation: false,
    allowedActionKinds: [actionKind],
  };
}

export const PARENT_ASSISTANT_QUICK_ACTIONS: readonly ParentAssistantPortalQuickAction[] = [
  quickAction(
    ParentAssistantPortalQuickActionIdMap.NewChat,
    'New Chat',
    'Start a fresh parent assistant thread.',
    'Start a new MIA chat for a parent question.',
    'Ask MIA about activity, rules, reports, setup, or choose a quick action.',
    'I can start fresh. Pick a quick action or type what you want to know.',
    'query-report',
    ['What happened today?', 'Open report', 'Explain rules', 'Check setup'],
    []
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Overview,
    'Overview',
    'Current device state, area, runtime, browser, and activity snapshot.',
    'Give me the overall parent overview and tell me what matters first.',
    'Overview shortcuts. Pick status, attention, gaps, or next action.',
    'I will turn the dashboard snapshot into parent language: what is ready, what is offline, and what should happen next.',
    'query-overview',
    ['Current state', 'Needs attention', 'Setup gaps', 'What next?'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Overview,
        'status',
        'Status',
        'Summarize the current device state, runtime, browser, and activity status.',
        'I will summarize the live overview cards and separate ready state from missing or offline areas.',
        'query-overview',
        ['What is offline?', 'What is ready?', 'What changed?', 'Open overview']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Overview,
        'attention',
        'Attention',
        'Tell me which overview item needs attention first and why.',
        'I will rank attention items by parent impact, evidence, and whether action is possible now.',
        'query-overview',
        ['Show evidence', 'Explain risk', 'What can I fix?', 'Ignore for now']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Overview,
        'next',
        'Next step',
        'Tell me the next practical step from the current overview state.',
        'I will choose the next setup or review step based on what is missing, stale, or ready.',
        'query-overview',
        ['Pair device', 'Check browser', 'Review rules', 'Open setup']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Start,
    'Start',
    'Setup and controls map for first-run parent guidance.',
    'Walk me through what I should set up first and why.',
    'Start shortcuts. Pick setup map, child device, controls, or privacy.',
    'I will guide setup in parent order: device, browser, controls, reports, privacy, then AI.',
    'query-start',
    ['Setup map', 'Child device', 'Controls', 'Privacy basics'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Start,
        'map',
        'Setup map',
        'Give me a simple setup map for Ocentra Parent from empty state to useful monitoring.',
        'I will outline the setup sequence and call out what can be skipped until later.',
        'query-start',
        ['Pair a device', 'Set rules', 'Set reports', 'Explain privacy']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Start,
        'device',
        'Child device',
        'Help me start with child device connection and explain what should happen next.',
        'I will ask about the child device, pairing state, and what proof is needed before controls are meaningful.',
        'query-start',
        ['LAN pairing', 'Device not found', 'What is visible?', 'Open devices']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Start,
        'controls',
        'Controls',
        'Help me choose the first controls to set without overblocking.',
        'I will frame controls by parent goal: visibility, schedules, allowed apps, blocked sites, and review habits.',
        'query-start',
        ['Browser controls', 'App limits', 'Schedules', 'Reports']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Report,
    'Report',
    'Stored activity and summary evidence prompts.',
    "Give me today's parent report with activity, browser evidence, and anything that needs attention.",
    'Report shortcuts. Pick today, this week, or attention items, or type your own question.',
    "I will frame the report around real stored activity, browser evidence, rule events, and anything that still needs a parent's decision.",
    'query-report',
    ['What happened today?', 'What needs attention?', 'Show evidence', 'Open report'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Report,
        'today',
        'Today',
        "Give me today's parent report with activity, browser evidence, and anything that needs attention.",
        "I will prepare today's report view with activity highlights, browser evidence, open questions, and missing data called out clearly.",
        'query-report',
        ['What needs attention?', 'Show evidence', 'Explain a blocked moment', 'Open report settings']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Report,
        'week',
        'This week',
        'Give me this week summary for activity, browser patterns, rule hits, and anything I should review.',
        'I will summarize the week by activity patterns, rule hits, browser evidence, and parent follow-up areas.',
        'query-report',
        ['Compare to today', 'Show top app use', 'Show browser patterns', 'Prepare weekly report']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Report,
        'attention',
        'Attention',
        'Tell me what needs my attention first and why it matters.',
        'I will rank attention items by evidence, policy impact, and what action a parent can take next.',
        'query-report',
        ['Why is this important?', 'What can I change?', 'Show evidence', 'Ignore this for now']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.BrowserState,
    'Browser State',
    'Blocked, allowed, and unmanaged web prompts.',
    'Explain the current browser state, blocked moments, allowed moments, and unmanaged browser risks.',
    'Browser shortcuts. Pick blocked, allowed, or unmanaged state, or type your own question.',
    'I will explain browser state from available evidence, including what was blocked, allowed, unmanaged, or still missing.',
    'query-browser-state',
    ['Why was this allowed?', 'Why was this blocked?', 'Change browser rules', 'Show web evidence'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.BrowserState,
        'blocked',
        'Blocked',
        'Show me recent blocked browser moments and explain which rule caused each one.',
        'I will group blocked browser moments by rule, evidence, and likely parent action.',
        'query-browser-state',
        ['Explain one block', 'Change a browser rule', 'Show evidence', 'Allow this once']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.BrowserState,
        'allowed',
        'Allowed',
        'Show me recent allowed browser moments and explain why they were allowed.',
        'I will explain allowed browser moments by rule state, evidence source, and whether anything looks risky.',
        'query-browser-state',
        ['Why was this allowed?', 'Tighten the rule', 'Show web evidence', 'Check unmanaged browsers']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.BrowserState,
        'unmanaged',
        'Unmanaged',
        'Check whether any browser state is unmanaged or missing evidence.',
        'I will call out browser coverage gaps, unsupported state, and setup steps needed before enforcement can rely on it.',
        'query-browser-state',
        ['Fix setup', 'Show missing evidence', 'Open browser settings', 'Explain the risk']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Rules,
    'Rules',
    'House rule explain, change, allow, and block prompts.',
    'Explain the current house rules and tell me what I can change safely.',
    'Rules shortcuts. Pick explain, change, allow, or block, or type your own question.',
    'I can explain current rules and prepare parent-approved rule changes. Nothing applies until preview and confirmation exist.',
    'query-rule-context',
    ['Explain a rule', 'Change a rule', 'Allow something', 'Block something'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Rules,
        'explain',
        'Explain',
        'Explain the current house rules and why the recent activity matched or missed them.',
        'I will explain active rules, matching evidence, and what is not wired yet.',
        'query-rule-context',
        ['Change a rule', 'Why was this allowed?', 'Show evidence', 'Open rule settings']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Rules,
        'change',
        'Change',
        'Help me change a house rule. Ask what I want changed, show safe options, and explain the effect before applying anything.',
        'I will guide the rule change as choices first, then turn the selected option into a typed action preview.',
        'preview-rule-change',
        ['Make it stricter', 'Allow one app', 'Block a site', 'Set a schedule']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Rules,
        'allow',
        'Allow',
        'Help me allow something for my child and explain the safest scope: once, today, schedule, or permanent.',
        'I will ask what should be allowed and frame options by duration, scope, evidence, and risk.',
        'preview-rule-change',
        ['Allow once', 'Allow today', 'Allow on schedule', 'Explain the risk']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Rules,
        'block',
        'Block',
        'Help me block something for my child and explain what evidence or rule will support it.',
        'I will ask what should be blocked and prepare choices for site, app, category, or schedule.',
        'preview-rule-change',
        ['Block a site', 'Block an app', 'Set bedtime block', 'Show recent use']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Memory,
    'Memory',
    'Cited local memory, chat context, evidence scope, and privacy boundaries.',
    'Explain what MIA can remember or cite and what is not available yet.',
    'Memory shortcuts. Pick chat context, evidence, privacy, or reset.',
    'I will separate current chat context, local evidence, parent-owned exports, and future memory behavior.',
    'query-memory-context',
    ['Chat context', 'Evidence scope', 'Privacy boundary', 'Reset memory'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Memory,
        'context',
        'Chat context',
        'Explain what this assistant thread can use from the current chat.',
        'I will explain what is in-thread context versus what requires explicit evidence or tool access.',
        'query-memory-context',
        ['What can MIA see?', 'What is missing?', 'Use current page', 'Forget this']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Memory,
        'evidence',
        'Evidence',
        'Explain what local activity or browser evidence MIA can cite.',
        'I will distinguish reported evidence from unavailable evidence and avoid pretending missing data exists.',
        'query-memory-context',
        ['Show sources', 'Missing evidence', 'Report scope', 'Privacy risk']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Memory,
        'reset',
        'Reset',
        'Help me clear or start over with assistant context.',
        'I will frame reset choices without deleting any real evidence unless a separate confirmed action exists.',
        'query-memory-context',
        ['New chat', 'Clear draft', 'Keep evidence', 'Explain reset']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.AiSetup,
    'AI Setup',
    'Local AI, API provider, model, and memory prompts.',
    'Check AI setup and tell me what is ready, missing, or not connected yet.',
    'AI setup shortcuts. Pick readiness, providers, model, or memory.',
    'I will separate ready local pieces from missing provider, model, and memory wiring so the parent sees honest status.',
    'provider-status',
    ['Check AI setup', 'Check API providers', 'Check model state', 'Check memory'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.AiSetup,
        'readiness',
        'Readiness',
        'Check whether MIA is ready and list what is connected, missing, or not implemented yet.',
        'I will show readiness as connected, missing, or planned, without overstating AI capability.',
        'provider-status',
        ['What is missing?', 'Connect provider', 'Check local model', 'Open AI settings']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.AiSetup,
        'provider',
        'Provider',
        'Check API provider setup and explain what key, model, or permission is needed.',
        'I will organize provider setup by key state, model choice, route status, and parent safety boundary.',
        'provider-status',
        ['Add provider', 'Check model', 'Explain privacy', 'Use local only']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.AiSetup,
        'memory',
        'Memory',
        'Check what MIA can remember or reference and what memory is not wired yet.',
        'I will distinguish chat context, parent evidence, and future memory features so expectations stay clear.',
        'provider-status',
        ['What can MIA see?', 'Clear memory', 'Explain evidence', 'Open privacy']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Private,
    'Private',
    'Privacy, data custody, local-first evidence, and parent-owned exports.',
    'Explain privacy and data custody for the current parent setup.',
    'Private shortcuts. Pick local data, exports, remote access, or retention.',
    'I will explain what stays local, what parent-owned export means, and what needs parent confirmation.',
    'query-privacy-context',
    ['What stays local?', 'Exports', 'Remote access', 'Retention'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Private,
        'local',
        'Local data',
        'Explain what stays local and what does not leave this setup.',
        'I will describe local-first evidence boundaries and any feature that would require a separate connection.',
        'query-privacy-context',
        ['Show evidence types', 'Explain exports', 'Remote access risk', 'Open privacy']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Private,
        'exports',
        'Exports',
        'Explain what parent-owned exports could include and what should be excluded.',
        'I will frame export choices by report, evidence, privacy, and retention.',
        'query-privacy-context',
        ['Report export', 'Remove private data', 'Retention choice', 'Open drives']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Private,
        'remote',
        'Remote access',
        'Explain remote access boundaries and what would need confirmation.',
        'I will separate local control from remote access and call out what is not wired yet.',
        'query-privacy-context',
        ['Disable remote', 'Parent confirmation', 'Support access', 'Explain risk']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Devices,
    'Devices',
    'Child devices, LAN discovery, pairing, selected device, and capability state.',
    'Help me understand device pairing and what the current child device state means.',
    'Device shortcuts. Pick LAN scan, pairing, selected device, or capability.',
    'I will explain the LAN/device surface as the one device truth: discovery, pairing, selected device, and capabilities.',
    'query-device-state',
    ['LAN scan', 'Pair device', 'Selected device', 'Capability'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Devices,
        'lan',
        'LAN scan',
        'Explain LAN scan status and what should happen when devices are found.',
        'I will describe scanning, available devices, unsupported devices, and why a scan may be stuck.',
        'query-device-state',
        ['Why scanning?', 'Device not found', 'Pair next', 'Open devices']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Devices,
        'pair',
        'Pairing',
        'Help me pair a child device and explain each required step.',
        'I will ask for the pairing state, selected device, and parent confirmation before any control action.',
        'query-device-state',
        ['Pair selected', 'Set active', 'Open capability', 'Troubleshoot']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Devices,
        'capability',
        'Capability',
        'Explain device capability status and what controls are actually available.',
        'I will separate available capabilities from missing, unsupported, or not-reported state.',
        'query-device-state',
        ['What is ready?', 'What is missing?', 'Browser capability', 'Activity capability']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Alerts,
    'Alerts',
    'Parent notifications, channels, attention triggers, and quiet hours.',
    'Help me decide what alerts a parent should receive and when.',
    'Alert shortcuts. Pick attention, channels, quiet hours, or digest.',
    'I will turn alerts into parent choices: what deserves interruption, what belongs in a digest, and what channel to use.',
    'query-alert-context',
    ['Attention alerts', 'Channels', 'Quiet hours', 'Digest'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Alerts,
        'attention',
        'Attention',
        'Help me choose which events should notify me immediately.',
        'I will classify alerts by urgency, evidence confidence, and whether parent action is needed.',
        'query-alert-context',
        ['Show evidence', 'Set quiet hours', 'Use digest', 'Open notifications']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Alerts,
        'channels',
        'Channels',
        'Help me pick the right notification channel for parent alerts.',
        'I will compare channels by speed, reliability, and parent interruption level.',
        'query-alert-context',
        ['Phone push', 'Email digest', 'In app only', 'Multiple channels']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Alerts,
        'quiet-hours',
        'Quiet hours',
        'Help me set quiet hours without missing important alerts.',
        'I will separate urgent alerts from digest-only alerts and keep quiet-hours rules readable.',
        'query-alert-context',
        ['Change quiet hours', 'Show digest', 'Keep urgent only', 'Open settings']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.Drives,
    'Drives',
    'Parent-owned storage, exports, files, backups, and retention.',
    'Explain drives, exports, and parent-owned storage in this setup.',
    'Drives shortcuts. Pick exports, files, backups, or retention.',
    'I will explain storage in parent terms: what is stored, what can be exported, and what stays parent-owned.',
    'query-privacy-context',
    ['Exports', 'Files', 'Backups', 'Retention'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.Drives,
        'exports',
        'Exports',
        'Explain how parent-owned exports should work.',
        'I will keep export choices separate from live device control and evidence capture.',
        'query-privacy-context',
        ['Show export types', 'Open storage', 'Retention', 'Remove evidence']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Drives,
        'files',
        'Files',
        'Explain which files should be visible to a parent and which should stay local.',
        'I will sort files by custody, visibility, and whether they are evidence, reports, or settings.',
        'query-privacy-context',
        ['Show local files', 'Show exports', 'Backups', 'Open drives']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.Drives,
        'backups',
        'Backups',
        'Explain how backups fit with parent-owned storage.',
        'I will keep backups separate from evidence and explain where restore or retention changes matter.',
        'query-privacy-context',
        ['Restore data', 'Change retention', 'Show files', 'Open storage']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionIdMap.SupportApi,
    'Support API',
    'API provider status, key setup, permission, and support boundaries.',
    'Check support API status and tell me what is ready or missing.',
    'Support API shortcuts. Pick provider status, key setup, or support message.',
    'I will separate provider readiness from support flow so the parent can see what is real now.',
    'provider-status',
    ['Provider status', 'Key setup', 'Permission', 'Support message'],
    [
      choice(
        ParentAssistantPortalQuickActionIdMap.SupportApi,
        'status',
        'Status',
        'Check whether the provider is reachable and configured correctly.',
        'I will report provider readiness without inventing missing integration details.',
        'provider-status',
        ['Add a key', 'Check model', 'Show privacy', 'Open AI settings']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.SupportApi,
        'key',
        'Key setup',
        'Help me add or fix the API key setup.',
        'I will keep key setup separate from local runtime readiness and parent policy.',
        'provider-status',
        ['Rotate key', 'Check provider', 'Open settings', 'Use local only']
      ),
      choice(
        ParentAssistantPortalQuickActionIdMap.SupportApi,
        'support',
        'Support message',
        'Prepare a support message that explains what is missing.',
        'I will help write a support message without claiming unsupported capabilities.',
        'prepare-support-message',
        ['Copy details', 'Show error', 'Open support', 'Add logs']
      ),
    ]
  ),
] as const;
