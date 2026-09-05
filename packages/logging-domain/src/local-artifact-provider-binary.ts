import fs from 'node:fs';
import path from 'node:path';
import { validateProviderBinary, type ProviderBinary } from './local-artifact-provider-binary-validation';
import { LocalArtifactProviderError } from './local-artifact-provider-error';
import {
  loggingPackageIdentity,
  loggingPackageRoot,
  parseProviderManifest,
  providerManifestPaths,
  readStableJsonFile,
} from './local-artifact-provider-manifest';

export interface LocalArtifactMutationCapability {
  readonly status: 'supported' | 'unsupported';
  readonly platform: NodeJS.Platform;
  readonly provider: 'rust-windows-owner' | 'unavailable';
  readonly reason?: 'provider-binary-invalid' | 'provider-binary-not-found' | 'unsupported-platform';
}

type ProviderBinaryResolution =
  | { readonly status: 'ready'; readonly binary: ProviderBinary }
  | { readonly status: 'missing' }
  | { readonly status: 'invalid' };

function resolveConfiguredProviderBinary(): ProviderBinaryResolution {
  if (process.platform !== 'win32') return { status: 'missing' };
  let manifestObserved = false;
  for (const manifestPath of providerManifestPaths()) {
    if (!fs.existsSync(manifestPath)) continue;
    manifestObserved = true;
    try {
      const manifest = parseProviderManifest(readStableJsonFile(manifestPath, 64 * 1024));
      const identity = loggingPackageIdentity();
      if (manifest == null || identity == null || manifest.packageVersion !== identity.version) {
        continue;
      }
      const binary = validateProviderBinary(path.dirname(manifestPath), manifest.binaryPath, manifest.binarySha256);
      if (binary != null) return { status: 'ready', binary };
    } catch {
      continue;
    }
  }
  return manifestObserved ? { status: 'invalid' } : { status: 'missing' };
}

export function providerPackageRoot(): string {
  return loggingPackageRoot();
}

export function providerMutationCapability(): LocalArtifactMutationCapability {
  if (process.platform !== 'win32') {
    return {
      status: 'unsupported',
      platform: process.platform,
      provider: 'unavailable',
      reason: 'unsupported-platform',
    };
  }
  const resolution = resolveConfiguredProviderBinary();
  if (resolution.status === 'ready') {
    return { status: 'supported', platform: process.platform, provider: 'rust-windows-owner' };
  }
  return {
    status: 'unsupported',
    platform: process.platform,
    provider: 'unavailable',
    reason: resolution.status === 'invalid' ? 'provider-binary-invalid' : 'provider-binary-not-found',
  };
}

export function requireProviderBinary(): ProviderBinary {
  const resolution = resolveConfiguredProviderBinary();
  if (resolution.status === 'ready') return resolution.binary;
  throw new LocalArtifactProviderError(
    'unsupported-provider',
    'the pinned Rust local-artifact provider is unavailable or invalid'
  );
}
