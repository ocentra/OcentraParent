import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';
import { SetupPairingState } from '@ocentra-parent/setup-domain/pairing-intent';
import {
  createSetupReadinessChecklist,
  SetupAccountReadinessState,
  SetupChildAppReadinessState,
  SetupDataCustodySyncState,
  SetupNetworkReachabilityState,
  SetupParentAppReadinessState,
  SetupPermissionReadinessState,
  SetupPolicyBaselineState,
  SetupReadinessReportSchema,
  SetupRecoveryState,
  type SetupReadinessChecklistItem,
  type SetupReadinessReport,
} from '@ocentra-parent/setup-domain/readiness';
import {
  getAllowedSetupFirstRunTransitions,
  resolveSetupFirstRunState,
  transitionSetupFirstRunState,
  SetupFirstRunStateId,
  type SetupFirstRunState,
  type SetupFirstRunStateId as SetupFirstRunStateIdValue,
} from '@ocentra-parent/setup-domain/setup-state-machine';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';

const SetupRequiredLabels = [
  'notImplemented',
  'previewOnly',
  'manualRequired',
  'readyForTest',
  'productionReady',
  'blocked',
  'stale',
  'degraded',
  'unavailable',
] as const;

const SetupSourceCustodyLabels = [
  'live-local',
  'physical-household-lan',
  'parent-cache',
  'parent-owned-storage',
  'stale',
  'degraded',
  'unavailable',
  'manual-required',
] as const;

const ParentEntryPhaseStates = [
  SetupFirstRunStateId.Welcome,
  SetupFirstRunStateId.AccountEntry,
  SetupFirstRunStateId.HouseholdSelection,
  SetupFirstRunStateId.ParentInstall,
  SetupFirstRunStateId.ParentBootstrapAgreement,
  SetupFirstRunStateId.ParentBootstrapCodeEntry,
  SetupFirstRunStateId.ParentInstallProgress,
  SetupFirstRunStateId.ParentGuidedSetupStart,
] as const;

const ChildReadinessPhaseStates = [
  SetupFirstRunStateId.ChildProfile,
  SetupFirstRunStateId.ChildPairing,
  SetupFirstRunStateId.ChildInstallInstructions,
  SetupFirstRunStateId.WaitingForChildDevice,
  SetupFirstRunStateId.ConfirmChildTrust,
  SetupFirstRunStateId.PermissionReadiness,
  SetupFirstRunStateId.PolicyBaseline,
  SetupFirstRunStateId.DataCustody,
] as const;

const ProductClaim = decodeDisplayText(
  'First-run setup rendering is a route-contract projection only. It does not claim live account readiness, signed installer readiness, pairing runtime ownership, device-trust proof, data-custody execution, or production onboarding completion.'
);

const SetupDetailLabels = {
  ScreensMapped: decodeDisplayText('Screens mapped'),
  ReadyGate: decodeDisplayText('Ready gate'),
  RecoveryProjection: decodeDisplayText('Recovery projection'),
  SignedInNoHousehold: decodeDisplayText('Signed-in without household'),
  CoParentInviteState: decodeDisplayText('Co-parent invite'),
  ObserverInviteState: decodeDisplayText('Observer invite'),
  RoleVisibility: decodeDisplayText('Role visibility'),
  SupportAccessStatus: decodeDisplayText('Support access status'),
  TrustStatus: decodeDisplayText('Trust status'),
  WrongAccountState: decodeDisplayText('Wrong-account state'),
  ReauthState: decodeDisplayText('Reauth/manual-required state'),
  RevokedChildState: decodeDisplayText('Revoked child state'),
  StaleParentState: decodeDisplayText('Stale parent state'),
  DirectEntryState: decodeDisplayText('Direct-entry-required state'),
  States: decodeDisplayText('States'),
  Screens: decodeDisplayText('Screens'),
  NextStates: decodeDisplayText('Next states'),
  HandoffBoundary: decodeDisplayText('Handoff boundary'),
  EmptyState: decodeDisplayText('Empty state'),
  DegradedState: decodeDisplayText('Degraded state'),
  ManualRequiredState: decodeDisplayText('Manual-required state'),
  BlockedCompletion: decodeDisplayText('Blocked completion'),
  ReadyCompletion: decodeDisplayText('Ready completion'),
  ChecklistRows: decodeDisplayText('Checklist rows'),
  DegradedChecklist: decodeDisplayText('Degraded checklist'),
  BlockedChecklist: decodeDisplayText('Blocked checklist'),
  SourceCustodyLabels: decodeDisplayText('Source/custody labels'),
  RequiredLabels: decodeDisplayText('Required labels'),
  AccountHandoff: decodeDisplayText('Account/session owner'),
  RuntimeHandoff: decodeDisplayText('Runtime distribution owner'),
  TrustHandoff: decodeDisplayText('LAN/device-trust owner'),
  CustodyHandoff: decodeDisplayText('Data custody owner'),
  PolicyHandoff: decodeDisplayText('Policy baseline owner'),
} as const;

