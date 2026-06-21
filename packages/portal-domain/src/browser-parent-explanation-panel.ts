import {
  BrowserAiParentExplanationBundleSchema,
  type BrowserAiParentExplanationBundle,
} from '@ocentra-parent/schema-domain/browser-ai-parent-explanation-schemas';
import { type DisplayText, decodeDisplayText } from '@ocentra-parent/text-domain/contracts';
import {
  BrowserParentExplanationTextToken,
  type BrowserParentExplanationTextTokenValue,
  resolveBrowserParentExplanationText,
} from '@ocentra-parent/text-domain/browser-parent-explanation';
import { decodePortalDetailValue, type PortalDetailValue } from './detail-values';
import { PortalDetails } from './details';

export type BrowserParentExplanationPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText | PortalDetailValue;
};

export type BrowserParentExplanationPanelRow = {
  readonly key: PortalDetailValue;
  readonly title: DisplayText;
  readonly details: readonly BrowserParentExplanationPanelDetail[];
};

export type BrowserParentExplanationPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly state: PortalDetailValue;
  readonly summary: PortalDetailValue;
  readonly productClaim: DisplayText;
  readonly metrics: readonly BrowserParentExplanationPanelDetail[];
  readonly rows: readonly BrowserParentExplanationPanelRow[];
  readonly emptyMessage: DisplayText;
};

const BrowserParentExplanationValues = {
  EmptyRowCount: '0',
  EmptyRowsSummary: '0 parent explanation rows',
  EmptyState: 'unavailable',
  NotReported: 'not reported',
  ReadyState: 'ready',
  RefsSeparator: ', ',
  RowsSummarySuffix: ' parent explanation rows',
} as const;

const BrowserParentExplanationCopy = {
  Body: decodeDisplayText(
    'Schema-backed parent explanations show evidence, model, policy, action, child experience, fallback, and audit sections only when a validated browser AI explanation bundle is present.'
  ),
  Empty: decodeDisplayText('No browser parent explanation bundle has been reported yet.'),
  ProductClaim: decodeDisplayText(
    'Rendered parent explanation surface only; runtime service delivery, final policy authority, browser mutation, enforcement, remote AI, and raw page or prompt content remain unclaimed.'
  ),
} as const;

export function createBrowserParentExplanationPanelIntent(bundleInput: unknown): BrowserParentExplanationPanelIntent {
  const parsed = BrowserAiParentExplanationBundleSchema.safeParse(bundleInput);
  if (!parsed.success) {
    return emptyPanelIntent();
  }
  return populatedPanelIntent(parsed.data);
}

function populatedPanelIntent(bundle: BrowserAiParentExplanationBundle): BrowserParentExplanationPanelIntent {
  return {
    eyebrow: resolveBrowserParentExplanationText(BrowserParentExplanationTextToken.Title),
    title: resolveBrowserParentExplanationText(textTokenForParentExplanationToken(bundle.titleTextToken)),
    body: BrowserParentExplanationCopy.Body,
    state: detailValue(bundle.state),
    summary: detailValue(String(bundle.sections.length) + BrowserParentExplanationValues.RowsSummarySuffix),
    productClaim: BrowserParentExplanationCopy.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, String(bundle.sections.length)),
      detail(PortalDetails.GeneratedAt, bundle.createdAt),
      detail(PortalDetails.Status, bundle.state),
      detail(PortalDetails.ProductClaim, BrowserParentExplanationCopy.ProductClaim),
    ],
    rows: [
      {
        key: detailValue('summary'),
        title: resolveBrowserParentExplanationText(BrowserParentExplanationTextToken.Summary),
        details: [
          detail(PortalDetails.BrowserEvidence, refsValue(bundle.sourceEvidenceIds)),
          detail(PortalDetails.EvidenceReferences, refsValue(bundle.aiAnalysis.metadataEvidenceIds)),
          detail(PortalDetails.ReasonCodes, refsValue(bundle.policyDecision.reasonCodes)),
        ],
      },
      {
        key: detailValue('ai-analysis'),
        title: resolveBrowserParentExplanationText(BrowserParentExplanationTextToken.Ai),
        details: [
          detail(PortalDetails.Model, bundle.aiAnalysis.modelRuntimeRef),
          detail(PortalDetails.Version, bundle.aiAnalysis.promptTemplate.promptTemplateVersion),
          detail(PortalDetails.DegradedState, bundle.aiAnalysis.degradedState),
          detail(PortalDetails.ProductClaim, BrowserParentExplanationCopy.ProductClaim),
        ],
      },
      {
        key: detailValue('policy-action'),
        title: resolveBrowserParentExplanationText(BrowserParentExplanationTextToken.Action),
        details: [
          detail(PortalDetails.PolicyEvaluation, bundle.policyDecision.outcome),
          detail(PortalDetails.RuleIds, refsValue(bundle.policyDecision.parentRuleRefs)),
          detail(PortalDetails.InterventionAction, refsValue(bundle.postAnalysisActionPlan.actionLabels)),
          detail(PortalDetails.AdapterBoundary, bundle.postAnalysisActionPlan.adapterProofRef),
          detail(PortalDetails.ChildDelivery, bundle.childUxSnapshot.deliveryState),
        ],
      },
      {
        key: detailValue('audit'),
        title: resolveBrowserParentExplanationText(BrowserParentExplanationTextToken.Audit),
        details: [
          detail(PortalDetails.InterventionAuditId, refsValue(bundle.explanationAuditRefs)),
          detail(PortalDetails.Custody, bundle.childUxSnapshot.surface),
          detail(PortalDetails.ManualReview, String(bundle.manualFallbackVisible)),
          detail(PortalDetails.Enforcement, String(bundle.directEnforcementClaimed)),
        ],
      },
    ],
    emptyMessage: BrowserParentExplanationCopy.Empty,
  };
}

