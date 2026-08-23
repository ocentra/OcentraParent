import type { BridgeEntry } from '../transport/bridgeLogPayload';
import { sendToBridge } from '../transport/bridgeTransport';

interface BridgeLogQueueRuntime {
  readonly endpoint: string | null;
  readonly skipHealthCheck: boolean;
}

export class BridgeLogQueue {
  private readonly entries: BridgeEntry[] = [];
  private generation = 0;
  private flushInFlight: Promise<void> | null = null;

  constructor(private readonly resolveRuntime: () => BridgeLogQueueRuntime) {}

  enqueue(entry: BridgeEntry): void {
    this.entries.push(entry);
  }

  reset(): void {
    this.entries.length = 0;
    this.generation += 1;
  }

  async flush(): Promise<void> {
    if (this.flushInFlight != null) {
      return this.flushInFlight;
    }
    const flush = this.drain();
    this.flushInFlight = flush;
    try {
      await flush;
    } finally {
      if (this.flushInFlight === flush) {
        this.flushInFlight = null;
      }
    }
  }

  private async drain(): Promise<void> {
    while (this.entries.length > 0) {
      const runtime = this.resolveRuntime();
      if (runtime.endpoint == null || runtime.endpoint.length === 0) {
        throw new Error('log bridge endpoint is unavailable; queued entries were retained');
      }
      const generation = this.generation;
      const entries = this.entries.slice();
      await sendToBridge(entries, runtime.endpoint, { skipHealthCheck: runtime.skipHealthCheck });
      if (generation !== this.generation) {
        continue;
      }
      this.entries.splice(0, entries.length);
    }
  }
}
