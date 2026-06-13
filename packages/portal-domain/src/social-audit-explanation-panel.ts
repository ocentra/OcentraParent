import {
  SocialAuditExplanationSnapshotSchema,
  type SocialAuditExplanationEntry,
  type SocialAuditExplanationSnapshot,
} from '@ocentra-parent/browser-domain/social-audit-explanation-read-model';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/text-domain/contracts';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

export type SocialAuditExplanationPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type SocialAuditExplanationPanelRow = {
  readonly key: PortalDetailValue;
  readonly title: DisplayText;
  readonly details: readonly SocialAuditExplanationPanelDetail[];
};

export type SocialAuditExplanationPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly productClaim: DisplayText;
  readonly metrics: readonly SocialAuditExplanationPanelDetail[];
  readonly rows: readonly SocialAuditExplanationPanelRow[];
  readonly emptyMessage: DisplayText;
};

const SocialAuditExplanationValues = {
  EmptyRowCount: '0',
  EmptyRowsSummary: '0 social explanation rows',
  EmptyState: 'unavailable',
  NotReported: 'not reported',
  ReadyState: 'ready',
  RefsSeparator: ', ',
  RowsSummarySuffix: ' social explanation rows',
} as const;

const SocialAuditExplanationCopy = {
  Body: decodeDisplayText(
    'Schema-backed social explanations show parent-visible evidence, policy, approval, memory, connector, native, manual, and audit refs without raw social content.'
  ),
  Empty: decodeDisplayText('No social audit explanation snapshot has been reported yet.'),
  ProductClaim: decodeDisplayText(
    'Rendered parent explanation surface only; runtime audit-store delivery, notifications, connector authorization, native app control, final policy execution, and enforcement remain unclaimed.'
  ),
  Title: decodeDisplayText('Social explanations'),
} as const;

const SocialAuditExplanationTitles = {
  AccountApproval: decodeDisplayText('Account approval explanation'),
  ConnectorBoundary: decodeDisplayText('Connected account boundary explanation'),
  DecisionMemory: decodeDisplayText('Remembered decision explanation'),
  FeedVideoGate: decodeDisplayText('Feed and video gate explanation'),
  ManualRequiredGap: decodeDisplayText('Manual proof gap explanation'),
  NativeAppGap: decodeDisplayText('Native app gap explanation'),
} as const;

export function createSocialAuditExplanationPanelIntent(snapshotInput: unknown): SocialAuditExplanationPanelIntent {
  const parsed = SocialAuditExplanationSnapshotSchema.safeParse(snapshotInput);
  if (!parsed.success) {
    return emptyPanelIntent();
  }
  return populatedPanelIntent(parsed.data);
}

function populatedPanelIntent(snapshot: SocialAuditExplanationSnapshot): SocialAuditExplanationPanelIntent {
  return {
    eyebrow: SocialAuditExplanationCopy.Title,
    title: SocialAuditExplanationCopy.Title,
    body: SocialAuditExplanationCopy.Body,
    state: detailValue(
      snapshot.entries.length > 0 ? SocialAuditExplanationValues.ReadyState : SocialAuditExplanationValues.EmptyState
    ),
    summary: detailValue(String(snapshot.entries.length) + SocialAuditExplanationValues.RowsSummarySuffix),
    productClaim: SocialAuditExplanationCopy.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, String(snapshot.entries.length)),
      detail(PortalDetails.GeneratedAt, snapshot.capturedAt),
      detail(PortalDetails.ProductClaim, SocialAuditExplanationCopy.ProductClaim),
    ],
    rows: snapshot.entries.map(entryRow),
    emptyMessage: SocialAuditExplanationCopy.Empty,
  };
}

function emptyPanelIntent(): SocialAuditExplanationPanelIntent {
  return {
    eyebrow: SocialAuditExplanationCopy.Title,
    title: SocialAuditExplanationCopy.Title,
    body: SocialAuditExplanationCopy.Body,
    state: detailValue(SocialAuditExplanationValues.EmptyState),
    summary: detailValue(SocialAuditExplanationValues.EmptyRowsSummary),
    productClaim: SocialAuditExplanationCopy.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, SocialAuditExplanationValues.EmptyRowCount),
      detail(PortalDetails.Status, SocialAuditExplanationValues.NotReported),
      detail(PortalDetails.ProductClaim, SocialAuditExplanationCopy.ProductClaim),
    ],
    rows: [],
    emptyMessage: SocialAuditExplanationCopy.Empty,
  };
}

function entryRow(entry: SocialAuditExplanationEntry): SocialAuditExplanationPanelRow {
  return {
    key: detailValue(entry.eventId),
    title: entryTitle(entry),
    details: [
      detail(PortalDetails.Status, entry.status),
      detail(PortalDetails.DecisionSource, entry.decisionState),
      detail(PortalDetails.DecisionAction, entry.actionCandidate),
      detail(PortalDetails.PolicyEvaluation, entry.policyVersionRef),
      detail(PortalDetails.EvidenceReferences, evidenceRefsValue(entry)),
      detail(PortalDetails.ReasonCodes, refsValue(entry.policyReasonCodes)),
      detail(PortalDetails.Reason, refsValue(entry.explanationReasons)),
      detail(PortalDetails.InterventionAuditId, refsValue(entry.auditRefs)),
      detail(PortalDetails.ProductClaim, SocialAuditExplanationCopy.ProductClaim),
    ],
  };
}

function entryTitle(entry: SocialAuditExplanationEntry): DisplayText {
  if (entry.subjectKind === 'account-approval') {
    return SocialAuditExplanationTitles.AccountApproval;
  }
  if (entry.subjectKind === 'feed-video-gate') {
    return SocialAuditExplanationTitles.FeedVideoGate;
  }
  if (entry.subjectKind === 'native-app-gap') {
    return SocialAuditExplanationTitles.NativeAppGap;
  }
  if (entry.subjectKind === 'connector-boundary') {
    return SocialAuditExplanationTitles.ConnectorBoundary;
  }
  if (entry.subjectKind === 'decision-memory') {
    return SocialAuditExplanationTitles.DecisionMemory;
  }
  return SocialAuditExplanationTitles.ManualRequiredGap;
}

function evidenceRefsValue(entry: SocialAuditExplanationEntry): PortalDetailValue {
  return refsValue(entry.evidenceLinks.map((link) => `${link.evidenceKind}:${link.evidenceRef}`));
}

function detail(label: DisplayText, value: unknown): SocialAuditExplanationPanelDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(
    refs.length > 0 ? refs.join(SocialAuditExplanationValues.RefsSeparator) : SocialAuditExplanationValues.NotReported
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text = typeof value === 'string' && value.trim().length > 0 ? value : SocialAuditExplanationValues.NotReported;
  return decodePortalDetailValue(text);
}
