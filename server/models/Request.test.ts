import { REQUEST_LAYOUT } from './Request';
import { TASK_LAYOUT, TASK_TAG_LAYOUT } from './Task';

describe('Request', () => {
  it('should decode 0 padded tasks request', () => {
    const url = 'https://ftx.us/api/markets/BTC/USD';
    const path = 'request.price';
    const getTagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
    const jsonTagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
    const uint256Tagbuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
    const httpGetTaskBuffer = Buffer.alloc(
      TASK_LAYOUT.span - TASK_TAG_LAYOUT.span,
    );
    const jsonParseTaskBuffer = Buffer.alloc(
      TASK_LAYOUT.span - TASK_TAG_LAYOUT.span,
    );
    const uint256TaskBuffer = Buffer.alloc(
      TASK_LAYOUT.span - TASK_TAG_LAYOUT.span,
    );
    jsonTagBuffer.writeUInt16LE(1);
    uint256Tagbuffer.writeUInt16LE(2);
    const urlBuffer = Buffer.from(url, 'utf8');
    urlBuffer.copy(httpGetTaskBuffer);
    const pathBuffer = Buffer.from(path, 'utf8');
    pathBuffer.copy(jsonParseTaskBuffer);
    const offsetBuffer = Buffer.alloc(4);
    offsetBuffer.writeUInt32LE(5);

    const requestBuffer = Buffer.concat([
      getTagBuffer,
      httpGetTaskBuffer,
      jsonTagBuffer,
      jsonParseTaskBuffer,
      uint256Tagbuffer,
      uint256TaskBuffer,
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