export type SetupFirstRunPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type SetupFirstRunPanelCard = {
  readonly title: DisplayText;
  readonly summary: DisplayText;
  readonly details: readonly SetupFirstRunPanelDetail[];
};

export type SetupFirstRunPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly summary: DisplayText;
  readonly summaryDetails: readonly SetupFirstRunPanelDetail[];
  readonly cards: readonly SetupFirstRunPanelCard[];
  readonly productClaim: DisplayText;
};

export function createSetupFirstRunPanelIntent(): SetupFirstRunPanelIntent {
  const readyReport = createSetupReadinessReport();
  const pendingTrustReport = createSetupReadinessReport({
    pairingState: SetupPairingState.Accepted,
  });
  const degradedReport = createSetupReadinessReport({
    childAppState: SetupChildAppReadinessState.Offline,
    networkReachabilityState: SetupNetworkReachabilityState.OfflineChild,
  });
  const manualReport = createSetupReadinessReport({
    pairingState: SetupPairingState.Accepted,
    recoveryState: SetupRecoveryState.Required,
  });
  const blockedReport = createSetupReadinessReport({
    accountState: SetupAccountReadinessState.WrongAccount,
    pairingState: SetupPairingState.Accepted,
    dataCustodySyncState: SetupDataCustodySyncState.Blocked,
    networkReachabilityState: SetupNetworkReachabilityState.LanUnavailable,
  });
  const wrongAccountReport = createSetupReadinessReport({
    accountState: SetupAccountReadinessState.WrongAccount,
    pairingState: SetupPairingState.Accepted,
  });
  const recoveryRequiredReport = createSetupReadinessReport({
    accountState: SetupAccountReadinessState.RecoveryRequired,
    pairingState: SetupPairingState.Accepted,
    recoveryState: SetupRecoveryState.Required,
  });
  const revokedChildReport = createSetupReadinessReport({
    childAppState: SetupChildAppReadinessState.Revoked,
    pairingState: SetupPairingState.Accepted,
  });
  const staleParentReport = createSetupReadinessReport({
    parentAppState: SetupParentAppReadinessState.Stale,
    pairingState: SetupPairingState.Accepted,
  });
  const directEntryReport = createSetupReadinessReport({
    networkReachabilityState: SetupNetworkReachabilityState.DirectEntryRequired,
    pairingState: SetupPairingState.Accepted,
  });

  const emptyStartState = resolveSetupFirstRunState({
    stateId: SetupFirstRunStateId.Welcome,
    readinessReport: null,
  });
  const degradedState = resolveSetupFirstRunState({
    stateId: SetupFirstRunStateId.WaitingForChildDevice,
    readinessReport: degradedReport,
  });
  const manualState = resolveSetupFirstRunState({
    stateId: SetupFirstRunStateId.PermissionReadiness,
    readinessReport: manualReport,
  });
  const blockedCompletionState = transitionSetupFirstRunState({
    fromStateId: SetupFirstRunStateId.DataCustody,
    toStateId: SetupFirstRunStateId.SetupComplete,
    readinessReport: blockedReport,
  });
  const readyCompletionState = transitionSetupFirstRunState({
    fromStateId: SetupFirstRunStateId.DataCustody,
    toStateId: SetupFirstRunStateId.SetupComplete,
    readinessReport: readyReport,
  });

  const readyChecklist = createSetupReadinessChecklist(readyReport);
  const pendingTrustChecklist = createSetupReadinessChecklist(pendingTrustReport);
  const degradedChecklist = createSetupReadinessChecklist(degradedReport);
  const blockedChecklist = createSetupReadinessChecklist(blockedReport);
  const wrongAccountChecklist = createSetupReadinessChecklist(wrongAccountReport);
  const recoveryRequiredChecklist = createSetupReadinessChecklist(recoveryRequiredReport);
  const revokedChildChecklist = createSetupReadinessChecklist(revokedChildReport);
  const staleParentChecklist = createSetupReadinessChecklist(staleParentReport);
  const directEntryChecklist = createSetupReadinessChecklist(directEntryReport);

  const phaseCards = [
    createPhaseCard(
      'Parent entry and install',
      'Public entry stays separate from parent bootstrap, install progress, and guided-start handoff.',
      ParentEntryPhaseStates,
      'Account, registration, and household authority remain explicit setup handoffs before package/runtime ownership.'
    ),
    createPhaseCard(
      'Child device and readiness',
      'Child profile, pairing, trust, permissions, policy baseline, and custody stay visible as separate setup screens.',
      ChildReadinessPhaseStates,
      'Waiting, trust, permission, policy, and custody states may degrade, block, or route to manual-required instead of pretending setup succeeded.'
    ),
    createRecoveryCard(
      emptyStartState,
      degradedState,
      manualState,
      blockedCompletionState,
      readyCompletionState
    ),
    createInviteAndRoleCard(),
    createTrustAndSessionCard(
      readyChecklist,
      pendingTrustChecklist,
      wrongAccountChecklist,
      recoveryRequiredChecklist,
      revokedChildChecklist,
      staleParentChecklist,
      directEntryChecklist
    ),
    createChecklistAndLabelCard(readyChecklist, degradedChecklist, blockedChecklist),
    createHandoffCard(),
  ] as const;

  const mappedStates = [...ParentEntryPhaseStates, ...ChildReadinessPhaseStates];

  return {
    eyebrow: decodeDisplayText('Setup route'),
    title: decodeDisplayText('First-run family setup'),
    body: decodeDisplayText(
      'Parent-visible projection of the typed setup-domain first-run state machine, with explicit degraded, blocked, manual-required, and no-fake-ready boundaries.'
    ),
    summary: decodeDisplayText(
      'The Start route maps public entry, parent install, child bootstrap, and readiness gates without collapsing sibling-owner handoffs into a fake complete state.'
    ),
    summaryDetails: [
      detail(SetupDetailLabels.ScreensMapped, literalValue(String(mappedStates.length + 4))),
      detail(
        SetupDetailLabels.ReadyGate,
        decodeDisplayText('setup-complete requires overall readiness = ready after data-custody')
      ),
      detail(
        SetupDetailLabels.RecoveryProjection,
        literalValue('setup-degraded | manual-required | setup-blocked stay visible')
      ),
      detail(PortalDetails.ProductClaim, ProductClaim),
    ],
    cards: phaseCards,
    productClaim: ProductClaim,
  };
}

