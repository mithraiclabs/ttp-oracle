import { seq, struct, u32 } from 'buffer-layout';
import { Task, TASK_LAYOUT } from './Task';

export interface Request {
  tasks: Task[]; // TODO this is probably wrong since it needs to be a buffer?
  offset: number;
}

export const REQUEST_LAYOUT = struct([
  seq(TASK_LAYOUT, 3, 'tasks'),
  u32('offset'),
]);
