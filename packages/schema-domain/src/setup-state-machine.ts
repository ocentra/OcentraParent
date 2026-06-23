import { type Infer, Schema, withParser } from './effect';
import {
  deriveSetupReadinessOverallState,
  setupReadinessNeedsManualRecovery,
  SetupReadinessOverallState,
  SetupReadinessOverallStateSchema,
  SetupReadinessReportSchema,
} from './setup-readiness';

export const SetupFirstRunStateIdLiteral = {
  Welcome: 'welcome',
  AccountEntry: 'account-entry',
  HouseholdSelection: 'household-selection',
  ParentInstall: 'parent-install',
  ParentBootstrapAgreement: 'parent-bootstrap-agreement',
  ParentBootstrapCodeEntry: 'parent-bootstrap-code-entry',
  ParentInstallProgress: 'parent-install-progress',
  ParentGuidedSetupStart: 'parent-guided-setup-start',
  ChildProfile: 'child-profile',
  ChildPairing: 'child-pairing',
  ChildInstallInstructions: 'child-install-instructions',
  WaitingForChildDevice: 'waiting-for-child-device',
  ConfirmChildTrust: 'confirm-child-trust',
  PermissionReadiness: 'permission-readiness',
  PolicyBaseline: 'policy-baseline',
  DataCustody: 'data-custody',
  SetupDegraded: 'setup-degraded',
  ManualRequired: 'manual-required',
  SetupBlocked: 'setup-blocked',
  SetupComplete: 'setup-complete',
} as const;

export const SetupFirstRunScreenIdLiteral = {
  Welcome: 'welcome-screen',
  AccountEntry: 'sign-in-or-create-account-screen',
  HouseholdSelection: 'create-or-join-household-screen',
  ParentInstall: 'parent-install-screen',
  ParentBootstrapAgreement: 'parent-bootstrap-agreement-screen',
  ParentBootstrapCodeEntry: 'parent-bootstrap-code-screen',
  ParentInstallProgress: 'parent-install-progress-screen',
  ParentGuidedSetupStart: 'parent-guided-setup-start-screen',
  ChildProfile: 'child-profile-screen',
  ChildPairing: 'child-pairing-screen',
  ChildInstallInstructions: 'child-install-screen',
  WaitingForChildDevice: 'waiting-for-child-device-screen',
  ConfirmChildTrust: 'confirm-child-device-screen',
  PermissionReadiness: 'permission-checklist-screen',
  PolicyBaseline: 'policy-baseline-screen',
  DataCustody: 'data-custody-status-screen',
  Recovery: 'recovery-screen',
  ManualRequired: 'manual-required-screen',
  SetupBlocked: 'setup-blocked-screen',
  SetupComplete: 'setup-complete-screen',
} as const;

export const SetupFirstRunStateIdSchema = withParser(
  Schema.Literal(
    SetupFirstRunStateIdLiteral.Welcome,
    SetupFirstRunStateIdLiteral.AccountEntry,
    SetupFirstRunStateIdLiteral.HouseholdSelection,
    SetupFirstRunStateIdLiteral.ParentInstall,
    SetupFirstRunStateIdLiteral.ParentBootstrapAgreement,
    SetupFirstRunStateIdLiteral.ParentBootstrapCodeEntry,
    SetupFirstRunStateIdLiteral.ParentInstallProgress,
    SetupFirstRunStateIdLiteral.ParentGuidedSetupStart,
    SetupFirstRunStateIdLiteral.ChildProfile,
    SetupFirstRunStateIdLiteral.ChildPairing,
    SetupFirstRunStateIdLiteral.ChildInstallInstructions,
    SetupFirstRunStateIdLiteral.WaitingForChildDevice,
    SetupFirstRunStateIdLiteral.ConfirmChildTrust,
    SetupFirstRunStateIdLiteral.PermissionReadiness,
    SetupFirstRunStateIdLiteral.PolicyBaseline,
    SetupFirstRunStateIdLiteral.DataCustody,
    SetupFirstRunStateIdLiteral.SetupDegraded,
    SetupFirstRunStateIdLiteral.ManualRequired,
    SetupFirstRunStateIdLiteral.SetupBlocked,
    SetupFirstRunStateIdLiteral.SetupComplete
  )
);

