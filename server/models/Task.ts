import { seq, u8, u16, union } from 'buffer-layout';

export enum Task {
  HTTP_GET = 0,
  JSON_PARSE = 1,
  UINT_128 = 2,
}

/**
 * Map each task to the key BufferLayout outputs to
 */
export const TaskVariantKeys: Record<Task, string> = {
  [Task.HTTP_GET]: 'urlBuffer',
  [Task.JSON_PARSE]: 'pathBuffer',
  [Task.UINT_128]: 'uint128',
};

export const TASK_TAG_LAYOUT = u16('tag');

// default layout must have the max length of the enum
const defaultLayout = seq(u8(), 34, 'defaultLayout');
export const TASK_LAYOUT = union(TASK_TAG_LAYOUT, defaultLayout);
// Can't use utf8 as it kills composability
// would be nice to find a way to have this auto convert to a utf8
// string instead of a u8 array
TASK_LAYOUT.addVariant(0, seq(u8(), 34), TaskVariantKeys[Task.HTTP_GET]);
TASK_LAYOUT.addVariant(1, seq(u8(), 12), TaskVariantKeys[Task.JSON_PARSE]);
TASK_LAYOUT.addVariant(2, TaskVariantKeys[Task.UINT_128]);

const getTaskVariant = (src: any): Task => {
  return TASK_LAYOUT.defaultGetSourceVariant(src).variant as Task;
};

TASK_LAYOUT.configGetSourceVariant(getTaskVariant);
