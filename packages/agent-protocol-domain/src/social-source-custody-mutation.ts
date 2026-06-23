import {
  SocialSourceCustodyMutationSnapshotSchema,
  type SocialSourceCustodyMutationSnapshot,
} from '@ocentra-parent/schema-domain/agent-social-source-custody-mutation';
import { type SocialVideoSourceCustodySettings } from '@ocentra-parent/schema-domain/agent-social-video-source-custody-settings';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export { SocialSourceCustodyMutationSnapshotSchema };
export type { SocialSourceCustodyMutationSnapshot };
export type SocialSourceCustodyMutationSettings = SocialVideoSourceCustodySettings;

export type AgentSocialSourceCustodyMutationFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialSourceCustodyMutationResult =
  | {
      readonly ok: true;
      readonly value: SocialSourceCustodyMutationSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialSourceCustodyMutationFailureReason;
    };

export function parseAgentSocialSourceCustodyMutationEvent(
  event: AgentEventEnvelope
): AgentSocialSourceCustodyMutationResult {
  if (event.event !== AgentEvent.BrowserSocialSourceCustodyMutationApplied) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialSourceCustodyMutation];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = SocialSourceCustodyMutationSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentSocialSourceCustodyMutationFailureReason): AgentSocialSourceCustodyMutationResult {
  return {
    ok: false,
    reason,
  };
}
