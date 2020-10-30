import { REQUEST_LAYOUT } from './Request';

describe('Request', () => {
  it('should decode request', () => {
    const url = 'https://ftx.us/api/markets/BTC/USD';
    const path = 'request.price';
    const getTagBuffer = Buffer.alloc(4);
    const jsonTagBuffer = Buffer.alloc(4);
    const uint256buffer = Buffer.alloc(4);
    jsonTagBuffer.writeUInt32LE(1);
    uint256buffer.writeUInt32LE(2);
    const urlBuffer = Buffer.from(url, 'utf8');
    const pathBuffer = Buffer.from(path, 'utf8');
    const offsetBuffer = Buffer.alloc(4);
    offsetBuffer.writeUInt32LE(5);
    const requestBuffer = Buffer.concat([
      getTagBuffer,
      urlBuffer,
      jsonTagBuffer,
      pathBuffer,
      uint256buffer,
      offsetBuffer,
    ]);

    const request = REQUEST_LAYOUT.decode(requestBuffer);

    expect(request).toEqual({
      tasks: [
        {
          urlBuffer: Array.from(urlBuffer),
        },
        {
          pathBuffer: Array.from(pathBuffer),
        },
        {
          solUint256: true,
        },
      ],
      offset: 5,
    });
  });
});
