import {
  Account,
  Connection,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';

export const sendTransactionDirectlyToOracleProgram = async (
  connection: Connection,
  payerAccount: Account,
  oracleProgramId: PublicKey,
  oracleId: PublicKey,
  data: Buffer,
): Promise<string> => {
  const createRequestTxInstruction = new TransactionInstruction({
    keys: [{ pubkey: oracleId, isSigner: false, isWritable: true }],
    programId: oracleProgramId,
    data,
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

export const sendTransactionToCreateRequest = async (
  connection: Connection,
  payerAccount: Account,
  programId: PublicKey,
  oracleProgramId: PublicKey,
  oracleId: PublicKey,
): Promise<string> => {
  // must generate a program address in order to use invoke_signed
  const createRequestTxInstruction = new TransactionInstruction({
    keys: [
      { pubkey: oracleProgramId, isSigner: false, isWritable: false },
      { pubkey: oracleId, isSigner: false, isWritable: true },
    ],
    programId,
    data: Buffer.alloc(0),
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
