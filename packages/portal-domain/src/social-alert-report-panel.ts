import {
  SocialAlertReportIntentKind,
  type SocialAlertReportIntent,
} from '@ocentra-parent/social-domain/social-alert-report-intent';
import {
  SocialAlertReportReadModelSnapshotSchema,
  type SocialAlertReportProviderStatusRow,
  type SocialAlertReportReadModelSnapshot,
} from '@ocentra-parent/agent-protocol-domain/social-alert-report-read-model';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/text-domain/contracts';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

export type SocialAlertReportPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type SocialAlertReportPanelRow = {
  readonly key: PortalDetailValue;
  readonly title: DisplayText;
  readonly details: readonly SocialAlertReportPanelDetail[];
};

export type SocialAlertReportPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly productClaim: DisplayText;
  readonly metrics: readonly SocialAlertReportPanelDetail[];
  readonly rows: readonly SocialAlertReportPanelRow[];
  readonly emptyMessage: DisplayText;
};

const SocialAlertReportValues = {
  EmptyRowCount: '0',
  EmptyRowsSummary: '0 social alert/report rows',
  EmptyState: 'unavailable',
  NotReported: 'not reported',
  ReadyState: 'ready',
  RefsSeparator: ', ',
  RowsSummarySuffix: ' social alert/report rows',
} as const;

const SocialAlertReportCopy = {
  Body: decodeDisplayText(
    'Schema-backed social alert and report intents show ref-only local outbox or manual-required rows without provider delivery or enforcement claims.'
  ),
  Empty: decodeDisplayText('No social alert/report read model has been reported yet.'),
  ProductClaim: decodeDisplayText(
    'Rendered parent alert/report intent surface only; provider delivery, report delivery, notification UI delivery, final policy execution, and enforcement remain unclaimed.'
  ),
  Title: decodeDisplayText('Social alerts and reports'),
} as const;

const SocialAlertReportTitles = {
  AccountApproval: decodeDisplayText('Account approval alert intent'),
  CapabilityUnavailable: decodeDisplayText('Unavailable alert/report capability'),
  FeedVideoGate: decodeDisplayText('Feed and video gate alert intent'),
  HighRiskSignal: decodeDisplayText('High-risk social alert intent'),
  ManualRequired: decodeDisplayText('Manual alert/report proof required'),
  ProviderManualRequired: decodeDisplayText('Provider status manual required'),
  ProviderUnavailable: decodeDisplayText('Provider status unavailable'),
  WeeklySummary: decodeDisplayText('Weekly social report intent'),
} as const;

export function createSocialAlertReportPanelIntent(snapshotInput: unknown): SocialAlertReportPanelIntent {
  const parsed = SocialAlertReportReadModelSnapshotSchema.safeParse(snapshotInput);
  if (!parsed.success) {
    return emptyPanelIntent();
  }
  return populatedPanelIntent(parsed.data);
}

function populatedPanelIntent(snapshot: SocialAlertReportReadModelSnapshot): SocialAlertReportPanelIntent {
  const rows = [...snapshot.intents.map(intentRow), ...snapshot.providerStatusRows.map(providerStatusRow)];
  return {
    eyebrow: SocialAlertReportCopy.Title,
    title: SocialAlertReportCopy.Title,
    body: SocialAlertReportCopy.Body,
    state: detailValue(rows.length > 0 ? SocialAlertReportValues.ReadyState : SocialAlertReportValues.EmptyState),
    summary: detailValue(String(rows.length) + SocialAlertReportValues.RowsSummarySuffix),
    productClaim: SocialAlertReportCopy.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, String(rows.length)),
      detail(PortalDetails.GeneratedAt, snapshot.generatedAt),
      detail(PortalDetails.ProductClaim, SocialAlertReportCopy.ProductClaim),
    ],
    rows,
    emptyMessage: SocialAlertReportCopy.Empty,
  };
}

