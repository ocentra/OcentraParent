import {
  PortalClipboardTextSchema as SharedPortalClipboardTextSchema,
  PortalDetailValueSchema as SharedPortalDetailValueSchema,
  decodePortalClipboardText as sharedDecodePortalClipboardText,
  decodePortalDetailValue as sharedDecodePortalDetailValue,
  type PortalClipboardText as SharedPortalClipboardTextValue,
  type PortalDetailValue as SharedPortalDetailValueValue,
} from '@ocentra-parent/schema-domain/portal-contracts';

export const PortalDetailValueSchema = SharedPortalDetailValueSchema;
export const PortalClipboardTextSchema = SharedPortalClipboardTextSchema;
export type PortalDetailValue = SharedPortalDetailValueValue;
export type PortalClipboardText = SharedPortalClipboardTextValue;

export const decodePortalDetailValue = sharedDecodePortalDetailValue;
export const decodePortalClipboardText = sharedDecodePortalClipboardText;

