import { ParentUiActionKind, type ParentUiAction, type ParentUiActionResult } from '../../generated/parent-ui-bridge';
import { DirectEnforcementCommandBoundaryErrorText, isDirectEnforcementCommand } from '../transport';

export function dispatchPortalAction(
  action: ParentUiAction,
  dispatch: () => Promise<ParentUiActionResult>
): Promise<ParentUiActionResult> {
  if (action.action === ParentUiActionKind.AgentCommandRequested && isDirectEnforcementCommand(action.command)) {
    return Promise.reject(new Error(DirectEnforcementCommandBoundaryErrorText));
  }
  return dispatch();
}