function createPhaseCard(
  title: string,
  summary: string,
  stateIds: readonly SetupFirstRunStateIdValue[],
  handoffBoundary: string
): SetupFirstRunPanelCard {
  const states = stateIds.map((stateId) =>
    resolveSetupFirstRunState({
      stateId,
      readinessReport: null,
    })
  );
  const lastState = states.at(-1);

  return {
    title: decodeDisplayText(title),
    summary: decodeDisplayText(summary),
    details: [
      detail(SetupDetailLabels.States, literalValue(joinStateIds(states))),
      detail(SetupDetailLabels.Screens, literalValue(joinScreenIds(states))),
      detail(
        SetupDetailLabels.NextStates,
        literalValue(
          lastState === undefined
            ? ''
            : joinLiteralValues(getAllowedSetupFirstRunTransitions(lastState.stateId))
        )
      ),
      detail(SetupDetailLabels.HandoffBoundary, decodeDisplayText(handoffBoundary)),
    ],
  };
}

function createRecoveryCard(
  emptyStartState: SetupFirstRunState,
  degradedState: SetupFirstRunState,
  manualState: SetupFirstRunState,
  blockedCompletionState: SetupFirstRunState,
  readyCompletionState: SetupFirstRunState
): SetupFirstRunPanelCard {
  return {
    title: decodeDisplayText('Recovery and completion gates'),
    summary: decodeDisplayText(
      'Setup never skips past degraded, manual-required, or blocked outcomes, and setup-complete stays unavailable until the readiness gate resolves to ready.'
    ),
    details: [
      detail(
        SetupDetailLabels.EmptyState,
        literalValue(`${emptyStartState.stateId} -> ${emptyStartState.screenId} | readiness-report-absent`)
      ),
      detail(
        SetupDetailLabels.DegradedState,
        literalValue(
          `${degradedState.stateId} -> ${degradedState.screenId} | ${String(degradedState.readinessState)}`
        )
      ),
      detail(
        SetupDetailLabels.ManualRequiredState,
        literalValue(
          `${manualState.stateId} -> ${manualState.screenId} | recovery-required | ${String(manualState.readinessState)}`
        )
      ),
      detail(
        SetupDetailLabels.BlockedCompletion,
        literalValue(
          `${blockedCompletionState.stateId} -> ${blockedCompletionState.screenId} | setup-complete withheld`
        )
      ),
      detail(
        SetupDetailLabels.ReadyCompletion,
        literalValue(
          `${readyCompletionState.stateId} -> ${readyCompletionState.screenId} | ${String(readyCompletionState.readinessState)}`
        )
      ),
    ],
  };
}

