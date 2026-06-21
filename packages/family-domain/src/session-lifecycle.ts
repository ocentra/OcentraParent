import {
  AuditRequirementState,
  SessionFreshnessState,
} from '@ocentra-parent/schema-domain/family-household-authority';
import {
  SessionActivityState,
  SessionCredentialIssuanceAction,
  SessionCredentialIssuanceDecision,
  SessionCredentialIssuanceDecisionSchema,
  SessionCredentialIssuanceInput,
  SessionCredentialIssuanceInputSchema,
  SessionCredentialIssuanceState,
  SessionCredentialKind,
  SessionLifecycleAction,
  SessionTokenAuthorizationState,
  SessionTokenDecision,
  SessionTokenDecisionSchema,
  SessionTokenFailureReason,
  SessionTokenInput,
  SessionTokenInputSchema,
  TokenAuditRedactionState,
  TokenReplayState,
  TokenValidityWindowState,
} from '@ocentra-parent/schema-domain/family-session-lifecycle';

const CredentialKindByAction: Record<SessionLifecycleAction, SessionCredentialKind> = Object.freeze({
  'refresh-browser-session': SessionCredentialKind.BrowserUserSession,
  'perform-privileged-user-action': SessionCredentialKind.BrowserUserSession,
  'logout-browser-session': SessionCredentialKind.BrowserUserSession,
  'use-device-credential': SessionCredentialKind.DeviceCredential,
  'accept-invite-token': SessionCredentialKind.InviteToken,
  'accept-pairing-token': SessionCredentialKind.PairingToken,
  'redeem-recovery-token': SessionCredentialKind.RecoveryToken,
  'use-remote-session-grant': SessionCredentialKind.RemoteSessionGrant,
});

const CredentialKindByIssuanceAction: Record<
  SessionCredentialIssuanceAction,
  SessionCredentialKind
> = Object.freeze({
  'create-browser-session': SessionCredentialKind.BrowserUserSession,
  'rotate-browser-session': SessionCredentialKind.BrowserUserSession,
  'issue-device-credential': SessionCredentialKind.DeviceCredential,
  'issue-invite-token': SessionCredentialKind.InviteToken,
  'issue-pairing-token': SessionCredentialKind.PairingToken,
  'issue-recovery-token': SessionCredentialKind.RecoveryToken,
  'issue-remote-session-grant': SessionCredentialKind.RemoteSessionGrant,
});

export function authorizeSessionTokenAction(input: SessionTokenInput): SessionTokenDecision {
  const parsedInput = SessionTokenInputSchema.parse(input);

  if (parsedInput.replayState !== TokenReplayState.Fresh) {
    return rejectedSessionTokenAction(parsedInput.action, SessionTokenFailureReason.TokenReplayRejected);
  }

  const validityFailureReason = tokenValidityWindowFailureReason(parsedInput.validityWindowState);
  if (validityFailureReason !== null) {
    return rejectedSessionTokenAction(parsedInput.action, validityFailureReason);
  }

  const activityFailureReason = sessionActivityFailureReason(parsedInput.activityState);
  if (activityFailureReason !== null) {
    return rejectedSessionTokenAction(parsedInput.action, activityFailureReason);
  }

  if (sessionFreshnessFailureReason(parsedInput.action, parsedInput.sessionFreshnessState) !== null) {
    return rejectedSessionTokenAction(parsedInput.action, SessionTokenFailureReason.SessionNotFresh);
  }

  if (!credentialKindMatchesAction(parsedInput.credentialKind, parsedInput.action)) {
    return rejectedSessionTokenAction(parsedInput.action, SessionTokenFailureReason.WrongCredentialKind);
  }

  return SessionTokenDecisionSchema.parse({
    authorizationState: SessionTokenAuthorizationState.Authorized,
    auditRequirementState: auditRequirementState(parsedInput.action),
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason: null,
  });
}

