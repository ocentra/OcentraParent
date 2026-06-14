import { decodeDisplayText, type DisplayText } from '@ocentra-parent/text-domain/contracts';
import { PortalDetails, PortalReadableValues } from './details';

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
} as const;

const ProductClaim = decodeDisplayText(
  'Policy preview is advisory parent-surface state only. It does not claim enforcement, adapter execution, provider delivery, or child-device application.'
);

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
  readModel: PolicyPreviewPanelReadModel | null
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
        ...boundaryDetails(),
        detail(PortalDetails.ProductClaim, ProductClaim),
      ],
      cards: [boundaryCard()],
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
        ...boundaryDetails(),
        detail(PortalDetails.ProductClaim, ProductClaim),
      ],
      cards: [boundaryCard()],
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
      ...boundaryDetails(),
      detail(PortalDetails.ProductClaim, ProductClaim),
    ],
    cards: [previewCard(readModel), sourceCard(readModel), boundaryCard()],
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
    summary: displayText(`Source lifecycle: ${String(previewReadableValue(sourceStatus))}`),
    details: [
      detail(PreviewDetails.SourceStatus, previewReadableValue(sourceStatus)),
      detail(PreviewDetails.SourceSurface, previewReadableValue(readModel.policySourceSurface ?? null)),
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
