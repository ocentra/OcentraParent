type JsonDecodeResult = { readonly status: 'decoded'; readonly value: unknown } | { readonly status: 'rejected' };

export function decodeJsonRejectingDuplicateKeys(text: string): JsonDecodeResult {
  const scanner = new DuplicateKeyJsonScanner(text);
  if (!scanner.acceptsDocument()) return { status: 'rejected' };
  try {
    return { status: 'decoded', value: JSON.parse(text) };
  } catch {
    return { status: 'rejected' };
  }
}

class DuplicateKeyJsonScanner {
  private offset = 0;
  private static readonly MAX_DEPTH = 128;

  public constructor(private readonly text: string) {}

  public acceptsDocument(): boolean {
    this.skipWhitespace();
    if (!this.readValue(0)) return false;
    this.skipWhitespace();
    return this.offset === this.text.length;
  }

  private readValue(depth: number): boolean {
    this.skipWhitespace();
    const token = this.text[this.offset];
    if (token === '{') return this.readObject(depth + 1);
    if (token === '[') return this.readArray(depth + 1);
    if (token === '"') return this.readStringToken() !== null;
    if (token === 't') return this.readLiteral('true');
    if (token === 'f') return this.readLiteral('false');
    if (token === 'n') return this.readLiteral('null');
    return token === '-' || this.isDigit(token) ? this.readNumber() : false;
  }

  private readObject(depth: number): boolean {
    if (depth > DuplicateKeyJsonScanner.MAX_DEPTH) return false;
    this.offset += 1;
    this.skipWhitespace();
    if (this.consume('}')) return true;
    const keys = new Set<string>();
    while (this.offset < this.text.length) {
      const keyToken = this.readStringToken();
      if (keyToken === null) return false;
      const key = this.decodeKey(keyToken);
      if (key === null || keys.has(key)) return false;
      keys.add(key);
      this.skipWhitespace();
      if (!this.consume(':') || !this.readValue(depth)) return false;
      this.skipWhitespace();
      if (this.consume('}')) return true;
      if (!this.consume(',')) return false;
      this.skipWhitespace();
    }
    return false;
  }

  private readArray(depth: number): boolean {
    if (depth > DuplicateKeyJsonScanner.MAX_DEPTH) return false;
    this.offset += 1;
    this.skipWhitespace();
    if (this.consume(']')) return true;
    while (this.offset < this.text.length) {
      if (!this.readValue(depth)) return false;
      this.skipWhitespace();
      if (this.consume(']')) return true;
      if (!this.consume(',')) return false;
      this.skipWhitespace();
    }
    return false;
  }

  private readStringToken(): string | null {
    if (!this.consume('"')) return null;
    const start = this.offset - 1;
    while (this.offset < this.text.length) {
      const character = this.text[this.offset];
      if (character === '"') {
        this.offset += 1;
        return this.text.slice(start, this.offset);
      }
      if (character === '\\') {
        this.offset += 1;
        const escape = this.text[this.offset];
        if (escape === 'u') {
          const hex = this.text.slice(this.offset + 1, this.offset + 5);
          if (!/^[0-9a-fA-F]{4}$/.test(hex)) return null;
          this.offset += 5;
          continue;
        }
        if (escape === undefined || !'"\\/bfnrt'.includes(escape)) return null;
        this.offset += 1;
        continue;
      }
      if (character === undefined || character.charCodeAt(0) <= 0x1f) return null;
      this.offset += 1;
    }
    return null;
  }

  private readNumber(): boolean {
    const start = this.offset;
    this.consume('-');
    if (this.consume('0')) {
      if (this.isDigit(this.text[this.offset])) return false;
    } else if (!this.readDigits()) {
      return false;
    }
    if (this.consume('.') && !this.readDigits()) return false;
    if (this.text[this.offset] === 'e' || this.text[this.offset] === 'E') {
      this.offset += 1;
      if (this.text[this.offset] === '+' || this.text[this.offset] === '-') this.offset += 1;
      if (!this.readDigits()) return false;
    }
    return this.offset > start;
  }

  private readDigits(): boolean {
    const start = this.offset;
    while (this.isDigit(this.text[this.offset])) this.offset += 1;
    return this.offset > start;
  }

  private readLiteral(literal: string): boolean {
    if (!this.text.startsWith(literal, this.offset)) return false;
    this.offset += literal.length;
    return true;
  }

  private decodeKey(token: string): string | null {
    try {
      const value: unknown = JSON.parse(token);
      return typeof value === 'string' ? value : null;
    } catch {
      return null;
    }
  }

  private consume(expected: string): boolean {
    if (this.text[this.offset] !== expected) return false;
    this.offset += 1;
    return true;
  }

  private skipWhitespace(): void {
    while (' \t\r\n'.includes(this.text[this.offset] ?? 'x')) this.offset += 1;
  }

  private isDigit(value: string | undefined): boolean {
    return value !== undefined && value >= '0' && value <= '9';
  }
}
