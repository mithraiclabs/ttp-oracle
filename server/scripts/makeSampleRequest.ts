import { PublicKey } from '@solana/web3.js';
import { fs } from 'mz';

import { sendTransactionToCreateRequest } from '../../exampleOracleClient';
import TestHelper from '../../testing/testHelper';

const CLIET_PROGRAM_KEY = 'example_oracle_client.so';
const ORACLE_PROGRAM_KEY = 'solana_bpf_ttp_oracle.so';

/**
 * yarn send-test [ORACLE_ID] [--loop]
 */
const main = async () => {
  const deployedJsonRaw = fs.readFileSync('./testDeployed.json');
  const deployedJson = JSON.parse(deployedJsonRaw.toString());
  const programIds = Object.keys(deployedJson).reduce((acc, key) => {
    acc[key] = new PublicKey(deployedJson[key]);
    return acc;
  }, {} as Record<string, PublicKey>);
  const testHelper = new TestHelper(programIds);
  const clientProgramId = testHelper.programs[CLIET_PROGRAM_KEY];
  const oracleProgramId = testHelper.programs[ORACLE_PROGRAM_KEY];
  await testHelper.createAccounts();

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const oraclePubKey = new PublicKey(process.argv[2]);
  const payerAccount = testHelper.accounts[2];

  const sendTx = async () =>
    sendTransactionToCreateRequest(
      testHelper.connection,
      payerAccount,
      clientProgramId,
      oracleProgramId,
      oraclePubKey,
    );

  if (!process.argv[3]) {
    await sendTx();
    return;
  }
  let count = 0;
  setInterval(async () => {
    count += 1;
    console.log(`Sending TX #${count} to invoke request`);
    await sendTx();
  }, 500);
};

main();
