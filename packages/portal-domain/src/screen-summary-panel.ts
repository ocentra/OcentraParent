import type { ActivitySurfaceAdapterResult } from '@ocentra-parent/agent-protocol-domain/activity-surface-adapter';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

type ScreenReadModel = {
  readonly state: string;
  readonly generatedAt: string;
  readonly returned: number;
  readonly rows: readonly ScreenReadModelRow[];
};

type ScreenReadModelRow = {
  readonly rowId: string;
  readonly label: string;
  readonly state: string;
  readonly captureReason: string;
  readonly capabilityStatus: string;
  readonly queueJobId: string;
  readonly modelRuntimeRef: string;
  readonly modelId: string;
  readonly providerKind: string;
  readonly promptOrTemplateVersion: string;
  readonly primaryCategory: string | null;
  readonly confidence: string;
  readonly imageDeletionState: string;
  readonly rawImageRetained: false;
  readonly imageDigest: string;
  readonly custodyState: string;
  readonly evidence: readonly { readonly evidenceId: string }[];
  readonly policyDecisionRef?: string | null;
  readonly policyReasonCodes?: readonly string[];
  readonly parentRuleRefs?: readonly string[];
  readonly parentExplanationRefs?: readonly string[];
};

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

  const readModel = screenReadModelFromUnknown(readModelResult.value);
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

function summaryDetails(readModel: ScreenReadModel, productClaim: DisplayText): readonly ScreenSummaryPanelDetail[] {
  const latestRow = readModel.rows.at(0);
  return [
    detail(PortalDetails.Status, readableValue(readModel.state)),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PortalDetails.RowsReturned, countText(readModel.returned)),
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
      detail(PortalDetails.EnforcementHandoff, readableValue('not-claimed')),
      detail(PortalDetails.EvidenceReferences, evidenceReferences(row)),
      detail(PortalDetails.ReasonCodes, referenceList(row.policyReasonCodes ?? [])),
      detail(PortalDetails.ParentRuleContextReferences, referenceList(row.parentRuleRefs ?? [])),
      detail(PortalDetails.LocalAiResult, referenceList(row.parentExplanationRefs ?? [])),
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

function screenReadModelFromUnknown(value: unknown): ScreenReadModel | null {
  if (!isRecord(value)) {
    return null;
  }
  const rows = Array.isArray(value['rows']) ? value['rows'].map(screenReadModelRowFromUnknown) : null;
  if (rows === null || rows.some((row) => row === null)) {
    return null;
  }
  return {
    state: textValue(value['state']),
    generatedAt: textValue(value['generatedAt']),
    returned: numberValue(value['returned']) ?? rows.length,
    rows: rows.filter((row): row is ScreenReadModelRow => row !== null),
  };
}

function screenReadModelRowFromUnknown(value: unknown): ScreenReadModelRow | null {
  if (!isRecord(value)) {
    return null;
  }
  const evidence = Array.isArray(value['evidence']) ? value['evidence'].map(evidenceRefFromUnknown) : [];
  return {
    rowId: textValue(value['rowId']),
    label: textValue(value['label']),
    state: textValue(value['state']),
    captureReason: textValue(value['captureReason']),
    capabilityStatus: textValue(value['capabilityStatus']),
    queueJobId: textValue(value['queueJobId']),
    modelRuntimeRef: textValue(value['modelRuntimeRef']),
    modelId: textValue(value['modelId']),
    providerKind: textValue(value['providerKind']),
    promptOrTemplateVersion: textValue(value['promptOrTemplateVersion']),
    primaryCategory: nullableTextValue(value['primaryCategory']),
    confidence: textValue(value['confidence']),
    imageDeletionState: textValue(value['imageDeletionState']),
    rawImageRetained: false,
    imageDigest: textValue(value['imageDigest']),
    custodyState: textValue(value['custodyState']),
    evidence: evidence.filter((reference): reference is { readonly evidenceId: string } => reference !== null),
    policyDecisionRef: nullableTextValue(value['policyDecisionRef']),
    policyReasonCodes: textListValue(value['policyReasonCodes']),
    parentRuleRefs: textListValue(value['parentRuleRefs']),
    parentExplanationRefs: textListValue(value['parentExplanationRefs']),
  };
}

function evidenceRefFromUnknown(value: unknown): { readonly evidenceId: string } | null {
  if (!isRecord(value)) {
    return null;
  }
  return {
    evidenceId: textValue(value['evidenceId']),
  };
}

function textListValue(value: unknown): readonly string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.map((item) => String(item)).filter(Boolean);
}

function nullableTextValue(value: unknown): string | null {
  return value === null || value === undefined ? null : textValue(value);
}

function textValue(value: unknown): string {
  return String(value ?? resolvePortalDevText(PortalDevTextToken.NotReported));
}

function numberValue(value: unknown): number | null {
  return typeof value === 'number' && Number.isFinite(value) ? value : null;
}

function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === 'object' && value !== null;
}
