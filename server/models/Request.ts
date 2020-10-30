import { seq, struct, u32 } from 'buffer-layout';
import { TASK_LAYOUT } from './Task';

export const REQUEST_LAYOUT = struct([
  seq(TASK_LAYOUT, 3, 'tasks'),
  u32('offset'),
]);
