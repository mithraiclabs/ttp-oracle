import nock from 'nock';

import { REQUEST_LAYOUT } from '../models/Request';
import {
  mockRequestBuffer,
  mockUrlBase,
  mockUrlPath,
} from '../../testing/mockData';
import { reduceTasks } from './reduceTasks';

describe('reduceTasks', () => {
  it('should reduce array of tasks to single output', async () => {
    const request = REQUEST_LAYOUT.decode(mockRequestBuffer);
    const price = 15417.5;
    const nockRequest = nock(mockUrlBase)
      .get(mockUrlPath)
      .reply(200, {
        result: {
          price,
          ask: 15421.0,
          baseCurrency: 'BTC',
          bid: 15417.5,
          change1h: -0.0016835561886877975,
          change24h: 0.026054838280314123,
          changeBod: -0.005899800116061641,
          enabled: true,
          last: 15403.5,
          minProvideSize: 0.0001,
          name: 'BTC/USD',
          postOnly: false,
          priceIncrement: 0.5,
          quoteCurrency: 'USD',
          quoteVolume24h: 109722.8683,
          restricted: false,
          sizeIncrement: 0.0001,
          type: 'spot',
          underlying: null,
          volumeUsd24h: 109722.8683,
        },
        success: true,
      });

    const result = await reduceTasks(request.tasks);
    expect(nockRequest.isDone()).toBe(true);
    expect(result).toEqual(price);
  });
});
