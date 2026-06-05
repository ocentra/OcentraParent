import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyChildDisclosureText = Schema.String.pipe(Schema.minLength(1));

export const ScreenChildDisclosureUxSchemaVersion = 1;

export const ScreenChildDisclosureProofIdSchema = NonEmptyChildDisclosureText.pipe(
  Schema.brand('ScreenChildDisclosureProofId')
);

export const ScreenChildDisclosureStatusIdSchema = NonEmptyChildDisclosureText.pipe(
  Schema.brand('ScreenChildDisclosureStatusId')
);

export const ScreenChildDisclosureAuditRefSchema = NonEmptyChildDisclosureText.pipe(
  Schema.brand('ScreenChildDisclosureAuditRef')
);

export const ScreenChildDisclosureTextTokenRefSchema = NonEmptyChildDisclosureText.pipe(
  Schema.brand('ScreenChildDisclosureTextTokenRef')
);

export const ScreenChildDisclosureStateSchema = withParser(
  Schema.Literal('disabled', 'paused', 'ready', 'captureActive', 'protectedSurface')
);

export const ScreenChildDisclosureIndicatorSchema = withParser(
  Schema.Literal('off', 'paused', 'ready', 'active', 'unavailable')
);

export const ScreenChildDisclosureSurfaceSchema = withParser(
  Schema.Literal('localStatus', 'captureNotice', 'settingsPreview')
);

export const ScreenChildDisclosureDeliverySchema = withParser(
  Schema.Literal('childDeviceLocal', 'parentSettingsPreview')
);

export type ScreenChildDisclosureState = Infer<typeof ScreenChildDisclosureStateSchema>;
export type ScreenChildDisclosureIndicator = Infer<typeof ScreenChildDisclosureIndicatorSchema>;
export type ScreenChildDisclosureSurface = Infer<typeof ScreenChildDisclosureSurfaceSchema>;
export type ScreenChildDisclosureDelivery = Infer<typeof ScreenChildDisclosureDeliverySchema>;
