export function guideRuntimeOwnerFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/audit|retention|custody|report|redact|local-first|parent-owned|journal|storage/iu.test(searchable)) {
    return 'parent-owned-storage';
  }
  if (
    /manual|mdm|device-owner|supervised|entitlement|AppLocker|WDAC|platform management|permission/iu.test(searchable)
  ) {
    return 'os-adapter';
  }
  if (/policy|decision|rule|fallback|compile|deterministic/iu.test(searchable)) {
    return 'rust-parent-runtime';
  }
  return 'child-agent';
}
