import { Status as GitStatus } from 'nodegit';

export enum Status {
  Unknown,
  Modified,
  Added,
  Removed,
  Clean,
  Missing,
  NotTracked,
  Ignored,
};

export function statusToShortString(status: Status) {
  switch (status) {
    case Status.Unknown: return '';
    case Status.Modified: return 'M';
    case Status.Added: return 'A';
    case Status.Removed: return 'R';
    case Status.Clean: return 'C';
    case Status.Missing: return '!';
    case Status.NotTracked: return '?';
    case Status.Ignored: return 'I';
  }
}

export function gitStatusToStatus(status: GitStatus.STATUS): Status {
  if (status & GitStatus.STATUS.INDEX_NEW) {
    return Status.Added;
  } else if (status & GitStatus.STATUS.WT_NEW) {
    return Status.NotTracked;
  } else if ((status & GitStatus.STATUS.WT_MODIFIED) || (status & GitStatus.STATUS.INDEX_MODIFIED)) {
    return Status.Modified;
  } else if (status & GitStatus.STATUS.WT_DELETED) {
    return Status.Removed;
  }
  console.error(`Unknown status: ${status}`);
  return Status.Unknown;
}
