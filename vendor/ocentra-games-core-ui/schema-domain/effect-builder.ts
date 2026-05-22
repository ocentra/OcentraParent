type Parser<T> = {
  default: (value: T) => Parser<T>;
  min: (value: number) => Parser<T>;
  optional: () => Parser<T | undefined>;
  parse: (input: unknown) => T;
};

function parser<T>(fallback: T): Parser<T> {
  return {
    default: () => parser(fallback),
    min: () => parser(fallback),
    optional: () => parser(fallback as T | undefined),
    parse: (input: unknown) => input as T,
  };
}

export const schema = {
  array: <T>(_item: Parser<T>) => parser<T[]>([]),
  boolean: () => parser(false),
  enum: <T extends readonly unknown[]>(values: T) => parser(values[0] as T[number]),
  number: () => parser(0),
  object: <T extends Record<string, unknown>>(_shape: T) => parser<Record<string, unknown>>({}),
  string: () => parser(''),
};