export const SetupFirstRunScreenIdSchema = withParser(
  Schema.Literal(
    SetupFirstRunScreenIdLiteral.Welcome,
    SetupFirstRunScreenIdLiteral.AccountEntry,
    SetupFirstRunScreenIdLiteral.HouseholdSelection,
    SetupFirstRunScreenIdLiteral.ParentInstall,
    SetupFirstRunScreenIdLiteral.ParentBootstrapAgreement,
    SetupFirstRunScreenIdLiteral.ParentBootstrapCodeEntry,
    SetupFirstRunScreenIdLiteral.ParentInstallProgress,
    SetupFirstRunScreenIdLiteral.ParentGuidedSetupStart,
    SetupFirstRunScreenIdLiteral.ChildProfile,
    SetupFirstRunScreenIdLiteral.ChildPairing,
    SetupFirstRunScreenIdLiteral.ChildInstallInstructions,
    SetupFirstRunScreenIdLiteral.WaitingForChildDevice,
    SetupFirstRunScreenIdLiteral.ConfirmChildTrust,
    SetupFirstRunScreenIdLiteral.PermissionReadiness,
    SetupFirstRunScreenIdLiteral.PolicyBaseline,
    SetupFirstRunScreenIdLiteral.DataCustody,
    SetupFirstRunScreenIdLiteral.Recovery,
    SetupFirstRunScreenIdLiteral.ManualRequired,
    SetupFirstRunScreenIdLiteral.SetupBlocked,
    SetupFirstRunScreenIdLiteral.SetupComplete
  )
);

export type SetupFirstRunStateId = Infer<typeof SetupFirstRunStateIdSchema>;
export type SetupFirstRunScreenId = Infer<typeof SetupFirstRunScreenIdSchema>;

export const SetupFirstRunStateId = {
  Welcome: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.Welcome),
  AccountEntry: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.AccountEntry),
  HouseholdSelection: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.HouseholdSelection),
  ParentInstall: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ParentInstall),
  ParentBootstrapAgreement: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ParentBootstrapAgreement),
  ParentBootstrapCodeEntry: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ParentBootstrapCodeEntry),
  ParentInstallProgress: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ParentInstallProgress),
  ParentGuidedSetupStart: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ParentGuidedSetupStart),
  ChildProfile: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ChildProfile),
  ChildPairing: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ChildPairing),
  ChildInstallInstructions: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ChildInstallInstructions),
  WaitingForChildDevice: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.WaitingForChildDevice),
  ConfirmChildTrust: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ConfirmChildTrust),
  PermissionReadiness: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.PermissionReadiness),
  PolicyBaseline: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.PolicyBaseline),
  DataCustody: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.DataCustody),
  SetupDegraded: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.SetupDegraded),
  ManualRequired: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.ManualRequired),
  SetupBlocked: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.SetupBlocked),
  SetupComplete: SetupFirstRunStateIdSchema.parse(SetupFirstRunStateIdLiteral.SetupComplete),
} as const;

export const SetupFirstRunScreenId = {
  Welcome: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.Welcome),
  AccountEntry: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.AccountEntry),
  HouseholdSelection: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.HouseholdSelection),
  ParentInstall: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ParentInstall),
  ParentBootstrapAgreement: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ParentBootstrapAgreement),
  ParentBootstrapCodeEntry: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ParentBootstrapCodeEntry),
  ParentInstallProgress: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ParentInstallProgress),
  ParentGuidedSetupStart: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ParentGuidedSetupStart),
  ChildProfile: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ChildProfile),
  ChildPairing: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ChildPairing),
  ChildInstallInstructions: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ChildInstallInstructions),
  WaitingForChildDevice: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.WaitingForChildDevice),
  ConfirmChildTrust: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ConfirmChildTrust),
  PermissionReadiness: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.PermissionReadiness),
  PolicyBaseline: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.PolicyBaseline),
  DataCustody: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.DataCustody),
  Recovery: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.Recovery),
  ManualRequired: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.ManualRequired),
  SetupBlocked: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.SetupBlocked),
  SetupComplete: SetupFirstRunScreenIdSchema.parse(SetupFirstRunScreenIdLiteral.SetupComplete),
} as const;

