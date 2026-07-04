export function guideEffectStatusFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/manual|required setup|mdm|device-owner|supervised|entitlement|custody model|AppLocker|WDAC/iu.test(searchable)) {
    return 'manual-required';
  }
  if (/permission|visibility-limited|privacy|protected|unreadable|uncontrollable/iu.test(searchable)) {
    return /limited|partial|varies/iu.test(searchable) ? 'permission-limited' : 'permission-required';
  }
  if (
    /proof|unknown|confidence|must not|does not prove|cannot prove|without proof|source\/confidence/iu.test(searchable)
  ) {
    return 'proof-required';
  }
  if (/future|later|not yet|planned|missing|gap/iu.test(sourceText)) {
    return 'future-gap';
  }
  if (/unsupported|unavailable|stale|degraded|fallback|adapter-error|varies|partial|miss/iu.test(searchable)) {
    return 'degraded';
  }
  if (/audit|retention|report|redact|local-first|parent-owned|never collect|show/iu.test(searchable)) {
    return 'already-represented';
  }
  return 'needs-effect-wiring';
}
