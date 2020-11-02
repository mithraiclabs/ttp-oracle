import { Task, TASK_LAYOUT, TASK_TAG_LAYOUT } from './Task';

describe('Task', () => {
  describe('HTTP_GET', () => {
    const url = 'https://ftx.us/api/markets/BTC/USD';
    const urlBuffer = Buffer.from(url, 'utf8');
    const tagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
    const taskBuffer = Buffer.concat([tagBuffer, urlBuffer]);

    it('should decode task', () => {
      const task = TASK_LAYOUT.decode(taskBuffer);
      expect(task).toEqual({
        urlBuffer: Array.from(urlBuffer),
      });
      expect(TASK_LAYOUT.getSourceVariant(task)).toEqual(Task.HTTP_GET);
    });
  });

  describe('JSON_PARSE', () => {
    it('should decode task', () => {
      const path = 'request.price';
      const pathBuffer = Buffer.from(path, 'utf8');
      const tagBuffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
      tagBuffer.writeUInt16LE(1);
      const taskBuffer = Buffer.concat([tagBuffer, pathBuffer]);

      const task = TASK_LAYOUT.decode(taskBuffer);
      expect(task).toEqual({
        pathBuffer: Array.from(pathBuffer),
      });
      expect(TASK_LAYOUT.getSourceVariant(task)).toEqual(Task.JSON_PARSE);
    });
  });

  describe('SOL_UINT_256', () => {
    it('should decode task', () => {
      const buffer = Buffer.alloc(TASK_TAG_LAYOUT.span);
      buffer.writeUInt16LE(2);

      const task = TASK_LAYOUT.decode(buffer);
      expect(task).toEqual({
        solUint256: true,
      });
      expect(TASK_LAYOUT.getSourceVariant(task)).toEqual(Task.SOL_UINT_256);
    });
  });
});
