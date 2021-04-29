import { Command } from 'commander';
import { Repository } from 'nodegit';
import { Status, gitStatusToStatus, statusSortFunction, statusToShortString } from '../model/Status';
import { discoverRepository, convertRelativePathToRelativePath } from '../utilities/repository-discovery';

async function main(args: any[]) {
  const repo = await discoverRepository(process.cwd());
  const result = await repo.getStatus()
    .then(s => s
      .map(sf => ({
        status: gitStatusToStatus(sf.statusBit()),
        path: sf.path()
      }))
      .sort((s1, s2) => statusSortFunction(s1, s2))
      .map(s => statusToString(repo, s))
      .join('\n')
    );
  console.log(result);
}

function statusToString(repo: Repository, s: { status: Status, path: string }): string {
  return `${statusToShortString(s.status)} ${convertRelativePathToRelativePath(repo, s.path, process.cwd())}`;
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