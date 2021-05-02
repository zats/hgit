import { execSync } from 'child_process';
import { writeFileSync, rmSync } from 'fs';
import { git } from './setup';

export class TestRepo {
  constructor() {
  }

  createFile(arg0: string, arg1: string) {
    throw new Error('Method not implemented.');
  }
  updateFile(arg0: string, arg1: string) {
    throw new Error('Method not implemented.');
  }
  removeFile(arg0: string) {
    throw new Error('Method not implemented.');
  }
  create() {
    git('init');
  }
  stage(files: TestFile[]) {
    files.forEach(file => file.execute());
  }
  commit(message: string) {
    git('add .');
    git(`commit -a -m "${message}"`);
  }
}

enum TestFileOperation {
  Create,
  Update,
  Remove,
}

export class TestFile {
  path: string;
  content?: string;
  operation: TestFileOperation;

  static createFile(path: string, content: string): TestFile {
    return new TestFile(path, TestFileOperation.Create, content);
  }

  static removeFile(path: string): TestFile {
    return new TestFile(path, TestFileOperation.Remove, undefined);
  }
  static updateFile(path: string, content: string): any {
    return new TestFile(path, TestFileOperation.Update, content);
  }

  constructor(path: string, operation: TestFileOperation, content: string) {
    this.path = path;
    this.content = content;
    this.operation = operation;
  }

  execute() {
    switch (this.operation) {
      case TestFileOperation.Create:
      case TestFileOperation.Update:
        writeFileSync(this.path, this.content);
        break;
      case TestFileOperation.Remove:
        rmSync(this.path);
        break;
    }
  }
}
