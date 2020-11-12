import { seq, struct } from 'buffer-layout';
import { Request, REQUEST_LAYOUT } from './Request';

export const MAX_REQUESTS = 10;
export const REQUEST_QUEUE_LAYOUT = struct([
  seq(REQUEST_LAYOUT, MAX_REQUESTS, 'requests'),
]);

export default class OracleRequestQueue {
  requests: Request[];

  constructor(buffer: Buffer) {
    const req = REQUEST_QUEUE_LAYOUT.decode(buffer);
    this.requests = req.requests;
  }
}
