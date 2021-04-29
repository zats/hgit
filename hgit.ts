import { Command } from 'commander';
import { status } from './src/commands/status';
import { commit } from './src/commands/commit';

async function main() {
  const program = new Command();
  program
    .version('0.0.1')
    .addCommand(commit)
    .addCommand(status)
    .parse(process.argv);
}

main();