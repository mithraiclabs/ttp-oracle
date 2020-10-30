import { seq, u8, u32, union, utf8 } from 'buffer-layout';

export enum Task {
  HTTP_GET = 0,
  JSON_PARSE = 1,
  SOL_UINT_256 = 2,
}

export const TASK_LAYOUT = union(u32('tag'));
// Can't use utf8 as it kills composability
// would be nice to find a way to have this auto convert to a utf8
// string instead of a u8 array
TASK_LAYOUT.addVariant(0, seq(u8(), 34), 'urlBuffer');
TASK_LAYOUT.addVariant(1, seq(u8(), 13), 'pathBuffer');
TASK_LAYOUT.addVariant(2, 'solUint256');

const getTaskVariant = (src: any): Task => {
  return TASK_LAYOUT.defaultGetSourceVariant(src).variant as Task;
};

TASK_LAYOUT.configGetSourceVariant(getTaskVariant);
