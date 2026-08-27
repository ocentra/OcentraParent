import { ParentAgentCommand, type ParentAgentCommandName } from '../generated/parent-ui-bridge';
import { PORTAL_HOST_BRIDGE_RUNTIME } from '@ocentra-parent/portal-domain/parent-portal-service-state';

const DirectEnforcementCommands = [
  ParentAgentCommand.EnforcementExecute,
  ParentAgentCommand.EnforcementTimerRecover,
  ParentAgentCommand.EnforcementTimerExpire,
  ParentAgentCommand.EnforcementOverrideCancel,
] as const satisfies readonly ParentAgentCommandName[];

export const DirectEnforcementCommandBoundaryErrorText =
  PORTAL_HOST_BRIDGE_RUNTIME.DirectEnforcementCommandBoundaryErrorText;

export function isDirectEnforcementCommand(command: unknown): command is ParentAgentCommandName {
  return DirectEnforcementCommands.some((candidate) => candidate === command);
}
