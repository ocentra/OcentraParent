import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './text-contracts';

export const PortalDevTrackingReportTextToken = {
  TrackingReportExportHostedUi: decodeTextTokenId('portal.dev.trackingReportExportHostedUi'),
  TrackingReportExportHostedUiBody: decodeTextTokenId('portal.dev.trackingReportExportHostedUiBody'),
  TrackingReportExportRedactedReport: decodeTextTokenId('portal.dev.trackingReportExportRedactedReport'),
  TrackingReportExportRetentionAudit: decodeTextTokenId('portal.dev.trackingReportExportRetentionAudit'),
  TrackingReportExportFamilySummary: decodeTextTokenId('portal.dev.trackingReportExportFamilySummary'),
  TrackingReportExportPolicyDrillIn: decodeTextTokenId('portal.dev.trackingReportExportPolicyDrillIn'),
  TrackingReportExportReadModelReady: decodeTextTokenId('portal.dev.trackingReportExportReadModelReady'),
  TrackingReportExportRedactedCustody: decodeTextTokenId('portal.dev.trackingReportExportRedactedCustody'),
  TrackingReportExportLocalCustody: decodeTextTokenId('portal.dev.trackingReportExportLocalCustody'),
  TrackingReportExportRedactedReportEvidence: decodeTextTokenId(
    'portal.dev.trackingReportExportRedactedReportEvidence'
  ),
  TrackingReportExportRetentionAuditEvidence: decodeTextTokenId(
    'portal.dev.trackingReportExportRetentionAuditEvidence'
  ),
  TrackingReportExportFamilySummaryEvidence: decodeTextTokenId('portal.dev.trackingReportExportFamilySummaryEvidence'),
  TrackingReportExportPolicyDrillInEvidence: decodeTextTokenId('portal.dev.trackingReportExportPolicyDrillInEvidence'),
  TrackingReportExportHostedBoundary: decodeTextTokenId('portal.dev.trackingReportExportHostedBoundary'),
  TrackingReportPolicyConsumerHostedUi: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerHostedUi'),
  TrackingReportPolicyConsumerHostedUiBody: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerHostedUiBody'),
  TrackingReportPolicyConsumerParentReport: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerParentReport'),
  TrackingReportPolicyConsumerPolicyDrillIn: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerPolicyDrillIn'),
  TrackingReportPolicyConsumerRetentionAudit: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerRetentionAudit'
  ),
  TrackingReportPolicyConsumerReady: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerReady'),
  TrackingReportPolicyConsumerReportEvidence: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerReportEvidence'
  ),
  TrackingReportPolicyConsumerPolicyEvidence: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerPolicyEvidence'
  ),
  TrackingReportPolicyConsumerRetentionEvidence: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerRetentionEvidence'
  ),
  TrackingReportPolicyConsumerReportJournal: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerReportJournal'),
  TrackingReportPolicyConsumerPolicyJournal: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerPolicyJournal'),
  TrackingReportPolicyConsumerRetentionJournal: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerRetentionJournal'
  ),
  TrackingReportPolicyConsumerReportReadModel: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerReportReadModel'
  ),
  TrackingReportPolicyConsumerPolicyReadModel: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerPolicyReadModel'
  ),
  TrackingReportPolicyConsumerRetentionReadModel: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerRetentionReadModel'
  ),
  TrackingReportPolicyConsumerReportSurface: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerReportSurface'),
  TrackingReportPolicyConsumerPolicySurface: decodeTextTokenId('portal.dev.trackingReportPolicyConsumerPolicySurface'),
  TrackingReportPolicyConsumerRetentionSurface: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerRetentionSurface'
  ),
  TrackingReportPolicyConsumerHostedBoundary: decodeTextTokenId(
    'portal.dev.trackingReportPolicyConsumerHostedBoundary'
  ),
} as const;

export type PortalDevTrackingReportTextTokenValue =
  (typeof PortalDevTrackingReportTextToken)[keyof typeof PortalDevTrackingReportTextToken];

