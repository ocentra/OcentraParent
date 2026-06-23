import {
  SocialAlertReportParentSurfaceReadModelSnapshotSchema,
  type SocialAlertReportParentSurfaceReadModelRow,
  type SocialAlertReportParentSurfaceReadModelSnapshot,
} from '@ocentra-parent/schema-domain/agent-social-alert-report-parent-surface-read-model';
import {
  AgentEvent,
  isAgentProtocolLogText,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

export { SocialAlertReportParentSurfaceReadModelSnapshotSchema };
export type { SocialAlertReportParentSurfaceReadModelRow, SocialAlertReportParentSurfaceReadModelSnapshot };

export type AgentSocialAlertReportParentSurfaceReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentSocialAlertReportParentSurfaceReadModelResult =
  | {
      readonly ok: true;
      readonly value: SocialAlertReportParentSurfaceReadModelSnapshot;
    }
  | {
      readonly ok: false;
      readonly reason: AgentSocialAlertReportParentSurfaceReadModelFailureReason;
    };

export function parseAgentSocialAlertReportParentSurfaceReadModelEvent(
  event: AgentEventEnvelope
): AgentSocialAlertReportParentSurfaceReadModelResult {
  if (event.event !== AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported) {
    return failure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return failure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return failure('invalid-json');
  }

  const parsed = SocialAlertReportParentSurfaceReadModelSnapshotSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return failure('invalid-payload');
  }

  return { ok: true, value: parsed.data };
}

function failure(reason: AgentSocialAlertReportParentSurfaceReadModelFailureReason) {
  return { ok: false, reason } as const;
}
