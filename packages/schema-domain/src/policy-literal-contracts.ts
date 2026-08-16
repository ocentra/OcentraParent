/* generic helper for literal/value contracts */

import { Schema, withParser } from './effect';

export function literalValues<Literal extends Record<string, string>>(
  literal: Literal
): readonly Literal[keyof Literal][] {
  return Object.freeze(Object.values(literal)) as readonly Literal[keyof Literal][];
}

export function literalRecordFromValues<const Values extends readonly string[]>(
  values: Values
): Readonly<Record<Values[number], Values[number]>> {
  return Object.freeze(
    Object.fromEntries(values.map((value) => [value, value])) as Record<Values[number], Values[number]>
  );
}

export function literalSchema<Literal extends Record<string, string>>(literal: Literal) {
  return withParser(Schema.Literal(...literalValues(literal)));
}

export function parsedLiteralRecord<
  Literal extends Record<string, string>,
  Value extends Literal[keyof Literal] = Literal[keyof Literal],
>(literal: Literal, parseValue: (value: Literal[keyof Literal]) => Value): Readonly<Record<keyof Literal, Value>> {
  return Object.freeze(
    Object.fromEntries(literalValues(literal).map((value) => [literalRecordKey(value), parseValue(value)]))
  ) as Readonly<Record<keyof Literal, Value>>;
}

export function hasExactlySameValues<T>(values: readonly T[], expectedValues: readonly T[]): boolean {
  if (values.length !== expectedValues.length) {
    return false;
  }

  const remaining = new Set(expectedValues);
  for (const value of values) {
    if (!remaining.delete(value)) {
      return false;
    }
  }

  return remaining.size === 0;
}

export function hasUniqueValues<T>(values: readonly T[]): boolean {
  return new Set(values).size === values.length;
}

function literalRecordKey(value: string): string {
  return value
    .split('-')
    .map((segment) => segment.charAt(0).toUpperCase() + segment.slice(1))
    .join('');
}
