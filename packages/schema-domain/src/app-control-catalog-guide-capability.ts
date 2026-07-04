export function guideCapabilityStateFor(effectStatus: string) {
  if (effectStatus === 'manual-required') {
    return 'manual-required';
  }
  if (effectStatus === 'permission-required') {
    return 'permission-required';
  }
  if (effectStatus === 'permission-limited') {
    return 'permission-limited';
  }
  if (effectStatus === 'future-gap') {
    return 'future-gap';
  }
  if (effectStatus === 'degraded') {
    return 'degraded';
  }
  if (effectStatus === 'proof-required') {
    return 'protected';
  }
  return 'available';
}
