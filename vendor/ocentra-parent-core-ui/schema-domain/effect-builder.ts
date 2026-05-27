type Parser<T> = {
  default: (value: T) => Parser<T>;
  min: (value: number) => Parser<T>;
  nullable: () => Parser<T | null>;
  optional: () => Parser<T | undefined>;
  partial: () => Parser<Partial<T>>;
  parse: (input: unknown) => T;
  strict: () => Parser<T>;
};

function parser<T>(fallback: T): Parser<T> {
  return {
    default: () => parser(fallback),
    min: () => parser(fallback),
    nullable: () => parser(fallback as T | null),
    optional: () => parser(fallback as T | undefined),
    partial: () => parser({} as Partial<T>),
    parse: (input: unknown) => input as T,
    strict: () => parser(fallback),
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
