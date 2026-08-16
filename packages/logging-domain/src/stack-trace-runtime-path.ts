/* generated from crates/logging-core/src/stack_trace_runtime.rs */

export function normalizeGeneratedStackPath(value: string): string {
  return value.replaceAll('\\', '/');
}

function generatedPercentDecode(value: string): string {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
}

function generatedTrimWindowsFileUrlPrefix(pathname: string): string {
  if (pathname.length >= 3 && pathname[0] === '/' && /[A-Za-z]/.test(pathname[1] ?? '') && pathname[2] === ':') {
    return pathname.slice(1);
  }
  return pathname;
}

export function decodeGeneratedStackFilePath(value: string): string {
  if (value.startsWith('file://')) {
    return normalizeGeneratedStackPath(
      generatedTrimWindowsFileUrlPrefix(generatedPercentDecode(value.slice('file://'.length)))
    );
  }
  return normalizeGeneratedStackPath(value);
}

export function fileNameFromGeneratedPath(filePath: string | null): string | null {
  if (filePath == null) {
    return null;
  }
  const normalized = normalizeGeneratedStackPath(filePath);
  const lastSlash = normalized.lastIndexOf('/');
  return lastSlash >= 0 ? normalized.slice(lastSlash + 1) : normalized;
}

export function moduleNameFromGeneratedPath(filePath: string): string {
  const fileName = (fileNameFromGeneratedPath(filePath) ?? '').replace(/\.[^.]+$/, '');
  return fileName
    .split(/[^a-zA-Z0-9]+/)
    .filter((segment) => segment.length > 0)
    .map((segment) => segment.slice(0, 1).toUpperCase() + segment.slice(1))
    .join('');
}
