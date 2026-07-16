import { decodeDisplayText, type DisplayText } from './display-text';
import { PortalReadableValues } from './details';

type SetupFirstRunPanelSnapshotLike = {
  readonly eyebrow: string;
  readonly title: string;
  readonly body: string;
  readonly summaryCardTitle: string;
  readonly summary: string;
  readonly summaryDetails: readonly SetupFirstRunPanelDetailSnapshotLike[];
  readonly cards: readonly SetupFirstRunPanelCardSnapshotLike[];
  readonly productClaim: string;
};

type SetupFirstRunPanelCardSnapshotLike = {
  readonly title: string;
  readonly summary: string;
  readonly details: readonly SetupFirstRunPanelDetailSnapshotLike[];
};

type SetupFirstRunPanelDetailSnapshotLike = {
  readonly label: string;
  readonly value: string;
};

export type SetupFirstRunPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type SetupFirstRunPanelCard = {
  readonly title: DisplayText;
  readonly summary: DisplayText;
  readonly details: readonly SetupFirstRunPanelDetail[];
};

export type SetupFirstRunPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly summaryCardTitle: DisplayText;
  readonly summary: DisplayText;
  readonly summaryDetails: readonly SetupFirstRunPanelDetail[];
  readonly cards: readonly SetupFirstRunPanelCard[];
  readonly productClaim: DisplayText;
};

export function createSetupFirstRunPanelIntent(panel: SetupFirstRunPanelSnapshotLike): SetupFirstRunPanelIntent {
  return {
    eyebrow: decodeDisplayText(panel.eyebrow),
    title: decodeDisplayText(panel.title),
    body: decodeDisplayText(panel.body),
    summaryCardTitle: decodeDisplayText(panel.summaryCardTitle),
    summary: decodeDisplayText(panel.summary),
    summaryDetails: panel.summaryDetails.map(projectDetail),
    cards: panel.cards.map(projectCard),
    productClaim: decodeDisplayText(panel.productClaim),
  };
}

function projectCard(card: SetupFirstRunPanelCardSnapshotLike): SetupFirstRunPanelCard {
  return {
    title: decodeDisplayText(card.title),
    summary: decodeDisplayText(card.summary),
    details: card.details.map(projectDetail),
  };
}

function projectDetail(detail: SetupFirstRunPanelDetailSnapshotLike): SetupFirstRunPanelDetail {
  return {
    label: decodeDisplayText(detail.label),
    value: decodeDisplayText(detail.value),
  };
}

export function readableSetupValue(value: string): DisplayText {
  return PortalReadableValues[value] ?? decodeDisplayText(titleCase(value));
}

function titleCase(value: string): string {
  return value
    .split(/[-_.\s]+/u)
    .filter((part) => part.length > 0)
    .map((part) => part[0]!.toUpperCase() + part.slice(1))
    .join(' ');
}
