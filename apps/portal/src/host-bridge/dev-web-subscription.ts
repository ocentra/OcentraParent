import {
  ParentHostBridgeRuntime,
  type ParentDevBridgeUrl,
  type ParentRouteContext,
  type ParentRouteId,
  type ParentRouteSnapshot,
  type ParentSubscriptionEvent,
} from '../../generated/parent-ui-bridge';
import { createUnavailableDevWebRouteSnapshot } from './dev-web-unavailable-snapshot';

type LoadRouteSnapshot = (
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId,
  context?: ParentRouteContext
) => Promise<ParentRouteSnapshot>;

export function createDevWebRouteSubscription(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId,
  context: ParentRouteContext | undefined,
  onEvent: (event: ParentSubscriptionEvent) => void,
  loadRouteSnapshot: LoadRouteSnapshot
): () => void {
  const subscriptionState = {
    active: true,
    inFlight: false,
    lastSnapshotJson: JSON.stringify(null),
    lastSnapshot: null as ParentRouteSnapshot | null,
  };
  const emitNextSnapshot = createDevWebEmitNextSnapshot(
    subscriptionState,
    parentDevBridgeUrl,
    route,
    context,
    onEvent,
    loadRouteSnapshot
  );
  void emitNextSnapshot();
  const intervalId = globalThis.setInterval(() => {
    void emitNextSnapshot();
  }, ParentHostBridgeRuntime.DevRouteSubscriptionPollMs);

  return () => {
    subscriptionState.active = false;
    globalThis.clearInterval(intervalId);
  };
}

function createDevWebEmitNextSnapshot(
  subscriptionState: {
    active: boolean;
    inFlight: boolean;
    lastSnapshotJson: ReturnType<typeof JSON.stringify>;
    lastSnapshot: ParentRouteSnapshot | null;
  },
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId,
  context: ParentRouteContext | undefined,
  onEvent: (event: ParentSubscriptionEvent) => void,
  loadRouteSnapshot: LoadRouteSnapshot
): () => Promise<void> {
  return async function emitNextSnapshot(): Promise<void> {
    if (!subscriptionState.active || subscriptionState.inFlight) {
      return;
    }
    subscriptionState.inFlight = true;
    try {
      let snapshot: ParentRouteSnapshot;
      try {
        snapshot = await loadRouteSnapshot(parentDevBridgeUrl, route, context);
      } catch {
        snapshot = createUnavailableDevWebRouteSnapshot(parentDevBridgeUrl, route);
      }
      if (!subscriptionState.active) {
        return;
      }
      if (
        snapshot.serviceHealth?.state === 'unavailable' &&
        subscriptionState.lastSnapshot?.serviceHealth?.state === 'unavailable'
      ) {
        snapshot = subscriptionState.lastSnapshot;
      }
      const snapshotJson = JSON.stringify(snapshot);
      if (snapshotJson === subscriptionState.lastSnapshotJson) {
        return;
      }
      subscriptionState.lastSnapshotJson = snapshotJson;
      subscriptionState.lastSnapshot = snapshot;
      onEvent({
        schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
        route,
        snapshot,
      });
    } finally {
      subscriptionState.inFlight = false;
    }
  };
}
