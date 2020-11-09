import {
  Account,
  Connection,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';
import { REQUEST_LAYOUT } from '../models/Request';
const INSTRUCTION_SIZE = 2;

export const sendTransactionToHandleResponse = async (
  connection: Connection,
  payerAccount: Account,
  programId: PublicKey,
  oracleId: PublicKey,
  callerProgramId: PublicKey,
  responseData: Buffer,
): Promise<string> => {
  const handleResponseInstruction = Buffer.alloc(INSTRUCTION_SIZE);
  handleResponseInstruction.writeUInt8(1);
  // TODO change this so it doesn't have to be the size of the request...
  const instructionBuffer = Buffer.alloc(
    REQUEST_LAYOUT.span + INSTRUCTION_SIZE,
  );
  Buffer.concat([handleResponseInstruction, responseData]).copy(
    instructionBuffer,
  );
  const createRequestTxInstruction = new TransactionInstruction({
    keys: [
      { pubkey: oracleId, isSigner: false, isWritable: true },
      { pubkey: callerProgramId, isSigner: false, isWritable: false },
    ],
    programId,
    data: instructionBuffer,
  });

  return await sendAndConfirmTransaction(
    connection,
    new Transaction().add(createRequestTxInstruction),
    [payerAccount],
    {
      skipPreflight: true,
      commitment: 'recent',
    },
  );
};
