import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  ScreenOptionalVisibilityAuditRefSchema,
  ScreenOptionalVisibilityPlatformProofRefSchema,
} from './screen-optional-visibility-mode-values';

export const ScreenLiveViewRequiredFalseSchema = Schema.Literal(false);
export const ScreenLiveViewRequiredTrueSchema = Schema.Literal(true);
export const ScreenLiveViewOptionalProofRefSchema = Schema.Union(
  ScreenOptionalVisibilityPlatformProofRefSchema,
  Schema.Null
);
export const ScreenLiveViewOptionalAuditRefSchema = Schema.Union(ScreenOptionalVisibilityAuditRefSchema, Schema.Null);
