import { struct } from 'buffer-layout';
import OracleRequestQueue, { REQUEST_QUEUE_LAYOUT } from './RequestQueue';

export const ORACLE_ACCOUNT_LAYOUT = struct([
  REQUEST_QUEUE_LAYOUT.replicate('requestQueue'),
]);

export default class OracleAccount {
  requestQueue: OracleRequestQueue;

  constructor(buffer: Buffer) {
    const req = ORACLE_ACCOUNT_LAYOUT.decode(buffer);
    this.requestQueue = req.requestQueue;
  }
}
