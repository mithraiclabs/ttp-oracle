import * as semver from 'semver';

import { PROGRAM_PATHS } from './testHelper';

describe('With established connection', () => {
  test('A valid connection should be established', async () => {
    expect(solanaTestHelper.connection).not.toBe(null);
    const version = await solanaTestHelper.connection.getVersion();
    expect(semver.gte(version['solana-core'].split(' ')[0], '1.3.9')).toBe(
      true,
    );
  });

  test('accounts should be created', async () => {
    await solanaTestHelper.createAccounts();
    expect(Array.isArray(solanaTestHelper.accounts)).toBe(true);
    expect(solanaTestHelper.accounts.length).toBeGreaterThanOrEqual(10);
  });

  // Skip deploying contracts since it times out often
  describe.skip('deployContracts', () => {
    test('it should deploy the hello world contract to the chain', async () => {
      jest.setTimeout(30000);
      await solanaTestHelper.deployContracts();
      expect(Object.keys(solanaTestHelper.programs).length).toEqual(
        Object.keys(PROGRAM_PATHS).length,
      );
      await Promise.all(
        Object.values(solanaTestHelper.programs).map(async (programId) => {
          // check if the first program is actually deployed
          let accountInfo;
          try {
            accountInfo = await solanaTestHelper.connection.getAccountInfo(
              programId,
            );
          } catch (error) {
            // swallow error
          } finally {
            // not sure the best way to test the successfully deploy, but can
            // assume if the account info returned successfully then the program
            // exists on chain
            expect(accountInfo).toBeDefined();
          }
        }),
      );
    });
  });
});
