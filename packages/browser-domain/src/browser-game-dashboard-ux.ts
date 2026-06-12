import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  BrowserGameDashboardPanelActionSchema,
  BrowserGameDashboardPanelIdSchema,
  type BrowserGameDashboardPanelKind,
  BrowserGameDashboardPanelKindSchema,
  BrowserGameDashboardPanelReasonSchema,
  BrowserGameDashboardPanelSeveritySchema,
  BrowserGameDashboardPanelStatusSchema,
  BrowserGameDashboardUxSchemaVersionSchema,
} from './browser-game-dashboard-ux-values';

const PositiveBrowserGameDashboardSortOrderSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value >= 0 || 'Expected non-negative browser-game dashboard sort order')
);
const BrowserGameDashboardEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game dashboard evidence refs')
);
const BrowserGameDashboardPanelReasonsSchema = Schema.Array(BrowserGameDashboardPanelReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game dashboard panel reasons')
);
const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);

const BrowserGameDashboardPanelBaseSchema = Schema.Struct({
  panelId: BrowserGameDashboardPanelIdSchema,
  panelKind: BrowserGameDashboardPanelKindSchema,
  status: BrowserGameDashboardPanelStatusSchema,
  primaryAction: BrowserGameDashboardPanelActionSchema,
  severity: BrowserGameDashboardPanelSeveritySchema,
  sortOrder: PositiveBrowserGameDashboardSortOrderSchema,
  sourceEvidenceRefs: BrowserGameDashboardEvidenceRefsSchema,
  approvalRequestRef: OptionalParentEvidenceRefSchema,
  policyCandidateRef: OptionalParentEvidenceRefSchema,
  mobileCapabilityRef: OptionalParentEvidenceRefSchema,
  reasons: BrowserGameDashboardPanelReasonsSchema,
  renderedPortalUiClaimed: Schema.Boolean,
  notificationClaimed: Schema.Boolean,
  runtimeDataFetchClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameDashboardPanelCandidate = Infer<typeof BrowserGameDashboardPanelBaseSchema>;

export const BrowserGameDashboardPanelSchema = withParser(
  BrowserGameDashboardPanelBaseSchema.pipe(
    Schema.filter(
      (panel) =>
        browserGameDashboardPanelIsHonest(panel) ||
        'Expected browser-game dashboard panel to stay section/action/status contract-only without rendered UI, runtime, native game, cloud-frame, or enforcement claims'
    )
  )
);

export const BrowserGameDashboardClaimBoundariesSchema = withParser(
  Schema.Struct({
    renderedPortalUi: Schema.Literal('not-claimed'),
    notificationDelivery: Schema.Literal('not-claimed'),
    runtimeDataFetch: Schema.Literal('not-claimed'),
    finalPolicyDecision: Schema.Literal('not-claimed'),
    cloudFrameAnalysis: Schema.Literal('not-claimed'),
    nativeGameControl: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
  })
);

const BrowserGameDashboardUxSnapshotBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameDashboardUxSchemaVersionSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  generatedAt: ParentTimestampSchema,
  panels: Schema.Array(BrowserGameDashboardPanelSchema),
  claimBoundaries: BrowserGameDashboardClaimBoundariesSchema,
});

type BrowserGameDashboardUxSnapshotCandidate = Infer<typeof BrowserGameDashboardUxSnapshotBaseSchema>;

export const BrowserGameDashboardUxSnapshotSchema = withParser(
  BrowserGameDashboardUxSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        browserGameDashboardUxSnapshotIsHonest(snapshot) ||
        'Expected parent browser-game dashboard UX snapshot to include required sections without rendered UI or runtime claims'
    )
  )
);

export const decodeBrowserGameDashboardUxSnapshot = Schema.decodeUnknownSync(BrowserGameDashboardUxSnapshotSchema);

export type BrowserGameDashboardPanel = Infer<typeof BrowserGameDashboardPanelSchema>;
export type BrowserGameDashboardUxSnapshot = Infer<typeof BrowserGameDashboardUxSnapshotSchema>;

