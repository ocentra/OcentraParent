import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';
import type { PortalShellParentAccessState } from './parent-portal-shell-status';

const ReasonField = 'reason';

const PreviewState = {
  SaveState: {
    PreviewRequired: 'preview-required',
    ReadyToSave: 'ready-to-save',
    Blocked: 'blocked',
  },
  ManualReviewState: {
    Required: 'required',
    NotRequired: 'not-required',
  },
  TargetState: {
    Supported: 'supported',
    Unsupported: 'unsupported',
    ManualRequired: 'manual-required',
    Offline: 'offline',
    Stale: 'stale',
  },
  SourceStatus: {
    Draft: 'draft',
    Preview: 'preview',
    Confirmed: 'confirmed',
    Queued: 'queued',
    Delivered: 'delivered',
    Acknowledged: 'acknowledged',
    Active: 'active',
    PartiallyActive: 'partially-active',
    Rejected: 'rejected',
    Superseded: 'superseded',
    RolledBack: 'rolled-back',
    Stale: 'stale',
    Expired: 'expired',
    ManualRequired: 'manual-required',
  },
  SourceSurface: {
    ParentPortal: 'parent-portal',
    ParentCompanion: 'parent-companion',
    AiPreview: 'ai-preview',
    DomainCache: 'domain-cache',
  },
  RequestOrigin: {
    Child: 'child',
    AssistantDraft: 'assistant-draft',
  },
  AssistantConfirmationState: {
    NotRequired: 'not-required',
    ParentConfirmationRequired: 'parent-confirmation-required',
    ParentConfirmed: 'parent-confirmed',
  },
  RequestStatus: {
    PreviewOnly: 'preview-only',
    PendingParentReview: 'pending-parent-review',
    Approved: 'approved',
    Denied: 'denied',
    Modified: 'modified',
    Expired: 'expired',
    ReplayRejected: 'replay-rejected',
  },
} as const;

const PreviewDetails = {
  SaveState: decodeDisplayText('Save state'),
  TargetState: decodeDisplayText('Target state'),
  TargetExplanationCode: decodeDisplayText('Target explanation code'),
  FindingKinds: decodeDisplayText('Finding kinds'),
  SourceStatus: decodeDisplayText('Source status'),
  SourceSurface: decodeDisplayText('Source surface'),
  RequestOrigin: decodeDisplayText('Request origin'),
  AssistantConfirmation: decodeDisplayText('Assistant confirmation'),
  RequestStatus: decodeDisplayText('Request status'),
  ApprovalId: decodeDisplayText('Approval ID'),
  OverrideId: decodeDisplayText('Override ID'),
  ReplayOfApproval: decodeDisplayText('Replay of approval'),
  ReviewedBy: decodeDisplayText('Reviewed by'),
  ReviewedAt: decodeDisplayText('Reviewed at'),
  AuditReference: decodeDisplayText('Audit reference'),
  ParentAccess: decodeDisplayText('Parent access'),
  WriteAuthority: decodeDisplayText('Write authority'),
} as const;

const PreviewReadableValues: Readonly<Record<string, DisplayText>> = {
  'preview-required': decodeDisplayText('Preview required'),
  'ready-to-save': decodeDisplayText('Ready to save'),
  blocked: decodeDisplayText('Blocked'),
  required: decodeDisplayText('Required'),
  'not-required': decodeDisplayText('Not required'),
  supported: decodeDisplayText('Supported'),
  unsupported: decodeDisplayText('Unsupported'),
  'manual-required': decodeDisplayText('Manual required'),
  offline: decodeDisplayText('Offline'),
  stale: decodeDisplayText('Stale'),
  draft: decodeDisplayText('Draft'),
  preview: decodeDisplayText('Preview'),
  confirmed: decodeDisplayText('Confirmed'),
  queued: decodeDisplayText('Queued'),
  delivered: decodeDisplayText('Delivered'),
  acknowledged: decodeDisplayText('Acknowledged'),
  active: decodeDisplayText('Active'),
  'partially-active': decodeDisplayText('Partially active'),
  rejected: decodeDisplayText('Rejected'),
  superseded: decodeDisplayText('Superseded'),
  'rolled-back': decodeDisplayText('Rolled back'),
  expired: decodeDisplayText('Expired'),
  'parent-portal': decodeDisplayText('Parent portal'),
  'parent-companion': decodeDisplayText('Parent companion'),
  'ai-preview': decodeDisplayText('AI preview'),
  'domain-cache': decodeDisplayText('Domain cache'),
  child: decodeDisplayText('Child'),
  'assistant-draft': decodeDisplayText('Assistant draft'),
  'parent-confirmation-required': decodeDisplayText('Parent confirmation required'),
  'parent-confirmed': decodeDisplayText('Parent confirmed'),
  'preview-only': decodeDisplayText('Preview only'),
  'pending-parent-review': decodeDisplayText('Pending parent review'),
  approved: decodeDisplayText('Approved'),
  denied: decodeDisplayText('Denied'),
  modified: decodeDisplayText('Modified'),
  'replay-rejected': decodeDisplayText('Replay rejected'),
} as const;

