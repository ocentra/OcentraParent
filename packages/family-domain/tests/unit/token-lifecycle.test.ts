import { describe, expect, it } from 'vitest';
import {
  authorizeSessionTokenAction,
} from '../../src/session-lifecycle';
import {
  SessionActivityState,
  SessionCredentialKind,
  SessionLifecycleAction,
  SessionTokenDecisionSchema,
  SessionTokenFailureReason,
  SessionTokenInputSchema,
  TokenAuditRedactionState,
  TokenReplayState,
  TokenValidityWindowState,
} from '@ocentra-parent/schema-domain/family-session-lifecycle';
import {
  AuditRequirementState,
  SessionFreshnessState,
} from '@ocentra-parent/schema-domain/family-household-authority';

describe('session token lifecycle contracts', () => {
  it('rejects replayed and invalid token windows', () => {
    expect(
      authorizeSessionTokenAction({
        ...activeBrowserToken(SessionLifecycleAction.PerformPrivilegedUserAction),
        replayState: TokenReplayState.ReplayDetected,
      }).failureReason
    ).toBe(SessionTokenFailureReason.TokenReplayRejected);

    expect(
      authorizeSessionTokenAction({
        ...activeBrowserToken(SessionLifecycleAction.PerformPrivilegedUserAction),
        validityWindowState: TokenValidityWindowState.Expired,
      }).failureReason
    ).toBe(SessionTokenFailureReason.TokenExpired);

    expect(
      authorizeSessionTokenAction({
        ...activeBrowserToken(SessionLifecycleAction.PerformPrivilegedUserAction),
        validityWindowState: TokenValidityWindowState.NotYetValid,
      }).failureReason
    ).toBe(SessionTokenFailureReason.TokenNotYetValid);

    expect(
      authorizeSessionTokenAction({
        ...activeBrowserToken(SessionLifecycleAction.PerformPrivilegedUserAction),
        validityWindowState: TokenValidityWindowState.ValidWithinClockSkewTolerance,
      }).authorizationState
    ).toBe('authorized');
  });

  it('keeps token decisions redacted for privileged browser actions', () => {
    const decision = authorizeSessionTokenAction(
      activeBrowserToken(SessionLifecycleAction.PerformPrivilegedUserAction)
    );

    expect(decision).toEqual(
      SessionTokenDecisionSchema.parse({
        authorizationState: 'authorized',
        auditRequirementState: AuditRequirementState.Required,
        auditRedactionState: TokenAuditRedactionState.Redacted,
        failureReason: null,
      })
    );
  });
});

function activeBrowserToken(
  action: typeof SessionLifecycleAction[keyof typeof SessionLifecycleAction]
) {
  return SessionTokenInputSchema.parse({
    credentialKind: SessionCredentialKind.BrowserUserSession,
    action,
    activityState: SessionActivityState.Active,
    replayState: TokenReplayState.Fresh,
    validityWindowState: TokenValidityWindowState.Valid,
    sessionFreshnessState: SessionFreshnessState.Fresh,
  });
}