const RequiredBrowserGameDashboardPanels = [
  'detected-game-review',
  'unknown-game-approval-queue',
  'cloud-gaming-approval',
  'educational-game-allowlist',
  'game-time-budget-candidates',
  'mobile-native-capability-gaps',
  'manual-required-gaps',
] as const satisfies ReadonlyArray<BrowserGameDashboardPanelKind>;

type BrowserGameDashboardPanelValidator = (panel: BrowserGameDashboardPanelCandidate) => boolean;

const BrowserGameDashboardPanelValidators = {
  'detected-game-review': detectedGameReviewPanelIsHonest,
  'unknown-game-approval-queue': unknownGameApprovalQueuePanelIsHonest,
  'cloud-gaming-approval': cloudGamingApprovalPanelIsHonest,
  'educational-game-allowlist': educationalGameAllowlistPanelIsHonest,
  'game-time-budget-candidates': gameTimeBudgetCandidatesPanelIsHonest,
  'mobile-native-capability-gaps': mobileNativeCapabilityGapsPanelIsHonest,
  'manual-required-gaps': manualRequiredGapsPanelIsHonest,
} satisfies Record<BrowserGameDashboardPanelKind, BrowserGameDashboardPanelValidator>;

function browserGameDashboardUxSnapshotIsHonest(snapshot: BrowserGameDashboardUxSnapshotCandidate): boolean {
  const panelKinds = new Set(snapshot.panels.map((panel) => panel.panelKind));
  return (
    panelKinds.size === snapshot.panels.length &&
    RequiredBrowserGameDashboardPanels.every((panelKind) => panelKinds.has(panelKind))
  );
}

function browserGameDashboardPanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  if (browserGameDashboardPanelClaimsRuntime(panel)) {
    return false;
  }
  return BrowserGameDashboardPanelValidators[panel.panelKind](panel);
}

function detectedGameReviewPanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  return panel.status === 'ready-for-review' && panel.primaryAction === 'review-detected-game';
}

function unknownGameApprovalQueuePanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  return (
    panel.status === 'ready-for-review' &&
    panel.primaryAction === 'open-parent-approval' &&
    panel.approvalRequestRef !== null
  );
}

function cloudGamingApprovalPanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  return manualRequiredPanelIsHonest(panel, 'review-cloud-gaming', 'cloud-gaming-manual-required');
}

function educationalGameAllowlistPanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  return panel.status === 'contract-only' && panel.primaryAction === 'review-educational-allowlist';
}

function gameTimeBudgetCandidatesPanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  return (
    panel.status === 'contract-only' &&
    panel.primaryAction === 'review-time-budget' &&
    panel.policyCandidateRef !== null
  );
}

function mobileNativeCapabilityGapsPanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  return (
    manualRequiredPanelIsHonest(panel, 'review-mobile-capability', 'mobile-native-proof-gap') &&
    panel.mobileCapabilityRef !== null
  );
}

function manualRequiredGapsPanelIsHonest(panel: BrowserGameDashboardPanelCandidate): boolean {
  return manualRequiredPanelIsHonest(panel, 'manual-review', 'platform-proof-gap');
}

function manualRequiredPanelIsHonest(
  panel: BrowserGameDashboardPanelCandidate,
  primaryAction: BrowserGameDashboardPanelCandidate['primaryAction'],
  reason: BrowserGameDashboardPanelCandidate['reasons'][number]
): boolean {
  return panel.status === 'manual-required' && panel.primaryAction === primaryAction && panel.reasons.includes(reason);
}

function browserGameDashboardPanelClaimsRuntime(panel: BrowserGameDashboardPanelCandidate): boolean {
  return (
    panel.renderedPortalUiClaimed ||
    panel.notificationClaimed ||
    panel.runtimeDataFetchClaimed ||
    panel.finalPolicyDecisionClaimed ||
    panel.cloudFrameAnalysisClaimed ||
    panel.nativeGameControlClaimed ||
    panel.enforcementClaimed
  );
}
