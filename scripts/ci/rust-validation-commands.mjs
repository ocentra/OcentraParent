export function buildCrateRustValidationCommands(crateDir) {
  const manifestPath = `${crateDir}/Cargo.toml`;
  return [
    ['cargo', ['check', '--manifest-path', manifestPath]],
    ['cargo', ['test', '--manifest-path', manifestPath]],
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
