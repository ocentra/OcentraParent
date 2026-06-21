import { describe, expect, it } from 'vitest';
import {
  authorizeSessionCredentialIssuance,
  authorizeSessionTokenAction,
} from '../../src/session-lifecycle';
import {
  SessionActivityState,
  SessionActivityStateSchema,
  SessionCredentialKind,
  SessionCredentialIssuanceAction,
  SessionCredentialIssuanceState,
  SessionCredentialKindSchema,
  SessionLifecycleAction,
  SessionLifecycleActionSchema,
  SessionTokenAuthorizationState,
  SessionCredentialIssuanceDecisionSchema,
  SessionTokenDecisionSchema,
  SessionTokenFailureReason,
  SessionCredentialIssuanceInputSchema,
  SessionTokenInputSchema,
  TokenAuditRedactionState,
  TokenReplayState,
  TokenValidityWindowState,
} from '@ocentra-parent/schema-domain/family-session-lifecycle';
import {
  AuditRequirementState,
  SessionFreshnessState,
} from '@ocentra-parent/schema-domain/family-household-authority';

describe('family session lifecycle contracts', () => {
  registerSessionLifecycleParsingTests();
  registerSessionCredentialScopeTests();
  registerSessionBoundaryTests();
  registerSessionStateTests();
  registerSessionFreshnessTests();
  registerSessionSchemaBoundaryTests();
});

function activeSessionToken(
  credentialKind: typeof SessionCredentialKind[keyof typeof SessionCredentialKind],
  action: typeof SessionLifecycleAction[keyof typeof SessionLifecycleAction]
) {
  return SessionTokenInputSchema.parse({
    credentialKind,
    action,
    activityState: SessionActivityState.Active,
    replayState: TokenReplayState.Fresh,
    validityWindowState: TokenValidityWindowState.Valid,
    sessionFreshnessState: SessionFreshnessState.Fresh,
  });
}

function issuanceInput(
  issuanceAction: typeof SessionCredentialIssuanceAction[keyof typeof SessionCredentialIssuanceAction],
  issuedCredentialKind: typeof SessionCredentialKind[keyof typeof SessionCredentialKind],
  sourceSession: ReturnType<typeof activeSessionToken> | null
) {
  return SessionCredentialIssuanceInputSchema.parse({
    issuanceAction,
    issuedCredentialKind,
    sourceSession,
  });
}

function registerSessionLifecycleParsingTests(): void {
  it('parses session token input and decision contracts exactly', () => {
    expect(
      SessionTokenInputSchema.parse({
        credentialKind: 'browser-user-session',
        action: 'perform-privileged-user-action',
        activityState: 'active',
        replayState: 'fresh',
        validityWindowState: 'valid-within-clock-skew-tolerance',
        sessionFreshnessState: 'fresh',
      })
    ).toEqual({
      credentialKind: 'browser-user-session',
      action: 'perform-privileged-user-action',
      activityState: 'active',
      replayState: 'fresh',
      validityWindowState: 'valid-within-clock-skew-tolerance',
      sessionFreshnessState: 'fresh',
    });

    expect(
      SessionTokenDecisionSchema.parse({
        authorizationState: 'rejected',
        auditRequirementState: 'required',
        auditRedactionState: 'redacted',
        failureReason: 'session-globally-revoked',
      })
    ).toEqual({
      authorizationState: 'rejected',
      auditRequirementState: 'required',
      auditRedactionState: 'redacted',
      failureReason: 'session-globally-revoked',
    });

    expect(
      SessionCredentialIssuanceDecisionSchema.parse({
        issuanceState: 'rotated',
        auditRequirementState: 'required',
        auditRedactionState: 'redacted',
        failureReason: null,
      })
    ).toEqual({
      issuanceState: 'rotated',
      auditRequirementState: 'required',
      auditRedactionState: 'redacted',
      failureReason: null,
    });
  });
}

