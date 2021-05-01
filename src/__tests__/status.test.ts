import { createTemporaryRepository, removeTemporaryRepository, hgit } from './shared/setup';
import { TestRepo, TestFile } from './shared/model';

beforeEach(createTemporaryRepository);
afterEach(removeTemporaryRepository);

describe('status', () => {
  it('is empty when just committed', () => {
    const repo = new TestRepo();
    repo.create();
    repo.stage([
      TestFile.createFile('a.txt', 'a.txt content\n'),
      TestFile.createFile('b.txt', 'b.txt content\n'),
      TestFile.createFile('c.txt', 'c.txt content\n'),
    ]);
    repo.commit('Initial commit');
    expect(hgit('status')).toMatchSnapshot();
  });

  it('is not empty when has uncommitted changes', () => {
    const repo = new TestRepo();
    repo.create();
    repo.stage([
      TestFile.createFile('a.txt', 'a.txt content\n'),
      TestFile.createFile('b.txt', 'b.txt content\n'),
      TestFile.createFile('c.txt', 'c.txt content\n'),
    ]);
    repo.commit('Initial commit');
    repo.stage([
      TestFile.updateFile('a.txt', 'New Content\n'),
      TestFile.removeFile('b.txt'),
      TestFile.createFile('d.txt', 'd.txt content\n'),
    ]);
    expect(hgit('status')).toMatchSnapshot();
  });
})

