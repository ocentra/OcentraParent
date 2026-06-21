import { type Infer, brandedNonEmptyStringSchema, withParser } from './effect';

export const SocialVideoSourcePrivacyEvidenceIdSchema = withParser(
  brandedNonEmptyStringSchema('SocialVideoSourcePrivacyEvidenceId')
);

export type SocialVideoSourcePrivacyEvidenceId = Infer<typeof SocialVideoSourcePrivacyEvidenceIdSchema>;