function registerSessionCredentialScopeTests(): void {
  it('authorizeSessionTokenAction keeps credential kinds scoped to their matching actions', () => {
    expectAuthorized(
      authorizeSessionTokenAction(
        activeSessionToken(SessionCredentialKind.BrowserUserSession, SessionLifecycleAction.RefreshBrowserSession)
      ),
      AuditRequirementState.NotRequired
    );

    expectRejected(
      authorizeSessionTokenAction(
        activeSessionToken(SessionCredentialKind.InviteToken, SessionLifecycleAction.PerformPrivilegedUserAction)
      ),
      AuditRequirementState.Required,
      SessionTokenFailureReason.WrongCredentialKind
    );

    expectRejected(
      authorizeSessionTokenAction(
        activeSessionToken(SessionCredentialKind.RecoveryToken, SessionLifecycleAction.RefreshBrowserSession)
      ),
      AuditRequirementState.NotRequired,
      SessionTokenFailureReason.WrongCredentialKind
    );

    expectAuthorized(
      authorizeSessionTokenAction(
        activeSessionToken(SessionCredentialKind.RemoteSessionGrant, SessionLifecycleAction.UseRemoteSessionGrant)
      ),
      AuditRequirementState.Required
    );
  });

  it('keeps issuance kinds separated across browser sessions, device credentials, invite tokens, recovery tokens, and remote grants', () => {
    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.CreateBrowserSession,
          SessionCredentialKind.BrowserUserSession,
          null
        )
      ),
      SessionCredentialIssuanceState.Created
    );

    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.IssueDeviceCredential,
          SessionCredentialKind.DeviceCredential,
          activeSessionToken(
            SessionCredentialKind.BrowserUserSession,
            SessionLifecycleAction.PerformPrivilegedUserAction
          )
        )
      ),
      SessionCredentialIssuanceState.Issued
    );

    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.IssueInviteToken,
          SessionCredentialKind.InviteToken,
          activeSessionToken(
            SessionCredentialKind.BrowserUserSession,
            SessionLifecycleAction.PerformPrivilegedUserAction
          )
        )
      ),
      SessionCredentialIssuanceState.Issued
    );

    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.IssueRecoveryToken,
          SessionCredentialKind.RecoveryToken,
          activeSessionToken(
            SessionCredentialKind.BrowserUserSession,
            SessionLifecycleAction.PerformPrivilegedUserAction
          )
        )
      ),
      SessionCredentialIssuanceState.Issued
    );

    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.IssuePairingToken,
          SessionCredentialKind.PairingToken,
          activeSessionToken(
            SessionCredentialKind.BrowserUserSession,
            SessionLifecycleAction.PerformPrivilegedUserAction
          )
        )
      ),
      SessionCredentialIssuanceState.Issued
    );

    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.IssueRemoteSessionGrant,
          SessionCredentialKind.RemoteSessionGrant,
          activeSessionToken(
            SessionCredentialKind.BrowserUserSession,
            SessionLifecycleAction.PerformPrivilegedUserAction
          )
        )
      ),
      SessionCredentialIssuanceState.Issued
    );

    expectIssuanceRejected(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.IssueInviteToken,
          SessionCredentialKind.BrowserUserSession,
          activeSessionToken(
            SessionCredentialKind.BrowserUserSession,
            SessionLifecycleAction.PerformPrivilegedUserAction
          )
        )
      ),
      SessionTokenFailureReason.WrongCredentialKind
    );

    expectIssuanceRejected(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.IssueDeviceCredential,
          SessionCredentialKind.DeviceCredential,
          null
        )
      ),
      SessionTokenFailureReason.SessionLoggedOut
    );
  });
}

function registerSessionBoundaryTests(): void {
  it('authorizeSessionTokenAction enforces replay, expiry, and not-yet-valid boundaries', () => {
    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(SessionCredentialKind.PairingToken, SessionLifecycleAction.AcceptPairingToken),
        replayState: TokenReplayState.ReplayDetected,
      }),
      AuditRequirementState.Required,
      SessionTokenFailureReason.TokenReplayRejected
    );

    expectAuthorized(
      authorizeSessionTokenAction({
        ...activeSessionToken(
          SessionCredentialKind.BrowserUserSession,
          SessionLifecycleAction.PerformPrivilegedUserAction
        ),
        validityWindowState: TokenValidityWindowState.ValidWithinClockSkewTolerance,
      }),
      AuditRequirementState.Required
    );

    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(
          SessionCredentialKind.BrowserUserSession,
          SessionLifecycleAction.PerformPrivilegedUserAction
        ),
        validityWindowState: TokenValidityWindowState.Expired,
      }),
      AuditRequirementState.Required,
      SessionTokenFailureReason.TokenExpired
    );

    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(
          SessionCredentialKind.BrowserUserSession,
          SessionLifecycleAction.PerformPrivilegedUserAction
        ),
        validityWindowState: TokenValidityWindowState.NotYetValid,
      }),
      AuditRequirementState.Required,
      SessionTokenFailureReason.TokenNotYetValid
    );
  });
}

