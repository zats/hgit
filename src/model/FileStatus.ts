import { Status } from './Status';

export class FileStatus {
  repoPath: string;
  relativePath: string;
  status: Status;

  constructor(repoPath: string, relativePath: string, status: Status) {
    this.repoPath = repoPath;
    this.relativePath = relativePath;
    this.status = status;
  }

  static sortFunction(s1: FileStatus, s2: FileStatus) {
    const statusOrder = s1.status - s2.status;
    return statusOrder != 0 ? statusOrder : (s1.repoPath > s2.repoPath ? 1 : -1);
  }
}

