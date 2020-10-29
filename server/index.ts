/**
 * TTP Oracle server
 *
 */

import { Account, Connection, PublicKey } from "@solana/web3.js";
import { createDataAccountForProgram } from "./utils";

class MissingENVVarError extends Error {
  constructor(envVar: string) {
    super(`Must specify ${envVar} env var`);
  }
}

const main = async () => {
  if (!process.env.WALLET_PRIVATE_KEY) {
    throw new MissingENVVarError("WALLET_PRIVATE_KEY");
  }
  if (!process.env.ORACLE_PROGRAM_ID) {
    throw new MissingENVVarError("ORACLE_PROGRAM_ID");
  }
  if (!process.env.SOLANA_URL) {
    throw new MissingENVVarError("SOLANA_URL");
  }

  const connection = new Connection(process.env.SOLANA_URL);
  const payerAccount = new Account(
    Buffer.from(process.env.WALLET_PRIVATE_KEY, "utf-8")
  );
  const programId = new PublicKey(process.env.ORACLE_PROGRAM_ID);

  let dataAccount;
  if (!process.env.DATA_ACCOUNT_ADDRESS) {
    dataAccount = await createDataAccountForProgram(
      connection,
      payerAccount,
      programId
    );
    console.log(
      "Generated data account at address ",
      dataAccount.publicKey.toBase58()
    );
  } else {
    dataAccount = new Account(
      Buffer.from(process.env.DATA_ACCOUNT_ADDRESS, "utf-8")
    );
  }
};

main();
