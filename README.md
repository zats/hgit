The goal is to hide two-step commit as an implementation detail. It should still be possible but as an opt-in, not as a default. Additionally this is meant to be much easier to use. Heavily inspired by [hg](https://www.mercurial-scm.org).

## Roadmap:

- [ ] `$ hgit status` - list files with pending changes
- [ ] `$ hgit commit` - save all pending changes or specified files in a new commit
- [ ] `$ hgit sl` - show a graph of the commits that are relevant to you