export const AndroidParentMobileCapabilityStatuses = [
  ['parent-mobile-observer', 'scaffold'],
  ['parent-mobile-controller', 'manual-required'],
  ['foreground-mobile-service', 'manual-required'],
  ['notifications', 'manual-required'],
  ['package-lifecycle', 'manual-required'],
  ['store-distribution', 'planned'],
] as const;

export const IosParentMobileCapabilityStatuses = [
  ['parent-mobile-observer', 'scaffold'],
  ['parent-mobile-controller', 'manual-required'],
  ['foreground-mobile-service', 'unavailable'],
  ['notifications', 'manual-required'],
  ['background-execution', 'manual-required'],
  ['signing-entitlements', 'manual-required'],
  ['testflight-distribution', 'manual-required'],
  ['store-distribution', 'planned'],
] as const;

