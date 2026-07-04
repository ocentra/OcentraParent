import type { PolicyControlDeliveryReadModelSnapshotCandidate } from './agent-policy-control-delivery-read-model-snapshot-validators';

export function deriveSnapshotState(
  snapshot: PolicyControlDeliveryReadModelSnapshotCandidate
): PolicyControlDeliveryReadModelSnapshotCandidate['parentVisibleState'] {
  if (snapshot.manualRequiredCount > 0) return 'manual-required';
  if (snapshot.degradedCount > 0) return 'degraded';
  if (snapshot.partiallyAppliedCount > 0) return 'partially-applied';
  if (snapshot.rejectedCount > 0) return 'rejected';
  if (snapshot.rolledBackCount > 0) return 'rolled-back';
  if (snapshot.supersededCount === snapshot.rows.length) return 'superseded';
  if (snapshot.expiredBeforeDeliveryCount === snapshot.rows.length) return 'expired-before-delivery';
  if (snapshot.appliedCount === snapshot.rows.length) return 'applied';
  if (snapshot.acknowledgedCount > 0 && snapshot.pendingCount === 0) return 'acknowledged';
  return 'pending';
}
