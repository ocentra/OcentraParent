import { PARENT_ASSISTANT_QUICK_ACTIONS } from './parent-assistant-chat-quick-actions';

export const ParentAssistantPortalQuickActionId = {
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

export type ParentAssistantPortalQuickActionId =
  (typeof ParentAssistantPortalQuickActionId)[keyof typeof ParentAssistantPortalQuickActionId];

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

export const PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS = PARENT_ASSISTANT_QUICK_ACTIONS.filter(
  (action) => action.quickActionId !== ParentAssistantPortalQuickActionId.NewChat
);
export const PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION = PARENT_ASSISTANT_QUICK_ACTIONS.find(
  (action) => action.quickActionId === ParentAssistantPortalQuickActionId.NewChat
)!;