const ProductClaim = decodeDisplayText(
  'Policy preview is advisory parent-surface state only. It does not claim enforcement, adapter execution, provider delivery, or child-device application.'
);
const ParentAccessReadableValues: Readonly<Record<PortalShellParentAccessState, DisplayText>> = {
  'active-controller': decodeDisplayText('Active controller'),
  'observer-only': decodeDisplayText('Observer only'),
  unauthenticated: decodeDisplayText('Unauthenticated'),
  'proof-missing': decodeDisplayText('Proof missing'),
} as const;

export type PolicyPreviewPanelEvent = {
  readonly payload: Record<string, unknown>;
};

export type PolicyPreviewPanelReadModel = {
  readonly returned: number;
  readonly previewId: string | null;
  readonly targetType: string | null;
  readonly targetValue: string | null;
  readonly decisionAction: string | null;
  readonly parentRuleContextReferenceCount: number | null;
  readonly parentRuleContextRefIds: string | null;
  readonly dryRun: boolean | null;
  readonly policyPreviewSaveState?: string | null;
  readonly policyPreviewManualReviewState?: string | null;
  readonly policyPreviewTargetState?: string | null;
  readonly policyPreviewTargetExplanationCode?: string | null;
  readonly policyPreviewFindingKinds?: string | null;
  readonly policySourceStatus?: string | null;
  readonly policySourceSurface?: string | null;
  readonly policyRequestOrigin?: string | null;
  readonly policyAssistantConfirmationState?: string | null;
  readonly policyRequestStatus?: string | null;
  readonly policyApprovalId?: string | null;
  readonly policyOverrideId?: string | null;
  readonly policyReplayOfApprovalId?: string | null;
  readonly policyReviewedByActorId?: string | null;
  readonly policyReviewedByActorRole?: string | null;
  readonly policyReviewedAt?: string | null;
  readonly policyAuditReferenceId?: string | null;
};

export type PolicyPreviewPanelDetail = {
  readonly label: DisplayText;
  readonly value: DisplayText;
};

export type PolicyPreviewPanelCard = {
  readonly title: DisplayText;
  readonly summary: DisplayText;
  readonly details: readonly PolicyPreviewPanelDetail[];
};

export type PolicyPreviewPanelIntent = {
  readonly title: DisplayText;
  readonly body: DisplayText;
  readonly summary: DisplayText;
  readonly summaryDetails: readonly PolicyPreviewPanelDetail[];
  readonly cards: readonly PolicyPreviewPanelCard[];
  readonly emptyMessage: DisplayText;
  readonly productClaim: DisplayText;
};

export function createPolicyPreviewPanelIntent(
  event: PolicyPreviewPanelEvent | null,
  readModel: PolicyPreviewPanelReadModel | null,
  parentAccessState: PortalShellParentAccessState = 'proof-missing'
): PolicyPreviewPanelIntent {
  const base = baseIntent();

  if (readModel === null) {
    const summary = unavailableSummary(event);
    return {
      ...base,
      summary,
      summaryDetails: [
        detail(PortalDetails.Status, requiredReadableValue('unavailable')),
        detail(PortalDetails.Reason, summary),
        detail(PreviewDetails.ParentAccess, parentAccessReadableValue(parentAccessState)),
        ...boundaryDetails(),
        detail(PortalDetails.ProductClaim, ProductClaim),
      ],
      cards: [accessCard(parentAccessState, null), boundaryCard()],
    };
  }

  if (readModel.returned === 0) {
    const summary = decodeDisplayText('No policy preview rows have been reported yet.');
    return {
      ...base,
      summary,
      summaryDetails: [
        detail(PortalDetails.Status, requiredReadableValue('unavailable')),
        detail(PortalDetails.Reason, summary),
        detail(PreviewDetails.ParentAccess, parentAccessReadableValue(parentAccessState)),
        ...boundaryDetails(),
        detail(PortalDetails.ProductClaim, ProductClaim),
      ],
      cards: [accessCard(parentAccessState, null), boundaryCard()],
    };
  }

  return {
    ...base,
    summary: previewSummary(readModel),
    summaryDetails: [
      detail(PortalDetails.PreviewStatus, previewSummary(readModel)),
      detail(PortalDetails.PolicyPreview, optionalValue(readModel.previewId)),
      detail(
        PortalDetails.ParentRuleContextReferences,
        optionalValue(readModel.parentRuleContextReferenceCount?.toString() ?? null)
      ),
      detail(PortalDetails.ParentRuleContextRefIds, optionalValue(readModel.parentRuleContextRefIds)),
      detail(PreviewDetails.ParentAccess, parentAccessReadableValue(parentAccessState)),
      ...boundaryDetails(),
      detail(PortalDetails.ProductClaim, ProductClaim),
    ],
    cards: [previewCard(readModel), sourceCard(readModel), accessCard(parentAccessState, readModel), boundaryCard()],
  };
}

