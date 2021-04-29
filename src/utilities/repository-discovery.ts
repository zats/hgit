import { Repository } from 'nodegit';
import fs = require('fs');
import path = require('path');


const GIT_MARKER_FILENAME = '.git';

export async function discoverRepository(directory: string = process.cwd()): Promise<Repository> {
  do {
    const gitFilePath = path.join(directory, GIT_MARKER_FILENAME);
    if (fs.existsSync(gitFilePath)) {
      return Repository.open(gitFilePath);
    }
    directory = path.resolve(directory, '..');
  } while (directory !== path.parse(directory).root)
  return Promise.reject('Not a git repository (or any of the parent directories)');
}

export function convertRelativePathToRelativePath(repository: Repository, filepath: string, directory: string = process.cwd()): string {
  return path.relative(directory, path.join(path.resolve(repository.path(), '..'), filepath));
}