import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';

const DetailSeparator = ' | ';

const Readable = {
  Ready: requiredReadableValue('ready'),
  Review: requiredReadableValue('warn'),
  Unavailable: requiredReadableValue('unavailable'),
  NotClaimed: requiredReadableValue('not-claimed'),
} as const;

const PlatformProofClaim = decodeDisplayText(
  'Windows, Android, Linux, macOS, and iOS platform proof rows are parent-visible evidence only. Native enforcement, broad blocking, rollback, audit, Apple CI artifacts, and child delivery remain unclaimed until platform authority proof is attached.'
);
const AuthorityState = decodeDisplayText('Authority state');
const PackageVisibility = decodeDisplayText('Package visibility');
const RuntimeVisibility = decodeDisplayText('Runtime visibility');
const OpenGaps = decodeDisplayText('Open gaps');
const PlatformProofs = decodeDisplayText('Platform proofs');
const HostVisibleRows = decodeDisplayText('Host-visible rows');
const HostNotDetectedRows = decodeDisplayText('Host not detected rows');
const NotApplicableRows = decodeDisplayText('Not-applicable rows');
const EnforcementReadyRows = decodeDisplayText('Enforcement-ready rows');
const HostCapability = decodeDisplayText('Host capability');
const HostProbeRefs = decodeDisplayText('Host probe refs');

export type AppGamePlatformProofStatusPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type AppGamePlatformProofStatusPanelRow = {
  readonly title: DisplayText;
  readonly details: readonly AppGamePlatformProofStatusPanelDetail[];
};

export type AppGamePlatformProofStatusPanelIntent = {
  readonly eyebrow: DisplayText;
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly loadState: DisplayText;
  readonly summaryDetails: readonly AppGamePlatformProofStatusPanelDetail[];
  readonly rows: readonly AppGamePlatformProofStatusPanelRow[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

type AppGamePlatformProofStatusPanelReadModel = {
  readonly generatedAt: string;
  readonly returned: number;
  readonly hostVisibleCount: number;
  readonly hostNotDetectedCount: number;
  readonly localRuntimeNotApplicableCount: number;
  readonly enforcementReadyCount: number;
  readonly openGapCount: number;
  readonly rows: readonly AppGamePlatformProofStatusPanelReadModelRow[];
};

type AppGamePlatformProofStatusPanelReadModelRow = {
  readonly platform: string;
  readonly proofState: string;
  readonly authorityState: string;
  readonly hostCapabilityState: string;
  readonly hostCapabilityEvidenceRefs: readonly string[];
  readonly hostCapabilityProbeRefs: readonly string[];
  readonly adapterDispatchClaimed: boolean;
  readonly broadInstalledAppBlockingClaimed: boolean;
  readonly platformEnforcementClaimed: boolean;
  readonly providerDeliveryClaimed: boolean;
  readonly childDeliveryClaimed: boolean;
  readonly privateDiagnosticsClaimed: boolean;
  readonly proofRefs: readonly string[];
  readonly openGaps: readonly string[];
};

export function createAppGamePlatformProofStatusPanelIntent(
  readModel: AppGamePlatformProofStatusPanelReadModel | null
): AppGamePlatformProofStatusPanelIntent {
  const base = baseIntent();

  if (readModel === null) {
    return {
      ...base,
      loadState: Readable.Unavailable,
      summaryDetails: [
        detail(PortalDetails.Status, Readable.Unavailable),
        detail(PortalDetails.ProductClaim, PlatformProofClaim),
      ],
      rows: [],
    };
  }

  return {
    ...base,
    loadState: readModel.enforcementReadyCount > 0 ? Readable.Ready : Readable.Review,
    summaryDetails: readModelSummary(readModel),
    rows: readModel.rows.map(platformProofStatusRow),
  };
}

function baseIntent() {
  return {
    eyebrow: PortalDetails.RuntimeReference,
    title: decodeDisplayText('App/game platform proof status'),
    body: decodeDisplayText(
      'Parent-safe platform proof status for Windows, Android, Linux, macOS, and iOS evidence and CI-required proof rows.'
    ),
    emptyMessage: decodeDisplayText('No app/game platform proof status has been reported yet.'),
    productClaim: PlatformProofClaim,
  };
}

function readModelSummary(
  readModel: AppGamePlatformProofStatusPanelReadModel
): readonly AppGamePlatformProofStatusPanelDetail[] {
  return [
    detail(PortalDetails.Status, readModel.enforcementReadyCount > 0 ? Readable.Ready : Readable.Review),
    detail(PortalDetails.GeneratedAt, displayText(readModel.generatedAt)),
    detail(PlatformProofs, countText(readModel.returned)),
    detail(HostVisibleRows, countText(readModel.hostVisibleCount)),
    detail(HostNotDetectedRows, countText(readModel.hostNotDetectedCount)),
    detail(NotApplicableRows, countText(readModel.localRuntimeNotApplicableCount)),
    detail(EnforcementReadyRows, countText(readModel.enforcementReadyCount)),
    detail(OpenGaps, countText(readModel.openGapCount)),
    detail(PortalDetails.ProductClaim, PlatformProofClaim),
  ];
}

function platformProofStatusRow(row: AppGamePlatformProofStatusPanelReadModelRow): AppGamePlatformProofStatusPanelRow {
  return {
    title: displayText(row.platform),
    details: [
      detail(PortalDetails.Platform, displayText(row.platform)),
      detail(PortalDetails.Status, displayText(row.proofState)),
      detail(AuthorityState, displayText(row.authorityState)),
      detail(HostCapability, displayText(row.hostCapabilityState)),
      detail(PackageVisibility, joinedOrNotReported(row.hostCapabilityEvidenceRefs)),
      detail(RuntimeVisibility, joinedOrNotReported(row.proofRefs)),
      detail(HostProbeRefs, joinedOrNotReported(row.hostCapabilityProbeRefs)),
      detail(PortalDetails.EvidenceReferences, joinedOrNotReported(row.proofRefs)),
      detail(OpenGaps, joinedOrNotReported(row.openGaps)),
      detail(PortalDetails.AdapterDispatch, claimedValue(row.adapterDispatchClaimed)),
      detail(PortalDetails.Enforcement, claimedValue(row.broadInstalledAppBlockingClaimed)),
      detail(PortalDetails.PlatformState, claimedValue(row.platformEnforcementClaimed)),
      detail(PortalDetails.Provider, claimedValue(row.providerDeliveryClaimed)),
      detail(PortalDetails.ChildDelivery, claimedValue(row.childDeliveryClaimed)),
      detail(PortalDetails.HostCapabilityEvidence, claimedValue(row.privateDiagnosticsClaimed)),
      detail(PortalDetails.ProductClaim, PlatformProofClaim),
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

function detail(label: DisplayText, value: DisplayText): AppGamePlatformProofStatusPanelDetail {
  return {
    label,
    value,
  };
}
