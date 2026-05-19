import { Schema } from '@ocentra-parent/schema-domain/effect';

const NonEmptyText = Schema.String.pipe(Schema.minLength(1));

export const DisplayTextSchema = NonEmptyText.pipe(Schema.brand('DisplayText'));
export type DisplayText = typeof DisplayTextSchema.Type;

export const TextTokenIdSchema = NonEmptyText.pipe(Schema.brand('TextTokenId'));
export type TextTokenId = typeof TextTokenIdSchema.Type;

export const decodeDisplayText = Schema.decodeUnknownSync(DisplayTextSchema);
export const decodeTextTokenId = Schema.decodeUnknownSync(TextTokenIdSchema);
