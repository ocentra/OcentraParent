import { describe, expect, it } from 'vitest';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  RemoteAccessConsentState,
  RemoteAccessDecisionState,
  RemoteAccessSessionRequestSchema,
  RemoteAccessTransportMode,
  decideRemoteAccessSession,
} from '@ocentra-parent/schema-domain/remote-access-session';

const Request = RemoteAccessSessionRequestSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  family: {
    familyId: 'family-local-1',
  },
  requestedBy: {
    actorId: 'parent-actor-1',
    role: ParentActorRole.Parent,
  },
  targetDevice: {
    deviceId: 'child-device-1',
    childProfileId: 'child-profile-1',
    label: 'Kitchen laptop',
    platform: ParentPlatform.Windows,
  },
  requestId: 'remote-access-request-1',
  sessionId: 'remote-access-session-1',
  purpose: 'parent-support-session',
  requestedAt: '2026-06-01T00:00:00Z',
  consentState: RemoteAccessConsentState.ChildConsented,
  transportMode: RemoteAccessTransportMode.LanDirect,
} as const);

describe('remote access session contracts', () => {
  it('allows only child-consented sessions over an enabled transport', () => {
    const decision = decideRemoteAccessSession(Request);

    expect(decision.decisionState).toBe(RemoteAccessDecisionState.Allowed);
  });

  it('blocks sessions with disabled transport', () => {
    const decision = decideRemoteAccessSession({
      ...Request,
      transportMode: RemoteAccessTransportMode.Disabled,
    });

    expect(decision.decisionState).toBe(RemoteAccessDecisionState.Blocked);
  });

  it('rejects missing purpose values', () => {
    expect(
      RemoteAccessSessionRequestSchema.safeParse({
        ...Request,
        purpose: '',
      }).success
    ).toBe(false);
  });
});
