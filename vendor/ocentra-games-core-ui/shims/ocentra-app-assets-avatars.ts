export const avatarImageById = {
  1: '/parent-orbit-mark.svg',
  2: '/parent-control-glyph.svg',
  3: '/nav-overview.svg',
  4: '/nav-activity.svg',
  5: '/nav-devices.svg',
  6: '/nav-policy.svg',
  7: '/nav-browser.svg',
  8: '/nav-diagnostics.svg',
  9: '/nav-ai-runtime.svg',
  10: '/nav-memory.svg',
  11: '/nav-settings-rules.svg',
  12: '/portal-status-card-art.svg',
  13: '/header-login.svg',
  14: '/ocentra-logo.svg',
  15: '/parent-orbit-mark.svg',
  16: '/parent-control-glyph.svg',
  17: '/nav-devices.svg',
  18: '/nav-policy.svg',
} as const;

export const avatarImageUrls = Object.values(avatarImageById);
export const defaultAvatarImageUrl = avatarImageById[1];

export function getAvatarImageUrl(id: number): string | null {
  return avatarImageById[id as keyof typeof avatarImageById] ?? null;
}
