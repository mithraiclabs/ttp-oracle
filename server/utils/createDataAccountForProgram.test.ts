import { REQUEST_LAYOUT } from '../models/Request';
import { createDataAccountForProgram } from './createDataAccountForProgram';

describe('createDataAccountForProgram', () => {
  it('should create data account owned by program', async () => {
    const payerAccount = solanaTestHelper.accounts[0];
    const firstProgramId = Object.values(solanaTestHelper.programs)[0];

    const dataAccount = await createDataAccountForProgram(
      solanaTestHelper.connection,
      payerAccount,
      firstProgramId
    );

    expect(dataAccount).toBeDefined();

    const accountInfo = await solanaTestHelper.connection.getAccountInfo(
      dataAccount.publicKey
    );

    expect(accountInfo!.owner.equals(firstProgramId)).toBe(true);
  });
});
