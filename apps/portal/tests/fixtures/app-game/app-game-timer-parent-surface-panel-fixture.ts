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
    { label: PortalDetails.ProductClaim, value: 'Timer runtime remains Rust-owned.' },
  ],
  parentActionRows: [],
  parentPreferenceSetupRows: [],
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