function baseIntent() {
  return {
    title: decodeDisplayText('Policy preview parent authoring'),
    body: decodeDisplayText(
      'Preview stays advisory until a parent confirms the request and a child-device contract applies it.'
    ),
    emptyMessage: decodeDisplayText('No policy preview has been reported yet.'),
    productClaim: ProductClaim,
  };
}

function previewCard(readModel: PolicyPreviewPanelReadModel): PolicyPreviewPanelCard {
  return {
    title: decodeDisplayText('Preview state'),
    summary: previewSummary(readModel),
    details: [
      detail(PortalDetails.TargetType, optionalValue(readModel.targetType)),
      detail(PortalDetails.TargetValue, optionalValue(readModel.targetValue)),
      detail(PortalDetails.DecisionAction, optionalValue(readModel.decisionAction)),
      detail(PreviewDetails.SaveState, previewReadableValue(readModel.policyPreviewSaveState ?? null)),
      detail(
        PortalDetails.ManualReview,
        previewReadableValue(readModel.policyPreviewManualReviewState ?? null)
      ),
      detail(PreviewDetails.TargetState, previewReadableValue(readModel.policyPreviewTargetState ?? null)),
      detail(
        PreviewDetails.TargetExplanationCode,
        optionalValue(readModel.policyPreviewTargetExplanationCode ?? null)
      ),
      detail(PreviewDetails.FindingKinds, optionalValue(readModel.policyPreviewFindingKinds ?? null)),
      detail(PreviewDetails.RequestOrigin, previewReadableValue(readModel.policyRequestOrigin ?? null)),
      detail(
        PreviewDetails.AssistantConfirmation,
        previewReadableValue(readModel.policyAssistantConfirmationState ?? null)
      ),
      detail(PreviewDetails.RequestStatus, previewReadableValue(readModel.policyRequestStatus ?? null)),
      detail(PreviewDetails.ApprovalId, optionalValue(readModel.policyApprovalId ?? null)),
      detail(PreviewDetails.OverrideId, optionalValue(readModel.policyOverrideId ?? null)),
      detail(PreviewDetails.ReplayOfApproval, optionalValue(readModel.policyReplayOfApprovalId ?? null)),
      detail(PreviewDetails.ReviewedBy, reviewedByValue(readModel)),
      detail(PreviewDetails.ReviewedAt, optionalValue(readModel.policyReviewedAt ?? null)),
      detail(PreviewDetails.AuditReference, optionalValue(readModel.policyAuditReferenceId ?? null)),
    ],
  };
}

function sourceCard(readModel: PolicyPreviewPanelReadModel): PolicyPreviewPanelCard {
  const sourceStatus = readModel.policySourceStatus ?? null;
  if (sourceStatus === null) {
    return {
      title: decodeDisplayText('Source lifecycle'),
      summary: decodeDisplayText('No source lifecycle has been reported.'),
      details: [
        detail(PreviewDetails.SourceStatus, optionalValue(null)),
        detail(PreviewDetails.SourceSurface, optionalValue(readModel.policySourceSurface ?? null)),
      ],
    };
  }

  return {
    title: decodeDisplayText('Source lifecycle'),
    summary: sourceLifecycleSummary(sourceStatus),
    details: [
      detail(PreviewDetails.SourceStatus, previewReadableValue(sourceStatus)),
      detail(PreviewDetails.SourceSurface, previewReadableValue(readModel.policySourceSurface ?? null)),
    ],
  };
}

