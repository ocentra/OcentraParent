import type { ActivitySurfaceAdapterResult } from '@ocentra-parent/agent-protocol-domain/activity-surface-adapter';
import {
  ActivityScreenReadModelSchema,
  type ActivityScreenReadModel,
} from '@ocentra-parent/activity-domain/activity-surface';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

type ScreenReadModelRow = ActivityScreenReadModel['rows'][number];

export type ScreenSummaryPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type ScreenSummaryPanelRow = {
  readonly title: DisplayText;
  readonly details: readonly ScreenSummaryPanelDetail[];
};

export type ScreenSummaryPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly ScreenSummaryPanelDetail[];
  readonly rows: readonly ScreenSummaryPanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

export function createScreenSummaryPanelIntent(
  readModelResult: ActivitySurfaceAdapterResult<unknown> | null
): ScreenSummaryPanelIntent {
  const base = baseIntent();

  if (readModelResult === null) {
    return unavailableIntent(base);
  }

  if (!readModelResult.ok) {
    return failedIntent(base, readModelResult.reason);
  }

  const readModel = parseScreenReadModel(readModelResult.value);
  if (readModel === null) {
    return failedIntent(base, 'screen-read-model-shape');
  }

  return {
    ...base,
    loadState: readableValue(readModel.state),
    summaryDetails: summaryDetails(readModel, base.productClaim),
    rows: readModel.rows.map((row) => screenSummaryRow(row, base.productClaim)),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.ActivityKind,
    title: resolvePortalDevText(PortalDevTextToken.ScreenAnalysis),
    body: resolvePortalDevText(PortalDevTextToken.ActivityDescription),
    emptyMessage: resolvePortalDevText(PortalDevTextToken.NoRecentActivity),
    productClaim: resolvePortalDevText(PortalDevTextToken.ProductSurfacePending),
  };
}

function unavailableIntent(base: ReturnType<typeof baseIntent>): ScreenSummaryPanelIntent {
  return {
    ...base,
    loadState: readableValue('unavailable'),
    summaryDetails: [
      detail(PortalDetails.Status, readableValue('unavailable')),
      detail(PortalDetails.ProductClaim, base.productClaim),
    ],
    rows: [],
  };
}

function failedIntent(base: ReturnType<typeof baseIntent>, reason: string): ScreenSummaryPanelIntent {
  return {
    ...base,
    loadState: readableValue('warn'),
    summaryDetails: [
      detail(PortalDetails.Status, readableValue('warn')),
      detail(PortalDetails.Reason, displayText(reason)),
      detail(PortalDetails.ProductClaim, base.productClaim),
    ],
    rows: [],
  };
}

function summaryDetails(
  readModel: ActivityScreenReadModel,
  productClaim: DisplayText
): readonly ScreenSummaryPanelDetail[] {
  const latestRow = readModel.rows.at(0);
  return [
    detail(PortalDetails.Status, readableValue(readModel.state)),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PortalDetails.RowsReturned, countText(readModel.rows.length)),
    detail(PortalDetails.Capability, readableValue(latestRow?.capabilityStatus ?? 'unavailable')),
    detail(PortalDetails.Custody, readableValue(latestRow?.custodyState ?? 'unavailable')),
    detail(PortalDetails.DeletedEvidence, readableValue(latestRow?.imageDeletionState ?? 'unavailable')),
    detail(
      PortalDetails.Model,
      displayText(latestRow?.modelId ?? String(resolvePortalDevText(PortalDevTextToken.NotReported)))
    ),
    detail(PortalDetails.ProductClaim, productClaim),
  ];
}

function screenSummaryRow(row: ScreenReadModelRow, productClaim: DisplayText): ScreenSummaryPanelRow {
  return {
    title: displayText(row.label),
    details: [
      detail(PortalDetails.Status, readableValue(row.state)),
      detail(PortalDetails.EventId, displayText(row.rowId)),
      detail(PortalDetails.Source, displayText(row.captureReason)),
      detail(PortalDetails.Capability, readableValue(row.capabilityStatus)),
      detail(PortalDetails.RuntimeReference, displayText(row.modelRuntimeRef)),
      detail(PortalDetails.Model, displayText(modelSummary(row))),
      detail(PortalDetails.Provider, displayText(row.providerKind)),
      detail(PortalDetails.Level, readableValue(row.confidence)),
      detail(
        PortalDetails.ActivityKind,
        displayText(row.primaryCategory ?? String(resolvePortalDevText(PortalDevTextToken.NotReported)))
      ),
      detail(PortalDetails.Custody, readableValue(row.custodyState)),
      detail(PortalDetails.DeletedEvidence, readableValue(row.imageDeletionState)),
      detail(
        PortalDetails.PolicyPreview,
        displayText(row.policyDecisionRef ?? String(resolvePortalDevText(PortalDevTextToken.NotReported)))
      ),
      detail(
        PortalDetails.DecisionAction,
        displayText(row.policyAction ?? String(resolvePortalDevText(PortalDevTextToken.NotReported)))
      ),
      detail(PortalDetails.EnforcementHandoff, readableValue('not-claimed')),
      detail(PortalDetails.EvidenceReferences, evidenceReferences(row)),
      detail(PortalDetails.ReasonCodes, referenceList(row.policyReasonCodes ?? [])),
      detail(PortalDetails.ParentRuleContextReferences, referenceList(row.parentRuleRefs ?? [])),
      detail(PortalDetails.Reason, referenceList(row.explanationReasons ?? [])),
      detail(PortalDetails.OcrSnippets, referenceList(row.ocrTextSnippets ?? [])),
      detail(PortalDetails.RedactionNotes, referenceList(row.redactionNotes ?? [])),
      detail(PortalDetails.ParentExplanationReferences, referenceList(row.parentExplanationRefs ?? [])),
      detail(PortalDetails.ProductClaim, productClaim),
    ],
  };
}

function modelSummary(row: ScreenReadModelRow): string {
  return [row.modelId, row.promptOrTemplateVersion, row.queueJobId].join(DetailSeparator);
}

function evidenceReferences(row: ScreenReadModelRow): DisplayText {
  return referenceList(row.evidence.map((evidence) => evidence.evidenceId));
}

function referenceList(references: readonly string[]): DisplayText {
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

function countText(value: number): DisplayText {
  return displayText(String(value));
}

function displayText(value: string): DisplayText {
  return decodeDisplayText(value);
}

function detail(label: DisplayText, value: DisplayText): ScreenSummaryPanelDetail {
  return {
    label,
    value,
  };
}

function parseScreenReadModel(value: unknown): ActivityScreenReadModel | null {
  const parsed = ActivityScreenReadModelSchema.safeParse(value);
  return parsed.success && parsed.data !== undefined ? parsed.data : null;
}
