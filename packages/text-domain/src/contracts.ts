import {
  DisplayTextSchema as SchemaDomainDisplayTextSchema,
  TextTokenIdSchema as SchemaDomainTextTokenIdSchema,
  decodeDisplayText as decodeSchemaDomainDisplayText,
  decodeTextTokenId as decodeSchemaDomainTextTokenId,
} from '@ocentra-parent/schema-domain/text-contracts';

export const DisplayTextSchema = SchemaDomainDisplayTextSchema;
export type DisplayText = typeof DisplayTextSchema.Type;

export const TextTokenIdSchema = SchemaDomainTextTokenIdSchema;
export type TextTokenId = typeof TextTokenIdSchema.Type;

export const decodeDisplayText = decodeSchemaDomainDisplayText;
export const decodeTextTokenId = decodeSchemaDomainTextTokenId;
