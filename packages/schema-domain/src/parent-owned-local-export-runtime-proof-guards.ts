import { RequiredParentOwnedLocalExportRuntimeNonClaims, RequiredParentOwnedLocalExportRuntimeStates } from './parent-owned-local-export-runtime-values';
import type { ParentOwnedLocalExportRuntimeJob, ParentOwnedLocalExportRuntimeProof } from './parent-owned-local-export-runtime';

const RuntimeClaimFlags = [
  'cloudTransferRuntimeClaimed',
  'connectorOAuthClaimed',
  'providerApiClaimed',
  'portalUiClaimed',
  'ocentraHostedFamilyDataCustodyClaimed',
  'remoteReportCompilerClaimed',
  'childDeviceMutationClaimed',
  'rawEvidenceUploadClaimed',
] as const;

export function localExportRuntimeProofIsSafe(proof: ParentOwnedLocalExportRuntimeProof): boolean {
  return (
    requiredStatesAreCovered(proof.jobs) &&
    RequiredParentOwnedLocalExportRuntimeNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    RuntimeClaimFlags.every((flag) => proof[flag] === false)
  );
}

export function requiredStatesAreCovered(jobs: ReadonlyArray<ParentOwnedLocalExportRuntimeJob>): boolean {
  return RequiredParentOwnedLocalExportRuntimeStates.every((state) => jobs.some((job) => job.state === state));
}
