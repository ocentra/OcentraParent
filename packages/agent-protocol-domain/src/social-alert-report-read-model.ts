import {
  SocialAlertReportReadModelSnapshotSchema,
  type SocialAlertReportReadModelIntent,
  type SocialAlertReportReadModelSnapshot,
} from '@ocentra-parent/schema-domain/agent-social-alert-report-read-model';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export { SocialAlertReportReadModelSnapshotSchema };
export type { SocialAlertReportReadModelIntent, SocialAlertReportReadModelSnapshot };

export type AgentSocialAlertReportReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialAlertReportReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialAlertReportReadModelSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialAlertReportReadModelFailureReason;
    };

export function parseAgentSocialAlertReportReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialAlertReportReadModelResult {
  if (event.event !== AgentEvent.BrowserSocialAlertReportReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = SocialAlertReportReadModelSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentSocialAlertReportReadModelFailureReason): AgentSocialAlertReportReadModelResult {
  return {
    ok: false,
    reason,
  };
}
