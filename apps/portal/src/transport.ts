import { ParentAgentCommand, type ParentAgentCommandName } from '../generated/parent-ui-bridge';

const DirectEnforcementCommands = [
  ParentAgentCommand.EnforcementExecute,
  ParentAgentCommand.EnforcementTimerRecover,
  ParentAgentCommand.EnforcementTimerExpire,
  ParentAgentCommand.EnforcementOverrideCancel,
] as const satisfies readonly ParentAgentCommandName[];

export const DirectEnforcementCommandBoundaryErrorText =
  'Portal cannot dispatch enforcement mutation commands directly; use the enforcement authority boundary.';

export function isDirectEnforcementCommand(command: string | null | undefined): command is ParentAgentCommandName {
  return DirectEnforcementCommands.some((candidate) => candidate === command);
}
