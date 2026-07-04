import type { ParentOwnedLocalExportRuntimeJob } from './parent-owned-local-export-runtime';

export function localExportRuntimeJobIsSafe(job: ParentOwnedLocalExportRuntimeJob): boolean {
  if (job.localEvidenceMutated || job.parentOwnedOutputMutatedByFailure || !job.localSafetyStatePreserved) {
    return false;
  }

  if (job.operation === 'export') {
    return exportJobStateIsSafe(job);
  }

  return deleteJobStateIsSafe(job);
}

export function exportJobStateIsSafe(job: ParentOwnedLocalExportRuntimeJob): boolean {
  if (job.state === 'export-written') {
    return job.output !== null && job.deleteReceipt === null && !job.manualActionRequired;
  }

  if (job.state === 'offline-queued' || job.state === 'manual-required') {
    return job.output === null && job.manualActionRequired;
  }

  if (job.state === 'export-queued' || job.state === 'export-running') {
    return job.output === null && job.deleteReceipt === null;
  }

  return false;
}

export function deleteJobStateIsSafe(job: ParentOwnedLocalExportRuntimeJob): boolean {
  if (job.state === 'delete-requested') {
    return job.output !== null && job.deleteReceipt !== null && !job.deleteReceipt.deleteConfirmed;
  }

  if (job.state === 'delete-confirmed') {
    return job.output !== null && job.deleteReceipt !== null && job.deleteReceipt.deleteConfirmed;
  }

  if (job.state === 'delete-failed') {
    return job.output !== null && job.deleteReceipt !== null && !job.deleteReceipt.deleteConfirmed;
  }

  return false;
}
