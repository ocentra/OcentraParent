import {
  AgentAppGamePolicyReadinessKind,
  AgentAppGamePolicyReadinessState,
  type AgentAppGamePolicyReadinessReadModel,
  type AgentAppGamePolicyReadinessResult,
  type AgentAppGamePolicyReadinessRow,
} from '@ocentra-parent/agent-protocol-domain/app-game-policy-readiness';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  ManualRequired: requiredReadableValue('manual-required'),
  NotClaimed: requiredReadableValue('not-claimed'),
  Off: requiredReadableValue('false'),
  Ready: requiredReadableValue('ready'),
  Review: requiredReadableValue('warn'),
  Unavailable: requiredReadableValue('unavailable'),
} as const;

const PolicyReadinessKindLabels = {
  [AgentAppGamePolicyReadinessKind.PolicyEvidence]: decodeDisplayText('Policy evidence'),
  [AgentAppGamePolicyReadinessKind.ApprovalAuthority]: decodeDisplayText('Approval authority'),
  [AgentAppGamePolicyReadinessKind.ApprovalActionResult]: decodeDisplayText('Approval action result'),
  [AgentAppGamePolicyReadinessKind.PlatformAuthority]: decodeDisplayText('Platform authority'),
  [AgentAppGamePolicyReadinessKind.AiClassifierContext]: decodeDisplayText('AI classifier context'),
  [AgentAppGamePolicyReadinessKind.CategoryCandidate]: decodeDisplayText('Category candidate'),
  [AgentAppGamePolicyReadinessKind.UnknownReview]: decodeDisplayText('Unknown review'),
} satisfies Readonly<Record<AgentAppGamePolicyReadinessKind, DisplayText>>;

const PolicyReadinessManualReasons = {
  [AgentAppGamePolicyReadinessKind.PolicyEvidence]: decodeDisplayText('Policy evidence requires manual review'),
  [AgentAppGamePolicyReadinessKind.ApprovalAuthority]: decodeDisplayText('Approval authority requires manual review'),
  [AgentAppGamePolicyReadinessKind.ApprovalActionResult]: decodeDisplayText(
    'Approval action result requires manual review'
  ),
  [AgentAppGamePolicyReadinessKind.PlatformAuthority]: decodeDisplayText('Platform authority requires manual review'),
  [AgentAppGamePolicyReadinessKind.AiClassifierContext]: decodeDisplayText(
    'AI classifier context requires manual review'
  ),
  [AgentAppGamePolicyReadinessKind.CategoryCandidate]: decodeDisplayText('Category candidate requires manual review'),
  [AgentAppGamePolicyReadinessKind.UnknownReview]: decodeDisplayText('Unknown evidence requires manual review'),
} satisfies Readonly<Record<AgentAppGamePolicyReadinessKind, DisplayText>>;

const PolicyReadinessMissingReasons = {
  [AgentAppGamePolicyReadinessKind.PolicyEvidence]: decodeDisplayText('Policy evidence is missing'),
  [AgentAppGamePolicyReadinessKind.ApprovalAuthority]: decodeDisplayText('Approval authority is missing'),
  [AgentAppGamePolicyReadinessKind.ApprovalActionResult]: decodeDisplayText('Approval action result is missing'),
  [AgentAppGamePolicyReadinessKind.PlatformAuthority]: decodeDisplayText('Platform authority is missing'),
  [AgentAppGamePolicyReadinessKind.AiClassifierContext]: decodeDisplayText('AI classifier context is missing'),
  [AgentAppGamePolicyReadinessKind.CategoryCandidate]: decodeDisplayText('Category candidate evidence is missing'),
  [AgentAppGamePolicyReadinessKind.UnknownReview]: decodeDisplayText('Unknown review evidence is missing'),
} satisfies Readonly<Record<AgentAppGamePolicyReadinessKind, DisplayText>>;

export type AppGamePolicyReadinessPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type AppGamePolicyReadinessPanelRow = {
  readonly title: DisplayText;
  readonly details: readonly AppGamePolicyReadinessPanelDetail[];
};

export type AppGamePolicyReadinessPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly AppGamePolicyReadinessPanelDetail[];
  readonly rows: readonly AppGamePolicyReadinessPanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

export function createAppGamePolicyReadinessPanelIntent(
  readModelResult: AgentAppGamePolicyReadinessResult | null
): AppGamePolicyReadinessPanelIntent {
  const base = baseIntent();

  if (readModelResult === null) {
    return {
      ...base,
      loadState: Readable.Unavailable,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Unavailable),
        detail(PortalDetails.ProductClaim, base.productClaim),
      ],
      rows: [],
    };
  }

  if (!readModelResult.ok) {
    return {
      ...base,
      loadState: Readable.Review,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Review),
        detail(PortalDetails.Reason, displayText(readModelResult.reason)),
        detail(PortalDetails.ProductClaim, base.productClaim),
      ],
      rows: [],
    };
  }

  return {
    ...base,
    loadState: readinessLoadState(readModelResult.value),
    summaryDetails: readModelSummary(readModelResult.value, base.productClaim),
    rows: readModelResult.value.rows.map((row) => readinessRow(row, base.productClaim)),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.PolicyReadiness,
    title: resolvePortalDevText(PortalDevTextToken.AppGamePolicyReadiness),
    body: resolvePortalDevText(PortalDevTextToken.AppGamePolicyReadinessBody),
    emptyMessage: resolvePortalDevText(PortalDevTextToken.AppGamePolicyReadinessNoData),
    productClaim: resolvePortalDevText(PortalDevTextToken.AppGamePolicyReadinessNoProductClaim),
  };
}

