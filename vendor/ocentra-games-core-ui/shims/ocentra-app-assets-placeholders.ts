export const placeholderImageUrls = Array.from(
  { length: 25 },
  (_value, index) => `/ocentra-game-assets/placeholders/image${index}.jpg`
) as readonly string[];

export const placeholderImageCount = placeholderImageUrls.length;

export function getPlaceholderImageUrl(index: number): string {
  return placeholderImageUrls[
    ((index % placeholderImageUrls.length) + placeholderImageUrls.length) % placeholderImageUrls.length
  ]!;
}
