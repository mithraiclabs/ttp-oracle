import { PublicKey } from '@solana/web3.js';
import { seq, struct, u8 } from 'buffer-layout';
import { TASK_LAYOUT } from './Task';

// TODO remove this in favor of the Request class
export interface Request {
  tasks: Record<string, any>[];
  callerProgramId: string;
}

export const REQUEST_LAYOUT = struct([
  seq(TASK_LAYOUT, 3, 'tasks'),
  seq(u8(), 32, 'callerProgramIdBuffer'),
]);

export default class OracleRequest {
  tasks = [];
  callerProgramId: PublicKey;

  constructor(buffer: Buffer) {
    const req = REQUEST_LAYOUT.decode(buffer);
    this.tasks = req.tasks;
    const callerBuf = Buffer.from(req.callerProgramIdBuffer);
    this.callerProgramId = new PublicKey(callerBuf);
  }
}
