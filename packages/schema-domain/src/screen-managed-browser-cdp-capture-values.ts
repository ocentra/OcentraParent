import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';

export const ScreenManagedBrowserCdpCaptureSchemaVersion = 1;
export const ScreenManagedBrowserCdpMaxPixels = 4_000_000;
export const ScreenManagedBrowserCdpMaxDimension = 4096;

export const ScreenManagedBrowserCdpCaptureIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenManagedBrowserCdpCaptureId')
);
export const ScreenManagedBrowserCdpCaptureRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenManagedBrowserCdpCaptureRequestId')
);
export const ScreenManagedBrowserCdpTargetIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenManagedBrowserCdpTargetId')
);
export const ScreenManagedBrowserCdpCaptureModeSchema = withParser(Schema.Literal('page', 'viewport', 'crop'));
export const ScreenManagedBrowserCdpTargetTypeSchema = withParser(Schema.Literal('page'));
export const ScreenManagedBrowserCdpMethodSchema = withParser(Schema.Literal('Page.captureScreenshot'));

export type ScreenManagedBrowserCdpCaptureMode = Infer<typeof ScreenManagedBrowserCdpCaptureModeSchema>;
