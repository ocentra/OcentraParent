export const ParentAssistantPortalQuickActionId = {
  NewChat: 'new-chat',
  Report: 'report',
  BrowserState: 'browser-state',
  Rules: 'rules',
  AiSetup: 'ai-setup',
  Drives: 'drives',
  SupportApi: 'support-api',
} as const;

export type ParentAssistantPortalQuickActionId =
  (typeof ParentAssistantPortalQuickActionId)[keyof typeof ParentAssistantPortalQuickActionId];

type ParentAssistantPortalSourceScope =
  | 'child-local-evidence'
  | 'parent-owned-thread'
  | 'parent-owned-storage'
  | 'local-ai-runtime'
  | 'api-provider-status';

type ParentAssistantPortalActionKind =
  | 'query-report'
  | 'query-browser-state'
  | 'query-rule-context'
  | 'preview-rule-change'
  | 'provider-status'
  | 'prepare-support-message';

export type ParentAssistantPortalChoice = {
  readonly choiceId: string;
  readonly label: string;
  readonly promptTemplateId: string;
  readonly resolvedPromptPreview: string;
  readonly assistantReply: string;
  readonly requiredSourceScopes: readonly ParentAssistantPortalSourceScope[];
  readonly nextActionKind: ParentAssistantPortalActionKind;
  readonly followUps: readonly string[];
};

export type ParentAssistantPortalQuickAction = {
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

function sourceScopeForQuickAction(
  quickActionId: ParentAssistantPortalQuickActionId
): ParentAssistantPortalSourceScope {
  if (quickActionId === ParentAssistantPortalQuickActionId.AiSetup) return 'local-ai-runtime';
  if (quickActionId === ParentAssistantPortalQuickActionId.Drives) return 'parent-owned-storage';
  if (quickActionId === ParentAssistantPortalQuickActionId.SupportApi) return 'api-provider-status';
  if (quickActionId === ParentAssistantPortalQuickActionId.NewChat) return 'parent-owned-thread';
  return 'child-local-evidence';
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
    category: quickActionId === ParentAssistantPortalQuickActionId.NewChat ? 'freeform' : quickActionId,
    requiredSourceScopes: [sourceScopeForQuickAction(quickActionId)],
    requiresParentConfirmation: false,
    allowedActionKinds: [actionKind],
  };
}