function accessCard(
  parentAccessState: PortalShellParentAccessState,
  readModel: PolicyPreviewPanelReadModel | null
): PolicyPreviewPanelCard {
  return {
    title: decodeDisplayText('Approval authority'),
    summary: accessSummary(parentAccessState, readModel),
    details: [
      detail(PreviewDetails.ParentAccess, parentAccessReadableValue(parentAccessState)),
      detail(
        PreviewDetails.AssistantConfirmation,
        previewReadableValue(readModel?.policyAssistantConfirmationState ?? null)
      ),
      detail(PreviewDetails.RequestStatus, previewReadableValue(readModel?.policyRequestStatus ?? null)),
      detail(PreviewDetails.ApprovalId, optionalValue(readModel?.policyApprovalId ?? null)),
      detail(PreviewDetails.OverrideId, optionalValue(readModel?.policyOverrideId ?? null)),
      detail(PreviewDetails.ReplayOfApproval, optionalValue(readModel?.policyReplayOfApprovalId ?? null)),
      detail(PreviewDetails.ReviewedBy, reviewedByValue(readModel)),
      detail(PreviewDetails.ReviewedAt, optionalValue(readModel?.policyReviewedAt ?? null)),
      detail(PreviewDetails.AuditReference, optionalValue(readModel?.policyAuditReferenceId ?? null)),
      detail(PreviewDetails.WriteAuthority, accessWriteAuthority(parentAccessState, readModel)),
    ],
  };
}

function boundaryCard(): PolicyPreviewPanelCard {
  return {
    title: decodeDisplayText('Boundary'),
    summary: decodeDisplayText('No enforcement claim'),
    details: [...boundaryDetails(), detail(PortalDetails.ProductClaim, ProductClaim)],
  };
}

function previewSummary(readModel: PolicyPreviewPanelReadModel): DisplayText {
  if (
    readModel.policyRequestOrigin === PreviewState.RequestOrigin.AssistantDraft &&
    readModel.policyAssistantConfirmationState !==
      PreviewState.AssistantConfirmationState.ParentConfirmed
  ) {
    return decodeDisplayText('Assistant draft remains preview-only until parent confirmation.');
  }

  if (readModel.policyPreviewSaveState === PreviewState.SaveState.Blocked || hasConflictFinding(readModel)) {
    return decodeDisplayText('Preview is blocked and conflict details stay visible for parent review.');
  }

  if (
    readModel.policyPreviewManualReviewState === PreviewState.ManualReviewState.Required ||
    readModel.policyPreviewTargetState === PreviewState.TargetState.Unsupported ||
    readModel.policyPreviewTargetState === PreviewState.TargetState.ManualRequired ||
    readModel.policyPreviewTargetState === PreviewState.TargetState.Offline ||
    readModel.policyPreviewTargetState === PreviewState.TargetState.Stale
  ) {
    return decodeDisplayText('Preview stays visible, but it is not ready to save.');
  }

  if (readModel.policyPreviewSaveState === PreviewState.SaveState.ReadyToSave) {
    return decodeDisplayText('Preview is ready to save, but it is still not enforced.');
  }

  return decodeDisplayText('Preview remains advisory and not enforced.');
}

function sourceLifecycleSummary(sourceStatus: string): DisplayText {
  if (sourceStatus === PreviewState.SourceStatus.Delivered) {
    return decodeDisplayText('Delivered is reported, but active enforcement is separate.');
  }
  if (sourceStatus === PreviewState.SourceStatus.Acknowledged) {
    return decodeDisplayText('Acknowledged delivery is reported, but active enforcement is separate.');
  }
  if (
    sourceStatus === PreviewState.SourceStatus.Active ||
    sourceStatus === PreviewState.SourceStatus.PartiallyActive
  ) {
    return decodeDisplayText('Active lifecycle is adapter-owned and stays distinct from delivery or audit claims.');
  }
  return displayText(`Source lifecycle: ${String(previewReadableValue(sourceStatus))}`);
}

function accessSummary(
  parentAccessState: PortalShellParentAccessState,
  readModel: PolicyPreviewPanelReadModel | null
): DisplayText {
  if (parentAccessState === 'observer-only') {
    return decodeDisplayText('Observer-only parents can review policy explanation but cannot confirm or save writes.');
  }
  if (parentAccessState === 'unauthenticated') {
    return decodeDisplayText('Sign-in is required before reviewing or confirming policy changes.');
  }
  if (parentAccessState === 'proof-missing') {
    return decodeDisplayText('Parent authority proof is missing, so the portal cannot claim write permission.');
  }
  if (readModel?.policyRequestStatus === PreviewState.RequestStatus.ReplayRejected) {
    return decodeDisplayText('The latest approval attempt was rejected as a replay, so no new override was created.');
  }
  if (readModel?.policyAssistantConfirmationState === PreviewState.AssistantConfirmationState.ParentConfirmationRequired) {
    return decodeDisplayText('Controller authority is present, but parent confirmation is still required before any write.');
  }
  if (
    readModel?.policyAuditReferenceId !== null &&
    readModel?.policyAuditReferenceId !== undefined &&
    (readModel?.policyRequestStatus === PreviewState.RequestStatus.Approved ||
      readModel?.policyRequestStatus === PreviewState.RequestStatus.Modified ||
      readModel?.policyRequestStatus === PreviewState.RequestStatus.Denied)
  ) {
    return decodeDisplayText('Controller review is recorded with reviewer and audit details, but delivery and enforcement remain separate states.');
  }
  if (
    readModel?.policyAssistantConfirmationState === PreviewState.AssistantConfirmationState.ParentConfirmed ||
    readModel?.policyRequestStatus === PreviewState.RequestStatus.Approved
  ) {
    return decodeDisplayText('Controller confirmation is recorded, but delivery and enforcement remain separate states.');
  }
  return decodeDisplayText('Controller authority is present, but the portal still treats this policy path as preview-only.');
}

