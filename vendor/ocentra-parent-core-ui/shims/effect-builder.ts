type SchemaPath = readonly (string | number)[];

type Decoder<T> = (input: unknown, path: SchemaPath) => T;

interface RuntimeParserState<T> {
  readonly optional: boolean;
  readonly decode: Decoder<T>;
}

const parserState = Symbol('parserState');

interface SchemaField {
  readonly [parserState]: RuntimeParserState<unknown>;
  readonly _output?: unknown;
  readonly _optional?: boolean;
  parse(input: unknown): unknown;
}

type SchemaShape = {
  readonly [key: string]: SchemaField;
};

export interface Parser<T, Optional extends boolean = false, Shape extends SchemaShape | undefined = undefined> {
  readonly [parserState]: RuntimeParserState<T>;
  readonly _output?: T;
  readonly _optional?: Optional;
  readonly _shape?: Shape;
  default(value: Exclude<T, undefined>): Parser<Exclude<T, undefined>, false, Shape>;
  min(value: number): Parser<T, Optional, Shape>;
  nullable(): Parser<T | null, Optional, Shape>;
  optional(): Parser<T | undefined, true, Shape>;
  partial(): Parser<Partial<T>, false, Shape>;
  parse(input: unknown): T;
  strict(): Parser<T, Optional, Shape>;
}

export type Infer<S> = S extends { readonly _output?: infer Output } ? Output : never;

type OptionalKeys<Shape extends SchemaShape> = {
  [Key in keyof Shape]-?: Shape[Key] extends { readonly _optional?: true } ? Key : never;
}[keyof Shape];

type RequiredKeys<Shape extends SchemaShape> = Exclude<keyof Shape, OptionalKeys<Shape>>;

type ObjectOutput<Shape extends SchemaShape> = {
  [Key in RequiredKeys<Shape>]: Infer<Shape[Key]>;
} & {
  [Key in OptionalKeys<Shape>]?: Exclude<Infer<Shape[Key]>, undefined>;
};

interface ParserOptions<T> {
  readonly decode: Decoder<T>;
  readonly optional: boolean;
  readonly strict: boolean;
  readonly partialDecode?: Decoder<Partial<T>>;
  readonly strictDecode?: Decoder<T>;
  readonly strictPartialDecode?: Decoder<Partial<T>>;
}

export class SchemaParseError extends Error {
  readonly path: SchemaPath;

  constructor(message: string, path: SchemaPath) {
    super(`${path.length === 0 ? '<root>' : path.join('.')}: ${message}`);
    this.name = 'SchemaParseError';
    this.path = path;
  }
}

function fail(message: string, path: SchemaPath): never {
  throw new SchemaParseError(message, path);
}

function isRecord(input: unknown): input is Record<string, unknown> {
  return typeof input === 'object' && input !== null && !Array.isArray(input);
}

function hasOwn(value: Record<string, unknown> | object, key: string): boolean {
  return Object.prototype.hasOwnProperty.call(value, key);
}

function isDefined<T>(value: T): value is Exclude<T, undefined> {
  return value !== undefined;
}

function assertMinimum(value: unknown, minimum: number, path: SchemaPath): void {
  if (value === undefined || value === null) {
    return;
  }

  if (typeof value === 'string' || Array.isArray(value)) {
    if (value.length < minimum) {
      fail(`must have a length of at least ${minimum}`, path);
    }
    return;
  }

  if (typeof value === 'number') {
    if (value < minimum) {
      fail(`must be at least ${minimum}`, path);
    }
    return;
  }

  fail('does not support a minimum constraint', path);
}

function isDecodedObject<Shape extends SchemaShape>(
  value: unknown,
  shape: Shape,
  allowMissing: boolean
): value is ObjectOutput<Shape> {
  if (!isRecord(value)) {
    return false;
  }

  if (allowMissing) {
    return true;
  }

  for (const key in shape) {
    const field = shape[key];
    if (field === undefined) {
      return false;
    }
    if (!hasOwn(value, key) && !field[parserState].optional) {
      return false;
    }
  }

  return true;
}

function decodeObject<Shape extends SchemaShape>(
  shape: Shape,
  strict: boolean,
  allowMissing: boolean
): Decoder<ObjectOutput<Shape>> {
  return (input, path) => {
    if (!isRecord(input)) {
      return fail('must be an object', path);
    }

    if (strict) {
      for (const key of Object.keys(input)) {
        if (!hasOwn(shape, key)) {
          fail(`contains unknown field "${key}"`, [...path, key]);
        }
      }
    }

    const decoded: Record<string, unknown> = {};
    for (const key in shape) {
      const field = shape[key];
      if (field === undefined) {
        return fail(`schema field "${key}" is unavailable`, [...path, key]);
      }
      if (!hasOwn(input, key)) {
        if (allowMissing || field[parserState].optional) {
          continue;
        }
        fail('is required', [...path, key]);
      }
      decoded[key] = field[parserState].decode(input[key], [...path, key]);
    }

    if (!isDecodedObject(decoded, shape, allowMissing)) {
      return fail('does not match the object schema', path);
    }
    return decoded;
  };
}

