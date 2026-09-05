import { decodeDisplayText } from './display-text';

const CapabilityStatusDomain = {
  AppUse: 'app-use',
  Browser: 'browser',
  Games: 'games',
  Network: 'network',
  Screen: 'screen',
  Tracking: 'tracking',
} as const;

export const CAPABILITY_STATUS_TEXT = {
  ariaLabel: decodeDisplayText('Capability status'),
  eyebrow: decodeDisplayText('Local service read model'),
  title: decodeDisplayText('Capability and service status'),
  body: decodeDisplayText(
    'Only capability and read-model states reported by the local service are shown. Missing domains are not inferred.'
  ),
  unavailableTitle: decodeDisplayText('Capability status unavailable'),
  unavailableBody: decodeDisplayText(
    'No capability or service read model has been reported. Start or reconnect the local service, then refresh.'
  ),
  unavailableStatus: decodeDisplayText('Not reported'),
  capabilityDomains: decodeDisplayText('Capability domains'),
  refresh: decodeDisplayText('Refresh capability status'),
  browser: decodeDisplayText('Browser'),
  tracking: decodeDisplayText('Tracking'),
  screen: decodeDisplayText('Screen activity'),
  appUse: decodeDisplayText('App activity'),
  games: decodeDisplayText('Game activity'),
  network: decodeDisplayText('Network activity'),
  status: decodeDisplayText('Status'),
  source: decodeDisplayText('Source'),
  localService: decodeDisplayText('Local service'),
  reported: decodeDisplayText('Service status reported'),
} as const;

export type CapabilityStatusCardModel = Readonly<{
  id: string;
  title: string;
  status: string;
  reason: string;
  source: string;
}>;

export type CapabilityStatusShellStatus = Readonly<{
  cards: readonly Readonly<{
    id: string;
    label: string;
    value: string;
    detail: string;
  }>[];
  dataSourceLabel: string;
}>;

export type CapabilityStatusRouteModel = Readonly<{
  cards: readonly CapabilityStatusCardModel[];
  reported: boolean;
}>;

type ActivityAdapterResult =
  | Readonly<{ ok: true; state: string }>
  | Readonly<{ ok: false; state: string; reason: string }>;

export type CapabilityStatusLiveActivity = Readonly<{
  browserManagedStatus: Readonly<{
    capabilityStatus?: string | null;
    managedState: string;
    degradedReason?: string | null;
  }> | null;
  activityTrackingReadModel:
    | Readonly<{ ok: true; value: Readonly<{ capabilityStatus: string; returned: number }> }>
    | Readonly<{ ok: false; reason: string }>
    | null;
  activityScreenReadModel: ActivityAdapterResult | null;
  activityAppUseReadModel: ActivityAdapterResult | null;
  activityGamesReadModel: ActivityAdapterResult | null;
  activityNetworkReadModel: ActivityAdapterResult | null;
  networkFlowReadModel: Readonly<{
    returned: number;
    capabilityStatus: string | null;
    custody: string | null;
  }> | null;
}>;

const CAPABILITY_UNAVAILABLE_DOMAINS = [
  {
    id: CapabilityStatusDomain.Browser,
    title: CAPABILITY_STATUS_TEXT.browser,
    reason: decodeDisplayText('No browser capability is reported. Browser controls remain unavailable.'),
  },
  {
    id: CapabilityStatusDomain.Tracking,
    title: CAPABILITY_STATUS_TEXT.tracking,
    reason: decodeDisplayText('No tracking capability is reported. Location and child status remain unavailable.'),
  },
  {
    id: CapabilityStatusDomain.Screen,
    title: CAPABILITY_STATUS_TEXT.screen,
    reason: decodeDisplayText('No screen capability is reported. Capture and live-view controls remain unavailable.'),
  },
  {
    id: CapabilityStatusDomain.AppUse,
    title: CAPABILITY_STATUS_TEXT.appUse,
    reason: decodeDisplayText('No app capability is reported. App activity and controls remain unavailable.'),
  },
  {
    id: CapabilityStatusDomain.Games,
    title: CAPABILITY_STATUS_TEXT.games,
    reason: decodeDisplayText('No game capability is reported. Game activity and controls remain unavailable.'),
  },
  {
    id: CapabilityStatusDomain.Network,
    title: CAPABILITY_STATUS_TEXT.network,
    reason: decodeDisplayText('No network capability is reported. Network evidence and controls remain unavailable.'),
  },
] as const;

export function buildCapabilityStatusRouteModel(
  shellStatus: CapabilityStatusShellStatus | null,
  liveActivity: CapabilityStatusLiveActivity
): CapabilityStatusRouteModel {
  const shellCards = serviceStatusCards(shellStatus);
  const reportedCapabilityCards = capabilityStatusCards(liveActivity);
  return {
    cards: [...shellCards, ...completeCapabilityDomainCards(reportedCapabilityCards)],
    reported: shellCards.length > 0 || reportedCapabilityCards.length > 0,
  };
}

