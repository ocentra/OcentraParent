#!/usr/bin/env node
import { execSync, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { env, platform } from 'node:process';

function log(msg) {
  console.log(`[setup] ${msg}`);
}

function logError(msg) {
  console.error(`[setup] [ERROR] ${msg}`);
}

console.log('===================================================');
console.log('    Ocentra Parent Developer Environment Setup     ');
console.log('===================================================');

// Step 1: Check Node.js version
log('Checking Node.js version...');
const nodeVersion = process.version;
log(`Found Node.js: ${nodeVersion}`);
const majorVersion = parseInt(nodeVersion.slice(1).split('.')[0], 10);
if (majorVersion < 22) {
  logError('Node.js >= 22.15.0 is required. Please upgrade Node.js.');
  process.exit(1);
}

// Step 2: Check Rust/Cargo
log('Checking Rust toolchain...');
let hasCargo = false;
try {
  const cargoVer = execSync('cargo --version', { encoding: 'utf8' }).trim();
  log(`Found Cargo in PATH: ${cargoVer}`);
  hasCargo = true;
} catch {
  // Cargo not in current process PATH. Check default cargo home path on Windows
  if (platform === 'win32') {
    const userProfile = env.USERPROFILE || 'C:\\Users\\default';
    const defaultCargoBin = join(userProfile, '.cargo', 'bin');
    const defaultCargoExe = join(defaultCargoBin, 'cargo.exe');

    if (existsSync(defaultCargoExe)) {
      log(`Detected cargo at default home path: ${defaultCargoExe}`);
      log('Prepend .cargo\\bin to PATH for the current session.');
      env.PATH = `${defaultCargoBin};${env.PATH}`;
      hasCargo = true;
    }
  }
}

if (!hasCargo) {
  log('Rust/Cargo toolchain was not found.');
  if (platform === 'win32') {
    log('Attempting to install Rustup via winget...');
    try {
      execSync('winget install Rustlang.Rustup --accept-source-agreements --accept-package-agreements', {
        stdio: 'inherit',
      });
      log('Successfully triggered winget installation of Rustlang.Rustup.');
      log('Initializing default toolchain...');

      const userProfile = env.USERPROFILE || 'C:\\Users\\default';
      const defaultCargoBin = join(userProfile, '.cargo', 'bin');
      const defaultCargoExe = join(defaultCargoBin, 'cargo.exe');

      // Let it initialize default toolchain
      spawnSync(defaultCargoExe, ['--version'], { stdio: 'inherit', shell: true });

      env.PATH = `${defaultCargoBin};${env.PATH}`;
      hasCargo = true;
    } catch (err) {
      logError('Failed to install Rustup via winget.');
      logError('Please download and install it manually from https://rustup.rs/');
      process.exit(1);
    }
  } else {
    log('Please install Rustup manually by running:');
    console.log("  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
    process.exit(1);
  }
}

// Step 3: Run npm install
log('Ensuring all npm packages are installed...');
try {
  execSync('npm install', { stdio: 'inherit' });
  log('NPM packages are up-to-date.');
} catch (err) {
  logError('Failed to run npm install.');
  process.exit(1);
}

// Step 4: Install Git Hooks
log('Installing pre-commit git hooks...');
try {
  execSync('npm run hooks:install', { stdio: 'inherit' });
  log('Git hooks installed successfully.');
} catch (err) {
  logError('Failed to install Git hooks.');
  process.exit(1);
}

// Step 5: Install Playwright Chromium Browser
log('Ensuring Playwright Chromium browser is installed...');
try {
  execSync('npx playwright install chromium', { stdio: 'inherit' });
  log('Playwright Chromium is ready.');
} catch (err) {
  logError('Failed to install Playwright browser.');
  process.exit(1);
}

// Step 6: Verify workspace using validate
log('Running full project validation...');
try {
  execSync('npm run validate', { stdio: 'inherit' });
  log('Project is fully verified and functional! Setup is complete.');
} catch (err) {
  logError('Project validation failed. Please check the logs above.');
  process.exit(1);
}

console.log('\n===================================================');
console.log('  Setup completed successfully! Enjoy coding!      ');
console.log('===================================================');
