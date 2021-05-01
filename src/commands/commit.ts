import { Command, OptionValues } from 'commander';
import { Signature } from 'nodegit';
import { discoverRepository } from '../utilities/repository-discovery';

async function main(args: OptionValues) {
  const repo = await discoverRepository();
  const signature = await Signature.default(repo);
  console.log(signature.toString());
  // repo.createCommitOnHead(filesList, signature, signature, args.message);
}

const program = new Command();
program
  .name('commit').alias('com')
  .description('list status of changed files')
  .requiredOption('-m, --message', 'message')
  .action(_ => main(program.opts()));

export { program as commit };