function strictOptions<T>(options: ParserOptions<T>): ParserOptions<T> {
  if (options.strictDecode === undefined) {
    return {
      decode: options.decode,
      optional: options.optional,
      strict: true,
      ...(options.partialDecode === undefined ? {} : { partialDecode: options.partialDecode }),
    };
  }

  return {
    decode: options.strictDecode,
    optional: options.optional,
    strict: true,
    ...(options.strictPartialDecode === undefined ? {} : { partialDecode: options.strictPartialDecode }),
  };
}

function createParser<T, Optional extends boolean = false, Shape extends SchemaShape | undefined = undefined>(
  options: ParserOptions<T>
): Parser<T, Optional, Shape> {
  const result: Parser<T, Optional, Shape> = {
    [parserState]: { optional: options.optional, decode: options.decode },
    default(value) {
      const decode: Decoder<Exclude<T, undefined>> = (input, path) => {
        if (input === undefined) {
          return value;
        }
        const decoded = options.decode(input, path);
        if (!isDefined(decoded)) {
          return fail('must be defined', path);
        }
        return decoded;
      };
      return createParser<Exclude<T, undefined>, false, Shape>({
        decode,
        optional: false,
        strict: options.strict,
      });
    },
    min(value) {
      const decode: Decoder<T> = (input, path) => {
        const decoded = options.decode(input, path);
        assertMinimum(decoded, value, path);
        return decoded;
      };
      return createParser<T, Optional, Shape>({
        decode,
        optional: options.optional,
        strict: options.strict,
        ...(options.partialDecode === undefined ? {} : { partialDecode: options.partialDecode }),
      });
    },
    nullable() {
      const decode: Decoder<T | null> = (input, path) => (input === null ? null : options.decode(input, path));
      return createParser<T | null, Optional, Shape>({
        decode,
        optional: options.optional,
        strict: options.strict,
      });
    },
    optional() {
      const decode: Decoder<T | undefined> = (input, path) =>
        input === undefined ? undefined : options.decode(input, path);
      return createParser<T | undefined, true, Shape>({
        decode,
        optional: true,
        strict: options.strict,
      });
    },
    partial() {
      if (options.partialDecode === undefined) {
        throw new SchemaParseError('partial() is only supported for object schemas', []);
      }
      return createParser<Partial<T>, false, Shape>({
        decode: options.partialDecode,
        optional: false,
        strict: options.strict,
        ...(options.strictPartialDecode === undefined ? {} : { strictDecode: options.strictPartialDecode }),
      });
    },
    parse(input) {
      return options.decode(input, []);
    },
    strict() {
      return createParser<T, Optional, Shape>(strictOptions(options));
    },
  };
  return result;
}

function createObjectParser<Shape extends SchemaShape>(shape: Shape): Parser<ObjectOutput<Shape>, false, Shape> {
  return createParser<ObjectOutput<Shape>, false, Shape>({
    decode: decodeObject(shape, false, false),
    optional: false,
    strict: false,
    partialDecode: decodeObject(shape, false, true),
    strictDecode: decodeObject(shape, true, false),
    strictPartialDecode: decodeObject(shape, true, true),
  });
}

export const schema = {
  array<Item extends Parser<Infer<Item>, boolean, SchemaShape | undefined>>(item: Item): Parser<Infer<Item>[]> {
    const decode: Decoder<Infer<Item>[]> = (input, path) => {
      if (!Array.isArray(input)) {
        return fail('must be an array', path);
      }
      return input.map((entry, index) => item[parserState].decode(entry, [...path, index]));
    };
    return createParser<Infer<Item>[]>({ decode, optional: false, strict: false });
  },
  boolean(): Parser<boolean> {
    const decode: Decoder<boolean> = (input, path) => {
      if (typeof input !== 'boolean') {
        return fail('must be a boolean', path);
      }
      return input;
    };
    return createParser<boolean>({ decode, optional: false, strict: false });
  },
  enum<const Values extends readonly unknown[]>(values: Values): Parser<Values[number]> {
    const decode: Decoder<Values[number]> = (input, path) => {
      for (const value of values) {
        if (Object.is(value, input)) {
          return value;
        }
      }
      return fail(`must be one of ${values.map((value) => String(value)).join(', ')}`, path);
    };
    return createParser<Values[number]>({ decode, optional: false, strict: false });
  },
  number(): Parser<number> {
    const decode: Decoder<number> = (input, path) => {
      if (typeof input !== 'number' || !Number.isFinite(input)) {
        return fail('must be a finite number', path);
      }
      return input;
    };
    return createParser<number>({ decode, optional: false, strict: false });
  },
  object<Shape extends SchemaShape>(shape: Shape): Parser<ObjectOutput<Shape>, false, Shape> {
    return createObjectParser(shape);
  },
  string(): Parser<string> {
    const decode: Decoder<string> = (input, path) => {
      if (typeof input !== 'string') {
        return fail('must be a string', path);
      }
      return input;
    };
    return createParser<string>({ decode, optional: false, strict: false });
  },
};
