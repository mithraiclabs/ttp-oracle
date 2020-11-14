/**
 * TTP Oracle server
 *
 */

import { Account, Connection, PublicKey } from '@solana/web3.js';

import {
  cluster,
  ClusterEnv,
  createDataAccountForProgram,
  createHandleAccountChange,
} from './utils';

class MissingENVVarError extends Error {
  constructor(envVar: string) {
    super(`Must specify ${envVar} env var`);
  }
}

const main = async () => {
  if (!process.env.SOLANA_PRIVATE_KEY) {
    throw new MissingENVVarError('SOLANA_PRIVATE_KEY');
  }
  if (!process.env.ORACLE_PROGRAM_ID) {
    throw new MissingENVVarError('ORACLE_PROGRAM_ID');
  }

  const environment = (process.env.SOLANA_ENV as ClusterEnv) ?? ClusterEnv.dev;
  const connection = new Connection(cluster.rest[environment]);
  const privKeyArray = JSON.parse(process.env.SOLANA_PRIVATE_KEY);
  const payerAccount = new Account(Buffer.from(privKeyArray));
  const programId = new PublicKey(process.env.ORACLE_PROGRAM_ID);

  let oracleId: PublicKey;
  if (!process.env.ORACLE_ID) {
    const oracleAccount = await createDataAccountForProgram(
      connection,
      payerAccount,
      programId,
    );
    oracleId = oracleAccount.publicKey;
  } else {
    oracleId = new PublicKey(process.env.ORACLE_ID);
  }

  const ws = new Connection(cluster.socket[environment]);

  console.log('Howdy, World!');
  console.log(`listening to Oracle: ${oracleId.toString()}`);

  ws.onAccountChange(
    oracleId,
    createHandleAccountChange(connection, payerAccount, programId, oracleId),
  );

  // start listening to account changes
  // on open check request queue for requests, subscribe to account changes

  // TODO check request queue for requests that need to be handled
  // TODO ignore account data changes when there is no new request
};

main();
