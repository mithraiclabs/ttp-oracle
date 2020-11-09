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
  const createRequestTxInstruction = new TransactionInstruction({
    keys: [
      { pubkey: oracleProgramId, isSigner: false, isWritable: false },
      { pubkey: oracleId, isSigner: false, isWritable: true },
    ],
    programId,
    // creating request is a u8 of 0
    data: Buffer.alloc(1),
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