function createInviteAndRoleCard(): SetupFirstRunPanelCard {
  return {
    title: decodeDisplayText('Invite, role, and support visibility'),
    summary: decodeDisplayText(
      'First-run setup keeps household role invites, observer read-only scope, and audited support access separate from parent-owner and child-device trust.'
    ),
    details: [
      detail(
        SetupDetailLabels.SignedInNoHousehold,
        literalValue('account-entry -> household-selection | signed-in account still lacks household authority')
      ),
      detail(
        SetupDetailLabels.CoParentInviteState,
        literalValue('pending invite -> co-parent role stays distinct from parent-owner and child-device trust')
      ),
      detail(
        SetupDetailLabels.ObserverInviteState,
        literalValue('pending invite -> observer stays read-only and cannot inherit owner controls')
      ),
      detail(
        SetupDetailLabels.RoleVisibility,
        literalValue('parent-owner | co-parent | observer | child-profile | child-device remain distinct UI concepts')
      ),
      detail(
        SetupDetailLabels.SupportAccessStatus,
        literalValue('support-admin remains a separate audited support state | never parent-owner')
      ),
    ],
  };
}

function createTrustAndSessionCard(
  readyChecklist: readonly SetupReadinessChecklistItem[],
  pendingTrustChecklist: readonly SetupReadinessChecklistItem[],
  wrongAccountChecklist: readonly SetupReadinessChecklistItem[],
  recoveryRequiredChecklist: readonly SetupReadinessChecklistItem[],
  revokedChildChecklist: readonly SetupReadinessChecklistItem[],
  staleParentChecklist: readonly SetupReadinessChecklistItem[],
  directEntryChecklist: readonly SetupReadinessChecklistItem[]
): SetupFirstRunPanelCard {
  return {
    title: decodeDisplayText('Trust and session distinction'),
    summary: decodeDisplayText(
      'Parent login, trusted child-device proof, revoked child runtime, stale parent state, and reauth/manual-required recovery all remain visible as separate setup outcomes.'
    ),
    details: [
      detail(
        SetupDetailLabels.TrustStatus,
        literalValue(
          `${checklistItemSummary(pendingTrustChecklist, 'Pairing')} | ${checklistItemSummary(readyChecklist, 'Pairing')}`
        )
      ),
      detail(
        SetupDetailLabels.WrongAccountState,
        literalValue(checklistItemSummary(wrongAccountChecklist, 'Account'))
      ),
      detail(
        SetupDetailLabels.ReauthState,
        literalValue(
          `${checklistItemSummary(recoveryRequiredChecklist, 'Account')} | manual-required-screen`
        )
      ),
      detail(
        SetupDetailLabels.RevokedChildState,
        literalValue(checklistItemSummary(revokedChildChecklist, 'Child service'))
      ),
      detail(
        SetupDetailLabels.StaleParentState,
        literalValue(checklistItemSummary(staleParentChecklist, 'Parent app'))
      ),
      detail(
        SetupDetailLabels.DirectEntryState,
        literalValue(checklistItemSummary(directEntryChecklist, 'Network reachability'))
      ),
    ],
  };
}

