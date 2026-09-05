import type childProcess from 'node:child_process';
import type { Worker } from 'node:worker_threads';
import {
  terminateLocalArtifactProviderProcess,
  terminateLocalArtifactProviderWorker,
} from './local-artifact-provider-session-process';

function terminationFailures(results: readonly PromiseSettledResult<void>[]): readonly unknown[] {
  return results.flatMap((result) => (result.status === 'rejected' ? [result.reason] : []));
}

function throwTerminationFailures(failures: readonly unknown[]): void {
  if (failures.length === 0) return;
  if (failures.length === 1) throw failures[0];
  throw new AggregateError(failures, 'local artifact provider resources did not terminate cleanly');
}

export async function terminateLocalArtifactProviderSessionResources(
  worker: Worker,
  provider: childProcess.ChildProcess,
  processExit: Promise<void>
): Promise<void> {
  const results = await Promise.allSettled([
    terminateLocalArtifactProviderWorker(worker),
    terminateLocalArtifactProviderProcess(provider, processExit),
  ]);
  throwTerminationFailures(terminationFailures(results));
}
