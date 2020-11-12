import { REQUEST_QUEUE_LAYOUT } from './RequestQueue';
import {
  decodedRequestQueue,
  mockRequestQueueBuffer,
} from '../../testing/mockData';

describe('RequestQueue', () => {
  it('should decode queue with 2 requests', () => {
    const requestQueue = REQUEST_QUEUE_LAYOUT.decode(mockRequestQueueBuffer);

    expect(requestQueue).toEqual(decodedRequestQueue);
  });
});
