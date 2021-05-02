import { createTemporaryRepository, removeTemporaryRepository, hgit, git } from './shared/setup';
import { TestRepo, TestFile } from './shared/model';

beforeEach(createTemporaryRepository);
afterEach(removeTemporaryRepository);

describe('commit', () => {
  it('Last commit message matches', () => {
    const repo = new TestRepo();
    repo.create();
    repo.stage([
      TestFile.createFile('a.txt', 'a.txt content\n'),
      TestFile.createFile('b.txt', 'b.txt content\n'),
      TestFile.createFile('c.txt', 'c.txt content\n'),
    ]);
    hgit(`commit -m "Initial commit"`);
    expect(git('log --format=%B -n 1 HEAD')).toMatchSnapshot();
  });
})

