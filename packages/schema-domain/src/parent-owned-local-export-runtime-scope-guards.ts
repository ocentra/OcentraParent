import type {
  ParentOwnedLocalExportRuntimeDeleteReceipt,
  ParentOwnedLocalExportRuntimeOutput,
  ParentOwnedLocalExportRuntimeScope,
} from './parent-owned-local-export-runtime';

export function localExportRuntimeScopeIsSafe(scope: ParentOwnedLocalExportRuntimeScope): boolean {
  return (
    scope.parentAuthorized &&
    !scope.rawEvidenceUploaded &&
    !scope.ocentraHostedFamilyDataStored &&
    scope.requestedDataClasses.length > 0 &&
    scope.destinationOwnership !== 'ocentra-hosted-non-activity-metadata'
  );
}

export function localExportRuntimeOutputIsSafe(output: ParentOwnedLocalExportRuntimeOutput): boolean {
  if (output.destinationOwnership === 'ocentra-hosted-non-activity-metadata') {
    return false;
  }

  if (output.rawEvidenceIncludedByDefault || output.ocentraHostedCopyRetained) {
    return false;
  }

  if (output.outputFormat !== 'human-readable-parent-report' && !output.encryptedAtRest) {
    return false;
  }

  return output.sourceEvidenceRefs.length > 0 && output.childDetailMinimized;
}

export function localExportRuntimeDeleteReceiptIsSafe(receipt: ParentOwnedLocalExportRuntimeDeleteReceipt): boolean {
  if (!receipt.localSafetyStatePreserved || receipt.sourceEvidenceRetained) {
    return false;
  }

  if (receipt.deleteConfirmed) {
    return receipt.deletedAt !== null && receipt.exportedOutputDeleted && receipt.auditState === 'audit-recorded';
  }

  return receipt.failureReasonRef !== null && !receipt.exportedOutputDeleted;
}
