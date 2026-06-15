import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema } from '@ocentra-parent/evidence-domain/primitives';

const NonEmptySocialBypassText = Schema.String.pipe(Schema.minLength(1));

export const BrowserSocialUnmanagedBypassSchemaVersion = 1;

export const BrowserSocialUnmanagedBypassEvidenceIdSchema = withParser(
  NonEmptySocialBypassText.pipe(Schema.brand('BrowserSocialUnmanagedBypassEvidenceId'))
);

export const OptionalBrowserSocialUnmanagedBypassTextSchema = Schema.Union(NonEmptySocialBypassText, Schema.Null);

export const BrowserSocialUnmanagedBypassSourceEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected unmanaged social bypass source evidence ids')
);

export const BrowserSocialUnmanagedBypassTargetStateSchema = withParser(
  Schema.Literal('bypass-detected', 'manual-required', 'unavailable')
);

export const BrowserSocialUnmanagedBypassReasonSchema = withParser(
  Schema.Literal(
    'possible-social-bypass-process',
    'supported-browser-outside-managed-session',
    'unsupported-browser-social-attempt',
    'portable-browser-social-attempt',
    'tor-browser-social-attempt',
    'browser-like-social-attempt',
    'managed-browser-required',
    'exact-url-unavailable',
    'manual-required',
    'allowed-unmanaged-exception'
  )
);

export const BrowserSocialUnmanagedBypassReasonsSchema = Schema.Array(BrowserSocialUnmanagedBypassReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected unmanaged social bypass reasons')
);

export type BrowserSocialUnmanagedBypassReason = Infer<typeof BrowserSocialUnmanagedBypassReasonSchema>;
export type BrowserSocialUnmanagedBypassTargetState = Infer<typeof BrowserSocialUnmanagedBypassTargetStateSchema>;
