import { withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const BrowserTargetIdSchema = withParser(brandedNonEmptyStringSchema('BrowserTargetId'));

