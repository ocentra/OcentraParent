import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AuditRequirementState,
  AuditRequirementStateSchema,
  SessionFreshnessState,
  SessionFreshnessStateSchema,
} from './household-authority';

export const SessionCredentialKindLiteral = {
  BrowserUserSession: 'browser-user-session',
  DeviceCredential: 'device-credential',
  InviteToken: 'invite-token',
  PairingToken: 'pairing-token',
  RecoveryToken: 'recovery-token',
  RemoteSessionGrant: 'remote-session-grant',
} as const;

export const SessionLifecycleActionLiteral = {
  RefreshBrowserSession: 'refresh-browser-session',
  PerformPrivilegedUserAction: 'perform-privileged-user-action',
  LogoutBrowserSession: 'logout-browser-session',
  UseDeviceCredential: 'use-device-credential',
  AcceptInviteToken: 'accept-invite-token',
  AcceptPairingToken: 'accept-pairing-token',
  RedeemRecoveryToken: 'redeem-recovery-token',
  UseRemoteSessionGrant: 'use-remote-session-grant',
} as const;

export const SessionCredentialIssuanceActionLiteral = {
  CreateBrowserSession: 'create-browser-session',
  RotateBrowserSession: 'rotate-browser-session',
  IssueDeviceCredential: 'issue-device-credential',
  IssueInviteToken: 'issue-invite-token',
  IssuePairingToken: 'issue-pairing-token',
  IssueRecoveryToken: 'issue-recovery-token',
  IssueRemoteSessionGrant: 'issue-remote-session-grant',
} as const;

export const SessionActivityStateLiteral = {
  Active: 'active',
  LoggedOut: 'logged-out',
  Revoked: 'revoked',
  GloballyRevoked: 'globally-revoked',
} as const;

export const TokenReplayStateLiteral = {
  Fresh: 'fresh',
  ReplayDetected: 'replay-detected',
} as const;

export const TokenValidityWindowStateLiteral = {
  Valid: 'valid',
  ValidWithinClockSkewTolerance: 'valid-within-clock-skew-tolerance',
  Expired: 'expired',
  NotYetValid: 'not-yet-valid',
} as const;

export const SessionTokenAuthorizationStateLiteral = {
  Authorized: 'authorized',
  Rejected: 'rejected',
} as const;

export const SessionCredentialIssuanceStateLiteral = {
  Created: 'created',
  Rotated: 'rotated',
  Issued: 'issued',
  Rejected: 'rejected',
} as const;

export const TokenAuditRedactionStateLiteral = {
  Redacted: 'redacted',
} as const;

export const SessionTokenFailureReasonLiteral = {
  TokenExpired: 'token-expired',
  TokenNotYetValid: 'token-not-yet-valid',
  TokenReplayRejected: 'token-replay-rejected',
  SessionLoggedOut: 'session-logged-out',
  SessionRevoked: 'session-revoked',
  SessionGloballyRevoked: 'session-globally-revoked',
  SessionNotFresh: 'session-not-fresh',
  WrongCredentialKind: 'wrong-credential-kind',
} as const;

export const SessionCredentialKindSchema = withParser(
  Schema.Literal(
    SessionCredentialKindLiteral.BrowserUserSession,
    SessionCredentialKindLiteral.DeviceCredential,
    SessionCredentialKindLiteral.InviteToken,
    SessionCredentialKindLiteral.PairingToken,
    SessionCredentialKindLiteral.RecoveryToken,
    SessionCredentialKindLiteral.RemoteSessionGrant
  )
);

export const SessionLifecycleActionSchema = withParser(
  Schema.Literal(
    SessionLifecycleActionLiteral.RefreshBrowserSession,
    SessionLifecycleActionLiteral.PerformPrivilegedUserAction,
    SessionLifecycleActionLiteral.LogoutBrowserSession,
    SessionLifecycleActionLiteral.UseDeviceCredential,
    SessionLifecycleActionLiteral.AcceptInviteToken,
    SessionLifecycleActionLiteral.AcceptPairingToken,
    SessionLifecycleActionLiteral.RedeemRecoveryToken,
    SessionLifecycleActionLiteral.UseRemoteSessionGrant
  )
);

