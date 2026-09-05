import {
  BrowserEvidenceReadModelSchema,
  BrowserManagedSessionStatusSchema,
  type BrowserEvidenceReadModel,
  type BrowserManagedSessionStatus,
} from '@ocentra-parent/schema-domain/browser-schemas';

export function decodeBrowserEvidenceReadModel(value: unknown): BrowserEvidenceReadModel | null {
  const parsed = BrowserEvidenceReadModelSchema.safeParse(value);
  return parsed.success ? parsed.data : null;
}

export function decodeBrowserManagedStatus(value: unknown): BrowserManagedSessionStatus | null {
  const parsed = BrowserManagedSessionStatusSchema.safeParse(value);
  return parsed.success ? parsed.data : null;
}
