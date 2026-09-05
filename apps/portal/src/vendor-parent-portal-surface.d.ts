import type { ReactElement } from 'react';
import type { ParentAgentCommandName, ParentAgentProtocolPayload } from '../generated/parent-ui-bridge';
import type { ResolvedPortalLiveActivityState } from './route-live-activity-state';

export type ParentPortalSvgSurfaceProps = Readonly<{
  pageMode: unknown;
  controlCode: number;
  seasonId: string;
  lastUpdated: string;
  parentPortalRows: readonly unknown[];
  userEntry: unknown;
  nearbyAbove: readonly unknown[];
  nearbyBelow: readonly unknown[];
  error: string | null;
  statusMessage?: string | null;
  content: unknown;
  controls: unknown;
  initialNavLabel: string;
  initialSelectedControlId: string;
  assistantRouteActive: boolean;
  assistantRoutePath: string;
  assistantReturnRoutePath: string;
  assistantCommandAvailable: boolean;
  workspaceVisible?: boolean;
  assistantResponse: Readonly<{
    eventId: string;
    kind: 'answer' | 'error' | 'unavailable';
    state: string;
    text: string;
  }> | null;
  activityState: ResolvedPortalLiveActivityState;
  lanPairingAutoScanSequence: number;
  onInitialLayoutReady: () => void;
  onRefreshParentPortal: (controlCode: number) => void;
  onMatchmaking: () => void;
  onNavigate: (routePath: string) => void;
  onAssistantCommand?: (command: ParentAgentCommandName, payload: ParentAgentProtocolPayload) => void;
}>;

export declare function ParentPortalSvgSurface(props: ParentPortalSvgSurfaceProps): ReactElement;