const SetupFirstRunStateIdArraySchema = withParser(Schema.Array(SetupFirstRunStateIdSchema));

export const SetupFirstRunStateSchema = withParser(
  Schema.Struct({
    stateId: SetupFirstRunStateIdSchema,
    screenId: SetupFirstRunScreenIdSchema,
    readinessState: Schema.Union(SetupReadinessOverallStateSchema, Schema.Null),
    manualRecoveryRequired: Schema.Boolean,
    allowedNextStateIds: SetupFirstRunStateIdArraySchema,
    terminal: Schema.Boolean,
    degraded: Schema.Boolean,
    manualRequired: Schema.Boolean,
    blocked: Schema.Boolean,
    complete: Schema.Boolean,
  })
);

export const SetupFirstRunStateRequestSchema = withParser(
  Schema.Struct({
    stateId: SetupFirstRunStateIdSchema,
    readinessReport: Schema.Union(SetupReadinessReportSchema, Schema.Null),
  })
);

export const SetupFirstRunTransitionRequestSchema = withParser(
  Schema.Struct({
    fromStateId: SetupFirstRunStateIdSchema,
    toStateId: SetupFirstRunStateIdSchema,
    readinessReport: Schema.Union(SetupReadinessReportSchema, Schema.Null),
  })
);

export type SetupFirstRunState = Infer<typeof SetupFirstRunStateSchema>;
export type SetupFirstRunStateRequest = Infer<typeof SetupFirstRunStateRequestSchema>;
export type SetupFirstRunTransitionRequest = Infer<typeof SetupFirstRunTransitionRequestSchema>;

type SetupFirstRunStateDefinition = {
  readonly screenId: SetupFirstRunScreenId;
  readonly baseNextStateIds: readonly SetupFirstRunStateId[];
  readonly readinessGate: boolean;
  readonly terminal: boolean;
};

