import { PortalDevTextToken, resolvePortalDevText } from './display-text';
import { PortalFormatting } from './formatting';
import { decodePortalDetailValue, type PortalDetailValue } from './portal-contract-text-contracts';
import type { GeneratedPortalTrackingContracts } from './generated-portal-contracts';
import type { TrackingStatusLiveCitation } from './tracking-status-panel';

type GeneratedParentActivityTrackingReadModelSnapshot = NonNullable<
  ReturnType<typeof GeneratedPortalTrackingContracts.ActivityTrackingReadModel.decode>
>;
type GeneratedParentActivityTrackingReadModelRowSnapshot =
  GeneratedParentActivityTrackingReadModelSnapshot['rows'][number];
type GeneratedParentActivityTrackingReadModelCountSnapshot =
  GeneratedParentActivityTrackingReadModelSnapshot['activeKindCounts'][number];
type GeneratedParentActivityTrackingEvidenceReferenceIds =
  GeneratedParentActivityTrackingReadModelRowSnapshot['evidenceReferenceIds'];

export function activeReadModelRows(
  readModel: GeneratedParentActivityTrackingReadModelSnapshot
): readonly GeneratedParentActivityTrackingReadModelRowSnapshot[] {
  return readModel.rows.filter((rowValue) => rowValue.deletedAt === null);
}

export function activeReadModelEvidenceReferences(
  readModel: GeneratedParentActivityTrackingReadModelSnapshot
): PortalDetailValue {
  const references = new Set<GeneratedParentActivityTrackingEvidenceReferenceIds[number]>();
  for (const rowValue of activeReadModelRows(readModel)) {
    for (const evidenceReferenceId of rowValue.evidenceReferenceIds) {
      references.add(evidenceReferenceId);
    }
  }
  return evidenceReferenceDetail([...references]);
}

export function readModelDeletedEvidenceReferences(
  readModel: GeneratedParentActivityTrackingReadModelSnapshot
): PortalDetailValue {
  const references = new Set<GeneratedParentActivityTrackingEvidenceReferenceIds[number]>();
  for (const rowValue of readModel.rows) {
    for (const evidenceReferenceId of rowValue.deletedEvidenceReferenceIds) {
      references.add(evidenceReferenceId);
    }
  }
  for (const evidenceReferenceId of readModel.deletedEvidenceReferenceIds) {
    references.add(evidenceReferenceId);
  }
  return evidenceReferenceDetail([...references]);
}

export function liveCitation(
  rowValue: GeneratedParentActivityTrackingReadModelRowSnapshot
): TrackingStatusLiveCitation {
  return {
    title: detailFromValue(rowValue.subjectDisplayName ?? rowValue.kind),
    eventId: detailFromValue(rowValue.eventId),
    observedAt: detailFromValue(rowValue.observedAt),
    device: detailFromValue(rowValue.deviceId),
    platform: detailFromValue(rowValue.platform),
    observer: detailFromValue(rowValue.observer),
    activityKind: detailFromValue(rowValue.kind),
    subject: detailFromValue([rowValue.subjectKind, rowValue.subjectId].join(PortalFormatting.EventDetailSeparator)),
    status: detailFromValue(
      [rowValue.queryVisibility, rowValue.capabilityStatus]
        .filter((part) => part !== null && part !== undefined)
        .join(PortalFormatting.EventDetailSeparator)
    ),
    evidenceReferences: evidenceReferenceDetail(rowValue.evidenceReferenceIds),
    deletedEvidence: evidenceReferenceDetail(rowValue.deletedEvidenceReferenceIds),
    productClaim: resolvePortalDevText(PortalDevTextToken.TrackingNoProductClaim),
  };
}

export function listDetail(values: readonly unknown[]): PortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  return evidenceReferenceDetail([...new Set(normalizedValues)]);
}

export function countDetail(
  values: readonly GeneratedParentActivityTrackingReadModelCountSnapshot[]
): PortalDetailValue {
  if (values.length === 0) {
    return notReported();
  }
  return detailFromValue(
    values.map((value) => `${value.value} (${String(value.count)})`).join(PortalFormatting.EventDetailSeparator)
  );
}

export function readModelActivityKindCoverage(
  readModel: GeneratedParentActivityTrackingReadModelSnapshot
): PortalDetailValue {
  const activeKindCounts = readModel.activeKindCounts ?? [];
  if (activeKindCounts.length > 0) {
    return countDetail(activeKindCounts);
  }
  return listDetail(activeReadModelRows(readModel).map((rowValue) => rowValue.kind));
}

export function readModelDeviceCoverage(
  readModel: GeneratedParentActivityTrackingReadModelSnapshot
): PortalDetailValue {
  const activeDeviceCounts = readModel.activeDeviceCounts ?? [];
  if (activeDeviceCounts.length > 0) {
    return countDetail(activeDeviceCounts);
  }
  return listDetail(activeReadModelRows(readModel).map((rowValue) => rowValue.deviceId));
}

export function readModelCapabilityCoverage(
  readModel: GeneratedParentActivityTrackingReadModelSnapshot
): PortalDetailValue {
  const activeCapabilityStatusCounts = readModel.activeCapabilityStatusCounts ?? [];
  if (activeCapabilityStatusCounts.length > 0) {
    return countDetail(activeCapabilityStatusCounts);
  }
  return detailFromValue(readModel.capabilityStatus);
}

export function sequenceDetail(values: readonly unknown[]): PortalDetailValue {
  const normalizedValues = values.map((value) => String(value)).filter((value) => value.length > 0);
  if (normalizedValues.length === 0) {
    return notReported();
  }
  return detailFromValue(normalizedValues.join(PortalFormatting.EventDetailSeparator));
}

function detailFromValue(value: unknown): PortalDetailValue {
  if (value === undefined || value === null) {
    return notReported();
  }
  return decodePortalDetailValue(String(value));
}

function notReported(): PortalDetailValue {
  return detailFromValue(resolvePortalDevText(PortalDevTextToken.NotReported));
}

function evidenceReferenceDetail(evidenceReferenceIds: readonly string[] | undefined): PortalDetailValue {
  if (evidenceReferenceIds === undefined || evidenceReferenceIds.length === 0) {
    return notReported();
  }
  return detailFromValue(evidenceReferenceIds.join(PortalFormatting.EventDetailSeparator));
}
