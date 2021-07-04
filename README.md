*Make 2-phase nature of git opt-in.*

The goal is to hide two-step commit as an implementation detail. It should still be possible but as an opt-in, not as a default. Additionally this is meant to be much easier to use. Heavily inspired by [hg](https://www.mercurial-scm.org).

## Roadmap:

- [x] `status` - list files with pending changes
  - [ ] `status --rev` - show difference from revision
  - [ ] `status --change` - list the changed files of a revision
- [ ] `commit` - save all pending changes or specified files in a new commit
  - [ ] `commit --amend`
  - [ ] `amend` - shortcut for `commit --amend`
- [x] `diff` - show differences between commits
  - [ ] `diff --rev` - show difference from revision
  - [ ] `diff --change` - list the changed files of a revision
  - [ ] `diff --color` - when to colorize (boolean, always, or nevers)
- [ ] `record` - interactively select changes to commit
- [ ] `update` - interactively select changes to commit
- [ ] `smartlog` - show a graph of the commits that are relevant to you
