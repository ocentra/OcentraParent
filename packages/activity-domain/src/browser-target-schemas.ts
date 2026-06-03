import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyBrowserText = Schema.String.pipe(Schema.minLength(1));

export const BrowserTargetIdSchema = withParser(NonEmptyBrowserText.pipe(Schema.brand('BrowserTargetId')));
