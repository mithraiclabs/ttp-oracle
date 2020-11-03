import { fs } from 'mz';

import TestHelper from './testHelper';

declare global {
  // eslint-disable-next-line no-var
  var solanaTestHelper: TestHelper;
}

// read already deployed programs
beforeAll(async () => {
  const deployedJsonRaw = fs.readFileSync('./testDeployed.json');
  const deployedJson = JSON.parse(deployedJsonRaw.toString());
  global.solanaTestHelper = new TestHelper(deployedJson);
});