const SetupFirstRunStateGraph: { [StateId in SetupFirstRunStateId]: SetupFirstRunStateDefinition } = {
  welcome: {
    screenId: SetupFirstRunScreenId.Welcome,
    baseNextStateIds: [SetupFirstRunStateId.AccountEntry],
    readinessGate: false,
    terminal: false,
  },
  'account-entry': {
    screenId: SetupFirstRunScreenId.AccountEntry,
    baseNextStateIds: [SetupFirstRunStateId.HouseholdSelection],
    readinessGate: false,
    terminal: false,
  },
  'household-selection': {
    screenId: SetupFirstRunScreenId.HouseholdSelection,
    baseNextStateIds: [SetupFirstRunStateId.ParentInstall],
    readinessGate: false,
    terminal: false,
  },
  'parent-install': {
    screenId: SetupFirstRunScreenId.ParentInstall,
    baseNextStateIds: [SetupFirstRunStateId.ParentBootstrapAgreement],
    readinessGate: false,
    terminal: false,
  },
  'parent-bootstrap-agreement': {
    screenId: SetupFirstRunScreenId.ParentBootstrapAgreement,
    baseNextStateIds: [SetupFirstRunStateId.ParentBootstrapCodeEntry],
    readinessGate: false,
    terminal: false,
  },
  'parent-bootstrap-code-entry': {
    screenId: SetupFirstRunScreenId.ParentBootstrapCodeEntry,
    baseNextStateIds: [SetupFirstRunStateId.ParentInstallProgress],
    readinessGate: false,
    terminal: false,
  },
  'parent-install-progress': {
    screenId: SetupFirstRunScreenId.ParentInstallProgress,
    baseNextStateIds: [SetupFirstRunStateId.ParentGuidedSetupStart],
    readinessGate: false,
    terminal: false,
  },
  'parent-guided-setup-start': {
    screenId: SetupFirstRunScreenId.ParentGuidedSetupStart,
    baseNextStateIds: [SetupFirstRunStateId.ChildProfile],
    readinessGate: false,
    terminal: false,
  },
  'child-profile': {
    screenId: SetupFirstRunScreenId.ChildProfile,
    baseNextStateIds: [SetupFirstRunStateId.ChildPairing],
    readinessGate: false,
    terminal: false,
  },
  'child-pairing': {
    screenId: SetupFirstRunScreenId.ChildPairing,
    baseNextStateIds: [SetupFirstRunStateId.ChildInstallInstructions],
    readinessGate: false,
    terminal: false,
  },
  'child-install-instructions': {
    screenId: SetupFirstRunScreenId.ChildInstallInstructions,
    baseNextStateIds: [SetupFirstRunStateId.WaitingForChildDevice],
    readinessGate: false,
    terminal: false,
  },
  'waiting-for-child-device': {
    screenId: SetupFirstRunScreenId.WaitingForChildDevice,
    baseNextStateIds: [SetupFirstRunStateId.ConfirmChildTrust],
    readinessGate: true,
    terminal: false,
  },
  'confirm-child-trust': {
    screenId: SetupFirstRunScreenId.ConfirmChildTrust,
    baseNextStateIds: [SetupFirstRunStateId.PermissionReadiness],
    readinessGate: true,
    terminal: false,
  },
  'permission-readiness': {
    screenId: SetupFirstRunScreenId.PermissionReadiness,
    baseNextStateIds: [SetupFirstRunStateId.PolicyBaseline],
    readinessGate: true,
    terminal: false,
  },
  'policy-baseline': {
    screenId: SetupFirstRunScreenId.PolicyBaseline,
    baseNextStateIds: [SetupFirstRunStateId.DataCustody],
    readinessGate: true,
    terminal: false,
  },
  'data-custody': {
    screenId: SetupFirstRunScreenId.DataCustody,
    baseNextStateIds: [SetupFirstRunStateId.SetupComplete],
    readinessGate: true,
    terminal: false,
  },
  'setup-degraded': {
    screenId: SetupFirstRunScreenId.Recovery,
    baseNextStateIds: [
      SetupFirstRunStateId.DataCustody,
      SetupFirstRunStateId.ManualRequired,
      SetupFirstRunStateId.SetupBlocked,
    ],
    readinessGate: true,
    terminal: true,
  },
  'manual-required': {
    screenId: SetupFirstRunScreenId.ManualRequired,
    baseNextStateIds: [SetupFirstRunStateId.DataCustody, SetupFirstRunStateId.SetupBlocked],
    readinessGate: true,
    terminal: true,
  },
  'setup-blocked': {
    screenId: SetupFirstRunScreenId.SetupBlocked,
    baseNextStateIds: [SetupFirstRunStateId.DataCustody, SetupFirstRunStateId.ManualRequired],
    readinessGate: true,
    terminal: true,
  },
  'setup-complete': {
    screenId: SetupFirstRunScreenId.SetupComplete,
    baseNextStateIds: [],
    readinessGate: true,
    terminal: true,
  },
};

export function getSetupFirstRunScreenId(input: SetupFirstRunStateId): SetupFirstRunScreenId {
  const stateId = SetupFirstRunStateIdSchema.parse(input);
  return SetupFirstRunStateGraph[stateId].screenId;
}

export function getAllowedSetupFirstRunTransitions(input: SetupFirstRunStateId): readonly SetupFirstRunStateId[] {
  const stateId = SetupFirstRunStateIdSchema.parse(input);
  const definition = SetupFirstRunStateGraph[stateId];

  if (definition.terminal) {
    return SetupFirstRunStateIdArraySchema.parse(definition.baseNextStateIds);
  }

  const nextStateIds = [
    ...definition.baseNextStateIds,
    SetupFirstRunStateId.ManualRequired,
    SetupFirstRunStateId.SetupBlocked,
  ];

  if (definition.readinessGate) {
    nextStateIds.push(SetupFirstRunStateId.SetupDegraded);
  }

  return SetupFirstRunStateIdArraySchema.parse(uniqueStateIds(nextStateIds));
}