function createChecklistAndLabelCard(
  readyChecklist: readonly SetupReadinessChecklistItem[],
  degradedChecklist: readonly SetupReadinessChecklistItem[],
  blockedChecklist: readonly SetupReadinessChecklistItem[]
): SetupFirstRunPanelCard {
  return {
    title: decodeDisplayText('Checklist and label legend'),
    summary: decodeDisplayText(
      'Readiness stays a checklist instead of a boolean, while screen and platform labels keep non-ready states explicit.'
    ),
    details: [
      detail(SetupDetailLabels.ChecklistRows, literalValue(checklistSummary(readyChecklist))),
      detail(SetupDetailLabels.DegradedChecklist, literalValue(checklistSummary(degradedChecklist))),
      detail(SetupDetailLabels.BlockedChecklist, literalValue(checklistSummary(blockedChecklist))),
      detail(SetupDetailLabels.SourceCustodyLabels, literalValue(joinLiteralValues(SetupSourceCustodyLabels))),
      detail(SetupDetailLabels.RequiredLabels, literalValue(joinLiteralValues(SetupRequiredLabels))),
    ],
  };
}

function createHandoffCard(): SetupFirstRunPanelCard {
  return {
    title: decodeDisplayText('Adjacent owner handoffs'),
    summary: decodeDisplayText(
      'The Start route keeps sibling-plan ownership visible instead of promoting setup, package, or device-trust claims into this panel.'
    ),
    details: [
      detail(
        SetupDetailLabels.AccountHandoff,
        literalValue('account-identity-family-plan | account/provider/session implementation')
      ),
      detail(
        SetupDetailLabels.RuntimeHandoff,
        literalValue('parent-desktop-runtime-package-plan | signed installers, updates, publishing')
      ),
      detail(
        SetupDetailLabels.TrustHandoff,
        literalValue('lan-plan + device-trust-bootstrap-plan | LAN and trusted-device proof')
      ),
      detail(
        SetupDetailLabels.CustodyHandoff,
        literalValue('data-custody-storage-plan | export/delete/sync execution')
      ),
      detail(
        SetupDetailLabels.PolicyHandoff,
        literalValue('policy-control-plane-plan | production policy baseline proof')
      ),
    ],
  };
}

function createSetupReadinessReport(overrides: Partial<SetupReadinessReport> = {}): SetupReadinessReport {
  return SetupReadinessReportSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readinessReportId: 'portal-setup-first-run-report-1',
    family: {
      familyId: 'family-local-1',
    },
    parentAccount: {
      parentAccountId: 'parent-account-1',
    },
    parentDevice: {
      deviceId: 'parent-device-1',
      childProfileId: null,
      label: 'Mom phone',
      platform: 'android',
    },
    childProfile: {
      childProfileId: 'child-profile-1',
      displayName: 'Ari',
    },
    pairingIntentId: 'setup-pairing-intent-1',
    accountState: SetupAccountReadinessState.Ready,
    parentAppState: SetupParentAppReadinessState.Ready,
    childAppState: SetupChildAppReadinessState.Ready,
    permissionState: SetupPermissionReadinessState.Granted,
    pairingState: SetupPairingState.Trusted,
    policyBaselineState: SetupPolicyBaselineState.Applied,
    dataCustodySyncState: SetupDataCustodySyncState.Synced,
    networkReachabilityState: SetupNetworkReachabilityState.Reachable,
    recoveryState: SetupRecoveryState.Normal,
    observedAt: '2026-06-01T00:15:00Z',
    checklist: [],
    ...overrides,
  });
}

function checklistSummary(checklist: readonly SetupReadinessChecklistItem[]): string {
  return checklist.map((item) => `${item.label}:${item.state}:${item.supportCode}`).join(' | ');
}

function checklistItemSummary(checklist: readonly SetupReadinessChecklistItem[], label: string): string {
  const item = checklist.find((entry) => entry.label === label);
  return item === undefined ? `${label}:missing` : `${item.label}:${item.state}:${item.supportCode}`;
}

function joinStateIds(states: readonly SetupFirstRunState[]): string {
  return joinLiteralValues(states.map((state) => state.stateId));
}

function joinScreenIds(states: readonly SetupFirstRunState[]): string {
  return joinLiteralValues(states.map((state) => state.screenId));
}

function joinLiteralValues(values: readonly string[]): string {
  return values.join(' | ');
}

function literalValue(value: string): DisplayText {
  return decodeDisplayText(value);
}

function detail(label: DisplayText, value: DisplayText): SetupFirstRunPanelDetail {
  return { label, value };
}

export function readableSetupValue(value: string): DisplayText {
  return PortalReadableValues[value] ?? decodeDisplayText(titleCase(value));
}

function titleCase(value: string): string {
  return value
    .split(/[-_.\s]+/u)
    .filter((part) => part.length > 0)
    .map((part) => part[0]!.toUpperCase() + part.slice(1))
    .join(' ');
}
