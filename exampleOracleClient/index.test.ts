import { REQUEST_LAYOUT } from '../server/models/Request';

import { createDataAccountForProgram } from '../server/utils/createDataAccountForProgram';
import { mockRequestBuffer } from '../testing/mockData';
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
    const createRequestInstructionBuffer = Buffer.alloc(2);
    const dataBuffer = Buffer.concat([
      createRequestInstructionBuffer,
      mockRequestBuffer,
    ]);

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
