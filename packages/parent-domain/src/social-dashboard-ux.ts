import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  SocialDashboardPanelActionSchema,
  SocialDashboardPanelIdSchema,
  type SocialDashboardPanelKind,
  SocialDashboardPanelKindSchema,
  SocialDashboardPanelReasonSchema,
  SocialDashboardPanelSeveritySchema,
  SocialDashboardPanelStatusSchema,
  SocialDashboardUxSchemaVersionSchema,
} from './social-dashboard-ux-values';

const PositiveSocialDashboardSortOrderSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value >= 0 || 'Expected non-negative social dashboard sort order')
);
const SocialDashboardEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social dashboard evidence refs')
);
const SocialDashboardPanelReasonsSchema = Schema.Array(SocialDashboardPanelReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social dashboard panel reasons')
);

const SocialDashboardPanelBaseSchema = Schema.Struct({
  panelId: SocialDashboardPanelIdSchema,
  panelKind: SocialDashboardPanelKindSchema,
  status: SocialDashboardPanelStatusSchema,
  primaryAction: SocialDashboardPanelActionSchema,
  severity: SocialDashboardPanelSeveritySchema,
  sortOrder: PositiveSocialDashboardSortOrderSchema,
  sourceEvidenceRefs: SocialDashboardEvidenceRefsSchema,
  reasons: SocialDashboardPanelReasonsSchema,
  renderedUiClaimed: Schema.Boolean,
  notificationClaimed: Schema.Boolean,
  runtimeDataFetchClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  connectorAuthorizationClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialDashboardPanelCandidate = Infer<typeof SocialDashboardPanelBaseSchema>;

export const SocialDashboardPanelSchema = withParser(
  SocialDashboardPanelBaseSchema.pipe(
    Schema.filter(
      (panel) =>
        socialDashboardPanelIsHonest(panel) ||
        'Expected social dashboard panel to stay section/action/status contract-only without rendered UI or runtime claims'
    )
  )
);

export const SocialDashboardClaimBoundariesSchema = withParser(
  Schema.Struct({
    renderedPortalUi: Schema.Literal('not-claimed'),
    notificationDelivery: Schema.Literal('not-claimed'),
    runtimeDataFetch: Schema.Literal('not-claimed'),
    policyDecision: Schema.Literal('not-claimed'),
    nativeAppControl: Schema.Literal('not-claimed'),
    connectorAuthorization: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
  })
);

const SocialDashboardUxSnapshotBaseSchema = Schema.Struct({
  schemaVersion: SocialDashboardUxSchemaVersionSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  generatedAt: ParentTimestampSchema,
  panels: Schema.Array(SocialDashboardPanelSchema),
  claimBoundaries: SocialDashboardClaimBoundariesSchema,
});

type SocialDashboardUxSnapshotCandidate = Infer<typeof SocialDashboardUxSnapshotBaseSchema>;

export const SocialDashboardUxSnapshotSchema = withParser(
  SocialDashboardUxSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        socialDashboardUxSnapshotIsHonest(snapshot) ||
        'Expected parent social dashboard UX snapshot to include required sections without rendered UI claims'
    )
  )
);

export const decodeSocialDashboardUxSnapshot = Schema.decodeUnknownSync(SocialDashboardUxSnapshotSchema);

export type SocialDashboardPanel = Infer<typeof SocialDashboardPanelSchema>;
export type SocialDashboardUxSnapshot = Infer<typeof SocialDashboardUxSnapshotSchema>;

const RequiredSocialDashboardPanels = [
  'account-approval-queue',
  'feed-video-gates',
  'native-app-capability',
  'connector-boundaries',
  'decision-memory',
  'settings-custody',
  'manual-required-gaps',
] as const satisfies ReadonlyArray<SocialDashboardPanelKind>;

function socialDashboardUxSnapshotIsHonest(snapshot: SocialDashboardUxSnapshotCandidate): boolean {
  const panelKinds = new Set(snapshot.panels.map((panel) => panel.panelKind));
  return (
    panelKinds.size === snapshot.panels.length &&
    RequiredSocialDashboardPanels.every((panelKind) => panelKinds.has(panelKind))
  );
}

function socialDashboardPanelIsHonest(panel: SocialDashboardPanelCandidate): boolean {
  if (socialDashboardPanelClaimsRuntime(panel)) {
    return false;
  }
  if (panel.panelKind === 'account-approval-queue') {
    return panel.status === 'ready-for-review' && panel.primaryAction === 'open-parent-approval';
  }
  if (panel.panelKind === 'feed-video-gates') {
    return panel.status === 'ready-for-review' && panel.primaryAction === 'review-feed-gate';
  }
  if (panel.panelKind === 'native-app-capability') {
    return manualRequiredPanelIsHonest(panel, 'review-native-capability', 'native-app-manual-required');
  }
  if (panel.panelKind === 'connector-boundaries') {
    return manualRequiredPanelIsHonest(panel, 'review-connector-boundary', 'connector-boundary-manual-required');
  }
  if (panel.panelKind === 'decision-memory') {
    return panel.status === 'contract-only' && panel.primaryAction === 'review-memory-entry';
  }
  if (panel.panelKind === 'settings-custody') {
    return manualRequiredPanelIsHonest(panel, 'review-settings-custody', 'settings-custody-runtime-gap');
  }
  return manualRequiredPanelIsHonest(panel, 'manual-review', 'platform-proof-gap');
}

function manualRequiredPanelIsHonest(
  panel: SocialDashboardPanelCandidate,
  primaryAction: SocialDashboardPanelCandidate['primaryAction'],
  reason: SocialDashboardPanelCandidate['reasons'][number]
): boolean {
  return panel.status === 'manual-required' && panel.primaryAction === primaryAction && panel.reasons.includes(reason);
}

function socialDashboardPanelClaimsRuntime(panel: SocialDashboardPanelCandidate): boolean {
  return (
    panel.renderedUiClaimed ||
    panel.notificationClaimed ||
    panel.runtimeDataFetchClaimed ||
    panel.policyDecisionClaimed ||
    panel.nativeAppControlClaimed ||
    panel.connectorAuthorizationClaimed ||
    panel.enforcementClaimed
  );
}
