export const parentPortalPlaceholderImageUrls = [
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

export const parentPortalPlaceholderImageCount = parentPortalPlaceholderImageUrls.length;

export function getParentPortalPlaceholderImageUrl(index: number) {
  return parentPortalPlaceholderImageUrls[
    ((index % parentPortalPlaceholderImageUrls.length) + parentPortalPlaceholderImageUrls.length) %
      parentPortalPlaceholderImageUrls.length
  ]!;
}
