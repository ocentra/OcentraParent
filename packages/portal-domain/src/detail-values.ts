import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyPortalText = Schema.String.pipe(Schema.minLength(1));

export const PortalDetailValueSchema = withParser(NonEmptyPortalText.pipe(Schema.brand('PortalDetailValue')));
export const PortalClipboardTextSchema = withParser(NonEmptyPortalText.pipe(Schema.brand('PortalClipboardText')));
export type PortalDetailValue = Infer<typeof PortalDetailValueSchema>;
export type PortalClipboardText = Infer<typeof PortalClipboardTextSchema>;

export const decodePortalDetailValue = PortalDetailValueSchema.parse;
export const decodePortalClipboardText = PortalClipboardTextSchema.parse;
