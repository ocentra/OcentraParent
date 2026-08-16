/* generated support for crates/browser-core/src/social_alert_report_provider_dispatch_execution.rs */

import type { SocialAlertReportProviderDispatchExecutionRow } from './social_alert_report_provider_dispatch_execution';

export function socialAlertReportProviderDispatchExecutionRowIsHonest(
  row: SocialAlertReportProviderDispatchExecutionRow
): boolean {
  if (row.dispatchExecutionState === 'local-dispatch-packet-ready') {
    return dispatchPacketReadyRowIsHonest(row);
  }
  if (row.dispatchExecutionState === 'provider-unavailable') {
    return providerUnavailableRowIsHonest(row);
  }
  return manualRequiredRowIsHonest(row);
}

function dispatchPacketReadyRowIsHonest(row: SocialAlertReportProviderDispatchExecutionRow): boolean {
  return (
    row.sourceReceiptBoundaryState === 'provider-dispatch-required' &&
    row.sourceLocalOutboxRecordRef !== null &&
    row.dispatchPacket !== null &&
    String(row.dispatchPacket.outboxEntryRef) === String(row.sourceLocalOutboxRecordRef) &&
    String(row.dispatchPacket.providerAttemptRef) === String(row.sourceProviderAttemptRef) &&
    row.manualProofRequirements.length === 0 &&
    providerDispatchClaimsStayFalse(row)
  );
}

function manualRequiredRowIsHonest(row: SocialAlertReportProviderDispatchExecutionRow): boolean {
  return row.dispatchPacket === null && row.manualProofRequirements.length > 0 && providerDispatchClaimsStayFalse(row);
}

function providerUnavailableRowIsHonest(row: SocialAlertReportProviderDispatchExecutionRow): boolean {
  return (
    row.sourceReceiptBoundaryState === 'provider-unavailable' &&
    row.dispatchPacket === null &&
    row.manualProofRequirements.length > 0 &&
    providerDispatchClaimsStayFalse(row)
  );
}

function providerDispatchClaimsStayFalse(row: SocialAlertReportProviderDispatchExecutionRow): boolean {
  return [
    row.providerDeliveryAttempted,
    row.providerDeliveryObserved,
    row.providerReceiptIngested,
    row.providerWebhookRuntimeClaimed,
    row.providerCredentialsClaimed,
    row.cloudRoutingClaimed,
    row.parentNotificationUiDeliveryClaimed,
    row.reportDeliveryExecutionClaimed,
    row.finalPolicyExecutionClaimed,
    row.connectorNativeRuntimeClaimed,
    row.enforcementClaimed,
  ].every((claim) => claim === false);
}