export function canTransitionSetupFirstRunState(
  fromStateId: SetupFirstRunStateId,
  toStateId: SetupFirstRunStateId
): boolean {
  const from = SetupFirstRunStateIdSchema.parse(fromStateId);
  const to = SetupFirstRunStateIdSchema.parse(toStateId);

  return getAllowedSetupFirstRunTransitions(from).includes(to);
}

export function resolveSetupFirstRunState(input: SetupFirstRunStateRequest): SetupFirstRunState {
  const request = SetupFirstRunStateRequestSchema.parse(input);
  const readinessState =
    request.readinessReport === null ? null : deriveSetupReadinessOverallState(request.readinessReport);
  const manualRecoveryRequired =
    request.readinessReport === null ? false : setupReadinessNeedsManualRecovery(request.readinessReport);
  const resolvedStateId = resolveSetupFirstRunStateId(request.stateId, readinessState, manualRecoveryRequired);
  const definition = SetupFirstRunStateGraph[resolvedStateId];

  return SetupFirstRunStateSchema.parse({
    stateId: resolvedStateId,
    screenId: definition.screenId,
    readinessState,
    manualRecoveryRequired,
    allowedNextStateIds: getAllowedSetupFirstRunTransitions(resolvedStateId),
    terminal: definition.terminal,
    degraded: resolvedStateId === SetupFirstRunStateId.SetupDegraded,
    manualRequired: resolvedStateId === SetupFirstRunStateId.ManualRequired,
    blocked: resolvedStateId === SetupFirstRunStateId.SetupBlocked,
    complete: resolvedStateId === SetupFirstRunStateId.SetupComplete,
  });
}

export function transitionSetupFirstRunState(input: SetupFirstRunTransitionRequest): SetupFirstRunState {
  const transition = SetupFirstRunTransitionRequestSchema.parse(input);

  if (transition.toStateId === SetupFirstRunStateId.SetupComplete && transition.readinessReport === null) {
    throw new Error('Setup first-run completion requires a readiness report.');
  }

  if (!canTransitionSetupFirstRunState(transition.fromStateId, transition.toStateId)) {
    throw new Error(`Invalid setup first-run transition from ${transition.fromStateId} to ${transition.toStateId}.`);
  }

  const resolvedState = resolveSetupFirstRunState({
    stateId: transition.toStateId,
    readinessReport: transition.readinessReport,
  });

  if (!canTransitionSetupFirstRunState(transition.fromStateId, resolvedState.stateId)) {
    throw new Error(
      `Resolved setup first-run transition from ${transition.fromStateId} to ${resolvedState.stateId} is not allowed.`
    );
  }

  return resolvedState;
}

function resolveSetupFirstRunStateId(
  stateId: SetupFirstRunStateId,
  readinessState: Infer<typeof SetupReadinessOverallStateSchema> | null,
  manualRecoveryRequired: boolean
): SetupFirstRunStateId {
  const parsedStateId = SetupFirstRunStateIdSchema.parse(stateId);
  const definition = SetupFirstRunStateGraph[parsedStateId];

  if (!definition.readinessGate || readinessState === null) {
    return parsedStateId;
  }

  if (manualRecoveryRequired) {
    return SetupFirstRunStateId.ManualRequired;
  }

  if (readinessState === SetupReadinessOverallState.Degraded) {
    return SetupFirstRunStateId.SetupDegraded;
  }

  if (readinessState === SetupReadinessOverallState.Blocked) {
    return SetupFirstRunStateId.SetupBlocked;
  }

  if (
    parsedStateId === SetupFirstRunStateId.DataCustody ||
    parsedStateId === SetupFirstRunStateId.SetupDegraded ||
    parsedStateId === SetupFirstRunStateId.ManualRequired ||
    parsedStateId === SetupFirstRunStateId.SetupBlocked
  ) {
    return SetupFirstRunStateId.SetupComplete;
  }

  return parsedStateId;
}

function uniqueStateIds(stateIds: readonly SetupFirstRunStateId[]): readonly SetupFirstRunStateId[] {
  const seen = new Set<SetupFirstRunStateId>();

  return stateIds.filter((stateId) => {
    if (seen.has(stateId)) {
      return false;
    }

    seen.add(stateId);
    return true;
  });
}
