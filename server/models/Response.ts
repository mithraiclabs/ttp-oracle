import { seq, struct, u8 } from 'buffer-layout';

// TODO remove this in favor of the Response class
export interface Response {
  data: number[];
  index: number;
}

export const RESPONSE_LAYOUT = struct([
  u8('determinant'),
  seq(u8(), 16, 'data'),
  u8('index'),
]);

export default class OracleResponse {
  data: number[];
  index: number;

  constructor(buffer: Buffer) {
    const res = RESPONSE_LAYOUT.decode(buffer);
    this.data = res.data;
    this.index = res.index;
  }
}