function readModelSummary(
  readModel: AgentAppGamePolicyReadinessReadModel,
  productClaim: DisplayText
): readonly AppGamePolicyReadinessPanelDetail[] {
  return [
    detail(PortalDetails.Status, readinessLoadState(readModel)),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PortalDetails.Custody, displayText(readModel.custodyLabel)),
    detail(PortalDetails.Capability, readableValue(readModel.capabilityStatus)),
    detail(PortalDetails.RowsReturned, countText(readModel.returned)),
    detail(PortalDetails.EvidenceClaimRows, countText(readModel.evidenceClaimRowCount)),
    detail(PortalDetails.IdentityRows, countText(readModel.identityRowCount)),
    detail(PortalDetails.ApprovalAuthorityRows, countText(readModel.approvalAuthorityRowCount)),
    detail(PortalDetails.ApprovalActionResultRows, countText(readModel.approvalActionResultRowCount)),
    detail(PortalDetails.PlatformAuthorityRows, countText(readModel.platformAuthorityRowCount)),
    detail(PortalDetails.AiClassifierRows, countText(readModel.aiClassifierResultRowCount)),
    detail(PortalDetails.CategoryCandidateRows, countText(readModel.categoryCandidateRowCount)),
    detail(PortalDetails.UnknownReviewRows, countText(readModel.unknownReviewRowCount)),
    detail(PortalDetails.PolicyEvaluation, readinessBoolean(readModel.policyEvaluationReady)),
    detail(PortalDetails.CategoryRouting, readinessBoolean(readModel.categoryRoutingReady)),
    detail(PortalDetails.ManualReview, manualReviewValue(readModel.manualReviewRequired)),
    detail(PortalDetails.UnknownReview, manualReviewValue(readModel.unknownReviewRequired)),
    detail(PortalDetails.AdapterDispatch, Readable.NotClaimed),
    detail(PortalDetails.ProductClaim, productClaim),
  ];
}

function readinessRow(row: AgentAppGamePolicyReadinessRow, productClaim: DisplayText): AppGamePolicyReadinessPanelRow {
  return {
    title: PolicyReadinessKindLabels[row.readinessKind],
    details: [
      detail(PortalDetails.ReadinessKind, PolicyReadinessKindLabels[row.readinessKind]),
      detail(PortalDetails.Status, readinessState(row.readinessState)),
      detail(PortalDetails.RowCount, countText(row.rowCount)),
      detail(PortalDetails.Reason, readinessReason(row)),
      detail(PortalDetails.EvidenceReferences, evidenceReferences(row)),
      detail(PortalDetails.ProductClaim, productClaim),
    ],
  };
}

function readinessLoadState(readModel: AgentAppGamePolicyReadinessReadModel): DisplayText {
  if (readModel.policyEvaluationReady && !readModel.manualReviewRequired) {
    return Readable.Ready;
  }
  if (readModel.manualReviewRequired) {
    return Readable.Review;
  }
  return Readable.Unavailable;
}

function readinessState(state: AgentAppGamePolicyReadinessState): DisplayText {
  if (state === AgentAppGamePolicyReadinessState.Ready) {
    return Readable.Ready;
  }
  if (state === AgentAppGamePolicyReadinessState.ManualRequired) {
    return Readable.ManualRequired;
  }
  return Readable.Unavailable;
}

function readinessReason(row: AgentAppGamePolicyReadinessRow): DisplayText {
  if (row.readinessState === AgentAppGamePolicyReadinessState.Ready) {
    return Readable.Ready;
  }
  if (row.readinessState === AgentAppGamePolicyReadinessState.ManualRequired) {
    return PolicyReadinessManualReasons[row.readinessKind];
  }
  return PolicyReadinessMissingReasons[row.readinessKind];
}

function readinessBoolean(value: boolean): DisplayText {
  return value ? Readable.Ready : Readable.Review;
}

function manualReviewValue(value: boolean): DisplayText {
  return value ? Readable.ManualRequired : Readable.Off;
}

function evidenceReferences(row: AgentAppGamePolicyReadinessRow): DisplayText {
  const references = [
    ...row.evidenceReferenceIds.map((reference) => String(reference)),
    ...row.evidence.map((evidence) => String(evidence.evidenceId)),
  ];
  const uniqueReferences = [...new Set(references)].filter(Boolean);

  if (uniqueReferences.length === 0) {
    return resolvePortalDevText(PortalDevTextToken.NotReported);
  }
  return displayText(uniqueReferences.join(DetailSeparator));
}

function readableValue(value: unknown): DisplayText {
  const key = String(value);
  return PortalReadableValues[key] ?? displayText(key);
}

function requiredReadableValue(key: string): DisplayText {
  const value = PortalReadableValues[key];
  if (value === undefined) {
    throw new Error(`Missing portal readable value: ${key}`);
  }
  return value;
}

function countText(value: number): DisplayText {
  return displayText(String(value));
}

function displayText(value: string): DisplayText {
  return decodeDisplayText(value);
}

function detail(label: DisplayText, value: DisplayText): AppGamePolicyReadinessPanelDetail {
  return {
    label,
    value,
  };
}
