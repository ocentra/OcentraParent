export function countProductionProofValues<const Value extends string>(
  values: ReadonlyArray<Value>,
  expectedValues: ReadonlyArray<Value>
): Record<Value, number> {
  const counts = Object.fromEntries(expectedValues.map((value) => [value, 0])) as Record<Value, number>;
  for (const value of values) {
    counts[value] += 1;
  }
  return counts;
}
