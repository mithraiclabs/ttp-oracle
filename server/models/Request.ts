import { PublicKey } from '@solana/web3.js';
import { seq, struct, u8 } from 'buffer-layout';
import { TASK_LAYOUT } from './Task';

// TODO remove this in favor of the Request class
export interface Request {
  tasks: Record<string, any>[];
  callerProgramIdBuffer: Buffer;
  index: number;
}

export const REQUEST_LAYOUT = struct([
  seq(TASK_LAYOUT, 3, 'tasks'),
  seq(u8(), 32, 'callerProgramIdBuffer'),
  u8('index'),
]);

export default class OracleRequest {
  tasks = [];
  callerProgramId: PublicKey;
  index: number;

  constructor(buffer: Buffer) {
    const req = REQUEST_LAYOUT.decode(buffer);
    this.tasks = req.tasks;
    const callerBuf = Buffer.from(req.callerProgramIdBuffer);
    this.callerProgramId = new PublicKey(callerBuf);
    this.index = req.index;
  }
}
