import type {
  GeneratedPortalScreenSummaryPanelDetailSnapshot,
  GeneratedPortalScreenSummaryPanelRowSnapshot,
  GeneratedPortalScreenSummaryPanelSnapshot,
} from './generated-portal-contracts';
import { decodeDisplayText, PortalDevTextToken, resolvePortalDevText, type DisplayText } from './display-text';
import { PortalDetails, PortalReadableValues } from './details';

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
  panel: GeneratedPortalScreenSummaryPanelSnapshot | null
): ScreenSummaryPanelIntent {
  if (panel === null) {
    return unavailableIntent(baseIntent());
  }

  return {
    eyebrow: displayText(panel.eyebrow),
    title: displayText(panel.title),
    body: displayText(panel.body),
    loadState: displayText(panel.loadState),
    summaryDetails: panel.summaryDetails.map(screenSummaryDetail),
    rows: panel.rows.map(screenSummaryRow),
    emptyMessage: displayText(panel.emptyMessage),
    productClaim: displayText(panel.productClaim),
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

function screenSummaryRow(row: GeneratedPortalScreenSummaryPanelRowSnapshot): ScreenSummaryPanelRow {
  return {
    title: displayText(row.title),
    details: row.details.map(screenSummaryDetail),
  };
}

function screenSummaryDetail(source: GeneratedPortalScreenSummaryPanelDetailSnapshot): ScreenSummaryPanelDetail {
  return detail(displayText(source.label), displayText(source.value));
}

function readableValue(value: string): DisplayText {
  return PortalReadableValues[value] ?? displayText(value);
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
