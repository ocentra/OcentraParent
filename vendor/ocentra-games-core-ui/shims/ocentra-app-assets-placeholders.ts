export const placeholderImageUrls = [
  '/portal-status-card-art.svg',
  '/nav-overview.svg',
  '/nav-activity.svg',
  '/nav-devices.svg',
  '/nav-policy.svg',
  '/nav-browser.svg',
  '/nav-diagnostics.svg',
  '/nav-ai-runtime.svg',
  '/nav-memory.svg',
  '/nav-settings-rules.svg',
] as const;

export const placeholderImageCount = placeholderImageUrls.length;

export function getPlaceholderImageUrl(index: number): string {
  return placeholderImageUrls[
    ((index % placeholderImageUrls.length) + placeholderImageUrls.length) % placeholderImageUrls.length
  ]!;
}