export function authorizeSessionCredentialIssuance(
  input: SessionCredentialIssuanceInput
): SessionCredentialIssuanceDecision {
  const parsedInput = SessionCredentialIssuanceInputSchema.parse(input);

  if (!credentialKindMatchesIssuanceAction(parsedInput.issuedCredentialKind, parsedInput.issuanceAction)) {
    return rejectedSessionCredentialIssuance(SessionTokenFailureReason.WrongCredentialKind);
  }

  if (parsedInput.issuanceAction === SessionCredentialIssuanceAction.CreateBrowserSession) {
    return SessionCredentialIssuanceDecisionSchema.parse({
      issuanceState: SessionCredentialIssuanceState.Created,
      auditRequirementState: AuditRequirementState.Required,
      auditRedactionState: TokenAuditRedactionState.Redacted,
      failureReason: null,
    });
  }

  if (parsedInput.sourceSession === null) {
    return rejectedSessionCredentialIssuance(SessionTokenFailureReason.SessionLoggedOut);
  }

  const sourceSessionDecision = authorizeSessionTokenAction({
    ...parsedInput.sourceSession,
    action: sourceSessionActionForIssuance(parsedInput.issuanceAction),
  });

  if (sourceSessionDecision.failureReason !== null) {
    return rejectedSessionCredentialIssuance(sourceSessionDecision.failureReason);
  }

  return SessionCredentialIssuanceDecisionSchema.parse({
    issuanceState:
      parsedInput.issuanceAction === SessionCredentialIssuanceAction.RotateBrowserSession
        ? SessionCredentialIssuanceState.Rotated
        : SessionCredentialIssuanceState.Issued,
    auditRequirementState: AuditRequirementState.Required,
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason: null,
  });
}

function rejectedSessionTokenAction(
  action: SessionLifecycleAction,
  failureReason: SessionTokenFailureReason
): SessionTokenDecision {
  return SessionTokenDecisionSchema.parse({
    authorizationState: SessionTokenAuthorizationState.Rejected,
    auditRequirementState: auditRequirementState(action),
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason,
  });
}

function rejectedSessionCredentialIssuance(
  failureReason: SessionTokenFailureReason
): SessionCredentialIssuanceDecision {
  return SessionCredentialIssuanceDecisionSchema.parse({
    issuanceState: SessionCredentialIssuanceState.Rejected,
    auditRequirementState: AuditRequirementState.Required,
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason,
  });
}

function credentialKindMatchesAction(
  credentialKind: SessionCredentialKind,
  action: SessionLifecycleAction
): boolean {
  return CredentialKindByAction[action] === credentialKind;
}

function credentialKindMatchesIssuanceAction(
  credentialKind: SessionCredentialKind,
  issuanceAction: SessionCredentialIssuanceAction
): boolean {
  return CredentialKindByIssuanceAction[issuanceAction] === credentialKind;
}

function sourceSessionActionForIssuance(
  issuanceAction: SessionCredentialIssuanceAction
): SessionLifecycleAction {
  if (issuanceAction === SessionCredentialIssuanceAction.RotateBrowserSession) {
    return SessionLifecycleAction.RefreshBrowserSession;
  }

  return SessionLifecycleAction.PerformPrivilegedUserAction;
}

function sessionActionRequiresFreshness(action: SessionLifecycleAction): boolean {
  return (
    action === SessionLifecycleAction.PerformPrivilegedUserAction ||
    action === SessionLifecycleAction.UseRemoteSessionGrant
  );
}

function auditRequirementState(action: SessionLifecycleAction) {
  if (action === SessionLifecycleAction.RefreshBrowserSession) {
    return AuditRequirementState.NotRequired;
  }

  return AuditRequirementState.Required;
}

function tokenValidityWindowFailureReason(
  validityWindowState: TokenValidityWindowState
): SessionTokenFailureReason | null {
  if (
    validityWindowState === TokenValidityWindowState.Valid ||
    validityWindowState === TokenValidityWindowState.ValidWithinClockSkewTolerance
  ) {
    return null;
  }

  if (validityWindowState === TokenValidityWindowState.Expired) {
    return SessionTokenFailureReason.TokenExpired;
  }

  return SessionTokenFailureReason.TokenNotYetValid;
}

function sessionActivityFailureReason(
  activityState: SessionActivityState
): SessionTokenFailureReason | null {
  switch (activityState) {
    case SessionActivityState.Active:
      return null;
    case SessionActivityState.LoggedOut:
      return SessionTokenFailureReason.SessionLoggedOut;
    case SessionActivityState.Revoked:
      return SessionTokenFailureReason.SessionRevoked;
    case SessionActivityState.GloballyRevoked:
      return SessionTokenFailureReason.SessionGloballyRevoked;
  }

  return null;
}

function sessionFreshnessFailureReason(
  action: SessionLifecycleAction,
  sessionFreshnessState: typeof SessionFreshnessState[keyof typeof SessionFreshnessState]
): SessionTokenFailureReason | null {
  if (sessionActionRequiresFreshness(action) && sessionFreshnessState !== SessionFreshnessState.Fresh) {
    return SessionTokenFailureReason.SessionNotFresh;
  }

  return null;
}
