"use strict";

import { Command } from 'commander';
import { Repository } from 'nodegit';
import { Status, gitStatusToStatus, statusToShortString } from '../model/Status';
import { RepoStatus } from '../model/RepoStatus';
import { FileStatus } from '../model/FileStatus';
import { discoverRepository, convertRepoPathToRelativePath } from '../utilities/repository-discovery';

export async function getRepoStatus(repo: Repository, cwd: string): Promise<RepoStatus> {
  return await repo.getStatus().then(s => s
    .map(sf => new FileStatus(
      sf.path(),
      convertRepoPathToRelativePath(repo, sf.path(), cwd),
      gitStatusToStatus(sf.statusBit()))
    )
    .sort((s1, s2) => FileStatus.sortFunction(s1, s2))
  ).then(fss => new RepoStatus(repo, fss));
}

async function main(args: any[]) {
  const repo = await discoverRepository(process.cwd());
  const result = await getRepoStatus(repo, process.cwd()).then(rs => rs.fileStatus.map(fs => statusToString(rs.repo, fs)).join('\n'));
  console.log(result);
}

function statusToString(repo: Repository, s: FileStatus): string {
  return `${statusToShortString(s.status)} ${s.relativePath}`;
}

const program = new Command();
program
  .name('status').alias('st')
  .description('list status of changed files')
  .option('-A, --all', 'list all files')
  .option('--rev [REV]', 'show difference from revision')
  .option('--change [REV]', 'list the changed files of a revision')
  .action(args => main(args));

export { program as status };