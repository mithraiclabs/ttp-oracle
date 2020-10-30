import {
  Account,
  Connection,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from '@solana/web3.js';

import { REQUEST_LAYOUT } from '../models/Request';

export const createDataAccountForProgram = async (
  connection: Connection,
  payerAccount: Account,
  programId: PublicKey
): Promise<Account> => {
  const dataAccount = new Account();
  const space = REQUEST_LAYOUT.bufferLayout.span;
  const lamps = await connection.getMinimumBalanceForRentExemption(space);
  const createAccountTX = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payerAccount.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: lamps,
      space,
      programId,
    })
  );
  const signers = [payerAccount, dataAccount];
  try {
    await sendAndConfirmTransaction(connection, createAccountTX, signers, {
      skipPreflight: true,
      commitment: 'recent',
    });
  } catch (err) {
    throw new Error(`Failed to create new Oracle Data Account ${err}`);
  }

  return dataAccount;
};