export function capabilityStatusCardState(status: string): 'reported' | 'unavailable' {
  const normalized = status.trim().toLowerCase();
  return normalized === CAPABILITY_STATUS_TEXT.unavailableStatus.toLowerCase() || normalized === 'unavailable'
    ? 'unavailable'
    : 'reported';
}

function capabilityStatusCards(liveActivity: CapabilityStatusLiveActivity): readonly CapabilityStatusCardModel[] {
  const cards: CapabilityStatusCardModel[] = [];
  appendBrowserCard(cards, liveActivity);
  appendTrackingCard(cards, liveActivity);
  appendActivityCard(
    cards,
    CapabilityStatusDomain.Screen,
    CAPABILITY_STATUS_TEXT.screen,
    liveActivity.activityScreenReadModel
  );
  appendActivityCard(
    cards,
    CapabilityStatusDomain.AppUse,
    CAPABILITY_STATUS_TEXT.appUse,
    liveActivity.activityAppUseReadModel
  );
  appendActivityCard(
    cards,
    CapabilityStatusDomain.Games,
    CAPABILITY_STATUS_TEXT.games,
    liveActivity.activityGamesReadModel
  );
  appendNetworkCard(cards, liveActivity);
  return cards;
}

function appendBrowserCard(cards: CapabilityStatusCardModel[], liveActivity: CapabilityStatusLiveActivity): void {
  const browser = liveActivity.browserManagedStatus;
  if (browser === null) return;
  cards.push({
    id: CapabilityStatusDomain.Browser,
    title: CAPABILITY_STATUS_TEXT.browser,
    status: displayValue(browser.capabilityStatus ?? browser.managedState),
    reason: displayValue(browser.degradedReason, CAPABILITY_STATUS_TEXT.reported),
    source: CAPABILITY_STATUS_TEXT.localService,
  });
}

function appendTrackingCard(cards: CapabilityStatusCardModel[], liveActivity: CapabilityStatusLiveActivity): void {
  const tracking = liveActivity.activityTrackingReadModel;
  if (tracking === null) return;
  cards.push({
    id: CapabilityStatusDomain.Tracking,
    title: CAPABILITY_STATUS_TEXT.tracking,
    status: tracking.ok ? tracking.value.capabilityStatus : 'unavailable',
    reason: tracking.ok ? `${tracking.value.returned} rows reported` : tracking.reason,
    source: CAPABILITY_STATUS_TEXT.localService,
  });
}

function appendNetworkCard(cards: CapabilityStatusCardModel[], liveActivity: CapabilityStatusLiveActivity): void {
  if (liveActivity.activityNetworkReadModel !== null) {
    appendActivityCard(
      cards,
      CapabilityStatusDomain.Network,
      CAPABILITY_STATUS_TEXT.network,
      liveActivity.activityNetworkReadModel
    );
    return;
  }
  const readModel = liveActivity.networkFlowReadModel;
  if (readModel === null) return;
  const rowLabel = readModel.returned === 1 ? 'row' : 'rows';
  cards.push({
    id: CapabilityStatusDomain.Network,
    title: CAPABILITY_STATUS_TEXT.network,
    status: displayValue(readModel.capabilityStatus),
    reason: `${readModel.returned} network flow ${rowLabel} reported; custody ${displayValue(readModel.custody)}`,
    source: CAPABILITY_STATUS_TEXT.localService,
  });
}

function appendActivityCard(
  cards: CapabilityStatusCardModel[],
  id: string,
  title: string,
  result: ActivityAdapterResult | null
): void {
  if (result === null) return;
  cards.push({
    id,
    title,
    status: result.state,
    reason: result.ok ? CAPABILITY_STATUS_TEXT.reported : result.reason,
    source: CAPABILITY_STATUS_TEXT.localService,
  });
}

function serviceStatusCards(shellStatus: CapabilityStatusShellStatus | null): readonly CapabilityStatusCardModel[] {
  if (shellStatus === null) return [];
  return shellStatus.cards.map((card) => ({
    id: `service-${card.id}`,
    title: card.label,
    status: card.value,
    reason: card.detail,
    source: shellStatus.dataSourceLabel,
  }));
}

function completeCapabilityDomainCards(
  reportedCards: readonly CapabilityStatusCardModel[]
): readonly CapabilityStatusCardModel[] {
  return CAPABILITY_UNAVAILABLE_DOMAINS.map(({ id, reason, title }) => {
    const reported = reportedCards.find((card) => card.id === id);
    return reported ?? unavailableCapabilityCard(id, title, reason);
  });
}

function unavailableCapabilityCard(id: string, title: string, reason: string): CapabilityStatusCardModel {
  return {
    id,
    title,
    status: CAPABILITY_STATUS_TEXT.unavailableStatus,
    reason,
    source: CAPABILITY_STATUS_TEXT.localService,
  };
}

function displayValue(
  value: string | null | undefined,
  fallback: string = CAPABILITY_STATUS_TEXT.unavailableStatus
): string {
  return value === null || value === undefined || value === '' ? fallback : value;
}