export const PortalDevTrackingReportText: Record<PortalDevTrackingReportTextTokenValue, DisplayText> = {
  [PortalDevTrackingReportTextToken.TrackingReportExportHostedUi]: decodeDisplayText('Report export read-model UI'),
  [PortalDevTrackingReportTextToken.TrackingReportExportHostedUiBody]: decodeDisplayText(
    'Hosted route renders redacted report/export packet rows from existing read-model proof refs without exposing raw location payloads or claiming product-ready export.'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportRedactedReport]: decodeDisplayText('Redacted report packet'),
  [PortalDevTrackingReportTextToken.TrackingReportExportRetentionAudit]: decodeDisplayText(
    'Retention audit export packet'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportFamilySummary]: decodeDisplayText(
    'Family dashboard summary packet'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportPolicyDrillIn]: decodeDisplayText(
    'Policy drill-in export packet'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportReadModelReady]: decodeDisplayText(
    'report-export-read-model-ready'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportRedactedCustody]:
    decodeDisplayText('parent-owned-redacted-report'),
  [PortalDevTrackingReportTextToken.TrackingReportExportLocalCustody]: decodeDisplayText('parent-owned-local-export'),
  [PortalDevTrackingReportTextToken.TrackingReportExportRedactedReportEvidence]: decodeDisplayText(
    'tracking-report-export-evidence-redacted-report'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportRetentionAuditEvidence]: decodeDisplayText(
    'tracking-report-export-evidence-retention-audit'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportFamilySummaryEvidence]: decodeDisplayText(
    'tracking-report-export-evidence-family-dashboard'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportPolicyDrillInEvidence]: decodeDisplayText(
    'tracking-report-export-evidence-policy-drill-in'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportExportHostedBoundary]: decodeDisplayText(
    'Hosted report/export packet rendering only; raw location payload export, service mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerHostedUi]:
    decodeDisplayText('Report policy consumer UI'),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerHostedUiBody]: decodeDisplayText(
    'Hosted route renders parent report summary, policy drill-in, and retention audit consumer rows from stored journal/read-model refs without claiming product-ready report or policy execution.'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerParentReport]: decodeDisplayText(
    'Parent report summary consumer'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerPolicyDrillIn]: decodeDisplayText(
    'Policy evidence drill-in consumer'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerRetentionAudit]: decodeDisplayText(
    'Retention audit export consumer'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerReady]: decodeDisplayText('consumer-ready'),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerReportEvidence]: decodeDisplayText(
    'tracking-report-policy-evidence-summary'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerPolicyEvidence]: decodeDisplayText(
    'tracking-report-policy-evidence-decision'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerRetentionEvidence]: decodeDisplayText(
    'tracking-report-policy-evidence-retention'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerReportJournal]: decodeDisplayText(
    'tracking-journal-row-report-summary'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerPolicyJournal]: decodeDisplayText(
    'tracking-journal-row-policy-drill-in'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerRetentionJournal]: decodeDisplayText(
    'tracking-journal-row-retention-export'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerReportReadModel]: decodeDisplayText(
    'tracking-read-model-row-report-summary'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerPolicyReadModel]: decodeDisplayText(
    'tracking-read-model-row-policy-drill-in'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerRetentionReadModel]: decodeDisplayText(
    'tracking-read-model-row-retention-export'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerReportSurface]: decodeDisplayText(
    'parent-report-location-summary-row'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerPolicySurface]: decodeDisplayText(
    'parent-policy-evidence-drill-in-row'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerRetentionSurface]: decodeDisplayText(
    'parent-retention-audit-export-row'
  ),
  [PortalDevTrackingReportTextToken.TrackingReportPolicyConsumerHostedBoundary]: decodeDisplayText(
    'Hosted report/policy consumer rendering only; AI execution, product policy mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, production, and product readiness remain unclaimed.'
  ),
};
