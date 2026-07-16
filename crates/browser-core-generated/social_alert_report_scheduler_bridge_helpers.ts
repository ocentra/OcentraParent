/* generated support for crates/browser-core/src/social_alert_report_scheduler_bridge.rs */

import {
  SocialAlertReportLocalOutboxBridgeStatus,
  type SocialAlertReportLocalOutboxBridgeRow,
} from './social-alert-report-local-outbox-bridge';

type SchedulerBridgeStatus =
  | 'scheduled-local-proof-row'
  | 'not-scheduled-manual-required'
  | 'not-scheduled-unavailable';

type SchedulerBridgeRowHonesty = {
  readonly status: SchedulerBridgeStatus;
  readonly sourceOutboxRecordRef: string | null;
  readonly schedulerRecord: { readonly schedulerEntryId: string } | null;
  readonly blockedReasonRefs: ReadonlyArray<unknown>;
};

type SchedulerBridgeReadModelCounts = {
  readonly rows: ReadonlyArray<{ readonly status: SchedulerBridgeStatus }>;
  readonly scheduledRecordCount: number;
  readonly unscheduledManualRequiredCount: number;
  readonly unscheduledUnavailableCount: number;
  readonly schedulerNonClaims: ReadonlyArray<string>;
};

export function schedulerStatusForOutboxBridgeRow(row: SocialAlertReportLocalOutboxBridgeRow): SchedulerBridgeStatus {
  return row.status === SocialAlertReportLocalOutboxBridgeStatus.Linked
    ? 'scheduled-local-proof-row'
    : row.status === SocialAlertReportLocalOutboxBridgeStatus.Unavailable
      ? 'not-scheduled-unavailable'
      : 'not-scheduled-manual-required';
}

export function socialAlertReportSchedulerBridgeRowIsHonest(row: SchedulerBridgeRowHonesty): boolean {
  if (row.status === 'scheduled-local-proof-row') {
    return row.schedulerRecord !== null && row.sourceOutboxRecordRef !== null && row.blockedReasonRefs.length === 0;
  }
  return row.schedulerRecord === null && row.sourceOutboxRecordRef === null && row.blockedReasonRefs.length > 0;
}

export function socialAlertReportSchedulerBridgeReadModelCountsMatch(
  readModel: SchedulerBridgeReadModelCounts,
  requiredNonClaims: ReadonlyArray<string>
): boolean {
  return (
    readModel.scheduledRecordCount === countRows(readModel.rows, 'scheduled-local-proof-row') &&
    readModel.unscheduledManualRequiredCount === countRows(readModel.rows, 'not-scheduled-manual-required') &&
    readModel.unscheduledUnavailableCount === countRows(readModel.rows, 'not-scheduled-unavailable') &&
    requiredNonClaims.every((claim) => readModel.schedulerNonClaims.includes(claim))
  );
}

export function countRows(
  rows: ReadonlyArray<{ readonly status: SchedulerBridgeStatus }>,
  status: SchedulerBridgeStatus
): number {
  return rows.filter((row) => row.status === status).length;
}
