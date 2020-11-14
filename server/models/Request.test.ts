import { REQUEST_LAYOUT } from './Request';
import { Task, TaskVariantKeys } from './Task';
import {
  mockCallerProgramIdBuffer,
  mockUrlBuffer,
  mockPathBuffer,
  mockRequestBuffer,
} from '../../testing/mockData';

describe('Request', () => {
  it('should decode 0 padded tasks request', () => {
    const request = REQUEST_LAYOUT.decode(mockRequestBuffer);

    expect(request).toEqual({
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
    });
  });
});