export const SessionCredentialIssuanceActionSchema = withParser(
  Schema.Literal(
    SessionCredentialIssuanceActionLiteral.CreateBrowserSession,
    SessionCredentialIssuanceActionLiteral.RotateBrowserSession,
    SessionCredentialIssuanceActionLiteral.IssueDeviceCredential,
    SessionCredentialIssuanceActionLiteral.IssueInviteToken,
    SessionCredentialIssuanceActionLiteral.IssuePairingToken,
    SessionCredentialIssuanceActionLiteral.IssueRecoveryToken,
    SessionCredentialIssuanceActionLiteral.IssueRemoteSessionGrant
  )
);

export const SessionActivityStateSchema = withParser(
  Schema.Literal(
    SessionActivityStateLiteral.Active,
    SessionActivityStateLiteral.LoggedOut,
    SessionActivityStateLiteral.Revoked,
    SessionActivityStateLiteral.GloballyRevoked
  )
);

export const TokenReplayStateSchema = withParser(
  Schema.Literal(TokenReplayStateLiteral.Fresh, TokenReplayStateLiteral.ReplayDetected)
);

export const TokenValidityWindowStateSchema = withParser(
  Schema.Literal(
    TokenValidityWindowStateLiteral.Valid,
    TokenValidityWindowStateLiteral.ValidWithinClockSkewTolerance,
    TokenValidityWindowStateLiteral.Expired,
    TokenValidityWindowStateLiteral.NotYetValid
  )
);

export const SessionTokenAuthorizationStateSchema = withParser(
  Schema.Literal(
    SessionTokenAuthorizationStateLiteral.Authorized,
    SessionTokenAuthorizationStateLiteral.Rejected
  )
);

export const SessionCredentialIssuanceStateSchema = withParser(
  Schema.Literal(
    SessionCredentialIssuanceStateLiteral.Created,
    SessionCredentialIssuanceStateLiteral.Rotated,
    SessionCredentialIssuanceStateLiteral.Issued,
    SessionCredentialIssuanceStateLiteral.Rejected
  )
);

export const TokenAuditRedactionStateSchema = withParser(
  Schema.Literal(TokenAuditRedactionStateLiteral.Redacted)
);

export const SessionTokenFailureReasonSchema = withParser(
  Schema.Literal(
    SessionTokenFailureReasonLiteral.TokenExpired,
    SessionTokenFailureReasonLiteral.TokenNotYetValid,
    SessionTokenFailureReasonLiteral.TokenReplayRejected,
    SessionTokenFailureReasonLiteral.SessionLoggedOut,
    SessionTokenFailureReasonLiteral.SessionRevoked,
    SessionTokenFailureReasonLiteral.SessionGloballyRevoked,
    SessionTokenFailureReasonLiteral.SessionNotFresh,
    SessionTokenFailureReasonLiteral.WrongCredentialKind
  )
);

export const SessionTokenInputSchema = withParser(
  Schema.Struct({
    credentialKind: SessionCredentialKindSchema,
    action: SessionLifecycleActionSchema,
    activityState: SessionActivityStateSchema,
    replayState: TokenReplayStateSchema,
    validityWindowState: TokenValidityWindowStateSchema,
    sessionFreshnessState: SessionFreshnessStateSchema,
  })
);

export const SessionCredentialIssuanceInputSchema = withParser(
  Schema.Struct({
    issuanceAction: SessionCredentialIssuanceActionSchema,
    issuedCredentialKind: SessionCredentialKindSchema,
    sourceSession: Schema.Union(SessionTokenInputSchema, Schema.Null),
  })
);

export const SessionTokenDecisionSchema = withParser(
  Schema.Struct({
    authorizationState: SessionTokenAuthorizationStateSchema,
    auditRequirementState: AuditRequirementStateSchema,
    auditRedactionState: TokenAuditRedactionStateSchema,
    failureReason: Schema.Union(SessionTokenFailureReasonSchema, Schema.Null),
  })
);

export const SessionCredentialIssuanceDecisionSchema = withParser(
  Schema.Struct({
    issuanceState: SessionCredentialIssuanceStateSchema,
    auditRequirementState: AuditRequirementStateSchema,
    auditRedactionState: TokenAuditRedactionStateSchema,
    failureReason: Schema.Union(SessionTokenFailureReasonSchema, Schema.Null),
  })
);

