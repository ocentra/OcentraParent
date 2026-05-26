import {
  PARENT_ASSISTANT_QUICK_ACTIONS,
  ParentAssistantQuickActionId,
  type ParentAssistantQuickAction,
  type ParentAssistantQuickActionId as ParentAssistantQuickActionIdValue,
} from '@ocentra-parent/parent-domain/parent-assistant';

export type ParentAssistantPortalQuickAction = ParentAssistantQuickAction;
export type ParentAssistantPortalQuickActionId = ParentAssistantQuickActionIdValue;

export const PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS: readonly ParentAssistantPortalQuickAction[] =
  PARENT_ASSISTANT_QUICK_ACTIONS.filter((action) => action.quickActionId !== ParentAssistantQuickActionId.NewChat);

export const PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION = PARENT_ASSISTANT_QUICK_ACTIONS.find(
  (action) => action.quickActionId === ParentAssistantQuickActionId.NewChat
);
