export function buildCrateRustValidationCommands(crateDir, { testArgs = [] } = {}) {
  const manifestPath = `${crateDir}/Cargo.toml`;
  return [
    ['cargo', ['check', '--manifest-path', manifestPath]],
    ['cargo', ['test', '--manifest-path', manifestPath, ...testArgs]],
  ];
}

export function buildWorkspaceRustValidationCommands() {
  return [
    ['npm', ['run', 'format:rust']],
    ['npm', ['run', 'lint:rust']],
    ['cargo', ['check', '--workspace']],
    ['cargo', ['test', '--workspace']],
  ];
}
