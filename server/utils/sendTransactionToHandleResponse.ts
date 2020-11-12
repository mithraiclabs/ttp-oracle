import {
  Account,
  Connection,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';
const INSTRUCTION_SIZE = 2;

export const sendTransactionToHandleResponse = async (
  connection: Connection,
  payerAccount: Account,
  programId: PublicKey,
  oracleId: PublicKey,
  callerProgramId: PublicKey,
  response: Buffer,
): Promise<string> => {
  const handleResponseInstruction = Buffer.alloc(INSTRUCTION_SIZE);
  handleResponseInstruction.writeUInt8(1);

  const createRequestTxInstruction = new TransactionInstruction({
    keys: [
      { pubkey: oracleId, isSigner: false, isWritable: true },
      { pubkey: callerProgramId, isSigner: false, isWritable: false },
    ],
    programId,
    data: Buffer.concat([handleResponseInstruction, response]),
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