const PARENT_ASSISTANT_QUICK_ACTIONS: readonly ParentAssistantPortalQuickAction[] = [
  quickAction(
    ParentAssistantPortalQuickActionId.NewChat,
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
    ParentAssistantPortalQuickActionId.Report,
    'Report',
    'Stored activity and summary evidence prompts.',
    "Give me today's parent report with activity, browser evidence, and anything that needs attention.",
    'Report shortcuts. Pick today, this week, or attention items, or type your own question.',
    "I will frame the report around real stored activity, browser evidence, rule events, and anything that still needs a parent's decision.",
    'query-report',
    ['What happened today?', 'What needs attention?', 'Show evidence', 'Open report'],
    [
      choice(
        ParentAssistantPortalQuickActionId.Report,
        'today',
        'Today',
        "Give me today's parent report with activity, browser evidence, and anything that needs attention.",
        "I will prepare today's report view with activity highlights, browser evidence, open questions, and missing data called out clearly.",
        'query-report',
        ['What needs attention?', 'Show evidence', 'Explain a blocked moment', 'Open report settings']
      ),
      choice(
        ParentAssistantPortalQuickActionId.Report,
        'week',
        'This week',
        'Give me this week summary for activity, browser patterns, rule hits, and anything I should review.',
        'I will summarize the week by activity patterns, rule hits, browser evidence, and parent follow-up areas.',
        'query-report',
        ['Compare to today', 'Show top app use', 'Show browser patterns', 'Prepare weekly report']
      ),
      choice(
        ParentAssistantPortalQuickActionId.Report,
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
    ParentAssistantPortalQuickActionId.BrowserState,
    'Browser State',
    'Blocked, allowed, and unmanaged web prompts.',
    'Explain the current browser state, blocked moments, allowed moments, and unmanaged browser risks.',
    'Browser shortcuts. Pick blocked, allowed, or unmanaged state, or type your own question.',
    'I will explain browser state from available evidence, including what was blocked, allowed, unmanaged, or still missing.',
    'query-browser-state',
    ['Why was this allowed?', 'Why was this blocked?', 'Change browser rules', 'Show web evidence'],
    [
      choice(
        ParentAssistantPortalQuickActionId.BrowserState,
        'blocked',
        'Blocked',
        'Show me recent blocked browser moments and explain which rule caused each one.',
        'I will group blocked browser moments by rule, evidence, and likely parent action.',
        'query-browser-state',
        ['Explain one block', 'Change a browser rule', 'Show evidence', 'Allow this once']
      ),
      choice(
        ParentAssistantPortalQuickActionId.BrowserState,
        'allowed',
        'Allowed',
        'Show me recent allowed browser moments and explain why they were allowed.',
        'I will explain allowed browser moments by rule state, evidence source, and whether anything looks risky.',
        'query-browser-state',
        ['Why was this allowed?', 'Tighten the rule', 'Show web evidence', 'Check unmanaged browsers']
      ),
      choice(
        ParentAssistantPortalQuickActionId.BrowserState,
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
    ParentAssistantPortalQuickActionId.Rules,
    'Rules',
    'House rule explain, change, allow, and block prompts.',
    'Explain the current house rules and tell me what I can change safely.',
    'Rules shortcuts. Pick explain, change, allow, or block, or type your own question.',
    'I can explain current rules and prepare parent-approved rule changes. Nothing applies until preview and confirmation exist.',
    'query-rule-context',
    ['Explain a rule', 'Change a rule', 'Allow something', 'Block something'],
    [
      choice(
        ParentAssistantPortalQuickActionId.Rules,
        'explain',
        'Explain',
        'Explain the current house rules and why the recent activity matched or missed them.',
        'I will explain active rules, matching evidence, and what is not wired yet.',
        'query-rule-context',
        ['Change a rule', 'Why was this allowed?', 'Show evidence', 'Open rule settings']
      ),
      choice(
        ParentAssistantPortalQuickActionId.Rules,
        'change',
        'Change',
        'Help me change a house rule. Ask what I want changed, show safe options, and explain the effect before applying anything.',
        'I will guide the rule change as choices first, then turn the selected option into a typed action preview.',
        'preview-rule-change',
        ['Make it stricter', 'Allow one app', 'Block a site', 'Set a schedule']
      ),
      choice(
        ParentAssistantPortalQuickActionId.Rules,
        'allow',
        'Allow',
        'Help me allow something for my child and explain the safest scope: once, today, schedule, or permanent.',
        'I will ask what should be allowed and frame options by duration, scope, evidence, and risk.',
        'preview-rule-change',
        ['Allow once', 'Allow today', 'Allow on schedule', 'Explain the risk']
      ),
      choice(
        ParentAssistantPortalQuickActionId.Rules,
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
    ParentAssistantPortalQuickActionId.AiSetup,
    'AI Setup',
    'Local AI, API provider, model, and memory prompts.',
    'Check AI setup and tell me what is ready, missing, or not connected yet.',
    'AI setup shortcuts. Pick readiness, providers, model, or memory.',
    'I will separate ready local pieces from missing provider, model, and memory wiring so the parent sees honest status.',
    'provider-status',
    ['Check AI setup', 'Check API providers', 'Check model state', 'Check memory'],
    [
      choice(
        ParentAssistantPortalQuickActionId.AiSetup,
        'readiness',
        'Readiness',
        'Check whether MIA is ready and list what is connected, missing, or not implemented yet.',
        'I will show readiness as connected, missing, or planned, without overstating AI capability.',
        'provider-status',
        ['What is missing?', 'Connect provider', 'Check local model', 'Open AI settings']
      ),
      choice(
        ParentAssistantPortalQuickActionId.AiSetup,
        'provider',
        'Provider',
        'Check API provider setup and explain what key, model, or permission is needed.',
        'I will organize provider setup by key state, model choice, route status, and parent safety boundary.',
        'provider-status',
        ['Add provider', 'Check model', 'Explain privacy', 'Use local only']
      ),
      choice(
        ParentAssistantPortalQuickActionId.AiSetup,
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
    ParentAssistantPortalQuickActionId.Drives,
    'Drives',
    'Parent-owned export, custody, and retention prompts.',
    'Help me manage parent-owned exports, custody notes, and retention settings.',
    'Drive shortcuts. Pick connect, export, custody, or retention.',
    'I will frame drive work around parent-owned evidence, export custody, and retention, without claiming a connection that is not wired.',
    'query-report',
    ['Connect a drive', 'Review exports', 'Custody notes', 'Retention settings'],
    [
      choice(
        ParentAssistantPortalQuickActionId.Drives,
        'connect',
        'Connect',
        'Help me connect a parent-owned drive for exports and explain what data would be stored there.',
        'I will outline the drive connection flow, ownership boundary, and data categories before anything connects.',
        'query-report',
        ['What gets exported?', 'Explain custody', 'Set retention', 'Open drive settings']
      ),
      choice(
        ParentAssistantPortalQuickActionId.Drives,
        'exports',
        'Exports',
        'Show me what parent export or report data would be prepared from the available evidence.',
        'I will describe export-ready report data, missing evidence, and what should stay private.',
        'query-report',
        ['Prepare report', 'Show evidence', 'Remove private data', 'Explain format']
      ),
      choice(
        ParentAssistantPortalQuickActionId.Drives,
        'retention',
        'Retention',
        'Help me choose retention settings for activity reports and evidence exports.',
        'I will compare retention choices by privacy, custody, and parent review needs.',
        'query-report',
        ['Keep 30 days', 'Keep weekly reports', 'Delete old evidence', 'Explain privacy']
      ),
    ]
  ),
  quickAction(
    ParentAssistantPortalQuickActionId.SupportApi,
    'Support Message',
    'Draft a parent-authored support request.',
    'Draft a support message with the issue, account context, and reply email.',
    'Support shortcuts. Pick message, account, billing, or setup.',
    'I will help draft a support message from parent-entered details only. Attachments are not collected from this screen.',
    'prepare-support-message',
    ['Draft support message', 'Account help', 'Billing help', 'Setup help'],
    [
      choice(
        ParentAssistantPortalQuickActionId.SupportApi,
        'message',
        'Message',
        'Help me write a concise support message with the issue and what I already tried.',
        'I will draft a message that the parent can review before sending.',
        'prepare-support-message',
        ['Shorten it', 'Add setup details', 'Add billing context', 'Open support form']
      ),
      choice(
        ParentAssistantPortalQuickActionId.SupportApi,
        'account',
        'Account',
        'Help me describe an account or sign-in issue for support.',
        'I will structure the account issue, expected reply path, and urgency.',
        'prepare-support-message',
        ['Add reply email', 'Explain login state', 'Open account page', 'Save draft']
      ),
      choice(
        ParentAssistantPortalQuickActionId.SupportApi,
        'billing',
        'Billing',
        'Help me write a billing or plan question for support.',
        'I will draft the plan question with parent-entered subscription context.',
        'prepare-support-message',
        ['Mention trial', 'Ask about seats', 'Open plan page', 'Save draft']
      ),
    ]
  ),
];

export const PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS: readonly ParentAssistantPortalQuickAction[] =
  PARENT_ASSISTANT_QUICK_ACTIONS.filter(
    (action) => action.quickActionId !== ParentAssistantPortalQuickActionId.NewChat
  );

export const PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION = PARENT_ASSISTANT_QUICK_ACTIONS.find(
  (action) => action.quickActionId === ParentAssistantPortalQuickActionId.NewChat
);