function textTokenForParentExplanationToken(token: string): BrowserParentExplanationTextTokenValue {
  if (token === 'browser.parent.explanation.summary') {
    return BrowserParentExplanationTextToken.Summary;
  }
  if (token === 'browser.parent.explanation.evidence') {
    return BrowserParentExplanationTextToken.Evidence;
  }
  if (token === 'browser.parent.explanation.ai') {
    return BrowserParentExplanationTextToken.Ai;
  }
  if (token === 'browser.parent.explanation.policy') {
    return BrowserParentExplanationTextToken.Policy;
  }
  if (token === 'browser.parent.explanation.action') {
    return BrowserParentExplanationTextToken.Action;
  }
  if (token === 'browser.parent.explanation.childExperience') {
    return BrowserParentExplanationTextToken.ChildExperience;
  }
  if (token === 'browser.parent.explanation.degraded') {
    return BrowserParentExplanationTextToken.Degraded;
  }
  if (token === 'browser.parent.explanation.audit') {
    return BrowserParentExplanationTextToken.Audit;
  }
  return BrowserParentExplanationTextToken.Title;
}

function emptyPanelIntent(): BrowserParentExplanationPanelIntent {
  return {
    eyebrow: resolveBrowserParentExplanationText(BrowserParentExplanationTextToken.Title),
    title: resolveBrowserParentExplanationText(BrowserParentExplanationTextToken.Title),
    body: BrowserParentExplanationCopy.Body,
    state: detailValue(BrowserParentExplanationValues.EmptyState),
    summary: detailValue(BrowserParentExplanationValues.EmptyRowsSummary),
    productClaim: BrowserParentExplanationCopy.ProductClaim,
    metrics: [
      detail(PortalDetails.RowsReturned, BrowserParentExplanationValues.EmptyRowCount),
      detail(PortalDetails.Status, BrowserParentExplanationValues.NotReported),
      detail(PortalDetails.ProductClaim, BrowserParentExplanationCopy.ProductClaim),
    ],
    rows: [],
    emptyMessage: BrowserParentExplanationCopy.Empty,
  };
}

function detail(label: DisplayText, value: unknown): BrowserParentExplanationPanelDetail {
  return {
    label,
    value: detailValue(value),
  };
}

function refsValue(values: readonly unknown[]): PortalDetailValue {
  const refs = values.map((value) => String(value).trim()).filter((value) => value.length > 0);
  return detailValue(
    refs.length > 0
      ? refs.join(BrowserParentExplanationValues.RefsSeparator)
      : BrowserParentExplanationValues.NotReported
  );
}

function detailValue(value: unknown): PortalDetailValue {
  const text =
    typeof value === 'string' && value.trim().length > 0 ? value : BrowserParentExplanationValues.NotReported;
  return decodePortalDetailValue(text);
}
