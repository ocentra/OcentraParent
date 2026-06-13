import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const PortalDetailValueSchema = withParser(brandedNonEmptyStringSchema('PortalDetailValue'));
export const PortalClipboardTextSchema = withParser(brandedNonEmptyStringSchema('PortalClipboardText'));
export type PortalDetailValue = Infer<typeof PortalDetailValueSchema>;
export type PortalClipboardText = Infer<typeof PortalClipboardTextSchema>;

export const decodePortalDetailValue = PortalDetailValueSchema.parse;
export const decodePortalClipboardText = PortalClipboardTextSchema.parse;

