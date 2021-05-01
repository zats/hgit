import { FileStatus } from './FileStatus';
import { Repository } from 'nodegit';

export class RepoStatus {
  repo: Repository;
  fileStatus: FileStatus[];

  constructor(repo: Repository, fileStatus: FileStatus[]) {
    this.repo = repo;
    this.fileStatus = fileStatus;
  }
}