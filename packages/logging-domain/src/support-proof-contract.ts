export function supportProofHasAnyClaimUpgrade(claimFlags: readonly boolean[]): boolean {
  return claimFlags.some(Boolean);
}

export function supportProofRequiredValuesArePresent<T extends string>(
  actualValues: ReadonlyArray<T>,
  requiredValues: ReadonlyArray<T>
): boolean {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}
