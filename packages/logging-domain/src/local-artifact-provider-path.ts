import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { MaximumProviderRelativePathBytes } from './local-artifact-provider-protocol';

export function providerRelativePath(value: string, allowEmpty: boolean): string {
  const invalidSegment = value
    .split('/')
    .some((segment) => segment.length === 0 || segment === '.' || segment === '..');
  if (
    (!allowEmpty && value.length === 0) ||
    Buffer.byteLength(value, 'utf8') > MaximumProviderRelativePathBytes ||
    value.includes('\\') ||
    value.includes('\0') ||
    value.includes(':') ||
    value.startsWith('/') ||
    (value.length > 0 && invalidSegment)
  ) {
    throw new LocalArtifactProviderError('invalid-arguments', 'provider relative artifact path is invalid');
  }
  return value;
}
