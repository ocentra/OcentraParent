const LanEvidenceSourceAliases = new Map([
  ['local-service', 'local-service'],
  ['trusted-registry', 'trusted-registry'],
]);

function inferLanEvidenceSource(sourceLabels: readonly string[]): string {
  for (const sourceLabel of sourceLabels) {
    const inferred = LanEvidenceSourceAliases.get(sourceLabel);
    if (inferred != null) {
      return inferred;
    }
  }
  return 'windows-neighbor-table';
}

export { inferLanEvidenceSource };
