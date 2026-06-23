import {
  SocialDashboardUxSnapshotSchema,
  type SocialDashboardUxSnapshot,
} from '@ocentra-parent/schema-domain/social-dashboard-ux';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export type AgentSocialDashboardReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialDashboardReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialDashboardUxSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialDashboardReadModelFailureReason;
    };

export function parseAgentSocialDashboardReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialDashboardReadModelResult {
  if (event.event !== AgentEvent.BrowserSocialDashboardReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialDashboardReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = SocialDashboardUxSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentSocialDashboardReadModelFailureReason): AgentSocialDashboardReadModelResult {
  return {
    ok: false,
    reason,
  };
}