function accessWriteAuthority(
  parentAccessState: PortalShellParentAccessState,
  readModel: PolicyPreviewPanelReadModel | null
): DisplayText {
  if (parentAccessState === 'observer-only') {
    return decodeDisplayText('Observer scope is read-only and cannot confirm or save policy writes.');
  }
  if (parentAccessState === 'unauthenticated') {
    return decodeDisplayText('Sign-in required before any review or confirmation action.');
  }
  if (parentAccessState === 'proof-missing') {
    return decodeDisplayText('Write authority is unavailable until household role proof is visible.');
  }
  if (readModel?.policyAssistantConfirmationState === PreviewState.AssistantConfirmationState.ParentConfirmed) {
    return decodeDisplayText('Parent-confirmed preview is visible, but the portal still has no typed write command.');
  }
  if (readModel?.policyAssistantConfirmationState === PreviewState.AssistantConfirmationState.ParentConfirmationRequired) {
    return decodeDisplayText('Parent confirmation is required before any write.');
  }
  return decodeDisplayText('Preview-only route; no typed write command is exposed from this surface.');
}

function hasConflictFinding(readModel: PolicyPreviewPanelReadModel): boolean {
  const findingKinds = readModel.policyPreviewFindingKinds?.toLowerCase() ?? '';
  const explanationCode = readModel.policyPreviewTargetExplanationCode?.toLowerCase() ?? '';
  return findingKinds.includes('conflict') || explanationCode.includes('conflict');
}

function unavailableSummary(event: PolicyPreviewPanelEvent | null): DisplayText {
  const reason = event?.payload[ReasonField];
  if (typeof reason === 'string' && reason.length > 0) {
    return displayText(reason);
  }
  return decodeDisplayText('Policy preview read-model is unavailable.');
}

function boundaryDetails(): readonly PolicyPreviewPanelDetail[] {
  return [
    detail(PortalDetails.PrivacyMode, requiredReadableValue('local-only')),
    detail(PortalDetails.AdapterBoundary, requiredReadableValue('local-adapter-unavailable')),
    detail(PortalDetails.ExecutionState, requiredReadableValue('disabled')),
    detail(PortalDetails.ProviderSource, requiredReadableValue('unavailable')),
  ];
}

function previewReadableValue(value: string | null): DisplayText {
  if (value === null) {
    return optionalValue(null);
  }
  return PreviewReadableValues[value] ?? PortalReadableValues[value] ?? displayText(value);
}

function parentAccessReadableValue(value: PortalShellParentAccessState): DisplayText {
  return ParentAccessReadableValues[value];
}

function reviewedByValue(readModel: PolicyPreviewPanelReadModel | null): DisplayText {
  const actorId = readModel?.policyReviewedByActorId ?? null;
  const actorRole = readModel?.policyReviewedByActorRole ?? null;
  if (actorId === null && actorRole === null) {
    return optionalValue(null);
  }
  if (actorId !== null && actorRole !== null) {
    return displayText(`${actorId} (${actorRole})`);
  }
  return optionalValue(actorId ?? actorRole);
}

function optionalValue(value: string | null): DisplayText {
  if (value === null || value.length === 0) {
    return decodeDisplayText('Not reported');
  }
  return displayText(value);
}

function requiredReadableValue(key: string): DisplayText {
  const value = PortalReadableValues[key];
  if (value === undefined) {
    throw new Error(`Missing portal readable value: ${key}`);
  }
  return value;
}

function displayText(value: string): DisplayText {
  return decodeDisplayText(value);
}

function detail(label: DisplayText, value: DisplayText): PolicyPreviewPanelDetail {
  return {
    label,
    value,
  };
}