export type SessionCredentialKind = Infer<typeof SessionCredentialKindSchema>;
export type SessionLifecycleAction = Infer<typeof SessionLifecycleActionSchema>;
export type SessionCredentialIssuanceAction = Infer<typeof SessionCredentialIssuanceActionSchema>;
export type SessionActivityState = Infer<typeof SessionActivityStateSchema>;
export type TokenReplayState = Infer<typeof TokenReplayStateSchema>;
export type TokenValidityWindowState = Infer<typeof TokenValidityWindowStateSchema>;
export type SessionTokenAuthorizationState = Infer<typeof SessionTokenAuthorizationStateSchema>;
export type SessionCredentialIssuanceState = Infer<typeof SessionCredentialIssuanceStateSchema>;
export type TokenAuditRedactionState = Infer<typeof TokenAuditRedactionStateSchema>;
export type SessionTokenFailureReason = Infer<typeof SessionTokenFailureReasonSchema>;
export type SessionTokenInput = Infer<typeof SessionTokenInputSchema>;
export type SessionCredentialIssuanceInput = Infer<typeof SessionCredentialIssuanceInputSchema>;
export type SessionTokenDecision = Infer<typeof SessionTokenDecisionSchema>;
export type SessionCredentialIssuanceDecision = Infer<typeof SessionCredentialIssuanceDecisionSchema>;

export const SessionCredentialKind = {
  BrowserUserSession: SessionCredentialKindSchema.parse(SessionCredentialKindLiteral.BrowserUserSession),
  DeviceCredential: SessionCredentialKindSchema.parse(SessionCredentialKindLiteral.DeviceCredential),
  InviteToken: SessionCredentialKindSchema.parse(SessionCredentialKindLiteral.InviteToken),
  PairingToken: SessionCredentialKindSchema.parse(SessionCredentialKindLiteral.PairingToken),
  RecoveryToken: SessionCredentialKindSchema.parse(SessionCredentialKindLiteral.RecoveryToken),
  RemoteSessionGrant: SessionCredentialKindSchema.parse(SessionCredentialKindLiteral.RemoteSessionGrant),
} as const;

export const SessionLifecycleAction = {
  RefreshBrowserSession: SessionLifecycleActionSchema.parse(SessionLifecycleActionLiteral.RefreshBrowserSession),
  PerformPrivilegedUserAction: SessionLifecycleActionSchema.parse(
    SessionLifecycleActionLiteral.PerformPrivilegedUserAction
  ),
  LogoutBrowserSession: SessionLifecycleActionSchema.parse(SessionLifecycleActionLiteral.LogoutBrowserSession),
  UseDeviceCredential: SessionLifecycleActionSchema.parse(SessionLifecycleActionLiteral.UseDeviceCredential),
  AcceptInviteToken: SessionLifecycleActionSchema.parse(SessionLifecycleActionLiteral.AcceptInviteToken),
  AcceptPairingToken: SessionLifecycleActionSchema.parse(SessionLifecycleActionLiteral.AcceptPairingToken),
  RedeemRecoveryToken: SessionLifecycleActionSchema.parse(SessionLifecycleActionLiteral.RedeemRecoveryToken),
  UseRemoteSessionGrant: SessionLifecycleActionSchema.parse(SessionLifecycleActionLiteral.UseRemoteSessionGrant),
} as const;

export const SessionCredentialIssuanceAction = {
  CreateBrowserSession: SessionCredentialIssuanceActionSchema.parse(
    SessionCredentialIssuanceActionLiteral.CreateBrowserSession
  ),
  RotateBrowserSession: SessionCredentialIssuanceActionSchema.parse(
    SessionCredentialIssuanceActionLiteral.RotateBrowserSession
  ),
  IssueDeviceCredential: SessionCredentialIssuanceActionSchema.parse(
    SessionCredentialIssuanceActionLiteral.IssueDeviceCredential
  ),
  IssueInviteToken: SessionCredentialIssuanceActionSchema.parse(
    SessionCredentialIssuanceActionLiteral.IssueInviteToken
  ),
  IssuePairingToken: SessionCredentialIssuanceActionSchema.parse(
    SessionCredentialIssuanceActionLiteral.IssuePairingToken
  ),
  IssueRecoveryToken: SessionCredentialIssuanceActionSchema.parse(
    SessionCredentialIssuanceActionLiteral.IssueRecoveryToken
  ),
  IssueRemoteSessionGrant: SessionCredentialIssuanceActionSchema.parse(
    SessionCredentialIssuanceActionLiteral.IssueRemoteSessionGrant
  ),
} as const;

