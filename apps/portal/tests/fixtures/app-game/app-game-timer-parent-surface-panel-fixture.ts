import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import type { ParentAppGameTimerParentSurfacePanelSnapshot } from '../../../generated/parent-ui-bridge';

export const AppGameTimerParentSurfacePanelFixture: ParentAppGameTimerParentSurfacePanelSnapshot = {
  eyebrow: 'Rust-owned panel',
  title: 'App/game timer parent surface',
  body: 'Rust-owned timer rows stay grounded in stored evidence.',
  loadState: 'ready',
  summaryDetails: [
    { label: 'Timer runtime', value: 'Ready' },
    { label: 'Audit runtime', value: 'Ready' },
    { label: 'Rollback runtime', value: 'Ready' },
    { label: 'Session duration', value: '15 min from stored evidence' },
    { label: 'Control action results', value: '1' },
    { label: 'Control action result refs', value: 'action-result-app-game-1' },
    { label: 'Child UX handoff ready', value: '1' },
    { label: 'Child UX handoff blocked', value: '0' },
    { label: 'Child UX handoff refs', value: 'app-game-child-ux-local-handoff-action-result-app-game-1' },
    { label: PortalDetails.ProductClaim, value: 'Timer runtime remains Rust-owned.' },
  ],
  parentActionRows: [
    {
      title: 'Child UX parent action',
      details: [
        { label: 'Action result ref', value: 'action-result-app-game-1' },
        { label: 'Child reason refs', value: 'reason-app-game-1' },
        { label: 'Child status refs', value: 'status-app-game-1' },
        { label: 'Manual action', value: 'Manual required' },
        { label: 'Delivery', value: 'Not claimed' },
        { label: 'Adapter dispatch', value: 'Not claimed' },
        { label: 'Platform state', value: 'Not claimed' },
      ],
      actionLabel: null,
      actionPayload: null,
    },
  ],
  parentPreferenceSetupRows: [
    {
      title: 'Parent preference setup',
      details: [
        { label: 'Target', value: 'Native app' },
        { label: 'Draft status', value: 'Manual required' },
        { label: 'Parent preference refs', value: 'preference-setup-app-game-1' },
        { label: 'Mutation', value: 'Not claimed' },
        { label: 'Notification delivery', value: 'Not claimed' },
        { label: 'Adapter dispatch', value: 'Not claimed' },
        { label: 'Child delivery', value: 'Not claimed' },
        { label: 'Platform state', value: 'Not claimed' },
      ],
      actionLabel: null,
      actionPayload: null,
    },
  ],
  localHandoffArtifactRows: [
    {
      title: 'action-result-app-game-1',
      details: [
        { label: 'Target', value: 'Native app' },
        { label: 'Child reason refs', value: 'reason-app-game-1' },
        { label: 'Child status refs', value: 'status-app-game-1' },
        { label: 'Delivery', value: 'Not claimed' },
        { label: 'Notification delivery', value: 'Not claimed' },
        { label: 'Adapter dispatch', value: 'Not claimed' },
        { label: 'Platform state', value: 'Not claimed' },
        { label: 'Raw private source rows', value: 'Not claimed' },
      ],
    },
  ],
  rows: [
    {
      title: 'Study Timer',
      details: [
        { label: 'Session duration', value: '15 min' },
        { label: 'Evidence source', value: 'stored journal rows' },
      ],
    },
  ],
  emptyMessage: 'No timer parent surface rows have been reported yet.',
  productClaim: 'Timer runtime, child handoff, and preference setup remain unreported.',
};
