import type { ReactElement } from 'react';
import type { ParentAgentCommandName, ParentAgentProtocolPayload } from '../generated/parent-ui-bridge';

export type ParentPortalSvgSurfaceProps = Readonly<{
  pageMode: unknown;
  controlCode: number;
  seasonId: string;
  lastUpdated: string;
  parentPortalRows: readonly unknown[];
  userEntry: unknown;
  nearbyAbove: readonly unknown[];
  nearbyBelow: readonly unknown[];
  content: unknown;
  controls: unknown;
  initialNavLabel: string;
  initialSelectedControlId: string;
  assistantRouteActive: boolean;
  assistantRoutePath: string;
  assistantReturnRoutePath: string;
  activityState: unknown;
  lanPairingAutoScanSequence: number;
  onInitialLayoutReady: () => void;
  onRefreshParentPortal: (controlCode: number) => void;
  onMatchmaking: () => void;
  onNavigate: (routePath: string) => void;
  onAssistantCommand: (command: ParentAgentCommandName, payload: ParentAgentProtocolPayload) => void;
}>;

export declare function ParentPortalSvgSurface(props: ParentPortalSvgSurfaceProps): ReactElement;
