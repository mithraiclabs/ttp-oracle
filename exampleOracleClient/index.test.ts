import { REQUEST_LAYOUT } from '../server/models/Request';

import { createDataAccountForProgram } from '../server/utils/createDataAccountForProgram';
import {
  sendTransactionDirectlyToOracleProgram,
  sendTransactionToCreateRequest,
} from './';

const CLIET_PROGRAM_KEY = 'example_oracle_client.so';
const ORACLE_PROGRAM_KEY = 'solana_bpf_ttp_oracle.so';

describe('exampleOracleRequest', () => {
  it('should create a request in the correct data account', async () => {
    const clientProgramId = solanaTestHelper.programs[CLIET_PROGRAM_KEY];
    const oracleProgramId = solanaTestHelper.programs[ORACLE_PROGRAM_KEY];
    const oracleDataAccount = await createDataAccountForProgram(
      solanaTestHelper.connection,
      solanaTestHelper.accounts[0],
      oracleProgramId,
    );

    const payerAccount = solanaTestHelper.accounts[2];
    const resp = await sendTransactionToCreateRequest(
      solanaTestHelper.connection,
      payerAccount,
      clientProgramId,
      oracleProgramId,
      oracleDataAccount.publicKey,
    );
    expect(resp).toBeTruthy();

    const accountInfo = await solanaTestHelper.connection.getAccountInfo(
      oracleDataAccount.publicKey,
    );

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    expect(Buffer.from(accountInfo!.data)).not.toEqual(
      Buffer.alloc(REQUEST_LAYOUT.span),
    );
    // TODO assert on the decoded request
  });

  it('should create a request by calling to the ttp oracle program directly', async () => {
    const oracleProgramId = solanaTestHelper.programs[ORACLE_PROGRAM_KEY];
    const oracleDataAccount = await createDataAccountForProgram(
      solanaTestHelper.connection,
      solanaTestHelper.accounts[0],
      oracleProgramId,
    );
    const payerAccount = solanaTestHelper.accounts[2];
    // eslint-disable-next-line prettier/prettier
    const dataBuffer = Buffer.from(bufferdata);

    const resp = await sendTransactionDirectlyToOracleProgram(
      solanaTestHelper.connection,
      payerAccount,
      oracleProgramId,
      oracleDataAccount.publicKey,
      dataBuffer,
    );

    expect(resp).toBeTruthy();

    const accountInfo = await solanaTestHelper.connection.getAccountInfo(
      oracleDataAccount.publicKey,
    );

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    expect(Buffer.from(accountInfo!.data)).not.toEqual(
      Buffer.alloc(REQUEST_LAYOUT.span),
    );
  });
});

const bufferdata = [
  0,
  0,
  0,
  0,
  104,
  116,
  116,
  112,
  115,
  58,
  47,
  47,
  102,
  116,
  120,
  46,
  117,
  115,
  47,
  97,
  112,
  105,
  47,
  109,
  97,
  114,
  107,
  101,
  116,
  115,
  47,
  66,
  84,
  67,
  47,
  85,
  83,
  68,
  1,
  0,
  114,
  101,
  115,
  117,
  108,
  116,
  46,
  112,
  114,
  105,
  99,
  101,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  2,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
];
