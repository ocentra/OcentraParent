export function titleFromToken(value: string): string {
  return value
    .split('-')
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

export function slugToken(value: string): string {
  const slugged = value
    .toLowerCase()
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '');
  return slugged.length > 0 ? slugged : 'item';
}

export function cleanOptionLabel(value: string): string {
  return titleFromToken(value.trim().replace(/\.$/u, '').replace(/\s+/gu, '-'));
}
