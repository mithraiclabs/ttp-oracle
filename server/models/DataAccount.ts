import * as BufferLayout from "buffer-layout";

/**
 * Data Account structure
 *
 * Must be X * size of Request, where X is the total number of requests that could be in
 * flight at the same time. How the program handles an overflow is currently unknown.
 */

// offset is single uint32
const OffsetLayout = [BufferLayout.u32("offset")];
// taks is 35 bytes
const TaskLayout = [BufferLayout.seq(BufferLayout.u8(), 35)];
const RequestLayout = [
  ...OffsetLayout,
  ...TaskLayout,
  ...TaskLayout,
  ...TaskLayout,
];

export default class DataAccount {
  static bufferLayout = BufferLayout.struct(RequestLayout);
}
