export const avatarImageById = {
  1: '/ocentra-game-assets/avatars/1.png',
  2: '/ocentra-game-assets/avatars/2.png',
  3: '/ocentra-game-assets/avatars/3.png',
  4: '/ocentra-game-assets/avatars/4.png',
  5: '/ocentra-game-assets/avatars/5.png',
  6: '/ocentra-game-assets/avatars/6.png',
  7: '/ocentra-game-assets/avatars/7.png',
  8: '/ocentra-game-assets/avatars/8.png',
  9: '/ocentra-game-assets/avatars/9.png',
  10: '/ocentra-game-assets/avatars/10.png',
  11: '/ocentra-game-assets/avatars/11.png',
  12: '/ocentra-game-assets/avatars/12.png',
  13: '/ocentra-game-assets/avatars/13.png',
  14: '/ocentra-game-assets/avatars/14.png',
  15: '/ocentra-game-assets/avatars/15.png',
  16: '/ocentra-game-assets/avatars/16.png',
  17: '/ocentra-game-assets/avatars/17.png',
  18: '/ocentra-game-assets/avatars/18.png',
} as const;

export const avatarImageUrls = Object.values(avatarImageById);
export const defaultAvatarImageUrl = avatarImageById[1];

export function getAvatarImageUrl(id: number): string | null {
  return avatarImageById[id as keyof typeof avatarImageById] ?? null;
}
