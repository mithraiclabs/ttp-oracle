import {
  Task,
  TaskVariantKeys,
  TASK_LAYOUT,
  TASK_TAG_LAYOUT,
} from '../server/models/Task';
import { REQUEST_LAYOUT } from '../server/models/Request';
import { RESPONSE_LAYOUT } from '../server/models/Response';
import { MAX_REQUESTS } from '../server/models/RequestQueue';

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

const requestIndex = Buffer.alloc(1);
requestIndex.writeUIntLE(1, 0, 1);
export const mockRequestBuffer = Buffer.concat([
  getTagBuffer,
  httpGetTaskBuffer,
  jsonTagBuffer,
  jsonParseTaskBuffer,
  uint256Tagbuffer,
  uint256TaskBuffer,
  mockCallerProgramIdBuffer,
  requestIndex,
]);

export const mockRequestQueueBuffer = Buffer.alloc(
  REQUEST_LAYOUT.span * MAX_REQUESTS,
);
mockRequestBuffer.copy(mockRequestQueueBuffer);
mockRequestBuffer.copy(mockRequestQueueBuffer, REQUEST_LAYOUT.span);

export const mockOracleAccountBuffer = Buffer.concat([mockRequestQueueBuffer]);

const requests = [
  {
    tasks: [
      {
        [TaskVariantKeys[Task.HTTP_GET]]: Array.from(mockUrlBuffer),
      },
      {
        [TaskVariantKeys[Task.JSON_PARSE]]: Array.from(mockPathBuffer),
      },
      {
        [TaskVariantKeys[Task.UINT32]]: true,
      },
    ],
    callerProgramIdBuffer: Array.from(mockCallerProgramIdBuffer),
    index: 1,
  },
  {
    tasks: [
      {
        [TaskVariantKeys[Task.HTTP_GET]]: Array.from(mockUrlBuffer),
      },
      {
        [TaskVariantKeys[Task.JSON_PARSE]]: Array.from(mockPathBuffer),
      },
      {
        [TaskVariantKeys[Task.UINT32]]: true,
      },
    ],
    callerProgramIdBuffer: Array.from(mockCallerProgramIdBuffer),
    index: 1,
  },
];
const blankRequest = {
  callerProgramIdBuffer: [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
  ],
  tasks: [
    {
      urlBuffer: [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
      ],
    },
    {
      urlBuffer: [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
      ],
    },
    {
      urlBuffer: [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
      ],
    },
  ],
  index: 0,
};
while (requests.length < MAX_REQUESTS) {
  requests.push(blankRequest);
}

export const decodedRequestQueue = { requests };

const responseDataBuffer = Buffer.alloc(4);
responseDataBuffer.writeUInt32LE(16645);
export const mockResponseBuffer = Buffer.concat([
  // padding for determinant
  Buffer.alloc(1),
  responseDataBuffer,
  requestIndex,
]);
