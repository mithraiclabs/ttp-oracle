import { TASK_LAYOUT, TASK_TAG_LAYOUT } from '../server/models/Task';

export const mockUrlBase = 'https://ftx.us';
export const mockUrlPath = '/api/markets/BTC/USD';

export const mockURL = `${mockUrlBase}${mockUrlPath}`;

export const mockPath = 'result.price';

const getTagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
const jsonTagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
const uint256Tagbuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
const httpGetTaskBuffer = Buffer.alloc(TASK_LAYOUT.span - TASK_TAG_LAYOUT.span);
const jsonParseTaskBuffer = Buffer.alloc(
  TASK_LAYOUT.span - TASK_TAG_LAYOUT.span,
);
const uint256TaskBuffer = Buffer.alloc(TASK_LAYOUT.span - TASK_TAG_LAYOUT.span);
jsonTagBuffer.writeUInt16LE(1);
uint256Tagbuffer.writeUInt16LE(2);
export const mockUrlBuffer = Buffer.from(mockURL, 'utf8');
export const mockPathBuffer = Buffer.from(mockPath, 'utf8');
mockUrlBuffer.copy(httpGetTaskBuffer);
mockPathBuffer.copy(jsonParseTaskBuffer);
export const mockCallerProgramIdBuffer = Buffer.alloc(32);

export const mockRequestBuffer = Buffer.concat([
  getTagBuffer,
  httpGetTaskBuffer,
  jsonTagBuffer,
  jsonParseTaskBuffer,
  uint256Tagbuffer,
  uint256TaskBuffer,
  mockCallerProgramIdBuffer,
]);
