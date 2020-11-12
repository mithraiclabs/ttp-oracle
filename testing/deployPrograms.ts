import TestHelper, { LOCALNET_URL } from './testHelper';

(async () => {
  const solanaTestHelper = new TestHelper();
  solanaTestHelper.establishConnection(LOCALNET_URL);
  await solanaTestHelper.createAccounts();
  await solanaTestHelper.deployContracts();
  if (Object.keys(solanaTestHelper.programs).length === 0) {
    console.warn(
      'No programs were deployed. Try building your program with `yarn build`',
    );
  }
})();
