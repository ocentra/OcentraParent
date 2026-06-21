export function enforcementProofEntriesHaveUniqueField<Entry, Value extends string>(
  entries: ReadonlyArray<Entry>,
  valueForEntry: (entry: Entry) => Value
): boolean {
  return enforcementProofValuesAreUnique(entries.map(valueForEntry));
}

export function enforcementProofValuesAreUnique<Value extends string>(values: ReadonlyArray<Value>): boolean {
  return new Set(values).size === values.length;
}

export function enforcementProofClaimFlagsAreUnset(flags: ReadonlyArray<boolean>): boolean {
  return flags.every((flag) => flag === false);
}

export function enforcementProofRequiredValuesAreCovered<Value extends string>(
  actualValues: ReadonlyArray<Value>,
  requiredValues: ReadonlyArray<Value>
): boolean {
  const actual = new Set(actualValues);
  return requiredValues.every((value) => actual.has(value));
}

export function enforcementProofRequiredUniqueValuesAreCovered<Value extends string>(
  actualValues: ReadonlyArray<Value>,
  requiredValues: ReadonlyArray<Value>
): boolean {
  return (
    enforcementProofValuesAreUnique(actualValues) &&
    enforcementProofRequiredValuesAreCovered(actualValues, requiredValues)
  );
}

export function enforcementProofCloneReadonlyArrayFields<T extends Record<string, unknown>, K extends keyof T>(
  value: T,
  keys: readonly K[]
): T {
  const cloned = { ...value };

  for (const key of keys) {
    const fieldValue = cloned[key];
    if (Array.isArray(fieldValue)) {
      cloned[key] = [...fieldValue] as T[K];
    }
  }

  return cloned;
}
