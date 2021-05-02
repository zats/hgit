import { Command, OptionValues } from 'commander';
import { Index, Reference, Repository, Signature } from 'nodegit';
import { FileStatus } from '../model/FileStatus';
import { RepoStatus } from '../model/RepoStatus';
import { Status } from '../model/Status';
import { discoverRepository } from '../utilities/repository-discovery';
import { getRepoStatus } from './status';
const path = require("path");

async function main(args: OptionValues) {
  console.log(args);
  const cwd = process.cwd();
  const result = await discoverRepository()
    .then(repo => getRepoStatus(repo, cwd))
    .then(repoStatus => repoStatus.repo.refreshIndex().then(index => ({ ...repoStatus, index })))
    .then(async ({ index, repo, fileStatus }) => {
      const paths = fileStatus
        .filter(fs => isCommittableFile(fs.status))
        .map(fs => fs.repoPath);
      await index.addAll(paths);
      await index.write();
      return { index, repo };
    })
    .then(async ({ index, repo }) => {
      const oid = await index.writeTree();
      return { repo, oid };
    })
    .then(async ({ repo, oid }) => {
      const parent = await repo.getHeadCommit();
      const author = await Signature.default(repo);
      return await repo.createCommit('HEAD', author, author, args.message, oid, parent != undefined ? [parent] : []);
    });
  console.log(`Commit id '${result}'`);
}

function isCommittableFile(status: Status): boolean {
  return status !== Status.Unknown
    && status !== Status.Ignored
    && status !== Status.NotTracked;
}

const program = new Command();
program
  .name('commit').alias('com')
  .description('list status of changed files')
  .requiredOption('-m, --message <string>', 'message')
  .action(_ => main(program.opts()));

export { program as commit };