function emptyPanelIntent(): SocialAlertReportPanelIntent {
  return {
    eyebrow: SocialAlertReportCopy.Title,
    title: SocialAlertReportCopy.Title,
    body: SocialAlertReportCopy.Body,
    state: detailValue(SocialAlertReportValues.EmptyState),
    summary: detailValue(SocialAlertReportValues.EmptyRowsSummary),
    productClaim: SocialAlertReportCopy.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, SocialAlertReportValues.EmptyRowCount),
      detail(PortalDetails.Status, SocialAlertReportValues.NotReported),
      detail(PortalDetails.ProductClaim, SocialAlertReportCopy.ProductClaim),
    ],
    rows: [],
    emptyMessage: SocialAlertReportCopy.Empty,
  };
}

function intentRow(intent: SocialAlertReportIntent): SocialAlertReportPanelRow {
  return {
    key: detailValue(intent.alertReportIntentId),
    title: intentTitle(intent),
    details: [
      detail(PortalDetails.Status, intent.intentStatus),
      detail(PortalDetails.Capability, intent.deliveryClaimState),
      detail(PortalDetails.ReasonCodes, intent.notificationReasonCode),
      detail(PortalDetails.EvidenceReferences, evidenceRefsValue(intent)),
      detail(PortalDetails.PolicyEvaluation, refsValue(intent.policyRefs)),
      detail(PortalDetails.InterventionAuditId, refsValue(intent.auditRefs)),
      detail(PortalDetails.ProductClaim, SocialAlertReportCopy.ProductClaim),
    ],
  };
}

function intentTitle(intent: SocialAlertReportIntent): DisplayText {
  if (intent.intentKind === SocialAlertReportIntentKind.HighRiskSignal) {
    return SocialAlertReportTitles.HighRiskSignal;
  }
  if (intent.intentKind === SocialAlertReportIntentKind.AccountApprovalNeeded) {
    return SocialAlertReportTitles.AccountApproval;
  }
  if (intent.intentKind === SocialAlertReportIntentKind.FeedVideoGate) {
    return SocialAlertReportTitles.FeedVideoGate;
  }
  if (intent.intentKind === SocialAlertReportIntentKind.WeeklySummary) {
    return SocialAlertReportTitles.WeeklySummary;
  }
  if (intent.intentKind === SocialAlertReportIntentKind.CapabilityUnavailable) {
    return SocialAlertReportTitles.CapabilityUnavailable;
  }
  return SocialAlertReportTitles.ManualRequired;
}

function providerStatusRow(row: SocialAlertReportProviderStatusRow): SocialAlertReportPanelRow {
  return {
    key: detailValue(row.statusEntryId),
    title:
      row.providerStatus === 'unavailable'
        ? SocialAlertReportTitles.ProviderUnavailable
        : SocialAlertReportTitles.ProviderManualRequired,
    details: [
      detail(PortalDetails.Status, row.providerStatus),
      detail(PortalDetails.Capability, row.deliveryClaimState),
      detail(PortalDetails.ReasonCodes, row.sourcePreflightStatus),
      detail(PortalDetails.EvidenceReferences, refsValue(row.readinessRefs)),
      detail(PortalDetails.PolicyEvaluation, refsValue(row.manualProofRequirements)),
      detail(PortalDetails.InterventionAuditId, row.providerAttemptRef),
      detail(PortalDetails.ProductClaim, SocialAlertReportCopy.ProductClaim),
    ],
  };
}

function evidenceRefsValue(intent: SocialAlertReportIntent): PortalDetailValue {
  return refsValue(intent.evidenceReferences.map((ref) => `${ref.kind}:${ref.evidenceReferenceId}`));
}

function detail(label: DisplayText, value: unknown): SocialAlertReportPanelDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(
    refs.length > 0 ? refs.join(SocialAlertReportValues.RefsSeparator) : SocialAlertReportValues.NotReported
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text = typeof value === 'string' && value.trim().length > 0 ? value : SocialAlertReportValues.NotReported;
  return decodePortalDetailValue(text);
}
