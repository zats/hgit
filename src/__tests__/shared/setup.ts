import { mkdtempSync, rmdirSync } from 'fs';
import { execSync } from 'child_process';
const path = require('path');
const os = require('os');

const projectPath = process.cwd();
const bin = path.join(projectPath, 'hgitdev');
let temporaryPath: string;

export function createTemporaryRepository() {
  temporaryPath = mkdtempSync(path.join(os.tmpdir(), `hgit-test-${Date.now()}`));
  execSync(`cd ${temporaryPath}`);
  process.chdir(temporaryPath);
}

export function removeTemporaryRepository() {
  rmdirSync(temporaryPath, { recursive: true });
  process.chdir(projectPath);
  temporaryPath = undefined;
}

export function hgit(command: string): string {
  return execSync(`${bin} ${command}`).toString();
}