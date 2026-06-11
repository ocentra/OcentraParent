import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  Ready: requiredReadableValue('ready'),
  Review: requiredReadableValue('warn'),
  Unavailable: requiredReadableValue('unavailable'),
  NotClaimed: requiredReadableValue('not-claimed'),
} as const;

const TransportReceiptClaim = decodeDisplayText(
  'Child runtime transport receipt rows are parent-visible readiness only. Runtime transport execution, receipt ingestion, provider delivery, platform channel delivery, adapter dispatch, platform enforcement, and raw private rows remain unclaimed.'
);
const TransportRows = decodeDisplayText('Transport rows');
const TransportRequiredRows = decodeDisplayText('Transport-required rows');
const ManualRequiredRows = decodeDisplayText('Manual-required rows');
const UnavailableRows = decodeDisplayText('Unavailable rows');
const RequiredTransportRefs = decodeDisplayText('Required transport refs');
const RequiredReceiptRefs = decodeDisplayText('Required receipt refs');
const OpenGaps = decodeDisplayText('Open gaps');
const SourceRuntimeWriter = decodeDisplayText('Source runtime writer');
const ProductMeanings = decodeDisplayText('Product meanings');
const RuntimeTransport = decodeDisplayText('Runtime transport');
const RuntimeReceipt = decodeDisplayText('Runtime receipt');
const PlatformDelivery = decodeDisplayText('Platform delivery');
const RawPrivateRows = decodeDisplayText('Raw private rows');

export type AppGameChildRuntimeTransportReceiptPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type AppGameChildRuntimeTransportReceiptPanelRow = {
  readonly title: DisplayText;
  readonly details: readonly AppGameChildRuntimeTransportReceiptPanelDetail[];
};

export type AppGameChildRuntimeTransportReceiptPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly AppGameChildRuntimeTransportReceiptPanelDetail[];
  readonly rows: readonly AppGameChildRuntimeTransportReceiptPanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

type AppGameChildRuntimeTransportReceiptPanelReadModel = {
  readonly generatedAt: string;
  readonly returned: number;
  readonly transportRequiredCount: number;
  readonly manualRequiredCount: number;
  readonly unavailableCount: number;
  readonly runtimeTransportExecuted: boolean;
  readonly runtimeReceiptIngested: boolean;
  readonly providerDeliveryExecuted: boolean;
  readonly platformDeliveryChannelClaimed: boolean;
  readonly adapterDispatchClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly rawPrivateSourceRowsIncluded: boolean;
  readonly rows: readonly AppGameChildRuntimeTransportReceiptPanelReadModelRow[];
};

type AppGameChildRuntimeTransportReceiptPanelReadModelRow = {
  readonly rowId: string;
  readonly sourceRuntimeWriterRowId: string;
  readonly boundaryState: string;
  readonly productMeanings: readonly string[];
  readonly requiredTransportRefs: readonly string[];
  readonly requiredReceiptRefs: readonly string[];
  readonly openGaps: readonly string[];
  readonly runtimeTransportExecuted: boolean;
  readonly runtimeReceiptIngested: boolean;
  readonly providerDeliveryExecuted: boolean;
  readonly platformDeliveryChannelClaimed: boolean;
};

export function createAppGameChildRuntimeTransportReceiptPanelIntent(
  readModel: AppGameChildRuntimeTransportReceiptPanelReadModel | null
): AppGameChildRuntimeTransportReceiptPanelIntent {
  const base = baseIntent();

  if (readModel === null) {
    return {
      ...base,
      loadState: Readable.Unavailable,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Unavailable),
        detail(PortalDetails.ProductClaim, TransportReceiptClaim),
      ],
      rows: [],
    };
  }

  return {
    ...base,
    loadState:
      readModel.transportRequiredCount > 0 || readModel.manualRequiredCount > 0 ? Readable.Review : Readable.Ready,
    summaryDetails: readModelSummary(readModel),
    rows: readModel.rows.map(transportReceiptRow),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.RuntimeReference,
    title: decodeDisplayText('App/game child runtime transport receipts'),
    body: decodeDisplayText(
      'Parent-visible child runtime transport and receipt readiness for native app and native game warning/request delivery.'
    ),
    emptyMessage: decodeDisplayText('No app/game child runtime transport receipt read model has been reported yet.'),
    productClaim: TransportReceiptClaim,
  };
}

function readModelSummary(
  readModel: AppGameChildRuntimeTransportReceiptPanelReadModel
): readonly AppGameChildRuntimeTransportReceiptPanelDetail[] {
  return [
    detail(PortalDetails.Status, readModel.transportRequiredCount > 0 ? Readable.Review : Readable.Ready),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(TransportRows, countText(readModel.returned)),
    detail(TransportRequiredRows, countText(readModel.transportRequiredCount)),
    detail(ManualRequiredRows, countText(readModel.manualRequiredCount)),
    detail(UnavailableRows, countText(readModel.unavailableCount)),
    detail(RuntimeTransport, claimedValue(readModel.runtimeTransportExecuted)),
    detail(RuntimeReceipt, claimedValue(readModel.runtimeReceiptIngested)),
    detail(PortalDetails.Provider, claimedValue(readModel.providerDeliveryExecuted)),
    detail(PlatformDelivery, claimedValue(readModel.platformDeliveryChannelClaimed)),
    detail(PortalDetails.AdapterDispatch, claimedValue(readModel.adapterDispatchClaimed)),
    detail(PortalDetails.PlatformState, claimedValue(readModel.platformEnforcementClaimed)),
    detail(RawPrivateRows, claimedValue(readModel.rawPrivateSourceRowsIncluded)),
    detail(PortalDetails.ProductClaim, TransportReceiptClaim),
  ];
}

function transportReceiptRow(
  row: AppGameChildRuntimeTransportReceiptPanelReadModelRow
): AppGameChildRuntimeTransportReceiptPanelRow {
  return {
    title: displayText(row.rowId),
    details: [
      detail(PortalDetails.Status, displayText(row.boundaryState)),
      detail(SourceRuntimeWriter, displayText(row.sourceRuntimeWriterRowId)),
      detail(ProductMeanings, joinedOrNotReported(row.productMeanings)),
      detail(RequiredTransportRefs, joinedOrNotReported(row.requiredTransportRefs)),
      detail(RequiredReceiptRefs, joinedOrNotReported(row.requiredReceiptRefs)),
      detail(OpenGaps, joinedOrNotReported(row.openGaps)),
      detail(RuntimeTransport, claimedValue(row.runtimeTransportExecuted)),
      detail(RuntimeReceipt, claimedValue(row.runtimeReceiptIngested)),
      detail(PortalDetails.Provider, claimedValue(row.providerDeliveryExecuted)),
      detail(PlatformDelivery, claimedValue(row.platformDeliveryChannelClaimed)),
      detail(PortalDetails.ProductClaim, TransportReceiptClaim),
    ],
  };
}

function claimedValue(value: boolean): DisplayText {
  return value ? Readable.Ready : Readable.NotClaimed;
}

function joinedOrNotReported(values: readonly string[]): DisplayText {
  if (values.length === 0) {
    return decodeDisplayText('Not reported');
  }
  return displayText(values.join(DetailSeparator));
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

function detail(label: DisplayText, value: DisplayText): AppGameChildRuntimeTransportReceiptPanelDetail {
  return {
    label,
    value,
  };
}
