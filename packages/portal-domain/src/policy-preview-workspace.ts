import type { PortalPolicyPreviewReadModel } from '@ocentra-parent/schema-domain/agent-policy-preview-read-model';
import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { createPolicyPreviewPanelIntent, type PolicyPreviewPanelDetail } from './policy-preview-panel';

export type PolicyWorkspacePreviewTone = 'cyan' | 'gold' | 'purple' | 'red';

export type PolicyWorkspacePreviewRow = {
  readonly label: DisplayText;
  readonly value: DisplayText;
  readonly body: DisplayText;
  readonly tone: PolicyWorkspacePreviewTone;
};

const WorkspaceRowLabel = {
  Preview: decodeDisplayText('Preview'),
  Approval: decodeDisplayText('Approval'),
  Lifecycle: decodeDisplayText('Lifecycle'),
  Boundary: decodeDisplayText('Boundary'),
} as const;

const WorkspaceDetailLabel = {
  TargetValue: decodeDisplayText('Target value'),
  DecisionAction: decodeDisplayText('Decision action'),
  TargetState: decodeDisplayText('Target state'),
  FindingKinds: decodeDisplayText('Finding kinds'),
  RequestOrigin: decodeDisplayText('Request origin'),
  AssistantConfirmation: decodeDisplayText('Assistant confirmation'),
  RequestStatus: decodeDisplayText('Request status'),
  SourceStatus: decodeDisplayText('Source status'),
  SourceSurface: decodeDisplayText('Source surface'),
} as const;

const NotReported = decodeDisplayText('Not reported');
const AdvisoryOnly = decodeDisplayText('Advisory only');
const ApprovalGuard = decodeDisplayText('Parent confirmation is required before any write.');
const LifecycleGuard = decodeDisplayText('Delivery and acknowledgment stay separate from active enforcement.');
const MissingPreviewBody = decodeDisplayText(
  'Refresh the policy preview read model before making parent authoring claims.'
);
const BoundaryUnavailableBody = decodeDisplayText(
  'Preview remains parent-surface only until a typed confirmation and adapter-owned handoff exist.'
);

export function createPolicyWorkspacePreviewRows(
  readModel: PortalPolicyPreviewReadModel | null
): readonly PolicyWorkspacePreviewRow[] {
  const intent = createPolicyPreviewPanelIntent(null, readModel);
  if (readModel === null || readModel.returned === 0) {
    return [
      previewRow(WorkspaceRowLabel.Preview, intent.summary, MissingPreviewBody, 'red'),
      previewRow(WorkspaceRowLabel.Approval, NotReported, ApprovalGuard, 'purple'),
      previewRow(WorkspaceRowLabel.Lifecycle, NotReported, LifecycleGuard, 'gold'),
      previewRow(WorkspaceRowLabel.Boundary, AdvisoryOnly, BoundaryUnavailableBody, 'purple'),
    ];
  }

  const previewDetails = intent.cards[0]?.details ?? [];
  const sourceDetails = intent.cards[1]?.details ?? [];

  const previewValue = intent.summary;
  const previewBody = joinDisplayTextParts([
    detailValue(previewDetails, WorkspaceDetailLabel.TargetValue),
    detailValue(previewDetails, WorkspaceDetailLabel.DecisionAction),
    detailValue(previewDetails, WorkspaceDetailLabel.TargetState),
    evidenceGradeValue(readModel),
  ]);

  const approvalValue = firstKnownDetail(
    previewDetails,
    WorkspaceDetailLabel.AssistantConfirmation,
    WorkspaceDetailLabel.RequestStatus
  );
  const approvalBody = joinDisplayTextParts([
    detailValue(previewDetails, WorkspaceDetailLabel.RequestOrigin),
    detailValue(previewDetails, WorkspaceDetailLabel.RequestStatus),
    ApprovalGuard,
  ]);

  const lifecycleValue = detailValue(sourceDetails, WorkspaceDetailLabel.SourceStatus);
  const lifecycleBody = joinDisplayTextParts([
    detailValue(sourceDetails, WorkspaceDetailLabel.SourceSurface),
    LifecycleGuard,
  ]);

  return [
    previewRow(WorkspaceRowLabel.Preview, previewValue, previewBody, previewTone(previewValue)),
    previewRow(WorkspaceRowLabel.Approval, approvalValue, approvalBody, approvalTone(approvalValue)),
    previewRow(WorkspaceRowLabel.Lifecycle, lifecycleValue, lifecycleBody, lifecycleTone(lifecycleValue)),
    previewRow(WorkspaceRowLabel.Boundary, AdvisoryOnly, intent.productClaim, 'purple'),
  ];
}

function previewRow(
  label: DisplayText,
  value: DisplayText,
  body: DisplayText,
  tone: PolicyWorkspacePreviewTone
): PolicyWorkspacePreviewRow {
  return {
    label,
    value,
    body,
    tone,
  };
}

function detailValue(
  details: readonly PolicyPreviewPanelDetail[],
  label: DisplayText
): DisplayText {
  const match = details.find((detail) => String(detail.label) === String(label));
  return match?.value ?? NotReported;
}

function firstKnownDetail(
  details: readonly PolicyPreviewPanelDetail[],
  primaryLabel: DisplayText,
  fallbackLabel: DisplayText
): DisplayText {
  const primaryValue = detailValue(details, primaryLabel);
  if (String(primaryValue) !== String(NotReported)) {
    return primaryValue;
  }
  return detailValue(details, fallbackLabel);
}

function evidenceGradeValue(readModel: PortalPolicyPreviewReadModel): DisplayText {
  if (readModel.networkEvidenceGrade === null) {
    return decodeDisplayText('Proof tier not reported');
  }
  return decodeDisplayText(`Proof tier ${String(readModel.networkEvidenceGrade)}`);
}

function joinDisplayTextParts(parts: readonly DisplayText[]): DisplayText {
  const visibleParts = parts
    .map((part) => String(part).trim())
    .filter((part) => part.length > 0 && part !== String(NotReported));
  if (visibleParts.length === 0) {
    return NotReported;
  }
  return decodeDisplayText(visibleParts.join(' | '));
}

function previewTone(value: DisplayText): PolicyWorkspacePreviewTone {
  return toneFromText(value, 'gold');
}

function approvalTone(value: DisplayText): PolicyWorkspacePreviewTone {
  return toneFromText(value, 'purple');
}

function lifecycleTone(value: DisplayText): PolicyWorkspacePreviewTone {
  return toneFromText(value, 'gold');
}

function toneFromText(value: DisplayText, fallback: PolicyWorkspacePreviewTone): PolicyWorkspacePreviewTone {
  const normalizedValue = String(value).toLowerCase();
  if (
    normalizedValue.includes('blocked') ||
    normalizedValue.includes('denied') ||
    normalizedValue.includes('expired') ||
    normalizedValue.includes('rejected') ||
    normalizedValue.includes('unavailable')
  ) {
    return 'red';
  }
  if (normalizedValue.includes('active') || normalizedValue.includes('approved') || normalizedValue.includes('ready')) {
    return 'cyan';
  }
  if (
    normalizedValue.includes('preview') ||
    normalizedValue.includes('confirmation') ||
    normalizedValue.includes('draft') ||
    normalizedValue.includes('pending')
  ) {
    return 'purple';
  }
  return fallback;
}
