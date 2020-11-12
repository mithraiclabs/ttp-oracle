import { ORACLE_ACCOUNT_LAYOUT } from './OracleAccount';
import {
  decodedRequestQueue,
  mockOracleAccountBuffer,
} from '../../testing/mockData';

describe('OracleAccount', () => {
  it('should decode account with mock queue', () => {
    const oracle_account = ORACLE_ACCOUNT_LAYOUT.decode(
      mockOracleAccountBuffer,
    );
    expect(oracle_account).toEqual({ requestQueue: decodedRequestQueue });
  });
});
