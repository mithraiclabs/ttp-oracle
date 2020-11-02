import { PublicKey } from '@solana/web3.js';
import { fs } from 'mz';

import TestHelper from './testHelper';

declare global {
  // eslint-disable-next-line no-var
  var solanaTestHelper: TestHelper;
}

// read already deployed programs and create accounts
beforeAll(async () => {
  const deployedJsonRaw = fs.readFileSync('./testDeployed.json');
  const deployedJson = JSON.parse(deployedJsonRaw.toString());
  const programIds = Object.keys(deployedJson).reduce((acc, key) => {
    acc[key] = new PublicKey(deployedJson[key]);
    return acc;
  }, {} as Record<string, PublicKey>);
  global.solanaTestHelper = new TestHelper(programIds);
});

beforeEach(async () => {
  // reset accounts before every test
  await global.solanaTestHelper.createAccounts();
});