export const SessionActivityState = {
  Active: SessionActivityStateSchema.parse(SessionActivityStateLiteral.Active),
  LoggedOut: SessionActivityStateSchema.parse(SessionActivityStateLiteral.LoggedOut),
  Revoked: SessionActivityStateSchema.parse(SessionActivityStateLiteral.Revoked),
  GloballyRevoked: SessionActivityStateSchema.parse(SessionActivityStateLiteral.GloballyRevoked),
} as const;

export const TokenReplayState = {
  Fresh: TokenReplayStateSchema.parse(TokenReplayStateLiteral.Fresh),
  ReplayDetected: TokenReplayStateSchema.parse(TokenReplayStateLiteral.ReplayDetected),
} as const;

export const TokenValidityWindowState = {
  Valid: TokenValidityWindowStateSchema.parse(TokenValidityWindowStateLiteral.Valid),
  ValidWithinClockSkewTolerance: TokenValidityWindowStateSchema.parse(
    TokenValidityWindowStateLiteral.ValidWithinClockSkewTolerance
  ),
  Expired: TokenValidityWindowStateSchema.parse(TokenValidityWindowStateLiteral.Expired),
  NotYetValid: TokenValidityWindowStateSchema.parse(TokenValidityWindowStateLiteral.NotYetValid),
} as const;

export const SessionTokenAuthorizationState = {
  Authorized: SessionTokenAuthorizationStateSchema.parse(SessionTokenAuthorizationStateLiteral.Authorized),
  Rejected: SessionTokenAuthorizationStateSchema.parse(SessionTokenAuthorizationStateLiteral.Rejected),
} as const;

export const SessionCredentialIssuanceState = {
  Created: SessionCredentialIssuanceStateSchema.parse(SessionCredentialIssuanceStateLiteral.Created),
  Rotated: SessionCredentialIssuanceStateSchema.parse(SessionCredentialIssuanceStateLiteral.Rotated),
  Issued: SessionCredentialIssuanceStateSchema.parse(SessionCredentialIssuanceStateLiteral.Issued),
  Rejected: SessionCredentialIssuanceStateSchema.parse(SessionCredentialIssuanceStateLiteral.Rejected),
} as const;

export const TokenAuditRedactionState = {
  Redacted: TokenAuditRedactionStateSchema.parse(TokenAuditRedactionStateLiteral.Redacted),
} as const;

export const SessionTokenFailureReason = {
  TokenExpired: SessionTokenFailureReasonSchema.parse(SessionTokenFailureReasonLiteral.TokenExpired),
  TokenNotYetValid: SessionTokenFailureReasonSchema.parse(SessionTokenFailureReasonLiteral.TokenNotYetValid),
  TokenReplayRejected: SessionTokenFailureReasonSchema.parse(SessionTokenFailureReasonLiteral.TokenReplayRejected),
  SessionLoggedOut: SessionTokenFailureReasonSchema.parse(SessionTokenFailureReasonLiteral.SessionLoggedOut),
  SessionRevoked: SessionTokenFailureReasonSchema.parse(SessionTokenFailureReasonLiteral.SessionRevoked),
  SessionGloballyRevoked: SessionTokenFailureReasonSchema.parse(
    SessionTokenFailureReasonLiteral.SessionGloballyRevoked
  ),
  SessionNotFresh: SessionTokenFailureReasonSchema.parse(SessionTokenFailureReasonLiteral.SessionNotFresh),
  WrongCredentialKind: SessionTokenFailureReasonSchema.parse(SessionTokenFailureReasonLiteral.WrongCredentialKind),
} as const;

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
