import type { PolicyControlDeliveryReadModelRowCandidate } from './agent-policy-control-delivery-read-model-row-validators';
import { deriveSnapshotState } from './agent-policy-control-delivery-read-model-snapshot-state';

type PolicyControlDeliveryReadModelSnapshotCountField =
  | 'pendingCount'
  | 'acknowledgedCount'
  | 'degradedCount'
  | 'manualRequiredCount'
  | 'appliedCount'
  | 'partiallyAppliedCount'
  | 'rejectedCount'
  | 'rolledBackCount'
  | 'supersededCount'
  | 'expiredBeforeDeliveryCount';

export type PolicyControlDeliveryReadModelSnapshotCandidate = {
  readonly rows: readonly PolicyControlDeliveryReadModelRowCandidate[];
  readonly parentVisibleState: PolicyControlDeliveryReadModelRowCandidate['parentVisibleState'];
  readonly activationBlocked: boolean;
  readonly pendingCount: number;
  readonly acknowledgedCount: number;
  readonly degradedCount: number;
  readonly manualRequiredCount: number;
  readonly appliedCount: number;
  readonly partiallyAppliedCount: number;
  readonly rejectedCount: number;
  readonly rolledBackCount: number;
  readonly supersededCount: number;
  readonly expiredBeforeDeliveryCount: number;
};

export function validateSnapshotRowCounts(snapshot: PolicyControlDeliveryReadModelSnapshotCandidate): true | string {
  const expectations = [
    { field: 'pendingCount', state: 'pending', message: 'Pending count must match pending rows' },
    { field: 'acknowledgedCount', state: 'acknowledged', message: 'Acknowledged count must match acknowledged rows' },
    { field: 'degradedCount', state: 'degraded', message: 'Degraded count must match degraded rows' },
    {
      field: 'manualRequiredCount',
      state: 'manual-required',
      message: 'Manual-required count must match manual-required rows',
    },
    { field: 'appliedCount', state: 'applied', message: 'Applied count must match applied rows' },
    {
      field: 'partiallyAppliedCount',
      state: 'partially-applied',
      message: 'Partially applied count must match partially applied rows',
    },
    { field: 'rejectedCount', state: 'rejected', message: 'Rejected count must match rejected rows' },
    { field: 'rolledBackCount', state: 'rolled-back', message: 'Rolled-back count must match rolled-back rows' },
    { field: 'supersededCount', state: 'superseded', message: 'Superseded count must match superseded rows' },
    {
      field: 'expiredBeforeDeliveryCount',
      state: 'expired-before-delivery',
      message: 'Expired-before-delivery count must match expired rows',
    },
  ] as const satisfies ReadonlyArray<{
    field: PolicyControlDeliveryReadModelSnapshotCountField;
    state: PolicyControlDeliveryReadModelRowCandidate['parentVisibleState'];
    message: string;
  }>;

  for (const expectation of expectations) {
    if (snapshot[expectation.field] !== countRows(snapshot.rows, expectation.state)) {
      return expectation.message;
    }
  }

  return true;
}

export function validateSnapshotParentVisibleState(
  snapshot: PolicyControlDeliveryReadModelSnapshotCandidate
): true | string {
  return snapshot.parentVisibleState === deriveSnapshotState(snapshot)
    ? true
    : 'Snapshot parent-visible state must match row severity ordering';
}

export function validateSnapshotActivationBlockedState(
  snapshot: PolicyControlDeliveryReadModelSnapshotCandidate
): true | string {
  const shouldBeBlocked = snapshot.parentVisibleState !== 'applied';
  return snapshot.activationBlocked === shouldBeBlocked
    ? true
    : 'Activation blocked must reflect whether every row is applied';
}

function countRows(
  rows: readonly PolicyControlDeliveryReadModelRowCandidate[],
  state: PolicyControlDeliveryReadModelRowCandidate['parentVisibleState']
): number {
  return rows.filter((row) => row.parentVisibleState === state).length;
}
