/* generated support for crates/browser-core/src/social_alert_report_provider_dispatch_execution.rs */

import type { SocialAlertReportProviderReceiptBoundaryRow } from './social_alert_report_provider_receipt_boundary_support';
import type { NotificationLocalOutboxRecord } from './social_alert_report_local_outbox_bridge_support';
import type {
  SocialAlertReportProviderDispatchExecutionReadModel,
  SocialAlertReportProviderDispatchExecutionRow,
  SocialAlertReportProviderDispatchExecutionState,
} from './social_alert_report_provider_dispatch_execution';
import { socialAlertReportProviderDispatchExecutionRowIsHonest as socialAlertReportProviderDispatchExecutionRowIsHonestFromRowHonesty } from './social_alert_report_provider_dispatch_execution_row_honesty';

export function localOutboxRecordForReceiptRow(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  localOutboxRecords: ReadonlyArray<NotificationLocalOutboxRecord>
): NotificationLocalOutboxRecord | null {
  if (row.sourceLocalOutboxRecordRef === null) {
    return null;
  }

  return localOutboxRecords.find((record) => String(record.entryId) === String(row.sourceLocalOutboxRecordRef)) ?? null;
}

export function dispatchExecutionStateFor(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  outboxRecord: NotificationLocalOutboxRecord | null
): SocialAlertReportProviderDispatchExecutionState {
  if (row.receiptBoundaryState === 'provider-unavailable') {
    return 'provider-unavailable';
  }
  if (row.receiptBoundaryState !== 'provider-dispatch-required' || outboxRecord === null) {
    return 'manual-required';
  }
  return 'local-dispatch-packet-ready';
}

export function manualProofRequirementsFor(
  row: SocialAlertReportProviderReceiptBoundaryRow,
  state: SocialAlertReportProviderDispatchExecutionState
): readonly string[] {
  if (state === 'local-dispatch-packet-ready') {
    return [];
  }
  if (state === 'provider-unavailable') {
    return [`social-provider-dispatch-provider-unavailable-${row.sourceIntentRef}`];
  }
  if (row.receiptBoundaryState === 'provider-dispatch-required') {
    return [`social-provider-dispatch-local-outbox-record-required-${row.sourceIntentRef}`];
  }
  return row.manualProofRequirements;
}

export function socialAlertReportProviderDispatchExecutionRowIsHonest(
  row: SocialAlertReportProviderDispatchExecutionRow
): boolean {
  return socialAlertReportProviderDispatchExecutionRowIsHonestFromRowHonesty(row);
}

export function socialAlertReportProviderDispatchExecutionReadModelIsHonest(
  readModel: SocialAlertReportProviderDispatchExecutionReadModel,
  requiredSourceNonClaims: readonly string[],
  requiredDispatchNonClaims: readonly string[]
): boolean {
  const sourceNonClaims: readonly string[] = readModel.sourceReceiptBoundaryNonClaims;

  return (
    readModel.localDispatchPacketReadyCount === countRows(readModel.rows, 'local-dispatch-packet-ready') &&
    readModel.manualRequiredCount === countRows(readModel.rows, 'manual-required') &&
    readModel.providerUnavailableCount === countRows(readModel.rows, 'provider-unavailable') &&
    requiredSourceNonClaims.every((claim) => sourceNonClaims.includes(claim)) &&
    requiredDispatchNonClaims.every((claim) => readModel.dispatchExecutionNonClaims.includes(claim)) &&
    readModel.providerDeliveryAttempted === false &&
    readModel.providerDeliveryObserved === false &&
    readModel.providerReceiptIngested === false &&
    readModel.enforcementClaimed === false
  );
}

export function countRows(
  rows: ReadonlyArray<{ readonly dispatchExecutionState: SocialAlertReportProviderDispatchExecutionState }>,
  state: SocialAlertReportProviderDispatchExecutionState
): number {
  return rows.filter((row) => row.dispatchExecutionState === state).length;
}
