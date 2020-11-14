import { RESPONSE_LAYOUT } from './Response';
import { mockResponseBuffer } from '../../testing/mockData';

describe('Response', () => {
  it('should decode repsponse', () => {
    const response = RESPONSE_LAYOUT.decode(mockResponseBuffer);

    expect(response).toEqual({
      determinant: 0,
      data: [5, 65, 0, 0],
      index: 1,
    });
  });
});
