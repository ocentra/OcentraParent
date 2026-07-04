export function guideCapabilityRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/unknown/iu.test(searchable)) {
    return 'unknown-app-state-must-remain-explicit';
  }
  if (
    /block|shield|suspend|hide|terminate|install|uninstall|AppLocker|WDAC|MDM|device-owner|entitlement/iu.test(
      searchable
    )
  ) {
    return 'platform-adapter-proof-required-before-product-claim';
  }
  if (/inventory|package|bundle|identity|process|window|foreground|duration|session/iu.test(searchable)) {
    return 'typed-local-app-evidence-required';
  }
  if (/audit|retention|report|redact|custody/iu.test(searchable)) {
    return 'parent-owned-local-storage-and-redaction';
  }
  return 'app-control-capability-registry';
}

export function guideProofRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string): string | null {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (
    /broad app blocking|block launch|AppLocker|WDAC|shield|suspend|hide|install|uninstall|MDM|device-owner|entitlement/iu.test(
      searchable
    )
  ) {
    return 'strict app control requires real platform adapter or managed-device proof.';
  }
  if (/unknown|confidence|identity|category|proof|evidence|foreground|duration|process|window/iu.test(searchable)) {
    return 'app claims require fresh evidence references with confidence and custody.';
  }
  return null;
}

export function guideFallbackFor(effectStatus: string, sectionTitle: string, groupTitle: string, sourceText: string): string {
  if (/unknown/iu.test(`${sectionTitle} ${groupTitle} ${sourceText}`)) {
    return 'Keep unknown apps labeled unknown; do not promote to known, risky, game, or blocked without proof.';
  }
  if (effectStatus === 'manual-required') {
    return 'Disable strict action or show manual-required until platform setup and adapter proof exist.';
  }
  if (effectStatus === 'permission-required' || effectStatus === 'permission-limited') {
    return 'Show permission-limited state and compile observe, warn, or ask fallback instead of hidden enforcement.';
  }
  if (effectStatus === 'degraded') {
    return 'Render degraded capability and keep unsupported behavior out of compiled enforcement plans.';
  }
  if (effectStatus === 'proof-required') {
    return 'Require evidence proof before strict effect; otherwise fall back to observe, warn, ask, or unavailable.';
  }
  if (effectStatus === 'future-gap') {
    return 'Expose as future or planning-only; do not claim current runtime behavior.';
  }
  return 'Portal renders the control; child-agent/runtime ownership remains explicit.';
}
