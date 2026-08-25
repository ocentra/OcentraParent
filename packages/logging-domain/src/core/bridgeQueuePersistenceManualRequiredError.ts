export const BridgeQueuePersistenceManualRequiredCode = 'BRIDGE_QUEUE_PERSISTENCE_MANUAL_REQUIRED';

export class BridgeQueuePersistenceManualRequiredError extends Error {
  readonly code = BridgeQueuePersistenceManualRequiredCode;
  readonly disposition = 'retained-for-owner-resolution';

  constructor() {
    super(
      'persisted log bridge queue is invalid and requires owner resolution; state was retained without modification'
    );
    this.name = 'BridgeQueuePersistenceManualRequiredError';
  }
}
