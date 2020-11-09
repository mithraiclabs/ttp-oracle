import { Account, Connection, PublicKey } from '@solana/web3.js';
import { cluster, ClusterEnv, createDataAccountForProgram } from '../utils';

// DO not add more console logs to this script.
// The setup.sh script relies on the Oracle Data Account Id
// being the only output
const main = async () => {
  if (!process.env.SOLANA_PRIVATE_KEY) {
    throw new Error('No env varaible SOLANA_PRIVATE_KEY found');
  }
  if (!process.env.ORACLE_PROGRAM_ID) {
    throw new Error('No env varaible ORACLE_PROGRAM_ID found');
  }

  // currently assumes the privKey is a string of the json file contents
  const privKeyArray = JSON.parse(process.env.SOLANA_PRIVATE_KEY);

  const oracleProgramId = new PublicKey(process.env.ORACLE_PROGRAM_ID);
  const payerAccount = new Account(Buffer.from(privKeyArray));
  const environment = (process.env.SOLANA_ENV as ClusterEnv) ?? ClusterEnv.dev;
  const connection = new Connection(cluster.rest[environment]);

  const oracleDataAccount = await createDataAccountForProgram(
    connection,
    payerAccount,
    oracleProgramId,
  );

  // Do NOT change this
  console.log(oracleDataAccount.publicKey.toString());
};

main();