function registerSessionStateTests(): void {
  it('authorizeSessionTokenAction treats logout, revocation, and global revoke as terminal states', () => {
    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(
          SessionCredentialKind.BrowserUserSession,
          SessionLifecycleAction.PerformPrivilegedUserAction
        ),
        activityState: SessionActivityState.LoggedOut,
      }),
      AuditRequirementState.Required,
      SessionTokenFailureReason.SessionLoggedOut
    );

    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(SessionCredentialKind.BrowserUserSession, SessionLifecycleAction.RefreshBrowserSession),
        activityState: SessionActivityState.Revoked,
      }),
      AuditRequirementState.NotRequired,
      SessionTokenFailureReason.SessionRevoked
    );

    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(
          SessionCredentialKind.BrowserUserSession,
          SessionLifecycleAction.PerformPrivilegedUserAction
        ),
        activityState: SessionActivityState.GloballyRevoked,
      }),
      AuditRequirementState.Required,
      SessionTokenFailureReason.SessionGloballyRevoked
    );
  });

  it('creates browser sessions and rotates refresh sessions only from active browser state', () => {
    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.CreateBrowserSession,
          SessionCredentialKind.BrowserUserSession,
          null
        )
      ),
      SessionCredentialIssuanceState.Created
    );

    expectIssuanceAuthorized(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.RotateBrowserSession,
          SessionCredentialKind.BrowserUserSession,
          activeSessionToken(
            SessionCredentialKind.BrowserUserSession,
            SessionLifecycleAction.RefreshBrowserSession
          )
        )
      ),
      SessionCredentialIssuanceState.Rotated
    );

    expectIssuanceRejected(
      authorizeSessionCredentialIssuance(
        issuanceInput(
          SessionCredentialIssuanceAction.RotateBrowserSession,
          SessionCredentialKind.BrowserUserSession,
          {
            ...activeSessionToken(
              SessionCredentialKind.BrowserUserSession,
              SessionLifecycleAction.RefreshBrowserSession
            ),
            activityState: SessionActivityState.Revoked,
          }
        )
      ),
      SessionTokenFailureReason.SessionRevoked
    );
  });
}

function registerSessionFreshnessTests(): void {
  it('authorizeSessionTokenAction requires fresh user context for privileged and remote actions', () => {
    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(
          SessionCredentialKind.BrowserUserSession,
          SessionLifecycleAction.PerformPrivilegedUserAction
        ),
        sessionFreshnessState: SessionFreshnessState.Stale,
      }),
      AuditRequirementState.Required,
      SessionTokenFailureReason.SessionNotFresh
    );

    expectRejected(
      authorizeSessionTokenAction({
        ...activeSessionToken(SessionCredentialKind.RemoteSessionGrant, SessionLifecycleAction.UseRemoteSessionGrant),
        sessionFreshnessState: SessionFreshnessState.Stale,
      }),
      AuditRequirementState.Required,
      SessionTokenFailureReason.SessionNotFresh
    );
  });
}

function registerSessionSchemaBoundaryTests(): void {
  it('schema boundary rejects unsupported session literals', () => {
    expect(SessionCredentialKindSchema.safeParse('bearer-token').success).toBe(false);
    expect(SessionCredentialKindSchema.safeParse('controller-lease').success).toBe(false);
    expect(SessionCredentialKindSchema.safeParse('support-admin-session').success).toBe(false);
    expect(SessionLifecycleActionSchema.safeParse('reset-password').success).toBe(false);
    expect(SessionActivityStateSchema.safeParse('paused').success).toBe(false);
  });
}

function expectAuthorized(
  decision: ReturnType<typeof authorizeSessionTokenAction>,
  auditRequirementState: typeof AuditRequirementState[keyof typeof AuditRequirementState]
): void {
  expect(decision).toEqual({
    authorizationState: SessionTokenAuthorizationState.Authorized,
    auditRequirementState,
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason: null,
  });
}

function expectRejected(
  decision: ReturnType<typeof authorizeSessionTokenAction>,
  auditRequirementState: typeof AuditRequirementState[keyof typeof AuditRequirementState],
  failureReason: typeof SessionTokenFailureReason[keyof typeof SessionTokenFailureReason]
): void {
  expect(decision).toEqual({
    authorizationState: SessionTokenAuthorizationState.Rejected,
    auditRequirementState,
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason,
  });
}

function expectIssuanceAuthorized(
  decision: ReturnType<typeof authorizeSessionCredentialIssuance>,
  issuanceState: typeof SessionCredentialIssuanceState[keyof typeof SessionCredentialIssuanceState]
): void {
  expect(decision).toEqual({
    issuanceState,
    auditRequirementState: AuditRequirementState.Required,
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason: null,
  });
}

function expectIssuanceRejected(
  decision: ReturnType<typeof authorizeSessionCredentialIssuance>,
  failureReason: typeof SessionTokenFailureReason[keyof typeof SessionTokenFailureReason]
): void {
  expect(decision).toEqual({
    issuanceState: SessionCredentialIssuanceState.Rejected,
    auditRequirementState: AuditRequirementState.Required,
    auditRedactionState: TokenAuditRedactionState.Redacted,
    failureReason,
  });
}
