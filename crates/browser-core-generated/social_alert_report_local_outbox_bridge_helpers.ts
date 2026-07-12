/* generated support for crates/browser-core/src/social_alert_report_local_outbox_bridge.rs */

import { V3NotificationRuleReasonCodeSchema } from '@ocentra-parent/schema-domain/notification-v3-provider-retry';
import {
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentStatus,
  type SocialAlertReportIntent,
} from './social_alert_report_local_outbox_bridge_support';

export function bridgeStatusForIntent(
  intent: SocialAlertReportIntent
): 'linked-local-outbox-record' | 'manual-required' | 'unavailable' {
  if (intent.intentStatus === SocialAlertReportIntentStatus.LocalOutboxEligible) {
    return 'linked-local-outbox-record';
  }
  if (intent.intentStatus === SocialAlertReportIntentStatus.Unavailable) {
    return 'unavailable';
  }
  return 'manual-required';
}

export function socialAlertReportReasonToProviderReason(reasonCode: string) {
  if (reasonCode === 'social-high-risk-signal' || reasonCode === 'social-feed-video-gate') {
    return V3NotificationRuleReasonCodeSchema.parse('policy-violation');
  }
  if (reasonCode === 'social-capability-unavailable') {
    return V3NotificationRuleReasonCodeSchema.parse('provider-failure');
  }
  return V3NotificationRuleReasonCodeSchema.parse('parent-request');
}

type BridgeHonestyRow = {
  readonly status: 'linked-local-outbox-record' | 'manual-required' | 'unavailable';
  readonly intent: SocialAlertReportIntent;
  readonly outboxRecord: { readonly entryId: string } | null;
  readonly blockedReasonRefs: ReadonlyArray<unknown>;
};

export function socialAlertReportBridgeRowIsHonest(row: BridgeHonestyRow): boolean {
  if (row.status === 'linked-local-outbox-record') {
    return (
      row.intent.intentStatus === SocialAlertReportIntentStatus.LocalOutboxEligible &&
      row.intent.deliveryClaimState === SocialAlertReportDeliveryClaimState.LocalOutboxOnly &&
      row.outboxRecord !== null &&
      String(row.outboxRecord.entryId) === String(row.intent.localOutboxRecordRef) &&
      row.blockedReasonRefs.length === 0
    );
  }
  return row.outboxRecord === null && row.blockedReasonRefs.length > 0;
}

export function socialAlertReportBridgeReadModelCountsMatch(readModel: {
  readonly rows: ReadonlyArray<{ readonly status: 'linked-local-outbox-record' | 'manual-required' | 'unavailable' }>;
  readonly linkedRecordCount: number;
  readonly manualRequiredCount: number;
  readonly unavailableCount: number;
}): boolean {
  return (
    readModel.linkedRecordCount === countRows(readModel.rows, 'linked-local-outbox-record') &&
    readModel.manualRequiredCount === countRows(readModel.rows, 'manual-required') &&
    readModel.unavailableCount === countRows(readModel.rows, 'unavailable')
  );
}

export function countRows(
  rows: ReadonlyArray<{ readonly status: 'linked-local-outbox-record' | 'manual-required' | 'unavailable' }>,
  status: 'linked-local-outbox-record' | 'manual-required' | 'unavailable'
): number {
  return rows.filter((row) => row.status === status).length;
}
